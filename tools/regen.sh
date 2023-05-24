#!/bin/bash

set -eu

branch="${1:-4}"

script_dir="$(dirname "$(readlink -f "$BASH_SOURCE")")"

. "$script_dir/env-$branch.sh"

cd "$BINDINGS_OUT_DIR"

bindings_dir="$BINDINGS_OUT_DIR/opencv-$branch"
rm -rf "$bindings_dir"
mkdir -p "$bindings_dir"

temp_proj="opencv-temp-$branch"
rm -rf "$temp_proj"
git clone --depth=1 "file://$script_dir/.." "$temp_proj"
rm -rf "$temp_proj/.git"
pushd "$temp_proj"

export OCVRS_DOCS_GENERATE_DIR=docs
cargo -vv build

bindings_cpp_dir="$bindings_dir/cpp"
mkdir -p "$bindings_cpp_dir"
mv -v target/debug/build/opencv-*/out/*.{cpp,hpp} -t "$bindings_cpp_dir"

bindings_rust_dir="$bindings_dir/rust"
mkdir -p "$bindings_rust_dir"
mv -v docs/*.rs -t "$bindings_rust_dir"

popd

rm -rf "$temp_proj"
