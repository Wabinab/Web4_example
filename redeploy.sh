#!/bin/bash

cd precompiler
cargo run

cd ..
cd contract2
bash redeploy.sh

cd ..
cd contract
bash redeploy.sh