# Rust OpenCV bindings

[![Build status](https://travis-ci.org/twistedfall/opencv-rust.svg?branch=master)](https://travis-ci.org/twistedfall/opencv-rust)
[![Documentation](https://docs.rs/opencv/badge.svg)](https://docs.rs/opencv)
[![Package](https://img.shields.io/crates/v/opencv.svg)](https://crates.io/crates/opencv)

Experimental Rust bindings for OpenCV 3 and 4.

The API is usable but unstable and not very battle-tested; use at your own risk.

## API

[API Documentation](https://docs.rs/opencv) is, to varying success,
translated from OpenCV's doxygen docs. Most likely you'll want to
refer to the official [OpenCV C++
documentation](https://docs.opencv.org/master/) as well.

### OpenCV version support

The following OpenCV versions are supported at the moment:
* 3.2
* 3.4
* 4.1

You can choose the target OpenCV version with the following Cargo features:
* `opencv-32`
* `opencv-34` (default)
* `opencv-41`

If you need support for `contrib` modules, also enable `contrib` feature.

### Quickstart

Make sure the supported OpenCV version is installed in your system along with `pkg_config`
files or `-dev` packages.

Update your Cargo.toml
```toml
opencv = "0.27"
```

Select OpenCV version if different from default in Cargo.toml:
```toml
opencv = {version = "0.27", default-features = false, features = ["opencv-41"]}
```

And enable usage of `contrib` modules:
```toml
opencv = {version = "0.27", default-features = false, features = ["opencv-41", "contrib"]}
```

Import prelude
```rust
use opencv::prelude::*;
```

### Minimum rustc version

Generally you should use the latest stable rustc to compile this crate.

### Platform support

Currently the main development and testing of the crate is performed on Linux, but other 
major platforms are also supported: Mac OS X and Windows.

Support for Windows is a bit tricky at the moment: you will need to set up `OPENCV_LINK_LIBS`,
`OPENCV_LINK_PATHS`, `OPENCV_INCLUDE_PATHS` and potentially `OPENCV_PYTHON3_BIN` environment
variables. Then build the crate with `buildtime-bindgen` feature enabled. If you get linking
errors try also building with `--release` flag.

Mac OS X build currently also requires `buildtime-bindgen` feature enabled.

Also refer to the corresponding [issue #6](https://github.com/twistedfall/opencv-rust/issues/6), 
[issue #70](https://github.com/twistedfall/opencv-rust/issues/70) and Travis
[build script](https://github.com/twistedfall/opencv-rust/blob/master/ci/script.sh).

### Features
* `opencv-32` - build against OpenCV 3.2.0, this feature is aimed primarily on stable Debian and
  Ubuntu users who can install OpenCV from the repository without having to compile it from the
  source
* `opencv-34` (default) - build against OpenCV 3.4.x
* `opencv-41` - build against OpenCV 4.1.x
* `contrib` - enable the usage of OpenCV contrib modules for corresponding OpenCV version
* `buildtime-bindgen` - regenerate all bindings, should only be used during the crate development
  or when building on Windows or Mac OS X, with this feature enabled the bundled headers are no
  longer used for the code generation, the ones from the installed OpenCV are
* `force-3rd-party-libs-discovery` - legacy feature that enables some additional logic for
  discovery of dependent libs, should not be needed anymore
* `docs-only` - internal usage, for building docs on [docs.rs](https://docs.rs/opencv)

### Functionality

Generally the crate tries to only wrap OpenCV API and provide some convenience functions
to be able to use it in Rust easier. We try to avoid adding any functionality besides
that.

### Callbacks

Some API functions accept callbacks, e.g. set_mouse_callback. While currently
it's possible to successfully use those functions there are some limitations to
keep in mind. Current implementation of callback handling keeps hold of the
passed callback argument forever. That means that the closure used as a callback
will never be freed during the lifetime of a program and moreover Drop will
not be called for it (they are stored in global static [`Slab`](https://crates.io/crates/slab)).
There is a plan to implement possibility to be able to free at least some of the
closures.

### Unsafety

Although crate tries to provide ergonomic Rust interface for OpenCV, don't expect
Rust safety guarantees at this stage. It's especially true for borrow checking and
shared mutable ownership. Notable example would be `Mat` which is a reference counted
object in its essence. You can own a seemingly separate `Mat` in Rust terms, but
it's going to be a mutable reference to the other `Mat` under the hood. Treat safety
of the crate's API as you would treat one of C++, use `clone()` when needed.

## Getting Started

The following external dependencies are required:
- python3
- OpenCV 3.2.0, 3.4.8 or 4.1.2

OpenCV is a complicated dependency with a lot of different
configurations that can break this crate since it relies on fragile
header parsing. Try these strategies in order, continuing to the next
if builds with this crate fail.

### See if you already have the right version of OpenCV installed

if OpenCV is installed, its version can be checked with:

```sh
opencv_version
```

### Install OpenCV

Install supported version of OpenCV. Check your platform's package manager or see the
upstream OpenCV
[installation guides](https://docs.opencv.org/master/df/d65/tutorial_table_of_content_introduction.html).

When your OpenCV is installed in a location that's not being picked up by build script
you can try setting one or both of the following environment vars prior to trying to
build the crate:

```bash
export PKG_CONFIG_PATH=/custom/prefix/lib/pkgconfig/
export LD_LIBRARY_PATH=/custom/prefix/lib/
```

Path specified by `PKG_CONFIG_PATH` must contain `opencv.pc` or `opencv4.pc` (for OpenCV 4.x).
Path specified by `LD_LIBRARY_PATH` must contain `libopencv_*.so` files.

### Environment variables

* `OPENCV_HEADER_DIR`
  During crate build it uses OpenCV headers supplied inside the crate for binding generation.
  If you want to use your own (system) headers supply `OPENCV_HEADER_DIR` environment variable.
  The directory in that environment variable should contain `opencv2` dir, e.g. `/usr/include` for
  OpenCV-3.4.x or `/usr/include/opencv4` for OpenCV-4.x.

* `OPENCV_PKGCONFIG_NAME`
  In some cases you might want to override the pkgconfig package name, you can use `OPENCV_PKGCONFIG_NAME`
  environment variable for that. If you set it pkgconfig will expect to find the file with that name
  and `.pc` extension in the package directory.
  
* `OPENCV_PYTHON3_BIN`
  During the build crate uses Python 3 to run header parsing and generation. The binary is usually
  autodiscovered, but you can use `OPENCV_PYTHON3_BIN` to specify the full path to the binary to
  invoke. E.g. "/usr/bin/python3" or "C:\Python37\python.exe"

The following variables must be set when building without `pkg_config` or `vcpkg`
(e.g. on Windows msvc target):

* `OPENCV_LINK_LIBS`
  Comma separated list of library names to link to. `.lib`, `.so` or `.dylib` extension is optional.
  E.g. "opencv_world411".
  
* `OPENCV_LINK_PATHS`
  Comma separated list of paths to search for libraries to link. E.g. "C:\tools\opencv\build\x64\vc14\lib".

* `OPENCV_INCLUDE_PATHS`
  Comma separated list of paths to search for system include files during compilation. E.g.
  "C:\tools\opencv\build\include". One of the directories specified therein must contain
  "opencv2/core/version.hpp" file, it's used to detect the version of the headers.

You can also set them on other platforms, then `pkg_config` or `vcpkg` usage will be disabled and the set
values will be used.

### Compiling OpenCV

See the [upstream
guides](https://docs.opencv.org/master/df/d65/tutorial_table_of_content_introduction.html)
for compiling OpenCV for your platform. Make sure to compile from the
correct release tag! We recommend including opencv_contrib and
configuring your build with the same flags the travis build uses:

[install-bionic.sh](https://github.com/twistedfall/opencv-rust/blob/master/ci/install-bionic.sh)

## Contrib modules

The following modules require [`opencv_contrib`](https://github.com/opencv/opencv_contrib/)
installed:
 * aruco
 * bgsegm
 * bioinspired
 * ccalib
 * cvv
 * dpm
 * freetype
 * fuzzy
 * hdf
 * img_hash
 * line_descriptor
 * phase_unwrapping
 * plot
 * sfm
 * shape
 * structured_light
 * superres
 * videostab
 * viz
 * xfeatures2d
 * xobjdetect
 * xphoto

## OpenCV 2 support

If you can't use OpenCV 3.x or higher, the (no longer maintained) `0.2.4`
version of this crate is known to work with OpenCV `2.4.7.13` (and probably other 2.4 versions).

## The binding strategy

This crate works following the model of python and java's OpenCV
wrappers - it parses the OpenCV C++ headers, generates a C interface
to the C++ api, and wraps the C interface in Rust.

All the major modules in the C++ API are merged together in a huge
`cv::` namespace. We instead made one rust module for each major
OpenCV module. So, for example, C++ `cv::Mat` is `opencv::core::Mat`
in this crate.

The methods and field names have been snake_cased. Methods arguments with
default value lose these default values, but they are reported in the
API documentation.

Overloaded methods have been — manually — given different names.

All methods return a Result to hack around C++ exception handling.

Most of the API is covered, but for various reasons several modules
are not yet implemented. If a missing module is near and dear to you,
file an issue (or better, open a pull request!)

## Contributor's Guide

The crate itself, as imported by users, consists of generated rust
code in [src](src) committed to the repo. This way, users don't have
to handle the code generation overhead in their builds. When
developing this crate, you can test changes to the binding generation
using `cargo build -vv --features buildtime-bindgen`. When changing
the codegen, be sure to push changes to the generated code!

`hdr_parser.py` comes from OpenCV python/java generator. We've tried
not to mess too much with this file, but had to make a few changes.

`gen_rust.py` is initially a copy of gen_java, also from the OpenCV
generators, but it has drifted significantly from the original.

The license for the original work is [MIT](https://opensource.org/licenses/MIT).

Special thanks to [ttacon](https://github.com/ttacon) for yielding the crate name.
