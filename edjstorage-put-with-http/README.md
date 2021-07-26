<!--
title: .'Upload content on a particular bucket on EDJX P2P Object Store'
description: 'Boilerplate code to upload content on EDJX P2P Object Store'
platform: EDJX
language: Rust
-->

# Serverless Upload object on EDJX's P2P Object Store Example

Boilerplate code to upload content on EDJX P2P Object Store.

This example uses EDJX HttpRequest, HttpResponse and Storage APIs.

This function is a basic demonstration of how to use the `storage::put` method to upload the object on the EDJX P2P Object Store. It expects the file name, properties and bucket id to be sent as query params in the requst URL and uses a default content to be uploaded to the store. The function checks for the error returned the library function and send corresponding HTTP status response back to user.

Function URL : `{function_url}?bucket_id=some_bucket_id&file_name=some_file_name&properties=SOME_KEY=SOME_VALUE`
