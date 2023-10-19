#!/bin/bash

set -xeu

brew list --versions
brew -v update

# fixes the install on 2023-10-20
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

brew uninstall --force node@18

brew upgrade --force --display-times
brew list --versions
brew -v install --force --display-times opencv"$BREW_OPENCV_VERSION"
