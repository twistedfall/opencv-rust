#!/bin/bash

set -xeu

sudo apt-get update
sudo apt-get -y install clang

# workaround to make clang_sys crate detect installed libclang
sudo ln -fs libclang.so.1 /usr/lib/llvm-14/lib/libclang.so

export RUST_BACKTRACE=full
cargo run --release -p opencv-binding-generator --bin settings-cleanup -- headers src_cpp
