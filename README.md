<!--
title: .'EDJX Serverless Rust Samples'
description: 'Different detailed serverless example functions in Rust to get started with Serverless@EDJX'
platform: EDJX
language: Rust
-->

# edjsamples-rust
This repository contains EDJX's sample serverless functions written in Rust. Each sample also contains a README, which has a detailed explanation of the specifics of the Serverless Function. These sample functions can be deployed to the EDJX network using EDJX CLI. More details on how to deploy can be found [in the EDJX Documentation](https://docs.edjx.io/docs/latest/how_tos/cli_build_wasm_file.html).

It is also possible to build Rust applications into WASM executables manually using the steps below.

## Directory Structure

Each directory in this repository corresponds to a different application example written in Rust.

## Build an Example Application

Applications can be built using Cargo:

    cd <application>
    cargo build --release --target wasm32-unknown-unknown

The resulting WASM file will be created in `<application>/target/wasm32-unknown-unknown/release/<app>.wasm`.
