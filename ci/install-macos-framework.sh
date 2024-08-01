#!/bin/bash

set -xeu

macos_version="$(sw_vers -productVersion)"

dist_dir="$HOME/dist/"
build_dir="$HOME/build/opencv/opencv-$OPENCV_VERSION-build/"

# if the build dir already present, assume it's the current version and skip download & build to save time
# the build_framework.py doesn't seem to handle the rebuild with unchanged sources too well and does a full rebuild
if [[ -d "$build_dir" ]]; then
	exit 0
fi

mkdir -p "$dist_dir" "$build_dir"

opencv_src="$dist_dir/opencv-$OPENCV_VERSION"
if [ ! -d "$opencv_src" ]; then
	curl -L "https://github.com/opencv/opencv/archive/$OPENCV_VERSION.tar.gz" | tar -xz -C "$dist_dir"
fi

opencv_contrib_src="$dist_dir/opencv_contrib-$OPENCV_VERSION"
if [ ! -d "$opencv_contrib_src" ]; then
	curl -L "https://github.com/opencv/opencv_contrib/archive/$OPENCV_VERSION.tar.gz" | tar -xz -C "$dist_dir"
fi

if [[ "$OPENCV_VERSION" == "3.4.20" ]]; then # old OpenCV doesn't support choosing archs
	arch_arg=
else
	arch_arg="--macos_archs $(uname -m)"
fi
python "$opencv_src/platforms/osx/build_framework.py" \
	--contrib "$opencv_contrib_src" \
	--enable_nonfree \
	$arch_arg \
	"$build_dir"
