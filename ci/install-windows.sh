#!/bin/bash

set -vex

choco install -y python3 --version "3.7.4"
choco install -y opencv --version "$OPENCV_VERSION"
