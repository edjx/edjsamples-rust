use edjx::{error, info, BaseStream, HttpFetch, HttpRequest, HttpResponse, StatusCode, Uri};
use std::str::FromStr;

pub fn serverless(_req: HttpRequest) -> HttpResponse {
    info!("** HTTP Fetch Streaming Example **");

    // An HTTP request will be sent to this address
    let fetch_uri = Uri::from_str("https://httpbin.org/post").unwrap();

    // Open a write stream
    let (fetch_resp_pending, mut write_stream) = match HttpFetch::post(fetch_uri)
        .set_header(
            "Content-Type".parse().unwrap(),
            "text/plain".parse().unwrap(),
        )
        .send_streaming()
    {
        Ok(val) => val,
        Err(err) => {
            error!("Error when opening a write stream: {}", err.to_string());
            let res = HttpResponse::from(format!(
                "Error when opening a write stream: {}",
                err.to_string()
            ))
            .set_status(StatusCode::INTERNAL_SERVER_ERROR)
            .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap());

            return res;
        }
    };

    // Prepare data that will be streamed
    let content = "WHERE IS THE EDGE, ANYWAY?\nIf you ask a cloud company, they tell you the edge is their multi-billion dollar collection of server farms. A content delivery network (CDN) provider says it's their hundreds of points of presence. Wireless carriers will try to convince you it's their tens of thousands of macrocell and picocell sites.\n\nAt EDJX, we say the edge is anywhere and everywhere, a thousand feet away from you at all times. We believe computing needs to become ubiquitous, like electricity, to power billions of connected devices.\nThe edge will go so far out into the woods, you can hear the sasquatch scream.";
    let mut stream_success = true;

    // Send chunks of increasing sizes (1, 2, 3, ...)
    let mut i = 0;
    let mut len = 1;
    while i < content.len() {
        // Alternate between sending chunks as text and as bytes
        let chunk = content.chars().skip(i).take(len).collect::<String>();
        if let Err(err) = write_stream.write_chunk_text(chunk.as_str()) {
            error!("Error when writing a text chunk: {}", err);
            stream_success = false;
            break;
        }

        i += len;
        len += 1;
    }

    // Close the stream
    if stream_success {
        // No error encountered - cleanly close the stream
        if let Err(err) = write_stream.close() {
            error!("Error when closing a stream: {}", err);
            stream_success = false;
        }
    } else {
        // There was an error - abort the stream
        if let Err(err) = write_stream.abort() {
            error!("Error when aborting a stream: {}", err);
            stream_success = false;
        }
    }

    // Exit if streaming failed
    if !stream_success {
        let res = HttpResponse::from("Write stream failed. See error log.")
            .set_status(StatusCode::INTERNAL_SERVER_ERROR)
            .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap());

        return res;
    }

    // Get response from the server
    let mut fetch_resp = match fetch_resp_pending.get_fetch_response() {
        Ok(fetch_resp) => fetch_resp,
        Err(err) => {
            error!("Could not obtain fetch response: {}", err.to_string());
            let res = HttpResponse::from(format!(
                "Could not obtain fetch response: {}",
                err.to_string()
            ))
            .set_status(StatusCode::INTERNAL_SERVER_ERROR)
            .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap());

            return res;
        }
    };

    // Get body from the response
    let body = match fetch_resp.read_body() {
        Ok(body) => body,
        Err(err) => {
            error!("Could not read fetch response body: {}", err.to_string());
            let res = HttpResponse::from(format!(
                "Could not read fetch response body: {}",
                err.to_string()
            ))
            .set_status(StatusCode::INTERNAL_SERVER_ERROR)
            .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap());

            return res;
        }
    };

    // Send the fetch response body as a response to the client, all at once
    let res = HttpResponse::from(body)
        .set_status(StatusCode::OK)
        .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap());

    return res;
}
