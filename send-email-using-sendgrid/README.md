<!--
title: .'Send Email using http fetch'
description: 'Boilerplate code to send email using third party service'
platform: EDJX
language: Rust
-->

# Send email using Fetch

Boilerplate code for rust to use serverless to send email alert.

This example uses EDJX HttpRequest, HttpResponse and Fetch APIs.

This function is a basic demonstration of how to use third party email HTTP APIs to send email alerts using functions. It takes message and subject of the email as a query parameter to function which will be used in the request to third party service's http endpoint to send email.


Replace the `SENDGRID_API_KEY` with your own key to use this example.

Like same way, HTTP API webhooks can be used to integrate with any third party systems (like Slack, email, github, Twilio etc.)
