<!--
title: .'HTTP Fetch Response with Stream Piping'
description: 'Boilerplate code to use pipe(using streams) Http Respons from Fetch Read'
platform: EDJX
language: Rust
-->

# Serverless HTTP Fetch Example with Stream Piping

Boilerplate code to pipe a stream from Fetch response into HTTP Response.

This example uses EDJX HttpRequest, HttpResponse, and Fetch APIs with streaming.

This function is a basic demonstration of how to use the libraries in serverless code on the EDJX platform.

This function fetches a large file from a remote URL, reads the file chunk by chunk, and forwards each chunk to the client using the `pipe_to()` method.
