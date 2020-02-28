* 0.30.1
  * Dummy version to fix docs.rs build

* 0.30
  * Total rewrite of the binding generator in Rust. It no longer relies on Python, but instead uses libclang
    to parse the C++ code. Please be sure to install clang/llvm in your system if you're going to to regenerate
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

  * Traits are now used way more through the API, importing prelude is a must (```use opencv::prelude::*;``).

  * There are also separate preludes per module should you want to use that (```use opencv::imgproc::*;``).

  * All enums are now generated.

  * Full property access, you can now read and write all public properties of the OpenCV classes through
    getters and setters.

  * You can now access properties for `PtrOf...` objects directly without the need to go through `get()` and
    `get_mut()` (those methods are now dropped).

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
