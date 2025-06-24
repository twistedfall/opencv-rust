/// Conditional compilation macro based on whether OpenCV was built with or without the {{ feature }} feature
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
#[cfg({{ cfg_condition_prefix }}ocvrs_has_inherent_feature_{{ feature }}{{ cfg_condition_suffix }})]
#[macro_export]
macro_rules! opencv_has_inherent_feature_{{ feature }} {
	($bl_pos:block else $bl_neg:block) => { {{ macro_body_block }} };
	($($tt:tt)*) => { {{ macro_body_tt }} };
}


