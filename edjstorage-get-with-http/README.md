<!--
title: .'Download particular file on a particular bucket on EDJX P2P Object Store'
description: 'Boilerplate code to download content on EDJX P2P 
platform: EDJX
language: Rust
-->

# Serverless Example to Download Object From EDJX P2P Object Store

Boilerplate code to download content from EDJX P2P Object Store.

This example uses EDJX HttpRequest, HttpResponse, and Storage APIs.

This function is a basic demonstration of how to use the `storage::get` method to get the Object from the EDJX P2P Object Store. The file name and bucket id must be sent as query parameters in the request URL. The function checks for errors returned by the library function and sends 
a **Success** message or the corresponding HTTP status code in a response back to the client.

Function URL: `{function_url}?bucket_id=some_bucket_id&file_name=some_file_name`
