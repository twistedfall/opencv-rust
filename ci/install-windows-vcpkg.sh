#!/bin/bash

set -xeu

# remove system vcpkg
rm -rf "$VCPKG_INSTALLATION_ROOT"

# install llvm from choco in place of vcpkg to speed things up
choco install -y llvm --version "$CHOCO_LLVM_VERSION"

# 2024-04-15, cmake 3.29.1 doesn't work so well with vcpkg, remove this line when cmake is 3.29.2+
# https://github.com/microsoft/vcpkg/issues/37968
choco upgrade -y cmake --version 3.29.0

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
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/x64-windows.cmake
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/x64-windows-static.cmake
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/x86-windows.cmake
echo "set(VCPKG_BUILD_TYPE release)" >> triplets/community/x64-windows-static-md.cmake
export VCPKG_DEFAULT_TRIPLET=x64-windows
#./vcpkg install llvm  # takes very long time
(
	set +e
	which cmake
	cmake --version
	if ! ./vcpkg install --clean-after-build --recurse "opencv[contrib,nonfree,ade]"; then
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
