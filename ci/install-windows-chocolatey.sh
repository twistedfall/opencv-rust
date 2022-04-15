#!/bin/bash

set -vex

choco install -y llvm --version 14.0.3
choco install -y opencv --version "$OPENCV_VERSION"
