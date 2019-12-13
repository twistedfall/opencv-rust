#!/bin/bash

set -vex

choco install -y python3 --version "3.7.5"
choco install -y opencv --version "$CHOCO_OPENCV_VERSION"
