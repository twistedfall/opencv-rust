#!/bin/bash

set -vex

sudo apt update
sudo apt -y install clang

# workaround to make clang_sys crate detect installed libclang
sudo ln -s libclang.so.1 /usr/lib/llvm-10/lib/libclang.so

cargo run --release -p opencv-binding-generator --bin settings-cleanup -- headers src_cpp
