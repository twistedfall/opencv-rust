#!/bin/bash

set -vex

if [[ "$TRAVIS_OS_NAME" == "windows" ]]; then
    export PATH="C:\\Python37\\:$PATH"
    if [[ "$CHOCO_OPENCV_VERSION" != "" ]]; then # chocolatey build
        export PATH="$PATH:C:\\tools\\opencv\\build\\x64\\vc14\\bin"
        export OPENCV_LINK_PATHS="C:\\tools\\opencv\\build\\x64\\vc14\\lib"
        export OPENCV_LINK_LIBS="opencv_world${CHOCO_OPENCV_VERSION//./}"
        export OPENCV_INCLUDE_PATHS="C:\\tools\\opencv\\build\\include"
        export OPENCV_HEADER_DIR="C:\\tools\\opencv\\build\\include"
    else # vcpkg build
        export VCPKG_ROOT="$TRAVIS_BUILD_DIR\\vcpkg"
        export PATH="$PATH:$VCPKG_ROOT\\installed\\x64-windows\\bin"
        export VCPKGRS_DYNAMIC=1
    fi
    CARGO_FEATURES="$CARGO_FEATURES,buildtime-bindgen"
elif [[ "$TRAVIS_OS_NAME" == "osx" ]]; then
    if [[ "$OSX_OPENCV_VERSION" == "@3" ]]; then
        export PKG_CONFIG_PATH="/usr/local/opt/opencv@3/lib/pkgconfig"
        export OPENCV_HEADER_DIR="/usr/local/opt/opencv@3/include"
    elif [[ "$OSX_OPENCV_VERSION" == "" ]]; then
        export OPENCV_HEADER_DIR="/usr/local/include/opencv4"
    fi
    CARGO_FEATURES="$CARGO_FEATURES,buildtime-bindgen"
fi

# when building both debug and release we're getting above 50min travis if not using cached build of OpenCV
# so build stable rust only in --release, and beta only in debug
CARGO_EXTRA_FLAGS=""
if [[ "$TRAVIS_RUST_VERSION" == "stable" ]]; then
    CARGO_EXTRA_FLAGS="--release"
fi
cargo build -vv --no-default-features --features "$CARGO_FEATURES" $CARGO_EXTRA_FLAGS

# windows (gui?) binaries can't be run in travis:
# C:/Users/travis/.cargo/bin/cargo.exe: error while loading shared libraries: ?: cannot open shared object file: No such file or directory
# probably because of:
# https://travis-ci.community/t/python-and-opencv-dll-load-fails-every-time/4431
# https://travis-ci.community/t/make-running-gui-apps-available-in-windows/1557
# that's why we don't run test
if [[ "$TRAVIS_OS_NAME" != "windows" ]]; then
    cargo test -vv --no-default-features --features "$CARGO_FEATURES" $CARGO_EXTRA_FLAGS
fi
