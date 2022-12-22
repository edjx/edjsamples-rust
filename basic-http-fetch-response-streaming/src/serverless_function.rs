use edjx::{info, BaseStream, HttpFetch, HttpRequest, HttpResponse, StatusCode, Uri};
use std::str::FromStr;

pub fn serverless(_req: HttpRequest) -> HttpResponse {
    info!("** Read Stream Example **");

    let mut received_data = Vec::new();
    let mut chunk_count = 0;

    // URL to fetch from
    let fetch_uri = Uri::from_str("https://norvig.com/big.txt").unwrap();

    // Send an HTTP fetch request
    let fetch_res = HttpFetch::get(fetch_uri).send();

    match fetch_res {
        Ok(mut fetch_resp) => {
            // Open a read stream
            let read_stream = fetch_resp.get_read_stream();

            // Read the fetch response body chunk by chunk
            while let Some(chunk) = read_stream.read_chunk() {
                match chunk {
                    Ok(mut bytes) => {
                        // Record the chunks
                        received_data.append(&mut bytes);
                        chunk_count += 1;
                    }
                    Err(err) => {
                        let res = HttpResponse::from(format!(
                            "Failure in fetch request read_chunk(): {}",
                            err.to_string()
                        ))
                        .set_status(StatusCode::INTERNAL_SERVER_ERROR)
                        .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap());

                        return res;
                    }
                }
            }

            // Close the read stream
            if let Err(err) = read_stream.close() {
                let res = HttpResponse::from(format!("Failure in read stream close(): {}", err))
                    .set_status(StatusCode::INTERNAL_SERVER_ERROR)
                    .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap());

                return res;
            }
        }
        Err(err) => {
            let res =
                HttpResponse::from(format!("Failure in HTTP Fetch send(): {}", err.to_string()))
                    .set_status(StatusCode::INTERNAL_SERVER_ERROR)
                    .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap());

            return res;
        }
    }

    received_data.append(
        &mut format!("Received {} chunks", chunk_count)
            .as_bytes()
            .to_vec(),
    );

    let res = HttpResponse::from(received_data)
        .set_status(StatusCode::OK)
        .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap());

    return res;
}
