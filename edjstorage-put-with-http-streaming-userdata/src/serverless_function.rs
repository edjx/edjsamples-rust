use edjx::{error, info, storage, BaseStream, HttpRequest, HttpResponse, StatusCode};

pub fn serverless(mut req: HttpRequest) {
    info!("** Storage put - Streaming version with custom data **");

    // 1. param (required): "file_name" -> name that will be given to the uploaded content
    let file_name = match req.query_param_by_name("file_name") {
        Some(v) => v,
        None => {
            error!("No file name found in query params of the request");
            let _ = HttpResponse::from("No file name found in query params of the request")
                .set_status(StatusCode::BAD_REQUEST)
                .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap())
                .send();
            return;
        }
    };

    // 2. param (required): "bucket_id" -> in which bucket content will be uploaded
    let bucket_id = match req.query_param_by_name("bucket_id") {
        Some(v) => v,
        None => {
            error!("No bucket id found in query params of the request");
            let _ = HttpResponse::from("No bucket id found in query params of the request")
                .set_status(StatusCode::BAD_REQUEST)
                .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap())
                .send();
            return;
        }
    };

    // 3. param (optional): "properties" -> e.g., cache-control=true,a=b
    let properties = match req.query_param_by_name("properties") {
        Some(v) => v,
        None => "".to_owned(),
    };

    // Open a read stream from the HTTP request from the client
    let client_read_stream = match req.get_read_stream() {
        Ok(stream) => stream,
        Err(err) => {
            error!("Could not open read stream from the request: {}", err);
            let _ = HttpResponse::from(format!(
                "Could not open read stream from the request: {}",
                err
            ))
            .set_status(StatusCode::INTERNAL_SERVER_ERROR)
            .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap())
            .send();
            return;
        }
    };

    // Open a write stream to the storage
    let (storage_resp_pending, mut storage_write_stream) =
        match storage::put_streaming(&bucket_id, &file_name, &properties) {
            Ok(val) => val,
            Err(err) => {
                error!("Error when creating a storage write stream: {}", err);
                let _ = client_read_stream.close();
                let _ = HttpResponse::from(format!(
                    "Error when creating a storage write stream: {}",
                    err.to_string()
                ))
                .set_status(err.to_http_status_code())
                .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap())
                .send();
                return;
            }
        };

    // Prepare an HTTP response for the client
    let mut res = HttpResponse::new()
        .set_status(StatusCode::OK)
        .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap());

    // Open a write stream for the HTTP response
    let mut client_write_stream = match res.send_streaming() {
        Ok(stream) => stream,
        Err(err) => {
            error!(
                "Error when creating a write stream for the HTTP response: {}",
                err
            );
            let _ = client_read_stream.close();
            let _ = storage_write_stream.abort();
            let _ = HttpResponse::from(format!(
                "Error when creating a write stream for the HTTP response: {}",
                err.to_string()
            ))
            .set_status(StatusCode::INTERNAL_SERVER_ERROR)
            .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap())
            .send();
            return;
        }
    };

    // Read the request chunk by chunk
    let mut count = 0;
    while let Some(bytes_res) = client_read_stream.read_chunk() {
        match bytes_res {
            Ok(bytes) => {
                count += 1;
                let length = bytes.len();
                // Send the chunk to the storage
                if let Err(storage_write_err) = storage_write_stream.write_chunk_binary(bytes) {
                    error!(
                        "Error when writing a chunk to the storage: {}",
                        storage_write_err
                    );

                    if let Err(client_write_err) = client_write_stream.write_chunk_text(
                        format!(
                            "Error when writing a chunk to the storage: {}\r\n",
                            storage_write_err
                        )
                        .as_str(),
                    ) {
                        error!("Error when writing a response chunk: {}", client_write_err);
                    }
                    let _ = client_read_stream.close();
                    let _ = storage_write_stream.abort();
                    let _ = client_write_stream.close();
                    return;
                }

                // Stream the chunk upload progress back to the client
                if let Err(err) = client_write_stream.write_chunk_text(
                    format!("Sent chunk #{} of size {}\r\n", count, length).as_str(),
                ) {
                    error!("Error when writing a response chunk: {}", err);
                    let _ = client_read_stream.close();
                    let _ = storage_write_stream.abort();
                    let _ = client_write_stream.close();
                    return;
                }
            }
            Err(client_read_err) => {
                error!(
                    "Error when reading a chunk from the HTTP request: {}",
                    client_read_err
                );

                if let Err(client_write_err) = client_write_stream.write_chunk_text(
                    format!(
                        "Error when reading a chunk from the HTTP request: {}\r\n",
                        client_read_err
                    )
                    .as_str(),
                ) {
                    error!("Error when writing a response chunk: {}", client_write_err);
                }
                let _ = client_read_stream.close();
                let _ = storage_write_stream.abort();
                let _ = client_write_stream.close();
                return;
            }
        }
    }

    // Close the HTTP request read stream
    if let Err(err) = client_read_stream.close() {
        error!("Error when closing the read stream: {}", err);
    }

    // Report the end of the stream to the client
    if let Err(err) = client_write_stream.write_chunk_text("End of stream\r\n") {
        error!("Error when writing a response chunk: {}", err);
        let _ = storage_write_stream.abort();
        let _ = client_write_stream.close();
        return;
    }

    // Close the storage write stream
    if let Err(storage_write_close_err) = storage_write_stream.close() {
        error!(
            "Error when closing the storage write stream: {}",
            storage_write_close_err
        );

        if let Err(client_write_err) = client_write_stream.write_chunk_text(
            format!(
                "Error when closing the storage write stream: {}\r\n",
                storage_write_close_err
            )
            .as_str(),
        ) {
            error!("Error when writing a chunk: {}", client_write_err);
        }
        let _ = client_write_stream.close();
        return;
    }

    // Get a response from the storage
    match storage_resp_pending.get_storage_response() {
        Ok(mut storage_resp) => match storage_resp.read_body() {
            Ok(body) => {
                info!("Streaming to storage was successful");

                let s = match std::str::from_utf8(&body) {
                    Ok(v) => v,
                    Err(e) => panic!("Body Invalid: {}", e),
                };

                if let Err(client_write_err) = client_write_stream
                    .write_chunk_text(format!("Storage put successful: {}\r\n", s).as_str())
                {
                    error!("Error when writing a chunk: {}", client_write_err);
                    let _ = client_write_stream.close();
                    return;
                }
            }
            Err(storage_resp_body_err) => {
                error!(
                    "Error when reading storage response body: {}",
                    storage_resp_body_err.to_string()
                );

                if let Err(client_write_err) = client_write_stream.write_chunk_text(
                    format!(
                        "Error when reading storage response body: {}\r\n",
                        storage_resp_body_err
                    )
                    .as_str(),
                ) {
                    error!("Error when writing a chunk: {}", client_write_err);
                }
                let _ = client_write_stream.close();
                return;
            }
        },
        Err(storage_resp_err) => {
            error!("Storage response error: {}", storage_resp_err);

            if let Err(client_write_err) = client_write_stream.write_chunk_text(
                format!("Storage response error: {}\r\n", storage_resp_err).as_str(),
            ) {
                error!("Error when writing a chunk: {}", client_write_err);
            }
            let _ = client_write_stream.close();
            return;
        }
    }

    // Close the HTTP response stream
    if let Err(err) = client_write_stream.close() {
        error!("Error when closing the response stream: {}", err);
    }
}
