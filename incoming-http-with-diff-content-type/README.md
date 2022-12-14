<!--
title: .'HTTP Request with different content types'
description: 'Boilerplate code to use different content types'
platform: EDJX
language: Rust
-->

# Incoming HTTP Request with Different Content Types

Boilerplate code to use incoming requests with different content types.

This example uses EDJX HttpRequest and HttpResponse APIs.

This function is a basic demonstration of how to use the body of the incoming client request for different content types. This example appends a key-value "Modified By" "Example Function" to the incoming body and sends the modified body in the response back to the client.

You can send a `POST` request to the function URL with following body types:

* `application/json`
* `text/plain`
* `application/x-www-form-urlencoded`
