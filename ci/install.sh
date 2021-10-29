#!/bin/bash

set -vex

ci_dir="$(dirname "$0")"

if [[ "$OS_FAMILY" == "linux" ]]; then
	if [[ "$VCPKG_OPENCV_VERSION" != "" ]]; then # vcpkg build
		"$ci_dir/install-focal-vcpkg.sh"
	elif [[ "$OPENCV_VERSION" == "3.2.0" ]]; then
		"$ci_dir/install-bionic.sh"
	else
		"$ci_dir/install-focal.sh"
	fi
elif [[ "$OS_FAMILY" == "osx" ]]; then
	if [[ "$BREW_OPENCV_VERSION" != "" ]]; then # brew build
		"$ci_dir/install-macos-brew.sh"
	else
		"$ci_dir/install-macos-framework.sh"
	fi
elif [[ "$OS_FAMILY" == "windows" ]]; then
	if [[ "$CHOCO_OPENCV_VERSION" != "" ]]; then # chocolatey build
		"$ci_dir/install-windows-chocolatey.sh"
	else # vcpkg build
		"$ci_dir/install-windows-vcpkg.sh"
	fi
fi
