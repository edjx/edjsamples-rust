use edjx::{
    error, info, storage, HeaderMap, HeaderName, HttpRequest, HttpResponse, StatusCode,
    StorageResponse,
};

pub fn serverless(req: HttpRequest) -> HttpResponse {
    info!("**Storage get with http function**");

    // 1. param (required): "file_name" -> name that will be given to the uploaded content
    let file_name = match req.query_param_by_name("file_name") {
        Some(v) => v,
        None => {
            error!("No file_name found in query params of request");
            return HttpResponse::from("No file name found in query params of request")
                .set_status(StatusCode::BAD_REQUEST);
        }
    };

    // 2. param (required): "bucket_id" -> in which bucket content will be uploaded
    let bucket_id = match req.query_param_by_name("bucket_id") {
        Some(v) => v,
        None => {
            error!("No bucket id found in query params of request");
            return HttpResponse::from("No bucket id found in query params of request")
                .set_status(StatusCode::BAD_REQUEST);
        }
    };
    
    let mut storage_res: StorageResponse = match storage::get(&bucket_id, &file_name) {
        Ok(r) => r,
        Err(e) => return HttpResponse::from(e.to_string()).set_status(e.to_http_status_code()),
    };
    info!("Get Content Successful");

    let headers = storage_res.headers();
    let mut headers_n = HeaderMap::new();
    for (key, value) in headers {
        headers_n.insert(
            HeaderName::from_bytes(key.as_bytes()).unwrap(),
            value.parse().unwrap(),
        );
    }

    let mut res = HttpResponse::from(storage_res.read_body().unwrap());

    let o_headers = res.headers_mut();
    *o_headers = headers_n;

    let res = res
        .set_status(StatusCode::OK)
        .set_header(http::header::CONTENT_TYPE, "text/plain".parse().unwrap());
    return res;
}
