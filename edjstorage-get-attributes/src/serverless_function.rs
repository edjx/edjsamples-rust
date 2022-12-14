use edjx::{error, info, storage, FileAttributes, HttpRequest, HttpResponse, StatusCode};

pub fn serverless(req: HttpRequest) -> HttpResponse {
    info!("New Req Framework For Set-Attributes Functionality");

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

    let res_bytes: FileAttributes = match storage::get_attributes(&bucket_id, &file_name) {
        Ok(r) => r,
        Err(e) => {
            return HttpResponse::from(e.to_string().as_str().to_owned())
                .set_status(e.to_http_status_code())
        }
    };

    info!("{}, {:?}", "Get Attributes Successful", res_bytes);
    let attr_res_bytes = serde_json::to_vec(&res_bytes).unwrap();

    let res = HttpResponse::from(attr_res_bytes).set_status(StatusCode::OK);

    return res;
}
