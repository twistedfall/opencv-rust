#!/bin/bash

set -vex

brew unlink python@2
brew install llvm pkg-config
brew install opencv"$MACOS_OPENCV_VERSION"
# https://stackoverflow.com/a/53331571/1433768
#brew cleanup
