#!/bin/bash

bash build.sh
# export PARENT=fakenear.testnet
# export CONTRACT=oregon.$PARENT
export PARENT=wlog.near
export CONTRACT=contract.$PARENT

near deploy --accountId $CONTRACT --wasmFile ../res/contract2.wasm
# near deploy --accountId $CONTRACT --wasmFile ../res/contract2.wasm --initFunction "migrate" --initArgs '{}'