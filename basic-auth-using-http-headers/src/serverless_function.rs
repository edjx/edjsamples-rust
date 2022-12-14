use edjx::{info, HttpRequest, HttpResponse, StatusCode};

pub fn serverless(req: HttpRequest) -> HttpResponse {
    info!("**Basic Auth using http headers function**");

    let auth_header_value = match req.headers().get("Authorization") {
        Some(a) => a,
        None => {
            return HttpResponse::from(format!("Authentication Error : No token present"))
                .set_status(StatusCode::UNAUTHORIZED)
        }
    };

    if !verify_auth_token(auth_header_value.to_str().unwrap()) {
        return HttpResponse::from(format!("Authentication Error : Invalid auth token"))
            .set_status(StatusCode::UNAUTHORIZED);
    }

    let res = HttpResponse::new()
        .set_status(StatusCode::OK)
        .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap());

    return res;
}

fn verify_auth_token(_token: &str) -> bool {
    // Sample Code to Implement `verify_auth_token`
    // let token = _token.replace("Bearer ", "");
    // let fetch_uri =
    //     Uri::from_str(&("https://someauth.com/token/verify/".to_string() + &token)).unwrap();

    // match HttpFetch::get(fetch_uri).send() {
    //     Ok(resp) => resp.status_code() == &StatusCode::OK,
    //     Err(_) => {
    //         return false;
    //     }
    // }

    return true;
}
