#!/bin/bash

set -xeu

export VCPKG_DEFAULT_TRIPLET=arm64-osx

script_dir="$(dirname "$(readlink -f "$BASH_SOURCE")")"

. "$script_dir/install-vcpkg.sh"
