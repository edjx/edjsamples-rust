<!--
title: .'Get value of key from EDJX KV store'
description: 'Boilerplate code to get Key from EDJX KV store'
platform: EDJX
language: Rust
-->

# Get value of Key from EDJX KV store Example

Boilerplate code to get Key from EDJX KV store.

This example uses EDJX HttpRequest, HttpResponse and KV Store APIs.

This function is a basic demonstration of how to use the `kv::get` method to fetch the value set in the global P2P KV store of EDJX. It expects the key to be sent as query param in the requst URL. The function checks for the error returned the library function and send corresponding http response back to user.

Function URL : `{function_url}?key=some_key`
