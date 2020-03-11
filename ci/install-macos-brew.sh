#!/bin/bash

set -vex

brew unlink python@2
brew install pkg-config
brew install opencv"$BREW_OPENCV_VERSION"
