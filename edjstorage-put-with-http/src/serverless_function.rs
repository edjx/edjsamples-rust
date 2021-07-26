use edjx::{error, info, storage, HttpRequest, HttpResponse, StatusCode, StorageResponse};

pub fn serverless(req: HttpRequest) -> HttpResponse {
    info!("**Storage put with http function**");

    // 1.Param(Required) : "file_name" -> name that will be given to uploading content
    let file_name = match req.query_param_by_name("file_name") {
        Some(v) => v,
        None => {
            error!("No file_name found in query params of request");
            return HttpResponse::from("No file name found in query params of request")
                .set_status(StatusCode::BAD_REQUEST);
        }
    };

    // 2.Param(Required) : "bucket_id" ->  in which content will be uploaded
    let bucket_id = match req.query_param_by_name("bucket_id") {
        Some(v) => v,
        None => {
            error!("No bucket id found in query params of request");
            return HttpResponse::from("No bucket id found in query params of request")
                .set_status(StatusCode::BAD_REQUEST);
        }
    };

    // 3.Param : buf_data (content bytes to be uploaded)
    let buf_data = b"Sample data for example upload of storage put";

    //format as properties cache-control=true,a=b
    let properties = match req.query_param_by_name("properties") {

        Some(v) => v,
        None => "".to_owned(),
    };

    let _put_res: StorageResponse =
        match storage::put(&bucket_id, &file_name, &properties, &buf_data.as_ref()) {
            Ok(r) => r,
            Err(e) => return HttpResponse::from(e.to_string()).set_status(e.to_http_status_code()),
        };
    info!("Put Content Successful");

    let res = HttpResponse::from("Success").set_status(StatusCode::OK);

    return res;
}
