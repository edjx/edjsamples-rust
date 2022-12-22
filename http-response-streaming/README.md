<!--
title: .'EDJX HTTP Response Streaming'
description: 'Boilerplate code to send HTML content as HTTP response'
platform: EDJX
language: Rust
-->

# Streamed HTTP Response

Boilerplate code to use EDJX Response Streaming.

This example uses EDJX HttpRequest and HttpResponse.

This function is a basic demonstration of how to use the above libraries in serverless code for the EDJX platform.

When this function receives a request from a client, it sends back a stream of 10,000 chunks with the text 0/10000, 1/10000, etc.
