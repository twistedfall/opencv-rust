#!/bin/bash

set -xeu

echo "=== Current directory: $(pwd)"
echo "=== Environment variable dump:"
export
echo "=== Target settings:"
rustc --version
rustc --print=cfg

cargo update
rm -vf examples/cuda.rs # no CUDA support in CI
cargo check -vv --all-targets --all-features --workspace --tests
