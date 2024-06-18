#!/bin/bash

set -xeu

ci_dir="$(dirname "$0")"

if [[ "$OS_FAMILY" == "Linux" ]]; then
	# free up disk space in Github Actions image: https://github.com/actions/runner-images/issues/2840
	sudo rm -rf /usr/share/dotnet /opt/ghc /usr/local/share/boost
	if [[ "${VCPKG_VERSION:-}" != "" ]]; then # vcpkg build
		"$ci_dir/install-ubuntu-vcpkg.sh"
	else
		"$ci_dir/install-ubuntu.sh"
	fi
elif [[ "$OS_FAMILY" == "macOS" ]]; then
	if [[ "${BREW_OPENCV_VERSION:-}" != "" ]]; then # brew build
		"$ci_dir/install-macos-brew.sh"
	elif [[ "${VCPKG_VERSION:-}" != "" ]]; then # vcpkg build
		"$ci_dir/install-macos-vcpkg.sh"
	else
		"$ci_dir/install-macos-framework.sh"
	fi
elif [[ "$OS_FAMILY" == "Windows" ]]; then
	export CHOCO_LLVM_VERSION=18.1.6
	if [[ "${VCPKG_VERSION:-}" != "" ]]; then # vcpkg build
		"$ci_dir/install-windows-vcpkg.sh"
	else # chocolatey build
		"$ci_dir/install-windows-chocolatey.sh"
	fi
fi
