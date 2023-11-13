#!/bin/bash

set -xeu

sudo apt-get update
sudo apt-get install -y clang libclang-dev

# workaround to make clang_sys crate detect installed libclang
sudo ln -fs libclang.so.1 /usr/lib/llvm-14/lib/libclang.so

export RUST_BACKTRACE=full DOCS_RS=1
cargo doc -vv
