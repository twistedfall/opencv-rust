#!/bin/bash

set -xeu

if [[ "${VCPKG_VERSION:-}" == "" ]]; then
	echo "Must set VCPKG_VERSION environment var"
	exit 1
fi

export VCPKG_ROOT="$HOME/build/vcpkg"
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
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/arm64-osx.cmake
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/x64-osx.cmake
export VCPKG_DEFAULT_TRIPLET=arm64-osx
(
	set +e
	which cmake
	cmake --version
	if ! ./vcpkg install --clean-after-build --recurse "opencv[contrib,nonfree,ade,opencl]"; then
		for log in "$VCPKG_ROOT/buildtrees"/**/*out.log; do
			echo "=== $log"
			cat "$log"
		done
		exit 1
	fi
)
# remove build artifacts to save CI cache space
rm -rf "$VCPKG_ROOT/downloads" "$VCPKG_ROOT/buildtrees" "$VCPKG_ROOT/packages"
popd
