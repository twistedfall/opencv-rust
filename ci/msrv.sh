#!/bin/bash

set -vex

if [[ "$OPENCV_VERSION" == "4.2.0" ]]; then
	rm -vf tests/*4_2_0_norun.rs
fi

echo "=== Current directory: $(pwd)"
echo "=== Environment variable dump:"
export
echo "=== Target settings:"
rustc --print=cfg

cargo clippy -vv --all-targets --all-features --workspace --tests
