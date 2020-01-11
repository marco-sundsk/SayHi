#!/bin/bash
set -e
# to build
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/say_hi.wasm ./res/
#wasm-opt -Oz --output ./res/status_message.wasm ./res/status_message.wasm

# to test
cargo test -- --nocapture

# to clean
rm -rf target
