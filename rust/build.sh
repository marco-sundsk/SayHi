#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/say_hi.wasm ./res/
#wasm-opt -Oz --output ./res/status_message.wasm ./res/status_message.wasm
rm -rf target
