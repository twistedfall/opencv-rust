#[cfg(ocvrs_opencv_branch_34)]
#[doc(hidden)]
#[macro_export]
macro_rules! opencv_branch_34 {
	($($tt:tt)*) => { $($tt)* }
}

/// Conditional compilation macro based on OpenCV branch version for usage in external crates.
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
#[cfg(not(ocvrs_opencv_branch_34))]
#[macro_export]
macro_rules! opencv_branch_34 {
	($($tt:tt)*) => {};
}

/// Conditional compilation macro based on OpenCV branch version for usage in external crates.
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
#[cfg(not(ocvrs_opencv_branch_34))]
#[macro_export]
macro_rules! not_opencv_branch_34 {
	($($tt:tt)*) => { $($tt)* }
}

#[cfg(ocvrs_opencv_branch_34)]
#[doc(hidden)]
#[macro_export]
macro_rules! not_opencv_branch_34 {
	($($tt:tt)*) => {};
}

/// Conditional compilation macro based on OpenCV branch version for usage in external crates.
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
#[cfg(ocvrs_opencv_branch_4)]
#[macro_export]
macro_rules! opencv_branch_4 {
	($($tt:tt)*) => { $($tt)* }
}

#[cfg(not(ocvrs_opencv_branch_4))]
#[doc(hidden)]
#[macro_export]
macro_rules! opencv_branch_4 {
	($($tt:tt)*) => {};
}

#[cfg(not(ocvrs_opencv_branch_4))]
#[doc(hidden)]
#[macro_export]
macro_rules! not_opencv_branch_4 {
	($($tt:tt)*) => { $($tt)* }
}

/// Conditional compilation macro based on OpenCV branch version for usage in external crates.
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
#[cfg(ocvrs_opencv_branch_4)]
#[macro_export]
macro_rules! not_opencv_branch_4 {
	($($tt:tt)*) => {};
}

/// Conditional compilation macro based on OpenCV branch version for usage in external crates.
/// # Examples
///
/// Alternative import:
/// ```
/// opencv::opencv_branch_4! {
///     use opencv::imgproc::LINE_8;
/// }
/// opencv::not_opencv_branch_4! {
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
#[cfg(ocvrs_opencv_branch_5)]
#[macro_export]
macro_rules! opencv_branch_5 {
	($($tt:tt)*) => { $($tt)* }
}

#[cfg(not(ocvrs_opencv_branch_5))]
#[doc(hidden)]
#[macro_export]
macro_rules! opencv_branch_5 {
	($($tt:tt)*) => {};
}

#[cfg(not(ocvrs_opencv_branch_5))]
#[doc(hidden)]
#[macro_export]
macro_rules! not_opencv_branch_5 {
	($($tt:tt)*) => { $($tt)* }
}

/// Conditional compilation macro based on OpenCV branch version for usage in external crates.
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
#[cfg(ocvrs_opencv_branch_5)]
#[macro_export]
macro_rules! not_opencv_branch_5 {
	($($tt:tt)*) => {};
}
