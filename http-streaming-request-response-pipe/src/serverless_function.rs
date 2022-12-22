use edjx::{error, info, BaseStream, HttpRequest, HttpResponse, StatusCode};

pub fn serverless_streaming(mut req: HttpRequest) {
    info!("** Streamed HTTP request and response with pipe **");

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
    if let Err(err) = read_stream.pipe_to(&mut write_stream) {
        error!("Error when piping streams: {}", err);
        return;
    }

    // No need to call close() on streams because the pipe_to() method
    // automatically closes both the read stream and the write stream.
}
