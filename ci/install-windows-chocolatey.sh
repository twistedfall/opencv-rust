#!/bin/bash

set -vex

choco install -y llvm --version 15.0.3
choco install -y opencv --version "$OPENCV_VERSION"
