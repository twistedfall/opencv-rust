#!/bin/bash

set -vex

ci_dir="$(dirname "$0")"

if [[ "$OS_FAMILY" == "Linux" ]]; then
	if [[ "$VCPKG_VERSION" != "" ]]; then # vcpkg build
		"$ci_dir/install-focal-vcpkg.sh"
	else
		"$ci_dir/install-focal.sh"
	fi
elif [[ "$OS_FAMILY" == "macOS" ]]; then
	if [[ "$BREW_OPENCV_VERSION" != "" ]]; then # brew build
		"$ci_dir/install-macos-brew.sh"
	else
		"$ci_dir/install-macos-framework.sh"
	fi
elif [[ "$OS_FAMILY" == "Windows" ]]; then
	export CHOCO_LLVM_VERSION=15.0.7
	if [[ "$VCPKG_VERSION" != "" ]]; then # vcpkg build
		"$ci_dir/install-windows-vcpkg.sh"
	else # chocolatey build
		"$ci_dir/install-windows-chocolatey.sh"
	fi
fi
