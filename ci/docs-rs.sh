#!/bin/bash

set -xeu

sudo apt-get update
sudo apt-get install -y clang libclang-dev

export RUST_BACKTRACE=full DOCS_RS=1
cargo doc -vv
