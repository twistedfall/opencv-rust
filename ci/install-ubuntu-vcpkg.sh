#!/bin/bash

set -xeu

sudo apt-get update
sudo apt-get install -y clang libharfbuzz0b git curl zip unzip tar bison gperf libx11-dev libxft-dev libxext-dev \
	libgles2-mesa-dev autoconf libtool build-essential libxrandr-dev libxi-dev libxcursor-dev libxdamage-dev libxinerama-dev \
	libdbus-1-dev libxtst-dev

export VCPKG_ROOT="$HOME/build/vcpkg"
if [[ -e "$VCPKG_ROOT" && ! -e "$VCPKG_ROOT/.git" ]]; then
	rm -rf "$VCPKG_ROOT"
fi
if [ ! -e "$VCPKG_ROOT" ]; then
	git clone https://github.com/Microsoft/vcpkg.git "$VCPKG_ROOT"
fi
pushd "$VCPKG_ROOT"
git fetch --all --prune --tags
git status
git checkout .
git checkout "$VCPKG_VERSION"
./bootstrap-vcpkg.sh -disableMetrics
#./vcpkg integrate install
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/x64-linux.cmake
export VCPKG_DEFAULT_TRIPLET=x64-linux
#./vcpkg install llvm  # takes very long time
# workaround to make clang_sys crate detect installed libclang
sudo ln -fs libclang.so.1 /usr/lib/llvm-14/lib/libclang.so
./vcpkg install --recurse "opencv[contrib,nonfree]"
popd
