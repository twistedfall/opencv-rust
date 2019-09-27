#!/bin/bash

set -vex

if [[ "$OPENCV_VERSION" == 4* ]]; then
    BREW_OPENCV_VERSION=""
elif [[ "$OPENCV_VERSION" == 3.4* ]]; then
    BREW_OPENCV_VERSION="@3"
else
    BREW_OPENCV_VERSION=""
fi
brew install opencv"$BREW_OPENCV_VERSION"
