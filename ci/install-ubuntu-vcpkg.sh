#!/bin/bash

set -xeu

if [[ "${VCPKG_VERSION:-}" == "" ]]; then
	echo "Must set VCPKG_VERSION environment var"
	exit 1
fi

sudo apt-get update
sudo apt-get install -y \
	autoconf \
	bison \
	build-essential \
	clang \
	cmake \
	curl \
	git \
	gperf \
	libclang-dev \
	libdbus-1-dev \
	libgles2-mesa-dev \
	libharfbuzz0b \
	libltdl-dev \
	libtool \
	libx11-dev \
	libxcursor-dev \
	libxdamage-dev \
	libxext-dev \
	libxft-dev \
	libxi-dev \
	libxinerama-dev \
	libxrandr-dev \
	libxtst-dev \
	python3-jinja2 \
	sudo \
	tar \
	unzip \
	zip

export VCPKG_ROOT="$HOME/build/vcpkg"
export VCPKG_DISABLE_METRICS=1
if [[ -e "$VCPKG_ROOT" && ! -e "$VCPKG_ROOT/.git" ]]; then
	rm -rf "$VCPKG_ROOT"
fi
if [ ! -e "$VCPKG_ROOT" ]; then
	git clone https://github.com/Microsoft/vcpkg.git "$VCPKG_ROOT"
fi
pushd "$VCPKG_ROOT"
git fetch --all --prune --tags
git status
git checkout .
git checkout "$VCPKG_VERSION"
./bootstrap-vcpkg.sh -disableMetrics
#./vcpkg integrate install
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/x64-linux.cmake
export VCPKG_DEFAULT_TRIPLET=x64-linux
#./vcpkg install llvm  # takes very long time
(
	set +e
	which cmake
	cmake --version
	if ! ./vcpkg install --clean-after-build --recurse "opencv[contrib,nonfree,ade,opencl]"; then
		for log in "$VCPKG_ROOT/buildtrees"/**/*-{out,err}.log; do
			echo "=== $log"
			cat "$log"
		done
		exit 1
	fi
)
# remove build artifacts to save CI cache space
rm -rf "$VCPKG_ROOT/downloads" "$VCPKG_ROOT/buildtrees" "$VCPKG_ROOT/packages"
popd
