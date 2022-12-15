use edjx::{info, HttpMethod, HttpRequest, HttpResponse, StatusCode};
use http::header::CONTENT_TYPE;
use serde_json::{json, Value};
use std::str;
use url::form_urlencoded;

pub fn serverless(req: &mut HttpRequest) -> HttpResponse {
    info!("**Incoming HTTP with diff content type function**");

    match req.method() {
        HttpMethod::POST => {
            let content_type_header = match req.headers().get(CONTENT_TYPE) {
                Some(content_type) => content_type.to_str().unwrap(),
                None => "",
            };

            match content_type_header {
                "application/json" => {
                    let incoming_req_body = req.body().unwrap().as_ref().unwrap();
                    let body_str = std::str::from_utf8(incoming_req_body).unwrap();

                    let parsed = serde_json::from_str(&body_str);
                    if parsed.is_err() {
                        return HttpResponse::from("Request body must be a valid JSON")
                            .set_status(StatusCode::BAD_REQUEST);
                    }

                    let mut json_value: Value = parsed.unwrap();
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

                    let form_params = form_urlencoded::parse(incoming_req_body);

                    let outgoing_body = form_urlencoded::Serializer::new(String::new())
                        .append_pair("Modified By", "Example Function")
                        .extend_pairs(form_params)
                        .finish();

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
