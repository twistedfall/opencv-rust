#!/bin/bash

set -vex

brew -v update
brew -v install opencv"$BREW_OPENCV_VERSION"
