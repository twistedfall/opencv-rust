#!/bin/bash

set -ex

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
    -D BUILD_opencv_apps=ALL
    -D BUILD_opencv_python2=OFF
    -D BUILD_opencv_python3=OFF
    -D CMAKE_BUILD_TYPE=Release
    -D CMAKE_INSTALL_PREFIX=/usr
    -D CV_DISABLE_OPTIMIZATION=OFF
    -D ENABLE_CONFIG_VERIFICATION=OFF
    -D ENABLE_FAST_MATH=OFF
    -D CV_ENABLE_INTRINSICS=ON
    -D ENABLE_LTO=OFF
    -D ENABLE_PIC=ON
    -D INSTALL_CREATE_DISTRIB=OFF
    -D INSTALL_PYTHON_EXAMPLES=OFF
    -D INSTALL_C_EXAMPLES=OFF
    -D INSTALL_TESTS=OFF
    -D OPENCV_ENABLE_NONFREE=ON
    -D OPENCV_FORCE_3RDPARTY_BUILD=OFF
    -D OPENCV_GENERATE_PKGCONFIG=ON
    -D PROTOBUF_UPDATE_FILES=OFF
    -D WITH_1394=ON
    -D WITH_ADE=ON
    -D WITH_ARAVIS=OFF
    -D WITH_CLP=OFF
    -D WITH_CUDA=OFF
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
    -D WITH_OPENCL=ON
    -D WITH_OPENCLAMDBLAS=ON
    -D WITH_OPENCLAMDFFT=ON
    -D WITH_OPENCL_SVM=ON
    -D WITH_OPENEXR=ON
    -D WITH_OPENGL=ON
    -D WITH_OPENMP=OFF
    -D WITH_OPENNI=OFF
    -D WITH_OPENNI2=OFF
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
    -D WITH_XINE=OFF"

mkdir -p ~/build/opencv

wget -q -O- "https://github.com/opencv/opencv/archive/$OPENCV_VERSION.tar.gz" | tar -xzC ~/build/opencv
if [[ "$OPENCV_VERSION" == "3.2.0" ]]; then
    wget -O- https://patch-diff.githubusercontent.com/raw/opencv/opencv/pull/9228.patch | patch -p1 -d ~/"build/opencv/opencv-$OPENCV_VERSION"
fi
wget -q -O- "https://github.com/opencv/opencv_contrib/archive/$OPENCV_VERSION.tar.gz" | tar -xzC ~/build/opencv

mkdir -p ~/"build/opencv/opencv-$OPENCV_VERSION-build"
pushd ~/"build/opencv/opencv-$OPENCV_VERSION-build" > /dev/null
cmake $BUILD_FLAGS \
    -D CMAKE_INSTALL_PREFIX=/usr \
    -D OPENCV_EXTRA_MODULES_PATH=~/"build/opencv/opencv_contrib-$OPENCV_VERSION/modules" \
    ~/"build/opencv/opencv-$OPENCV_VERSION"
sudo make -j"$(nproc)" install
tree .
popd > /dev/null
