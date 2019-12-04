#!/bin/bash

set -vex

ci_dir="$(dirname "$0")"

if [[ "$TRAVIS_OS_NAME" == "linux" ]]; then
    "$ci_dir/install-bionic.sh"
elif [[ "$TRAVIS_OS_NAME" == "osx" ]]; then
    "$ci_dir/install-osx.sh"
elif [[ "$TRAVIS_OS_NAME" == "windows" ]]; then
    if [[ "$CHOCO_OPENCV_VERSION" != "" ]]; then # chocolatey build
        "$ci_dir/install-windows-chocolatey.sh"
    else # vcpkg build
        "$ci_dir/install-windows-vcpkg.sh"
    fi
fi
