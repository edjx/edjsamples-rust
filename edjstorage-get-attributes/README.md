<!--
title: .'Get attributes for a particular file on EDJX P2P Object Store'
description: 'Boilerplate code to get attributes associated with a file on Object Store'
platform: EDJX
language: Rust
-->

# Serverless Example to Get Attributes of a File on EDJX P2P Object Store

Boilerplate code to get attributes associated with a file on Object Store.

This example uses EDJX HttpRequest, HttpResponse, and Storage APIs.

This function is a basic demonstration of how to use the `storage::get_attributes` method to get attributes associated with the object from the EDJX P2P Object Store. The file name and bucket id must be sent as query parameters in the request URL. The function checks for errors returned by the library function and sends an HTTP response back to the client.

Function URL: `{function_url}?bucket_id=some_bucket_id&file_name=some_file_name`
