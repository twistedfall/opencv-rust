#!/bin/bash

# WIP, doesn't work yet

set -vex

choco install -y ninja
export PATH="$PATH:C:\\ProgramData\\chocolatey\\lib\\ninja\\tools"
git clone https://github.com/Microsoft/vcpkg.git --depth=1
pushd vcpkg
./scripts/bootstrap.sh --disableMetrics --useSystemBinaries
./vcpkg integrate install
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/x64-windows.cmake
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/x86-windows.cmake
./vcpkg install "opencv${VCPKG_OPENCV_VERSION}[contrib,nonfree]"
popd
