<!--
title: .'HTTP Fetch Request with different content types'
description: 'Boilerplate code to use different content types with http-fetch'
platform: EDJX
language: Rust
-->

# HTTP Fetch Request with different content types

Boilerplate code to use HTTP Fetch request with different content types.

This example uses EDJX HttpRequest, HttpFetch, HttpResponse APIs.

This function is a basic demonstration of how to construct the HTTP fetch request for different content types. This example appends a key-value (or a string) like "Requested By" and "Example Function" to the fetch body and set the received response as a response body to demonstrate the usecase.

You can try making the request to the function URL with following query params.

1. `?body_type=application/json`
2. `?body_type=text/plain`
3. `?body_type=application/x-www-form-urlencoded`
