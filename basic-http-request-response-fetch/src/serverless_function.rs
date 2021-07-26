use edjx::{error, info, FetchResponse, HttpFetch, HttpRequest, HttpResponse, StatusCode, Uri};
use std::str::FromStr;

pub fn serverless(_req: HttpRequest) -> HttpResponse {
    info!("**Basic HTTP request and response function**");

    let fetch_uri = Uri::from_str("https://httpbin.org/get").unwrap();

    let mut fetch_res: FetchResponse = match HttpFetch::get(fetch_uri).send() {
        Ok(resp) => resp,
        Err(e) => {
            error!("{}", &e.to_string());
            return HttpResponse::from(format!(
                "{} : {}",
                "failure in fetch req".to_owned(),
                &e.to_string()
            ))
            .set_status(StatusCode::BAD_REQUEST);
        }
    };

    let res = HttpResponse::from(fetch_res.body())
        .set_status(StatusCode::OK)
        .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap());

    return res;
}
