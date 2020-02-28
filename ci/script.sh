#!/bin/bash

set -vex

if [[ "$OS_FAMILY" == "windows" ]]; then
	export PATH="/C/Program Files/LLVM/bin:$PATH"
	if [[ "$CHOCO_OPENCV_VERSION" != "" ]]; then # chocolatey build
		# missing aruco module that will not allow us to run tests, that's why contrib feature is not enabled
		export PATH="/C/tools/opencv/build/x64/vc14/bin:$PATH"
		export OPENCV_LINK_PATHS="/C/tools/opencv/build/x64/vc14/lib"
		export OPENCV_LINK_LIBS="opencv_world${CHOCO_OPENCV_VERSION//./}"
		export OPENCV_INCLUDE_PATHS="/C/tools/opencv/build/include"
	else # vcpkg build
		CARGO_FEATURES="$CARGO_FEATURES,contrib"
		export VCPKGRS_DYNAMIC=1
		export VCPKG_ROOT="$HOME/build/vcpkg"
		echo "=== Installed vcpkg packages:"
		"$VCPKG_ROOT/vcpkg" list
	fi
	echo "=== Installed chocolatey packages:"
	choco list --local-only
elif [[ "$OS_FAMILY" == "osx" ]]; then
	if [[ "$MACOS_OPENCV_VERSION" == "@3" ]]; then
		export PKG_CONFIG_PATH="/usr/local/opt/opencv@3/lib/pkgconfig"
	fi
	CARGO_FEATURES="$CARGO_FEATURES,contrib"
elif [[ "$OS_FAMILY" == "linux" ]]; then
	if [[ "$OPENCV_VERSION" != "3.2.0" ]]; then
		# 3.2.0 version from the repository doesn't have dnn module, that's why contrib feature is not enabled
		CARGO_FEATURES="$CARGO_FEATURES,contrib"
	fi
fi

echo "=== Current directory: $(pwd)"
echo "=== Environment variable dump:"
export
echo "=== Target settings:"
rustc --print=cfg

cargo test -vv -p opencv-binding-generator

# only possible to build linux without `buildtime-bindgen`
if [[ "$OS_FAMILY" == "linux" ]]; then
	cargo test -vv --no-default-features --features "$CARGO_FEATURES"
	cargo test --release -vv --no-default-features --features "$CARGO_FEATURES"
fi

CARGO_FEATURES="$CARGO_FEATURES,buildtime-bindgen"
cargo test -vv --no-default-features --features "$CARGO_FEATURES"
cargo test --release -vv --no-default-features --features "$CARGO_FEATURES"

# on windows clang through `cc` crate fails because of -fPIC flag
if [[ "$OS_FAMILY" != "windows" ]]; then
	export CXX=clang++
	touch build.rs
	cargo test -vv --no-default-features --features "$CARGO_FEATURES"
	cargo test --release -vv --no-default-features --features "$CARGO_FEATURES"
fi
