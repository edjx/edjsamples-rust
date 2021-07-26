use edjx::{error, info, kv, kv::KVError, HttpRequest, HttpResponse, StatusCode};

pub fn serverless(req: HttpRequest) -> HttpResponse {
    info!("Inside KV get example function");

    match req.query_param_by_name("key") {
        Some(key) => {
            let val = match kv::get(&key) {
                Err(e) => {
                    let status = match e {
                        KVError::Unknown => StatusCode::BAD_REQUEST,
                        KVError::UnAuthorized => StatusCode::UNAUTHORIZED,
                        KVError::NotFound  => StatusCode::NOT_FOUND,
                    };

                    return HttpResponse::from(e.to_string()).set_status(status);
                }
                Ok(val) => val,
            };
            HttpResponse::from(std::str::from_utf8(val.as_ref()).unwrap())
                .set_status(StatusCode::OK)
        }
        None => {
            error!("No key provided in user request");
            HttpResponse::from("No key provided in user request")
                .set_status(StatusCode::BAD_REQUEST)
        }
    }
}
