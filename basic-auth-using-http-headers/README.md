<!--
title: .'Basic authentication in serverless'
description: 'Boilerplate code to implement basic authentication in the functions'
platform: EDJX
language: Rust
-->

# Serverless Basic Authentication Example

Boilerplate code to implement basic authentication in the functions.

This example demonstrates how to use third party `auth` providers to implement authentication in your serverless functions.

This function is a basic demonstration of how to use the http request and fetch libraries in serverless code for
EDJX platform to read "Authorization" header and then parse it to send and verify using fetch request from third party URL.

Replace the code in `verify_auth_token` function to satisfy the constarints of your auth service provider.
