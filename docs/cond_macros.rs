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
#[cfg(ocvrs_opencv_branch_34)]
#[macro_export]
macro_rules! opencv_branch_34 {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
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
#[cfg(ocvrs_opencv_branch_34)]
#[macro_export]
macro_rules! not_opencv_branch_34 {
	($($tt:tt)*) => {  }
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
/// let mut cam = opencv::opencv_branch_34! {
///     {
///         opencv::videoio::VideoCapture::new_default(0)?
///     } else {
///         opencv::videoio::VideoCapture::new(0, videoio::CAP_ANY)?
///     }
/// };
/// ```
#[cfg(not(ocvrs_opencv_branch_34))]
#[macro_export]
macro_rules! opencv_branch_34 {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  }
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
#[cfg(not(ocvrs_opencv_branch_34))]
#[macro_export]
macro_rules! not_opencv_branch_34 {
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
/// let mut cam = opencv::opencv_branch_34! {
///     {
///         opencv::videoio::VideoCapture::new_default(0)?
///     } else {
///         opencv::videoio::VideoCapture::new(0, videoio::CAP_ANY)?
///     }
/// };
/// ```
#[cfg(ocvrs_opencv_branch_4)]
#[macro_export]
macro_rules! opencv_branch_4 {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
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
#[cfg(ocvrs_opencv_branch_4)]
#[macro_export]
macro_rules! not_opencv_branch_4 {
	($($tt:tt)*) => {  }
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
/// let mut cam = opencv::opencv_branch_34! {
///     {
///         opencv::videoio::VideoCapture::new_default(0)?
///     } else {
///         opencv::videoio::VideoCapture::new(0, videoio::CAP_ANY)?
///     }
/// };
/// ```
#[cfg(not(ocvrs_opencv_branch_4))]
#[macro_export]
macro_rules! opencv_branch_4 {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  }
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
#[cfg(not(ocvrs_opencv_branch_4))]
#[macro_export]
macro_rules! not_opencv_branch_4 {
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
/// let mut cam = opencv::opencv_branch_34! {
///     {
///         opencv::videoio::VideoCapture::new_default(0)?
///     } else {
///         opencv::videoio::VideoCapture::new(0, videoio::CAP_ANY)?
///     }
/// };
/// ```
#[cfg(ocvrs_opencv_branch_5)]
#[macro_export]
macro_rules! opencv_branch_5 {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
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
#[cfg(ocvrs_opencv_branch_5)]
#[macro_export]
macro_rules! not_opencv_branch_5 {
	($($tt:tt)*) => {  }
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
/// let mut cam = opencv::opencv_branch_34! {
///     {
///         opencv::videoio::VideoCapture::new_default(0)?
///     } else {
///         opencv::videoio::VideoCapture::new(0, videoio::CAP_ANY)?
///     }
/// };
/// ```
#[cfg(not(ocvrs_opencv_branch_5))]
#[macro_export]
macro_rules! opencv_branch_5 {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  }
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
#[cfg(not(ocvrs_opencv_branch_5))]
#[macro_export]
macro_rules! not_opencv_branch_5 {
	($($tt:tt)*) => { $($tt)* }
}

/// Conditional compilation macro based on whether the 3d OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_3d)]
#[macro_export]
macro_rules! opencv_has_module_3d {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the 3d OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_3d))]
#[macro_export]
macro_rules! opencv_has_module_3d {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the alphamat OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_alphamat)]
#[macro_export]
macro_rules! opencv_has_module_alphamat {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the alphamat OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_alphamat))]
#[macro_export]
macro_rules! opencv_has_module_alphamat {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the aruco OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_aruco)]
#[macro_export]
macro_rules! opencv_has_module_aruco {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the aruco OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_aruco))]
#[macro_export]
macro_rules! opencv_has_module_aruco {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the aruco_detector OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_aruco_detector)]
#[macro_export]
macro_rules! opencv_has_module_aruco_detector {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the aruco_detector OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_aruco_detector))]
#[macro_export]
macro_rules! opencv_has_module_aruco_detector {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the barcode OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_barcode)]
#[macro_export]
macro_rules! opencv_has_module_barcode {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the barcode OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_barcode))]
#[macro_export]
macro_rules! opencv_has_module_barcode {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the bgsegm OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_bgsegm)]
#[macro_export]
macro_rules! opencv_has_module_bgsegm {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the bgsegm OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_bgsegm))]
#[macro_export]
macro_rules! opencv_has_module_bgsegm {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the bioinspired OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_bioinspired)]
#[macro_export]
macro_rules! opencv_has_module_bioinspired {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the bioinspired OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_bioinspired))]
#[macro_export]
macro_rules! opencv_has_module_bioinspired {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the calib OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_calib)]
#[macro_export]
macro_rules! opencv_has_module_calib {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the calib OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_calib))]
#[macro_export]
macro_rules! opencv_has_module_calib {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the calib3d OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_calib3d)]
#[macro_export]
macro_rules! opencv_has_module_calib3d {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the calib3d OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_calib3d))]
#[macro_export]
macro_rules! opencv_has_module_calib3d {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the ccalib OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_ccalib)]
#[macro_export]
macro_rules! opencv_has_module_ccalib {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the ccalib OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_ccalib))]
#[macro_export]
macro_rules! opencv_has_module_ccalib {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the core OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_core)]
#[macro_export]
macro_rules! opencv_has_module_core {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the core OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_core))]
#[macro_export]
macro_rules! opencv_has_module_core {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the cudaarithm OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_cudaarithm)]
#[macro_export]
macro_rules! opencv_has_module_cudaarithm {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the cudaarithm OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_cudaarithm))]
#[macro_export]
macro_rules! opencv_has_module_cudaarithm {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the cudabgsegm OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_cudabgsegm)]
#[macro_export]
macro_rules! opencv_has_module_cudabgsegm {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the cudabgsegm OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_cudabgsegm))]
#[macro_export]
macro_rules! opencv_has_module_cudabgsegm {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the cudacodec OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_cudacodec)]
#[macro_export]
macro_rules! opencv_has_module_cudacodec {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the cudacodec OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_cudacodec))]
#[macro_export]
macro_rules! opencv_has_module_cudacodec {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the cudafeatures2d OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_cudafeatures2d)]
#[macro_export]
macro_rules! opencv_has_module_cudafeatures2d {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the cudafeatures2d OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_cudafeatures2d))]
#[macro_export]
macro_rules! opencv_has_module_cudafeatures2d {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the cudafilters OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_cudafilters)]
#[macro_export]
macro_rules! opencv_has_module_cudafilters {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the cudafilters OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_cudafilters))]
#[macro_export]
macro_rules! opencv_has_module_cudafilters {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the cudaimgproc OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_cudaimgproc)]
#[macro_export]
macro_rules! opencv_has_module_cudaimgproc {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the cudaimgproc OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_cudaimgproc))]
#[macro_export]
macro_rules! opencv_has_module_cudaimgproc {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the cudalegacy OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_cudalegacy)]
#[macro_export]
macro_rules! opencv_has_module_cudalegacy {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the cudalegacy OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_cudalegacy))]
#[macro_export]
macro_rules! opencv_has_module_cudalegacy {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the cudaobjdetect OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_cudaobjdetect)]
#[macro_export]
macro_rules! opencv_has_module_cudaobjdetect {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the cudaobjdetect OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_cudaobjdetect))]
#[macro_export]
macro_rules! opencv_has_module_cudaobjdetect {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the cudaoptflow OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_cudaoptflow)]
#[macro_export]
macro_rules! opencv_has_module_cudaoptflow {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the cudaoptflow OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_cudaoptflow))]
#[macro_export]
macro_rules! opencv_has_module_cudaoptflow {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the cudastereo OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_cudastereo)]
#[macro_export]
macro_rules! opencv_has_module_cudastereo {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the cudastereo OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_cudastereo))]
#[macro_export]
macro_rules! opencv_has_module_cudastereo {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the cudawarping OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_cudawarping)]
#[macro_export]
macro_rules! opencv_has_module_cudawarping {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the cudawarping OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_cudawarping))]
#[macro_export]
macro_rules! opencv_has_module_cudawarping {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the cvv OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_cvv)]
#[macro_export]
macro_rules! opencv_has_module_cvv {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the cvv OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_cvv))]
#[macro_export]
macro_rules! opencv_has_module_cvv {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the dnn OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_dnn)]
#[macro_export]
macro_rules! opencv_has_module_dnn {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the dnn OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_dnn))]
#[macro_export]
macro_rules! opencv_has_module_dnn {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the dnn_superres OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_dnn_superres)]
#[macro_export]
macro_rules! opencv_has_module_dnn_superres {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the dnn_superres OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_dnn_superres))]
#[macro_export]
macro_rules! opencv_has_module_dnn_superres {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the dpm OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_dpm)]
#[macro_export]
macro_rules! opencv_has_module_dpm {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the dpm OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_dpm))]
#[macro_export]
macro_rules! opencv_has_module_dpm {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the face OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_face)]
#[macro_export]
macro_rules! opencv_has_module_face {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the face OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_face))]
#[macro_export]
macro_rules! opencv_has_module_face {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the features OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_features)]
#[macro_export]
macro_rules! opencv_has_module_features {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the features OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_features))]
#[macro_export]
macro_rules! opencv_has_module_features {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the features2d OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_features2d)]
#[macro_export]
macro_rules! opencv_has_module_features2d {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the features2d OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_features2d))]
#[macro_export]
macro_rules! opencv_has_module_features2d {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the flann OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_flann)]
#[macro_export]
macro_rules! opencv_has_module_flann {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the flann OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_flann))]
#[macro_export]
macro_rules! opencv_has_module_flann {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the freetype OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_freetype)]
#[macro_export]
macro_rules! opencv_has_module_freetype {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the freetype OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_freetype))]
#[macro_export]
macro_rules! opencv_has_module_freetype {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the fuzzy OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_fuzzy)]
#[macro_export]
macro_rules! opencv_has_module_fuzzy {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the fuzzy OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_fuzzy))]
#[macro_export]
macro_rules! opencv_has_module_fuzzy {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the gapi OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_gapi)]
#[macro_export]
macro_rules! opencv_has_module_gapi {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the gapi OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_gapi))]
#[macro_export]
macro_rules! opencv_has_module_gapi {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the hdf OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_hdf)]
#[macro_export]
macro_rules! opencv_has_module_hdf {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the hdf OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_hdf))]
#[macro_export]
macro_rules! opencv_has_module_hdf {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the hfs OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_hfs)]
#[macro_export]
macro_rules! opencv_has_module_hfs {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the hfs OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_hfs))]
#[macro_export]
macro_rules! opencv_has_module_hfs {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the highgui OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_highgui)]
#[macro_export]
macro_rules! opencv_has_module_highgui {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the highgui OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_highgui))]
#[macro_export]
macro_rules! opencv_has_module_highgui {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the img_hash OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_img_hash)]
#[macro_export]
macro_rules! opencv_has_module_img_hash {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the img_hash OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_img_hash))]
#[macro_export]
macro_rules! opencv_has_module_img_hash {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the imgcodecs OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_imgcodecs)]
#[macro_export]
macro_rules! opencv_has_module_imgcodecs {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the imgcodecs OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_imgcodecs))]
#[macro_export]
macro_rules! opencv_has_module_imgcodecs {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the imgproc OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_imgproc)]
#[macro_export]
macro_rules! opencv_has_module_imgproc {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the imgproc OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_imgproc))]
#[macro_export]
macro_rules! opencv_has_module_imgproc {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the intensity_transform OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_intensity_transform)]
#[macro_export]
macro_rules! opencv_has_module_intensity_transform {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the intensity_transform OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_intensity_transform))]
#[macro_export]
macro_rules! opencv_has_module_intensity_transform {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the line_descriptor OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_line_descriptor)]
#[macro_export]
macro_rules! opencv_has_module_line_descriptor {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the line_descriptor OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_line_descriptor))]
#[macro_export]
macro_rules! opencv_has_module_line_descriptor {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the mcc OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_mcc)]
#[macro_export]
macro_rules! opencv_has_module_mcc {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the mcc OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_mcc))]
#[macro_export]
macro_rules! opencv_has_module_mcc {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the ml OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_ml)]
#[macro_export]
macro_rules! opencv_has_module_ml {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the ml OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_ml))]
#[macro_export]
macro_rules! opencv_has_module_ml {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the objdetect OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_objdetect)]
#[macro_export]
macro_rules! opencv_has_module_objdetect {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the objdetect OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_objdetect))]
#[macro_export]
macro_rules! opencv_has_module_objdetect {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the optflow OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_optflow)]
#[macro_export]
macro_rules! opencv_has_module_optflow {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the optflow OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_optflow))]
#[macro_export]
macro_rules! opencv_has_module_optflow {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the ovis OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_ovis)]
#[macro_export]
macro_rules! opencv_has_module_ovis {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the ovis OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_ovis))]
#[macro_export]
macro_rules! opencv_has_module_ovis {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the phase_unwrapping OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_phase_unwrapping)]
#[macro_export]
macro_rules! opencv_has_module_phase_unwrapping {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the phase_unwrapping OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_phase_unwrapping))]
#[macro_export]
macro_rules! opencv_has_module_phase_unwrapping {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the photo OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_photo)]
#[macro_export]
macro_rules! opencv_has_module_photo {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the photo OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_photo))]
#[macro_export]
macro_rules! opencv_has_module_photo {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the plot OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_plot)]
#[macro_export]
macro_rules! opencv_has_module_plot {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the plot OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_plot))]
#[macro_export]
macro_rules! opencv_has_module_plot {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the quality OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_quality)]
#[macro_export]
macro_rules! opencv_has_module_quality {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the quality OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_quality))]
#[macro_export]
macro_rules! opencv_has_module_quality {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the rapid OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_rapid)]
#[macro_export]
macro_rules! opencv_has_module_rapid {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the rapid OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_rapid))]
#[macro_export]
macro_rules! opencv_has_module_rapid {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the rgbd OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_rgbd)]
#[macro_export]
macro_rules! opencv_has_module_rgbd {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the rgbd OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_rgbd))]
#[macro_export]
macro_rules! opencv_has_module_rgbd {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the saliency OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_saliency)]
#[macro_export]
macro_rules! opencv_has_module_saliency {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the saliency OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_saliency))]
#[macro_export]
macro_rules! opencv_has_module_saliency {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the sfm OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_sfm)]
#[macro_export]
macro_rules! opencv_has_module_sfm {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the sfm OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_sfm))]
#[macro_export]
macro_rules! opencv_has_module_sfm {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the shape OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_shape)]
#[macro_export]
macro_rules! opencv_has_module_shape {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the shape OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_shape))]
#[macro_export]
macro_rules! opencv_has_module_shape {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the signal OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_signal)]
#[macro_export]
macro_rules! opencv_has_module_signal {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the signal OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_signal))]
#[macro_export]
macro_rules! opencv_has_module_signal {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the stereo OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_stereo)]
#[macro_export]
macro_rules! opencv_has_module_stereo {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the stereo OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_stereo))]
#[macro_export]
macro_rules! opencv_has_module_stereo {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the stitching OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_stitching)]
#[macro_export]
macro_rules! opencv_has_module_stitching {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the stitching OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_stitching))]
#[macro_export]
macro_rules! opencv_has_module_stitching {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the structured_light OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_structured_light)]
#[macro_export]
macro_rules! opencv_has_module_structured_light {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the structured_light OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_structured_light))]
#[macro_export]
macro_rules! opencv_has_module_structured_light {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the superres OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_superres)]
#[macro_export]
macro_rules! opencv_has_module_superres {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the superres OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_superres))]
#[macro_export]
macro_rules! opencv_has_module_superres {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the surface_matching OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_surface_matching)]
#[macro_export]
macro_rules! opencv_has_module_surface_matching {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the surface_matching OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_surface_matching))]
#[macro_export]
macro_rules! opencv_has_module_surface_matching {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the text OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_text)]
#[macro_export]
macro_rules! opencv_has_module_text {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the text OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_text))]
#[macro_export]
macro_rules! opencv_has_module_text {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the tracking OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_tracking)]
#[macro_export]
macro_rules! opencv_has_module_tracking {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the tracking OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_tracking))]
#[macro_export]
macro_rules! opencv_has_module_tracking {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the video OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_video)]
#[macro_export]
macro_rules! opencv_has_module_video {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the video OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_video))]
#[macro_export]
macro_rules! opencv_has_module_video {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the videoio OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_videoio)]
#[macro_export]
macro_rules! opencv_has_module_videoio {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the videoio OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_videoio))]
#[macro_export]
macro_rules! opencv_has_module_videoio {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the videostab OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_videostab)]
#[macro_export]
macro_rules! opencv_has_module_videostab {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the videostab OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_videostab))]
#[macro_export]
macro_rules! opencv_has_module_videostab {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the viz OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_viz)]
#[macro_export]
macro_rules! opencv_has_module_viz {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the viz OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_viz))]
#[macro_export]
macro_rules! opencv_has_module_viz {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the wechat_qrcode OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_wechat_qrcode)]
#[macro_export]
macro_rules! opencv_has_module_wechat_qrcode {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the wechat_qrcode OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_wechat_qrcode))]
#[macro_export]
macro_rules! opencv_has_module_wechat_qrcode {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the xfeatures2d OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_xfeatures2d)]
#[macro_export]
macro_rules! opencv_has_module_xfeatures2d {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the xfeatures2d OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_xfeatures2d))]
#[macro_export]
macro_rules! opencv_has_module_xfeatures2d {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the ximgproc OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_ximgproc)]
#[macro_export]
macro_rules! opencv_has_module_ximgproc {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the ximgproc OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_ximgproc))]
#[macro_export]
macro_rules! opencv_has_module_ximgproc {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the xobjdetect OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_xobjdetect)]
#[macro_export]
macro_rules! opencv_has_module_xobjdetect {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the xobjdetect OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_xobjdetect))]
#[macro_export]
macro_rules! opencv_has_module_xobjdetect {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the xphoto OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_xphoto)]
#[macro_export]
macro_rules! opencv_has_module_xphoto {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the xphoto OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_xphoto))]
#[macro_export]
macro_rules! opencv_has_module_xphoto {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether the xstereo OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(ocvrs_has_module_xstereo)]
#[macro_export]
macro_rules! opencv_has_module_xstereo {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether the xstereo OpenCV module is enabled
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let mut imgproc_enabled = opencv::opencv_has_module_imgproc! {
///     {
///         let mut input = Mat::default();
///         let rect = opencv::core::Rect::default();
///         opencv::imgproc::rectangle_def(&mut input, rect, (255., 0., 0.).into());
///         true
///     } else {
///         false
///     }
/// };
/// if !imgproc_enabled {
///     panic!("imgproc module is required");
/// }
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_module_imgproc! {
///    use opencv::imgproc::rectangle_def;
/// }
/// ```
#[cfg(not(ocvrs_has_module_xstereo))]
#[macro_export]
macro_rules! opencv_has_module_xstereo {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether OpenCV was built with or without the opencl feature
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let cuda_available = opencv::opencv_has_inherent_feature_cuda! {
///     {
///         true
///     } else {
///         false
///     }
/// };
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_inherent_feature_cuda! {
///    use opencv::core::GpuMat;
/// }
/// ```
#[cfg(ocvrs_has_inherent_feature_opencl)]
#[macro_export]
macro_rules! opencv_has_inherent_feature_opencl {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether OpenCV was built with or without the opencl feature
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let cuda_available = opencv::opencv_has_inherent_feature_cuda! {
///     {
///         true
///     } else {
///         false
///     }
/// };
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_inherent_feature_cuda! {
///    use opencv::core::GpuMat;
/// }
/// ```
#[cfg(not(ocvrs_has_inherent_feature_opencl))]
#[macro_export]
macro_rules! opencv_has_inherent_feature_opencl {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether OpenCV was built with or without the cuda feature
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let cuda_available = opencv::opencv_has_inherent_feature_cuda! {
///     {
///         true
///     } else {
///         false
///     }
/// };
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_inherent_feature_cuda! {
///    use opencv::core::GpuMat;
/// }
/// ```
#[cfg(ocvrs_has_inherent_feature_cuda)]
#[macro_export]
macro_rules! opencv_has_inherent_feature_cuda {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether OpenCV was built with or without the cuda feature
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let cuda_available = opencv::opencv_has_inherent_feature_cuda! {
///     {
///         true
///     } else {
///         false
///     }
/// };
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_inherent_feature_cuda! {
///    use opencv::core::GpuMat;
/// }
/// ```
#[cfg(not(ocvrs_has_inherent_feature_cuda))]
#[macro_export]
macro_rules! opencv_has_inherent_feature_cuda {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether OpenCV was built with or without the hfloat feature
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let cuda_available = opencv::opencv_has_inherent_feature_cuda! {
///     {
///         true
///     } else {
///         false
///     }
/// };
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_inherent_feature_cuda! {
///    use opencv::core::GpuMat;
/// }
/// ```
#[cfg(ocvrs_has_inherent_feature_hfloat)]
#[macro_export]
macro_rules! opencv_has_inherent_feature_hfloat {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether OpenCV was built with or without the hfloat feature
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let cuda_available = opencv::opencv_has_inherent_feature_cuda! {
///     {
///         true
///     } else {
///         false
///     }
/// };
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_inherent_feature_cuda! {
///    use opencv::core::GpuMat;
/// }
/// ```
#[cfg(not(ocvrs_has_inherent_feature_hfloat))]
#[macro_export]
macro_rules! opencv_has_inherent_feature_hfloat {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

/// Conditional compilation macro based on whether OpenCV was built with or without the algorithm_hint feature
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let cuda_available = opencv::opencv_has_inherent_feature_cuda! {
///     {
///         true
///     } else {
///         false
///     }
/// };
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_inherent_feature_cuda! {
///    use opencv::core::GpuMat;
/// }
/// ```
#[cfg(ocvrs_has_inherent_feature_algorithm_hint)]
#[macro_export]
macro_rules! opencv_has_inherent_feature_algorithm_hint {
	($bl_pos:block else $bl_neg:block) => { $bl_pos };
	($($tt:tt)*) => { $($tt)* };
}

/// Conditional compilation macro based on whether OpenCV was built with or without the algorithm_hint feature
///
/// The macro has two forms:
/// 1. Two blocks, separated by the `else` keyword. The first block will be compiled if the OpenCV feature is enabled, the second
///    one — if it's not. Note that both blocks must have the same return type.
/// 2. Plain token tree, for usage on the item level, e.g., for `use` imports. The code inside will be compiled if the OpenCV
///    feature is enabled and completely skipped if it's not.
///
/// # Examples
///
/// Two blocks with `else`:
/// ```
/// let cuda_available = opencv::opencv_has_inherent_feature_cuda! {
///     {
///         true
///     } else {
///         false
///     }
/// };
/// ```
///
/// Plain token tree:
/// ```
/// opencv::opencv_has_inherent_feature_cuda! {
///    use opencv::core::GpuMat;
/// }
/// ```
#[cfg(not(ocvrs_has_inherent_feature_algorithm_hint))]
#[macro_export]
macro_rules! opencv_has_inherent_feature_algorithm_hint {
	($bl_pos:block else $bl_neg:block) => { $bl_neg };
	($($tt:tt)*) => {  };
}

