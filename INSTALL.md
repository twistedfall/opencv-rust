## Getting OpenCV

This crates requires OpenCV system library to be present in your system. See below for some of the more common
setups:

### Linux

#### Arch Linux:

OpenCV package in Arch is suitable for this:

`pacman -S clang qt6-base opencv`

and additionally to support more OpenCV modules:

`pacman -S vtk glew fmt openmpi`

#### Ubuntu:

`apt install libopencv-dev clang libclang-dev`

#### Opensuse:

`zypper install opencv-devel clang-devel gcc-c++`

#### Other Linux:
You have several options of getting the OpenCV library:

* install it from the repository, make sure to install `-dev` packages because they contain headers necessary
  for the crate build (also check that your package contains `pkg_config` or `cmake` files).

* build OpenCV manually and set up the following environment variables prior to building the project with
  `opencv` crate:
  * `PKG_CONFIG_PATH` for the location of `*.pc` files or `OpenCV_DIR` for the location of `*.cmake` files
  * `LD_LIBRARY_PATH` for where to look for the installed `*.so` files during runtime

Additionally, please make sure to install `clang` package or its derivative that contains `libclang.so` and
`clang` binary.
  * Gentoo, Fedora: `clang`
  * Debian, Ubuntu: `clang` and `libclang-dev`

### Windows package

Installing OpenCV is easy through the following sources:

* from [chocolatey](https://chocolatey.org), also install `llvm` package, it's required for building:
  ```shell script
  choco install llvm opencv
  ```
  also set `OPENCV_LINK_LIBS`, `OPENCV_LINK_PATHS` and `OPENCV_INCLUDE_PATHS` environment variables (see below
  for details).

  Also, check the user guides [here](https://github.com/twistedfall/opencv-rust/issues/118#issuecomment-619608278)
  and [here](https://github.com/twistedfall/opencv-rust/issues/113#issue-596076777).

* from [vcpkg](https://docs.microsoft.com/en-us/cpp/build/vcpkg), also install `llvm` package,
  necessary for building:
  ```shell script
  vcpkg install llvm opencv4[contrib,nonfree,opencl]
  ```
  You most probably want to set environment variable `VCPKGRS_DYNAMIC` to "1" unless you're specifically
  targeting a static build.

### macOS package

Get OpenCV from homebrew:

* [homebrew](https://brew.sh):
  ```shell script
  brew install opencv
  ```
  You will also need a working C++ compiler and libclang, you can install Command Line Tools (`xcode-select
  --install`), XCode (from App Store) or `llvm` (from Brew). You most probably need to also check the item 7 of the
  [troubleshooting](https://github.com/twistedfall/opencv-rust/blob/master/TROUBLESHOOTING.md).

  If using system LLVM doesn't work for you, you can get it from homebrew:
  ```shell script
  brew install llvm
  ```

  If you have already installed OpenCV via Homebrew, there is no need to configure any environment variables specifically for OpenCV on macOS. The OpenCV installation can be automatically detected if it was installed through Homebrew.

  To take advantage of this automatic detection, refrain from setting the following environment variables:

  `OPENCV_LINK_LIBS`, `OPENCV_LINK_PATHS`, and `OPENCV_INCLUDE_PATHS`.

  However, make sure you have configured the following environment variables:

  `DYLD_FALLBACK_LIBRARY_PATH`, `LDFLAGS` and `LD_LIBRARY_PATH`.

  If you have not made any custom changes to your installation, the following settings should work for your setup:

  ```
  export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/usr/lib/"
  export LDFLAGS=-L/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/lib
  export LD_LIBRARY_PATH=${LD_LIBRARY_PATH}:/usr/local/lib  
  ```

### Manual build

You can of course always compile OpenCV of the version you prefer manually. This is also supported, but it
requires some additional configuration.

You need to set up the following environment variables to point to the installed files of your OpenCV build:
`OPENCV_LINK_LIBS`, `OPENCV_LINK_PATHS` and `OPENCV_INCLUDE_PATHS` (see below for details).

### Static build

Static linking to OpenCV is supported and tested at least on Linux. For some hints on building OpenCV statically
please check this [comment](https://github.com/twistedfall/opencv-rust/issues/364#issuecomment-1308794985). Also,
you can get some information on how to perform the build in CI scripts:
[install-ubuntu.sh](https://github.com/twistedfall/opencv-rust/blob/master/ci/install-ubuntu.sh) and
[script.sh](https://github.com/twistedfall/opencv-rust/blob/master/ci/script.sh), search for `"static"` string.

### Crosscompilation

Cross-compilation is supported to at least some extent. The ability to crosscompile projects using `opencv` from x86-64
Linux host machine to Raspberry Pi is tested regularly. Cross-compilation is notoriously difficult to set up, so you can
use this example [rpi-xcompile.Dockerfile](https://github.com/twistedfall/opencv-rust/blob/master/tools/docker/rpi-xcompile.Dockerfile).

```shell
docker build -t rpi-xcompile -f tools/docker/rpi-xcompile.Dockerfile tools
```

Building this image requries `qemu-arm` to be present on the host system and the corresponding `binfmt-misc` set up 
- see e.g. https://wiki.debian.org/QemuUserEmulation, only `Installing packages` should be enough for debian-based distros,
- for opensuse, install `qemu-linux-user` via zypper to set up the host correctly.

After the successful build you will have an image configured for cross-compilation to Raspberry Pi. It will contain the
sample build script `/usr/local/bin/cargo-xbuild` that you can check for the correct environment setup and the specific
command line arguments to use when crosscompiling the project inside the container created from that image.
