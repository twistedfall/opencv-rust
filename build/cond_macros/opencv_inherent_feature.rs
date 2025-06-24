/// Conditional compilation macro based on whether OpenCV was built with the OPENCV_INHERENT_FEATURE feature
///
/// This macro is intended for usage in external crates.
///
/// # Example
/// ```
/// let mut cuda_available = false;
/// opencv::opencv_has_inherent_feature_cuda! {
///     cuda_available = true;
/// }
/// if !cuda_available {
///     panic!("CUDA is required");
/// }
/// ```
#[cfg(ocvrs_has_inherent_feature_OPENCV_INHERENT_FEATURE)]
#[macro_export]
macro_rules! opencv_has_inherent_feature_OPENCV_INHERENT_FEATURE {
	($($tt:tt)*) => { $($tt)* }
}

/// Conditional compilation macro based on whether OpenCV was built with the OPENCV_INHERENT_FEATURE feature
///
/// This macro is intended for usage in external crates.
///
/// # Example
/// ```
/// let mut cuda_available = false;
/// opencv::opencv_has_inherent_feature_cuda! {
///     cuda_available = true;
/// }
/// if !cuda_available {
///     panic!("CUDA is required");
/// }
/// ```
#[cfg(not(ocvrs_has_inherent_feature_OPENCV_INHERENT_FEATURE))]
#[macro_export]
macro_rules! opencv_has_inherent_feature_OPENCV_INHERENT_FEATURE {
	($($tt:tt)*) => {};
}
