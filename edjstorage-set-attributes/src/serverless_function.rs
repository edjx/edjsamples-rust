use edjx::{
    error, info, storage, FileAttributes, HttpRequest, HttpResponse, StatusCode, StorageResponse,
};
use std::collections::HashMap;

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

    // Set Meta data properties for any file as its PUT request so existing-metadata(if any) will override with this metadata
    let mut properties: HashMap<String, String> = HashMap::new();

    properties.insert("Content-Type".to_owned(), "image/jpeg".to_owned());
    properties.insert("Cache-Control".to_owned(), "no-cache".to_owned());

    let new_attributes = FileAttributes::new(Some(properties), None);

    // call edjlib::storage::set_attributes function to update metadata for content
    let _put_res: StorageResponse =
        match storage::set_attributes(&bucket_id, &file_name, new_attributes) {
            Ok(r) => r,
            Err(e) => {
                return HttpResponse::from(e.to_string().as_str().to_owned())
                    .set_status(e.to_http_status_code())
            }
        };
    info!("Set Attributes Successful");

    let res = HttpResponse::from("Success").set_status(StatusCode::OK);

    return res;
}
