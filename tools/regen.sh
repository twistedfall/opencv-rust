#!/bin/bash

set -e

branch="$1"

script_dir="$(dirname "$(readlink -f "$BASH_SOURCE")")"

cd "$script_dir/.."

. "$script_dir/env-$branch.sh"

export RUSTFLAGS="-Clink-arg=-fuse-ld=lld"

touch build.rs
cargo -vv test --release
