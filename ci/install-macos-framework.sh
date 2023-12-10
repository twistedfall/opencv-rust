#!/bin/bash

set -xeu

dist_dir="$HOME/dist/"
base_dir="$HOME/build/opencv/"
build_dir="$base_dir/opencv-$OPENCV_VERSION-build/"
mkdir -p "$dist_dir" "$base_dir" "$build_dir"

# if the build framework already present assume it's the current version and skip build to save time
# the build_framework.py doesn't seem to handle the rebuild with unchanged sources too well and does a full rebuild
if [[ -d "$build_dir/opencv2.framework" ]]; then
	exit 0
fi

opencv_src="$dist_dir/opencv-$OPENCV_VERSION"
if [ ! -d "$opencv_src" ]; then
	curl -L "https://github.com/opencv/opencv/archive/$OPENCV_VERSION.tar.gz" | tar -xz -C "$dist_dir"
fi

opencv_contrib_src="$dist_dir/opencv_contrib-$OPENCV_VERSION"
if [ ! -d "$opencv_contrib_src" ]; then
	curl -L "https://github.com/opencv/opencv_contrib/archive/$OPENCV_VERSION.tar.gz" | tar -xz -C "$dist_dir"
fi

python "$opencv_src/platforms/osx/build_framework.py" \
	--contrib "$opencv_contrib_src" \
	--enable_nonfree \
	"$build_dir"
