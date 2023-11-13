# Crosscompilation to Raspberry Pi on RasiOS using system OpenCV
#
# Building this image requries `qemu-arm` to be present on the host system and the corresponding `binfmt-misc` set up (see
# e.g. https://wiki.debian.org/QemuUserEmulation, only `Installing packages` should be enough).
#
# After the successful build you will have an image configured for cross-compilation to Raspberry Pi. It will contain the
# sample build script `/usr/local/bin/cargo-xbuild` that you can check for the correct environment setup and the specific
# command line arguments to use when crosscompiling the project inside the container created from that image.


# Download and extract rpi root filesystem
FROM alpine:3.18

RUN set -xeu && \
    apk add xz

ADD https://downloads.raspberrypi.org/raspios_lite_armhf/root.tar.xz /

RUN set -xeu && \
    mkdir "/rpi-root" && \
    tar xpaf /root.tar.xz -C /rpi-root


# Prepare the root of the rpi filesystem, it's going to be used later for crosscompilation
# This step requries qemu-arm to be present on the host system and the corresponding binfmt-misc set up
FROM scratch

COPY --from=0 /rpi-root /

RUN set -xeu && \
    apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get dist-upgrade -y && \
    apt-get autoremove -y --purge && \
    apt-get -y autoclean

RUN set -xeu && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y symlinks

# error: undefined symbol: _dl_pagesize (and __pointer_chk_guard_local)
# solution: fix the rpi-root symlink /usr/lib/arm-linux-gnueabihf/libpthread.so to be relative and point to ../../../lib/arm-linux-gnueabihf/libpthread.so.0
# source: https://github.com/Azure/azure-iot-sdk-c/issues/1093
RUN set -xeu && \
    symlinks -cr /

# Specify dependencies that you need to have on rpi
RUN set -xeu && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y libudev-dev libsqlite3-dev libopencv-dev libstrophe-dev libcamera-dev pkg-config


# Create the image that will be used for crosscompilation
FROM ubuntu:22.04

COPY --from=1 / /rpi-root

RUN set -xeu && \
    apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get dist-upgrade -y && \
    apt-get autoremove -y --purge && \
    apt-get -y autoclean

RUN set -xeu && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y clang libclang-dev lld curl git build-essential pkg-config cmake

RUN set -xeu && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile=minimal && \
    rm -rf /root/.rustup/tmp/* # warning: could not delete temp directory: /root/.rustup/tmp/szyc3h06vricp83o_dir

RUN set -xeu && \
    echo "[net]\ngit-fetch-with-cli = true\n[target.arm-unknown-linux-gnueabihf]\nlinker = \"clang-rpi\"" > /root/.cargo/config

ENV PATH="${PATH}:/root/.cargo/bin"

RUN set -xeu && \
    rustup target add arm-unknown-linux-gnueabihf

RUN echo '#!/bin/bash\n\
RPI_ROOT="/rpi-root"\n\
clang --target=arm-unknown-linux-gnueabihf -fuse-ld=lld --sysroot="$RPI_ROOT" --gcc-toolchain="$RPI_ROOT" "$@"' > /usr/local/bin/clang-rpi && chmod +x /usr/local/bin/clang-rpi

RUN echo '#!/bin/bash\n\
RPI_ROOT="/rpi-root"\n\
export PKG_CONFIG_SYSROOT_DIR="$RPI_ROOT"\n\
export PKG_CONFIG_LIBDIR="$RPI_ROOT/usr/lib/arm-linux-gnueabihf/pkgconfig"\n\
export CC="clang-rpi"\n\
export CXX="clang-rpi"\n\
cargo build -vv --target arm-unknown-linux-gnueabihf' > /usr/local/bin/cargo-xbuild && chmod +x /usr/local/bin/cargo-xbuild
