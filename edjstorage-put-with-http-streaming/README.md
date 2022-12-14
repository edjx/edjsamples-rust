<!--
title: .'Upload Content to a Bucket on EDJX P2P Object Store Using Streaming'
description: 'Boilerplate code to upload content on EDJX P2P Object Store using streaming'
platform: EDJX
language: Rust
-->

# Serverless Example to Upload Object on EDJX P2P Object Store

Boilerplate code to upload content on the EDJX P2P Object Store using Streaming.

This example uses EDJX HttpRequest, HttpResponse, and Storage APIs.

This function is a basic demonstration of how to use the `storage::put_streaming` method to stream data into an object in the EDJX P2P Object Store. The file name, bucket id, and optionally also properties must be sent as query parameters in the request URL. The function uses a default content to be uploaded to the object store. The function checks for errors returned the library function and sends a corresponding HTTP status response back to the client.

Function URL: `{function_url}?bucket_id=some_bucket_id&file_name=some_file_name&properties=SOME_KEY=SOME_VALUE`
