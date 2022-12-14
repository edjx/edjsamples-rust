use edjx::{info, FetchResponse, HttpFetch, HttpRequest, HttpResponse, Uri};
use http::header::CONTENT_TYPE;
use serde_json::json;
use std::str::FromStr;

const SENDGRID_API_KEY: &'static str = "SG.XXXXXXXXXXXXXXXXXXX.XXXXXXXXXXXXXXXXXXXXXX";

pub fn serverless(req: HttpRequest) -> HttpResponse {
  info!("**Send email using sendgrid function**");

  let message = match req.query_param_by_name("message") {
    Some(v) => v,
    None => "Default Message".to_owned(),
  };

  let subject = match req.query_param_by_name("subject") {
    Some(v) => v,
    None => "Default Subject".to_owned(),
  };

  let mut fetch_response = send_email(subject, message);

  let res = HttpResponse::from(fetch_response.read_body().unwrap())
    .set_status(*fetch_response.status_code())
    .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap());

  return res;
}

fn send_email(subject: String, message: String) -> FetchResponse {
  let send_uri = Uri::from_str("https://api.sendgrid.com/v3/mail/send").unwrap();

  let email_json = json!({"personalizations": [{"to": [{"email": "edjx@edjx.io"}]}],"from": {"email": "edjx@edjx.io"},"subject": subject,"content": [{"type": "text/plain", "value": message}]});

  let email_vec = serde_json::to_vec(&email_json).unwrap();
  HttpFetch::post(send_uri)
    .set_header(
      "Authorization".parse().unwrap(),
      ("Bearer ".to_string() + SENDGRID_API_KEY).parse().unwrap(),
    )
    .set_header(CONTENT_TYPE, "application/json".parse().unwrap())
    .with_binary_body(email_vec)
    .send()
    .unwrap()
}
