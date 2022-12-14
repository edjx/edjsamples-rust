use edjx::{error, info, HttpFetch, HttpMethod, HttpRequest, HttpResponse, StatusCode, Uri};
use std::str::FromStr;

pub fn serverless(req: HttpRequest) -> HttpResponse {
    info!("**Incoming HTTP request method function**");

    let (fetch_method, fetch_uri) = match req.method() {
        HttpMethod::GET => (
            http::Method::GET,
            Uri::from_str("https://httpbin.org/get").unwrap(),
        ),
        HttpMethod::POST => (
            http::Method::POST,
            Uri::from_str("https://httpbin.org/post").unwrap(),
        ),
        HttpMethod::PUT => (
            http::Method::PUT,
            Uri::from_str("https://httpbin.org/put").unwrap(),
        ),
        HttpMethod::DELETE => (
            http::Method::DELETE,
            Uri::from_str("https://httpbin.org/delete").unwrap(),
        ),
        HttpMethod::PATCH => (
            http::Method::PATCH,
            Uri::from_str("https://httpbin.org/patch").unwrap(),
        ),
        _ => {
            return HttpResponse::new().set_status(StatusCode::METHOD_NOT_ALLOWED);
        }
    };

    let mut fetch_res = match HttpFetch::new(fetch_uri, fetch_method)
        .set_header(
            "accept".parse().unwrap(),
            "application/json".parse().unwrap(),
        )
        .send()
    {
        Ok(resp) => resp,
        Err(e) => {
            error!("{}", &e.to_string());
            return HttpResponse::from(format!(
                "{} : {}",
                "failure in fetch req for given method".to_owned(),
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
