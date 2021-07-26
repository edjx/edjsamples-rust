<!--
title: .'SET value for a key on EDJX KV store'
description: 'Boilerplate code for Rust to set key on EDJX KV store'
platform: EDJX
language: Rust
-->

# Serverless Set value for Key on EDJX KV store Example

Boilerplate code to set key on EDJX KV store.

This example uses EDJX HttpRequest, HttpResponse and KV Store APIs.

This function is a basic demonstration of how to use the `kv::set` method to set the value in the EDJX P2P KV store. It expects the key and value to be sent as query param in the requst URL. The function checks for the error returned the library function and send corresponding http `BAD_REQUEST` response back to user.

Function URL : `{function_url}?key=some_key&value=some_value`
