use edjx::{info, HttpMethod, HttpRequest, HttpResponse, StatusCode};
use http::header::CONTENT_TYPE;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::str;
use url::form_urlencoded;

pub fn serverless(req: &mut HttpRequest) -> HttpResponse {
    info!("**Incoming HTTP with diff content type function**");

    match req.method() {
        HttpMethod::POST => {
            let content_type_header = req.headers().get(CONTENT_TYPE).unwrap().to_str().unwrap();

            match content_type_header {
                "application/json" => {
                    let incoming_req_body = req.body().unwrap().as_ref().unwrap();
                    let body_str = std::str::from_utf8(incoming_req_body).unwrap();

                    let mut json_value: Value = serde_json::from_str(&body_str).unwrap();
                    if json_value.is_object() {
                        let mutable_obj = json_value.as_object_mut().unwrap();
                        mutable_obj.insert("Modified By".to_string(), json!("Example function"));
                    }

                    let outgoing_body = serde_json::to_string(&json_value).unwrap();

                    let res = HttpResponse::from(outgoing_body).set_status(StatusCode::OK);
                    return res
                        .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap())
                        .set_header(CONTENT_TYPE, "application/json".parse().unwrap());
                }

                "text/plain" => {
                    let incoming_req_body = req.body().unwrap().as_ref().unwrap();
                    let body_str = str::from_utf8(incoming_req_body).unwrap();

                    let outgoing_body = "Modified By : Example Function ".to_owned() + body_str;

                    let res = HttpResponse::from(outgoing_body).set_status(StatusCode::OK);
                    return res
                        .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap())
                        .set_header(CONTENT_TYPE, "text/plain".parse().unwrap());
                }

                "application/x-www-form-urlencoded" => {
                    let incoming_req_body = req.body().unwrap().as_ref().unwrap();

                    let mut form_params: HashMap<String, String> =
                        form_urlencoded::parse(incoming_req_body)
                            .into_owned()
                            .collect();
                    form_params.insert("Modified By".to_owned(), "Example Function".to_owned());

                    let outgoing_body = serde_json::to_string(&form_params).unwrap();

                    let res = HttpResponse::from(outgoing_body).set_status(StatusCode::OK);
                    return res
                        .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap())
                        .set_header(
                            CONTENT_TYPE,
                            "application/x-www-form-urlencoded".parse().unwrap(),
                        );
                }

                _ => {
                    return HttpResponse::new().set_status(StatusCode::BAD_REQUEST);
                }
            }
        }
        _ => {
            return HttpResponse::new().set_status(StatusCode::METHOD_NOT_ALLOWED);
        }
    };
}
