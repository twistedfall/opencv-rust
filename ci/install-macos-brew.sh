#!/bin/bash

set -xeu

brew list --versions
brew -v update

# fixes the install on 2023-07-12
rm -f /usr/local/bin/go
rm -f /usr/local/bin/gofmt

rm -f /usr/local/bin/2to3-3.11
rm -f /usr/local/bin/idle3.11
rm -f /usr/local/bin/pydoc3.11
rm -f /usr/local/bin/python3.11
rm -f /usr/local/bin/python3.11-config
rm -f /usr/local/bin/2to3
rm -f /usr/local/bin/idle3
rm -f /usr/local/bin/pydoc3
rm -f /usr/local/bin/python3
rm -f /usr/local/bin/python3-config

brew upgrade
brew list --versions
brew -v install opencv"$BREW_OPENCV_VERSION"
