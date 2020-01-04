#!/bin/bash

set -vex

brew unlink python@2
brew install opencv"$OSX_OPENCV_VERSION"
