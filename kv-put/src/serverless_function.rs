use edjx::{error, info, kv, HttpRequest, HttpResponse, StatusCode};
use std::time::Duration;

pub fn serverless(req: HttpRequest) -> HttpResponse {
    info!("Inside KV put example function");

    let key = req.query_param_by_name("key").unwrap();
    match req.query_param_by_name("value") {
        Some(value) => match kv::put(&key, value, Some(Duration::from_secs(1000 * 5 * 60))) {
            Err(e) => {
                return HttpResponse::from(format!("{}", e)).set_status(StatusCode::BAD_REQUEST)
            }
            Ok(_) => HttpResponse::from("Value successfully inserted").set_status(StatusCode::OK),
        },
        None => {
            error!("No value provided in user request");
            HttpResponse::from("No value provided in user request")
                .set_status(StatusCode::BAD_REQUEST)
        }
    }
}
