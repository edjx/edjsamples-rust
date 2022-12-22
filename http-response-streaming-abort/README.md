<!--
title: .'Abort EDJX HTTP Response Streaming'
description: 'Boilerplate code to send HTML content as HTTP response'
platform: EDJX
language: Rust
-->

# Abort a Streamed HTTP Response

Code that tests the `abort()` function in EDJX Response Streaming.

This example uses EDJX **HttpRequest** and **HttpResponse**.

This function is a basic demonstration of how to use the libraries in serverless code on the EDJX platform.

This function is similar to the `http-response-streaming` sample function except that this function aborts the write stream after 10 chunks are sent.
