#!/bin/bash

bash build.sh
# export CONTRACT=fakenear.testnet
export CONTRACT=wlog.near

near deploy --accountId $CONTRACT --wasmFile ../res/web4_example.wasm