#!/bin/bash

set -xeu

echo "=== Current directory: $(pwd)"
echo "=== Environment variable dump:"
export
echo "=== Target settings:"
rustc --version
rustc --print=cfg

cargo update
cargo check -vv --all-targets --all-features --workspace --tests
