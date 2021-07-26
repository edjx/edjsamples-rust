use edjx::{info, HttpRequest, HttpResponse, StatusCode};
use http::header::CONTENT_TYPE;

pub fn serverless(req: HttpRequest) -> HttpResponse {
  info!("**HTTP response with HTML function**");

  let page = match req.query_param_by_name("page") {
    Some(v) => v,
    None => "home".to_owned(),
  };

  let res = match page.as_str() {
    "about" => HttpResponse::from(ABOUT_HTML),
    "services" => HttpResponse::from(SERVICES_HTML),
    "contact" => HttpResponse::from(CONTACT_HTML),
    "home" | _ => HttpResponse::from(WEBSITE_HTML_HOME),
  };

  let res = res
    .set_status(StatusCode::OK)
    .set_header(CONTENT_TYPE, "text/html".parse().unwrap())
    .set_header("Serverless".parse().unwrap(), "EDJX".parse().unwrap());

  return res;
}

const WEBSITE_HTML_HOME: &'static str = r##"
<html lang="en" dir="ltr">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title> Home | Serverless Function | EDJX</title>
   </head>
<body>
  <nav>
    <div class="menu">
      <div class="logo">
        <a href="#">EDJX</a>
      </div>
      <ul>
        <li><a href="?page=home">Home</a></li>
        <li><a href="?page=about">About</a></li>
        <li><a href="?page=services">Services</a></li>
        <li><a href="?page=contact">Contact</a></li>
      </ul>
    </div>
  </nav>
  <div class="img"></div>
  <div class="center">
    <div class="title">Deploy Serverless  Functions @Edge</div>
    <div class="btns">
      <button>Learn More</button>
      <button>Subscribe</button>
    </div>
  </div>
</body>
</html>
"##;

const ABOUT_HTML: &'static str = r##"
<html lang="en" dir="ltr">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title> About | Serverless Function | EDJX</title>
   </head>
<body>
<h1>About</h1>
<p>If you ask a cloud company, they tell you the edge is their multi-billion dollar collection of server farms. A content delivery network (CDN) provider says it's their hundreds of points of presence. Wireless carriers will try to convince you it's their tens of thousands of macrocell and picocell sites.<p>
<p>At EDJX, we say the edge is anywhere and everywhere, a thousand feet away from you at all times. We believe computing needs to become ubiquitous, like electricity, to power billions of connected devices.</p>
</body>
</html>
"##;

const SERVICES_HTML: &'static str = r##"
<html lang="en" dir="ltr">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title> Services | Serverless Function | EDJX</title>
   </head>
<body>
  <nav>
    <div class="menu">
      <div class="logo">
        <a href="#">EDJX Services</a>
      </div>
      <ul>
        <li><a href="#">CDN</a></li>
        <li><a href="#">Serverless Computing</a></li>
        <li><a href="#">Edge Systems</a></li>
        <li><a href="#">Serverless DB</a></li>
      </ul>
    </div>
  </nav>
</body>
</html>
"##;

const CONTACT_HTML: &'static str = r##"
<html lang="en" dir="ltr">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title> Services | Serverless Function | EDJX</title>
   </head>
<body>
<div class="tn-atom" field="tn_text_1571256120374">
We're here answer all your questions, provide you with a live demo, or talk about your specific requirements. 
Drop us a note or give us a call.<br><br>
<strong>SALES</strong><br><strong> </strong>hello@edjx.io<br><br><strong>SUPPORT</strong>
<br>support@edjx.io<br><br><strong>HEADQUARTERS</strong><br><strong> </strong>
EDJX, Inc.<br>8601 Six Forks Road, Suite 400<br>Raleigh, NC 27615
</div>
</body>
</html>"##;
