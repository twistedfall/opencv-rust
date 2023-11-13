#!/bin/bash

set -xeu

base_dir="$HOME/build/opencv/"
mkdir -p "$base_dir"

build_dir="$base_dir/opencv-$OPENCV_VERSION-build/"
mkdir -p "$build_dir"

# if the build framework already present assume it's the current version and skip build to save time
# the build_framework.py doesn't seem to handle the rebuild with unchanged sources too well and does a full rebuild
if [[ -d "$build_dir/opencv2.framework" ]]; then
	exit 0
fi

opencv_dist="$base_dir/opencv-$OPENCV_VERSION.tar.gz"
if [ ! -f "$opencv_dist" ]; then
	curl -L "https://github.com/opencv/opencv/archive/$OPENCV_VERSION.tar.gz" -o "$opencv_dist"
fi
tar -xzf "$opencv_dist" -C "$base_dir"

opencv_contrib_dist="$base_dir/opencv_contrib-$OPENCV_VERSION.tar.gz"
if [ ! -f "$opencv_contrib_dist" ]; then
	curl -L "https://github.com/opencv/opencv_contrib/archive/$OPENCV_VERSION.tar.gz" -o "$opencv_contrib_dist"
fi
tar -xzf "$opencv_contrib_dist" -C "$base_dir"

python "$base_dir/opencv-$OPENCV_VERSION/platforms/osx/build_framework.py" \
	--contrib "$base_dir/opencv_contrib-$OPENCV_VERSION" \
	--enable_nonfree \
	"$build_dir"
