#!/bin/bash

bash build.sh
export PARENT=fakenear.testnet
export CONTRACT=oregon.$PARENT

near delete $CONTRACT $PARENT
near create-account $CONTRACT --masterAccount $PARENT --initialBalance 3
near deploy --accountId $CONTRACT --wasmFile ../res/contract2.wasm
near call $CONTRACT new '{}' --accountId $CONTRACT