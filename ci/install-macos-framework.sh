#!/bin/bash

set -vex

base_dir="$HOME/build/opencv/"
mkdir -p "$base_dir"

curl -L "https://github.com/opencv/opencv/archive/$OPENCV_VERSION.tar.gz" | tar -xzC "$base_dir"
curl -L "https://github.com/opencv/opencv_contrib/archive/$OPENCV_VERSION.tar.gz" | tar -xzC "$base_dir"

build_dir="$base_dir/opencv-$OPENCV_VERSION-build/"
mkdir -p "$build_dir"

python "$base_dir/opencv-$OPENCV_VERSION/platforms/osx/build_framework.py" \
	--contrib "$base_dir/opencv_contrib-$OPENCV_VERSION" \
	--enable_nonfree \
	"$build_dir"
