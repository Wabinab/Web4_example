#!/bin/bash

bash build.sh
export PARENT=fakenear.testnet
export CONTRACT=oregon.$PARENT

near deploy --accountId $CONTRACT --wasmFile ../res/contract2.wasm