use edjx::{error, info, HttpFetch, HttpRequest, HttpResponse, StatusCode, Uri};
use http::header::CONTENT_TYPE;
use serde_json::Value;
use std::str::FromStr;

pub fn serverless(req: &mut HttpRequest) -> HttpResponse {
    info!("**HTTP Fetch with diff content type function**");

    let body_type = match req.query_param_by_name("body_type") {
        Some(v) => v,
        None => "application/json".to_owned(),
    };

    let body = match body_type.as_str() {
        "application/json" => {
            let data = r#"
        {
            "Requested By": "Example Function"
        }"#;

            let value: Value = serde_json::from_str(data).unwrap();
            serde_json::to_vec(&value).unwrap()
        }

        "text/plain" => "Requested By : Example Function".as_bytes().to_vec(),

        "application/x-www-form-urlencoded" => {
            let data = &[("Requested By", "Example Function")];
            serde_urlencoded::to_string(data)
                .unwrap()
                .as_bytes()
                .to_vec()
        }
        _ => {
            return HttpResponse::new().set_status(StatusCode::BAD_REQUEST);
        }
    };

    let fetch_uri = Uri::from_str("https://httpbin.org/post").unwrap();

    let mut fetch_res = match HttpFetch::post(fetch_uri)
        .with_binary_body(body)
        .set_header(CONTENT_TYPE, body_type.parse().unwrap())
        .send()
    {
        Ok(resp) => resp,
        Err(e) => {
            error!("{}", &e.to_string());
            return HttpResponse::from(format!(
                "{} : {}",
                "failure in fetch req for given body type".to_owned(),
                &e.to_string()
            ))
            .set_status(StatusCode::BAD_REQUEST);
        }
    };

    let body = match fetch_res.read_body() {
        Ok(b) => b,
        Err(err) => return HttpResponse::from(format!(
            "Failed to retrieve fetch response body: {}",
            err.to_string()
        )).set_status(StatusCode::INTERNAL_SERVER_ERROR)
    };
    let mut res = HttpResponse::from(body).set_status(StatusCode::OK);

    let headers = res.headers_mut();
    *headers = fetch_res.headers().clone();

    let res = res.set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap());

    return res;
}
