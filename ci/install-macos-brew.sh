#!/bin/bash

set -vex

brew -v update
brew install opencv"$BREW_OPENCV_VERSION"
