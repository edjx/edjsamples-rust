use bytes::Bytes;
use edjx::{error, info, FetchResponse, HttpFetch, HttpRequest, Uri};
use http::Response;
use std::str::FromStr;

pub fn serverless(_req: HttpRequest) -> Response<Option<Bytes>> {
    info!("**Basic HTTP request and response using http std library function**");

    let fetch_uri = Uri::from_str("https://httpbin.org/get").unwrap();

    let req_builder = http::request::Builder::new()
        .method(http::Method::GET)
        .uri(&fetch_uri);
    let req = req_builder.body(None).unwrap();

    let mut fetch_res: FetchResponse = match HttpFetch::send_using_standard_http_lib(req) {
        Ok(resp) => resp,
        Err(e) => {
            error!("{}", &e.to_string());
            return http::Response::builder()
                .header("Serverless", "EDJX")
                .status(400)
                .body(Some(Bytes::from(format!(
                    "{} : {}",
                    "failure in fetch  req".to_owned(),
                    &e.to_string()
                ))))
                .unwrap();
        }
    };

    let res = http::Response::builder()
        .header("Serverless", "EDJX")
        .body(Some(Bytes::from(fetch_res.body())))
        .unwrap();

    return res;
}
