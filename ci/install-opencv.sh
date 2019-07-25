#!/bin/bash

set -e

mkdir -p ~/build/opencv

wget -q -O- "https://github.com/opencv/opencv/archive/$OPENCV_VERSION.tar.gz" | tar -xzC ~/build/opencv
if [[ "$OPENCV_VERSION" == "3.2.0" ]]; then
    wget -O- https://patch-diff.githubusercontent.com/raw/opencv/opencv/pull/9228.patch | patch -p1 -d ~"/build/opencv/opencv-$OPENCV_VERSION"
fi
wget -q -O- "https://github.com/opencv/opencv_contrib/archive/$OPENCV_VERSION.tar.gz" | tar -xzC ~/build/opencv

mkdir -p ~"/build/opencv/opencv-$OPENCV_VERSION-build"
pushd ~"/build/opencv/opencv-$OPENCV_VERSION-build" > /dev/null
cmake -DCMAKE_BUILD_TYPE=Release -DBUILD_SHARED_LIBS=ON -DOPENCV_GENERATE_PKGCONFIG=ON -DOPENCV_ENABLE_NONFREE=ON -DBUILD_PERF_TESTS=OFF -DBUILD_TESTS=OFF -DINSTALL_TESTS=OFF -DBUILD_DOCS=OFF -DBUILD_EXAMPLES=OFF -DPYTHON_EXECUTABLE=OFF -DINSTALL_PYTHON_EXAMPLES=OFF -DBUILD_opencv_apps=ALL -DWITH_LAPACK=ON -DWITH_EIGEN=ON -DWITH_TBB=ON -DWITH_ADE=OFF -DCMAKE_INSTALL_PREFIX=/usr -DOPENCV_EXTRA_MODULES_PATH=~"/build/opencv/opencv_contrib-$OPENCV_VERSION/modules" ~"/build/opencv/opencv-$OPENCV_VERSION"
sudo make -j"$(nproc)" install
tree .
popd > /dev/null
