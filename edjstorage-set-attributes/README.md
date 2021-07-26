<!--
title: .'Set/Update attributes for a particular file on EDJX P2P Object Store'
description: 'Boilerplate code to set attributes for a file on Object Store'
platform: EDJX
language: Rust
-->

# Serverless Set/Update attributes for a particular file on EDJX P2P Object Store Example

Boilerplate code to set attributes associated with a file on Object Store.

This example uses EDJX HttpRequest, HttpResponse and Storage APIs.

This function is a basic demonstration of how to use the `storage::set_attributes` method to set associated attributes with the object on the EDJX P2P Object Store. It expects the file name and bucket id to be sent as query param in the requst URL. The function checks for the error returned the library function and send corresponding HTTP status response back to user.

Function URL : `{function_url}?bucket_id=some_bucket_id&file_name=some_file_name`
