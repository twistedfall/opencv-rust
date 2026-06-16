#!/bin/bash

set -eu

script_dir="$(dirname "$(readlink -f "$0")")"

. "$script_dir/config.sh"

SRC_CPP_DIR="$script_dir/../src_cpp/"

cd "$script_dir/.."

export RUST_BACKTRACE=full
if ! cargo build --release -p opencv-binding-generator --bin binding-generator; then
	exit
fi

all_modules="alphamat
	aruco
	bgsegm
	bioinspired
	calib
	calib3d
	ccalib
	cnn_3dobj
	core
	cudaarithm
	cudabgsegm
	cudacodec
	cudafeatures2d
	cudafilters
	cudaimgproc
	cudalegacy
	cudaobjdetect
	cudaoptflow
	cudastereo
	cudawarping
	cvv
	dnn
	dnn_objdetect
	dnn_superres
	dpm
	face
	features
	features2d
	flann
	freetype
	fuzzy
	gapi
	geometry
	hdf
	hfs
	highgui
	img_hash
	imgcodecs
	imgproc
	intensity_transform
	line_descriptor
	mcc
	ml
	objdetect
	optflow
	ovis
	phase_unwrapping
	photo
	plot
	ptcloud
	quality
	rapid
	reg
	rgbd
	saliency
	sfm
	shape
	signal
	stereo
	stitching
	structured_light
	superres
	surface_matching
	text
	tracking
	video
	videoio
	videostab
	viz
	wechat_qrcode
	xfeatures2d
	ximgproc
	xobjdetect
	xphoto
	xstereo
"

export OPENCV_BINDING_GENERATOR_EMIT_DEBUG=1

# Generate bindings for a specific OpenCV version
# Arguments: $1 = version, remaining args = optional module override
generate_version() {
	local version="$1"
	shift

	local header_dir_var="OPENCV_${version}_HEADER_DIR"
	local additional_include_dirs_var="OPENCV_${version}_ADDITIONAL_INCLUDE_DIRS"
	local header_dir="${!header_dir_var}"
	local additional_include_dirs="${!additional_include_dirs_var}"
	local out_dir="$script_dir/../out/${version}/"
	mkdir -p "$out_dir"

	echo "=== OpenCV ${version}..."
	local modules="${*:-$all_modules}"

	local filtered_modules=""
	local skipped_modules=""
	for module in $modules; do
		if [[ ! -f "$header_dir/opencv2/$module.hpp" ]]; then
			skipped_modules="$skipped_modules $module"
		else
			filtered_modules="$filtered_modules $module"
		fi
	done
	if [[ -n "$skipped_modules" ]]; then
		echo "Modules not found:$skipped_modules"
	fi
	modules="$filtered_modules"

	for module in $modules; do
		rm -f "$out_dir/$module.rs" "$out_dir/$module.externs.rs" "$out_dir/$module.cpp"
		rm -f "$out_dir"/???-"$module"-*.type.cpp "$out_dir"/???-"$module"-*.type.rs "$out_dir"/???-"$module"-*.type.externs.rs
	done

	parallel --eta "$script_dir/../target/release/binding-generator" --debug "$header_dir" "$SRC_CPP_DIR" "$out_dir" "{}" "$additional_include_dirs" ::: $modules
}

generate_version "4" $*
generate_version "5" $*
