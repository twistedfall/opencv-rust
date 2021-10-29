#!/bin/bash

set -e

script_dir="$(dirname "$(readlink -f "$BASH_SOURCE")")"

. "$script_dir/config.sh"

cd "$script_dir/.."

export RUSTFLAGS=-Clink-arg=-fuse-ld=lld
export RUST_BACKTRACE=full
cargo run --release -p opencv-binding-generator --bin settings-cleanup -- src_cpp "$OPENCV_32_HEADER_DIR" "$OPENCV_34_HEADER_DIR" "$OPENCV_4_HEADER_DIR"
