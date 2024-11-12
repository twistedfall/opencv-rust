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

   That often means that Windows can't find the OpenCV or Clang library dll. Be sure to set up `PATH` environment
   variable correctly or copy the dll next to the binary you're trying to run (e.g. "C:\tools\opencv\build\x64\vc16\bin" and
   "C:\Program Files\LLVM\bin"). Check [that](https://github.com/twistedfall/opencv-rust/issues/118#issuecomment-619608278) guide
   too.

5. On Windows with VCPKG you're getting a lot of linking errors in multiple files like in
   [this issue](https://github.com/twistedfall/opencv-rust/issues/340).

   Make sure to add missing linked libs to `OPENCV_LINK_LIBS` environment variable prepended by `+`, e.g.:
   ```
   OPENCV_LINK_LIBS="+user32,gdi32,comdlg32"
   ```

   Alternatively, switch to the dynamic linking mode by having environment variable `VCPKGRS_DYNAMIC` set to "1".

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

8. You're getting the ```a `libclang` shared library is not loaded on this thread``` error during crate build.

   Enable the `clang-runtime` feature. The reason for the issue is that some `clang-sys` crate can either link to the
   corresponding dynamic library statically or load it at runtime based on whether its feature `runtime` is enabled.
   If enabled this crate feature applies to all crates that depend on `clang-sys` even if they didn't explicitly enable that
   feature themselves (at least with Rust `edition` before 2021 and Cargo `resolver` before 2).

9. You're getting `'limits' file not found` error during crate build.

   This error is caused by the missing/invalid installation of C++ standard library (e.g. libstdc++ for GCC). To fix this make
   sure that the toolchain you're using has the corresponding C++ standard library. The toolchain is used through `libclang`, so
   to get useful diagnostic info run:
   ```shell
   clang -E -x c++ - -v
   ```
   Look for `Selected GCC installation` and `#include <...> search starts here` to get the sense of what system toolchain is used
   by clang. Refer to this [issue](https://github.com/twistedfall/opencv-rust/issues/322) for more fixes and workarounds.

10. Using a language server IDE on macOS you're getting `dyld: Library not loaded` error from `rust-analyzer`

    Check this [issue](https://github.com/twistedfall/opencv-rust/issues/592) for some ready-made configurations
    for VSCode and Zed.

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
