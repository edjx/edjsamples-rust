<!--
title: .'Delete particular file on a particular bucket on EDJX P2P Object Store'
description: 'Boilerplate code to delete content on EDJX P2P Object Store'
platform: EDJX
language: Rust
-->

# Serverless Delete object from EDJX's P2P Object Store Example

Bßoilerplate code to delete content from EDJX P2P Object Store.

This example uses EDJX HttpRequest, HttpResponse and Storage APIs.

This function is a basic demonstration of how to use the `storage::delete` method to get the object from the EDJX P2P Object Store. It expects the file name and bucket id to be sent as query param in the requst URL. The function checks for the error returned the library function and send corresponding HTTP status response back to user otherwise it sends "Success" to the response.

Function URL : `{function_url}?bucket_id=some_bucket_id&file_name=some_file_name`
