#!/bin/bash

set -xeu

ci_dir="$(dirname "$0")"

if [[ "$OS_FAMILY" == "Linux" ]]; then
	# free up disk space in Github Actions image: https://github.com/actions/runner-images/issues/2840
	sudo rm -rf /usr/share/dotnet
	sudo rm -rf /opt/ghc
	sudo rm -rf "/usr/local/share/boost"
	sudo rm -rf "$AGENT_TOOLSDIRECTORY"
	if [[ "${VCPKG_VERSION:-}" != "" ]]; then # vcpkg build
		"$ci_dir/install-ubuntu-vcpkg.sh"
	else
		"$ci_dir/install-ubuntu.sh"
	fi
elif [[ "$OS_FAMILY" == "macOS" ]]; then
	if [[ "${BREW_OPENCV_VERSION:-}" != "" ]]; then # brew build
		"$ci_dir/install-macos-brew.sh"
	else
		"$ci_dir/install-macos-framework.sh"
	fi
elif [[ "$OS_FAMILY" == "Windows" ]]; then
	export CHOCO_LLVM_VERSION=16.0.6
	if [[ "${VCPKG_VERSION:-}" != "" ]]; then # vcpkg build
		"$ci_dir/install-windows-vcpkg.sh"
	else # chocolatey build
		"$ci_dir/install-windows-chocolatey.sh"
	fi
fi
