#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Tracking API
//!    # Tracking API implementation details
//!    # Legacy Tracking API
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::TrackerCSRT_ParamsTraitConst, super::TrackerCSRT_ParamsTrait, super::TrackerCSRTConst, super::TrackerCSRT, super::TrackerKCFConst, super::TrackerKCF };
}

/// \brief Feature type to be used in the tracking grayscale, colornames, compressed color-names
/// The modes available now:
///  *   "GRAY" -- Use grayscale values as the feature
///  *   "CN" -- Color-names feature
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TrackerKCF_MODE {
	GRAY = 1,
	CN = 2,
	CUSTOM = 4,
}

opencv_type_enum! { crate::tracking::TrackerKCF_MODE }

pub type TrackerKCF_FeatureExtractorCallbackFN = Option<unsafe extern "C" fn(*const c_void, core::Rect, *mut c_void) -> ()>;
/// the CSRT tracker
/// 
/// The implementation is based on [Lukezic_IJCV2018](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Lukezic_IJCV2018) Discriminative Correlation Filter with Channel and Spatial Reliability
pub trait TrackerCSRTConst: crate::video::TrackerConst {
	fn as_raw_TrackerCSRT(&self) -> *const c_void;

}

pub trait TrackerCSRT: crate::tracking::TrackerCSRTConst + crate::video::Tracker {
	fn as_raw_mut_TrackerCSRT(&mut self) -> *mut c_void;

	#[inline]
	fn set_initial_mask(&mut self, mask: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(mask);
		unsafe { sys::cv_tracking_TrackerCSRT_setInitialMask_const__InputArrayR(self.as_raw_mut_TrackerCSRT(), mask.as_raw__InputArray()) }.into_result()
	}
	
}

impl dyn TrackerCSRT + '_ {
	/// Create CSRT tracker instance
	/// ## Parameters
	/// * parameters: CSRT parameters TrackerCSRT::Params
	/// 
	/// ## C++ default parameters
	/// * parameters: TrackerCSRT::Params()
	#[inline]
	pub fn create(parameters: &crate::tracking::TrackerCSRT_Params) -> Result<core::Ptr<dyn crate::tracking::TrackerCSRT>> {
		unsafe { sys::cv_tracking_TrackerCSRT_create_const_ParamsR(parameters.as_raw_TrackerCSRT_Params()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::tracking::TrackerCSRT>::opencv_from_extern(r) } )
	}
	
}
pub trait TrackerCSRT_ParamsTraitConst {
	fn as_raw_TrackerCSRT_Params(&self) -> *const c_void;

	#[inline]
	fn use_hog(&self) -> bool {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropUse_hog_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: use_hog")
	}
	
	#[inline]
	fn use_color_names(&self) -> bool {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropUse_color_names_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: use_color_names")
	}
	
	#[inline]
	fn use_gray(&self) -> bool {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropUse_gray_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: use_gray")
	}
	
	#[inline]
	fn use_rgb(&self) -> bool {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropUse_rgb_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: use_rgb")
	}
	
	#[inline]
	fn use_channel_weights(&self) -> bool {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropUse_channel_weights_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: use_channel_weights")
	}
	
	#[inline]
	fn use_segmentation(&self) -> bool {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropUse_segmentation_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: use_segmentation")
	}
	
	/// Window function: "hann", "cheb", "kaiser"
	#[inline]
	fn window_function(&self) -> String {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropWindow_function_const(self.as_raw_TrackerCSRT_Params()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: window_function")
	}
	
	#[inline]
	fn kaiser_alpha(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropKaiser_alpha_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: kaiser_alpha")
	}
	
	#[inline]
	fn cheb_attenuation(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropCheb_attenuation_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: cheb_attenuation")
	}
	
	#[inline]
	fn template_size(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropTemplate_size_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: template_size")
	}
	
	#[inline]
	fn gsl_sigma(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropGsl_sigma_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: gsl_sigma")
	}
	
	#[inline]
	fn hog_orientations(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropHog_orientations_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: hog_orientations")
	}
	
	#[inline]
	fn hog_clip(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropHog_clip_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: hog_clip")
	}
	
	#[inline]
	fn padding(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropPadding_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: padding")
	}
	
	#[inline]
	fn filter_lr(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropFilter_lr_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: filter_lr")
	}
	
	#[inline]
	fn weights_lr(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropWeights_lr_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: weights_lr")
	}
	
	#[inline]
	fn num_hog_channels_used(&self) -> i32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropNum_hog_channels_used_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: num_hog_channels_used")
	}
	
	#[inline]
	fn admm_iterations(&self) -> i32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropAdmm_iterations_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: admm_iterations")
	}
	
	#[inline]
	fn histogram_bins(&self) -> i32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropHistogram_bins_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: histogram_bins")
	}
	
	#[inline]
	fn histogram_lr(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropHistogram_lr_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: histogram_lr")
	}
	
	#[inline]
	fn background_ratio(&self) -> i32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropBackground_ratio_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: background_ratio")
	}
	
	#[inline]
	fn number_of_scales(&self) -> i32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropNumber_of_scales_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: number_of_scales")
	}
	
	#[inline]
	fn scale_sigma_factor(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropScale_sigma_factor_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: scale_sigma_factor")
	}
	
	#[inline]
	fn scale_model_max_area(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropScale_model_max_area_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: scale_model_max_area")
	}
	
	#[inline]
	fn scale_lr(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropScale_lr_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: scale_lr")
	}
	
	#[inline]
	fn scale_step(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropScale_step_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: scale_step")
	}
	
	/// we lost the target, if the psr is lower than this.
	#[inline]
	fn psr_threshold(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropPsr_threshold_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: psr_threshold")
	}
	
}

pub trait TrackerCSRT_ParamsTrait: crate::tracking::TrackerCSRT_ParamsTraitConst {
	fn as_raw_mut_TrackerCSRT_Params(&mut self) -> *mut c_void;

	#[inline]
	fn set_use_hog(&mut self, val: bool) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropUse_hog_bool(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_use_hog")
	}
	
	#[inline]
	fn set_use_color_names(&mut self, val: bool) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropUse_color_names_bool(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_use_color_names")
	}
	
	#[inline]
	fn set_use_gray(&mut self, val: bool) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropUse_gray_bool(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_use_gray")
	}
	
	#[inline]
	fn set_use_rgb(&mut self, val: bool) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropUse_rgb_bool(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_use_rgb")
	}
	
	#[inline]
	fn set_use_channel_weights(&mut self, val: bool) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropUse_channel_weights_bool(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_use_channel_weights")
	}
	
	#[inline]
	fn set_use_segmentation(&mut self, val: bool) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropUse_segmentation_bool(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_use_segmentation")
	}
	
	/// Window function: "hann", "cheb", "kaiser"
	#[inline]
	fn set_window_function(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropWindow_function_string(self.as_raw_mut_TrackerCSRT_Params(), val.opencv_as_extern_mut()) }.into_result().expect("Infallible function failed: set_window_function")
	}
	
	#[inline]
	fn set_kaiser_alpha(&mut self, val: f32) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropKaiser_alpha_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_kaiser_alpha")
	}
	
	#[inline]
	fn set_cheb_attenuation(&mut self, val: f32) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropCheb_attenuation_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_cheb_attenuation")
	}
	
	#[inline]
	fn set_template_size(&mut self, val: f32) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropTemplate_size_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_template_size")
	}
	
	#[inline]
	fn set_gsl_sigma(&mut self, val: f32) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropGsl_sigma_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_gsl_sigma")
	}
	
	#[inline]
	fn set_hog_orientations(&mut self, val: f32) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropHog_orientations_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_hog_orientations")
	}
	
	#[inline]
	fn set_hog_clip(&mut self, val: f32) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropHog_clip_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_hog_clip")
	}
	
	#[inline]
	fn set_padding(&mut self, val: f32) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropPadding_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_padding")
	}
	
	#[inline]
	fn set_filter_lr(&mut self, val: f32) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropFilter_lr_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_filter_lr")
	}
	
	#[inline]
	fn set_weights_lr(&mut self, val: f32) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropWeights_lr_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_weights_lr")
	}
	
	#[inline]
	fn set_num_hog_channels_used(&mut self, val: i32) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropNum_hog_channels_used_int(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_num_hog_channels_used")
	}
	
	#[inline]
	fn set_admm_iterations(&mut self, val: i32) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropAdmm_iterations_int(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_admm_iterations")
	}
	
	#[inline]
	fn set_histogram_bins(&mut self, val: i32) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropHistogram_bins_int(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_histogram_bins")
	}
	
	#[inline]
	fn set_histogram_lr(&mut self, val: f32) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropHistogram_lr_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_histogram_lr")
	}
	
	#[inline]
	fn set_background_ratio(&mut self, val: i32) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropBackground_ratio_int(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_background_ratio")
	}
	
	#[inline]
	fn set_number_of_scales(&mut self, val: i32) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropNumber_of_scales_int(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_number_of_scales")
	}
	
	#[inline]
	fn set_scale_sigma_factor(&mut self, val: f32) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropScale_sigma_factor_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_scale_sigma_factor")
	}
	
	#[inline]
	fn set_scale_model_max_area(&mut self, val: f32) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropScale_model_max_area_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_scale_model_max_area")
	}
	
	#[inline]
	fn set_scale_lr(&mut self, val: f32) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropScale_lr_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_scale_lr")
	}
	
	#[inline]
	fn set_scale_step(&mut self, val: f32) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropScale_step_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_scale_step")
	}
	
	/// we lost the target, if the psr is lower than this.
	#[inline]
	fn set_psr_threshold(&mut self, val: f32) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropPsr_threshold_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_psr_threshold")
	}
	
}

pub struct TrackerCSRT_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerCSRT_Params }

impl Drop for TrackerCSRT_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerCSRT_Params_delete(instance: *mut c_void); }
		unsafe { cv_TrackerCSRT_Params_delete(self.as_raw_mut_TrackerCSRT_Params()) };
	}
}

unsafe impl Send for TrackerCSRT_Params {}

impl crate::tracking::TrackerCSRT_ParamsTraitConst for TrackerCSRT_Params {
	#[inline] fn as_raw_TrackerCSRT_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerCSRT_ParamsTrait for TrackerCSRT_Params {
	#[inline] fn as_raw_mut_TrackerCSRT_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerCSRT_Params {
	#[inline]
	pub fn default() -> Result<crate::tracking::TrackerCSRT_Params> {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_Params() }.into_result().map(|r| unsafe { crate::tracking::TrackerCSRT_Params::opencv_from_extern(r) } )
	}
	
}

/// the KCF (Kernelized Correlation Filter) tracker
/// 
/// * KCF is a novel tracking framework that utilizes properties of circulant matrix to enhance the processing speed.
/// * This tracking method is an implementation of [KCF_ECCV](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_KCF_ECCV) which is extended to KCF with color-names features ([KCF_CN](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_KCF_CN)).
/// * The original paper of KCF is available at <http://www.robots.ox.ac.uk/~joao/publications/henriques_tpami2015.pdf>
/// * as well as the matlab implementation. For more information about KCF with color-names features, please refer to
/// * <http://www.cvl.isy.liu.se/research/objrec/visualtracking/colvistrack/index.html>.
pub trait TrackerKCFConst: crate::video::TrackerConst {
	fn as_raw_TrackerKCF(&self) -> *const c_void;

}

pub trait TrackerKCF: crate::tracking::TrackerKCFConst + crate::video::Tracker {
	fn as_raw_mut_TrackerKCF(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * pca_func: false
	#[inline]
	fn set_feature_extractor(&mut self, callback: crate::tracking::TrackerKCF_FeatureExtractorCallbackFN, pca_func: bool) -> Result<()> {
		unsafe { sys::cv_tracking_TrackerKCF_setFeatureExtractor_FeatureExtractorCallbackFN_bool(self.as_raw_mut_TrackerKCF(), callback, pca_func) }.into_result()
	}
	
}

impl dyn TrackerKCF + '_ {
	/// Create KCF tracker instance
	/// ## Parameters
	/// * parameters: KCF parameters TrackerKCF::Params
	/// 
	/// ## C++ default parameters
	/// * parameters: TrackerKCF::Params()
	#[inline]
	pub fn create(parameters: crate::tracking::TrackerKCF_Params) -> Result<core::Ptr<dyn crate::tracking::TrackerKCF>> {
		unsafe { sys::cv_tracking_TrackerKCF_create_const_ParamsR(&parameters) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::tracking::TrackerKCF>::opencv_from_extern(r) } )
	}
	
}
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TrackerKCF_Params {
	/// detection confidence threshold
	pub detect_thresh: f32,
	/// gaussian kernel bandwidth
	pub sigma: f32,
	/// regularization
	pub lambda: f32,
	/// linear interpolation factor for adaptation
	pub interp_factor: f32,
	/// spatial bandwidth (proportional to target)
	pub output_sigma_factor: f32,
	/// compression learning rate
	pub pca_learning_rate: f32,
	/// activate the resize feature to improve the processing speed
	pub resize: bool,
	/// split the training coefficients into two matrices
	pub split_coeff: bool,
	/// wrap around the kernel values
	pub wrap_kernel: bool,
	/// activate the pca method to compress the features
	pub compress_feature: bool,
	/// threshold for the ROI size
	pub max_patch_size: i32,
	/// feature size after compression
	pub compressed_size: i32,
	/// compressed descriptors of TrackerKCF::MODE
	pub desc_pca: i32,
	/// non-compressed descriptors of TrackerKCF::MODE
	pub desc_npca: i32,
}

opencv_type_simple! { crate::tracking::TrackerKCF_Params }

impl TrackerKCF_Params {
	#[inline]
	pub fn default() -> Result<crate::tracking::TrackerKCF_Params> {
		unsafe { sys::cv_tracking_TrackerKCF_Params_Params() }.into_result()
	}
	
}
