//! # Tracking API
//!    # Tracking API implementation details
//!    # Legacy Tracking API
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{TrackerCSRTTrait, TrackerCSRTTraitConst, TrackerCSRT_ParamsTrait, TrackerCSRT_ParamsTraitConst, TrackerKCFTrait, TrackerKCFTraitConst};
}

// CN /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:113
pub const TrackerKCF_CN: i32 = 2;
// CUSTOM /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:114
pub const TrackerKCF_CUSTOM: i32 = 4;
// GRAY /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:112
pub const TrackerKCF_GRAY: i32 = 1;
/// \brief Feature type to be used in the tracking grayscale, colornames, compressed color-names
/// The modes available now:
///  *   "GRAY" -- Use grayscale values as the feature
///  *   "CN" -- Color-names feature
// MODE /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:111
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TrackerKCF_MODE {
	GRAY = 1,
	CN = 2,
	CUSTOM = 4,
}

impl TryFrom<i32> for TrackerKCF_MODE {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			1 => Ok(Self::GRAY),
			2 => Ok(Self::CN),
			4 => Ok(Self::CUSTOM),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::tracking::TrackerKCF_MODE"))),
		}
	}
}

opencv_type_enum! { crate::tracking::TrackerKCF_MODE }

// FeatureExtractorCallbackFN /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:147
pub type TrackerKCF_FeatureExtractorCallbackFN = Option<unsafe extern "C" fn(*const c_void, core::Rect, *mut c_void) -> ()>;
/// Constant methods for [crate::tracking::TrackerCSRT]
// TrackerCSRT /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:34
pub trait TrackerCSRTTraitConst: crate::video::TrackerTraitConst {
	fn as_raw_TrackerCSRT(&self) -> *const c_void;

}

/// Mutable methods for [crate::tracking::TrackerCSRT]
pub trait TrackerCSRTTrait: crate::tracking::TrackerCSRTTraitConst + crate::video::TrackerTrait {
	fn as_raw_mut_TrackerCSRT(&mut self) -> *mut c_void;

	// setInitialMask(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:86
	// ("cv::tracking::TrackerCSRT::setInitialMask", vec![(pred!(mut, ["mask"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn set_initial_mask(&mut self, mask: &impl ToInputArray) -> Result<()> {
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_tracking_TrackerCSRT_setInitialMask_const__InputArrayR(self.as_raw_mut_TrackerCSRT(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// the CSRT tracker
///
/// The implementation is based on [Lukezic_IJCV2018](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Lukezic_IJCV2018) Discriminative Correlation Filter with Channel and Spatial Reliability
// TrackerCSRT /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:34
pub struct TrackerCSRT {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerCSRT }

impl Drop for TrackerCSRT {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_tracking_TrackerCSRT_delete(self.as_raw_mut_TrackerCSRT()) };
	}
}

unsafe impl Send for TrackerCSRT {}

impl crate::video::TrackerTraitConst for TrackerCSRT {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::TrackerTrait for TrackerCSRT {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerCSRT, crate::video::TrackerTraitConst, as_raw_Tracker, crate::video::TrackerTrait, as_raw_mut_Tracker }

impl crate::tracking::TrackerCSRTTraitConst for TrackerCSRT {
	#[inline] fn as_raw_TrackerCSRT(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerCSRTTrait for TrackerCSRT {
	#[inline] fn as_raw_mut_TrackerCSRT(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerCSRT, crate::tracking::TrackerCSRTTraitConst, as_raw_TrackerCSRT, crate::tracking::TrackerCSRTTrait, as_raw_mut_TrackerCSRT }

impl TrackerCSRT {
	/// Create CSRT tracker instance
	/// ## Parameters
	/// * parameters: CSRT parameters TrackerCSRT::Params
	///
	/// ## C++ default parameters
	/// * parameters: TrackerCSRT::Params()
	// create(const TrackerCSRT::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:81
	// ("cv::tracking::TrackerCSRT::create", vec![(pred!(mut, ["parameters"], ["const cv::tracking::TrackerCSRT::Params*"]), _)]),
	#[inline]
	pub fn create(parameters: &impl crate::tracking::TrackerCSRT_ParamsTraitConst) -> Result<core::Ptr<crate::tracking::TrackerCSRT>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_tracking_TrackerCSRT_create_const_ParamsR(parameters.as_raw_TrackerCSRT_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::tracking::TrackerCSRT>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Create CSRT tracker instance
	/// ## Parameters
	/// * parameters: CSRT parameters TrackerCSRT::Params
	///
	/// ## Note
	/// This alternative version of [TrackerCSRT::create] function uses the following default values for its arguments:
	/// * parameters: TrackerCSRT::Params()
	// cv::tracking::TrackerCSRT::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:81
	// ("cv::tracking::TrackerCSRT::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::tracking::TrackerCSRT>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_tracking_TrackerCSRT_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::tracking::TrackerCSRT>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TrackerCSRT, crate::video::Tracker, cv_tracking_TrackerCSRT_to_Tracker }

impl std::fmt::Debug for TrackerCSRT {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerCSRT")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerCSRT_Params]
// Params /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:41
pub trait TrackerCSRT_ParamsTraitConst {
	fn as_raw_TrackerCSRT_Params(&self) -> *const c_void;

	// cv::tracking::TrackerCSRT::Params::use_hog() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:45
	// ("cv::tracking::TrackerCSRT::Params::use_hog", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn use_hog(&self) -> bool {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propUse_hog_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::use_color_names() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:46
	// ("cv::tracking::TrackerCSRT::Params::use_color_names", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn use_color_names(&self) -> bool {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propUse_color_names_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::use_gray() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:47
	// ("cv::tracking::TrackerCSRT::Params::use_gray", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn use_gray(&self) -> bool {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propUse_gray_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::use_rgb() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:48
	// ("cv::tracking::TrackerCSRT::Params::use_rgb", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn use_rgb(&self) -> bool {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propUse_rgb_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::use_channel_weights() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:49
	// ("cv::tracking::TrackerCSRT::Params::use_channel_weights", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn use_channel_weights(&self) -> bool {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propUse_channel_weights_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::use_segmentation() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:50
	// ("cv::tracking::TrackerCSRT::Params::use_segmentation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn use_segmentation(&self) -> bool {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propUse_segmentation_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	/// Window function: "hann", "cheb", "kaiser"
	// cv::tracking::TrackerCSRT::Params::window_function() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:52
	// ("cv::tracking::TrackerCSRT::Params::window_function", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn window_function(&self) -> String {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propWindow_function_const(self.as_raw_TrackerCSRT_Params()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::kaiser_alpha() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:53
	// ("cv::tracking::TrackerCSRT::Params::kaiser_alpha", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn kaiser_alpha(&self) -> f32 {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propKaiser_alpha_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::cheb_attenuation() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:54
	// ("cv::tracking::TrackerCSRT::Params::cheb_attenuation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn cheb_attenuation(&self) -> f32 {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propCheb_attenuation_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::template_size() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:56
	// ("cv::tracking::TrackerCSRT::Params::template_size", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn template_size(&self) -> f32 {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propTemplate_size_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::gsl_sigma() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:57
	// ("cv::tracking::TrackerCSRT::Params::gsl_sigma", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn gsl_sigma(&self) -> f32 {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propGsl_sigma_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::hog_orientations() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:58
	// ("cv::tracking::TrackerCSRT::Params::hog_orientations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn hog_orientations(&self) -> f32 {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propHog_orientations_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::hog_clip() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:59
	// ("cv::tracking::TrackerCSRT::Params::hog_clip", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn hog_clip(&self) -> f32 {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propHog_clip_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::padding() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:60
	// ("cv::tracking::TrackerCSRT::Params::padding", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn padding(&self) -> f32 {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propPadding_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::filter_lr() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:61
	// ("cv::tracking::TrackerCSRT::Params::filter_lr", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn filter_lr(&self) -> f32 {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propFilter_lr_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::weights_lr() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:62
	// ("cv::tracking::TrackerCSRT::Params::weights_lr", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn weights_lr(&self) -> f32 {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propWeights_lr_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::num_hog_channels_used() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:63
	// ("cv::tracking::TrackerCSRT::Params::num_hog_channels_used", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn num_hog_channels_used(&self) -> i32 {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propNum_hog_channels_used_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::admm_iterations() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:64
	// ("cv::tracking::TrackerCSRT::Params::admm_iterations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn admm_iterations(&self) -> i32 {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propAdmm_iterations_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::histogram_bins() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:65
	// ("cv::tracking::TrackerCSRT::Params::histogram_bins", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn histogram_bins(&self) -> i32 {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propHistogram_bins_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::histogram_lr() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:66
	// ("cv::tracking::TrackerCSRT::Params::histogram_lr", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn histogram_lr(&self) -> f32 {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propHistogram_lr_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::background_ratio() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:67
	// ("cv::tracking::TrackerCSRT::Params::background_ratio", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn background_ratio(&self) -> i32 {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propBackground_ratio_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::number_of_scales() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:68
	// ("cv::tracking::TrackerCSRT::Params::number_of_scales", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn number_of_scales(&self) -> i32 {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propNumber_of_scales_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::scale_sigma_factor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:69
	// ("cv::tracking::TrackerCSRT::Params::scale_sigma_factor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn scale_sigma_factor(&self) -> f32 {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propScale_sigma_factor_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::scale_model_max_area() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:70
	// ("cv::tracking::TrackerCSRT::Params::scale_model_max_area", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn scale_model_max_area(&self) -> f32 {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propScale_model_max_area_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::scale_lr() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:71
	// ("cv::tracking::TrackerCSRT::Params::scale_lr", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn scale_lr(&self) -> f32 {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propScale_lr_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::scale_step() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:72
	// ("cv::tracking::TrackerCSRT::Params::scale_step", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn scale_step(&self) -> f32 {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propScale_step_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	/// we lost the target, if the psr is lower than this.
	// cv::tracking::TrackerCSRT::Params::psr_threshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:74
	// ("cv::tracking::TrackerCSRT::Params::psr_threshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn psr_threshold(&self) -> f32 {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propPsr_threshold_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

}

/// Mutable methods for [crate::tracking::TrackerCSRT_Params]
pub trait TrackerCSRT_ParamsTrait: crate::tracking::TrackerCSRT_ParamsTraitConst {
	fn as_raw_mut_TrackerCSRT_Params(&mut self) -> *mut c_void;

	// cv::tracking::TrackerCSRT::Params::setUse_hog(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:45
	// ("cv::tracking::TrackerCSRT::Params::setUse_hog", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_use_hog(&mut self, val: bool) {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propUse_hog_const_bool(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::setUse_color_names(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:46
	// ("cv::tracking::TrackerCSRT::Params::setUse_color_names", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_use_color_names(&mut self, val: bool) {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propUse_color_names_const_bool(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::setUse_gray(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:47
	// ("cv::tracking::TrackerCSRT::Params::setUse_gray", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_use_gray(&mut self, val: bool) {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propUse_gray_const_bool(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::setUse_rgb(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:48
	// ("cv::tracking::TrackerCSRT::Params::setUse_rgb", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_use_rgb(&mut self, val: bool) {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propUse_rgb_const_bool(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::setUse_channel_weights(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:49
	// ("cv::tracking::TrackerCSRT::Params::setUse_channel_weights", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_use_channel_weights(&mut self, val: bool) {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propUse_channel_weights_const_bool(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::setUse_segmentation(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:50
	// ("cv::tracking::TrackerCSRT::Params::setUse_segmentation", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_use_segmentation(&mut self, val: bool) {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propUse_segmentation_const_bool(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	/// Window function: "hann", "cheb", "kaiser"
	// cv::tracking::TrackerCSRT::Params::setWindow_function(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:52
	// ("cv::tracking::TrackerCSRT::Params::setWindow_function", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
	#[inline]
	fn set_window_function(&mut self, val: &str) {
		extern_container_arg!(nofail val);
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propWindow_function_const_string(self.as_raw_mut_TrackerCSRT_Params(), val.opencv_as_extern()) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::setKaiser_alpha(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:53
	// ("cv::tracking::TrackerCSRT::Params::setKaiser_alpha", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_kaiser_alpha(&mut self, val: f32) {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propKaiser_alpha_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::setCheb_attenuation(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:54
	// ("cv::tracking::TrackerCSRT::Params::setCheb_attenuation", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_cheb_attenuation(&mut self, val: f32) {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propCheb_attenuation_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::setTemplate_size(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:56
	// ("cv::tracking::TrackerCSRT::Params::setTemplate_size", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_template_size(&mut self, val: f32) {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propTemplate_size_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::setGsl_sigma(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:57
	// ("cv::tracking::TrackerCSRT::Params::setGsl_sigma", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_gsl_sigma(&mut self, val: f32) {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propGsl_sigma_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::setHog_orientations(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:58
	// ("cv::tracking::TrackerCSRT::Params::setHog_orientations", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_hog_orientations(&mut self, val: f32) {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propHog_orientations_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::setHog_clip(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:59
	// ("cv::tracking::TrackerCSRT::Params::setHog_clip", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_hog_clip(&mut self, val: f32) {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propHog_clip_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::setPadding(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:60
	// ("cv::tracking::TrackerCSRT::Params::setPadding", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_padding(&mut self, val: f32) {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propPadding_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::setFilter_lr(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:61
	// ("cv::tracking::TrackerCSRT::Params::setFilter_lr", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_filter_lr(&mut self, val: f32) {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propFilter_lr_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::setWeights_lr(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:62
	// ("cv::tracking::TrackerCSRT::Params::setWeights_lr", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_weights_lr(&mut self, val: f32) {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propWeights_lr_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::setNum_hog_channels_used(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:63
	// ("cv::tracking::TrackerCSRT::Params::setNum_hog_channels_used", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_num_hog_channels_used(&mut self, val: i32) {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propNum_hog_channels_used_const_int(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::setAdmm_iterations(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:64
	// ("cv::tracking::TrackerCSRT::Params::setAdmm_iterations", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_admm_iterations(&mut self, val: i32) {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propAdmm_iterations_const_int(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::setHistogram_bins(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:65
	// ("cv::tracking::TrackerCSRT::Params::setHistogram_bins", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_histogram_bins(&mut self, val: i32) {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propHistogram_bins_const_int(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::setHistogram_lr(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:66
	// ("cv::tracking::TrackerCSRT::Params::setHistogram_lr", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_histogram_lr(&mut self, val: f32) {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propHistogram_lr_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::setBackground_ratio(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:67
	// ("cv::tracking::TrackerCSRT::Params::setBackground_ratio", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_background_ratio(&mut self, val: i32) {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propBackground_ratio_const_int(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::setNumber_of_scales(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:68
	// ("cv::tracking::TrackerCSRT::Params::setNumber_of_scales", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_number_of_scales(&mut self, val: i32) {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propNumber_of_scales_const_int(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::setScale_sigma_factor(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:69
	// ("cv::tracking::TrackerCSRT::Params::setScale_sigma_factor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_scale_sigma_factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propScale_sigma_factor_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::setScale_model_max_area(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:70
	// ("cv::tracking::TrackerCSRT::Params::setScale_model_max_area", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_scale_model_max_area(&mut self, val: f32) {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propScale_model_max_area_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::setScale_lr(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:71
	// ("cv::tracking::TrackerCSRT::Params::setScale_lr", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_scale_lr(&mut self, val: f32) {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propScale_lr_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::tracking::TrackerCSRT::Params::setScale_step(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:72
	// ("cv::tracking::TrackerCSRT::Params::setScale_step", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_scale_step(&mut self, val: f32) {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propScale_step_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	/// we lost the target, if the psr is lower than this.
	// cv::tracking::TrackerCSRT::Params::setPsr_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:74
	// ("cv::tracking::TrackerCSRT::Params::setPsr_threshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_psr_threshold(&mut self, val: f32) {
		let ret = unsafe { sys::cv_tracking_TrackerCSRT_Params_propPsr_threshold_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

}

// Params /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:41
pub struct TrackerCSRT_Params {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerCSRT_Params }

impl Drop for TrackerCSRT_Params {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_delete(self.as_raw_mut_TrackerCSRT_Params()) };
	}
}

unsafe impl Send for TrackerCSRT_Params {}

impl crate::tracking::TrackerCSRT_ParamsTraitConst for TrackerCSRT_Params {
	#[inline] fn as_raw_TrackerCSRT_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerCSRT_ParamsTrait for TrackerCSRT_Params {
	#[inline] fn as_raw_mut_TrackerCSRT_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerCSRT_Params, crate::tracking::TrackerCSRT_ParamsTraitConst, as_raw_TrackerCSRT_Params, crate::tracking::TrackerCSRT_ParamsTrait, as_raw_mut_TrackerCSRT_Params }

impl TrackerCSRT_Params {
	// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:43
	// ("cv::tracking::TrackerCSRT::Params::Params", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::tracking::TrackerCSRT_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_tracking_TrackerCSRT_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerCSRT_Params::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl Clone for TrackerCSRT_Params {
	#[inline]
	fn clone(&self) -> Self {
		unsafe { Self::from_raw(sys::cv_tracking_TrackerCSRT_Params_implicitClone_const(self.as_raw_TrackerCSRT_Params())) }
	}
}

impl std::fmt::Debug for TrackerCSRT_Params {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerCSRT_Params")
			.field("use_hog", &crate::tracking::TrackerCSRT_ParamsTraitConst::use_hog(self))
			.field("use_color_names", &crate::tracking::TrackerCSRT_ParamsTraitConst::use_color_names(self))
			.field("use_gray", &crate::tracking::TrackerCSRT_ParamsTraitConst::use_gray(self))
			.field("use_rgb", &crate::tracking::TrackerCSRT_ParamsTraitConst::use_rgb(self))
			.field("use_channel_weights", &crate::tracking::TrackerCSRT_ParamsTraitConst::use_channel_weights(self))
			.field("use_segmentation", &crate::tracking::TrackerCSRT_ParamsTraitConst::use_segmentation(self))
			.field("window_function", &crate::tracking::TrackerCSRT_ParamsTraitConst::window_function(self))
			.field("kaiser_alpha", &crate::tracking::TrackerCSRT_ParamsTraitConst::kaiser_alpha(self))
			.field("cheb_attenuation", &crate::tracking::TrackerCSRT_ParamsTraitConst::cheb_attenuation(self))
			.field("template_size", &crate::tracking::TrackerCSRT_ParamsTraitConst::template_size(self))
			.field("gsl_sigma", &crate::tracking::TrackerCSRT_ParamsTraitConst::gsl_sigma(self))
			.field("hog_orientations", &crate::tracking::TrackerCSRT_ParamsTraitConst::hog_orientations(self))
			.field("hog_clip", &crate::tracking::TrackerCSRT_ParamsTraitConst::hog_clip(self))
			.field("padding", &crate::tracking::TrackerCSRT_ParamsTraitConst::padding(self))
			.field("filter_lr", &crate::tracking::TrackerCSRT_ParamsTraitConst::filter_lr(self))
			.field("weights_lr", &crate::tracking::TrackerCSRT_ParamsTraitConst::weights_lr(self))
			.field("num_hog_channels_used", &crate::tracking::TrackerCSRT_ParamsTraitConst::num_hog_channels_used(self))
			.field("admm_iterations", &crate::tracking::TrackerCSRT_ParamsTraitConst::admm_iterations(self))
			.field("histogram_bins", &crate::tracking::TrackerCSRT_ParamsTraitConst::histogram_bins(self))
			.field("histogram_lr", &crate::tracking::TrackerCSRT_ParamsTraitConst::histogram_lr(self))
			.field("background_ratio", &crate::tracking::TrackerCSRT_ParamsTraitConst::background_ratio(self))
			.field("number_of_scales", &crate::tracking::TrackerCSRT_ParamsTraitConst::number_of_scales(self))
			.field("scale_sigma_factor", &crate::tracking::TrackerCSRT_ParamsTraitConst::scale_sigma_factor(self))
			.field("scale_model_max_area", &crate::tracking::TrackerCSRT_ParamsTraitConst::scale_model_max_area(self))
			.field("scale_lr", &crate::tracking::TrackerCSRT_ParamsTraitConst::scale_lr(self))
			.field("scale_step", &crate::tracking::TrackerCSRT_ParamsTraitConst::scale_step(self))
			.field("psr_threshold", &crate::tracking::TrackerCSRT_ParamsTraitConst::psr_threshold(self))
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerKCF]
// TrackerKCF /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:98
pub trait TrackerKCFTraitConst: crate::video::TrackerTraitConst {
	fn as_raw_TrackerKCF(&self) -> *const c_void;

}

/// Mutable methods for [crate::tracking::TrackerKCF]
pub trait TrackerKCFTrait: crate::tracking::TrackerKCFTraitConst + crate::video::TrackerTrait {
	fn as_raw_mut_TrackerKCF(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * pca_func: false
	// setFeatureExtractor(FeatureExtractorCallbackFN, bool)(Function, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:148
	// ("cv::tracking::TrackerKCF::setFeatureExtractor", vec![(pred!(mut, ["callback", "pca_func"], ["cv::tracking::TrackerKCF::FeatureExtractorCallbackFN", "bool"]), _)]),
	#[inline]
	fn set_feature_extractor(&mut self, callback: crate::tracking::TrackerKCF_FeatureExtractorCallbackFN, pca_func: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_tracking_TrackerKCF_setFeatureExtractor_FeatureExtractorCallbackFN_bool(self.as_raw_mut_TrackerKCF(), callback, pca_func, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [TrackerKCFTrait::set_feature_extractor] function uses the following default values for its arguments:
	/// * pca_func: false
	// cv::tracking::TrackerKCF::setFeatureExtractor(Function) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:148
	// ("cv::tracking::TrackerKCF::setFeatureExtractor", vec![(pred!(mut, ["callback"], ["cv::tracking::TrackerKCF::FeatureExtractorCallbackFN"]), _)]),
	#[inline]
	fn set_feature_extractor_def(&mut self, callback: crate::tracking::TrackerKCF_FeatureExtractorCallbackFN) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_tracking_TrackerKCF_setFeatureExtractor_FeatureExtractorCallbackFN(self.as_raw_mut_TrackerKCF(), callback, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// the KCF (Kernelized Correlation Filter) tracker
///
/// * KCF is a novel tracking framework that utilizes properties of circulant matrix to enhance the processing speed.
/// * This tracking method is an implementation of [KCF_ECCV](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_KCF_ECCV) which is extended to KCF with color-names features ([KCF_CN](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_KCF_CN)).
/// * The original paper of KCF is available at <http://www.robots.ox.ac.uk/~joao/publications/henriques_tpami2015.pdf>
/// * as well as the matlab implementation. For more information about KCF with color-names features, please refer to
/// * <http://www.cvl.isy.liu.se/research/objrec/visualtracking/colvistrack/index.html>.
// TrackerKCF /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:98
pub struct TrackerKCF {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerKCF }

impl Drop for TrackerKCF {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_tracking_TrackerKCF_delete(self.as_raw_mut_TrackerKCF()) };
	}
}

unsafe impl Send for TrackerKCF {}

impl crate::video::TrackerTraitConst for TrackerKCF {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::TrackerTrait for TrackerKCF {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerKCF, crate::video::TrackerTraitConst, as_raw_Tracker, crate::video::TrackerTrait, as_raw_mut_Tracker }

impl crate::tracking::TrackerKCFTraitConst for TrackerKCF {
	#[inline] fn as_raw_TrackerKCF(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerKCFTrait for TrackerKCF {
	#[inline] fn as_raw_mut_TrackerKCF(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerKCF, crate::tracking::TrackerKCFTraitConst, as_raw_TrackerKCF, crate::tracking::TrackerKCFTrait, as_raw_mut_TrackerKCF }

impl TrackerKCF {
	/// Create KCF tracker instance
	/// ## Parameters
	/// * parameters: KCF parameters TrackerKCF::Params
	///
	/// ## C++ default parameters
	/// * parameters: TrackerKCF::Params()
	// create(const TrackerKCF::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:141
	// ("cv::tracking::TrackerKCF::create", vec![(pred!(mut, ["parameters"], ["const cv::tracking::TrackerKCF::Params*"]), _)]),
	#[inline]
	pub fn create(parameters: crate::tracking::TrackerKCF_Params) -> Result<core::Ptr<crate::tracking::TrackerKCF>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_tracking_TrackerKCF_create_const_ParamsR(&parameters, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::tracking::TrackerKCF>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Create KCF tracker instance
	/// ## Parameters
	/// * parameters: KCF parameters TrackerKCF::Params
	///
	/// ## Note
	/// This alternative version of [TrackerKCF::create] function uses the following default values for its arguments:
	/// * parameters: TrackerKCF::Params()
	// cv::tracking::TrackerKCF::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:141
	// ("cv::tracking::TrackerKCF::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::tracking::TrackerKCF>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_tracking_TrackerKCF_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::tracking::TrackerKCF>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TrackerKCF, crate::video::Tracker, cv_tracking_TrackerKCF_to_Tracker }

impl std::fmt::Debug for TrackerKCF {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerKCF")
			.finish()
	}
}

// Params /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:117
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
	// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/tracking.hpp:119
	// ("cv::tracking::TrackerKCF::Params::Params", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::tracking::TrackerKCF_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_tracking_TrackerKCF_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}
