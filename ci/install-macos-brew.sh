#!/bin/bash

set -vex

brew -v update
brew upgrade
brew install opencv"$BREW_OPENCV_VERSION"
