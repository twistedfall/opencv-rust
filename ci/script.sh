#!/bin/bash

set -vex

if [[ "$OS_FAMILY" == "windows" ]]; then
	export PATH="/C/Program Files/LLVM/bin:$PATH"
	export LIBCLANG_PATH="/C/Program Files/LLVM/bin"
	if [[ "$VCPKG_OPENCV_VERSION" != "" ]]; then # vcpkg build
		export VCPKGRS_DYNAMIC=1
		export VCPKG_ROOT="$HOME/build/vcpkg"
		echo "=== Installed vcpkg packages:"
		"$VCPKG_ROOT/vcpkg" list
	else # chocolatey build
		export PATH="/C/tools/opencv/build/x64/vc15/bin:$PATH"
		export OPENCV_LINK_PATHS="/C/tools/opencv/build/x64/vc15/lib"
		export OPENCV_LINK_LIBS="opencv_world${OPENCV_VERSION//./}"
		export OPENCV_INCLUDE_PATHS="/C/tools/opencv/build/include"
	fi
	echo "=== Installed chocolatey packages:"
	choco list --local-only
elif [[ "$OS_FAMILY" == "osx" ]]; then
	toolchain_path="$(xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/"
	export DYLD_FALLBACK_LIBRARY_PATH="$toolchain_path/usr/lib/"
	if [[ "$BREW_OPENCV_VERSION" != "" ]]; then # brew build
		if [[ "$BREW_OPENCV_VERSION" == "@3" ]]; then
			export CMAKE_PREFIX_PATH="$(echo /usr/local/Cellar/opencv@3/3.4.*)"
		fi
	else # framework build
		clang_dir="$(clang --print-search-dirs | awk -F= '/^libraries: =/ { print $2 }')"
		export OPENCV_LINK_PATHS="$HOME/build/opencv/opencv-$OPENCV_VERSION-build,$clang_dir/lib/darwin"
		export OPENCV_LINK_LIBS="opencv2.framework,OpenCL.framework,Cocoa.framework,Accelerate.framework,AVFoundation.framework,CoreGraphics.framework,CoreMedia.framework,CoreVideo.framework,QuartzCore.framework,clang_rt.osx"
		export OPENCV_INCLUDE_PATHS="$HOME/build/opencv/opencv-$OPENCV_VERSION-build"
	fi
	echo "=== Installed brew packages:"
	brew list --versions
elif [[ "$OS_FAMILY" == "linux" ]]; then
	if [[ "$VCPKG_OPENCV_VERSION" != "" ]]; then # vcpkg build
		export VCPKG_ROOT="$HOME/build/vcpkg"
		echo "=== Installed vcpkg packages:"
		"$VCPKG_ROOT/vcpkg" list
		# link order is often broken when vcpkg detection on Linux is used
		export OPENCV_LINK_LIBS="+opencv_imgcodecs,opencv_imgproc,opencv_core,png,tiff,jpeg,webp,lzma,z"
	fi
fi

echo "=== Current directory: $(pwd)"
echo "=== Environment variable dump:"
export
echo "=== Target settings:"
rustc --print=cfg

cargo test -vv -p opencv-binding-generator

FEATURES=rgb

cargo test -vv --features "$FEATURES"
cargo test --release -vv --features "$FEATURES"

pushd ci/test-proj
cargo run -vv
popd

export CXX=clang++
touch build.rs
cargo test -vv
cargo test --release -vv
