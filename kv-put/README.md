<!--
title: .'SET value for a key on EDJX KV store'
description: 'Boilerplate code to set key on EDJX KV store'
platform: EDJX
language: Rust
-->

# Serverless Set Value for Key on EDJX KV Store Example

Boilerplate code to store a key-value pair on the EDJX KV Store.

This example uses EDJX HttpRequest, HttpResponse, and KV Store APIs.

This function is a basic demonstration of how to use the `kv::set` method to set the value in the EDJX P2P KV store. The key and value must be sent as query parameter in the request URL. The function checks for errors returned by the library function and sends an HTTP response back to the client.

Function URL: `{function_url}?key=some_key&value=some_value`
