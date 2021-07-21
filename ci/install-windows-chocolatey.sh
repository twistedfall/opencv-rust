#!/bin/bash

set -vex

choco install -y llvm --version 12.0.1
choco install -y opencv --version "$CHOCO_OPENCV_VERSION"
