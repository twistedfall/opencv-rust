#!/bin/bash

set -e

script_dir="$(dirname "$(readlink -f "$BASH_SOURCE")")"
cd "$script_dir/.."

export RUSTFLAGS="-Clink-arg=-fuse-ld=lld"
export OS_FAMILY="linux"

(
	. "$script_dir/env-32.sh"
	ci/script.sh
)

(
	. "$script_dir/env-34.sh"
	ci/script.sh
)

(
	. "$script_dir/env-4.sh"
	ci/script.sh
)

cargo clean
