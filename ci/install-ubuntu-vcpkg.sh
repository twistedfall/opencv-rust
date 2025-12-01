#!/bin/bash

set -xeu

# Remove unused files to increase available disk space
sudo rm -rf /usr/local/.ghcup
sudo rm -rf /opt/hostedtoolcache/CodeQL
sudo rm -rf /usr/local/lib/android/sdk/ndk
sudo rm -rf /usr/share/dotnet
sudo rm -rf /opt/ghc
sudo rm -rf /usr/local/share/boost

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

export VCPKG_DEFAULT_TRIPLET=x64-linux

script_dir="$(dirname "$(readlink -f "$BASH_SOURCE")")"

. "$script_dir/install-vcpkg.sh"
