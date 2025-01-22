#!/bin/bash

set -xeu

# install llvm from choco in place of vcpkg to speed things up
choco install -y llvm --version "$CHOCO_LLVM_VERSION"

export VCPKG_DEFAULT_TRIPLET=x64-windows

script_dir="$(dirname "$(readlink -f "$BASH_SOURCE")")"

. "$script_dir/install-vcpkg.sh"
