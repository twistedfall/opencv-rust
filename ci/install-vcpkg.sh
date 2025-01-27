#!/bin/bash

set -xeu

export VCPKG_ROOT="$HOME/build/vcpkg"
export VCPKG_DISABLE_METRICS=1
if [[ -e "$VCPKG_ROOT" && ! -e "$VCPKG_ROOT/.git" ]]; then
	rm -rf "$VCPKG_ROOT"
fi
if [ ! -e "$VCPKG_ROOT" ]; then
	git clone https://github.com/Microsoft/vcpkg.git "$VCPKG_ROOT"
fi
cd "$VCPKG_ROOT"
git fetch --all --prune --tags
git status
git checkout .
git checkout "$VCPKG_VERSION"
./bootstrap-vcpkg.sh -disableMetrics
#./vcpkg integrate install
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/x64-linux.cmake
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/arm64-osx.cmake
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/x64-osx.cmake
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/x64-windows.cmake
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/x64-windows-static.cmake
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/x86-windows.cmake
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/community/x64-windows-static-md.cmake
#./vcpkg install llvm  # takes very long time
(
	set +e
	which cmake
	cmake --version
	if ! ./vcpkg install --clean-after-build --recurse "opencv4[$VCPKG_FEATURES]"; then
		for log in "$VCPKG_ROOT/buildtrees"/**/*-{out,err}.log; do
			echo "=== $log"
			cat "$log"
		done
		exit 1
	fi
)
# remove build artifacts to save CI cache space
rm -rf "$VCPKG_ROOT/downloads" "$VCPKG_ROOT/buildtrees" "$VCPKG_ROOT/packages"
