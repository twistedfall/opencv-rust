#!/bin/bash

set -xeu

if [[ "$OPENCV_VERSION" == "4.5.4" ]]; then
	rm -vf tests/*4_5_4_norun.rs
fi

echo "=== Current directory: $(pwd)"
echo "=== Environment variable dump:"
export
echo "=== Target settings:"
rustc --version
rustc --print=cfg

cargo update
cargo check -vv --all-targets --all-features --workspace --tests
