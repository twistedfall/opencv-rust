#!/bin/bash

set -e

branch="${1:-4}"

script_dir="$(dirname "$(readlink -f "$BASH_SOURCE")")"

. "$script_dir/env-$branch.sh"

export RUSTFLAGS="-Clink-arg=-fuse-ld=lld"

cd "$BINDINGS_OUT_DIR"

bindings_dir="$BINDINGS_OUT_DIR/opencv-$branch"
rm -rf "$bindings_dir"
mkdir -p "$bindings_dir"

temp_proj="opencv-temp-$branch"
git clone --depth=1 "file://$script_dir/.." "$temp_proj"
rm -rf "$temp_proj/.git"
pushd "$temp_proj"

cargo -vv build

bindings_cpp_dir="$bindings_dir/cpp"
mkdir -p "$bindings_cpp_dir"
mv -v target/debug/build/opencv-*/out/*.{cpp,hpp} -t "$bindings_cpp_dir"

bindings_rust_dir="$bindings_dir/rust"
mkdir -p "$bindings_rust_dir"
mv -v src/opencv/hub -t "$bindings_rust_dir"
mv -v src/opencv/hub.rs -t "$bindings_rust_dir"

popd

rm -rf "$temp_proj"
