#!/bin/bash

set -eu

script_dir="$(dirname "$(readlink -f "$BASH_SOURCE")")"

. "$script_dir/config.sh"

SRC_CPP_DIR="$script_dir/../src_cpp/"
OUT_DIR_34="$script_dir/../out/3.4/"
OUT_DIR_4="$script_dir/../out/4/"
OUT_DIR_5="$script_dir/../out/5/"

mkdir -p "$OUT_DIR_34" "$OUT_DIR_4" "$OUT_DIR_5"

cd "$script_dir/.."

export RUST_BACKTRACE=full
if ! cargo build --release -p opencv-binding-generator --bin binding-generator; then
	exit
fi

all_modules_34="aruco
	bgsegm
	bioinspired
	calib3d
	ccalib
	core
	dnn
	dpm
	face
	features2d
	flann
	freetype
	fuzzy
	hdf
	hfs
	highgui
	img_hash
	imgcodecs
	imgproc
	line_descriptor
	ml
	objdetect
	optflow
	phase_unwrapping
	photo
	plot
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
	xfeatures2d
	ximgproc
	xobjdetect
	xphoto
"

all_modules_4="alphamat
	aruco
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
	cudalegacy
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
"

all_modules_5="3d
	alphamat
	bgsegm
	bioinspired
	calib
	ccalib
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
	dnn
	dnn_superres
	dpm
	face
	features
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

modules_34="${*:-$all_modules_34}"
for module in $modules_34; do
	rm -f "$OUT_DIR_34/$module.rs" "$OUT_DIR_34/$module.externs.rs" "$OUT_DIR_34/$module.cpp"
	rm -f "$OUT_DIR_34"/???-"$module"-*.type.cpp "$OUT_DIR_34"/???-"$module"-*.type.rs "$OUT_DIR_34"/???-"$module"-*.type.externs.rs
done

modules_4="${*:-$all_modules_4}"
for module in $modules_4; do
	rm -f "$OUT_DIR_4/$module.rs" "$OUT_DIR_4/$module.externs.rs" "$OUT_DIR_4/$module.cpp"
	rm -f "$OUT_DIR_4"/???-"$module"-*.type.cpp "$OUT_DIR_4"/???-"$module"-*.type.rs "$OUT_DIR_4"/???-"$module"-*.type.externs.rs
done

modules_5="${*:-$all_modules_5}"
for module in $modules_5; do
	rm -f "$OUT_DIR_5/$module.rs" "$OUT_DIR_5/$module.externs.rs" "$OUT_DIR_5/$module.cpp"
	rm -f "$OUT_DIR_5"/???-"$module"-*.type.cpp "$OUT_DIR_5"/???-"$module"-*.type.rs "$OUT_DIR_5"/???-"$module"-*.type.externs.rs
done

export OPENCV_BINDING_GENERATOR_EMIT_DEBUG=1
#for module in $modules_4; do
#	echo "$module"
#	"$script_dir/../target/release/binding-generator" --debug "$OPENCV_4_HEADER_DIR" "$SRC_CPP_DIR" "$OUT_DIR_4" "$module"
#done
parallel --eta "$script_dir/../target/release/binding-generator" --debug "$OPENCV_34_HEADER_DIR" "$SRC_CPP_DIR" "$OUT_DIR_34" "{}" "$OPENCV_34_ADDITIONAL_INCLUDE_DIRS" ::: $modules_34
parallel --eta "$script_dir/../target/release/binding-generator" --debug "$OPENCV_4_HEADER_DIR" "$SRC_CPP_DIR" "$OUT_DIR_4" "{}" "$OPENCV_4_ADDITIONAL_INCLUDE_DIRS" ::: $modules_4
parallel --eta "$script_dir/../target/release/binding-generator" --debug "$OPENCV_5_HEADER_DIR" "$SRC_CPP_DIR" "$OUT_DIR_5" "{}" "$OPENCV_5_ADDITIONAL_INCLUDE_DIRS" ::: $modules_5

#parallel --eta "$script_dir/../target/release/binding-generator" --debug "{1}" "$SRC_CPP_DIR" "{2}" "{3}" \
#	::: "$OPENCV_4_HEADER_DIR" "$OPENCV_34_HEADER_DIR" \
#	:::+ "$OUT_DIR_4" "$OUT_DIR_34" \
#	::: $modules_4
