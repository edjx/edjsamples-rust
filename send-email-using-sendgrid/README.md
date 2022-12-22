<!--
title: .'Send Email using http fetch'
description: 'Boilerplate code to send email using third party service'
platform: EDJX
language: Rust
-->

# Send Email Using the Fetch API

Boilerplate code to send an email alert.

This example uses EDJX HttpRequest, HttpResponse, and Fetch APIs.

This function is a basic demonstration of how to use third-party email HTTP APIs to send email alerts using functions. It takes the message and the subject of the email as a query parameter to the function, and uses them in a request to a third-party service's HTTP endpoint, which then sends the email.

Replace the `SENDGRID_API_KEY` with your own key to use this example.

Similarly, HTTP API webhooks can be used to integrate with any third-party systems (like Slack, email, GitHub, Twilio, etc.)
