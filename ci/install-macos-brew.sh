#!/bin/bash

set -xeu

brew list --versions
brew -v update

# fixes the install on 2026-07-23
rm -rf /opt/homebrew/lib/ruby/gems/3.3.0/doc/rubygems-4.0.16
rm -rf /opt/homebrew/lib/ruby/gems/3.3.0/gems/bundler-4.0.16
rm -f /opt/homebrew/lib/ruby/gems/3.3.0/specifications/default/bundler-4.0.16.gemspec

brew upgrade --force --display-times
brew list --versions
brew -v install --force --display-times opencv"$BREW_OPENCV_VERSION"
brew -v link opencv"$BREW_OPENCV_VERSION"
brew -v link --force rustup
