/// Conditional compilation macro based on whether the OPENCV_MODULE OpenCV module is enabled
///
/// This macro is intended for usage in external crates.
///
/// # Example
/// ```
/// let mut imgproc_enabled = false;
/// opencv::opencv_has_module_imgproc! {
///     let mut input = Mat::default();
///     let rect = opencv::core::Rect::default();
///     opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///     imgproc_enabled = true;
/// }
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
#[cfg(ocvrs_has_module_OPENCV_MODULE)]
#[macro_export]
macro_rules! opencv_has_module_OPENCV_MODULE {
	($($tt:tt)*) => { $($tt)* }
}

/// Conditional compilation macro based on whether the OPENCV_MODULE OpenCV module is enabled
///
/// This macro is intended for usage in external crates.
///
/// # Example
/// ```
/// let mut imgproc_enabled = false;
/// opencv::opencv_has_module_imgproc! {
///     let mut input = Mat::default();
///     let rect = opencv::core::Rect::default();
///     opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///     imgproc_enabled = true;
/// }
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
#[cfg(not(ocvrs_has_module_OPENCV_MODULE))]
#[macro_export]
macro_rules! opencv_has_module_OPENCV_MODULE {
	($($tt:tt)*) => {};
}
