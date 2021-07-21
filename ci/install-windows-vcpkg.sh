#!/bin/bash

set -vex

export VCPKG_ROOT="$HOME/build/vcpkg"
rm -rf "VCPKG_ROOT"
if [[ -e "$VCPKG_ROOT" && ! -e "$VCPKG_ROOT/.git" ]]; then
	rm -rf "$VCPKG_ROOT"
fi
if [ ! -e "$VCPKG_ROOT" ]; then
	git clone --depth=1 https://github.com/Microsoft/vcpkg.git "$VCPKG_ROOT"
fi
pushd "$VCPKG_ROOT"
git fetch --all --prune --tags
git checkout .
git checkout 2021.05.12
cmd "/C bootstrap-vcpkg.bat -disableMetrics"
#./vcpkg integrate install
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/x64-windows.cmake
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/x64-windows-static.cmake
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/x86-windows.cmake
export VCPKG_DEFAULT_TRIPLET=x64-windows
#./vcpkg install llvm  # takes very long time
choco install -y llvm --version 12.0.0
./vcpkg upgrade --no-dry-run
./vcpkg install --recurse "opencv${VCPKG_OPENCV_VERSION}[contrib,nonfree]"
popd
