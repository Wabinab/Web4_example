#!/bin/bash

bash build.sh
export CONTRACT=fakenear.testnet

near deploy --accountId $CONTRACT --wasmFile ../res/web4_example.wasm