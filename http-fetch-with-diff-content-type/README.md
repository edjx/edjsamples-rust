<!--
title: .'HTTP Fetch Request with different content types'
description: 'Boilerplate code to use different content types with http-fetch'
platform: EDJX
language: Rust
-->

# HTTP Fetch Request with Different Content Types

Boilerplate code to use HTTP Fetch request with different content types.

This example uses EDJX HttpRequest, HttpFetch, and HttpResponse APIs.

This function is a basic demonstration of how to construct the HTTP fetch request for different content types. This example sends a key-value **Requested By**: **Example Function** in a fetch body encoded using the requested encoding. The function then sends the received response back in a response body to demonstrate the use case.

Use the following query parameters to make a request to the function URL:

* `?body_type=application/json`
* `?body_type=text/plain`
* `?body_type=application/x-www-form-urlencoded`
