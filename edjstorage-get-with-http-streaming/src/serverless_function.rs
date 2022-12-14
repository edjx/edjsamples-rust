use edjx::{
    error, info, storage, BaseStream, HttpRequest, HttpResponse, StatusCode, StorageResponse,
};

pub fn serverless_streaming(req: HttpRequest) {
    info!("** Storage get with http function - Streaming version **");

    // 1. param (required): "file_name" -> name that will be given to the uploaded content
    let file_name = match req.query_param_by_name("file_name") {
        Some(v) => v,
        None => {
            error!("No file_name found in query params of request");
            let _ = HttpResponse::from("No file name found in query params of request")
                .set_status(StatusCode::BAD_REQUEST)
                .send();
            return;
        }
    };

    // 2. param (required): "bucket_id" -> in which bucket content will be uploaded
    let bucket_id = match req.query_param_by_name("bucket_id") {
        Some(v) => v,
        None => {
            error!("No bucket id found in query params of request");
            let _ = HttpResponse::from("No bucket id found in query params of request")
                .set_status(StatusCode::BAD_REQUEST)
                .send();
            return;
        }
    };

    // Get the file from the storage
    let mut storage_res: StorageResponse = match storage::get(&bucket_id, &file_name) {
        Ok(r) => r,
        Err(e) => {
            error!("Error in storage::get(): {}", e);
            let _ = HttpResponse::from(e.to_string())
                .set_status(e.to_http_status_code())
                .send();
            return;
        }
    };
    info!("Get Content Successful");

    // Open a read stream from the file
    let read_stream = storage_res.get_read_stream();

    // Prepare a response
    let mut res = HttpResponse::new()
        .set_status(StatusCode::OK)
        .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap());

    // Open a write stream for the response
    let mut write_stream = match res.send_streaming() {
        Ok(stream) => stream,
        Err(err) => {
            error!("Could not open write stream: {}", err);
            let _ = read_stream.close();
            return;
        }
    };

    // Read the file chunk by chunk
    let mut count = 0;
    while let Some(bytes_res) = read_stream.read_chunk() {
        match bytes_res {
            Ok(bytes) => {
                count += 1;
                // Send the chunk in the HTTP response
                if let Err(err) = write_stream.write_chunk_binary(bytes) {
                    error!("Error when writing a chunk: {}", err);
                    let _ = read_stream.close();
                    let _ = write_stream.abort();
                    return;
                }
            }
            Err(err) => {
                error!("Error in read_chunk: {}", err);
                let _ = read_stream.close();
                let _ = write_stream.abort();
                return;
            }
        }
    }

    // Send some statistics at the end
    if let Err(err) = write_stream
        .write_chunk_text(format!("\nEnd of stream. Received {} chunks.", count).as_str())
    {
        error!("Error when writing a text chunk: {}", err);
        let _ = read_stream.close();
        let _ = write_stream.abort();
        return;
    }

    if let Err(err) = read_stream.close() {
        error!("Error when closing the read stream: {}", err);
    }

    if let Err(err) = write_stream.close() {
        error!("Error when closing the write stream: {}", err);
    }
}
