#!/bin/bash

OPENCV_VERSION=4.5.4
toolchain_path="$(xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/"
echo "export DYLD_FALLBACK_LIBRARY_PATH=\"$toolchain_path/usr/lib/\""
clang_dir="$(clang --print-search-dirs | awk -F= '/^libraries: =/ { print $2 }')"
echo "export OPENCV_LINK_PATHS=\"$HOME/build/opencv/opencv-$OPENCV_VERSION-build,$clang_dir/lib/darwin\""
echo "export OPENCV_LINK_LIBS=\"opencv2.framework,OpenCL.framework,Cocoa.framework,Accelerate.framework,AVFoundation.framework,CoreGraphics.framework,CoreMedia.framework,CoreVideo.framework,QuartzCore.framework,clang_rt.osx\""
echo "export OPENCV_INCLUDE_PATHS=\"$HOME/build/opencv/opencv-$OPENCV_VERSION-build\""
