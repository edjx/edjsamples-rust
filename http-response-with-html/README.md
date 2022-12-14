<!--
title: .'Sending HTML content as HTTP Response'
description: 'Boilerplate code to send HTML content as HTTP response'
platform: EDJX
language: Rust
-->

# HTTP Response with HTML Content

This function is a basic demonstration of how to use the different query parameters of the incoming client request. Based on the `page` query parameter, the function serves content for one of the defined HTML pages.

You can send a request to the function URL and then navigate using links on that HTML page to checkout the content of other pages.

####

Structure of the example website:

- HOME
  - ABOUT
  - SERVICES
  - CONTACT

URL to access the **About** page content: `{function_url}?page=about`
