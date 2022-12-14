<!--
title: .'Get value of key from EDJX KV store'
description: 'Boilerplate code to get Key from EDJX KV store'
platform: EDJX
language: Rust
-->

# Get the Value of a Key from the EDJX KV Store

Boilerplate code to get Key from EDJX KV Store.

This example uses EDJX HttpRequest, HttpResponse, and KV Store APIs.

This function is a basic demonstration of how to use the `kv::get` method to fetch the value set in the global P2P KV Store of EDJX. The key must be sent as query parameter in the request URL. The function checks for errors returned by the library function and sends an HTTP response back to the client.

Function URL: `{function_url}?key=some_key`
