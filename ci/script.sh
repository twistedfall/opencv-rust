#!/bin/bash

set -xeu

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

if [[ "$os_family" == "Windows" ]]; then
	export PATH="/C/Program Files/LLVM/bin:$PATH"
	export LIBCLANG_PATH="C:/Program Files/LLVM/bin"
	if [[ "${VCPKG_VERSION:-}" != "" ]]; then # vcpkg build
		export VCPKGRS_DYNAMIC=1
		export VCPKG_ROOT="$HOME/build/vcpkg"
		echo "=== Installed vcpkg packages:"
		"$VCPKG_ROOT/vcpkg" list
	else # chocolatey build
		export PATH="/C/tools/opencv/build/x64/vc16/bin:/C/tools/opencv/build/x64/vc15/bin:$PATH"
		export OPENCV_LINK_PATHS="C:/tools/opencv/build/x64/vc16/lib,C:/tools/opencv/build/x64/vc15/lib"
		export OPENCV_LINK_LIBS="opencv_world${OPENCV_VERSION//./}"
		export OPENCV_INCLUDE_PATHS="C:/tools/opencv/build/include"
	fi
	echo "=== Installed chocolatey packages:"
	choco list
elif [[ "$os_family" == "macOS" ]]; then
	xcode_select="xcode-select" # IDEA code highlighting workaround
	toolchain_path="$($xcode_select --print-path)/Toolchains/XcodeDefault.xctoolchain/"
	export DYLD_FALLBACK_LIBRARY_PATH="$toolchain_path/usr/lib/"
	if [[ "${VCPKG_VERSION:-}" != "" ]]; then # vcpkg build
		export VCPKG_ROOT="$HOME/build/vcpkg"
		export VCPKG_DISABLE_METRICS=1
		echo "=== Installed vcpkg packages:"
		"$VCPKG_ROOT/vcpkg" list
	elif [[ "${BREW_OPENCV_VERSION:-}" != "" ]]; then # brew build
		true
	else # framework build
		clang_dir="$(clang --print-search-dirs | awk -F= '/^libraries: =/ { print $2 }')"
		export OPENCV_LINK_PATHS="$HOME/build/opencv/opencv-$OPENCV_VERSION-build,$clang_dir/lib/darwin"
		export OPENCV_LINK_LIBS="opencv2.framework,OpenCL.framework,Cocoa.framework,Accelerate.framework,AVFoundation.framework,CoreGraphics.framework,CoreMedia.framework,CoreVideo.framework,QuartzCore.framework,clang_rt.osx"
		export OPENCV_INCLUDE_PATHS="$HOME/build/opencv/opencv-$OPENCV_VERSION-build"
	fi
	echo "=== Installed brew packages:"
	brew list --versions
elif [[ "$os_family" == "Linux" ]]; then
	if [[ "${VCPKG_VERSION:-}" != "" ]]; then # vcpkg build
		export VCPKG_ROOT="$HOME/build/vcpkg"
		echo "=== Installed vcpkg packages:"
		"$VCPKG_ROOT/vcpkg" list
		# fixes linking issue that started happening since 2024-07-04
		export OPENCV_LINK_LIBS="+freetype,bz2,brotlidec,brotlicommon"
	else
		if [[ "${OPENCV_LINKAGE:-dynamic}" == "static" ]]; then # static build
			export OPENCV_LINK_LIBS=opencv_gapi,opencv_highgui,opencv_objdetect,opencv_dnn,opencv_videostab,opencv_calib3d,opencv_features2d,opencv_stitching,opencv_flann,opencv_videoio,opencv_rgbd,opencv_aruco,opencv_video,opencv_ml,opencv_imgcodecs,opencv_imgproc,opencv_core,ade,ittnotify,liblibwebp,liblibtiff,liblibjpeg-turbo,liblibpng,liblibopenjp2,ippiw,ippicv,liblibprotobuf,quirc,zlib
		fi
	fi
fi

# remove tests and examples that require the latest OpenCV version so that they don't fail due to missing modules
if [[ "${OPENCV_VERSION:-}" != "4.10.0" ]]; then
	rm -vf tests/*_only_latest_opencv.rs
	rm -vf examples/dnn_face_detect.rs examples/text_detection.rs
fi

echo "=== Current directory: $(pwd)"
echo "=== Environment variable dump:"
export
echo "=== Target settings:"
rustc --version
rustc --print=cfg

export RUST_BACKTRACE=full

cargo test -vv -p opencv-binding-generator

FEATURES=rgb

cargo test -vv --features "$FEATURES"
cargo test --release -vv --features "$FEATURES"
cargo test --release -vv --features "$FEATURES,clang-runtime"

pushd ci/test-proj
cargo run -vv
popd

export CXX=clang++
touch build.rs
cargo test -vv
cargo test --release -vv
