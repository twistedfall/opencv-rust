#!/bin/bash

set -vex

choco install -y llvm --version 13.0.0
choco install -y opencv --version "$CHOCO_OPENCV_VERSION"
