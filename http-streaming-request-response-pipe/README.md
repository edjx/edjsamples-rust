<!--
title: .'EDJX HTTP Request and Response Streaming with a Pipe'
description: 'Boilerplate code to use show HTTP Response Streaming and Streaming Pipes'
platform: EDJX
language: Rust
-->

# Serverless Http Request, Response, and Fetch Example with Stream Piping

Boilerplate code to use EDJX Request and Response Streaming, and Stream Pipes.

This example uses EDJX HttpRequest and HttpResponse with streaming.

This function is a basic demonstration of how to use the libraries in serverless code for the EDJX platform.

This function reads chunks of streamed data from the client and immediately echoes the chunks back to the client. This sample function is similar to the `http-streaming-request-response` sample function with the exception that this function uses the stream `pipe_to()` method to pipe the read and write streams.
