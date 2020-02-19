#!/bin/bash

set -vex

ci_dir="$(dirname "$0")"

if [[ "$OS_FAMILY" == "linux" ]]; then
	"$ci_dir/install-bionic.sh"
elif [[ "$OS_FAMILY" == "osx" ]]; then
	"$ci_dir/install-macos.sh"
elif [[ "$OS_FAMILY" == "windows" ]]; then
	if [[ "$CHOCO_OPENCV_VERSION" != "" ]]; then # chocolatey build
		"$ci_dir/install-windows-chocolatey.sh"
	else # vcpkg build
		"$ci_dir/install-windows-vcpkg.sh"
	fi
fi
