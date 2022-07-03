#!/bin/bash

bash build.sh
export PARENT=fakenear.testnet
export CONTRACT=testoregon.$PARENT

near delete $CONTRACT $PARENT
near create-account $CONTRACT --masterAccount $PARENT --initialBalance 5
near deploy --accountId $CONTRACT --wasmFile ../res/contract2.wasm
near call $CONTRACT new '{}' --accountId $CONTRACT