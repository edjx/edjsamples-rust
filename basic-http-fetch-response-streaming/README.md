<!--
title: .'HTTP Fetch Response with Streaming'
description: 'Boilerplate code to stream a fetch response'
platform: EDJX
language: Rust
-->

# Serverless HTTP Fetch Example with Streaming

Boilerplate code that uses streaming to receive a body from HTTP Fetch.

This example uses EDJX HttpRequest, HttpResponse, and Fetch APIs.

This function is a basic demonstration of how to use the libraries in serverless code on the EDJX platform.

This function sends an HTTP Fetch request to a remote URL that hosts a large file. The function then reads the file from the HTTP Fetch response chunk-by-chunk and sends all the received data at once, along with the chunk count, to the client.
