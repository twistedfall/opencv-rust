#!/bin/bash

set -vex

sudo apt-get update

sudo apt-get install -y clang
# workaround to make clang_sys crate detect installed libclang
sudo ln -s libclang.so.1 /usr/lib/llvm-6.0/lib/libclang.so

sudo apt-get -y install "libopencv-dev=${OPENCV_VERSION}*"
