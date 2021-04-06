#!/bin/bash

set -vex

choco install -y llvm --version 11.1.0
choco install -y opencv --version "$CHOCO_OPENCV_VERSION"
