#!/bin/bash

set -xeu

sudo apt-get update

sudo apt-get install -y clang
# workaround to make clang_sys crate detect installed libclang
sudo ln -fs libclang.so.1 /usr/lib/llvm-14/lib/libclang.so

if [[ "$OPENCV_VERSION" == "4.5.4" ]]; then
	sudo apt-get -y install "libopencv-dev=${OPENCV_VERSION}*"
	exit 0
fi

BUILD_FLAGS="
	-D BUILD_CUDA_STUBS=OFF
	-D BUILD_DOCS=OFF
	-D BUILD_EXAMPLES=OFF
	-D BUILD_IPP_IW=ON
	-D BUILD_ITT=ON
	-D BUILD_JASPER=OFF
	-D BUILD_JAVA=OFF
	-D BUILD_JPEG=OFF
	-D BUILD_OPENEXR=OFF
	-D BUILD_OPENJPEG=OFF
	-D BUILD_PERF_TESTS=OFF
	-D BUILD_PNG=OFF
	-D BUILD_PROTOBUF=ON
	-D BUILD_SHARED_LIBS=ON
	-D BUILD_TBB=OFF
	-D BUILD_TESTS=OFF
	-D BUILD_TIFF=OFF
	-D BUILD_WEBP=OFF
	-D BUILD_WITH_DEBUG_INFO=OFF
	-D BUILD_WITH_DYNAMIC_IPP=OFF
	-D BUILD_ZLIB=OFF
	-D BUILD_opencv_apps=OFF
	-D BUILD_opencv_python2=OFF
	-D BUILD_opencv_python3=OFF
	-D CMAKE_BUILD_TYPE=Release
	-D CMAKE_INSTALL_PREFIX=/usr
	-D CV_DISABLE_OPTIMIZATION=OFF
	-D CV_ENABLE_INTRINSICS=ON
	-D ENABLE_CONFIG_VERIFICATION=OFF
	-D ENABLE_FAST_MATH=OFF
	-D ENABLE_LTO=OFF
	-D ENABLE_PIC=ON
	-D ENABLE_PRECOMPILED_HEADERS=OFF
	-D INSTALL_CREATE_DISTRIB=OFF
	-D INSTALL_C_EXAMPLES=OFF
	-D INSTALL_PYTHON_EXAMPLES=OFF
	-D INSTALL_TESTS=OFF
	-D OPENCV_ENABLE_MEMALIGN=OFF
	-D OPENCV_ENABLE_NONFREE=ON
	-D OPENCV_FORCE_3RDPARTY_BUILD=OFF
	-D OPENCV_GENERATE_PKGCONFIG=OFF
	-D PROTOBUF_UPDATE_FILES=OFF
	-D WITH_1394=ON
	-D WITH_ADE=ON
	-D WITH_ARAVIS=OFF
	-D WITH_CLP=OFF
	-D WITH_CUBLAS=OFF
	-D WITH_CUDA=OFF
	-D WITH_CUFFT=OFF
	-D WITH_EIGEN=ON
	-D WITH_FFMPEG=ON
	-D WITH_GDAL=ON
	-D WITH_GDCM=OFF
	-D WITH_GIGEAPI=OFF
	-D WITH_GPHOTO2=ON
	-D WITH_GSTREAMER=ON
	-D WITH_GSTREAMER_0_10=OFF
	-D WITH_GTK=OFF
	-D WITH_GTK_2_X=OFF
	-D WITH_HALIDE=OFF
	-D WITH_IMGCODEC_HDcR=ON
	-D WITH_IMGCODEC_PXM=ON
	-D WITH_IMGCODEC_SUNRASTER=ON
	-D WITH_INF_ENGINE=OFF
	-D WITH_IPP=ON
	-D WITH_ITT=ON
	-D WITH_JASPER=OFF
	-D WITH_JPEG=ON
	-D WITH_LAPACK=ON
	-D WITH_LIBV4L=OFF
	-D WITH_MATLAB=OFF
	-D WITH_MFX=OFF
	-D WITH_OPENCL=OFF
	-D WITH_OPENCLAMDBLAS=OFF
	-D WITH_OPENCLAMDFFT=OFF
	-D WITH_OPENCL_SVM=OFF
	-D WITH_OPENEXR=OFF
	-D WITH_OPENGL=ON
	-D WITH_OPENMP=OFF
	-D WITH_OPENNI2=OFF
	-D WITH_OPENNI=OFF
	-D WITH_OPENVX=OFF
	-D WITH_PNG=ON
	-D WITH_PROTOBUF=ON
	-D WITH_PTHREADS_PF=ON
	-D WITH_PVAPI=OFF
	-D WITH_QT=ON
	-D WITH_QUIRC=ON
	-D WITH_TBB=ON
	-D WITH_TIFF=ON
	-D WITH_UNICAP=OFF
	-D WITH_V4L=ON
	-D WITH_VA=ON
	-D WITH_VA_INTEL=ON
	-D WITH_VTK=ON
	-D WITH_WEBP=ON
	-D WITH_XIMEA=OFF
	-D WITH_XINE=OFF
"

sudo apt-get -y install build-essential cmake

non_static_version=${OPENCV_VERSION%-static}
if [[ "$non_static_version" != "$OPENCV_VERSION" ]]; then # static build
	OPENCV_VERSION="$non_static_version"
	BUILD_FLAGS="$BUILD_FLAGS
	-D BUILD_JPEG=ON
	-D BUILD_OPENJPEG=ON
	-D BUILD_PNG=ON
	-D BUILD_SHARED_LIBS=OFF
	-D BUILD_TBB=ON
	-D BUILD_TIFF=ON
	-D BUILD_WEBP=ON
	-D BUILD_ZLIB=ON
	-D BUILD_opencv_freetype=OFF
	-D OPENCV_FORCE_3RDPARTY_BUILD=ON
	-D WITH_1394=OFF
	-D WITH_FFMPEG=OFF
	-D WITH_FREETYPE=OFF
	-D WITH_GDAL=OFF
	-D WITH_GPHOTO2=OFF
	-D WITH_GSTREAMER=OFF
	-D WITH_GTK=OFF
	-D WITH_LAPACK=OFF
	-D WITH_OPENGL=OFF
	-D WITH_QT=OFF
"
else # dynamic build
	# runtime deps
	sudo apt-get -y install \
		libatlas3-base \
		libavcodec58 \
		libavformat58 \
		libceres2 \
		libdc1394-25 \
		libfreetype6 \
		libgdal30 \
		libgflags2.2 \
		libgoogle-glog0v5 \
		libgphoto2-6 \
		libgstreamer-plugins-base1.0-0 \
		libharfbuzz0b \
		libhdf5-103 \
		libjpeg8 \
		liblapacke \
		liblept5 \
		libopenexr25 \
		libpng16-16 \
		libqt5core5a \
		libqt5gui5 \
		libqt5opengl5 \
		libqt5test5 \
		libqt5widgets5 \
		libswscale5 \
		libtbb2 \
		libtesseract4 \
		libvtk7.1p \
		libwebp7

	# need to do that separately to remove conflicting packages
	sudo apt-get install libunwind-dev

	# build deps
	sudo apt-get -y install \
		libatlas-base-dev \
		libavcodec-dev \
		libavformat-dev \
		libceres-dev \
		libdc1394-dev \
		libeigen3-dev \
		libfreetype6-dev \
		libgdal-dev \
		libgflags-dev \
		libgoogle-glog-dev \
		libgphoto2-dev \
		libgstreamer-plugins-base1.0-dev \
		libharfbuzz-dev \
		libhdf5-dev \
		libjpeg-dev \
		liblapacke-dev \
		libleptonica-dev \
		libopenexr-dev \
		libpng-dev \
		libswscale-dev \
		libtbb2-dev \
		libtesseract-dev \
		libtiff-dev \
		libvtk7-dev \
		libwebp-dev \
		qtbase5-dev
fi

base_dir="$HOME/build/opencv/"
mkdir -p "$base_dir"

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

build_dir="$base_dir/opencv-$OPENCV_VERSION-build/"
mkdir -p "$build_dir"

pushd "$build_dir" > /dev/null
cmake $BUILD_FLAGS \
	-D CMAKE_INSTALL_PREFIX=/usr \
	-D OPENCV_EXTRA_MODULES_PATH="$base_dir/opencv_contrib-$OPENCV_VERSION/modules" \
	"$base_dir/opencv-$OPENCV_VERSION"
make -j"$(nproc)"
sudo make -j"$(nproc)" install
popd > /dev/null
