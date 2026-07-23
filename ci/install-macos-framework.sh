#!/bin/bash

set -xeu

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

if [[ "$OPENCV_VERSION" == "5.0.0" ]]; then
	# upstream fixes
	patch -p1 -d "$opencv_src" < "$(dirname "$0")/patches/swift_name-attr.patch"
	patch -p1 -d "$opencv_src" < "$(dirname "$0")/patches/enum-redefinition.patch"
fi
if [[ "$OPENCV_VERSION" == "5.0.0" || "$OPENCV_VERSION" == "4.14.0" ]]; then
	# handling of the `doc_hosting_base_path` is properly broken in 5.0.0 and 4.14.0:
	# 1. it's a store_true flag, so it can't be set to a string value
	# 2. it is used in a path.join() so, yeah, that can't work
	# 3. if it's not supplied then the script tries to get the current git branch which we don't have in a release tarball
	# so the patch hackily disabled building docs so that we don't run into 2. and the argument below is used to avoid 3.
	patch -p1 -d "$opencv_src" < "$(dirname "$0")/patches/no-build-docs.patch"
fi

# see the patches section above for why this --doc_hosting_base_path is needed
python "$opencv_src/platforms/osx/build_framework.py" \
	--contrib "$opencv_contrib_src" \
	--enable_nonfree \
	--macos_archs $(uname -m) \
	--macosx_deployment_target 10.13 \
	--doc_hosting_base_path \
	"$build_dir"
