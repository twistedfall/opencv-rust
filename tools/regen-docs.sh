#!/bin/bash

set -eu

branch="${1:-4}"

script_dir="$(dirname "$(readlink -f "$BASH_SOURCE")")"

. "$script_dir/env-$branch.sh"

cd "$script_dir/.."

export OCVRS_DOCS_GENERATE_DIR=docs
cargo -vv build
