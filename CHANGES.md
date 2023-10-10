* 0.85.1
  * Add workaround when building with rustc version before 1.66.

* 0.85.0
  * Generate companion functions `*_def()` that allow skipping the default arguments to improve API usability.
  * Remove bindings to `SparseMatIterator--` and change `OutputArray::from_gpu_mat_vec()` to take mutable argument (instead of
    `OutputArray::from_gpu_mat_vec_mut()` which is now removed). Those functions were often missing in the OpenCV shared libraries
    causing linking errors.
  * Update automatic case conversion algorithm to increase performance and improve correctness. Some function names are now slightly
    different. Especially those containing `2d` and `3d`.
  * Improve documentation generation.
 
* 0.84.5
  * Improve parallel build performance (more noticeable on lower thread count).
  * Take `VCPKGRS_TRIPLET` environment var into account for crate rebuild.
  * Add a way to select MSVC CRT library with `OPENCV_MSVC_CRT` environment variable.
  * Make sure to always generate bindings for `Ptr<float>` (fixes https://github.com/twistedfall/opencv-rust/issues/487).

* 0.84.4
  * Workaround for a build hang due to `cc` dependency

* 0.84.3
  * Update docs

* 0.84.2
  * Fix build targeting Android even more (definitely fixes https://github.com/twistedfall/opencv-rust/issues/477).
  * Fix double-free crash and linker errors when using CUDA (fixes https://github.com/twistedfall/opencv-rust/issues/478 and
    https://github.com/twistedfall/opencv-rust/issues/470).
  * Add `CommandLineParser::get()` function.
  * Improve examples.

* 0.84.1
  * Fix build targeting Android (fixes https://github.com/twistedfall/opencv-rust/issues/477).

* 0.84.0
  * Detect more cases where a Rust `char` can be used.
  * Remove deprecated `DataType::depth()`, `DataType::channels()` and `DataType::typ()` functions.
  * Speed up binding generation by reducing number of passes (3 → 2) and removing the usage of `ocvrs_ephemeral.hpp`.
  * Generate more casts (to base and to decendants) for `Ptr`s and classes.

* 0.83.0
  * Add support for OpenCV 4.8.
  * Tune naming for classes in `rapid` and CUDA modules.
  * Add a `Debug` implementation for all classes and smart pointers.

* 0.82.1
  * Fix generation issue with specific module setup involving `dnn` module.

* 0.82.0
  * Change the handling of abstract C++ classes, they are no longer exposed as `dyn Class` but a struct is generated for
    them making them easier to use from Rust. One notable change is calling static methods on those classes no longer
    requires UCS. So `<dyn ORB>::default()` becomes just `ORB::default()`. You might also need to adjust your imports
    because while traits are imported as part of the prelude, the structs need to be imported explicitly.
  * Functions that take `Input/OutputArray` now take them by `&impl` instead of `&dyn` making those calls faster because
    they don't use dynamic dispatch anymore.

* 0.81.5
  * Bring back the `clang-runtime` feature to improve cooperation with other crates.

* 0.81.4
  * Change the way the binding generator is run decreasing the complexity and improving the compatibility with vendored builds.

* 0.81.3
  * Don't require `runtime` feature of `clang`.

* 0.81.2
  * Fix building when crosscompiling with target-specific RUSTFLAGS.

* 0.81.1
  * Fix building on macOS (fixes https://github.com/twistedfall/opencv-rust/issues/460).

* 0.81.0
  * Fix compilation when using clang-6.
  * Add hough_lines example.
  * Add `Scalar::is_real()`.
  * Apply proper naming to some methods from RNG.

* 0.80.0
  * Generate default constructors for some classes that were missing any form of construction.

* 0.79.0
  * `DataType` can now be implemented by downstream types that will allow storage of some custom types inside `Mat`.
    Previously this trait was sealed, now it's an unsafe trait because types must guarantee the memory layout.0
  * Calling `.iter()` on an empty `Mat` no longer returns an error, but an empty iterator.
  * Some functions like `Point_::new()` or `Size_::new()` are now `const`.
  * Minor documentation updates.

* 0.78.2
  * Fix compilation with clang 16

* 0.78.1
  * Automatically implement `Clone` for those classes that are constructor-copiable in C++, e.g. `KeyPoint`.
  * Fixes for crosscompiling (kudos to icedrocket).

* 0.78.0
  * Don't strip "lib" prefix from linked libraries when building with MSVC (kudos to icedrocket)
  * Fix newline handling on Windows (kudos to grindvoll)
  * Fix for the build failure against the upcoming OpenCV version.

* 0.77.0
  * Rename `typ()`, `channels()`, `depth()` to `opencv_type()`, `opencv_channels()`, `opencv_depth()`. The old functions are
    scheduled for removal mid-2023.
  * Generate `DrawLinesMatchesFlags` enum correctly.
  * Make sure to generate `Vector<Point2d>` for `calib3d` module (fixes https://github.com/twistedfall/opencv-rust/issues/422).

* 0.76.4
  * Fix autocomplete and documentation for rust-analyzer based IDEs.

* 0.76.3
  * Introduce `Mat::from_slice_rows_cols()` function to simplify creation of 2d matrices from byte buffers.
  * Improve documentation.

* 0.76.2
  * Fix incorrect display of macros in documentation, some internal macros were showing up, but some usable were not.

* 0.76.1
  * Fix build failure.

* 0.76.0
  * Change the way the generated bindings are stored. They are no longer generated under `src/`, but stored in the output directory
    and included dynamically. Previously it didn't work very well with IDEs which resulted in missing autocomplete and documentation.
    This looks to be no longer the case at least in `rust-analyzer` and `intellij-rust`.
  * Bump crate edition to 2021 (from 2018) and require at least Rust 1.59.0 (the MSRV check is now included in CI).
  * Start phasing out OpenCV 3.2 support. This does not mean immediate breakage, but it's no longer going to be tested and problems
    in generation for that outdated and unsupported version will no longer be addressed.

* 0.75.0
  * Add support for OpenCV 4.7.0.
  * Add support for C++ function call operator: `operator ()`.

* 0.74.2
  * Adjust dependencies to ensure that `jobserver` is the appropriate version (fixes https://github.com/twistedfall/opencv-rust/issues/400).

* 0.74.1
  * Fix building when `-j1` cargo option is specified (fixes https://github.com/twistedfall/opencv-rust/issues/380).

* 0.74.0
  * Add support aruco_detector module present in the future opencv_contrib.

* 0.73.0
  * Add basic support for the Graph-API (gapi) module.
  * Generate bindings for `std::tuple` and `std::pair`.
  * Add support for the following operators: `!=`, `>`, `>=`, `<`, `<=`, `&`, `|`, `^`, `~`.
  * Correctly handle rvalue reference (`&&`) as move operation.
  * Due to multiple internal improvements some functions have slightly changed names.

* 0.72.0/0.72.1
  * Make `lower_bound` argument of `imgproc::emd()` optional.
  * Fix semantics of `VectorIterator::nth` to follow the documentation for `Iterator`.
  * Fix Android build failure ([#392](https://github.com/twistedfall/opencv-rust/issues/392))
  * Fix constant crate recompilation ([#390](https://github.com/twistedfall/opencv-rust/issues/390))

* 0.71.0
  * Multiple improvements to the `Vector` type:
    * `VectorRefIterator` is now cloneable.
    * Better performance and correctness for `Vector` iterators.
    * Implement `Vector::from_elem()`.
    * Make sure that `Vector<T: Send + Sync>` is also `Send + Sync`.
  * Fix building on platforms where `c_char` ≠ `u8`.

* 0.70.0
  * Internal improvements and reorganization

* 0.69.0
  * Fix building with clang-15.
  * Rename `distort_points` to `fisheye_distort_points` for consistency.

* 0.68.0
  * Make sure that `howToGetFeatures` argument of the `createStructuredEdgeDetection()` is nullable.
  * Add `OutputArray` and `InputOutputArray` implementations for `Mat_`.

* 0.67.0
  * Change `fourcc` method to accept `char`s instead of `i8`s.
  * Remove `gapi` feature as this module is not supported at the moment.

* 0.66.0
  * Generate `valid_pix_roi` argument of `get_optimal_new_camera_matrix` as optional argument (fixes [#349](https://github.com/twistedfall/opencv-rust/issues/349)).
  * `clang-runtime` feature has been dropped and the associated behavior is now the default. It's caused by the changed API of the
    `clang` crate.

* 0.65.0
  * Improve generation for OpenCV 4.6.0.

* 0.64.1
  * Improve function documentation generation (fixes [#266](https://github.com/twistedfall/opencv-rust/issues/266)).

* 0.64.0
  * Fix documentation for conditional compilation macros.
  * Make sure to generate `VectorOfPoint3f` and `VectorOfVectorOfPoint3f` for `calib3d` module.
  * Disable generation of `merge_slice` function because it's not usable from Rust.

* 0.63.2
  * Introduce macros that allow external crates to conditionally include code based on the OpenCV branch (3.2, 3.4 or 4).

* 0.63.1
  * Provide `From` implementations from tuples and similar C++ conversions for `Point`, `Size`, `Rect` and `Point3`.

* 0.63.0
  * Provide bindings for `++` and `--` operators (fixes [#319](https://github.com/twistedfall/opencv-rust/issues/319))
  * Drop type restrictions for `Rect_`, `Point_`, `Size_`, `Point3_`, `VecN` (and related structs) (fixes [#316](https://github.com/twistedfall/opencv-rust/issues/316))
  * Make sure that `VectorOfVectorOff64` is generated for calib3d module (fixes [#321](https://github.com/twistedfall/opencv-rust/issues/321))

* 0.62.0
  * Fixed a segfault when using functions that return small structures (like Point2f) with some C++ compiler combinations, e.g.
    when using repository OpenCV with the default compiler in Ubuntu 20.04.
  * Fix segfault and error handling while casting to descendant classes.
  * Test with OpenCV 4.5.5 and 3.4.17.

* 0.61.3
  * Implement `ToInput/OutputArray` for `Scalar` and all other often used combinations of `VecN`.
  * Fix building on macOS with OpenCV as framework.

* 0.61.2
  * Add support for parsing `CV_NODISCARD` attributes, they are converted to `#[must_use]`.

* 0.61.1
  * Make sure that `VectorOfVectorOfPoint` type is generated for the `imgproc` module.

* 0.61.0
  * The QR code decoding function in `objdetect` module are now returning `Vec<u8>` instead of `String` because the codes can
    contain binary raw binary data and are not always UTF-8.
  * You can now iterate over `Mat` using `for` loop. The iterator will yield elements of type `(Point, T)`, a position and an
    element.

* 0.60.0
  * The features for OpenCV module selections now have inter-dependencies so that you can't exclude a module that's needed for
    some other one.
  * Infallible functions returning references should be faster now due to the streamlined error handling.
  * More simple functions are marked as infallible (e.g. `Mat::total()`, `Mat::depth()`).
  * Functions that returned references to some internal data (e.g. `Mat::ptr()`, `Mat::data_mut()`) now return raw pointers, it
    makes more sense and allows checking for null pointer outside of call. The corresponding property setters also accept pointers.
  * Functions that accept raw pointers are now marked as unsafe.

* 0.59.0
  * You can now select which OpenCV modules you want to include in generation using the crate features, there is a feature for
    every OpenCV module except core. This supersedes the old system with environment variables `OPENCV_MODULE_WHITELIST` and
    `OPENCV_MODULE_BLACKLIST`, they are no longer supported (kudos to nsabovic).
  * You can now use the `rgb` crate types as elements of `Mat`, just enable the `rgb` feature (kudos to fzyzcjy).
  * You can now use `Vector::from_slice()` for a faster construction of `Vector` from a slice. This only works for simple types
    like `u8` or `Point`.

* 0.58.0
  * Infallible functions are no longer using exception handling internally so they should be faster.
  * `core::Vec` is renamed to `core::VecN` to avoid name conflict with std's `Vec`.
  * The `dnn::TextDetectionModel` struct is now generated correctly for newer OpenCV's.

* 0.57.0
  * Add support for mathematical operations on `Mat` (kudos to fzyzcjy).
  * `VecNT` (e.g. `Vec3b`) are now rewritten using const generics which makes them more flexible. That raises the minimum Rust
    version requirement to 1.51. Also be mindful of the name clash between `opencv::core::Vec` and `std::vec::Vec`.
  * Added mathematical operations on `Vec` that are available in the OpenCV library.
  * Added support for generating bindings for the `==` operator.

* 0.56.1
  * Structs that have infallible default constructors (e.g. `Mat`) now implement `Default` trait.
  * Make sure to generate `std::vector<cv::Vec[23][fd]>` bindings for `imgproc::HoughLines` function.

* 0.56.0
  * Mark most exported functions as `#[inline]` to facilitate inter-crate inlining. Those function calls are just wrappers around
    the corresponding FFI function.
  * Adjust the code to support OpenCV 4.5.4 better.
  * Allow passing `None` to `minMaxLoc` and `minMaxIdx` functions.

* 0.55.0
  * add `Mat::data_bytes[_mut]()` methods to access byte data inside `Mat` as slice, `Mat::data()` now returns
    raw pointer instead of reference.

* 0.54.0
  * highgui::create_trackbar now takes `Option<&mut i32>` for `value` to fix the deprecation warning (fixes [#261](https://github.com/twistedfall/opencv-rust/issues/261)).
  * Class traits are now split into const and mut variants. So what previously was `MatTrait` is now
    `MatTraitConst` with only const methods and `MatTrait` with only mut methods.

* 0.53.2
  * Fix building with cmake versions below 3.15.

* 0.53.1
  * Add support for casting classes to their bases. You can now call `.into()` on the descendant class to cast
    it to the parent.
  * Add support for newly released OpenCV 4.5.3 and 3.4.15.
  * Minor improvements in the `vcpkg` support.

* 0.53.0
  * This release removes some features and environment variables to make the crate easier to use:
    * `buildtime-bindgen` is removed. With all the possible OpenCV configurations out there it was practically
      impossible to build the crate without regenerating bindings to fit your exact installation.
    * `opencv-32`, `opencv-34` and `opencv-4` branch selection features are removed too. Version selection is
      now performed in build-time. If you were using those features in your `Cargo.toml` then just remove them
      (and `default-features = false` can probably go too).
    * Support for `OPENCV_CLANG_STDLIB_PATH` environment variable is removed. If you were using it, then you
      can add that directory as part of `OPENCV_INCLUDE_PATHS`. E.g.:
      ```
      OPENCV_INCLUDE_PATHS="<path_to_clang_stdlib_path>,+"
      ```
    * Support for `OPENCV_HEADER_DIR` is also removed. It was meant to be development-only environment variable,
      and it's no longer needed in the development process.
  * The crate is now smaller due to dropping of the bundled headers and bindings.
  * Fixes for building with older OpenCV versions (fixes [#227](https://github.com/twistedfall/opencv-rust/issues/227))

* 0.52.0
  * Add support for OpenCV 4.5.2 and 3.4.14
  * In those new OpenCV versions some methods are now marked NOEXCEPT which makes them return result directly,
    without the wrapping Result. Most notable of those new methods is `Mat::default`, which now returns a `Mat`
    directly. To keep the examples and tests running those methods are also manually marked as NOEXCEPT for previous
    versions of OpenCV.
  * The "contrib" feature flag is there no more, the crate now uses build-time module set detection, so just drop
    it from your `Cargo.toml` if you've been using it.

* 0.51.0
  * Make sure that casts to Ptr<Feature2D> are also generated (fixes [#218](https://github.com/twistedfall/opencv-rust/issues/218))
  * Port text detection example (requires OpenCV 4.5.1)
  * Adjust some function names

* 0.50.0
  * Improve smart pointer handling (`Ptr`), deeper analysis now generates more necessary `Ptr<T>` bindings and
    all `Ptr<Child>` types are now castable to `Ptr<Parent>` when it's required by the OpenCV API (fixes [#217](https://github.com/twistedfall/opencv-rust/issues/217))
  * Module-level documentation generation for `tracking` module is now fixed

* 0.49.1
  * Improved processing of environment variables

* 0.49.0
  * Fix conversion of slice arguments, allow nullable slices (fixes [#201](https://github.com/twistedfall/opencv-rust/issues/201))

* 0.48.0
  * Fix binding-generator build in cross-compilation environment (kudos to tylerhawkes)
  * Using `dnn_superres` now requires `contrib` feature enabled, it was incorrectly marked as core module

* 0.47.0
  * Fix building with OpenCV 4.5.1 and 3.4.13
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
