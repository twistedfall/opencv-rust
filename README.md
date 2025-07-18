# Rust OpenCV bindings

[![Build status](https://github.com/twistedfall/opencv-rust/workflows/opencv-rust/badge.svg)](https://github.com/twistedfall/opencv-rust/actions/workflows/opencv-rust.yml)
[![Documentation](https://docs.rs/opencv/badge.svg)](https://docs.rs/opencv)
[![Package](https://img.shields.io/crates/v/opencv.svg)](https://crates.io/crates/opencv)

Rust bindings for the popular [OpenCV](https://opencv.org/) computer vision library.

The API is usable, but unstable and not very battle-tested; use at your own risk.

[Changelog](https://github.com/twistedfall/opencv-rust/blob/master/CHANGES.md) |
[Troubleshooting](https://github.com/twistedfall/opencv-rust/blob/master/TROUBLESHOOTING.md) |
[Support the project](https://github.com/sponsors/twistedfall)

## Quickstart

Make sure the supported OpenCV version (3.4, 4.x or 5.x) and Clang (part of LLVM, needed for automatic binding
generation) are installed in your system.

Update your Cargo.toml

```toml
opencv = "0.95.0"
```

Import prelude

```rust
use opencv::prelude::*;
```

## Getting OpenCV

See [INSTALL.md](https://github.com/twistedfall/opencv-rust/blob/master/INSTALL.md) for instructions on how to install required
system dependencies.

## Troubleshooting

See [TROUBLESHOOTING.md](https://github.com/twistedfall/opencv-rust/blob/master/TROUBLESHOOTING.md) for some common issues and
their solutions.

## Environment variables

The following variables must be set when building without `pkg_config`, `cmake` or `vcpkg`. You can set them
on any platform, the specified values will override those automatically discovered.

* `OPENCV_LINK_LIBS`
  Comma separated list of library names to link to. `.lib`, `.so` or `.dylib` extension is optional. For every
  library you can specify optional "dylib=", "static=" or "framework=" prefix to indicate the specific type.
  E.g. "opencv_world411", "framework=OpenCL".

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

* `OPENCV_CLANG_ARGS`
  Allow custom arguments for generating and parsing code with clang, see
  the [documentation for clang arguments](https://docs.rs/clang/latest/clang/struct.Parser.html#method.arguments).

* clang crate environment variables
  See crate's [README](https://github.com/KyleMayes/clang-sys/blob/master/README.md#environment-variables)

## Cargo features

* There is a feature named after each OpenCV module (e.g. `imgproc`, `highgui`, etc.). They are all enabled by
  default, but if a corresponding module is not found then it will silently be ignored. If you need to select a
  specific set of modules be sure to disable the default features and provide the required feature set:
  ```
  opencv = { version = ..., default-features = false, features = ["calib3d", "features2d", "flann"]}
  ```
* `clang-runtime` - enables the runtime detection of libclang (`runtime` feature of `clang-sys`). Useful as a
  workaround for when your dependencies (like `bindgen`) pull in `clang-sys` with hard `runtime` feature.
* `rgb` - allow using [`rgb`](https://crates.io/crates/rgb) crate types as `Mat` elements
* `f16` - add intergration with `f16` type from the `half` crate

## API details

[API Documentation](https://docs.rs/opencv) is automatically translated from OpenCV's doxygen docs. Most
likely you'll still want to refer to the official [OpenCV C++ documentation](https://docs.opencv.org/master)
as well.

### OpenCV version support

The following OpenCV versions are supported at the moment:

* 3.4
* 4.x
* 5.x (preliminary)

### Minimum rustc version (MSRV)

Currently, Rust version 1.77.0 or later is required. General policy is that rust version from 1 year ago is supported.
Bumping versions older than that is not considered a breaking change.

### Platform support

Currently, the main development and testing of the crate is performed on Linux, but other major platforms are
also supported: macOS and Windows.

For some more details please refer to the CI build scripts:
[Linux OpenCV install](https://github.com/twistedfall/opencv-rust/blob/master/ci/install-ubuntu.sh),
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
* `=` → `set()`
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
tested and supported. Since version 0.94.0 the support of OpenCV 3.2 is removed from the codebase.

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
