#!/bin/bash

bash build.sh
# export PARENT=fakenear.testnet
# export CONTRACT=testoregon.$PARENT
export PARENT=wlog.near
export CONTRACT=contract.$PARENT

near delete $CONTRACT $PARENT
near create-account $CONTRACT --masterAccount $PARENT --initialBalance 3
near deploy --accountId $CONTRACT --wasmFile ../res/contract2.wasm
near call $CONTRACT new '{}' --accountId $CONTRACT