* 0.48.0
  * Fix binding-generator build in cross-compilation environment (kudos to tylerhawkes)
  * Using `dnn_superres` now requires `contrib` feature enabled, it was incorrectly marked as core module

* 0.47.0
  * Fix building with OpenCV 4.5.1 and 3.4.15
  * Due to the code adjustments, some `VectorOf...` types are no longer generated (e.g. `VectorOfPoint3i`)
  * The documentation for `tracking` module is now a copy of `imgproc`, which is a bug

* 0.46.3
  * Specify minimum required vcpkg version

* 0.46.2
  * Bring back deprecated cmake find-package discovery as a fallback, it's still useful in [some cases](https://github.com/twistedfall/opencv-rust/issues/177)

* 0.46.1
  * Improve cmake package detection (and remove deprecated find-package)
  * Re-enable parallel building with `clang-runtime` feature enabled
  * Add support for appending user values to detected OpenCV build configurations like include paths or link paths
  * Fix generation with OpenCV 4.5.0
  * Minimal required rust version is now 1.45.0 due to usage of `str::strip_prefix`

* 0.46.0
  * Fix linking in some configurations (https://github.com/twistedfall/opencv-rust/issues/164)
  * Fix argument name clashing in external functions

* 0.45.1
  * Internal improvements in const handling
  * Fix warnings in beta 
  * Misc CI improvements

* 0.45.0
  * Add implementation of `From<Vec<T>>` for `Vector<T>` + misc vector improvements
  * Enable OpenGL functions and classes from the `core` module

* 0.44.2
  * Fix building with Apple clang
  * Show libclang diagnostics during binding generation

* 0.44.1
  * Fix docs.rs build

* 0.44.0
  * Add support for ovis module
  * Add implementation of Ptr::new for non-abstract types
  * Rework string handling, that fixed core::convert_type_str() function which was handling memory very unsafely before
  * Internal refactor of cpp const handling, that changed the internal function identifiers

* 0.43.3
  * Test with the newly released OpenCV 3.4.11 and 4.4.0
  * Internal cleanups and CI fixes

* 0.43.2
  * Fix build on RPi (fixes #157)

* 0.43.1
  * Fix docs.rs build

* 0.43.0
  * Add `Clone` implementation (that panics on error) for `Mat`, `SparseMat`, `UMat`, `GpuMat` and `HostMem`.
  * Add `Vector::as_mut_slice()` for copy types.
  * Add `Vector::clone()` for types with `Clone` implementation (based on work of carbotaniuman).

* 0.42.0
  * Enable CUDA support, check `examples/cuda.rs` for a bootstrap.
  * `buildtime-bindgen` is now enabled by default, compilation without it enabled rarely worked in reality.
    You can still disable it by disabling default features.
  * `Vector` now implements `Into<Vec>`, `Debug`, `AsRef`, `Borrow`, `Default`, `Extend` and `FromIterator`,
    `Vector::to_slice` was renamed to `Vector::as_slice` (kudos to jerry73204 and carbotaniuman).
  * Improve tests and examples.
  * Internal reorganization for better logic separation.

* 0.41.1
  * Revert the parallelization of the build when `clang-runtime` is enabled, it led to #145

* 0.41.0
  * Enable parallel build even if `clang-runtime` is enabled (it uses helper generator binary in multiple threads)
  * Enable most of the remaining contrib modules, namely: quality, saliency, stereo, ximgproc, tracking, rgbd,
    hfs and optflow.
  * Move all the remaining methods from `VectorTrait` to `Vector` struct and remove `VectorTrait`.
  * Improve `Mat_` usability.
  * Enable class constant generations (these were previously methods).
  * Fix the issue where functions accepting strings will not reject strings with null characters in them, but
    silently truncate them.

* 0.40.1
  * Fix parsing of cmake output during package discovery (fixes #136)
  * Prefer vcpkg over cmake when building on Windows (fixes #137)

* 0.40.0
  * Optimized dependencies, the crate has now 19 (including transitional) dependencies less.
  * Fixed pre-generated bindings for version 3.4 that got left out in the previous release.

* 0.39.0
  * Iterators for `Vector` now correctly return remaining length in `size_hint` and `len` methods.
  * Internal improvements and cleanups

* 0.38.0
  * Fix crosscompilation when targeting Windows platform.
  * Fix generation of bigger binaries

* 0.37.0
  * Add `clang-runtime` feature for compatiblity with dependencies that require `clang` or `clang-sys` that
    enable its `runtime` feature unconditionally (like `bindgen`).
  * Add `OPENCV_CLANG_STDLIB_PATH` environment variable to allow working around header detection of libclang
    when it fails (should fix #125).
  * Stop using synchronization primitives for callback handling.
  * Improve cmake based library discovery.

* 0.36.0
  * Large internal rework of `Matx`, `PtrOf…` and `VectorOf…` structs. They have been made generic and their
    code generation have been mostly moved to Rust macros. This change should have no user facing consequences,
    type aliases with the old names are kept for backwards compatibility and code brevity.
  * Methods for accessing raw pointers for boxed classes have been moved into a common `Boxed` trait.
    Additionally, `from_raw_ptr()` is now `from_raw()`, `as_raw()` is now returning `*const c_void` (as
    opposed to `mut`) and new methods have been added: `as_raw_mut()`, `into_raw()`. This applies to all
    structs that are allocated by C++ side (e.g. Mat), `Ptr<T>` and `Vector<T>`.
  * `TermCriteria` is now a simple class so you can write and read its properties directly without using
    accessor methods, APIs involving this class have also been updated.
  * C++ operators * (multiplication, dereferencing), / (division), + (addition) and - (subtraction) are now
    also exposed to Rust under function names `mul_*`/`try_deref`, `div_*`, `add_*` and `sub_*`.
  

* 0.35.0
  * Add beta-level support for doing OpenCV library discovery using cmake, which is now a recommended method
    by the upstream.
  * Add ability to override `OPENCV_LINK_LIBS`, `OPENCV_LINK_PATHS` and `OPENCV_INCLUDE_PATHS` in case of
    library auto-discovery. This doesn't work for vcpkg yet.
  * Add ability to introduce to specify package names separately for different auto-discovery systems using
    `OPENCV_PKGCONFIG_NAME`, `OPENCV_CMAKE_NAME` and `OPENCV_VCPKG_NAME`.

* 0.34.0
  * Bump bindings to OpenCV version 4.3.0 and 3.4.10
  * Mark `Mat::new_*_with_data` functions unsafe because they allow passing arbitrary data pointer
  * Sort out internal generation of string return type wrappers, should help on some platforms in avoiding
    `buildtime-bindgen`, see https://github.com/twistedfall/opencv-rust/issues/110
  * Fix examples

* 0.33.1
  * Internal rework of `VectorOf…` generation logic, fix https://github.com/twistedfall/opencv-rust/issues/109

* 0.33.0
  * Basic support for building against OpenCV.framework on macOS, for details see
    [ci files](https://github.com/twistedfall/opencv-rust/blob/v0.33.0/ci/script.sh#L34)
  * Fix internals of `PtrOff32`
  * Big internal refactoring getting rid of most instances of type casting and `void*`

* 0.32.0
  * Improve Matx: better api ergonomics, add PartialEq, add ToInput/OutputArray implementations, add tests
  * 'u_mat' the method names is now 'umat' (underscore removed)

* 0.31.1
  * Fix passing bigger simple classes as arguments

* 0.31.0
  * Add from/to_vec method to `Point_` and `Point3_`
  * Add basic implementations of `Mat_`, `Matx` and `Affine3` generic types, and OpenCV methods that use them
  * Rename `Mat::*_mut_unchecked` methods to `*_unchecked_mut` for consistency with std library

* 0.30.1
  * Dummy version to fix docs.rs build

* 0.30
  * Total rewrite of the binding generator in Rust. It no longer relies on Python, but instead uses libclang
    to parse the C++ code. Please be sure to install clang/llvm in your system if you're going to regenerate
    bindings (mostly macOS and Windows users). You can expect all of the previous functionality to be there
    (sans name changes) and a lot of new functions and classes. Binding generation is now more precise and
    quick.

  * While most of the crate interface remain the same there are some quite changes in the API:
    * Some functions have changed name to the one hinted in the OpenCV headers, e.g. `core::sum()` is now
      `core::sum_elems()`.
    * Some functions got renamed for more consistency (e.g. `VideoCapture` constructors) or to avoid the name
      clash with the newly generated functions (like `MatConstIterator::get` → `current`). Please use the
      documentation to discover the new name.
    * A bunch of functions got renamed from their _1 automatic names.
    * Names that coincide with Rust keywords now get underscore at the end, not the beginning, e.g. `_ref`
      becomes `ref_`. Moreover `type_` is now `typ`.
    * Vector types now receive their Rust type suffix, so `VectorOfuchar` gets renamed to `VectorOfu8`.

  * `FileStorage`/`FileNode` API is now usable.

  * Some methods (like `Mat::new_nd_with_default`) now accept slices instead of `Vector`s. The length of the
    Rust slice is passed down to the OpenCV.

  * Traits are now used way more through the API, importing prelude is a must (```use opencv::prelude::*;```).
    There are also separate preludes per module should you want to use that (```use opencv::imgproc::*;```).

  * All enums are now generated.

  * Full property access, you can now read and write all public properties of the OpenCV classes through
    getters and setters.

  * You can now access properties for `PtrOf...` objects directly without the need to go through `get()` and
    `get_mut()` (those methods are now dropped, they were unsound).

  * All property setters and getters are now infallible so they don't return `Result`, but a value directly.

  * `_OutputArray` and `_InputOutputArray` have received a second set of constructors `from_*_mut`, so if you
    use those manually you most probably want to use `*_mut` version instead of the const one.

  * New modules: face, surface_matching, text

  * Calling `Mat::data_typed()` and `Mat::data_typed_mut()` will now do `is_continuous()` check and return an
    error if it's false.

* 0.29.2
  * add ability to get reference to the inner value of non-trait `PtrOf*` types (kudos to jerry73204)

* 0.29.1
  * fix generation of setters for properties (kudos to jerry73204)

* 0.29
  * default features now target OpenCV 4.2.0 (instead of OpenCV 3.4.x). Feature `opencv-42` was renamed to
    `opencv-4`. Bindings were bumped to OpenCV 4.2.0 and 3.4.9.

  * core::MAKETYPE was renamed to CV_MAKETYPE, core::MAT_DEPTH was renamed to CV_MAT_DEPTH

* 0.28
  * dnn::BackendNode constructor as well as stitching::createStitcher and stitching::createStitcherScans are no
    longer generated for the OpenCV version where they are not available, fixing the linking issues

* 0.27.0
  * add support for `vcpkg` when building for windows, it's being used by default in the absence of
    environment variables

  * with `buildtime-bidngen` feature enabled the crate no longer uses bundled source files for code
    generation by default, but the ones installed with OpenCV library detected by `pkg_config`, `vcpkg` or
    environment; set `OPENCV_HEADER_DIR` environment variable to override this behavior  

* 0.26.6
  * ...
