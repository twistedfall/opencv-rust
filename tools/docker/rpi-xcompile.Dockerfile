# Crosscompilation to Raspberry Pi on RasiOS using system OpenCV
#
# Building this image requries `qemu-arm` to be present on the host system and the corresponding `binfmt-misc` set up (see
# e.g. https://wiki.debian.org/QemuUserEmulation, only `Installing packages` should be enough).
#
# After the successful build you will have an image configured for cross-compilation to Raspberry Pi. It will contain the
# sample build script `/usr/local/bin/cargo-xbuild` that you can check for the correct environment setup and the specific
# command line arguments to use when cross-compiling the project inside the container created from that image.


# Download and extract rpi root filesystem
FROM alpine:3.22

ADD https://downloads.raspberrypi.org/raspios_lite_armhf/images/raspios_lite_armhf-2025-10-02/2025-10-01-raspios-trixie-armhf-lite.img.xz /raspios-lite-armhf.img.xz

# chmod calls are necessary because 7zip doesn't preserve file permissions during extraction
RUN set -xeu && \
    apk add xz 7zip && \
    unxz /raspios-lite-armhf.img.xz && \
    7z x -so /raspios-lite-armhf.img 1.img > rootfs.img && \
    7z x -snld -o/rpi-root rootfs.img && \
    chmod -R 755 /rpi-root/bin /rpi-root/boot /rpi-root/dev /rpi-root/etc /rpi-root/home /rpi-root/lib /rpi-root/media \
                 /rpi-root/mnt /rpi-root/opt /rpi-root/run /rpi-root/sbin /rpi-root/srv /rpi-root/usr /rpi-root/var && \
    chmod -R 1777 /rpi-root/tmp && \
    chmod -R 555 /rpi-root/sys /rpi-root/proc && \
    chmod -R 700 /rpi-root/root

# Prepare the root of the rpi filesystem, it's going to be used later for cross-compilation
# This step requries qemu-arm to be present on the host system and the corresponding binfmt-misc set up
FROM scratch

COPY --from=0 /rpi-root /

# initramfs-tools breaking dist-upgrade in Docker:
# mkinitramfs: failed to determine device for /
RUN set -xeu && \
    DEBIAN_FRONTEND=noninteractive apt-get purge -y initramfs-tools

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
    DEBIAN_FRONTEND=noninteractive apt-get install -y libopencv-dev


# Create the image that will be used for cross-compilation
FROM ubuntu:24.04

COPY --from=1 / /rpi-root

RUN set -xeu && \
    apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get dist-upgrade -y && \
    apt-get autoremove -y --purge && \
    apt-get -y autoclean

RUN set -xeu && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y clang libclang-dev lld curl git build-essential cmake

RUN set -xeu && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile=minimal && \
    rm -rf /root/.rustup/tmp/* # warning: could not delete temp directory: /root/.rustup/tmp/szyc3h06vricp83o_dir

ENV PATH="${PATH}:/root/.cargo/bin"

RUN set -xeu && \
    rustup target add arm-unknown-linux-gnueabihf

RUN set -xeu && \
    echo "[target.arm-unknown-linux-gnueabihf]\nlinker = \"clang-rpi\"" > /root/.cargo/config.toml

RUN echo '#!/bin/bash\n\
RPI_ROOT="/rpi-root"\n\
clang --target=arm-unknown-linux-gnueabihf -fuse-ld=lld --sysroot="$RPI_ROOT" --gcc-install-dir="$RPI_ROOT/usr/lib/gcc/arm-linux-gnueabihf/14" -Wl,-z,notext -Wno-unused-command-line-argument "$@"' > /usr/local/bin/clang-rpi && \
    chmod +x /usr/local/bin/clang-rpi

RUN echo '#!/bin/bash\n\
RPI_ROOT="/rpi-root"\n\
export OpenCV_DIR="$RPI_ROOT/usr/lib/arm-linux-gnueabihf/cmake"\n\
export CC_arm_unknown_linux_gnueabihf=clang-rpi\n\
export CXX_arm_unknown_linux_gnueabihf=clang-rpi\n\
export CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_LINKER=clang-rpi\n\
export BINDGEN_EXTRA_CLANG_ARGS_arm_unknown_linux_gnueabihf="--sysroot=${RPI_ROOT}"\n\
export CMAKE_CROSSCOMPILING=TRUE\n\
cargo build -vv --target arm-unknown-linux-gnueabihf' > /usr/local/bin/cargo-xbuild && chmod +x /usr/local/bin/cargo-xbuild
