#!/bin/bash

set -xeu

choco install -y llvm --version "$CHOCO_LLVM_VERSION"
choco install -y opencv --version "$OPENCV_VERSION"
