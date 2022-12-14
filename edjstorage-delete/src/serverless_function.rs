use edjx::{storage,info, error, HttpRequest, HttpResponse, StatusCode, StorageResponse};

pub fn serverless(req: HttpRequest) -> HttpResponse {
    info!("Content Delete Flow");

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

    let  _res_bytes: StorageResponse = match storage::delete(&bucket_id, &file_name) {
        Ok(r) => r,
        Err(e) => {
            error!("Content Deletion Failed");
            return HttpResponse::from(e.to_string().as_str().to_owned())
                .set_status(e.to_http_status_code());
        }
    };

    let res = HttpResponse::from("Success").set_status(StatusCode::OK);
    info!("Content Deletion Successful");

    return res;
}
