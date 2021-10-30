#!/bin/bash

script_dir="$(dirname "$(readlink -f "$BASH_SOURCE")")"

. "$script_dir/config.sh"

SRC_CPP_DIR="$script_dir/../src_cpp/"
OUT_DIR="$script_dir/../out/generator/"

cd "$script_dir/.."

export RUSTFLAGS=-Clink-arg=-fuse-ld=lld
export RUST_BACKTRACE=full
if ! cargo build --release -p opencv-binding-generator --bin binding-generator; then
	exit
fi

all_modules="alphamat
	aruco
	barcode
	bgsegm
	bioinspired
	calib3d
	ccalib
	core
	cudaarithm
	cudabgsegm
	cudacodec
	cudafeatures2d
	cudafilters
	cudaimgproc
	cudaobjdetect
	cudaoptflow
	cudastereo
	cudawarping
	cvv
	dnn
	dnn_superres
	dpm
	face
	features2d
	flann
	freetype
	fuzzy
	gapi
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
	quality
	rapid
	rgbd
	saliency
	sfm
	shape
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
	xfeatures2d
	ximgproc
	xobjdetect
	xphoto
	wechat_qrcode
"
modules="${*:-$all_modules}"

for module in $modules; do
	rm -f "$OUT_DIR/$module.rs" "$OUT_DIR/$module.externs.rs" "$OUT_DIR/$module.cpp" "$OUT_DIR/ocvrs_ephemeral_$module.hpp"
	rm -f "$OUT_DIR"/???-"$module"-*.type.cpp "$OUT_DIR"/???-"$module"-*.type.rs
done

export OPENCV_BINDING_GENERATOR_EMIT_DEBUG=1
parallel --eta "$script_dir/../target/release/binding-generator" --debug "$OPENCV_4_HEADER_DIR" "$SRC_CPP_DIR" "$OUT_DIR" "{}" ::: $modules
