#!/bin/bash

set -xeu

ci_dir="$(dirname "$0")"

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
	os_family="Linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
	os_family="macOS"
elif [[ "$OSTYPE" == "cygwin" || "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
	os_family="Windows"
elif [[ "$OSTYPE" == "freebsd"* ]]; then
	exit "FreeBSD is not supported"
else
	exit "Unknown OS: $OSTYPE"
fi

if [[ "$os_family" == "Linux" ]]; then
	if [[ "${VCPKG_VERSION:-}" != "" ]]; then # vcpkg build
		"$ci_dir/install-ubuntu-vcpkg.sh"
	else
		"$ci_dir/install-ubuntu.sh"
	fi
elif [[ "$os_family" == "macOS" ]]; then
	if [[ "${BREW_OPENCV_VERSION:-}" != "" ]]; then # brew build
		"$ci_dir/install-macos-brew.sh"
	elif [[ "${VCPKG_VERSION:-}" != "" ]]; then # vcpkg build
		"$ci_dir/install-macos-vcpkg.sh"
	else
		"$ci_dir/install-macos-framework.sh"
	fi
elif [[ "$os_family" == "Windows" ]]; then
	export CHOCO_LLVM_VERSION=20.1.0 # https://community.chocolatey.org/packages/llvm#versionhistory
	if [[ "${VCPKG_VERSION:-}" != "" ]]; then # vcpkg build
		"$ci_dir/install-windows-vcpkg.sh"
	else # chocolatey build
		"$ci_dir/install-windows-chocolatey.sh"
	fi
fi
