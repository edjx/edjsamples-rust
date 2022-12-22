<!--
title: .'Basic authentication in serverless'
description: 'Boilerplate code to implement basic authentication in functions'
platform: EDJX
language: Rust
-->

# Serverless Basic Authentication Example

Boilerplate code to implement basic authentication in functions.

This example demonstrates how to use third party `auth` providers to implement authentication in your serverless functions.

This function is a basic demonstration of how to use the HTTP request and HTTP fetch libraries in serverless code for the
EDJX platform. The function reads and verifies the `Authorization` header.

Replace the code in the `verify_auth_token` function to verify the `Authorization` header by your authentication service provider.
