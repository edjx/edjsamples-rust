<!--
title: .'HTTP Request different method'
description: 'Boilerplate code to use different methods in request'
platform: EDJX
language: Rust
-->

# Serverless HTTP client Request different methods Example

Boilerplate code to use HTTP request with different methods.

This example uses EDJX HttpRequest, HttpResponse and Fetch APIs.

This function is a basic demonstration of how to check for the method of the incoming client request and handle accordingly. This example makes a http fetch request to a specific url of this domain `https://httpbin.org/` depending upon the incoming method type and return the response sent by the above mentioned domain.

You can try making the request to the function URL with following different method types. Response will be populated with the respective parameters used to make the request from the function.

---

#### For `PUT` method, function makes PUT request to endpoint. Response is :

```{
    "args": {},
    "data": "",
    "files": {},
    "form": {},
    "headers": {
        "Accept": "*/*",
        "Accept-Encoding": "gzip, deflate, br",
        "Cache-Control": "no-cache",
        "Content-Length": "0",
        "Host": "httpbin.org",
        "Postman-Token": "",
        "User-Agent": "PostmanRuntime/7.26.8",
        "X-Amzn-Trace-Id": "Root=1-60e3dd9e-1njnxwn22jnj2nj2"
    },
    "json": null,
    "origin": "xx.xx.xx.xx",
    "url": "https://httpbin.org/put"
}
```
