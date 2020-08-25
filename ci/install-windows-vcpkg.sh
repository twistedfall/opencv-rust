#!/bin/bash

set -vex

export VCPKG_ROOT="$HOME/build/vcpkg"
if [ ! -e "$VCPKG_ROOT" ]; then
	git clone --depth=1 https://github.com/Microsoft/vcpkg.git "$VCPKG_ROOT"
fi
pushd "$VCPKG_ROOT"
git pull
cmd "/C bootstrap-vcpkg.bat -disableMetrics"
#./vcpkg integrate install
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/x64-windows.cmake
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/x86-windows.cmake
export VCPKG_DEFAULT_TRIPLET=x64-windows
#./vcpkg install llvm  # takes very long time
choco install llvm
./vcpkg install "opencv${VCPKG_OPENCV_VERSION}[contrib,nonfree]"
./vcpkg upgrade --no-dry-run
popd

