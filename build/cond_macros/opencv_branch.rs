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
/// opencv::opencv_branch_34! {
///     let mut cam = opencv::videoio::VideoCapture::new_default(0)?;
/// }
/// opencv::not_opencv_branch_34! {
///     let mut cam = opencv::videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
/// }
/// ```
#[cfg(ocvrs_opencv_branch_OPENCV_BRANCH)]
#[macro_export]
macro_rules! opencv_branch_OPENCV_BRANCH {
	($($tt:tt)*) => { $($tt)* }
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
///
/// Alternative function call:
/// ```
/// opencv::opencv_branch_34! {
///     let mut cam = opencv::videoio::VideoCapture::new_default(0)?;
/// }
/// opencv::not_opencv_branch_34! {
///     let mut cam = opencv::videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
/// }
/// ```
#[cfg(not(ocvrs_opencv_branch_OPENCV_BRANCH))]
#[macro_export]
macro_rules! opencv_branch_OPENCV_BRANCH {
	($($tt:tt)*) => {};
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
///
/// Alternative function call:
/// ```
/// opencv::opencv_branch_34! {
///     let mut cam = opencv::videoio::VideoCapture::new_default(0)?;
/// }
/// opencv::not_opencv_branch_34! {
///     let mut cam = opencv::videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
/// }
/// ```
#[cfg(not(ocvrs_opencv_branch_OPENCV_BRANCH))]
#[macro_export]
macro_rules! not_opencv_branch_OPENCV_BRANCH {
	($($tt:tt)*) => { $($tt)* }
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
///
/// Alternative function call:
/// ```
/// opencv::opencv_branch_34! {
///     let mut cam = opencv::videoio::VideoCapture::new_default(0)?;
/// }
/// opencv::not_opencv_branch_34! {
///     let mut cam = opencv::videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
/// }
/// ```
#[cfg(ocvrs_opencv_branch_OPENCV_BRANCH)]
#[macro_export]
macro_rules! not_opencv_branch_OPENCV_BRANCH {
	($($tt:tt)*) => {};
}
