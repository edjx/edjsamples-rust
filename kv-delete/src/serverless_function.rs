use edjx::{error, info, kv, kv::KVError, HttpRequest, HttpResponse, StatusCode};

pub fn serverless(req: HttpRequest) -> HttpResponse {
    info!("Inside KV delete example function");

    match req.query_param_by_name("key") {
        Some(key) => match kv::delete(&key) {
            Err(e) => {
                let status = match e {
                    KVError::Unknown => StatusCode::BAD_REQUEST,
                    KVError::UnAuthorized => StatusCode::UNAUTHORIZED,
                    KVError::NotFound => StatusCode::NOT_FOUND,
                };

                HttpResponse::from(e.to_string()).set_status(status)
            }
            Ok(_) => HttpResponse::from("Value succesfully deleted").set_status(StatusCode::OK),
        },
        None => {
            error!("No key provided in user request");
            HttpResponse::from("No key provided in user request")
                .set_status(StatusCode::BAD_REQUEST)
        }
    }
}
