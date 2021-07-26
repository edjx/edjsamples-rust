<!--
title: .'HTTP Request with different content types'
description: 'Boilerplate code to use different content types'
platform: EDJX
language: Rust
-->

# Incoming HTTP Request with different content types

Boilerplate code to use incoming request with different content types.

This example uses EDJX HttpRequest, HttpResponse APIs.

This function is a basic demonstration of how to use the body of the incoming client request for different content types. This example appends a key-value (or a string) like "Modified By" and "Example Function" to the incoming body and set this modified body in the response body to demonstrate the usecase.

You can try making the POST request to the function URL with following body types.

1. `application/json`
2. `text/plain`
3. `application/x-www-form-urlencoded`
