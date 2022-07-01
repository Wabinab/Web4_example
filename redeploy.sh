#!/bin/bash

cd precompiler
cargo run

cd ..
cd contract
bash redeploy.sh