use edjx::{error, info, BaseStream, HttpRequest, HttpResponse, StatusCode};

pub fn serverless_streaming(_req: HttpRequest) {
    info!("** Streamed HTTP Response **");

    // Prepare an HTTP response
    let mut response = HttpResponse::new()
        .set_status(StatusCode::OK)
        .set_header(
            "Content-Type".parse().unwrap(),
            "text/plain".parse().unwrap(),
        )
        .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap());

    // Open a write stream for the response body
    let mut write_stream = match response.send_streaming() {
        Ok(stream) => stream,
        Err(err) => {
            error!("Error in send_streaming: {}", err);
            return;
        }
    };

    // Stream the HTTP response body
    info!("** Streaming HTTP response **");

    let iterations: u32 = 10000;
    let mut success = true;

    for i in 0..iterations {
        // Abort after 10 iterations (Simulate an error)
        if i >= 10 {
            let _ = write_stream.abort();
            return;
        }
        // Send a chunk
        if let Err(err) =
            write_stream.write_chunk_text(format!("Chunk {}/{}\r\n", i, iterations).as_str())
        {
            error!("Error when writing a chunk: {}", err);
            success = false;
            break;
        }
    }

    // Close the stream
    if success {
        info!("** Closing the write stream **");
        if let Err(err) = write_stream.close() {
            error!("Error when closing the stream: {}", err);
        }
    } else {
        info!("** Aborting the write stream **");
        if let Err(err) = write_stream.abort() {
            error!("Error when aborting the stream: {}", err);
        }
    }
}
