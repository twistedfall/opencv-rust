#!/bin/bash

# WIP, doesn't work yet

set -vex

choco install -y ninja
choco install -y mingw --version "7.3.0"
choco install -y python3 --version "3.7.5"

export PATH="$PATH:C:\\ProgramData\\chocolatey\\lib\\ninja\\tools"
git clone https://github.com/Microsoft/vcpkg.git --depth=1
pushd vcpkg
./scripts/bootstrap.sh --disableMetrics --useSystemBinaries
./vcpkg integrate install
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/x64-windows.cmake
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/x86-windows.cmake
export VCPKG_DEFAULT_TRIPLET=x64-windows
./vcpkg install "opencv${VCPKG_OPENCV_VERSION}[contrib,nonfree]"
popd
