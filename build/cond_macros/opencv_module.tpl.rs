/// Conditional compilation macro based on whether the {{ feature }} OpenCV module is enabled
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
#[cfg({{ cfg_condition_prefix }}ocvrs_has_module_{{ feature }}{{ cfg_condition_suffix }})]
#[macro_export]
macro_rules! opencv_has_module_{{ feature }} {
	($bl_pos:block else $bl_neg:block) => { {{ macro_body_block }} };
	($($tt:tt)*) => { {{ macro_body_tt }} };
}


