<!--
title: .'Sending HTML content as HTTP Response'
description: 'Boilerplate code to send HTML content as HTTP response'
platform: EDJX
language: Rust
-->

# HTTP Response with HTML content

This function is a basic demonstration of how to use the different query params of the incoming client request and send corresponding HTML content accordingly. The same Serverless Function code serves the content for 4 pages.

You can try making the request to the function URL and then navigate using links on that html page to cheeckout content of other pages.

####

Structure of example website

- HOME
  - ABOUT
  - SERVICES
  - CONTACT

URL to access the "About" page content : `{function_url}?page=about`
