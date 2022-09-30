#!/bin/bash

set -vex

choco install -y llvm --version 15.0.1
choco install -y opencv --version "$OPENCV_VERSION"
