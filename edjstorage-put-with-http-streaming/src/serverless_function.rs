use edjx::{error, info, storage, BaseStream, HttpRequest, HttpResponse, StatusCode};

pub fn serverless(req: HttpRequest) -> HttpResponse {
    info!("** Storage put with http function - Streaming version **");

    // 1. param (required): "file_name" -> name that will be given to the uploaded content
    let file_name = match req.query_param_by_name("file_name") {
        Some(v) => v,
        None => {
            error!("No file_name found in query params of request");
            return HttpResponse::from("No file name found in query params of request")
                .set_status(StatusCode::BAD_REQUEST)
                .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap());
        }
    };

    // 2. param (required): "bucket_id" -> in which bucket content will be uploaded
    let bucket_id = match req.query_param_by_name("bucket_id") {
        Some(v) => v,
        None => {
            error!("No bucket id found in query params of request");
            return HttpResponse::from("No bucket id found in query params of request")
                .set_status(StatusCode::BAD_REQUEST)
                .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap());
        }
    };

    // 3. param (optional): "properties" -> e.g., cache-control=true,a=b
    let properties = match req.query_param_by_name("properties") {
        Some(v) => v,
        None => "".to_owned(),
    };

    // Create a write stream to the storage
    let (storage_resp_pending, mut write_stream) =
        match storage::put_streaming(&bucket_id, &file_name, &properties) {
            Ok(val) => val,
            Err(err) => {
                error!("Error when creating a stream: {}", err);
                return HttpResponse::from(format!(
                    "Error when creating a stream: {}",
                    err.to_string()
                ))
                .set_status(err.to_http_status_code())
                .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap());
            }
        };

    // Prepare the content that will be uploaded to the storage
    let content = "WHERE IS THE EDGE, ANYWAY?\nIf you ask a cloud company, they tell you the edge is their multi-billion dollar collection of server farms. A content delivery network (CDN) provider says it's their hundreds of points of presence. Wireless carriers will try to convince you it's their tens of thousands of macrocell and picocell sites.\n\nAt EDJX, we say the edge is anywhere and everywhere, a thousand feet away from you at all times. We believe computing needs to become ubiquitous, like electricity, to power billions of connected devices.\nThe edge will go so far out into the woods, you can hear the sasquatch scream.";

    let mut success = true;

    // Send chunks of increasing sizes (1, 2, 3, ...)
    let mut i = 0;
    let mut len = 1;
    while i < content.len() {
        let chunk = content.chars().skip(i).take(len).collect::<String>();
        if let Err(err) = write_stream.write_chunk_text(chunk.as_str()) {
            error!("Error when writing a text chunk: {}", err);
            success = false;
            break;
        }

        i += len;
        len += 1;
    }

    // Close the stream
    if success {
        // No error encountered - cleanly close the stream
        if let Err(err) = write_stream.close() {
            error!("Error when closing a stream: {}", err);
            success = false;
        }
    } else {
        // There was an error - abort the stream
        if let Err(err) = write_stream.abort() {
            error!("Error when aborting a stream: {}", err);
            success = false;
        }
    }

    // Exit if streaming failed
    if !success {
        error!("Streaming to storage failed.");
        let res = HttpResponse::from("Streaming to storage failed. See error log.")
            .set_status(StatusCode::INTERNAL_SERVER_ERROR)
            .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap());

        return res;
    }

    // Get a response from the storage
    match storage_resp_pending.get_storage_response() {
        Ok(mut storage_resp) => match storage_resp.read_body() {
            Ok(body) => {
                info!("Streaming to storage was successful");
                let res = HttpResponse::from(body)
                    .set_status(StatusCode::OK)
                    .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap());

                return res;
            }
            Err(err) => {
                error!(
                    "Error when reading storage response body: {}",
                    err.to_string()
                );
                let res = HttpResponse::from(format!(
                    "Error when reading storage response body: {}",
                    err.to_string()
                ))
                .set_status(StatusCode::INTERNAL_SERVER_ERROR)
                .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap());

                return res;
            }
        },
        Err(err) => {
            error!("Error when receiving a storage response: {}", err);
            let res = HttpResponse::from(format!(
                "Error when receiving a storage response: {}",
                err.to_string()
            ))
            .set_status(err.to_http_status_code())
            .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap());

            return res;
        }
    }
}
