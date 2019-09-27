#!/bin/bash

set -vex

ci_dir="$(dirname "$0")"

if [[ "$TRAVIS_OS_NAME" == "linux" ]]; then
    "$ci_dir/install-bionic.sh"
elif [[ "$TRAVIS_OS_NAME" == "osx" ]]; then
    "$ci_dir/install-osx.sh"
elif [[ "$TRAVIS_OS_NAME" == "windows" ]]; then
    "$ci_dir/install-windows.sh"
fi
