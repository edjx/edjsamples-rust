mod serverless_function;
use edjx::{error, HttpRequest, HttpResponse, StatusCode};

#[no_mangle]
pub fn init() {
    let req = match HttpRequest::from_client() {
        Ok(val) => val,
        Err(e) => {
            error!("{}", e.to_string().as_str());
            HttpResponse::new()
                .set_status(StatusCode::BAD_REQUEST)
                .send()
                .unwrap();
            return;
        }
    };

    crate::serverless_function::serverless(req);
}
