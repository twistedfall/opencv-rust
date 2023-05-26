#!/bin/bash

set -xeu

brew -v update
brew -v install opencv"$BREW_OPENCV_VERSION"
