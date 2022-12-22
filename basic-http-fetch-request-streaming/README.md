<!--
title: .'HTTP Fetch Request with Streaming'
description: 'Boilerplate code to stream a fetch request'
platform: EDJX
language: Rust
-->

# Serverless HTTP Fetch Example with Streaming

Boilerplate code that uses streaming to send body in HTTP Fetch.

This example uses EDJX HttpRequest, HttpResponse, and Fetch APIs.

This function is a basic demonstration of how to use the libraries in serverless code on the EDJX platform.

This function uses the Streaming Fetch API to send a stream of data to a remote server. The function records the response of the remote server and sends the response to the client.
To demonstrate that the chunk size can vary, the function starts with a chunk size of 1 byte and increments the chunk size with each successive chunk.
