<!--
title: .'Upload Content to a Bucket on EDJX P2P Object Store Using Streaming'
description: 'Boilerplate code to upload content on EDJX P2P Object Store using streaming'
platform: EDJX
language: Rust
-->

# Serverless Example to Upload Object on EDJX P2P Object Store

Boilerplate code to upload content on the EDJX P2P Object Store using Streaming.

This example uses EDJX HttpRequest, HttpResponse, and Storage APIs.

This function is a basic demonstration of how to use the `storage::put_streaming` method to stream data into an object in the EDJX P2P Object Store. The file name, bucket id, and optionally also properties must be sent as query parameters in the request URL. The function expects the body of the request to contain a stream of data. The function uploads the received data to the object store. The function streams information about the chunk upload progress, as well as the final library response, back to the client.

Function URL: `{function_url}?bucket_id=some_bucket_id&file_name=some_file_name&properties=SOME_KEY=SOME_VALUE`
