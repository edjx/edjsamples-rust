use edjx::{error, info, BaseStream, HttpRequest, HttpResponse, StatusCode};

pub fn serverless_streaming(mut req: HttpRequest) {
    info!("** Streamed HTTP request and response **");

    // Open a read stream from the request
    let read_stream = match req.get_read_stream() {
        Ok(stream) => stream,
        Err(err) => {
            error!("Could not open read stream: {}", err);
            return;
        }
    };

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

    // Stream the message
    if let Err(err) =
        write_stream.write_chunk_text("Welcome to EDJX! Streamed data will be echoed back.\r\n")
    {
        error!("Error when writing a chunk: {}", err);
        let _ = read_stream.close();
        let _ = write_stream.abort();
        return;
    }

    // Read all chunks from the read stream and send them to the write stream
    let mut count = 0;
    while let Some(read_result) = read_stream.read_chunk() {
        match read_result {
            Ok(bytes) => {
                // A chunk of data was received. Send it back to the client.
                if let Err(err) = write_stream.write_chunk_binary(bytes) {
                    error!("Error when writing a chunk: {}", err);
                    let _ = read_stream.close();
                    let _ = write_stream.abort();
                    return;
                }
                count += 1;
            }
            Err(err) => {
                error!("Error when reading a chunk: {}", err);
                let _ = read_stream.close();
                let _ = write_stream.abort();
                return;
            }
        }
    }

    // Write some statistics at the end
    if let Err(err) =
        write_stream.write_chunk_text(format!("\r\nTransmitted {} chunks.\r\n", count).as_str())
    {
        error!("Error when writing the summary info: {}", err);
        let _ = read_stream.close();
        let _ = write_stream.abort();
        return;
    }

    // Close the write stream
    if let Err(err) = write_stream.close() {
        error!("Error when closing the write stream: {}", err);
    }

    // Close the read stream
    if let Err(err) = read_stream.close() {
        error!("Error when closing the read stream: {}", err);
    }
}
