/// Conditional compilation macro based on OpenCV branch version for usage in external crates.
///
/// # Examples
///
/// Alternative import:
/// ```
/// opencv::not_opencv_branch_34! {
///     use opencv::imgproc::LINE_8;
/// }
/// opencv::opencv_branch_34! {
///     use opencv::core::LINE_8;
/// }
/// ```
///
/// Alternative function call:
/// ```
/// let mut cam = opencv::opencv_branch_34! {
///     {
///         opencv::videoio::VideoCapture::new_default(0)?
///     } else {
///         opencv::videoio::VideoCapture::new(0, videoio::CAP_ANY)?
///     }
/// };
/// ```
#[cfg({{ cfg_condition_prefix }}ocvrs_opencv_branch_{{ feature }}{{ cfg_condition_suffix }})]
#[macro_export]
macro_rules! opencv_branch_{{ feature }} {
	($bl_pos:block else $bl_neg:block) => { {{ macro_body_block }} };
	($($tt:tt)*) => { {{ macro_body_tt }} }
}

/// Conditional compilation macro based on OpenCV branch version for usage in external crates.
///
/// # Examples
///
/// Alternative import:
/// ```
/// opencv::not_opencv_branch_34! {
///     use opencv::imgproc::LINE_8;
/// }
/// opencv::opencv_branch_34! {
///     use opencv::core::LINE_8;
/// }
/// ```
#[cfg({{ cfg_condition_prefix }}ocvrs_opencv_branch_{{ feature }}{{ cfg_condition_suffix }})]
#[macro_export]
macro_rules! not_opencv_branch_{{ feature }} {
	($($tt:tt)*) => { {{ macro_body_not_tt }} }
}


