#!/bin/bash

set -eu

script_dir="$(dirname "$(readlink -f "$BASH_SOURCE")")"

. "$script_dir/config.sh"

cd "$script_dir/.."

export RUST_BACKTRACE=full
cargo run --release -p opencv-binding-generator --bin settings-cleanup -- src_cpp "$OPENCV_34_HEADER_DIR" "$OPENCV_4_HEADER_DIR"
