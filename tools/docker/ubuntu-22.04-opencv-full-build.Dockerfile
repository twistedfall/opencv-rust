# Ubuntu 22.04 container builds OpenCV from source with the maximum possible enabled amount of features

FROM ubuntu:22.04

RUN set -xeu && \
    apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get dist-upgrade -y && \
    apt-get autoremove -y --purge && \
    apt-get -y autoclean

RUN set -xeu && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y curl clang libclang-dev \
    cmake python3-numpy libatlas-base-dev libceres-dev libeigen3-dev liblapacke-dev libprotobuf-dev protobuf-compiler nvidia-cuda-dev libtesseract-dev \
    libwebp-dev libpng-dev libtiff-dev libopenexr-dev libgdal-dev libopenjp2-7-dev libopenjpip-server libopenjpip-dec-server libopenjp2-tools libhdf5-dev \
    libavcodec-dev libavformat-dev libavutil-dev  libgphoto2-dev libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev libva-dev libdc1394-dev \
    libfreetype6-dev libharfbuzz-dev qtbase5-dev libvtk9-dev libogre-1.12-dev

RUN set -xeu && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile=minimal

ENV PATH="${PATH}:/root/.cargo/bin"

ARG OPENCV_VERSION=4.8.0

RUN set -xeu && \
    mkdir -p /root/dist && \
    curl -sSfL "https://github.com/opencv/opencv/archive/refs/tags/${OPENCV_VERSION}.tar.gz" | tar xz -C /root/dist && \
    curl -sSfL "https://github.com/opencv/opencv_contrib/archive/refs/tags/${OPENCV_VERSION}.tar.gz" | tar xz -C /root/dist && \
    sed -ri 's/Ptr<FarnebackOpticalFlow> cv::cuda::FarnebackOpticalFlow::create/Ptr<cv::cuda::FarnebackOpticalFlow> cv::cuda::FarnebackOpticalFlow::create/' "/root/dist/opencv_contrib-${OPENCV_VERSION}/modules/cudaoptflow/src/farneback.cpp" # patch for version 4.8.0

RUN set -xeu && \
    mkdir -p /root/build && \
    cmake -B /root/build -S "/root/dist/opencv-${OPENCV_VERSION}" -D OPENCV_EXTRA_MODULES_PATH="/root/dist/opencv_contrib-${OPENCV_VERSION}/modules" -D CMAKE_INSTALL_PREFIX=/usr \
    	-D BUILD_CUDA_STUBS=ON \
    	-D BUILD_DOCS=OFF \
    	-D BUILD_EXAMPLES=OFF \
    	-D BUILD_IPP_IW=ON \
    	-D BUILD_ITT=ON \
    	-D BUILD_JASPER=OFF \
    	-D BUILD_JAVA=OFF \
    	-D BUILD_JPEG=OFF \
    	-D BUILD_OPENEXR=OFF \
    	-D BUILD_PERF_TESTS=OFF \
    	-D BUILD_PNG=OFF \
    	-D BUILD_PROTOBUF=OFF \
    	-D BUILD_SHARED_LIBS=ON \
    	-D BUILD_TBB=OFF \
    	-D BUILD_TESTS=OFF \
    	-D BUILD_TIFF=OFF \
    	-D BUILD_WEBP=OFF \
    	-D BUILD_WITH_DYNAMIC_IPP=OFF \
    	-D BUILD_ZLIB=OFF \
    	-D BUILD_opencv_apps=OFF \
    	-D BUILD_opencv_python2=OFF \
    	-D BUILD_opencv_python3=OFF \
    	-D CMAKE_BUILD_TYPE=Release \
    	-D ENABLE_CONFIG_VERIFICATION=OFF \
    	-D CV_ENABLE_INTRINSICS=ON \
    	-D ENABLE_PIC=ON \
    	-D INSTALL_CREATE_DISTRIB=OFF \
    	-D INSTALL_PYTHON_EXAMPLES=OFF \
    	-D INSTALL_C_EXAMPLES=OFF \
    	-D INSTALL_TESTS=OFF \
    	-D OPENCV_ENABLE_NONFREE=ON \
    	-D OPENCV_FORCE_3RDPARTY_BUILD=OFF \
    	-D OPENCV_GENERATE_PKGCONFIG=OFF \
    	-D PROTOBUF_UPDATE_FILES=ON \
    	-D WITH_1394=ON \
    	-D WITH_ADE=ON \
    	-D WITH_ARAVIS=OFF \
    	-D WITH_CLP=OFF \
    	-D WITH_CUBLAS=OFF \
    	-D WITH_CUDA=OFF \
    	-D WITH_CUFFT=OFF \
    	-D WITH_EIGEN=ON \
    	-D WITH_FFMPEG=ON \
    	-D WITH_GDAL=ON \
    	-D WITH_GDCM=OFF \
    	-D WITH_GIGEAPI=OFF \
    	-D WITH_GPHOTO2=ON \
    	-D WITH_GSTREAMER=ON \
    	-D WITH_GSTREAMER_0_10=OFF \
    	-D WITH_GTK=OFF \
    	-D WITH_GTK_2_X=OFF \
    	-D WITH_HALIDE=OFF \
    	-D WITH_IMGCODEC_HDcR=ON \
    	-D WITH_IMGCODEC_PXM=ON \
    	-D WITH_IMGCODEC_SUNRASTER=ON \
    	-D WITH_INF_ENGINE=OFF \
    	-D WITH_IPP=ON \
    	-D WITH_ITT=ON \
    	-D WITH_JASPER=OFF \
    	-D WITH_JPEG=ON \
    	-D WITH_LAPACK=ON \
    	-D WITH_LIBV4L=OFF \
    	-D WITH_MATLAB=OFF \
    	-D WITH_MFX=OFF \
    	-D WITH_NVCUVID=OFF \
    	-D WITH_OPENCL=ON \
    	-D WITH_OPENCLAMDBLAS=ON \
    	-D WITH_OPENCLAMDFFT=ON \
    	-D WITH_OPENCL_SVM=ON \
    	-D WITH_OPENEXR=ON \
    	-D WITH_OPENGL=ON \
    	-D WITH_OPENMP=OFF \
    	-D WITH_OPENNI2=OFF \
    	-D WITH_OPENNI=OFF \
    	-D WITH_OPENVX=OFF \
    	-D WITH_PNG=ON \
    	-D WITH_PROTOBUF=ON \
    	-D WITH_PTHREADS_PF=ON \
    	-D WITH_PVAPI=OFF \
    	-D WITH_QT=ON \
    	-D WITH_QUIRC=ON \
    	-D WITH_TBB=ON \
    	-D WITH_TIFF=ON \
    	-D WITH_UNICAP=OFF \
    	-D WITH_V4L=ON \
    	-D WITH_VA=ON \
    	-D WITH_VA_INTEL=ON \
    	-D WITH_VTK=ON \
    	-D WITH_WEBP=ON \
    	-D WITH_XIMEA=OFF \
    	-D WITH_XINE=OFF \
    	-D OpenGL_GL_PREFERENCE=LEGACY && \
    make -C /root/build -j`nproc` install
