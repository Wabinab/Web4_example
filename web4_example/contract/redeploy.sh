#!/bin/bash

export CONTRACT=fakenear.testnet

near deploy --accountId $CONTRACT --wasmFile contract/res/web4_example.wasm