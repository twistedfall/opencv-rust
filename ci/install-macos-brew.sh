#!/bin/bash

set -xeu

brew list --versions
brew -v update
brew upgrade --force --display-times
brew list --versions
brew -v install --force --display-times opencv"$BREW_OPENCV_VERSION"
brew -v link opencv"$BREW_OPENCV_VERSION"
brew -v link --force rustup
