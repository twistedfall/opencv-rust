#!/bin/bash

# WIP, doesn't work yet

set -vex

choco install -y python3 --version 3.7.5

git clone https://github.com/Microsoft/vcpkg.git --depth=1
pushd vcpkg
cmd "/C bootstrap-vcpkg.bat -disableMetrics"
#./vcpkg integrate install
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/x64-windows.cmake
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/x86-windows.cmake
export VCPKG_DEFAULT_TRIPLET=x64-windows
# this is going to output something to console while opencv builds
# otherwise travis terminates job because of terminal inactivity
( while true; do sleep 5m; done ) &
./vcpkg install "opencv${VCPKG_OPENCV_VERSION}[contrib,nonfree]"
kill $(jobs -p)
popd
