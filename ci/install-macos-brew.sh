#!/bin/bash

set -vex

brew -v update

# fixes the install at 2023-01-14
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

brew -v install opencv"$BREW_OPENCV_VERSION"
