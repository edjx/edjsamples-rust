<!--
title: .'HTTP Request different method'
description: 'Boilerplate code to use different methods in request'
platform: EDJX
language: Rust
-->

# Serverless Example for HTTP Client Request with Different Methods

Boilerplate code to use HTTP request with different methods.

This example uses EDJX HttpRequest, HttpResponse, and Fetch APIs.

This function is a basic demonstration of how to check the HTTP method of the incoming client request. This example makes an HTTP fetch request to a specific URL of the `https://httpbin.org/` service depending upon the incoming method type. It then returns the response sent by the service.

You can send a request to the function URL with the following different method types. 

**Note**: Response will be populated with the response received from `https://httpbin.org`. It will contain the parameters used to make the request from the function.

---

#### For the `PUT` method, the function makes a PUT request to the endpoint. The response looks like the following text:

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
