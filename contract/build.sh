#!/bin/bash
set -e

export WASM=web4_example.wasm

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/$WASM ../res/web4_example.wasm