# Rust OpenCV bindings

[![Build status](https://github.com/twistedfall/opencv-rust/workflows/opencv-rust/badge.svg)](https://github.com/twistedfall/opencv-rust/actions/workflows/opencv-rust.yml)
[![Documentation](https://docs.rs/opencv/badge.svg)](https://docs.rs/opencv)
[![Package](https://img.shields.io/crates/v/opencv.svg)](https://crates.io/crates/opencv)

Experimental Rust bindings for OpenCV 3 and 4.

The API is usable, but unstable and not very battle-tested; use at your own risk.

[Changelog](https://github.com/twistedfall/opencv-rust/blob/master/CHANGES.md)

## Quickstart

Make sure the supported OpenCV version (3.4 or 4.x) and Clang (part of LLVM, needed for automatic binding
generation) are installed in your system.

Update your Cargo.toml
```toml
opencv = "0.85.1"
```

Import prelude
```rust
use opencv::prelude::*;
```

## Getting OpenCV

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
  vcpkg install llvm opencv4[contrib,nonfree]
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
  --install`), XCode (from AppStore) or `llvm` (from Brew). You most probably need to also check the item 7 of the
  troubleshooting below.

### Manual build

You can of course always compile OpenCV of the version you prefer manually. This is also supported, but it
requires some additional configuration.

You need to set up the following environment variables to point to the installed files of your OpenCV build:
`OPENCV_LINK_LIBS`, `OPENCV_LINK_PATHS` and `OPENCV_INCLUDE_PATHS` (see below for details).

### Static build

Static linking to OpenCV is supported and tested at least on Linux. For some hints on building OpenCV statically
please check this [comment](https://github.com/twistedfall/opencv-rust/issues/364#issuecomment-1308794985). Also,
you can get some information on how to perform the build in CI scripts:
[install-focal.sh](https://github.com/twistedfall/opencv-rust/blob/master/ci/install-focal.sh) and
[script.sh](https://github.com/twistedfall/opencv-rust/blob/master/ci/script.sh), search for `non_static_version` variable.

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

## Troubleshooting

1. One of the common problems is link errors in the end of the build.

   Be sure to set up the relevant environment variables that will allow the linker to find the libraries it
   needs (see below).

2. You're getting runtime errors like:
   ```
   thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error { code: -215, message: "OpenCV(4.2.0) /build/opencv/src/opencv-4.2.0/modules/highgui/src/window.cpp:384: error: (-215:Assertion failed) size.width>0 && size.height>0 in function \'imshow\'\n" }', src/libcore/result.rs:1188:5
   ```
   ```
   thread 'extraction::tests::test_contour_matching' panicked at 'called `Result::unwrap()` on an `Err` value: Error { code: -215, message: "OpenCV(4.1.1) /tmp/opencv-20190908-41416-17rm3ol/opencv-4.1.1/modules/core/src/matrix_wrap.cpp:79: error: (-215:Assertion failed) 0 <= i && i < (int)vv.size() in function \'getMat_\'\n" }', src/libcore/result.rs:1165:5
   ```

   These errors (note the .cpp source file and `Error` return value) are coming from OpenCV itself, not from
   the crate. It means that you're using the OpenCV API incorrectly, e.g. passing incompatible or unexpected
   arguments. Please refer to the OpenCV documentation for details.

3. You're getting errors that methods don't exist or not implemented for specific `struct`s, but you can see
   them in the documentation and in the crate source.

   Be sure to import ```use opencv::prelude::*;```. The crate contains a lot of traits that need to be imported
   first.

4. On Windows, you're getting the `(exit code: 0xc0000135, STATUS_DLL_NOT_FOUND)` error when running the
   compiled binary.

   That often means that Windows can't find the OpenCV library dll. Be sure to set up `PATH` environment
   variable correctly or copy the dll next to the binary you're trying to run. Check
   [that](https://github.com/twistedfall/opencv-rust/issues/118#issuecomment-619608278) guide too.

5. On Windows with VCPKG you're getting a lot of linking errors in multiple files like in
   [this issue](https://github.com/twistedfall/opencv-rust/issues/161).

   Unless you're doing a very specific build, you want to have environment variable `VCPKGRS_DYNAMIC` set to
   "1".

6. On Windows with OpenCV 4.6.0 you're getting linking errors related to `img_hash` module like in
   [this issue](https://github.com/twistedfall/opencv-rust/issues/360).

   Be sure to add `opencv_img_hash460` to your `OPENCV_LINK_LIBS` environment variable because it's being built as a separate
   file.

7. On macOS you're getting the `dyld: Library not loaded: @rpath/libclang.dylib` error during the build process.

   OS can't find `libclang.dylib` dynamic library because it resides in a non-standard path, set up
   the `DYLD_FALLBACK_LIBRARY_PATH` environment variable to point to the path where libclang.dylib can be
   found, e.g. for Command Line Tools:
   ```
   export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/usr/lib/"
   ```

   or XCode:
   ```
   export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/usr/lib/"
   ```

   You might be running into the issue on the recent macOS versions where this environment variable remains empty after setting,
   please check [this issue](https://github.com/twistedfall/opencv-rust/issues/343) for some workarounds.

8. You're getting the panic: ```a `libclang` shared library is not loaded on this thread```.

   Enable the `clang-runtime` feature. The reason for the issue is that some `clang-sys` crate can either link to the
   corresponding dynamic library statically or load it at runtime based on whether its feature `runtime` is enabled.
   And if enabled this feature starts to apply to all crates that depend on `clang-sys` even if they didn't enable that
   feature themselves (applicable with Rust `edition` before 2021 and Cargo `resolver` before 2).

9. You're getting `'limits' file not found` error during crate build.

   This error is caused by the missing/invalid installation of C++ standard library (e.g. libstdc++ for GCC). To fix this make
   sure that the toolchain you're using has the corresponding C++ standard library. The toolchain is used through `libclang`, so
   to get useful diagnostic info run:
   ```shell
   clang -E -x c++ - -v
   ```
   Look for `Selected GCC installation` and `#include <...> search starts here` to get the sense of what system toolchain is used
   by clang. Refer to this [issue](https://github.com/twistedfall/opencv-rust/issues/322) for more fixes and workarounds.

## Reporting issues

If you still have trouble using the crate after going through the Troubleshooting steps please fill free to
report it to the [bugtracker](https://github.com/twistedfall/opencv-rust/issues).

When reporting an issue please state:

1. Operating system
2. The way you installed OpenCV: package, official binary distribution, manual compilation, etc.
3. OpenCV version
4. Attach the full output of the following command from your project directory:
   ```shell script
   RUST_BACKTRACE=full cargo build -vv
   ```

## Environment variables

The following variables must be set when building without `pkg_config`, `cmake` or `vcpkg`. You can set them
on any platform, the specified values will override those automatically discovered.

* `OPENCV_LINK_LIBS`
  Comma separated list of library names to link to. `.lib`, `.so` or `.dylib` extension is optional. If you
  specify the ".framework" extension then build script will link a macOS framework instead of plain shared
  library.
  E.g. "opencv_world411".

  If this list starts with '+' (plus sign) then the specified items will be appended to whatever the system
  probe returned. E.g. a value of "+dc1394" will do a system discovery of the OpenCV library and its linked
  libraries and then will additionally link `dc1394` library at the end. Can be useful if the system probe
  produces a mostly working setup, but has incomplete link list, or the order is wrong (especially important
  during static linking).

* `OPENCV_LINK_PATHS`
  Comma separated list of paths to search for libraries to link. E.g. "C:\tools\opencv\build\x64\vc15\lib".
  The path list can start with '+', see `OPENCV_LINK_LIBS` for a detailed explanation (e.g.
  "+/usr/local/lib").

* `OPENCV_INCLUDE_PATHS`
  Comma separated list of paths to search for system include files during compilation. E.g.
  "C:\tools\opencv\build\include". One of the directories specified therein must contain
  "opencv2/core/version.hpp" or "core/version.hpp" file, it's used to detect the version of the headers.
  The path list can start with '+', see `OPENCV_LINK_LIBS` for a detailed explanation (e.g.
  "+/opt/cuda/targets/x86_64-linux/include/").

The following variables are rarely used, but you might need them under some circumstances:

* `OPENCV_PACKAGE_NAME`
  In some cases you might want to override the pkg-config, cmake or vcpkg package name, you can use this
  environment variable for that. If you set it pkg-config will expect to find the file with that name and `.pc`
  extension in the package directory. Cmake will look for that file with `.cmake` extension. And vcpkg will use
  that name to try to find package in `packages` directory under `VCPKG_ROOT`. You can also use separate
  environment variables to set different package names for different package systems:
    * `OPENCV_PKGCONFIG_NAME`
    * `OPENCV_CMAKE_NAME`
    * `OPENCV_VCPKG_NAME`

* `OPENCV_CMAKE_BIN`
  Path to cmake binary (used in OpenCV discovery process using cmake). If not set then just "cmake" will be
  used. For example, you can set something like "/usr/local/bin/cmake" here.

* `OPENCV_DISABLE_PROBES`
  Comma separated list of OpenCV package auto-discovery systems to exclude from running. Might be useful if
  one of the higher priority systems is producing incorrect results. Can contain the following values:
    * environment - reads data only from the `OPENCV_LINK_LIBS`, `OPENCV_LINK_PATHS` and `OPENCV_INCLUDE_PATHS`
      environment variables
    * pkg_config
    * cmake
    * vcpkg_cmake - like vcpkg, but only uses vcpkg for path discovery, the actual OpenCV probe is done using
      cmake (cmake related environment variables are applicable with this probe)
    * vcpkg

* `OPENCV_MSVC_CRT`
  Allows selecting the CRT library when building with MSVC for Windows. Allowed values are `"static"` for `/MT`
  and `"dynamic"` for `/MD`.

The following variables affect the building the of the `opencv` crate, but belong to external components:

* `PKG_CONFIG_PATH`
  Where to look for `*.pc` files see the [man pkg-config](https://linux.die.net/man/1/pkg-config)
  Path specified here must contain `opencv.pc` (pre OpenCV 4) or `opencv4.pc` (OpenCV 4 and later).

* `VCPKG_ROOT`, `VCPKGRS_DYNAMIC` and `VCPKGRS_TRIPLET`
  The root of `vcpkg` installation, flag allowing use of `*.dll` libraries and selected `vcpkg` triplet, see the
  [documentation for `vcpkg` crate](https://docs.rs/vcpkg)

* `OpenCV_DIR`
  The directory that contains OpenCV package cmake files. Usually there are `OpenCVConfig.cmake`,
  `OpenCVConfig-version.cmake` and `OpenCVModules.cmake` in it.

* `LD_LIBRARY_PATH`
  On Linux it sets the list of directories to look for the installed `*.so` files during runtime.
  [Linux documentation](https://tldp.org/HOWTO/Program-Library-HOWTO/shared-libraries.html) has more info.
  Path specified here must contain `libopencv_*.so` files.

* `DYLD_LIBRARY_PATH` and `DYLD_FALLBACK_LIBRARY_PATH`
  Similar to `LD_LIBRARY_PATH`, but for loading `*.dylib` files on macOS, see [man dyld](https://man.cx/dyld(1))
  and [this SO answer](https://stackoverflow.com/a/3172515) for more info. Path specified here must contain
  `*.dylib` files.

* `PATH`
  Windows searches for `*.dll`s in `PATH` among other places, be sure to set it up, or copy required OpenCV
  `*.dll`s next to your binary. Be sure to specify paths in UNIX style (/C/Program Files/Dir) because colon
   in `PATH` might be interpreted as the entry separator. Summary [here](https://stackoverflow.com/a/6546427).

* clang crate environment variables
  See crate's [README](https://github.com/KyleMayes/clang-sys/blob/master/README.md#environment-variables)

## Cargo features
* There is a feature named after each OpenCV module (e.g. `imgproc`, `highgui`, etc.). They are all enabled by
  default, but if a corresponding module is not found then it will silently be ignored. If you need to select a
  specific set of modules be sure to disable the default features and provide the required feature set:
  ```
  opencv = { version = ..., default-features = false, features = ["calib3d", "features2d", "flann"]}
  ```
* `rgb` - allow using [`rgb`](https://crates.io/crates/rgb) crate types as `Mat` elements

## API details

[API Documentation](https://docs.rs/opencv) is automatically translated from OpenCV's doxygen docs. Most
likely you'll still want to refer to the official [OpenCV C++ documentation](https://docs.opencv.org/master)
as well.

### OpenCV version support

The following OpenCV versions are supported at the moment:
* 3.4
* 4.x

### Minimum rustc version (MSRV)

Currently, version 1.59.0 or later is required.

### Platform support

Currently, the main development and testing of the crate is performed on Linux, but other major platforms are
also supported: macOS and Windows.

For some more details please refer to the CI build scripts:
[Linux OpenCV install](https://github.com/twistedfall/opencv-rust/blob/master/ci/install-focal.sh),
[macOS OpenCV install as framework](https://github.com/twistedfall/opencv-rust/blob/master/ci/install-macos-framework.sh),
[macOS OpenCV install via brew](https://github.com/twistedfall/opencv-rust/blob/master/ci/install-macos-brew.sh),
[Windows OpenCV install via Chocolatey](https://github.com/twistedfall/opencv-rust/blob/master/ci/install-windows-chocolatey.sh),
[Windows OpenCV install via vcpkg](https://github.com/twistedfall/opencv-rust/blob/master/ci/install-windows-vcpkg.sh),
[Test runner script](https://github.com/twistedfall/opencv-rust/blob/master/ci/script.sh).

### Functionality

Generally the crate tries to only wrap OpenCV API and provide some convenience functions
to be able to use it in Rust easier. We try to avoid adding any functionality besides
that.

### Errors

Most functions return a `Result` to expose a potential C++ exception. Although some methods like property reads
or functions that are marked CV_NOEXCEPT in the OpenCV headers are infallible and return a naked value.

### CV_MAKETYPE

`CV_MAKETYPE` and related `CV_MAT_DEPTH` constant functions are available to replace the corresponding OpenCV macros.
Yet it's usually easier to call `::opencv_type()` function on the corresponding Rust type. E.g.:
```rust
let t = u16::opencv_type(); // equivalent to CV_MAKETYPE(CV_16U, 1)
let t = Vec2f::opencv_type(); // equivalent to CV_MAKETYPE(CV_32F, 2)
```

### C++ operators
Some C++ operators are supported, they are converted to the corresponding functions on Rust side. Here is the
list with the corresponding function name:
* `[]` → `get()` or `get_mut()`
* `+`, `-` → `add()`, `sub()`
* `*`, `/` → `mul()`, `div()`
* `()` (function call) → `apply()`
* `*` (deref) → `try_deref()` or `try_deref_mut()`
* `==`, `!=` → `equals()`, `not_equals()`
* `>`, `>=` → `greater_than()`, `greater_than_or_equal()`
* `<`, `<=` → `less_than()`, `less_than_or_equal()`
* `++`, `--` → `incr()`, `decr()`
* `&`, `|`, `^` → `and()`, `or()`, `xor()`
* `!` → `negate()`

### Class fields

Fields of OpenCV classes are accessible through setters and getters. Those functions are infallible, they
return the value directly instead of `Result`.

### Infallible functions

For infallible functions (like setters) that accept `&str` values the following logic applies: if a Rust
string passed as argument contains null byte then this string will be truncated up to that null byte. So if
for example you pass "123\0456" to the setter, the property will be set to "123".

### Callbacks

Some API functions accept callbacks, e.g. `set_mouse_callback`. While currently it's possible to successfully
use those functions there are some limitations to keep in mind. Current implementation of callback handling
leaks the passed callback argument. That means that the closure used as a callback will never be freed during
the lifetime of a program and moreover Drop will not be called for it. There is a plan to implement possibility
to be able to free at least some closures.

### Unsafety

Although the crate tries to provide an ergonomic Rust interface for OpenCV, don't expect
Rust safety guarantees at this stage. It's especially true for the borrow-checking and the
shared mutable ownership. Notable example would be `Mat` which is a reference counted
object in its essence. You can own a seemingly separate `Mat` in Rust terms, but
it's going to be a mutable reference to the other `Mat` under the hood. Treat safety
of the crate's API as you would treat one of C++, use `clone()` when needed.

## Contrib modules

To be able to use some modules you need to have [`opencv_contrib`](https://github.com/opencv/opencv_contrib)
installed. You can find the full list of contrib modules [here](https://github.com/opencv/opencv_contrib/tree/master/modules).

## Missing modules and functions

While most of the API is covered, for various reasons (that might no longer hold) there are modules and
functions that are not yet implemented. If a missing module/function is near and dear to you, please file an
issue (or better, open a pull request!).

## The binding strategy

This crate works similar to the model of python and java's OpenCV wrappers - it uses libclang to parse the
OpenCV C++ headers, generates a C interface to the C++ API, and wraps the C interface in Rust.

All the major modules in the C++ API are merged together in a huge `cv::` namespace. We instead made one rust
module for each major OpenCV module. So, for example, C++ `cv::Mat` is `opencv::core::Mat` in this crate.

The methods and field names have been snake_cased. Methods arguments with default value lose these default
values, but they are reported in the API documentation.

Overloaded methods have been mostly manually given different names or automatically renamed to *_1, *_2, etc.

## Older OpenCV branches support
### OpenCV 2

If you can't use OpenCV 3.x or higher, the (no longer maintained) `0.2.4` version of this crate is known to
work with OpenCV `2.4.7.13` (and probably other 2.4 versions). Please refer to the README.md file for that
version because the crate has gone through the considerable rewrite since.

### OpenCV 3.2

The last version with confirmed OpenCV 3.2 support is 0.75.0, after that this branch of OpenCV is no longer
tested and supported. It may still work though.

## Contributor's Guide

The binding generator code lives in a separate crate under [binding-generator](binding-generator). During the
build phase it creates bindings from the header files and puts them into [bindings](bindings) directory. Those
are then transferred to [src](src) for the consumption by the crate users.

The crate itself, as imported by users, consists of generated rust code in [src](src) committed to the repo.
This way, users don't have to handle the code generation overhead in their builds. When developing this crate,
you can test changes to the binding generation using `cargo build -vv`. When changing the `binding-generator`,
be sure to push changes to the generated code!

If you're looking for things to improve be sure to search for `todo` and `fixme` labels in the project
source, those usually carry the comment of what exactly needs to be fixed.

The license for the original work is [MIT](https://opensource.org/licenses/MIT).

Special thanks to [ttacon](https://github.com/ttacon) for yielding the crate name.
