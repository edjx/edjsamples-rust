use edjx::{error, info, BaseStream, HttpFetch, HttpRequest, HttpResponse, StatusCode, Uri};
use std::str::FromStr;

pub fn serverless_streaming(_req: HttpRequest) {
    // URL to fetch from
    let fetch_uri = Uri::from_str("https://norvig.com/big.txt").unwrap();

    // Send an HTTP fetch request
    let mut fetch_res = match HttpFetch::get(fetch_uri).send() {
        Ok(fetch_res) => fetch_res,
        Err(err) => {
            error!("Error in HTTP fetch: {}", err);
            let _ = HttpResponse::from(format!("Error in HTTP fetch: {}", err))
                .set_status(StatusCode::INTERNAL_SERVER_ERROR)
                .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap())
                .send();
            return;
        }
    };

    // Get a read stream from the fetch response
    let read_stream = fetch_res.get_read_stream();

    // Prepare an HTTP response
    let mut res = HttpResponse::new()
        .set_status(StatusCode::OK)
        .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap());

    // Open a write stream for the HTTP response
    let mut write_stream = match res.send_streaming() {
        Ok(stream) => stream,
        Err(err) => {
            error!("Error when opening a write stream: {}", err);
            let _ = read_stream.close();
            return;
        }
    };

    // Pipe the fetch response stream into the HTTP response stream
    match read_stream.pipe_to(&mut write_stream) {
        Ok(_) => {
            info!("Successfully executed pipe");
        }
        Err(err) => {
            error!("Error when piping: {}", err);
            let _ = write_stream.abort();
            return;
        }
    }

    // No need to call close() on streams because the pipe_to() method
    // automatically closes both the read stream and the write stream.
}
