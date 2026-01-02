pub mod tracking {
	//! # Tracking API
	//!    # Tracking API implementation details
	//!    # Legacy Tracking API
	use crate::mod_prelude::*;
	use crate::{core, sys, types};
	pub mod prelude {
		pub use super::{Detail_TrackerContribFeatureHAARTrait, Detail_TrackerContribFeatureHAARTraitConst, Detail_TrackerContribFeatureHAAR_ParamsTrait, Detail_TrackerContribFeatureHAAR_ParamsTraitConst, Detail_TrackerContribFeatureSetTrait, Detail_TrackerContribFeatureSetTraitConst, Detail_TrackerContribFeatureTrait, Detail_TrackerContribFeatureTraitConst, Detail_TrackerContribSamplerAlgorithmTrait, Detail_TrackerContribSamplerAlgorithmTraitConst, Detail_TrackerContribSamplerCSCTrait, Detail_TrackerContribSamplerCSCTraitConst, Detail_TrackerContribSamplerCSC_ParamsTrait, Detail_TrackerContribSamplerCSC_ParamsTraitConst, Detail_TrackerContribSamplerTrait, Detail_TrackerContribSamplerTraitConst, Detail_TrackerFeatureFeature2dTrait, Detail_TrackerFeatureFeature2dTraitConst, Detail_TrackerFeatureHOGTrait, Detail_TrackerFeatureHOGTraitConst, Detail_TrackerFeatureLBPTrait, Detail_TrackerFeatureLBPTraitConst, Detail_TrackerFeatureSetTrait, Detail_TrackerFeatureSetTraitConst, Detail_TrackerFeatureTrait, Detail_TrackerFeatureTraitConst, Detail_TrackerModelTrait, Detail_TrackerModelTraitConst, Detail_TrackerSamplerAlgorithmTrait, Detail_TrackerSamplerAlgorithmTraitConst, Detail_TrackerSamplerCSCTrait, Detail_TrackerSamplerCSCTraitConst, Detail_TrackerSamplerCSC_ParamsTrait, Detail_TrackerSamplerCSC_ParamsTraitConst, Detail_TrackerSamplerCSTrait, Detail_TrackerSamplerCSTraitConst, Detail_TrackerSamplerCS_ParamsTrait, Detail_TrackerSamplerCS_ParamsTraitConst, Detail_TrackerSamplerPFTrait, Detail_TrackerSamplerPFTraitConst, Detail_TrackerSamplerPF_ParamsTrait, Detail_TrackerSamplerPF_ParamsTraitConst, Detail_TrackerSamplerTrait, Detail_TrackerSamplerTraitConst, Detail_TrackerStateEstimatorAdaBoostingTrait, Detail_TrackerStateEstimatorAdaBoostingTraitConst, Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetStateTrait, Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetStateTraitConst, Detail_TrackerStateEstimatorSVMTrait, Detail_TrackerStateEstimatorSVMTraitConst, Detail_TrackerStateEstimatorTrait, Detail_TrackerStateEstimatorTraitConst, Detail_TrackerTargetStateTrait, Detail_TrackerTargetStateTraitConst, Legacy_MultiTrackerTLDTrait, Legacy_MultiTrackerTLDTraitConst, Legacy_MultiTrackerTrait, Legacy_MultiTrackerTraitConst, Legacy_MultiTracker_AltTrait, Legacy_MultiTracker_AltTraitConst, Legacy_TrackerBoostingTrait, Legacy_TrackerBoostingTraitConst, Legacy_TrackerBoosting_ParamsTrait, Legacy_TrackerBoosting_ParamsTraitConst, Legacy_TrackerCSRTTrait, Legacy_TrackerCSRTTraitConst, Legacy_TrackerCSRT_ParamsTrait, Legacy_TrackerCSRT_ParamsTraitConst, Legacy_TrackerKCFTrait, Legacy_TrackerKCFTraitConst, Legacy_TrackerKCF_ParamsTrait, Legacy_TrackerKCF_ParamsTraitConst, Legacy_TrackerMILTrait, Legacy_TrackerMILTraitConst, Legacy_TrackerMIL_ParamsTrait, Legacy_TrackerMIL_ParamsTraitConst, Legacy_TrackerMOSSETrait, Legacy_TrackerMOSSETraitConst, Legacy_TrackerMedianFlowTrait, Legacy_TrackerMedianFlowTraitConst, Legacy_TrackerMedianFlow_ParamsTrait, Legacy_TrackerMedianFlow_ParamsTraitConst, Legacy_TrackerTLDTrait, Legacy_TrackerTLDTraitConst, Legacy_TrackerTLD_ParamsTrait, Legacy_TrackerTLD_ParamsTraitConst, Legacy_TrackerTrait, Legacy_TrackerTraitConst, TrackerCSRTTrait, TrackerCSRTTraitConst, TrackerCSRT_ParamsTrait, TrackerCSRT_ParamsTraitConst, TrackerKCFTrait, TrackerKCFTraitConst};
	}

	pub const CC_FEATURE_PARAMS: &str = "featureParams";
	pub const CC_FEATURE_SIZE: &str = "featSize";
	pub const CC_ISINTEGRAL: &str = "isIntegral";
	pub const CC_MAX_CAT_COUNT: &str = "maxCatCount";
	pub const CC_NUM_FEATURES: &str = "numFeat";
	pub const CC_RECT: &str = "rect";
	pub const CC_RECTS: &str = "rects";
	pub const CC_TILTED: &str = "tilted";
	pub const CV_HAAR_FEATURE_MAX: i32 = 3;
	/// mode for detect samples
	pub const Detail_TrackerContribSamplerCSC_MODE_DETECT: i32 = 5;
	/// mode for init negative samples
	pub const Detail_TrackerContribSamplerCSC_MODE_INIT_NEG: i32 = 2;
	/// mode for init positive samples
	pub const Detail_TrackerContribSamplerCSC_MODE_INIT_POS: i32 = 1;
	/// mode for update negative samples
	pub const Detail_TrackerContribSamplerCSC_MODE_TRACK_NEG: i32 = 4;
	/// mode for update positive samples
	pub const Detail_TrackerContribSamplerCSC_MODE_TRACK_POS: i32 = 3;
	/// mode for detect samples
	pub const Detail_TrackerSamplerCSC_MODE_DETECT: i32 = 5;
	/// mode for init negative samples
	pub const Detail_TrackerSamplerCSC_MODE_INIT_NEG: i32 = 2;
	/// mode for init positive samples
	pub const Detail_TrackerSamplerCSC_MODE_INIT_POS: i32 = 1;
	/// mode for update negative samples
	pub const Detail_TrackerSamplerCSC_MODE_TRACK_NEG: i32 = 4;
	/// mode for update positive samples
	pub const Detail_TrackerSamplerCSC_MODE_TRACK_POS: i32 = 3;
	/// mode for classify samples
	pub const Detail_TrackerSamplerCS_MODE_CLASSIFY: i32 = 3;
	/// mode for negative samples
	pub const Detail_TrackerSamplerCS_MODE_NEGATIVE: i32 = 2;
	/// mode for positive samples
	pub const Detail_TrackerSamplerCS_MODE_POSITIVE: i32 = 1;
	pub const FEATURES: &str = "features";
	pub const HFP_NAME: &str = "haarFeatureParams";
	pub const HOGF_NAME: &str = "HOGFeatureParams";
	pub const LBPF_NAME: &str = "lbpFeatureParams";
	pub const N_BINS: i32 = 9;
	pub const N_CELLS: i32 = 4;
	pub const TrackerKCF_CN: i32 = 2;
	pub const TrackerKCF_CUSTOM: i32 = 4;
	pub const TrackerKCF_GRAY: i32 = 1;
	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum Detail_TrackerSamplerCSC_MODE {
		/// mode for init positive samples
		MODE_INIT_POS = 1,
		/// mode for init negative samples
		MODE_INIT_NEG = 2,
		/// mode for update positive samples
		MODE_TRACK_POS = 3,
		/// mode for update negative samples
		MODE_TRACK_NEG = 4,
		/// mode for detect samples
		MODE_DETECT = 5,
	}

	opencv_type_enum! { crate::tracking::Detail_TrackerSamplerCSC_MODE { MODE_INIT_POS, MODE_INIT_NEG, MODE_TRACK_POS, MODE_TRACK_NEG, MODE_DETECT } }

	/// \brief Feature type to be used in the tracking grayscale, colornames, compressed color-names
	/// The modes available now:
	///  *   "GRAY" -- Use grayscale values as the feature
	///  *   "CN" -- Color-names feature
	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum TrackerKCF_MODE {
		GRAY = 1,
		CN = 2,
		CUSTOM = 4,
	}

	opencv_type_enum! { crate::tracking::TrackerKCF_MODE { GRAY, CN, CUSTOM } }

	pub type TrackerKCF_FeatureExtractorCallbackFN = Option<unsafe extern "C" fn(*const c_void, core::Rect, *mut c_void) -> ()>;
	/// Represents the model of the target at frame ![inline formula](https://latex.codecogs.com/png.latex?k) (all states and scores)
	///
	/// See [AAM](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_AAM) The set of the pair ![inline formula](https://latex.codecogs.com/png.latex?%5Clangle%20%5Chat%7Bx%7D%5E%7Bi%7D%5F%7Bk%7D%2C%20C%5E%7Bi%7D%5F%7Bk%7D%20%5Crangle)
	/// ## See also
	/// TrackerTargetState
	pub type Detail_ConfidenceMap = core::Vector<core::Tuple<(core::Ptr<crate::tracking::Detail_TrackerTargetState>, f32)>>;
	/// Represents the estimate states for all frames
	///
	/// [AAM](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_AAM) ![inline formula](https://latex.codecogs.com/png.latex?x%5F%7Bk%7D) is the trajectory of the target up to time ![inline formula](https://latex.codecogs.com/png.latex?k)
	/// ## See also
	/// TrackerTargetState
	pub type Detail_Trajectory = core::Vector<core::Ptr<crate::tracking::Detail_TrackerTargetState>>;
	/// \brief Feature type to be used in the tracking grayscale, colornames, compressed color-names
	///  The modes available now:
	/// *   "GRAY" -- Use grayscale values as the feature
	/// *   "CN" -- Color-names feature
	pub type Legacy_TrackerKCF_MODE = crate::tracking::TrackerKCF_MODE;
	#[inline]
	pub fn upgrade_tracking_api(legacy_tracker: &core::Ptr<crate::tracking::Legacy_Tracker>) -> Result<core::Ptr<crate::video::Tracker>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_legacy_upgradeTrackingAPI_const_PtrLTrackerGR(legacy_tracker.as_raw_PtrOfLegacy_Tracker(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::video::Tracker>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// the CSRT tracker
	///
	/// The implementation is based on [Lukezic_IJCV2018](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_Lukezic_IJCV2018) Discriminative Correlation Filter with Channel and Spatial Reliability
	pub struct TrackerCSRT {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { TrackerCSRT }

	impl Drop for TrackerCSRT {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_TrackerCSRT_delete(self.as_raw_mut_TrackerCSRT()) };
		}
	}

	unsafe impl Send for TrackerCSRT {}

	impl TrackerCSRT {
		/// Create CSRT tracker instance
		/// ## Parameters
		/// * parameters: CSRT parameters TrackerCSRT::Params
		///
		/// ## C++ default parameters
		/// * parameters: TrackerCSRT::Params()
		#[inline]
		pub fn create(parameters: &impl crate::tracking::TrackerCSRT_ParamsTraitConst) -> Result<core::Ptr<crate::tracking::TrackerCSRT>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_TrackerCSRT_create_const_ParamsR(parameters.as_raw_TrackerCSRT_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::tracking::TrackerCSRT>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_TrackerCSRT_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::tracking::TrackerCSRT>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::TrackerCSRT]
	pub trait TrackerCSRTTraitConst: crate::video::TrackerTraitConst {
		fn as_raw_TrackerCSRT(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::tracking::TrackerCSRT]
	pub trait TrackerCSRTTrait: crate::tracking::TrackerCSRTTraitConst + crate::video::TrackerTrait {
		fn as_raw_mut_TrackerCSRT(&mut self) -> *mut c_void;

		#[inline]
		fn set_initial_mask(&mut self, mask: &impl ToInputArray) -> Result<()> {
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_TrackerCSRT_setInitialMask_const__InputArrayR(self.as_raw_mut_TrackerCSRT(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for TrackerCSRT {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("TrackerCSRT")
				.finish()
		}
	}

	boxed_cast_base! { TrackerCSRT, crate::video::Tracker, cv_TrackerCSRT_to_Tracker }

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

	pub struct TrackerCSRT_Params {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { TrackerCSRT_Params }

	impl Drop for TrackerCSRT_Params {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_TrackerCSRT_Params_delete(self.as_raw_mut_TrackerCSRT_Params()) };
		}
	}

	unsafe impl Send for TrackerCSRT_Params {}

	impl TrackerCSRT_Params {
		#[inline]
		pub fn default() -> Result<crate::tracking::TrackerCSRT_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_TrackerCSRT_Params_Params(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::TrackerCSRT_Params::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::TrackerCSRT_Params]
	pub trait TrackerCSRT_ParamsTraitConst {
		fn as_raw_TrackerCSRT_Params(&self) -> *const c_void;

		#[inline]
		fn use_hog(&self) -> bool {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propUse_hog_const(self.as_raw_TrackerCSRT_Params()) };
			ret
		}

		#[inline]
		fn use_color_names(&self) -> bool {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propUse_color_names_const(self.as_raw_TrackerCSRT_Params()) };
			ret
		}

		#[inline]
		fn use_gray(&self) -> bool {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propUse_gray_const(self.as_raw_TrackerCSRT_Params()) };
			ret
		}

		#[inline]
		fn use_rgb(&self) -> bool {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propUse_rgb_const(self.as_raw_TrackerCSRT_Params()) };
			ret
		}

		#[inline]
		fn use_channel_weights(&self) -> bool {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propUse_channel_weights_const(self.as_raw_TrackerCSRT_Params()) };
			ret
		}

		#[inline]
		fn use_segmentation(&self) -> bool {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propUse_segmentation_const(self.as_raw_TrackerCSRT_Params()) };
			ret
		}

		/// Window function: "hann", "cheb", "kaiser"
		#[inline]
		fn window_function(&self) -> String {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propWindow_function_const(self.as_raw_TrackerCSRT_Params()) };
			let ret = unsafe { String::opencv_from_extern(ret) };
			ret
		}

		#[inline]
		fn kaiser_alpha(&self) -> f32 {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propKaiser_alpha_const(self.as_raw_TrackerCSRT_Params()) };
			ret
		}

		#[inline]
		fn cheb_attenuation(&self) -> f32 {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propCheb_attenuation_const(self.as_raw_TrackerCSRT_Params()) };
			ret
		}

		#[inline]
		fn template_size(&self) -> f32 {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propTemplate_size_const(self.as_raw_TrackerCSRT_Params()) };
			ret
		}

		#[inline]
		fn gsl_sigma(&self) -> f32 {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propGsl_sigma_const(self.as_raw_TrackerCSRT_Params()) };
			ret
		}

		#[inline]
		fn hog_orientations(&self) -> f32 {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propHog_orientations_const(self.as_raw_TrackerCSRT_Params()) };
			ret
		}

		#[inline]
		fn hog_clip(&self) -> f32 {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propHog_clip_const(self.as_raw_TrackerCSRT_Params()) };
			ret
		}

		#[inline]
		fn padding(&self) -> f32 {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propPadding_const(self.as_raw_TrackerCSRT_Params()) };
			ret
		}

		#[inline]
		fn filter_lr(&self) -> f32 {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propFilter_lr_const(self.as_raw_TrackerCSRT_Params()) };
			ret
		}

		#[inline]
		fn weights_lr(&self) -> f32 {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propWeights_lr_const(self.as_raw_TrackerCSRT_Params()) };
			ret
		}

		#[inline]
		fn num_hog_channels_used(&self) -> i32 {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propNum_hog_channels_used_const(self.as_raw_TrackerCSRT_Params()) };
			ret
		}

		#[inline]
		fn admm_iterations(&self) -> i32 {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propAdmm_iterations_const(self.as_raw_TrackerCSRT_Params()) };
			ret
		}

		#[inline]
		fn histogram_bins(&self) -> i32 {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propHistogram_bins_const(self.as_raw_TrackerCSRT_Params()) };
			ret
		}

		#[inline]
		fn histogram_lr(&self) -> f32 {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propHistogram_lr_const(self.as_raw_TrackerCSRT_Params()) };
			ret
		}

		#[inline]
		fn background_ratio(&self) -> i32 {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propBackground_ratio_const(self.as_raw_TrackerCSRT_Params()) };
			ret
		}

		#[inline]
		fn number_of_scales(&self) -> i32 {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propNumber_of_scales_const(self.as_raw_TrackerCSRT_Params()) };
			ret
		}

		#[inline]
		fn scale_sigma_factor(&self) -> f32 {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propScale_sigma_factor_const(self.as_raw_TrackerCSRT_Params()) };
			ret
		}

		#[inline]
		fn scale_model_max_area(&self) -> f32 {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propScale_model_max_area_const(self.as_raw_TrackerCSRT_Params()) };
			ret
		}

		#[inline]
		fn scale_lr(&self) -> f32 {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propScale_lr_const(self.as_raw_TrackerCSRT_Params()) };
			ret
		}

		#[inline]
		fn scale_step(&self) -> f32 {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propScale_step_const(self.as_raw_TrackerCSRT_Params()) };
			ret
		}

		/// we lost the target, if the psr is lower than this.
		#[inline]
		fn psr_threshold(&self) -> f32 {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propPsr_threshold_const(self.as_raw_TrackerCSRT_Params()) };
			ret
		}

	}

	/// Mutable methods for [crate::tracking::TrackerCSRT_Params]
	pub trait TrackerCSRT_ParamsTrait: crate::tracking::TrackerCSRT_ParamsTraitConst {
		fn as_raw_mut_TrackerCSRT_Params(&mut self) -> *mut c_void;

		#[inline]
		fn set_use_hog(&mut self, val: bool) {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propUse_hog_const_bool(self.as_raw_mut_TrackerCSRT_Params(), val) };
			ret
		}

		#[inline]
		fn set_use_color_names(&mut self, val: bool) {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propUse_color_names_const_bool(self.as_raw_mut_TrackerCSRT_Params(), val) };
			ret
		}

		#[inline]
		fn set_use_gray(&mut self, val: bool) {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propUse_gray_const_bool(self.as_raw_mut_TrackerCSRT_Params(), val) };
			ret
		}

		#[inline]
		fn set_use_rgb(&mut self, val: bool) {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propUse_rgb_const_bool(self.as_raw_mut_TrackerCSRT_Params(), val) };
			ret
		}

		#[inline]
		fn set_use_channel_weights(&mut self, val: bool) {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propUse_channel_weights_const_bool(self.as_raw_mut_TrackerCSRT_Params(), val) };
			ret
		}

		#[inline]
		fn set_use_segmentation(&mut self, val: bool) {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propUse_segmentation_const_bool(self.as_raw_mut_TrackerCSRT_Params(), val) };
			ret
		}

		/// Window function: "hann", "cheb", "kaiser"
		#[inline]
		fn set_window_function(&mut self, val: &str) {
			extern_container_arg!(nofail val);
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propWindow_function_const_string(self.as_raw_mut_TrackerCSRT_Params(), val.opencv_as_extern()) };
			ret
		}

		#[inline]
		fn set_kaiser_alpha(&mut self, val: f32) {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propKaiser_alpha_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
			ret
		}

		#[inline]
		fn set_cheb_attenuation(&mut self, val: f32) {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propCheb_attenuation_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
			ret
		}

		#[inline]
		fn set_template_size(&mut self, val: f32) {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propTemplate_size_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
			ret
		}

		#[inline]
		fn set_gsl_sigma(&mut self, val: f32) {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propGsl_sigma_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
			ret
		}

		#[inline]
		fn set_hog_orientations(&mut self, val: f32) {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propHog_orientations_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
			ret
		}

		#[inline]
		fn set_hog_clip(&mut self, val: f32) {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propHog_clip_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
			ret
		}

		#[inline]
		fn set_padding(&mut self, val: f32) {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propPadding_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
			ret
		}

		#[inline]
		fn set_filter_lr(&mut self, val: f32) {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propFilter_lr_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
			ret
		}

		#[inline]
		fn set_weights_lr(&mut self, val: f32) {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propWeights_lr_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
			ret
		}

		#[inline]
		fn set_num_hog_channels_used(&mut self, val: i32) {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propNum_hog_channels_used_const_int(self.as_raw_mut_TrackerCSRT_Params(), val) };
			ret
		}

		#[inline]
		fn set_admm_iterations(&mut self, val: i32) {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propAdmm_iterations_const_int(self.as_raw_mut_TrackerCSRT_Params(), val) };
			ret
		}

		#[inline]
		fn set_histogram_bins(&mut self, val: i32) {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propHistogram_bins_const_int(self.as_raw_mut_TrackerCSRT_Params(), val) };
			ret
		}

		#[inline]
		fn set_histogram_lr(&mut self, val: f32) {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propHistogram_lr_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
			ret
		}

		#[inline]
		fn set_background_ratio(&mut self, val: i32) {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propBackground_ratio_const_int(self.as_raw_mut_TrackerCSRT_Params(), val) };
			ret
		}

		#[inline]
		fn set_number_of_scales(&mut self, val: i32) {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propNumber_of_scales_const_int(self.as_raw_mut_TrackerCSRT_Params(), val) };
			ret
		}

		#[inline]
		fn set_scale_sigma_factor(&mut self, val: f32) {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propScale_sigma_factor_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
			ret
		}

		#[inline]
		fn set_scale_model_max_area(&mut self, val: f32) {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propScale_model_max_area_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
			ret
		}

		#[inline]
		fn set_scale_lr(&mut self, val: f32) {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propScale_lr_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
			ret
		}

		#[inline]
		fn set_scale_step(&mut self, val: f32) {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propScale_step_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
			ret
		}

		/// we lost the target, if the psr is lower than this.
		#[inline]
		fn set_psr_threshold(&mut self, val: f32) {
			let ret = unsafe { sys::cv_TrackerCSRT_Params_propPsr_threshold_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
			ret
		}

	}

	impl Clone for TrackerCSRT_Params {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_TrackerCSRT_Params_implicitClone_const(self.as_raw_TrackerCSRT_Params())) }
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

	impl crate::tracking::TrackerCSRT_ParamsTraitConst for TrackerCSRT_Params {
		#[inline] fn as_raw_TrackerCSRT_Params(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::TrackerCSRT_ParamsTrait for TrackerCSRT_Params {
		#[inline] fn as_raw_mut_TrackerCSRT_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { TrackerCSRT_Params, crate::tracking::TrackerCSRT_ParamsTraitConst, as_raw_TrackerCSRT_Params, crate::tracking::TrackerCSRT_ParamsTrait, as_raw_mut_TrackerCSRT_Params }

	/// the KCF (Kernelized Correlation Filter) tracker
	///
	/// * KCF is a novel tracking framework that utilizes properties of circulant matrix to enhance the processing speed.
	/// * This tracking method is an implementation of [KCF_ECCV](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_KCF_ECCV) which is extended to KCF with color-names features ([KCF_CN](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_KCF_CN)).
	/// * The original paper of KCF is available at <http://www.robots.ox.ac.uk/~joao/publications/henriques_tpami2015.pdf>
	/// * as well as the matlab implementation. For more information about KCF with color-names features, please refer to
	/// * <http://www.cvl.isy.liu.se/research/objrec/visualtracking/colvistrack/index.html>.
	pub struct TrackerKCF {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { TrackerKCF }

	impl Drop for TrackerKCF {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_TrackerKCF_delete(self.as_raw_mut_TrackerKCF()) };
		}
	}

	unsafe impl Send for TrackerKCF {}

	impl TrackerKCF {
		/// Create KCF tracker instance
		/// ## Parameters
		/// * parameters: KCF parameters TrackerKCF::Params
		///
		/// ## C++ default parameters
		/// * parameters: TrackerKCF::Params()
		#[inline]
		pub fn create(parameters: crate::tracking::TrackerKCF_Params) -> Result<core::Ptr<crate::tracking::TrackerKCF>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_TrackerKCF_create_const_ParamsR(&parameters, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::tracking::TrackerKCF>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_TrackerKCF_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::tracking::TrackerKCF>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::TrackerKCF]
	pub trait TrackerKCFTraitConst: crate::video::TrackerTraitConst {
		fn as_raw_TrackerKCF(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::tracking::TrackerKCF]
	pub trait TrackerKCFTrait: crate::tracking::TrackerKCFTraitConst + crate::video::TrackerTrait {
		fn as_raw_mut_TrackerKCF(&mut self) -> *mut c_void;

		/// ## C++ default parameters
		/// * pca_func: false
		#[inline]
		fn set_feature_extractor(&mut self, callback: crate::tracking::TrackerKCF_FeatureExtractorCallbackFN, pca_func: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_TrackerKCF_setFeatureExtractor_FeatureExtractorCallbackFN_bool(self.as_raw_mut_TrackerKCF(), callback, pca_func, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// ## Note
		/// This alternative version of [TrackerKCFTrait::set_feature_extractor] function uses the following default values for its arguments:
		/// * pca_func: false
		#[inline]
		fn set_feature_extractor_def(&mut self, callback: crate::tracking::TrackerKCF_FeatureExtractorCallbackFN) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_TrackerKCF_setFeatureExtractor_FeatureExtractorCallbackFN(self.as_raw_mut_TrackerKCF(), callback, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for TrackerKCF {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("TrackerKCF")
				.finish()
		}
	}

	boxed_cast_base! { TrackerKCF, crate::video::Tracker, cv_TrackerKCF_to_Tracker }

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
			return_send!(via ocvrs_return);
			unsafe { sys::cv_TrackerKCF_Params_Params(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Abstract base class for TrackerContribFeature that represents the feature.
	pub struct Detail_TrackerContribFeature {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerContribFeature }

	impl Drop for Detail_TrackerContribFeature {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerContribFeature_delete(self.as_raw_mut_Detail_TrackerContribFeature()) };
		}
	}

	unsafe impl Send for Detail_TrackerContribFeature {}

	impl Detail_TrackerContribFeature {
		/// Create TrackerContribFeature by tracker feature type
		/// ## Parameters
		/// * trackerFeatureType: The TrackerContribFeature name
		///
		/// The modes available now:
		///
		/// *   "HAAR" -- Haar Feature-based
		///
		/// The modes that will be available soon:
		///
		/// *   "HOG" -- Histogram of Oriented Gradients features
		/// *   "LBP" -- Local Binary Pattern features
		/// *   "FEATURE2D" -- All types of Feature2D
		#[inline]
		pub fn create(tracker_feature_type: &str) -> Result<core::Ptr<crate::tracking::Detail_TrackerContribFeature>> {
			extern_container_arg!(tracker_feature_type);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribFeature_create_const_StringR(tracker_feature_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::tracking::Detail_TrackerContribFeature>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Detail_TrackerContribFeature]
	pub trait Detail_TrackerContribFeatureTraitConst: crate::tracking::Detail_TrackerFeatureTraitConst {
		fn as_raw_Detail_TrackerContribFeature(&self) -> *const c_void;

		/// Get the name of the specific TrackerContribFeature
		#[inline]
		fn get_class_name(&self) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribFeature_getClassName_const(self.as_raw_Detail_TrackerContribFeature(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerContribFeature]
	pub trait Detail_TrackerContribFeatureTrait: crate::tracking::Detail_TrackerContribFeatureTraitConst + crate::tracking::Detail_TrackerFeatureTrait {
		fn as_raw_mut_Detail_TrackerContribFeature(&mut self) -> *mut c_void;

		/// Identify most effective features
		/// ## Parameters
		/// * response: Collection of response for the specific TrackerContribFeature
		/// * npoints: Max number of features
		///
		///
		/// Note: This method modifies the response parameter
		#[inline]
		fn selection(&mut self, response: &mut impl core::MatTrait, npoints: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribFeature_selection_MatR_int(self.as_raw_mut_Detail_TrackerContribFeature(), response.as_raw_mut_Mat(), npoints, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Detail_TrackerContribFeature {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerContribFeature")
				.finish()
		}
	}

	boxed_cast_base! { Detail_TrackerContribFeature, crate::tracking::Detail_TrackerFeature, cv_detail_TrackerContribFeature_to_Detail_TrackerFeature }

	boxed_cast_descendant! { Detail_TrackerContribFeature, crate::tracking::Detail_TrackerContribFeatureHAAR, cv_detail_TrackerContribFeature_to_Detail_TrackerContribFeatureHAAR }

	boxed_cast_descendant! { Detail_TrackerContribFeature, crate::tracking::Detail_TrackerFeatureFeature2d, cv_detail_TrackerContribFeature_to_Detail_TrackerFeatureFeature2d }

	boxed_cast_descendant! { Detail_TrackerContribFeature, crate::tracking::Detail_TrackerFeatureHOG, cv_detail_TrackerContribFeature_to_Detail_TrackerFeatureHOG }

	boxed_cast_descendant! { Detail_TrackerContribFeature, crate::tracking::Detail_TrackerFeatureLBP, cv_detail_TrackerContribFeature_to_Detail_TrackerFeatureLBP }

	impl crate::tracking::Detail_TrackerFeatureTraitConst for Detail_TrackerContribFeature {
		#[inline] fn as_raw_Detail_TrackerFeature(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerFeatureTrait for Detail_TrackerContribFeature {
		#[inline] fn as_raw_mut_Detail_TrackerFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerContribFeature, crate::tracking::Detail_TrackerFeatureTraitConst, as_raw_Detail_TrackerFeature, crate::tracking::Detail_TrackerFeatureTrait, as_raw_mut_Detail_TrackerFeature }

	impl crate::tracking::Detail_TrackerContribFeatureTraitConst for Detail_TrackerContribFeature {
		#[inline] fn as_raw_Detail_TrackerContribFeature(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerContribFeatureTrait for Detail_TrackerContribFeature {
		#[inline] fn as_raw_mut_Detail_TrackerContribFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerContribFeature, crate::tracking::Detail_TrackerContribFeatureTraitConst, as_raw_Detail_TrackerContribFeature, crate::tracking::Detail_TrackerContribFeatureTrait, as_raw_mut_Detail_TrackerContribFeature }

	/// TrackerContribFeature based on HAAR features, used by TrackerMIL and many others algorithms
	///
	/// Note: HAAR features implementation is copied from apps/traincascade and modified according to MIL
	pub struct Detail_TrackerContribFeatureHAAR {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerContribFeatureHAAR }

	impl Drop for Detail_TrackerContribFeatureHAAR {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerContribFeatureHAAR_delete(self.as_raw_mut_Detail_TrackerContribFeatureHAAR()) };
		}
	}

	unsafe impl Send for Detail_TrackerContribFeatureHAAR {}

	impl Detail_TrackerContribFeatureHAAR {
		/// Constructor
		/// ## Parameters
		/// * parameters: TrackerContribFeatureHAAR parameters TrackerContribFeatureHAAR::Params
		///
		/// ## C++ default parameters
		/// * parameters: TrackerContribFeatureHAAR::Params()
		#[inline]
		pub fn new(parameters: &impl crate::tracking::Detail_TrackerContribFeatureHAAR_ParamsTraitConst) -> Result<crate::tracking::Detail_TrackerContribFeatureHAAR> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribFeatureHAAR_TrackerContribFeatureHAAR_const_ParamsR(parameters.as_raw_Detail_TrackerContribFeatureHAAR_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Detail_TrackerContribFeatureHAAR::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Constructor
		/// ## Parameters
		/// * parameters: TrackerContribFeatureHAAR parameters TrackerContribFeatureHAAR::Params
		///
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * parameters: TrackerContribFeatureHAAR::Params()
		#[inline]
		pub fn new_def() -> Result<crate::tracking::Detail_TrackerContribFeatureHAAR> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribFeatureHAAR_TrackerContribFeatureHAAR(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Detail_TrackerContribFeatureHAAR::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Detail_TrackerContribFeatureHAAR]
	pub trait Detail_TrackerContribFeatureHAARTraitConst: crate::tracking::Detail_TrackerContribFeatureTraitConst {
		fn as_raw_Detail_TrackerContribFeatureHAAR(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerContribFeatureHAAR]
	pub trait Detail_TrackerContribFeatureHAARTrait: crate::tracking::Detail_TrackerContribFeatureHAARTraitConst + crate::tracking::Detail_TrackerContribFeatureTrait {
		fn as_raw_mut_Detail_TrackerContribFeatureHAAR(&mut self) -> *mut c_void;

		/// Compute the features only for the selected indices in the images collection
		/// ## Parameters
		/// * selFeatures: indices of selected features
		/// * images: The images
		/// * response: Collection of response for the specific TrackerContribFeature
		#[inline]
		fn extract_selected(&mut self, sel_features: core::Vector<i32>, images: &core::Vector<core::Mat>, response: &mut impl core::MatTrait) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribFeatureHAAR_extractSelected_const_vectorLintG_const_vectorLMatGR_MatR(self.as_raw_mut_Detail_TrackerContribFeatureHAAR(), sel_features.as_raw_VectorOfi32(), images.as_raw_VectorOfMat(), response.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Identify most effective features
		/// ## Parameters
		/// * response: Collection of response for the specific TrackerContribFeature
		/// * npoints: Max number of features
		///
		///
		/// Note: This method modifies the response parameter
		#[inline]
		fn selection(&mut self, response: &mut impl core::MatTrait, npoints: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribFeatureHAAR_selection_MatR_int(self.as_raw_mut_Detail_TrackerContribFeatureHAAR(), response.as_raw_mut_Mat(), npoints, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Swap the feature in position source with the feature in position target
		/// ## Parameters
		/// * source: The source position
		/// * target: The target position
		#[inline]
		fn swap_feature(&mut self, source: i32, target: i32) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribFeatureHAAR_swapFeature_int_int(self.as_raw_mut_Detail_TrackerContribFeatureHAAR(), source, target, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Detail_TrackerContribFeatureHAAR {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerContribFeatureHAAR")
				.finish()
		}
	}

	boxed_cast_base! { Detail_TrackerContribFeatureHAAR, crate::tracking::Detail_TrackerContribFeature, cv_detail_TrackerContribFeatureHAAR_to_Detail_TrackerContribFeature }

	boxed_cast_base! { Detail_TrackerContribFeatureHAAR, crate::tracking::Detail_TrackerFeature, cv_detail_TrackerContribFeatureHAAR_to_Detail_TrackerFeature }

	impl crate::tracking::Detail_TrackerContribFeatureTraitConst for Detail_TrackerContribFeatureHAAR {
		#[inline] fn as_raw_Detail_TrackerContribFeature(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerContribFeatureTrait for Detail_TrackerContribFeatureHAAR {
		#[inline] fn as_raw_mut_Detail_TrackerContribFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerContribFeatureHAAR, crate::tracking::Detail_TrackerContribFeatureTraitConst, as_raw_Detail_TrackerContribFeature, crate::tracking::Detail_TrackerContribFeatureTrait, as_raw_mut_Detail_TrackerContribFeature }

	impl crate::tracking::Detail_TrackerFeatureTraitConst for Detail_TrackerContribFeatureHAAR {
		#[inline] fn as_raw_Detail_TrackerFeature(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerFeatureTrait for Detail_TrackerContribFeatureHAAR {
		#[inline] fn as_raw_mut_Detail_TrackerFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerContribFeatureHAAR, crate::tracking::Detail_TrackerFeatureTraitConst, as_raw_Detail_TrackerFeature, crate::tracking::Detail_TrackerFeatureTrait, as_raw_mut_Detail_TrackerFeature }

	impl crate::tracking::Detail_TrackerContribFeatureHAARTraitConst for Detail_TrackerContribFeatureHAAR {
		#[inline] fn as_raw_Detail_TrackerContribFeatureHAAR(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerContribFeatureHAARTrait for Detail_TrackerContribFeatureHAAR {
		#[inline] fn as_raw_mut_Detail_TrackerContribFeatureHAAR(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerContribFeatureHAAR, crate::tracking::Detail_TrackerContribFeatureHAARTraitConst, as_raw_Detail_TrackerContribFeatureHAAR, crate::tracking::Detail_TrackerContribFeatureHAARTrait, as_raw_mut_Detail_TrackerContribFeatureHAAR }

	pub struct Detail_TrackerContribFeatureHAAR_Params {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerContribFeatureHAAR_Params }

	impl Drop for Detail_TrackerContribFeatureHAAR_Params {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerContribFeatureHAAR_Params_delete(self.as_raw_mut_Detail_TrackerContribFeatureHAAR_Params()) };
		}
	}

	unsafe impl Send for Detail_TrackerContribFeatureHAAR_Params {}

	impl Detail_TrackerContribFeatureHAAR_Params {
		#[inline]
		pub fn default() -> Result<crate::tracking::Detail_TrackerContribFeatureHAAR_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribFeatureHAAR_Params_Params(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Detail_TrackerContribFeatureHAAR_Params::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Detail_TrackerContribFeatureHAAR_Params]
	pub trait Detail_TrackerContribFeatureHAAR_ParamsTraitConst {
		fn as_raw_Detail_TrackerContribFeatureHAAR_Params(&self) -> *const c_void;

		/// # of rects
		#[inline]
		fn num_features(&self) -> i32 {
			let ret = unsafe { sys::cv_detail_TrackerContribFeatureHAAR_Params_propNumFeatures_const(self.as_raw_Detail_TrackerContribFeatureHAAR_Params()) };
			ret
		}

		/// rect size
		#[inline]
		fn rect_size(&self) -> core::Size {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribFeatureHAAR_Params_propRectSize_const(self.as_raw_Detail_TrackerContribFeatureHAAR_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			ret
		}

		/// true if input images are integral, false otherwise
		#[inline]
		fn is_integral(&self) -> bool {
			let ret = unsafe { sys::cv_detail_TrackerContribFeatureHAAR_Params_propIsIntegral_const(self.as_raw_Detail_TrackerContribFeatureHAAR_Params()) };
			ret
		}

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerContribFeatureHAAR_Params]
	pub trait Detail_TrackerContribFeatureHAAR_ParamsTrait: crate::tracking::Detail_TrackerContribFeatureHAAR_ParamsTraitConst {
		fn as_raw_mut_Detail_TrackerContribFeatureHAAR_Params(&mut self) -> *mut c_void;

		/// # of rects
		#[inline]
		fn set_num_features(&mut self, val: i32) {
			let ret = unsafe { sys::cv_detail_TrackerContribFeatureHAAR_Params_propNumFeatures_const_int(self.as_raw_mut_Detail_TrackerContribFeatureHAAR_Params(), val) };
			ret
		}

		/// rect size
		#[inline]
		fn set_rect_size(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_detail_TrackerContribFeatureHAAR_Params_propRectSize_const_Size(self.as_raw_mut_Detail_TrackerContribFeatureHAAR_Params(), &val) };
			ret
		}

		/// true if input images are integral, false otherwise
		#[inline]
		fn set_is_integral(&mut self, val: bool) {
			let ret = unsafe { sys::cv_detail_TrackerContribFeatureHAAR_Params_propIsIntegral_const_bool(self.as_raw_mut_Detail_TrackerContribFeatureHAAR_Params(), val) };
			ret
		}

	}

	impl std::fmt::Debug for Detail_TrackerContribFeatureHAAR_Params {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerContribFeatureHAAR_Params")
				.field("num_features", &crate::tracking::Detail_TrackerContribFeatureHAAR_ParamsTraitConst::num_features(self))
				.field("rect_size", &crate::tracking::Detail_TrackerContribFeatureHAAR_ParamsTraitConst::rect_size(self))
				.field("is_integral", &crate::tracking::Detail_TrackerContribFeatureHAAR_ParamsTraitConst::is_integral(self))
				.finish()
		}
	}

	impl crate::tracking::Detail_TrackerContribFeatureHAAR_ParamsTraitConst for Detail_TrackerContribFeatureHAAR_Params {
		#[inline] fn as_raw_Detail_TrackerContribFeatureHAAR_Params(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerContribFeatureHAAR_ParamsTrait for Detail_TrackerContribFeatureHAAR_Params {
		#[inline] fn as_raw_mut_Detail_TrackerContribFeatureHAAR_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerContribFeatureHAAR_Params, crate::tracking::Detail_TrackerContribFeatureHAAR_ParamsTraitConst, as_raw_Detail_TrackerContribFeatureHAAR_Params, crate::tracking::Detail_TrackerContribFeatureHAAR_ParamsTrait, as_raw_mut_Detail_TrackerContribFeatureHAAR_Params }

	/// Class that manages the extraction and selection of features
	///
	/// [AAM](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_AAM) Feature Extraction and Feature Set Refinement (Feature Processing and Feature Selection).
	/// See table I and section III C [AMVOT](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_AMVOT) Appearance modelling -\> Visual representation (Table II,
	/// section 3.1 - 3.2)
	///
	/// TrackerContribFeatureSet is an aggregation of TrackerContribFeature
	/// ## See also
	/// TrackerContribFeature
	pub struct Detail_TrackerContribFeatureSet {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerContribFeatureSet }

	impl Drop for Detail_TrackerContribFeatureSet {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerContribFeatureSet_delete(self.as_raw_mut_Detail_TrackerContribFeatureSet()) };
		}
	}

	unsafe impl Send for Detail_TrackerContribFeatureSet {}

	impl Detail_TrackerContribFeatureSet {
		#[inline]
		pub fn default() -> Result<crate::tracking::Detail_TrackerContribFeatureSet> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribFeatureSet_TrackerContribFeatureSet(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Detail_TrackerContribFeatureSet::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Detail_TrackerContribFeatureSet]
	pub trait Detail_TrackerContribFeatureSetTraitConst {
		fn as_raw_Detail_TrackerContribFeatureSet(&self) -> *const c_void;

		/// Get the TrackerContribFeature collection (TrackerContribFeature name, TrackerContribFeature pointer)
		#[inline]
		fn get_tracker_feature(&self) -> Result<core::Vector<core::Tuple<(String, core::Ptr<crate::tracking::Detail_TrackerContribFeature>)>>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribFeatureSet_getTrackerFeature_const(self.as_raw_Detail_TrackerContribFeatureSet(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Tuple<(String, core::Ptr<crate::tracking::Detail_TrackerContribFeature>)>>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Get the responses
		///
		///
		/// Note: Be sure to call extraction before getResponses Example TrackerContribFeatureSet::getResponses : :
		#[inline]
		fn get_responses(&self) -> Result<core::Vector<core::Mat>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribFeatureSet_getResponses_const(self.as_raw_Detail_TrackerContribFeatureSet(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerContribFeatureSet]
	pub trait Detail_TrackerContribFeatureSetTrait: crate::tracking::Detail_TrackerContribFeatureSetTraitConst {
		fn as_raw_mut_Detail_TrackerContribFeatureSet(&mut self) -> *mut c_void;

		/// Extract features from the images collection
		/// ## Parameters
		/// * images: The input images
		#[inline]
		fn extraction(&mut self, images: &core::Vector<core::Mat>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribFeatureSet_extraction_const_vectorLMatGR(self.as_raw_mut_Detail_TrackerContribFeatureSet(), images.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Identify most effective features for all feature types (optional)
		#[inline]
		fn selection(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribFeatureSet_selection(self.as_raw_mut_Detail_TrackerContribFeatureSet(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Remove outliers for all feature types (optional)
		#[inline]
		fn remove_outliers(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribFeatureSet_removeOutliers(self.as_raw_mut_Detail_TrackerContribFeatureSet(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Add TrackerContribFeature in the collection. Return true if TrackerContribFeature is added, false otherwise
		/// ## Parameters
		/// * trackerFeatureType: The TrackerContribFeature name
		///
		/// The modes available now:
		///
		/// *   "HAAR" -- Haar Feature-based
		///
		/// The modes that will be available soon:
		///
		/// *   "HOG" -- Histogram of Oriented Gradients features
		/// *   "LBP" -- Local Binary Pattern features
		/// *   "FEATURE2D" -- All types of Feature2D
		///
		/// Example TrackerContribFeatureSet::addTrackerFeature : :
		/// ```C++
		///    //sample usage:
		///
		///    Ptr<TrackerContribFeature> trackerFeature = ...;
		///    featureSet->addTrackerFeature( trackerFeature );
		///
		///    //or add CSC sampler with default parameters
		///    //featureSet->addTrackerFeature( "HAAR" );
		/// ```
		///
		///
		/// Note: If you use the second method, you must initialize the TrackerContribFeature
		#[inline]
		fn add_tracker_feature(&mut self, tracker_feature_type: &str) -> Result<bool> {
			extern_container_arg!(tracker_feature_type);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribFeatureSet_addTrackerFeature_String(self.as_raw_mut_Detail_TrackerContribFeatureSet(), tracker_feature_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Add TrackerContribFeature in the collection. Return true if TrackerContribFeature is added, false otherwise
		/// ## Parameters
		/// * trackerFeatureType: The TrackerContribFeature name
		///
		/// The modes available now:
		///
		/// *   "HAAR" -- Haar Feature-based
		///
		/// The modes that will be available soon:
		///
		/// *   "HOG" -- Histogram of Oriented Gradients features
		/// *   "LBP" -- Local Binary Pattern features
		/// *   "FEATURE2D" -- All types of Feature2D
		///
		/// Example TrackerContribFeatureSet::addTrackerFeature : :
		/// ```C++
		///    //sample usage:
		///
		///    Ptr<TrackerContribFeature> trackerFeature = ...;
		///    featureSet->addTrackerFeature( trackerFeature );
		///
		///    //or add CSC sampler with default parameters
		///    //featureSet->addTrackerFeature( "HAAR" );
		/// ```
		///
		///
		/// Note: If you use the second method, you must initialize the TrackerContribFeature
		///
		/// ## Overloaded parameters
		///
		/// * feature: The TrackerContribFeature class
		#[inline]
		fn add_tracker_feature_1(&mut self, feature: &mut core::Ptr<crate::tracking::Detail_TrackerContribFeature>) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribFeatureSet_addTrackerFeature_PtrLTrackerContribFeatureGR(self.as_raw_mut_Detail_TrackerContribFeatureSet(), feature.as_raw_mut_PtrOfDetail_TrackerContribFeature(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Detail_TrackerContribFeatureSet {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerContribFeatureSet")
				.finish()
		}
	}

	impl crate::tracking::Detail_TrackerContribFeatureSetTraitConst for Detail_TrackerContribFeatureSet {
		#[inline] fn as_raw_Detail_TrackerContribFeatureSet(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerContribFeatureSetTrait for Detail_TrackerContribFeatureSet {
		#[inline] fn as_raw_mut_Detail_TrackerContribFeatureSet(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerContribFeatureSet, crate::tracking::Detail_TrackerContribFeatureSetTraitConst, as_raw_Detail_TrackerContribFeatureSet, crate::tracking::Detail_TrackerContribFeatureSetTrait, as_raw_mut_Detail_TrackerContribFeatureSet }

	/// Class that manages the sampler in order to select regions for the update the model of the tracker
	///
	/// [AAM](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_AAM) Sampling e Labeling. See table I and section III B
	///
	/// TrackerContribSampler is an aggregation of TrackerContribSamplerAlgorithm
	/// ## See also
	/// TrackerContribSamplerAlgorithm
	pub struct Detail_TrackerContribSampler {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerContribSampler }

	impl Drop for Detail_TrackerContribSampler {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerContribSampler_delete(self.as_raw_mut_Detail_TrackerContribSampler()) };
		}
	}

	unsafe impl Send for Detail_TrackerContribSampler {}

	impl Detail_TrackerContribSampler {
		/// \brief Constructor
		#[inline]
		pub fn default() -> Result<crate::tracking::Detail_TrackerContribSampler> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribSampler_TrackerContribSampler(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Detail_TrackerContribSampler::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Detail_TrackerContribSampler]
	pub trait Detail_TrackerContribSamplerTraitConst {
		fn as_raw_Detail_TrackerContribSampler(&self) -> *const c_void;

		/// Return the collection of the TrackerContribSamplerAlgorithm
		#[inline]
		fn get_samplers(&self) -> Result<core::Vector<core::Tuple<(String, core::Ptr<crate::tracking::Detail_TrackerContribSamplerAlgorithm>)>>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribSampler_getSamplers_const(self.as_raw_Detail_TrackerContribSampler(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Tuple<(String, core::Ptr<crate::tracking::Detail_TrackerContribSamplerAlgorithm>)>>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Return the samples from all TrackerContribSamplerAlgorithm, [AAM](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_AAM) Fig. 1 variable Sk
		#[inline]
		fn get_samples(&self) -> Result<core::Vector<core::Mat>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribSampler_getSamples_const(self.as_raw_Detail_TrackerContribSampler(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerContribSampler]
	pub trait Detail_TrackerContribSamplerTrait: crate::tracking::Detail_TrackerContribSamplerTraitConst {
		fn as_raw_mut_Detail_TrackerContribSampler(&mut self) -> *mut c_void;

		/// Computes the regions starting from a position in an image
		/// ## Parameters
		/// * image: The current frame
		/// * boundingBox: The bounding box from which regions can be calculated
		#[inline]
		fn sampling(&mut self, image: &impl core::MatTraitConst, bounding_box: core::Rect) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribSampler_sampling_const_MatR_Rect(self.as_raw_mut_Detail_TrackerContribSampler(), image.as_raw_Mat(), &bounding_box, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Add TrackerContribSamplerAlgorithm in the collection. Return true if sampler is added, false otherwise
		/// ## Parameters
		/// * trackerSamplerAlgorithmType: The TrackerContribSamplerAlgorithm name
		///
		/// The modes available now:
		/// *   "CSC" -- Current State Center
		/// *   "CS" -- Current State
		/// *   "PF" -- Particle Filtering
		///
		/// Example TrackerContribSamplerAlgorithm::addTrackerContribSamplerAlgorithm : :
		/// ```C++
		///      TrackerContribSamplerCSC::Params CSCparameters;
		///      Ptr<TrackerContribSamplerAlgorithm> CSCSampler = new TrackerContribSamplerCSC( CSCparameters );
		///
		///      if( !sampler->addTrackerSamplerAlgorithm( CSCSampler ) )
		///        return false;
		///
		///      //or add CSC sampler with default parameters
		///      //sampler->addTrackerSamplerAlgorithm( "CSC" );
		/// ```
		///
		///
		/// Note: If you use the second method, you must initialize the TrackerContribSamplerAlgorithm
		#[inline]
		fn add_tracker_sampler_algorithm(&mut self, tracker_sampler_algorithm_type: &str) -> Result<bool> {
			extern_container_arg!(tracker_sampler_algorithm_type);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribSampler_addTrackerSamplerAlgorithm_String(self.as_raw_mut_Detail_TrackerContribSampler(), tracker_sampler_algorithm_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Add TrackerContribSamplerAlgorithm in the collection. Return true if sampler is added, false otherwise
		/// ## Parameters
		/// * trackerSamplerAlgorithmType: The TrackerContribSamplerAlgorithm name
		///
		/// The modes available now:
		/// *   "CSC" -- Current State Center
		/// *   "CS" -- Current State
		/// *   "PF" -- Particle Filtering
		///
		/// Example TrackerContribSamplerAlgorithm::addTrackerContribSamplerAlgorithm : :
		/// ```C++
		///      TrackerContribSamplerCSC::Params CSCparameters;
		///      Ptr<TrackerContribSamplerAlgorithm> CSCSampler = new TrackerContribSamplerCSC( CSCparameters );
		///
		///      if( !sampler->addTrackerSamplerAlgorithm( CSCSampler ) )
		///        return false;
		///
		///      //or add CSC sampler with default parameters
		///      //sampler->addTrackerSamplerAlgorithm( "CSC" );
		/// ```
		///
		///
		/// Note: If you use the second method, you must initialize the TrackerContribSamplerAlgorithm
		///
		/// ## Overloaded parameters
		///
		/// * sampler: The TrackerContribSamplerAlgorithm
		#[inline]
		fn add_tracker_sampler_algorithm_1(&mut self, sampler: &mut core::Ptr<crate::tracking::Detail_TrackerContribSamplerAlgorithm>) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribSampler_addTrackerSamplerAlgorithm_PtrLTrackerContribSamplerAlgorithmGR(self.as_raw_mut_Detail_TrackerContribSampler(), sampler.as_raw_mut_PtrOfDetail_TrackerContribSamplerAlgorithm(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Detail_TrackerContribSampler {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerContribSampler")
				.finish()
		}
	}

	impl crate::tracking::Detail_TrackerContribSamplerTraitConst for Detail_TrackerContribSampler {
		#[inline] fn as_raw_Detail_TrackerContribSampler(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerContribSamplerTrait for Detail_TrackerContribSampler {
		#[inline] fn as_raw_mut_Detail_TrackerContribSampler(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerContribSampler, crate::tracking::Detail_TrackerContribSamplerTraitConst, as_raw_Detail_TrackerContribSampler, crate::tracking::Detail_TrackerContribSamplerTrait, as_raw_mut_Detail_TrackerContribSampler }

	/// Abstract base class for TrackerContribSamplerAlgorithm that represents the algorithm for the specific
	/// sampler.
	pub struct Detail_TrackerContribSamplerAlgorithm {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerContribSamplerAlgorithm }

	impl Drop for Detail_TrackerContribSamplerAlgorithm {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerContribSamplerAlgorithm_delete(self.as_raw_mut_Detail_TrackerContribSamplerAlgorithm()) };
		}
	}

	unsafe impl Send for Detail_TrackerContribSamplerAlgorithm {}

	impl Detail_TrackerContribSamplerAlgorithm {
		/// Create TrackerContribSamplerAlgorithm by tracker sampler type.
		/// ## Parameters
		/// * trackerSamplerType: The trackerSamplerType name
		///
		/// The modes available now:
		///
		/// *   "CSC" -- Current State Center
		/// *   "CS" -- Current State
		#[inline]
		pub fn create(tracker_sampler_type: &str) -> Result<core::Ptr<crate::tracking::Detail_TrackerContribSamplerAlgorithm>> {
			extern_container_arg!(tracker_sampler_type);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribSamplerAlgorithm_create_const_StringR(tracker_sampler_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::tracking::Detail_TrackerContribSamplerAlgorithm>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Detail_TrackerContribSamplerAlgorithm]
	pub trait Detail_TrackerContribSamplerAlgorithmTraitConst: crate::tracking::Detail_TrackerSamplerAlgorithmTraitConst {
		fn as_raw_Detail_TrackerContribSamplerAlgorithm(&self) -> *const c_void;

		/// Get the name of the specific TrackerContribSamplerAlgorithm
		#[inline]
		fn get_class_name(&self) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribSamplerAlgorithm_getClassName_const(self.as_raw_Detail_TrackerContribSamplerAlgorithm(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerContribSamplerAlgorithm]
	pub trait Detail_TrackerContribSamplerAlgorithmTrait: crate::tracking::Detail_TrackerContribSamplerAlgorithmTraitConst + crate::tracking::Detail_TrackerSamplerAlgorithmTrait {
		fn as_raw_mut_Detail_TrackerContribSamplerAlgorithm(&mut self) -> *mut c_void;

		/// Computes the regions starting from a position in an image.
		///
		/// Return true if samples are computed, false otherwise
		///
		/// ## Parameters
		/// * image: The current frame
		/// * boundingBox: The bounding box from which regions can be calculated
		///
		/// * sample: The computed samples [AAM](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_AAM) Fig. 1 variable Sk
		#[inline]
		fn sampling(&mut self, image: &impl core::MatTraitConst, bounding_box: core::Rect, sample: &mut core::Vector<core::Mat>) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribSamplerAlgorithm_sampling_const_MatR_const_RectR_vectorLMatGR(self.as_raw_mut_Detail_TrackerContribSamplerAlgorithm(), image.as_raw_Mat(), &bounding_box, sample.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Detail_TrackerContribSamplerAlgorithm {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerContribSamplerAlgorithm")
				.finish()
		}
	}

	boxed_cast_base! { Detail_TrackerContribSamplerAlgorithm, crate::tracking::Detail_TrackerSamplerAlgorithm, cv_detail_TrackerContribSamplerAlgorithm_to_Detail_TrackerSamplerAlgorithm }

	boxed_cast_descendant! { Detail_TrackerContribSamplerAlgorithm, crate::tracking::Detail_TrackerContribSamplerCSC, cv_detail_TrackerContribSamplerAlgorithm_to_Detail_TrackerContribSamplerCSC }

	boxed_cast_descendant! { Detail_TrackerContribSamplerAlgorithm, crate::tracking::Detail_TrackerSamplerCS, cv_detail_TrackerContribSamplerAlgorithm_to_Detail_TrackerSamplerCS }

	boxed_cast_descendant! { Detail_TrackerContribSamplerAlgorithm, crate::tracking::Detail_TrackerSamplerPF, cv_detail_TrackerContribSamplerAlgorithm_to_Detail_TrackerSamplerPF }

	impl crate::tracking::Detail_TrackerSamplerAlgorithmTraitConst for Detail_TrackerContribSamplerAlgorithm {
		#[inline] fn as_raw_Detail_TrackerSamplerAlgorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerSamplerAlgorithmTrait for Detail_TrackerContribSamplerAlgorithm {
		#[inline] fn as_raw_mut_Detail_TrackerSamplerAlgorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerContribSamplerAlgorithm, crate::tracking::Detail_TrackerSamplerAlgorithmTraitConst, as_raw_Detail_TrackerSamplerAlgorithm, crate::tracking::Detail_TrackerSamplerAlgorithmTrait, as_raw_mut_Detail_TrackerSamplerAlgorithm }

	impl crate::tracking::Detail_TrackerContribSamplerAlgorithmTraitConst for Detail_TrackerContribSamplerAlgorithm {
		#[inline] fn as_raw_Detail_TrackerContribSamplerAlgorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerContribSamplerAlgorithmTrait for Detail_TrackerContribSamplerAlgorithm {
		#[inline] fn as_raw_mut_Detail_TrackerContribSamplerAlgorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerContribSamplerAlgorithm, crate::tracking::Detail_TrackerContribSamplerAlgorithmTraitConst, as_raw_Detail_TrackerContribSamplerAlgorithm, crate::tracking::Detail_TrackerContribSamplerAlgorithmTrait, as_raw_mut_Detail_TrackerContribSamplerAlgorithm }

	/// TrackerSampler based on CSC (current state centered), used by MIL algorithm TrackerMIL
	pub struct Detail_TrackerContribSamplerCSC {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerContribSamplerCSC }

	impl Drop for Detail_TrackerContribSamplerCSC {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerContribSamplerCSC_delete(self.as_raw_mut_Detail_TrackerContribSamplerCSC()) };
		}
	}

	unsafe impl Send for Detail_TrackerContribSamplerCSC {}

	impl Detail_TrackerContribSamplerCSC {
		/// Constructor
		/// ## Parameters
		/// * parameters: TrackerContribSamplerCSC parameters TrackerContribSamplerCSC::Params
		///
		/// ## C++ default parameters
		/// * parameters: TrackerContribSamplerCSC::Params()
		#[inline]
		pub fn new(parameters: &impl crate::tracking::Detail_TrackerContribSamplerCSC_ParamsTraitConst) -> Result<crate::tracking::Detail_TrackerContribSamplerCSC> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribSamplerCSC_TrackerContribSamplerCSC_const_ParamsR(parameters.as_raw_Detail_TrackerContribSamplerCSC_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Detail_TrackerContribSamplerCSC::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Constructor
		/// ## Parameters
		/// * parameters: TrackerContribSamplerCSC parameters TrackerContribSamplerCSC::Params
		///
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * parameters: TrackerContribSamplerCSC::Params()
		#[inline]
		pub fn new_def() -> Result<crate::tracking::Detail_TrackerContribSamplerCSC> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribSamplerCSC_TrackerContribSamplerCSC(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Detail_TrackerContribSamplerCSC::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Detail_TrackerContribSamplerCSC]
	pub trait Detail_TrackerContribSamplerCSCTraitConst: crate::tracking::Detail_TrackerContribSamplerAlgorithmTraitConst {
		fn as_raw_Detail_TrackerContribSamplerCSC(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerContribSamplerCSC]
	pub trait Detail_TrackerContribSamplerCSCTrait: crate::tracking::Detail_TrackerContribSamplerAlgorithmTrait + crate::tracking::Detail_TrackerContribSamplerCSCTraitConst {
		fn as_raw_mut_Detail_TrackerContribSamplerCSC(&mut self) -> *mut c_void;

		/// Set the sampling mode of TrackerContribSamplerCSC
		/// ## Parameters
		/// * samplingMode: The sampling mode
		///
		/// The modes are:
		///
		/// *   "MODE_INIT_POS = 1" -- for the positive sampling in initialization step
		/// *   "MODE_INIT_NEG = 2" -- for the negative sampling in initialization step
		/// *   "MODE_TRACK_POS = 3" -- for the positive sampling in update step
		/// *   "MODE_TRACK_NEG = 4" -- for the negative sampling in update step
		/// *   "MODE_DETECT = 5" -- for the sampling in detection step
		#[inline]
		fn set_mode(&mut self, sampling_mode: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribSamplerCSC_setMode_int(self.as_raw_mut_Detail_TrackerContribSamplerCSC(), sampling_mode, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Detail_TrackerContribSamplerCSC {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerContribSamplerCSC")
				.finish()
		}
	}

	boxed_cast_base! { Detail_TrackerContribSamplerCSC, crate::tracking::Detail_TrackerContribSamplerAlgorithm, cv_detail_TrackerContribSamplerCSC_to_Detail_TrackerContribSamplerAlgorithm }

	boxed_cast_base! { Detail_TrackerContribSamplerCSC, crate::tracking::Detail_TrackerSamplerAlgorithm, cv_detail_TrackerContribSamplerCSC_to_Detail_TrackerSamplerAlgorithm }

	impl crate::tracking::Detail_TrackerContribSamplerAlgorithmTraitConst for Detail_TrackerContribSamplerCSC {
		#[inline] fn as_raw_Detail_TrackerContribSamplerAlgorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerContribSamplerAlgorithmTrait for Detail_TrackerContribSamplerCSC {
		#[inline] fn as_raw_mut_Detail_TrackerContribSamplerAlgorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerContribSamplerCSC, crate::tracking::Detail_TrackerContribSamplerAlgorithmTraitConst, as_raw_Detail_TrackerContribSamplerAlgorithm, crate::tracking::Detail_TrackerContribSamplerAlgorithmTrait, as_raw_mut_Detail_TrackerContribSamplerAlgorithm }

	impl crate::tracking::Detail_TrackerSamplerAlgorithmTraitConst for Detail_TrackerContribSamplerCSC {
		#[inline] fn as_raw_Detail_TrackerSamplerAlgorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerSamplerAlgorithmTrait for Detail_TrackerContribSamplerCSC {
		#[inline] fn as_raw_mut_Detail_TrackerSamplerAlgorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerContribSamplerCSC, crate::tracking::Detail_TrackerSamplerAlgorithmTraitConst, as_raw_Detail_TrackerSamplerAlgorithm, crate::tracking::Detail_TrackerSamplerAlgorithmTrait, as_raw_mut_Detail_TrackerSamplerAlgorithm }

	impl crate::tracking::Detail_TrackerContribSamplerCSCTraitConst for Detail_TrackerContribSamplerCSC {
		#[inline] fn as_raw_Detail_TrackerContribSamplerCSC(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerContribSamplerCSCTrait for Detail_TrackerContribSamplerCSC {
		#[inline] fn as_raw_mut_Detail_TrackerContribSamplerCSC(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerContribSamplerCSC, crate::tracking::Detail_TrackerContribSamplerCSCTraitConst, as_raw_Detail_TrackerContribSamplerCSC, crate::tracking::Detail_TrackerContribSamplerCSCTrait, as_raw_mut_Detail_TrackerContribSamplerCSC }

	pub struct Detail_TrackerContribSamplerCSC_Params {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerContribSamplerCSC_Params }

	impl Drop for Detail_TrackerContribSamplerCSC_Params {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerContribSamplerCSC_Params_delete(self.as_raw_mut_Detail_TrackerContribSamplerCSC_Params()) };
		}
	}

	unsafe impl Send for Detail_TrackerContribSamplerCSC_Params {}

	impl Detail_TrackerContribSamplerCSC_Params {
		#[inline]
		pub fn default() -> Result<crate::tracking::Detail_TrackerContribSamplerCSC_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerContribSamplerCSC_Params_Params(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Detail_TrackerContribSamplerCSC_Params::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Detail_TrackerContribSamplerCSC_Params]
	pub trait Detail_TrackerContribSamplerCSC_ParamsTraitConst {
		fn as_raw_Detail_TrackerContribSamplerCSC_Params(&self) -> *const c_void;

		/// radius for gathering positive instances during init
		#[inline]
		fn init_in_rad(&self) -> f32 {
			let ret = unsafe { sys::cv_detail_TrackerContribSamplerCSC_Params_propInitInRad_const(self.as_raw_Detail_TrackerContribSamplerCSC_Params()) };
			ret
		}

		/// radius for gathering positive instances during tracking
		#[inline]
		fn track_in_pos_rad(&self) -> f32 {
			let ret = unsafe { sys::cv_detail_TrackerContribSamplerCSC_Params_propTrackInPosRad_const(self.as_raw_Detail_TrackerContribSamplerCSC_Params()) };
			ret
		}

		/// size of search window
		#[inline]
		fn search_win_size(&self) -> f32 {
			let ret = unsafe { sys::cv_detail_TrackerContribSamplerCSC_Params_propSearchWinSize_const(self.as_raw_Detail_TrackerContribSamplerCSC_Params()) };
			ret
		}

		/// # negative samples to use during init
		#[inline]
		fn init_max_neg_num(&self) -> i32 {
			let ret = unsafe { sys::cv_detail_TrackerContribSamplerCSC_Params_propInitMaxNegNum_const(self.as_raw_Detail_TrackerContribSamplerCSC_Params()) };
			ret
		}

		/// # positive samples to use during training
		#[inline]
		fn track_max_pos_num(&self) -> i32 {
			let ret = unsafe { sys::cv_detail_TrackerContribSamplerCSC_Params_propTrackMaxPosNum_const(self.as_raw_Detail_TrackerContribSamplerCSC_Params()) };
			ret
		}

		/// # negative samples to use during training
		#[inline]
		fn track_max_neg_num(&self) -> i32 {
			let ret = unsafe { sys::cv_detail_TrackerContribSamplerCSC_Params_propTrackMaxNegNum_const(self.as_raw_Detail_TrackerContribSamplerCSC_Params()) };
			ret
		}

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerContribSamplerCSC_Params]
	pub trait Detail_TrackerContribSamplerCSC_ParamsTrait: crate::tracking::Detail_TrackerContribSamplerCSC_ParamsTraitConst {
		fn as_raw_mut_Detail_TrackerContribSamplerCSC_Params(&mut self) -> *mut c_void;

		/// radius for gathering positive instances during init
		#[inline]
		fn set_init_in_rad(&mut self, val: f32) {
			let ret = unsafe { sys::cv_detail_TrackerContribSamplerCSC_Params_propInitInRad_const_float(self.as_raw_mut_Detail_TrackerContribSamplerCSC_Params(), val) };
			ret
		}

		/// radius for gathering positive instances during tracking
		#[inline]
		fn set_track_in_pos_rad(&mut self, val: f32) {
			let ret = unsafe { sys::cv_detail_TrackerContribSamplerCSC_Params_propTrackInPosRad_const_float(self.as_raw_mut_Detail_TrackerContribSamplerCSC_Params(), val) };
			ret
		}

		/// size of search window
		#[inline]
		fn set_search_win_size(&mut self, val: f32) {
			let ret = unsafe { sys::cv_detail_TrackerContribSamplerCSC_Params_propSearchWinSize_const_float(self.as_raw_mut_Detail_TrackerContribSamplerCSC_Params(), val) };
			ret
		}

		/// # negative samples to use during init
		#[inline]
		fn set_init_max_neg_num(&mut self, val: i32) {
			let ret = unsafe { sys::cv_detail_TrackerContribSamplerCSC_Params_propInitMaxNegNum_const_int(self.as_raw_mut_Detail_TrackerContribSamplerCSC_Params(), val) };
			ret
		}

		/// # positive samples to use during training
		#[inline]
		fn set_track_max_pos_num(&mut self, val: i32) {
			let ret = unsafe { sys::cv_detail_TrackerContribSamplerCSC_Params_propTrackMaxPosNum_const_int(self.as_raw_mut_Detail_TrackerContribSamplerCSC_Params(), val) };
			ret
		}

		/// # negative samples to use during training
		#[inline]
		fn set_track_max_neg_num(&mut self, val: i32) {
			let ret = unsafe { sys::cv_detail_TrackerContribSamplerCSC_Params_propTrackMaxNegNum_const_int(self.as_raw_mut_Detail_TrackerContribSamplerCSC_Params(), val) };
			ret
		}

	}

	impl std::fmt::Debug for Detail_TrackerContribSamplerCSC_Params {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerContribSamplerCSC_Params")
				.field("init_in_rad", &crate::tracking::Detail_TrackerContribSamplerCSC_ParamsTraitConst::init_in_rad(self))
				.field("track_in_pos_rad", &crate::tracking::Detail_TrackerContribSamplerCSC_ParamsTraitConst::track_in_pos_rad(self))
				.field("search_win_size", &crate::tracking::Detail_TrackerContribSamplerCSC_ParamsTraitConst::search_win_size(self))
				.field("init_max_neg_num", &crate::tracking::Detail_TrackerContribSamplerCSC_ParamsTraitConst::init_max_neg_num(self))
				.field("track_max_pos_num", &crate::tracking::Detail_TrackerContribSamplerCSC_ParamsTraitConst::track_max_pos_num(self))
				.field("track_max_neg_num", &crate::tracking::Detail_TrackerContribSamplerCSC_ParamsTraitConst::track_max_neg_num(self))
				.finish()
		}
	}

	impl crate::tracking::Detail_TrackerContribSamplerCSC_ParamsTraitConst for Detail_TrackerContribSamplerCSC_Params {
		#[inline] fn as_raw_Detail_TrackerContribSamplerCSC_Params(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerContribSamplerCSC_ParamsTrait for Detail_TrackerContribSamplerCSC_Params {
		#[inline] fn as_raw_mut_Detail_TrackerContribSamplerCSC_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerContribSamplerCSC_Params, crate::tracking::Detail_TrackerContribSamplerCSC_ParamsTraitConst, as_raw_Detail_TrackerContribSamplerCSC_Params, crate::tracking::Detail_TrackerContribSamplerCSC_ParamsTrait, as_raw_mut_Detail_TrackerContribSamplerCSC_Params }

	/// Abstract base class for TrackerFeature that represents the feature.
	pub struct Detail_TrackerFeature {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerFeature }

	impl Drop for Detail_TrackerFeature {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerFeature_delete(self.as_raw_mut_Detail_TrackerFeature()) };
		}
	}

	unsafe impl Send for Detail_TrackerFeature {}

	/// Constant methods for [crate::tracking::Detail_TrackerFeature]
	pub trait Detail_TrackerFeatureTraitConst {
		fn as_raw_Detail_TrackerFeature(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerFeature]
	pub trait Detail_TrackerFeatureTrait: crate::tracking::Detail_TrackerFeatureTraitConst {
		fn as_raw_mut_Detail_TrackerFeature(&mut self) -> *mut c_void;

		/// Compute the features in the images collection
		/// ## Parameters
		/// * images: The images
		/// * response: The output response
		#[inline]
		fn compute(&mut self, images: &core::Vector<core::Mat>, response: &mut impl core::MatTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerFeature_compute_const_vectorLMatGR_MatR(self.as_raw_mut_Detail_TrackerFeature(), images.as_raw_VectorOfMat(), response.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Detail_TrackerFeature {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerFeature")
				.finish()
		}
	}

	boxed_cast_descendant! { Detail_TrackerFeature, crate::tracking::Detail_TrackerContribFeature, cv_detail_TrackerFeature_to_Detail_TrackerContribFeature }

	boxed_cast_descendant! { Detail_TrackerFeature, crate::tracking::Detail_TrackerContribFeatureHAAR, cv_detail_TrackerFeature_to_Detail_TrackerContribFeatureHAAR }

	boxed_cast_descendant! { Detail_TrackerFeature, crate::tracking::Detail_TrackerFeatureFeature2d, cv_detail_TrackerFeature_to_Detail_TrackerFeatureFeature2d }

	boxed_cast_descendant! { Detail_TrackerFeature, crate::tracking::Detail_TrackerFeatureHOG, cv_detail_TrackerFeature_to_Detail_TrackerFeatureHOG }

	boxed_cast_descendant! { Detail_TrackerFeature, crate::tracking::Detail_TrackerFeatureLBP, cv_detail_TrackerFeature_to_Detail_TrackerFeatureLBP }

	impl crate::tracking::Detail_TrackerFeatureTraitConst for Detail_TrackerFeature {
		#[inline] fn as_raw_Detail_TrackerFeature(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerFeatureTrait for Detail_TrackerFeature {
		#[inline] fn as_raw_mut_Detail_TrackerFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerFeature, crate::tracking::Detail_TrackerFeatureTraitConst, as_raw_Detail_TrackerFeature, crate::tracking::Detail_TrackerFeatureTrait, as_raw_mut_Detail_TrackerFeature }

	/// \brief TrackerContribFeature based on Feature2D
	pub struct Detail_TrackerFeatureFeature2d {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerFeatureFeature2d }

	impl Drop for Detail_TrackerFeatureFeature2d {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerFeatureFeature2d_delete(self.as_raw_mut_Detail_TrackerFeatureFeature2d()) };
		}
	}

	unsafe impl Send for Detail_TrackerFeatureFeature2d {}

	impl Detail_TrackerFeatureFeature2d {
		/// \brief Constructor
		/// \param detectorType string of FeatureDetector
		/// \param descriptorType string of DescriptorExtractor
		#[inline]
		pub fn new(detector_type: &str, descriptor_type: &str) -> Result<crate::tracking::Detail_TrackerFeatureFeature2d> {
			extern_container_arg!(detector_type);
			extern_container_arg!(descriptor_type);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerFeatureFeature2d_TrackerFeatureFeature2d_String_String(detector_type.opencv_as_extern(), descriptor_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Detail_TrackerFeatureFeature2d::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Detail_TrackerFeatureFeature2d]
	pub trait Detail_TrackerFeatureFeature2dTraitConst: crate::tracking::Detail_TrackerContribFeatureTraitConst {
		fn as_raw_Detail_TrackerFeatureFeature2d(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerFeatureFeature2d]
	pub trait Detail_TrackerFeatureFeature2dTrait: crate::tracking::Detail_TrackerContribFeatureTrait + crate::tracking::Detail_TrackerFeatureFeature2dTraitConst {
		fn as_raw_mut_Detail_TrackerFeatureFeature2d(&mut self) -> *mut c_void;

		#[inline]
		fn selection(&mut self, response: &mut impl core::MatTrait, npoints: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerFeatureFeature2d_selection_MatR_int(self.as_raw_mut_Detail_TrackerFeatureFeature2d(), response.as_raw_mut_Mat(), npoints, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Detail_TrackerFeatureFeature2d {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerFeatureFeature2d")
				.finish()
		}
	}

	boxed_cast_base! { Detail_TrackerFeatureFeature2d, crate::tracking::Detail_TrackerContribFeature, cv_detail_TrackerFeatureFeature2d_to_Detail_TrackerContribFeature }

	boxed_cast_base! { Detail_TrackerFeatureFeature2d, crate::tracking::Detail_TrackerFeature, cv_detail_TrackerFeatureFeature2d_to_Detail_TrackerFeature }

	impl crate::tracking::Detail_TrackerContribFeatureTraitConst for Detail_TrackerFeatureFeature2d {
		#[inline] fn as_raw_Detail_TrackerContribFeature(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerContribFeatureTrait for Detail_TrackerFeatureFeature2d {
		#[inline] fn as_raw_mut_Detail_TrackerContribFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerFeatureFeature2d, crate::tracking::Detail_TrackerContribFeatureTraitConst, as_raw_Detail_TrackerContribFeature, crate::tracking::Detail_TrackerContribFeatureTrait, as_raw_mut_Detail_TrackerContribFeature }

	impl crate::tracking::Detail_TrackerFeatureTraitConst for Detail_TrackerFeatureFeature2d {
		#[inline] fn as_raw_Detail_TrackerFeature(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerFeatureTrait for Detail_TrackerFeatureFeature2d {
		#[inline] fn as_raw_mut_Detail_TrackerFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerFeatureFeature2d, crate::tracking::Detail_TrackerFeatureTraitConst, as_raw_Detail_TrackerFeature, crate::tracking::Detail_TrackerFeatureTrait, as_raw_mut_Detail_TrackerFeature }

	impl crate::tracking::Detail_TrackerFeatureFeature2dTraitConst for Detail_TrackerFeatureFeature2d {
		#[inline] fn as_raw_Detail_TrackerFeatureFeature2d(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerFeatureFeature2dTrait for Detail_TrackerFeatureFeature2d {
		#[inline] fn as_raw_mut_Detail_TrackerFeatureFeature2d(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerFeatureFeature2d, crate::tracking::Detail_TrackerFeatureFeature2dTraitConst, as_raw_Detail_TrackerFeatureFeature2d, crate::tracking::Detail_TrackerFeatureFeature2dTrait, as_raw_mut_Detail_TrackerFeatureFeature2d }

	/// \brief TrackerContribFeature based on HOG
	pub struct Detail_TrackerFeatureHOG {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerFeatureHOG }

	impl Drop for Detail_TrackerFeatureHOG {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerFeatureHOG_delete(self.as_raw_mut_Detail_TrackerFeatureHOG()) };
		}
	}

	unsafe impl Send for Detail_TrackerFeatureHOG {}

	impl Detail_TrackerFeatureHOG {
		#[inline]
		pub fn default() -> Result<crate::tracking::Detail_TrackerFeatureHOG> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerFeatureHOG_TrackerFeatureHOG(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Detail_TrackerFeatureHOG::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Detail_TrackerFeatureHOG]
	pub trait Detail_TrackerFeatureHOGTraitConst: crate::tracking::Detail_TrackerContribFeatureTraitConst {
		fn as_raw_Detail_TrackerFeatureHOG(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerFeatureHOG]
	pub trait Detail_TrackerFeatureHOGTrait: crate::tracking::Detail_TrackerContribFeatureTrait + crate::tracking::Detail_TrackerFeatureHOGTraitConst {
		fn as_raw_mut_Detail_TrackerFeatureHOG(&mut self) -> *mut c_void;

		#[inline]
		fn selection(&mut self, response: &mut impl core::MatTrait, npoints: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerFeatureHOG_selection_MatR_int(self.as_raw_mut_Detail_TrackerFeatureHOG(), response.as_raw_mut_Mat(), npoints, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Detail_TrackerFeatureHOG {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerFeatureHOG")
				.finish()
		}
	}

	boxed_cast_base! { Detail_TrackerFeatureHOG, crate::tracking::Detail_TrackerContribFeature, cv_detail_TrackerFeatureHOG_to_Detail_TrackerContribFeature }

	boxed_cast_base! { Detail_TrackerFeatureHOG, crate::tracking::Detail_TrackerFeature, cv_detail_TrackerFeatureHOG_to_Detail_TrackerFeature }

	impl crate::tracking::Detail_TrackerContribFeatureTraitConst for Detail_TrackerFeatureHOG {
		#[inline] fn as_raw_Detail_TrackerContribFeature(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerContribFeatureTrait for Detail_TrackerFeatureHOG {
		#[inline] fn as_raw_mut_Detail_TrackerContribFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerFeatureHOG, crate::tracking::Detail_TrackerContribFeatureTraitConst, as_raw_Detail_TrackerContribFeature, crate::tracking::Detail_TrackerContribFeatureTrait, as_raw_mut_Detail_TrackerContribFeature }

	impl crate::tracking::Detail_TrackerFeatureTraitConst for Detail_TrackerFeatureHOG {
		#[inline] fn as_raw_Detail_TrackerFeature(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerFeatureTrait for Detail_TrackerFeatureHOG {
		#[inline] fn as_raw_mut_Detail_TrackerFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerFeatureHOG, crate::tracking::Detail_TrackerFeatureTraitConst, as_raw_Detail_TrackerFeature, crate::tracking::Detail_TrackerFeatureTrait, as_raw_mut_Detail_TrackerFeature }

	impl crate::tracking::Detail_TrackerFeatureHOGTraitConst for Detail_TrackerFeatureHOG {
		#[inline] fn as_raw_Detail_TrackerFeatureHOG(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerFeatureHOGTrait for Detail_TrackerFeatureHOG {
		#[inline] fn as_raw_mut_Detail_TrackerFeatureHOG(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerFeatureHOG, crate::tracking::Detail_TrackerFeatureHOGTraitConst, as_raw_Detail_TrackerFeatureHOG, crate::tracking::Detail_TrackerFeatureHOGTrait, as_raw_mut_Detail_TrackerFeatureHOG }

	/// \brief TrackerContribFeature based on LBP
	pub struct Detail_TrackerFeatureLBP {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerFeatureLBP }

	impl Drop for Detail_TrackerFeatureLBP {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerFeatureLBP_delete(self.as_raw_mut_Detail_TrackerFeatureLBP()) };
		}
	}

	unsafe impl Send for Detail_TrackerFeatureLBP {}

	impl Detail_TrackerFeatureLBP {
		#[inline]
		pub fn default() -> Result<crate::tracking::Detail_TrackerFeatureLBP> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerFeatureLBP_TrackerFeatureLBP(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Detail_TrackerFeatureLBP::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Detail_TrackerFeatureLBP]
	pub trait Detail_TrackerFeatureLBPTraitConst: crate::tracking::Detail_TrackerContribFeatureTraitConst {
		fn as_raw_Detail_TrackerFeatureLBP(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerFeatureLBP]
	pub trait Detail_TrackerFeatureLBPTrait: crate::tracking::Detail_TrackerContribFeatureTrait + crate::tracking::Detail_TrackerFeatureLBPTraitConst {
		fn as_raw_mut_Detail_TrackerFeatureLBP(&mut self) -> *mut c_void;

		#[inline]
		fn selection(&mut self, response: &mut impl core::MatTrait, npoints: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerFeatureLBP_selection_MatR_int(self.as_raw_mut_Detail_TrackerFeatureLBP(), response.as_raw_mut_Mat(), npoints, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Detail_TrackerFeatureLBP {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerFeatureLBP")
				.finish()
		}
	}

	boxed_cast_base! { Detail_TrackerFeatureLBP, crate::tracking::Detail_TrackerContribFeature, cv_detail_TrackerFeatureLBP_to_Detail_TrackerContribFeature }

	boxed_cast_base! { Detail_TrackerFeatureLBP, crate::tracking::Detail_TrackerFeature, cv_detail_TrackerFeatureLBP_to_Detail_TrackerFeature }

	impl crate::tracking::Detail_TrackerContribFeatureTraitConst for Detail_TrackerFeatureLBP {
		#[inline] fn as_raw_Detail_TrackerContribFeature(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerContribFeatureTrait for Detail_TrackerFeatureLBP {
		#[inline] fn as_raw_mut_Detail_TrackerContribFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerFeatureLBP, crate::tracking::Detail_TrackerContribFeatureTraitConst, as_raw_Detail_TrackerContribFeature, crate::tracking::Detail_TrackerContribFeatureTrait, as_raw_mut_Detail_TrackerContribFeature }

	impl crate::tracking::Detail_TrackerFeatureTraitConst for Detail_TrackerFeatureLBP {
		#[inline] fn as_raw_Detail_TrackerFeature(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerFeatureTrait for Detail_TrackerFeatureLBP {
		#[inline] fn as_raw_mut_Detail_TrackerFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerFeatureLBP, crate::tracking::Detail_TrackerFeatureTraitConst, as_raw_Detail_TrackerFeature, crate::tracking::Detail_TrackerFeatureTrait, as_raw_mut_Detail_TrackerFeature }

	impl crate::tracking::Detail_TrackerFeatureLBPTraitConst for Detail_TrackerFeatureLBP {
		#[inline] fn as_raw_Detail_TrackerFeatureLBP(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerFeatureLBPTrait for Detail_TrackerFeatureLBP {
		#[inline] fn as_raw_mut_Detail_TrackerFeatureLBP(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerFeatureLBP, crate::tracking::Detail_TrackerFeatureLBPTraitConst, as_raw_Detail_TrackerFeatureLBP, crate::tracking::Detail_TrackerFeatureLBPTrait, as_raw_mut_Detail_TrackerFeatureLBP }

	/// Class that manages the extraction and selection of features
	///
	/// [AAM](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_AAM) Feature Extraction and Feature Set Refinement (Feature Processing and Feature Selection).
	/// See table I and section III C [AMVOT](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_AMVOT) Appearance modelling -\> Visual representation (Table II,
	/// section 3.1 - 3.2)
	///
	/// TrackerFeatureSet is an aggregation of TrackerFeature
	/// ## See also
	/// TrackerFeature
	pub struct Detail_TrackerFeatureSet {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerFeatureSet }

	impl Drop for Detail_TrackerFeatureSet {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerFeatureSet_delete(self.as_raw_mut_Detail_TrackerFeatureSet()) };
		}
	}

	unsafe impl Send for Detail_TrackerFeatureSet {}

	impl Detail_TrackerFeatureSet {
		#[inline]
		pub fn default() -> Result<crate::tracking::Detail_TrackerFeatureSet> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerFeatureSet_TrackerFeatureSet(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Detail_TrackerFeatureSet::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Detail_TrackerFeatureSet]
	pub trait Detail_TrackerFeatureSetTraitConst {
		fn as_raw_Detail_TrackerFeatureSet(&self) -> *const c_void;

		/// Get the TrackerFeature collection (TrackerFeature name, TrackerFeature pointer)
		#[inline]
		fn get_tracker_features(&self) -> Result<core::Vector<core::Ptr<crate::tracking::Detail_TrackerFeature>>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerFeatureSet_getTrackerFeatures_const(self.as_raw_Detail_TrackerFeatureSet(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Ptr<crate::tracking::Detail_TrackerFeature>>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Get the responses
		///
		/// Note: Be sure to call extraction before getResponses Example TrackerFeatureSet::getResponses
		#[inline]
		fn get_responses(&self) -> Result<core::Vector<core::Mat>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerFeatureSet_getResponses_const(self.as_raw_Detail_TrackerFeatureSet(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerFeatureSet]
	pub trait Detail_TrackerFeatureSetTrait: crate::tracking::Detail_TrackerFeatureSetTraitConst {
		fn as_raw_mut_Detail_TrackerFeatureSet(&mut self) -> *mut c_void;

		/// Extract features from the images collection
		/// ## Parameters
		/// * images: The input images
		#[inline]
		fn extraction(&mut self, images: &core::Vector<core::Mat>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerFeatureSet_extraction_const_vectorLMatGR(self.as_raw_mut_Detail_TrackerFeatureSet(), images.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Add TrackerFeature in the collection. Return true if TrackerFeature is added, false otherwise
		/// ## Parameters
		/// * feature: The TrackerFeature class
		#[inline]
		fn add_tracker_feature(&mut self, feature: &core::Ptr<crate::tracking::Detail_TrackerFeature>) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerFeatureSet_addTrackerFeature_const_PtrLTrackerFeatureGR(self.as_raw_mut_Detail_TrackerFeatureSet(), feature.as_raw_PtrOfDetail_TrackerFeature(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Detail_TrackerFeatureSet {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerFeatureSet")
				.finish()
		}
	}

	impl crate::tracking::Detail_TrackerFeatureSetTraitConst for Detail_TrackerFeatureSet {
		#[inline] fn as_raw_Detail_TrackerFeatureSet(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerFeatureSetTrait for Detail_TrackerFeatureSet {
		#[inline] fn as_raw_mut_Detail_TrackerFeatureSet(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerFeatureSet, crate::tracking::Detail_TrackerFeatureSetTraitConst, as_raw_Detail_TrackerFeatureSet, crate::tracking::Detail_TrackerFeatureSetTrait, as_raw_mut_Detail_TrackerFeatureSet }

	/// Abstract class that represents the model of the target.
	///
	/// It must be instantiated by specialized tracker
	///
	/// See [AAM](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_AAM) Ak
	///
	/// Inherits this with your TrackerModel
	pub struct Detail_TrackerModel {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerModel }

	impl Drop for Detail_TrackerModel {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerModel_delete(self.as_raw_mut_Detail_TrackerModel()) };
		}
	}

	unsafe impl Send for Detail_TrackerModel {}

	/// Constant methods for [crate::tracking::Detail_TrackerModel]
	pub trait Detail_TrackerModelTraitConst {
		fn as_raw_Detail_TrackerModel(&self) -> *const c_void;

		/// Get the last TrackerTargetState from Trajectory
		#[inline]
		fn get_last_target_state(&self) -> Result<core::Ptr<crate::tracking::Detail_TrackerTargetState>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerModel_getLastTargetState_const(self.as_raw_Detail_TrackerModel(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::tracking::Detail_TrackerTargetState>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Get the list of the ConfidenceMap
		#[inline]
		fn get_confidence_maps(&self) -> Result<core::Vector<crate::tracking::Detail_ConfidenceMap>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerModel_getConfidenceMaps_const(self.as_raw_Detail_TrackerModel(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<crate::tracking::Detail_ConfidenceMap>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Get the last ConfidenceMap for the current frame
		#[inline]
		fn get_last_confidence_map(&self) -> Result<crate::tracking::Detail_ConfidenceMap> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerModel_getLastConfidenceMap_const(self.as_raw_Detail_TrackerModel(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Detail_ConfidenceMap::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Get the TrackerStateEstimator
		#[inline]
		fn get_tracker_state_estimator(&self) -> Result<core::Ptr<crate::tracking::Detail_TrackerStateEstimator>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerModel_getTrackerStateEstimator_const(self.as_raw_Detail_TrackerModel(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::tracking::Detail_TrackerStateEstimator>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerModel]
	pub trait Detail_TrackerModelTrait: crate::tracking::Detail_TrackerModelTraitConst {
		fn as_raw_mut_Detail_TrackerModel(&mut self) -> *mut c_void;

		/// Set TrackerEstimator, return true if the tracker state estimator is added, false otherwise
		/// ## Parameters
		/// * trackerStateEstimator: The TrackerStateEstimator
		///
		/// Note: You can add only one TrackerStateEstimator
		#[inline]
		fn set_tracker_state_estimator(&mut self, mut tracker_state_estimator: core::Ptr<crate::tracking::Detail_TrackerStateEstimator>) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerModel_setTrackerStateEstimator_PtrLTrackerStateEstimatorG(self.as_raw_mut_Detail_TrackerModel(), tracker_state_estimator.as_raw_mut_PtrOfDetail_TrackerStateEstimator(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Estimate the most likely target location
		///
		/// [AAM](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_AAM) ME, Model Estimation table I
		/// ## Parameters
		/// * responses: Features extracted from TrackerFeatureSet
		#[inline]
		fn model_estimation(&mut self, responses: &core::Vector<core::Mat>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerModel_modelEstimation_const_vectorLMatGR(self.as_raw_mut_Detail_TrackerModel(), responses.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Update the model
		///
		/// [AAM](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_AAM) MU, Model Update table I
		#[inline]
		fn model_update(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerModel_modelUpdate(self.as_raw_mut_Detail_TrackerModel(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Run the TrackerStateEstimator, return true if is possible to estimate a new state, false otherwise
		#[inline]
		fn run_state_estimator(&mut self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerModel_runStateEstimator(self.as_raw_mut_Detail_TrackerModel(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set the current TrackerTargetState in the Trajectory
		/// ## Parameters
		/// * lastTargetState: The current TrackerTargetState
		#[inline]
		fn set_last_target_state(&mut self, last_target_state: &core::Ptr<crate::tracking::Detail_TrackerTargetState>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerModel_setLastTargetState_const_PtrLTrackerTargetStateGR(self.as_raw_mut_Detail_TrackerModel(), last_target_state.as_raw_PtrOfDetail_TrackerTargetState(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Detail_TrackerModel {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerModel")
				.finish()
		}
	}

	impl crate::tracking::Detail_TrackerModelTraitConst for Detail_TrackerModel {
		#[inline] fn as_raw_Detail_TrackerModel(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerModelTrait for Detail_TrackerModel {
		#[inline] fn as_raw_mut_Detail_TrackerModel(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerModel, crate::tracking::Detail_TrackerModelTraitConst, as_raw_Detail_TrackerModel, crate::tracking::Detail_TrackerModelTrait, as_raw_mut_Detail_TrackerModel }

	/// Class that manages the sampler in order to select regions for the update the model of the tracker
	///
	/// [AAM](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_AAM) Sampling e Labeling. See table I and section III B
	///
	/// TrackerSampler is an aggregation of TrackerSamplerAlgorithm
	/// ## See also
	/// TrackerSamplerAlgorithm
	pub struct Detail_TrackerSampler {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerSampler }

	impl Drop for Detail_TrackerSampler {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerSampler_delete(self.as_raw_mut_Detail_TrackerSampler()) };
		}
	}

	unsafe impl Send for Detail_TrackerSampler {}

	impl Detail_TrackerSampler {
		#[inline]
		pub fn default() -> Result<crate::tracking::Detail_TrackerSampler> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerSampler_TrackerSampler(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Detail_TrackerSampler::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Detail_TrackerSampler]
	pub trait Detail_TrackerSamplerTraitConst {
		fn as_raw_Detail_TrackerSampler(&self) -> *const c_void;

		/// Return the collection of the TrackerSamplerAlgorithm
		#[inline]
		fn get_samplers(&self) -> Result<core::Vector<core::Ptr<crate::tracking::Detail_TrackerSamplerAlgorithm>>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerSampler_getSamplers_const(self.as_raw_Detail_TrackerSampler(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Ptr<crate::tracking::Detail_TrackerSamplerAlgorithm>>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Return the samples from all TrackerSamplerAlgorithm, [AAM](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_AAM) Fig. 1 variable Sk
		#[inline]
		fn get_samples(&self) -> Result<core::Vector<core::Mat>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerSampler_getSamples_const(self.as_raw_Detail_TrackerSampler(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerSampler]
	pub trait Detail_TrackerSamplerTrait: crate::tracking::Detail_TrackerSamplerTraitConst {
		fn as_raw_mut_Detail_TrackerSampler(&mut self) -> *mut c_void;

		/// Computes the regions starting from a position in an image
		/// ## Parameters
		/// * image: The current frame
		/// * boundingBox: The bounding box from which regions can be calculated
		#[inline]
		fn sampling(&mut self, image: &impl core::MatTraitConst, bounding_box: core::Rect) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerSampler_sampling_const_MatR_Rect(self.as_raw_mut_Detail_TrackerSampler(), image.as_raw_Mat(), &bounding_box, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Add TrackerSamplerAlgorithm in the collection. Return true if sampler is added, false otherwise
		/// ## Parameters
		/// * sampler: The TrackerSamplerAlgorithm
		#[inline]
		fn add_tracker_sampler_algorithm(&mut self, sampler: &core::Ptr<crate::tracking::Detail_TrackerSamplerAlgorithm>) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerSampler_addTrackerSamplerAlgorithm_const_PtrLTrackerSamplerAlgorithmGR(self.as_raw_mut_Detail_TrackerSampler(), sampler.as_raw_PtrOfDetail_TrackerSamplerAlgorithm(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Detail_TrackerSampler {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerSampler")
				.finish()
		}
	}

	impl crate::tracking::Detail_TrackerSamplerTraitConst for Detail_TrackerSampler {
		#[inline] fn as_raw_Detail_TrackerSampler(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerSamplerTrait for Detail_TrackerSampler {
		#[inline] fn as_raw_mut_Detail_TrackerSampler(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerSampler, crate::tracking::Detail_TrackerSamplerTraitConst, as_raw_Detail_TrackerSampler, crate::tracking::Detail_TrackerSamplerTrait, as_raw_mut_Detail_TrackerSampler }

	/// Abstract base class for TrackerSamplerAlgorithm that represents the algorithm for the specific
	/// sampler.
	pub struct Detail_TrackerSamplerAlgorithm {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerSamplerAlgorithm }

	impl Drop for Detail_TrackerSamplerAlgorithm {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerSamplerAlgorithm_delete(self.as_raw_mut_Detail_TrackerSamplerAlgorithm()) };
		}
	}

	unsafe impl Send for Detail_TrackerSamplerAlgorithm {}

	/// Constant methods for [crate::tracking::Detail_TrackerSamplerAlgorithm]
	pub trait Detail_TrackerSamplerAlgorithmTraitConst {
		fn as_raw_Detail_TrackerSamplerAlgorithm(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerSamplerAlgorithm]
	pub trait Detail_TrackerSamplerAlgorithmTrait: crate::tracking::Detail_TrackerSamplerAlgorithmTraitConst {
		fn as_raw_mut_Detail_TrackerSamplerAlgorithm(&mut self) -> *mut c_void;

		/// Computes the regions starting from a position in an image.
		///
		/// Return true if samples are computed, false otherwise
		///
		/// ## Parameters
		/// * image: The current frame
		/// * boundingBox: The bounding box from which regions can be calculated
		///
		/// * sample: The computed samples [AAM](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_AAM) Fig. 1 variable Sk
		#[inline]
		fn sampling(&mut self, image: &impl core::MatTraitConst, bounding_box: core::Rect, sample: &mut core::Vector<core::Mat>) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerSamplerAlgorithm_sampling_const_MatR_const_RectR_vectorLMatGR(self.as_raw_mut_Detail_TrackerSamplerAlgorithm(), image.as_raw_Mat(), &bounding_box, sample.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Detail_TrackerSamplerAlgorithm {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerSamplerAlgorithm")
				.finish()
		}
	}

	boxed_cast_descendant! { Detail_TrackerSamplerAlgorithm, crate::tracking::Detail_TrackerContribSamplerAlgorithm, cv_detail_TrackerSamplerAlgorithm_to_Detail_TrackerContribSamplerAlgorithm }

	boxed_cast_descendant! { Detail_TrackerSamplerAlgorithm, crate::tracking::Detail_TrackerContribSamplerCSC, cv_detail_TrackerSamplerAlgorithm_to_Detail_TrackerContribSamplerCSC }

	boxed_cast_descendant! { Detail_TrackerSamplerAlgorithm, crate::tracking::Detail_TrackerSamplerCS, cv_detail_TrackerSamplerAlgorithm_to_Detail_TrackerSamplerCS }

	boxed_cast_descendant! { Detail_TrackerSamplerAlgorithm, crate::tracking::Detail_TrackerSamplerCSC, cv_detail_TrackerSamplerAlgorithm_to_Detail_TrackerSamplerCSC }

	boxed_cast_descendant! { Detail_TrackerSamplerAlgorithm, crate::tracking::Detail_TrackerSamplerPF, cv_detail_TrackerSamplerAlgorithm_to_Detail_TrackerSamplerPF }

	impl crate::tracking::Detail_TrackerSamplerAlgorithmTraitConst for Detail_TrackerSamplerAlgorithm {
		#[inline] fn as_raw_Detail_TrackerSamplerAlgorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerSamplerAlgorithmTrait for Detail_TrackerSamplerAlgorithm {
		#[inline] fn as_raw_mut_Detail_TrackerSamplerAlgorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerSamplerAlgorithm, crate::tracking::Detail_TrackerSamplerAlgorithmTraitConst, as_raw_Detail_TrackerSamplerAlgorithm, crate::tracking::Detail_TrackerSamplerAlgorithmTrait, as_raw_mut_Detail_TrackerSamplerAlgorithm }

	/// TrackerContribSampler based on CS (current state), used by algorithm TrackerBoosting
	pub struct Detail_TrackerSamplerCS {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerSamplerCS }

	impl Drop for Detail_TrackerSamplerCS {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerSamplerCS_delete(self.as_raw_mut_Detail_TrackerSamplerCS()) };
		}
	}

	unsafe impl Send for Detail_TrackerSamplerCS {}

	impl Detail_TrackerSamplerCS {
		/// Constructor
		/// ## Parameters
		/// * parameters: TrackerSamplerCS parameters TrackerSamplerCS::Params
		///
		/// ## C++ default parameters
		/// * parameters: TrackerSamplerCS::Params()
		#[inline]
		pub fn new(parameters: &impl crate::tracking::Detail_TrackerSamplerCS_ParamsTraitConst) -> Result<crate::tracking::Detail_TrackerSamplerCS> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerSamplerCS_TrackerSamplerCS_const_ParamsR(parameters.as_raw_Detail_TrackerSamplerCS_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Detail_TrackerSamplerCS::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Constructor
		/// ## Parameters
		/// * parameters: TrackerSamplerCS parameters TrackerSamplerCS::Params
		///
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * parameters: TrackerSamplerCS::Params()
		#[inline]
		pub fn new_def() -> Result<crate::tracking::Detail_TrackerSamplerCS> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerSamplerCS_TrackerSamplerCS(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Detail_TrackerSamplerCS::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Detail_TrackerSamplerCS]
	pub trait Detail_TrackerSamplerCSTraitConst: crate::tracking::Detail_TrackerContribSamplerAlgorithmTraitConst {
		fn as_raw_Detail_TrackerSamplerCS(&self) -> *const c_void;

		#[inline]
		fn get_roi(&self) -> Result<core::Rect> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerSamplerCS_getROI_const(self.as_raw_Detail_TrackerSamplerCS(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerSamplerCS]
	pub trait Detail_TrackerSamplerCSTrait: crate::tracking::Detail_TrackerContribSamplerAlgorithmTrait + crate::tracking::Detail_TrackerSamplerCSTraitConst {
		fn as_raw_mut_Detail_TrackerSamplerCS(&mut self) -> *mut c_void;

		/// Set the sampling mode of TrackerSamplerCS
		/// ## Parameters
		/// * samplingMode: The sampling mode
		///
		/// The modes are:
		///
		/// *   "MODE_POSITIVE = 1" -- for the positive sampling
		/// *   "MODE_NEGATIVE = 2" -- for the negative sampling
		/// *   "MODE_CLASSIFY = 3" -- for the sampling in classification step
		#[inline]
		fn set_mode(&mut self, sampling_mode: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerSamplerCS_setMode_int(self.as_raw_mut_Detail_TrackerSamplerCS(), sampling_mode, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn sampling_impl(&mut self, image: &impl core::MatTraitConst, bounding_box: core::Rect, sample: &mut core::Vector<core::Mat>) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerSamplerCS_samplingImpl_const_MatR_Rect_vectorLMatGR(self.as_raw_mut_Detail_TrackerSamplerCS(), image.as_raw_Mat(), &bounding_box, sample.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Detail_TrackerSamplerCS {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerSamplerCS")
				.finish()
		}
	}

	boxed_cast_base! { Detail_TrackerSamplerCS, crate::tracking::Detail_TrackerContribSamplerAlgorithm, cv_detail_TrackerSamplerCS_to_Detail_TrackerContribSamplerAlgorithm }

	boxed_cast_base! { Detail_TrackerSamplerCS, crate::tracking::Detail_TrackerSamplerAlgorithm, cv_detail_TrackerSamplerCS_to_Detail_TrackerSamplerAlgorithm }

	impl crate::tracking::Detail_TrackerContribSamplerAlgorithmTraitConst for Detail_TrackerSamplerCS {
		#[inline] fn as_raw_Detail_TrackerContribSamplerAlgorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerContribSamplerAlgorithmTrait for Detail_TrackerSamplerCS {
		#[inline] fn as_raw_mut_Detail_TrackerContribSamplerAlgorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerSamplerCS, crate::tracking::Detail_TrackerContribSamplerAlgorithmTraitConst, as_raw_Detail_TrackerContribSamplerAlgorithm, crate::tracking::Detail_TrackerContribSamplerAlgorithmTrait, as_raw_mut_Detail_TrackerContribSamplerAlgorithm }

	impl crate::tracking::Detail_TrackerSamplerAlgorithmTraitConst for Detail_TrackerSamplerCS {
		#[inline] fn as_raw_Detail_TrackerSamplerAlgorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerSamplerAlgorithmTrait for Detail_TrackerSamplerCS {
		#[inline] fn as_raw_mut_Detail_TrackerSamplerAlgorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerSamplerCS, crate::tracking::Detail_TrackerSamplerAlgorithmTraitConst, as_raw_Detail_TrackerSamplerAlgorithm, crate::tracking::Detail_TrackerSamplerAlgorithmTrait, as_raw_mut_Detail_TrackerSamplerAlgorithm }

	impl crate::tracking::Detail_TrackerSamplerCSTraitConst for Detail_TrackerSamplerCS {
		#[inline] fn as_raw_Detail_TrackerSamplerCS(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerSamplerCSTrait for Detail_TrackerSamplerCS {
		#[inline] fn as_raw_mut_Detail_TrackerSamplerCS(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerSamplerCS, crate::tracking::Detail_TrackerSamplerCSTraitConst, as_raw_Detail_TrackerSamplerCS, crate::tracking::Detail_TrackerSamplerCSTrait, as_raw_mut_Detail_TrackerSamplerCS }

	pub struct Detail_TrackerSamplerCS_Params {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerSamplerCS_Params }

	impl Drop for Detail_TrackerSamplerCS_Params {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerSamplerCS_Params_delete(self.as_raw_mut_Detail_TrackerSamplerCS_Params()) };
		}
	}

	unsafe impl Send for Detail_TrackerSamplerCS_Params {}

	impl Detail_TrackerSamplerCS_Params {
		#[inline]
		pub fn default() -> Result<crate::tracking::Detail_TrackerSamplerCS_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerSamplerCS_Params_Params(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Detail_TrackerSamplerCS_Params::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Detail_TrackerSamplerCS_Params]
	pub trait Detail_TrackerSamplerCS_ParamsTraitConst {
		fn as_raw_Detail_TrackerSamplerCS_Params(&self) -> *const c_void;

		/// overlapping for the search windows
		#[inline]
		fn overlap(&self) -> f32 {
			let ret = unsafe { sys::cv_detail_TrackerSamplerCS_Params_propOverlap_const(self.as_raw_Detail_TrackerSamplerCS_Params()) };
			ret
		}

		/// search region parameter
		#[inline]
		fn search_factor(&self) -> f32 {
			let ret = unsafe { sys::cv_detail_TrackerSamplerCS_Params_propSearchFactor_const(self.as_raw_Detail_TrackerSamplerCS_Params()) };
			ret
		}

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerSamplerCS_Params]
	pub trait Detail_TrackerSamplerCS_ParamsTrait: crate::tracking::Detail_TrackerSamplerCS_ParamsTraitConst {
		fn as_raw_mut_Detail_TrackerSamplerCS_Params(&mut self) -> *mut c_void;

		/// overlapping for the search windows
		#[inline]
		fn set_overlap(&mut self, val: f32) {
			let ret = unsafe { sys::cv_detail_TrackerSamplerCS_Params_propOverlap_const_float(self.as_raw_mut_Detail_TrackerSamplerCS_Params(), val) };
			ret
		}

		/// search region parameter
		#[inline]
		fn set_search_factor(&mut self, val: f32) {
			let ret = unsafe { sys::cv_detail_TrackerSamplerCS_Params_propSearchFactor_const_float(self.as_raw_mut_Detail_TrackerSamplerCS_Params(), val) };
			ret
		}

	}

	impl std::fmt::Debug for Detail_TrackerSamplerCS_Params {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerSamplerCS_Params")
				.field("overlap", &crate::tracking::Detail_TrackerSamplerCS_ParamsTraitConst::overlap(self))
				.field("search_factor", &crate::tracking::Detail_TrackerSamplerCS_ParamsTraitConst::search_factor(self))
				.finish()
		}
	}

	impl crate::tracking::Detail_TrackerSamplerCS_ParamsTraitConst for Detail_TrackerSamplerCS_Params {
		#[inline] fn as_raw_Detail_TrackerSamplerCS_Params(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerSamplerCS_ParamsTrait for Detail_TrackerSamplerCS_Params {
		#[inline] fn as_raw_mut_Detail_TrackerSamplerCS_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerSamplerCS_Params, crate::tracking::Detail_TrackerSamplerCS_ParamsTraitConst, as_raw_Detail_TrackerSamplerCS_Params, crate::tracking::Detail_TrackerSamplerCS_ParamsTrait, as_raw_mut_Detail_TrackerSamplerCS_Params }

	/// TrackerSampler based on CSC (current state centered), used by MIL algorithm TrackerMIL
	pub struct Detail_TrackerSamplerCSC {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerSamplerCSC }

	impl Drop for Detail_TrackerSamplerCSC {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerSamplerCSC_delete(self.as_raw_mut_Detail_TrackerSamplerCSC()) };
		}
	}

	unsafe impl Send for Detail_TrackerSamplerCSC {}

	impl Detail_TrackerSamplerCSC {
		/// Constructor
		/// ## Parameters
		/// * parameters: TrackerSamplerCSC parameters TrackerSamplerCSC::Params
		///
		/// ## C++ default parameters
		/// * parameters: TrackerSamplerCSC::Params()
		#[inline]
		pub fn new(parameters: &impl crate::tracking::Detail_TrackerSamplerCSC_ParamsTraitConst) -> Result<crate::tracking::Detail_TrackerSamplerCSC> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerSamplerCSC_TrackerSamplerCSC_const_ParamsR(parameters.as_raw_Detail_TrackerSamplerCSC_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Detail_TrackerSamplerCSC::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Constructor
		/// ## Parameters
		/// * parameters: TrackerSamplerCSC parameters TrackerSamplerCSC::Params
		///
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * parameters: TrackerSamplerCSC::Params()
		#[inline]
		pub fn new_def() -> Result<crate::tracking::Detail_TrackerSamplerCSC> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerSamplerCSC_TrackerSamplerCSC(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Detail_TrackerSamplerCSC::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Detail_TrackerSamplerCSC]
	pub trait Detail_TrackerSamplerCSCTraitConst: crate::tracking::Detail_TrackerSamplerAlgorithmTraitConst {
		fn as_raw_Detail_TrackerSamplerCSC(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerSamplerCSC]
	pub trait Detail_TrackerSamplerCSCTrait: crate::tracking::Detail_TrackerSamplerAlgorithmTrait + crate::tracking::Detail_TrackerSamplerCSCTraitConst {
		fn as_raw_mut_Detail_TrackerSamplerCSC(&mut self) -> *mut c_void;

		/// Set the sampling mode of TrackerSamplerCSC
		/// ## Parameters
		/// * samplingMode: The sampling mode
		///
		/// The modes are:
		///
		/// *   "MODE_INIT_POS = 1" -- for the positive sampling in initialization step
		/// *   "MODE_INIT_NEG = 2" -- for the negative sampling in initialization step
		/// *   "MODE_TRACK_POS = 3" -- for the positive sampling in update step
		/// *   "MODE_TRACK_NEG = 4" -- for the negative sampling in update step
		/// *   "MODE_DETECT = 5" -- for the sampling in detection step
		#[inline]
		fn set_mode(&mut self, sampling_mode: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerSamplerCSC_setMode_int(self.as_raw_mut_Detail_TrackerSamplerCSC(), sampling_mode, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn sampling(&mut self, image: &impl core::MatTraitConst, bounding_box: core::Rect, sample: &mut core::Vector<core::Mat>) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerSamplerCSC_sampling_const_MatR_const_RectR_vectorLMatGR(self.as_raw_mut_Detail_TrackerSamplerCSC(), image.as_raw_Mat(), &bounding_box, sample.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Detail_TrackerSamplerCSC {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerSamplerCSC")
				.finish()
		}
	}

	boxed_cast_base! { Detail_TrackerSamplerCSC, crate::tracking::Detail_TrackerSamplerAlgorithm, cv_detail_TrackerSamplerCSC_to_Detail_TrackerSamplerAlgorithm }

	impl crate::tracking::Detail_TrackerSamplerAlgorithmTraitConst for Detail_TrackerSamplerCSC {
		#[inline] fn as_raw_Detail_TrackerSamplerAlgorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerSamplerAlgorithmTrait for Detail_TrackerSamplerCSC {
		#[inline] fn as_raw_mut_Detail_TrackerSamplerAlgorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerSamplerCSC, crate::tracking::Detail_TrackerSamplerAlgorithmTraitConst, as_raw_Detail_TrackerSamplerAlgorithm, crate::tracking::Detail_TrackerSamplerAlgorithmTrait, as_raw_mut_Detail_TrackerSamplerAlgorithm }

	impl crate::tracking::Detail_TrackerSamplerCSCTraitConst for Detail_TrackerSamplerCSC {
		#[inline] fn as_raw_Detail_TrackerSamplerCSC(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerSamplerCSCTrait for Detail_TrackerSamplerCSC {
		#[inline] fn as_raw_mut_Detail_TrackerSamplerCSC(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerSamplerCSC, crate::tracking::Detail_TrackerSamplerCSCTraitConst, as_raw_Detail_TrackerSamplerCSC, crate::tracking::Detail_TrackerSamplerCSCTrait, as_raw_mut_Detail_TrackerSamplerCSC }

	pub struct Detail_TrackerSamplerCSC_Params {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerSamplerCSC_Params }

	impl Drop for Detail_TrackerSamplerCSC_Params {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerSamplerCSC_Params_delete(self.as_raw_mut_Detail_TrackerSamplerCSC_Params()) };
		}
	}

	unsafe impl Send for Detail_TrackerSamplerCSC_Params {}

	impl Detail_TrackerSamplerCSC_Params {
		#[inline]
		pub fn default() -> Result<crate::tracking::Detail_TrackerSamplerCSC_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerSamplerCSC_Params_Params(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Detail_TrackerSamplerCSC_Params::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Detail_TrackerSamplerCSC_Params]
	pub trait Detail_TrackerSamplerCSC_ParamsTraitConst {
		fn as_raw_Detail_TrackerSamplerCSC_Params(&self) -> *const c_void;

		/// radius for gathering positive instances during init
		#[inline]
		fn init_in_rad(&self) -> f32 {
			let ret = unsafe { sys::cv_detail_TrackerSamplerCSC_Params_propInitInRad_const(self.as_raw_Detail_TrackerSamplerCSC_Params()) };
			ret
		}

		/// radius for gathering positive instances during tracking
		#[inline]
		fn track_in_pos_rad(&self) -> f32 {
			let ret = unsafe { sys::cv_detail_TrackerSamplerCSC_Params_propTrackInPosRad_const(self.as_raw_Detail_TrackerSamplerCSC_Params()) };
			ret
		}

		/// size of search window
		#[inline]
		fn search_win_size(&self) -> f32 {
			let ret = unsafe { sys::cv_detail_TrackerSamplerCSC_Params_propSearchWinSize_const(self.as_raw_Detail_TrackerSamplerCSC_Params()) };
			ret
		}

		/// # negative samples to use during init
		#[inline]
		fn init_max_neg_num(&self) -> i32 {
			let ret = unsafe { sys::cv_detail_TrackerSamplerCSC_Params_propInitMaxNegNum_const(self.as_raw_Detail_TrackerSamplerCSC_Params()) };
			ret
		}

		/// # positive samples to use during training
		#[inline]
		fn track_max_pos_num(&self) -> i32 {
			let ret = unsafe { sys::cv_detail_TrackerSamplerCSC_Params_propTrackMaxPosNum_const(self.as_raw_Detail_TrackerSamplerCSC_Params()) };
			ret
		}

		/// # negative samples to use during training
		#[inline]
		fn track_max_neg_num(&self) -> i32 {
			let ret = unsafe { sys::cv_detail_TrackerSamplerCSC_Params_propTrackMaxNegNum_const(self.as_raw_Detail_TrackerSamplerCSC_Params()) };
			ret
		}

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerSamplerCSC_Params]
	pub trait Detail_TrackerSamplerCSC_ParamsTrait: crate::tracking::Detail_TrackerSamplerCSC_ParamsTraitConst {
		fn as_raw_mut_Detail_TrackerSamplerCSC_Params(&mut self) -> *mut c_void;

		/// radius for gathering positive instances during init
		#[inline]
		fn set_init_in_rad(&mut self, val: f32) {
			let ret = unsafe { sys::cv_detail_TrackerSamplerCSC_Params_propInitInRad_const_float(self.as_raw_mut_Detail_TrackerSamplerCSC_Params(), val) };
			ret
		}

		/// radius for gathering positive instances during tracking
		#[inline]
		fn set_track_in_pos_rad(&mut self, val: f32) {
			let ret = unsafe { sys::cv_detail_TrackerSamplerCSC_Params_propTrackInPosRad_const_float(self.as_raw_mut_Detail_TrackerSamplerCSC_Params(), val) };
			ret
		}

		/// size of search window
		#[inline]
		fn set_search_win_size(&mut self, val: f32) {
			let ret = unsafe { sys::cv_detail_TrackerSamplerCSC_Params_propSearchWinSize_const_float(self.as_raw_mut_Detail_TrackerSamplerCSC_Params(), val) };
			ret
		}

		/// # negative samples to use during init
		#[inline]
		fn set_init_max_neg_num(&mut self, val: i32) {
			let ret = unsafe { sys::cv_detail_TrackerSamplerCSC_Params_propInitMaxNegNum_const_int(self.as_raw_mut_Detail_TrackerSamplerCSC_Params(), val) };
			ret
		}

		/// # positive samples to use during training
		#[inline]
		fn set_track_max_pos_num(&mut self, val: i32) {
			let ret = unsafe { sys::cv_detail_TrackerSamplerCSC_Params_propTrackMaxPosNum_const_int(self.as_raw_mut_Detail_TrackerSamplerCSC_Params(), val) };
			ret
		}

		/// # negative samples to use during training
		#[inline]
		fn set_track_max_neg_num(&mut self, val: i32) {
			let ret = unsafe { sys::cv_detail_TrackerSamplerCSC_Params_propTrackMaxNegNum_const_int(self.as_raw_mut_Detail_TrackerSamplerCSC_Params(), val) };
			ret
		}

	}

	impl std::fmt::Debug for Detail_TrackerSamplerCSC_Params {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerSamplerCSC_Params")
				.field("init_in_rad", &crate::tracking::Detail_TrackerSamplerCSC_ParamsTraitConst::init_in_rad(self))
				.field("track_in_pos_rad", &crate::tracking::Detail_TrackerSamplerCSC_ParamsTraitConst::track_in_pos_rad(self))
				.field("search_win_size", &crate::tracking::Detail_TrackerSamplerCSC_ParamsTraitConst::search_win_size(self))
				.field("init_max_neg_num", &crate::tracking::Detail_TrackerSamplerCSC_ParamsTraitConst::init_max_neg_num(self))
				.field("track_max_pos_num", &crate::tracking::Detail_TrackerSamplerCSC_ParamsTraitConst::track_max_pos_num(self))
				.field("track_max_neg_num", &crate::tracking::Detail_TrackerSamplerCSC_ParamsTraitConst::track_max_neg_num(self))
				.finish()
		}
	}

	impl crate::tracking::Detail_TrackerSamplerCSC_ParamsTraitConst for Detail_TrackerSamplerCSC_Params {
		#[inline] fn as_raw_Detail_TrackerSamplerCSC_Params(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerSamplerCSC_ParamsTrait for Detail_TrackerSamplerCSC_Params {
		#[inline] fn as_raw_mut_Detail_TrackerSamplerCSC_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerSamplerCSC_Params, crate::tracking::Detail_TrackerSamplerCSC_ParamsTraitConst, as_raw_Detail_TrackerSamplerCSC_Params, crate::tracking::Detail_TrackerSamplerCSC_ParamsTrait, as_raw_mut_Detail_TrackerSamplerCSC_Params }

	/// This sampler is based on particle filtering.
	///
	/// In principle, it can be thought of as performing some sort of optimization (and indeed, this
	/// tracker uses opencv's optim module), where tracker seeks to find the rectangle in given frame,
	/// which is the most *"similar"* to the initial rectangle (the one, given through the constructor).
	///
	/// The optimization performed is stochastic and somehow resembles genetic algorithms, where on each new
	/// image received (submitted via TrackerSamplerPF::sampling()) we start with the region bounded by
	/// boundingBox, then generate several "perturbed" boxes, take the ones most similar to the original.
	/// This selection round is repeated several times. At the end, we hope that only the most promising box
	/// remaining, and these are combined to produce the subrectangle of image, which is put as a sole
	/// element in array sample.
	///
	/// It should be noted, that the definition of "similarity" between two rectangles is based on comparing
	/// their histograms. As experiments show, tracker is *not* very succesfull if target is assumed to
	/// strongly change its dimensions.
	pub struct Detail_TrackerSamplerPF {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerSamplerPF }

	impl Drop for Detail_TrackerSamplerPF {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerSamplerPF_delete(self.as_raw_mut_Detail_TrackerSamplerPF()) };
		}
	}

	unsafe impl Send for Detail_TrackerSamplerPF {}

	impl Detail_TrackerSamplerPF {
		/// Constructor
		/// ## Parameters
		/// * chosenRect: Initial rectangle, that is supposed to contain target we'd like to track.
		/// * parameters: 
		///
		/// ## C++ default parameters
		/// * parameters: TrackerSamplerPF::Params()
		#[inline]
		pub fn new(chosen_rect: &impl core::MatTraitConst, parameters: &impl crate::tracking::Detail_TrackerSamplerPF_ParamsTraitConst) -> Result<crate::tracking::Detail_TrackerSamplerPF> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerSamplerPF_TrackerSamplerPF_const_MatR_const_ParamsR(chosen_rect.as_raw_Mat(), parameters.as_raw_Detail_TrackerSamplerPF_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Detail_TrackerSamplerPF::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Constructor
		/// ## Parameters
		/// * chosenRect: Initial rectangle, that is supposed to contain target we'd like to track.
		/// * parameters: 
		///
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * parameters: TrackerSamplerPF::Params()
		#[inline]
		pub fn new_def(chosen_rect: &impl core::MatTraitConst) -> Result<crate::tracking::Detail_TrackerSamplerPF> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerSamplerPF_TrackerSamplerPF_const_MatR(chosen_rect.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Detail_TrackerSamplerPF::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Detail_TrackerSamplerPF]
	pub trait Detail_TrackerSamplerPFTraitConst: crate::tracking::Detail_TrackerContribSamplerAlgorithmTraitConst {
		fn as_raw_Detail_TrackerSamplerPF(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerSamplerPF]
	pub trait Detail_TrackerSamplerPFTrait: crate::tracking::Detail_TrackerContribSamplerAlgorithmTrait + crate::tracking::Detail_TrackerSamplerPFTraitConst {
		fn as_raw_mut_Detail_TrackerSamplerPF(&mut self) -> *mut c_void;

	}

	impl std::fmt::Debug for Detail_TrackerSamplerPF {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerSamplerPF")
				.finish()
		}
	}

	boxed_cast_base! { Detail_TrackerSamplerPF, crate::tracking::Detail_TrackerContribSamplerAlgorithm, cv_detail_TrackerSamplerPF_to_Detail_TrackerContribSamplerAlgorithm }

	boxed_cast_base! { Detail_TrackerSamplerPF, crate::tracking::Detail_TrackerSamplerAlgorithm, cv_detail_TrackerSamplerPF_to_Detail_TrackerSamplerAlgorithm }

	impl crate::tracking::Detail_TrackerContribSamplerAlgorithmTraitConst for Detail_TrackerSamplerPF {
		#[inline] fn as_raw_Detail_TrackerContribSamplerAlgorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerContribSamplerAlgorithmTrait for Detail_TrackerSamplerPF {
		#[inline] fn as_raw_mut_Detail_TrackerContribSamplerAlgorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerSamplerPF, crate::tracking::Detail_TrackerContribSamplerAlgorithmTraitConst, as_raw_Detail_TrackerContribSamplerAlgorithm, crate::tracking::Detail_TrackerContribSamplerAlgorithmTrait, as_raw_mut_Detail_TrackerContribSamplerAlgorithm }

	impl crate::tracking::Detail_TrackerSamplerAlgorithmTraitConst for Detail_TrackerSamplerPF {
		#[inline] fn as_raw_Detail_TrackerSamplerAlgorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerSamplerAlgorithmTrait for Detail_TrackerSamplerPF {
		#[inline] fn as_raw_mut_Detail_TrackerSamplerAlgorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerSamplerPF, crate::tracking::Detail_TrackerSamplerAlgorithmTraitConst, as_raw_Detail_TrackerSamplerAlgorithm, crate::tracking::Detail_TrackerSamplerAlgorithmTrait, as_raw_mut_Detail_TrackerSamplerAlgorithm }

	impl crate::tracking::Detail_TrackerSamplerPFTraitConst for Detail_TrackerSamplerPF {
		#[inline] fn as_raw_Detail_TrackerSamplerPF(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerSamplerPFTrait for Detail_TrackerSamplerPF {
		#[inline] fn as_raw_mut_Detail_TrackerSamplerPF(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerSamplerPF, crate::tracking::Detail_TrackerSamplerPFTraitConst, as_raw_Detail_TrackerSamplerPF, crate::tracking::Detail_TrackerSamplerPFTrait, as_raw_mut_Detail_TrackerSamplerPF }

	/// This structure contains all the parameters that can be varied during the course of sampling
	/// algorithm. Below is the structure exposed, together with its members briefly explained with
	/// reference to the above discussion on algorithm's working.
	pub struct Detail_TrackerSamplerPF_Params {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerSamplerPF_Params }

	impl Drop for Detail_TrackerSamplerPF_Params {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerSamplerPF_Params_delete(self.as_raw_mut_Detail_TrackerSamplerPF_Params()) };
		}
	}

	unsafe impl Send for Detail_TrackerSamplerPF_Params {}

	impl Detail_TrackerSamplerPF_Params {
		#[inline]
		pub fn default() -> Result<crate::tracking::Detail_TrackerSamplerPF_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerSamplerPF_Params_Params(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Detail_TrackerSamplerPF_Params::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Detail_TrackerSamplerPF_Params]
	pub trait Detail_TrackerSamplerPF_ParamsTraitConst {
		fn as_raw_Detail_TrackerSamplerPF_Params(&self) -> *const c_void;

		/// number of selection rounds
		#[inline]
		fn iteration_num(&self) -> i32 {
			let ret = unsafe { sys::cv_detail_TrackerSamplerPF_Params_propIterationNum_const(self.as_raw_Detail_TrackerSamplerPF_Params()) };
			ret
		}

		/// number of "perturbed" boxes on each round
		#[inline]
		fn particles_num(&self) -> i32 {
			let ret = unsafe { sys::cv_detail_TrackerSamplerPF_Params_propParticlesNum_const(self.as_raw_Detail_TrackerSamplerPF_Params()) };
			ret
		}

		/// with each new round we exponentially decrease the amount of "perturbing" we allow (like in simulated annealing)
		/// and this very alpha controls how fast annealing happens, ie. how fast perturbing decreases
		#[inline]
		fn alpha(&self) -> f64 {
			let ret = unsafe { sys::cv_detail_TrackerSamplerPF_Params_propAlpha_const(self.as_raw_Detail_TrackerSamplerPF_Params()) };
			ret
		}

		/// initial values for perturbing (1-by-4 array, as each rectangle is given by 4 values -- coordinates of opposite vertices,
		/// hence we have 4 values to perturb)
		#[inline]
		fn std(&self) -> core::Mat_<f64> {
			let ret = unsafe { sys::cv_detail_TrackerSamplerPF_Params_propStd_const(self.as_raw_Detail_TrackerSamplerPF_Params()) };
			let ret = unsafe { core::Mat_::<f64>::opencv_from_extern(ret) };
			ret
		}

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerSamplerPF_Params]
	pub trait Detail_TrackerSamplerPF_ParamsTrait: crate::tracking::Detail_TrackerSamplerPF_ParamsTraitConst {
		fn as_raw_mut_Detail_TrackerSamplerPF_Params(&mut self) -> *mut c_void;

		/// number of selection rounds
		#[inline]
		fn set_iteration_num(&mut self, val: i32) {
			let ret = unsafe { sys::cv_detail_TrackerSamplerPF_Params_propIterationNum_const_int(self.as_raw_mut_Detail_TrackerSamplerPF_Params(), val) };
			ret
		}

		/// number of "perturbed" boxes on each round
		#[inline]
		fn set_particles_num(&mut self, val: i32) {
			let ret = unsafe { sys::cv_detail_TrackerSamplerPF_Params_propParticlesNum_const_int(self.as_raw_mut_Detail_TrackerSamplerPF_Params(), val) };
			ret
		}

		/// with each new round we exponentially decrease the amount of "perturbing" we allow (like in simulated annealing)
		/// and this very alpha controls how fast annealing happens, ie. how fast perturbing decreases
		#[inline]
		fn set_alpha(&mut self, val: f64) {
			let ret = unsafe { sys::cv_detail_TrackerSamplerPF_Params_propAlpha_const_double(self.as_raw_mut_Detail_TrackerSamplerPF_Params(), val) };
			ret
		}

		/// initial values for perturbing (1-by-4 array, as each rectangle is given by 4 values -- coordinates of opposite vertices,
		/// hence we have 4 values to perturb)
		#[inline]
		fn set_std(&mut self, val: core::Mat_<f64>) {
			let ret = unsafe { sys::cv_detail_TrackerSamplerPF_Params_propStd_const_Mat_LdoubleG(self.as_raw_mut_Detail_TrackerSamplerPF_Params(), val.as_raw_Mat_()) };
			ret
		}

	}

	impl std::fmt::Debug for Detail_TrackerSamplerPF_Params {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerSamplerPF_Params")
				.field("iteration_num", &crate::tracking::Detail_TrackerSamplerPF_ParamsTraitConst::iteration_num(self))
				.field("particles_num", &crate::tracking::Detail_TrackerSamplerPF_ParamsTraitConst::particles_num(self))
				.field("alpha", &crate::tracking::Detail_TrackerSamplerPF_ParamsTraitConst::alpha(self))
				.field("std", &crate::tracking::Detail_TrackerSamplerPF_ParamsTraitConst::std(self))
				.finish()
		}
	}

	impl crate::tracking::Detail_TrackerSamplerPF_ParamsTraitConst for Detail_TrackerSamplerPF_Params {
		#[inline] fn as_raw_Detail_TrackerSamplerPF_Params(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerSamplerPF_ParamsTrait for Detail_TrackerSamplerPF_Params {
		#[inline] fn as_raw_mut_Detail_TrackerSamplerPF_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerSamplerPF_Params, crate::tracking::Detail_TrackerSamplerPF_ParamsTraitConst, as_raw_Detail_TrackerSamplerPF_Params, crate::tracking::Detail_TrackerSamplerPF_ParamsTrait, as_raw_mut_Detail_TrackerSamplerPF_Params }

	/// Abstract base class for TrackerStateEstimator that estimates the most likely target state.
	///
	/// See [AAM](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_AAM) State estimator
	///
	/// See [AMVOT](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_AMVOT) Statistical modeling (Fig. 3), Table III (generative) - IV (discriminative) - V (hybrid)
	pub struct Detail_TrackerStateEstimator {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerStateEstimator }

	impl Drop for Detail_TrackerStateEstimator {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerStateEstimator_delete(self.as_raw_mut_Detail_TrackerStateEstimator()) };
		}
	}

	unsafe impl Send for Detail_TrackerStateEstimator {}

	impl Detail_TrackerStateEstimator {
		/// Create TrackerStateEstimator by tracker state estimator type
		/// ## Parameters
		/// * trackeStateEstimatorType: The TrackerStateEstimator name
		///
		/// The modes available now:
		///
		/// *   "BOOSTING" -- Boosting-based discriminative appearance models. See [AMVOT](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_AMVOT) section 4.4
		///
		/// The modes available soon:
		///
		/// *   "SVM" -- SVM-based discriminative appearance models. See [AMVOT](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_AMVOT) section 4.5
		#[inline]
		pub fn create(tracke_state_estimator_type: &str) -> Result<core::Ptr<crate::tracking::Detail_TrackerStateEstimator>> {
			extern_container_arg!(tracke_state_estimator_type);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerStateEstimator_create_const_StringR(tracke_state_estimator_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::tracking::Detail_TrackerStateEstimator>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Detail_TrackerStateEstimator]
	pub trait Detail_TrackerStateEstimatorTraitConst {
		fn as_raw_Detail_TrackerStateEstimator(&self) -> *const c_void;

		/// Get the name of the specific TrackerStateEstimator
		#[inline]
		fn get_class_name(&self) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerStateEstimator_getClassName_const(self.as_raw_Detail_TrackerStateEstimator(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerStateEstimator]
	pub trait Detail_TrackerStateEstimatorTrait: crate::tracking::Detail_TrackerStateEstimatorTraitConst {
		fn as_raw_mut_Detail_TrackerStateEstimator(&mut self) -> *mut c_void;

		/// Estimate the most likely target state, return the estimated state
		/// ## Parameters
		/// * confidenceMaps: The overall appearance model as a list of :cConfidenceMap
		#[inline]
		fn estimate(&mut self, confidence_maps: &core::Vector<crate::tracking::Detail_ConfidenceMap>) -> Result<core::Ptr<crate::tracking::Detail_TrackerTargetState>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerStateEstimator_estimate_const_vectorLConfidenceMapGR(self.as_raw_mut_Detail_TrackerStateEstimator(), confidence_maps.as_raw_VectorOfDetail_ConfidenceMap(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::tracking::Detail_TrackerTargetState>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Update the ConfidenceMap with the scores
		/// ## Parameters
		/// * confidenceMaps: The overall appearance model as a list of :cConfidenceMap
		#[inline]
		fn update(&mut self, confidence_maps: &mut core::Vector<crate::tracking::Detail_ConfidenceMap>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerStateEstimator_update_vectorLConfidenceMapGR(self.as_raw_mut_Detail_TrackerStateEstimator(), confidence_maps.as_raw_mut_VectorOfDetail_ConfidenceMap(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Detail_TrackerStateEstimator {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerStateEstimator")
				.finish()
		}
	}

	boxed_cast_descendant! { Detail_TrackerStateEstimator, crate::tracking::Detail_TrackerStateEstimatorAdaBoosting, cv_detail_TrackerStateEstimator_to_Detail_TrackerStateEstimatorAdaBoosting }

	boxed_cast_descendant! { Detail_TrackerStateEstimator, crate::tracking::Detail_TrackerStateEstimatorSVM, cv_detail_TrackerStateEstimator_to_Detail_TrackerStateEstimatorSVM }

	impl crate::tracking::Detail_TrackerStateEstimatorTraitConst for Detail_TrackerStateEstimator {
		#[inline] fn as_raw_Detail_TrackerStateEstimator(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerStateEstimatorTrait for Detail_TrackerStateEstimator {
		#[inline] fn as_raw_mut_Detail_TrackerStateEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerStateEstimator, crate::tracking::Detail_TrackerStateEstimatorTraitConst, as_raw_Detail_TrackerStateEstimator, crate::tracking::Detail_TrackerStateEstimatorTrait, as_raw_mut_Detail_TrackerStateEstimator }

	/// TrackerStateEstimatorAdaBoosting based on ADA-Boosting
	pub struct Detail_TrackerStateEstimatorAdaBoosting {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerStateEstimatorAdaBoosting }

	impl Drop for Detail_TrackerStateEstimatorAdaBoosting {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerStateEstimatorAdaBoosting_delete(self.as_raw_mut_Detail_TrackerStateEstimatorAdaBoosting()) };
		}
	}

	unsafe impl Send for Detail_TrackerStateEstimatorAdaBoosting {}

	impl Detail_TrackerStateEstimatorAdaBoosting {
		/// Constructor
		/// ## Parameters
		/// * numClassifer: Number of base classifiers
		/// * initIterations: Number of iterations in the initialization
		/// * nFeatures: Number of features/weak classifiers
		/// * patchSize: tracking rect
		/// * ROI: initial ROI
		#[inline]
		pub fn new(num_classifer: i32, init_iterations: i32, n_features: i32, patch_size: core::Size, roi: core::Rect) -> Result<crate::tracking::Detail_TrackerStateEstimatorAdaBoosting> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerStateEstimatorAdaBoosting_TrackerStateEstimatorAdaBoosting_int_int_int_Size_const_RectR(num_classifer, init_iterations, n_features, &patch_size, &roi, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Detail_TrackerStateEstimatorAdaBoosting::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Detail_TrackerStateEstimatorAdaBoosting]
	pub trait Detail_TrackerStateEstimatorAdaBoostingTraitConst: crate::tracking::Detail_TrackerStateEstimatorTraitConst {
		fn as_raw_Detail_TrackerStateEstimatorAdaBoosting(&self) -> *const c_void;

		/// Get the sampling ROI
		#[inline]
		fn get_sample_roi(&self) -> Result<core::Rect> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerStateEstimatorAdaBoosting_getSampleROI_const(self.as_raw_Detail_TrackerStateEstimatorAdaBoosting(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerStateEstimatorAdaBoosting]
	pub trait Detail_TrackerStateEstimatorAdaBoostingTrait: crate::tracking::Detail_TrackerStateEstimatorAdaBoostingTraitConst + crate::tracking::Detail_TrackerStateEstimatorTrait {
		fn as_raw_mut_Detail_TrackerStateEstimatorAdaBoosting(&mut self) -> *mut c_void;

		/// Set the sampling ROI
		/// ## Parameters
		/// * ROI: the sampling ROI
		#[inline]
		fn set_sample_roi(&mut self, roi: core::Rect) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerStateEstimatorAdaBoosting_setSampleROI_const_RectR(self.as_raw_mut_Detail_TrackerStateEstimatorAdaBoosting(), &roi, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set the current confidenceMap
		/// ## Parameters
		/// * confidenceMap: The current :cConfidenceMap
		#[inline]
		fn set_current_confidence_map(&mut self, confidence_map: &mut crate::tracking::Detail_ConfidenceMap) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerStateEstimatorAdaBoosting_setCurrentConfidenceMap_ConfidenceMapR(self.as_raw_mut_Detail_TrackerStateEstimatorAdaBoosting(), confidence_map.as_raw_mut_VectorOfTupleOfPtrOfDetail_TrackerTargetState_f32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get the list of the selected weak classifiers for the classification step
		#[inline]
		fn compute_selected_weak_classifier(&mut self) -> Result<core::Vector<i32>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerStateEstimatorAdaBoosting_computeSelectedWeakClassifier(self.as_raw_mut_Detail_TrackerStateEstimatorAdaBoosting(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Get the list of the weak classifiers that should be replaced
		#[inline]
		fn compute_replaced_classifier(&mut self) -> Result<core::Vector<i32>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerStateEstimatorAdaBoosting_computeReplacedClassifier(self.as_raw_mut_Detail_TrackerStateEstimatorAdaBoosting(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Get the list of the weak classifiers that replace those to be replaced
		#[inline]
		fn compute_swapped_classifier(&mut self) -> Result<core::Vector<i32>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerStateEstimatorAdaBoosting_computeSwappedClassifier(self.as_raw_mut_Detail_TrackerStateEstimatorAdaBoosting(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Detail_TrackerStateEstimatorAdaBoosting {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerStateEstimatorAdaBoosting")
				.finish()
		}
	}

	boxed_cast_base! { Detail_TrackerStateEstimatorAdaBoosting, crate::tracking::Detail_TrackerStateEstimator, cv_detail_TrackerStateEstimatorAdaBoosting_to_Detail_TrackerStateEstimator }

	impl crate::tracking::Detail_TrackerStateEstimatorTraitConst for Detail_TrackerStateEstimatorAdaBoosting {
		#[inline] fn as_raw_Detail_TrackerStateEstimator(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerStateEstimatorTrait for Detail_TrackerStateEstimatorAdaBoosting {
		#[inline] fn as_raw_mut_Detail_TrackerStateEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerStateEstimatorAdaBoosting, crate::tracking::Detail_TrackerStateEstimatorTraitConst, as_raw_Detail_TrackerStateEstimator, crate::tracking::Detail_TrackerStateEstimatorTrait, as_raw_mut_Detail_TrackerStateEstimator }

	impl crate::tracking::Detail_TrackerStateEstimatorAdaBoostingTraitConst for Detail_TrackerStateEstimatorAdaBoosting {
		#[inline] fn as_raw_Detail_TrackerStateEstimatorAdaBoosting(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerStateEstimatorAdaBoostingTrait for Detail_TrackerStateEstimatorAdaBoosting {
		#[inline] fn as_raw_mut_Detail_TrackerStateEstimatorAdaBoosting(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerStateEstimatorAdaBoosting, crate::tracking::Detail_TrackerStateEstimatorAdaBoostingTraitConst, as_raw_Detail_TrackerStateEstimatorAdaBoosting, crate::tracking::Detail_TrackerStateEstimatorAdaBoostingTrait, as_raw_mut_Detail_TrackerStateEstimatorAdaBoosting }

	/// Implementation of the target state for TrackerAdaBoostingTargetState
	pub struct Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState }

	impl Drop for Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_delete(self.as_raw_mut_Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState()) };
		}
	}

	unsafe impl Send for Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState {}

	impl Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState {
		/// \brief Constructor
		/// \param position Top left corner of the bounding box
		/// \param width Width of the bounding box
		/// \param height Height of the bounding box
		/// \param foreground label for target or background
		/// \param responses list of features
		#[inline]
		pub fn new(position: core::Point2f, width: i32, height: i32, foreground: bool, responses: &impl core::MatTraitConst) -> Result<crate::tracking::Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_TrackerAdaBoostingTargetState_const_Point2fR_int_int_bool_const_MatR(&position, width, height, foreground, responses.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState]
	pub trait Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetStateTraitConst: crate::tracking::Detail_TrackerTargetStateTraitConst {
		fn as_raw_Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState(&self) -> *const c_void;

		/// Get the features extracted
		#[inline]
		fn get_target_responses(&self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_getTargetResponses_const(self.as_raw_Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Get the label. Return true for target foreground, false for background
		#[inline]
		fn is_target_fg(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_isTargetFg_const(self.as_raw_Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState]
	pub trait Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetStateTrait: crate::tracking::Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetStateTraitConst + crate::tracking::Detail_TrackerTargetStateTrait {
		fn as_raw_mut_Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState(&mut self) -> *mut c_void;

		/// Set the features extracted from TrackerContribFeatureSet
		/// ## Parameters
		/// * responses: The features extracted
		#[inline]
		fn set_target_responses(&mut self, responses: &impl core::MatTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_setTargetResponses_const_MatR(self.as_raw_mut_Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState(), responses.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set label: true for target foreground, false for background
		/// ## Parameters
		/// * foreground: Label for background/foreground
		#[inline]
		fn set_target_fg(&mut self, foreground: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_setTargetFg_bool(self.as_raw_mut_Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState(), foreground, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState")
				.finish()
		}
	}

	boxed_cast_base! { Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState, crate::tracking::Detail_TrackerTargetState, cv_detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_to_Detail_TrackerTargetState }

	impl crate::tracking::Detail_TrackerTargetStateTraitConst for Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState {
		#[inline] fn as_raw_Detail_TrackerTargetState(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerTargetStateTrait for Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState {
		#[inline] fn as_raw_mut_Detail_TrackerTargetState(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState, crate::tracking::Detail_TrackerTargetStateTraitConst, as_raw_Detail_TrackerTargetState, crate::tracking::Detail_TrackerTargetStateTrait, as_raw_mut_Detail_TrackerTargetState }

	impl crate::tracking::Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetStateTraitConst for Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState {
		#[inline] fn as_raw_Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetStateTrait for Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState {
		#[inline] fn as_raw_mut_Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState, crate::tracking::Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetStateTraitConst, as_raw_Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState, crate::tracking::Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetStateTrait, as_raw_mut_Detail_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState }

	/// \brief TrackerStateEstimator based on SVM
	pub struct Detail_TrackerStateEstimatorSVM {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerStateEstimatorSVM }

	impl Drop for Detail_TrackerStateEstimatorSVM {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerStateEstimatorSVM_delete(self.as_raw_mut_Detail_TrackerStateEstimatorSVM()) };
		}
	}

	unsafe impl Send for Detail_TrackerStateEstimatorSVM {}

	impl Detail_TrackerStateEstimatorSVM {
		#[inline]
		pub fn default() -> Result<crate::tracking::Detail_TrackerStateEstimatorSVM> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerStateEstimatorSVM_TrackerStateEstimatorSVM(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Detail_TrackerStateEstimatorSVM::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Detail_TrackerStateEstimatorSVM]
	pub trait Detail_TrackerStateEstimatorSVMTraitConst: crate::tracking::Detail_TrackerStateEstimatorTraitConst {
		fn as_raw_Detail_TrackerStateEstimatorSVM(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerStateEstimatorSVM]
	pub trait Detail_TrackerStateEstimatorSVMTrait: crate::tracking::Detail_TrackerStateEstimatorSVMTraitConst + crate::tracking::Detail_TrackerStateEstimatorTrait {
		fn as_raw_mut_Detail_TrackerStateEstimatorSVM(&mut self) -> *mut c_void;

	}

	impl std::fmt::Debug for Detail_TrackerStateEstimatorSVM {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerStateEstimatorSVM")
				.finish()
		}
	}

	boxed_cast_base! { Detail_TrackerStateEstimatorSVM, crate::tracking::Detail_TrackerStateEstimator, cv_detail_TrackerStateEstimatorSVM_to_Detail_TrackerStateEstimator }

	impl crate::tracking::Detail_TrackerStateEstimatorTraitConst for Detail_TrackerStateEstimatorSVM {
		#[inline] fn as_raw_Detail_TrackerStateEstimator(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerStateEstimatorTrait for Detail_TrackerStateEstimatorSVM {
		#[inline] fn as_raw_mut_Detail_TrackerStateEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerStateEstimatorSVM, crate::tracking::Detail_TrackerStateEstimatorTraitConst, as_raw_Detail_TrackerStateEstimator, crate::tracking::Detail_TrackerStateEstimatorTrait, as_raw_mut_Detail_TrackerStateEstimator }

	impl crate::tracking::Detail_TrackerStateEstimatorSVMTraitConst for Detail_TrackerStateEstimatorSVM {
		#[inline] fn as_raw_Detail_TrackerStateEstimatorSVM(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerStateEstimatorSVMTrait for Detail_TrackerStateEstimatorSVM {
		#[inline] fn as_raw_mut_Detail_TrackerStateEstimatorSVM(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerStateEstimatorSVM, crate::tracking::Detail_TrackerStateEstimatorSVMTraitConst, as_raw_Detail_TrackerStateEstimatorSVM, crate::tracking::Detail_TrackerStateEstimatorSVMTrait, as_raw_mut_Detail_TrackerStateEstimatorSVM }

	/// Abstract base class for TrackerTargetState that represents a possible state of the target.
	///
	/// See [AAM](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_AAM) ![inline formula](https://latex.codecogs.com/png.latex?%5Chat%7Bx%7D%5E%7Bi%7D%5F%7Bk%7D) all the states candidates.
	///
	/// Inherits this class with your Target state, In own implementation you can add scale variation,
	/// width, height, orientation, etc.
	pub struct Detail_TrackerTargetState {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Detail_TrackerTargetState }

	impl Drop for Detail_TrackerTargetState {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_TrackerTargetState_delete(self.as_raw_mut_Detail_TrackerTargetState()) };
		}
	}

	unsafe impl Send for Detail_TrackerTargetState {}

	impl Detail_TrackerTargetState {
		/// Creates a default instance of the class by calling the default constructor
		#[inline]
		pub fn default() -> crate::tracking::Detail_TrackerTargetState {
			let ret = unsafe { sys::cv_detail_TrackerTargetState_defaultNew_const() };
			let ret = unsafe { crate::tracking::Detail_TrackerTargetState::opencv_from_extern(ret) };
			ret
		}

	}

	/// Constant methods for [crate::tracking::Detail_TrackerTargetState]
	pub trait Detail_TrackerTargetStateTraitConst {
		fn as_raw_Detail_TrackerTargetState(&self) -> *const c_void;

		/// Get the position
		/// ## Returns
		/// The position
		#[inline]
		fn get_target_position(&self) -> Result<core::Point2f> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerTargetState_getTargetPosition_const(self.as_raw_Detail_TrackerTargetState(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get the width of the target
		/// ## Returns
		/// The width of the target
		#[inline]
		fn get_target_width(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerTargetState_getTargetWidth_const(self.as_raw_Detail_TrackerTargetState(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get the height of the target
		/// ## Returns
		/// The height of the target
		#[inline]
		fn get_target_height(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerTargetState_getTargetHeight_const(self.as_raw_Detail_TrackerTargetState(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::tracking::Detail_TrackerTargetState]
	pub trait Detail_TrackerTargetStateTrait: crate::tracking::Detail_TrackerTargetStateTraitConst {
		fn as_raw_mut_Detail_TrackerTargetState(&mut self) -> *mut c_void;

		/// Set the position
		/// ## Parameters
		/// * position: The position
		#[inline]
		fn set_target_position(&mut self, position: core::Point2f) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerTargetState_setTargetPosition_const_Point2fR(self.as_raw_mut_Detail_TrackerTargetState(), &position, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set the width of the target
		/// ## Parameters
		/// * width: The width of the target
		#[inline]
		fn set_target_width(&mut self, width: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerTargetState_setTargetWidth_int(self.as_raw_mut_Detail_TrackerTargetState(), width, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set the height of the target
		/// ## Parameters
		/// * height: The height of the target
		#[inline]
		fn set_target_height(&mut self, height: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TrackerTargetState_setTargetHeight_int(self.as_raw_mut_Detail_TrackerTargetState(), height, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl Default for Detail_TrackerTargetState {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
		}
	}

	impl std::fmt::Debug for Detail_TrackerTargetState {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TrackerTargetState")
				.finish()
		}
	}

	impl crate::tracking::Detail_TrackerTargetStateTraitConst for Detail_TrackerTargetState {
		#[inline] fn as_raw_Detail_TrackerTargetState(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Detail_TrackerTargetStateTrait for Detail_TrackerTargetState {
		#[inline] fn as_raw_mut_Detail_TrackerTargetState(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Detail_TrackerTargetState, crate::tracking::Detail_TrackerTargetStateTraitConst, as_raw_Detail_TrackerTargetState, crate::tracking::Detail_TrackerTargetStateTrait, as_raw_mut_Detail_TrackerTargetState }

	/// ********************************** MultiTracker Class ---By Laksono Kurnianggoro---) ***********************************
	/// This class is used to track multiple objects using the specified tracker algorithm.
	///
	/// * The %MultiTracker is naive implementation of multiple object tracking.
	/// * It process the tracked objects independently without any optimization accross the tracked objects.
	pub struct Legacy_MultiTracker {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Legacy_MultiTracker }

	impl Drop for Legacy_MultiTracker {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_legacy_MultiTracker_delete(self.as_raw_mut_Legacy_MultiTracker()) };
		}
	}

	unsafe impl Send for Legacy_MultiTracker {}

	impl Legacy_MultiTracker {
		/// \brief Constructor.
		#[inline]
		pub fn default() -> Result<crate::tracking::Legacy_MultiTracker> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_MultiTracker_MultiTracker(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Legacy_MultiTracker::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// \brief Returns a pointer to a new instance of MultiTracker
		#[inline]
		pub fn create() -> Result<core::Ptr<crate::tracking::Legacy_MultiTracker>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_MultiTracker_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::tracking::Legacy_MultiTracker>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Legacy_MultiTracker]
	pub trait Legacy_MultiTrackerTraitConst: core::AlgorithmTraitConst {
		fn as_raw_Legacy_MultiTracker(&self) -> *const c_void;

		/// \brief Returns a reference to a storage for the tracked objects, each object corresponds to one tracker algorithm
		#[inline]
		fn get_objects(&self) -> Result<core::Vector<core::Rect2d>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_MultiTracker_getObjects_const(self.as_raw_Legacy_MultiTracker(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Rect2d>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::tracking::Legacy_MultiTracker]
	pub trait Legacy_MultiTrackerTrait: core::AlgorithmTrait + crate::tracking::Legacy_MultiTrackerTraitConst {
		fn as_raw_mut_Legacy_MultiTracker(&mut self) -> *mut c_void;

		/// \brief Add a new object to be tracked.
		///
		/// ## Parameters
		/// * newTracker: tracking algorithm to be used
		/// * image: input image
		/// * boundingBox: a rectangle represents ROI of the tracked object
		#[inline]
		fn add(&mut self, mut new_tracker: core::Ptr<crate::tracking::Legacy_Tracker>, image: &impl ToInputArray, bounding_box: core::Rect2d) -> Result<bool> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_MultiTracker_add_PtrLTrackerG_const__InputArrayR_const_Rect2dR(self.as_raw_mut_Legacy_MultiTracker(), new_tracker.as_raw_mut_PtrOfLegacy_Tracker(), image.as_raw__InputArray(), &bounding_box, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// \brief Add a set of objects to be tracked.
		/// ## Parameters
		/// * newTrackers: list of tracking algorithms to be used
		/// * image: input image
		/// * boundingBox: list of the tracked objects
		#[inline]
		fn add_1(&mut self, mut new_trackers: core::Vector<core::Ptr<crate::tracking::Legacy_Tracker>>, image: &impl ToInputArray, mut bounding_box: core::Vector<core::Rect2d>) -> Result<bool> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_MultiTracker_add_vectorLPtrLTrackerGG_const__InputArrayR_vectorLRect2dG(self.as_raw_mut_Legacy_MultiTracker(), new_trackers.as_raw_mut_VectorOfPtrOfLegacy_Tracker(), image.as_raw__InputArray(), bounding_box.as_raw_mut_VectorOfRect2d(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// \brief Update the current tracking status.
		/// The result will be saved in the internal storage.
		/// ## Parameters
		/// * image: input image
		#[inline]
		fn update(&mut self, image: &impl ToInputArray) -> Result<bool> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_MultiTracker_update_const__InputArrayR(self.as_raw_mut_Legacy_MultiTracker(), image.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// \brief Update the current tracking status.
		/// ## Parameters
		/// * image: input image
		/// * boundingBox: the tracking result, represent a list of ROIs of the tracked objects.
		#[inline]
		fn update_1(&mut self, image: &impl ToInputArray, bounding_box: &mut core::Vector<core::Rect2d>) -> Result<bool> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_MultiTracker_update_const__InputArrayR_vectorLRect2dGR(self.as_raw_mut_Legacy_MultiTracker(), image.as_raw__InputArray(), bounding_box.as_raw_mut_VectorOfRect2d(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Legacy_MultiTracker {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Legacy_MultiTracker")
				.finish()
		}
	}

	boxed_cast_base! { Legacy_MultiTracker, core::Algorithm, cv_legacy_MultiTracker_to_Algorithm }

	impl core::AlgorithmTraitConst for Legacy_MultiTracker {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for Legacy_MultiTracker {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_MultiTracker, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::tracking::Legacy_MultiTrackerTraitConst for Legacy_MultiTracker {
		#[inline] fn as_raw_Legacy_MultiTracker(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Legacy_MultiTrackerTrait for Legacy_MultiTracker {
		#[inline] fn as_raw_mut_Legacy_MultiTracker(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_MultiTracker, crate::tracking::Legacy_MultiTrackerTraitConst, as_raw_Legacy_MultiTracker, crate::tracking::Legacy_MultiTrackerTrait, as_raw_mut_Legacy_MultiTracker }

	/// Multi Object %Tracker for TLD.
	///
	/// TLD is a novel tracking framework that explicitly decomposes
	/// the long-term tracking task into tracking, learning and detection.
	///
	/// The tracker follows the object from frame to frame. The detector localizes all appearances that
	/// have been observed so far and corrects the tracker if necessary. The learning estimates detector's
	/// errors and updates it to avoid these errors in the future. The implementation is based on [TLD](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_TLD) .
	///
	/// The Median Flow algorithm (see cv::TrackerMedianFlow) was chosen as a tracking component in this
	/// implementation, following authors. The tracker is supposed to be able to handle rapid motions, partial
	/// occlusions, object absence etc.
	/// ## See also
	/// Tracker, MultiTracker, TrackerTLD
	pub struct Legacy_MultiTrackerTLD {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Legacy_MultiTrackerTLD }

	impl Drop for Legacy_MultiTrackerTLD {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_legacy_MultiTrackerTLD_delete(self.as_raw_mut_Legacy_MultiTrackerTLD()) };
		}
	}

	unsafe impl Send for Legacy_MultiTrackerTLD {}

	impl Legacy_MultiTrackerTLD {
		/// Creates a default instance of the class by calling the default constructor
		#[inline]
		pub fn default() -> crate::tracking::Legacy_MultiTrackerTLD {
			let ret = unsafe { sys::cv_legacy_MultiTrackerTLD_defaultNew_const() };
			let ret = unsafe { crate::tracking::Legacy_MultiTrackerTLD::opencv_from_extern(ret) };
			ret
		}

	}

	/// Constant methods for [crate::tracking::Legacy_MultiTrackerTLD]
	pub trait Legacy_MultiTrackerTLDTraitConst: crate::tracking::Legacy_MultiTracker_AltTraitConst {
		fn as_raw_Legacy_MultiTrackerTLD(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::tracking::Legacy_MultiTrackerTLD]
	pub trait Legacy_MultiTrackerTLDTrait: crate::tracking::Legacy_MultiTrackerTLDTraitConst + crate::tracking::Legacy_MultiTracker_AltTrait {
		fn as_raw_mut_Legacy_MultiTrackerTLD(&mut self) -> *mut c_void;

		/// Update all trackers from the tracking-list, find a new most likely bounding boxes for the targets by
		/// optimized update method using some techniques to speedup calculations specifically for MO TLD. The only limitation
		/// is that all target bounding boxes should have approximately same aspect ratios. Speed boost is around 20%
		///
		/// ## Parameters
		/// * image: The current frame.
		///
		/// ## Returns
		/// True means that all targets were located and false means that tracker couldn't locate one of the targets in
		/// current frame. Note, that latter *does not* imply that tracker has failed, maybe target is indeed
		/// missing from the frame (say, out of sight)
		#[inline]
		fn update_opt(&mut self, image: &impl ToInputArray) -> Result<bool> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_MultiTrackerTLD_update_opt_const__InputArrayR(self.as_raw_mut_Legacy_MultiTrackerTLD(), image.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl Default for Legacy_MultiTrackerTLD {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
		}
	}

	impl std::fmt::Debug for Legacy_MultiTrackerTLD {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Legacy_MultiTrackerTLD")
				.field("target_num", &crate::tracking::Legacy_MultiTracker_AltTraitConst::target_num(self))
				.field("trackers", &crate::tracking::Legacy_MultiTracker_AltTraitConst::trackers(self))
				.field("bounding_boxes", &crate::tracking::Legacy_MultiTracker_AltTraitConst::bounding_boxes(self))
				.field("colors", &crate::tracking::Legacy_MultiTracker_AltTraitConst::colors(self))
				.finish()
		}
	}

	boxed_cast_base! { Legacy_MultiTrackerTLD, crate::tracking::Legacy_MultiTracker_Alt, cv_legacy_MultiTrackerTLD_to_Legacy_MultiTracker_Alt }

	impl crate::tracking::Legacy_MultiTracker_AltTraitConst for Legacy_MultiTrackerTLD {
		#[inline] fn as_raw_Legacy_MultiTracker_Alt(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Legacy_MultiTracker_AltTrait for Legacy_MultiTrackerTLD {
		#[inline] fn as_raw_mut_Legacy_MultiTracker_Alt(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_MultiTrackerTLD, crate::tracking::Legacy_MultiTracker_AltTraitConst, as_raw_Legacy_MultiTracker_Alt, crate::tracking::Legacy_MultiTracker_AltTrait, as_raw_mut_Legacy_MultiTracker_Alt }

	impl crate::tracking::Legacy_MultiTrackerTLDTraitConst for Legacy_MultiTrackerTLD {
		#[inline] fn as_raw_Legacy_MultiTrackerTLD(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Legacy_MultiTrackerTLDTrait for Legacy_MultiTrackerTLD {
		#[inline] fn as_raw_mut_Legacy_MultiTrackerTLD(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_MultiTrackerTLD, crate::tracking::Legacy_MultiTrackerTLDTraitConst, as_raw_Legacy_MultiTrackerTLD, crate::tracking::Legacy_MultiTrackerTLDTrait, as_raw_mut_Legacy_MultiTrackerTLD }

	/// Base abstract class for the long-term Multi Object Trackers:
	/// ## See also
	/// Tracker, MultiTrackerTLD
	pub struct Legacy_MultiTracker_Alt {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Legacy_MultiTracker_Alt }

	impl Drop for Legacy_MultiTracker_Alt {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_legacy_MultiTracker_Alt_delete(self.as_raw_mut_Legacy_MultiTracker_Alt()) };
		}
	}

	unsafe impl Send for Legacy_MultiTracker_Alt {}

	impl Legacy_MultiTracker_Alt {
		/// Constructor for Multitracker
		#[inline]
		pub fn default() -> Result<crate::tracking::Legacy_MultiTracker_Alt> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_MultiTracker_Alt_MultiTracker_Alt(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Legacy_MultiTracker_Alt::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Legacy_MultiTracker_Alt]
	pub trait Legacy_MultiTracker_AltTraitConst {
		fn as_raw_Legacy_MultiTracker_Alt(&self) -> *const c_void;

		/// Current number of targets in tracking-list
		#[inline]
		fn target_num(&self) -> i32 {
			let ret = unsafe { sys::cv_legacy_MultiTracker_Alt_propTargetNum_const(self.as_raw_Legacy_MultiTracker_Alt()) };
			ret
		}

		/// Trackers list for Multi-Object-Tracker
		#[inline]
		fn trackers(&self) -> core::Vector<core::Ptr<crate::tracking::Legacy_Tracker>> {
			let ret = unsafe { sys::cv_legacy_MultiTracker_Alt_propTrackers_const(self.as_raw_Legacy_MultiTracker_Alt()) };
			let ret = unsafe { core::Vector::<core::Ptr<crate::tracking::Legacy_Tracker>>::opencv_from_extern(ret) };
			ret
		}

		/// Bounding Boxes list for Multi-Object-Tracker
		#[inline]
		fn bounding_boxes(&self) -> core::Vector<core::Rect2d> {
			let ret = unsafe { sys::cv_legacy_MultiTracker_Alt_propBoundingBoxes_const(self.as_raw_Legacy_MultiTracker_Alt()) };
			let ret = unsafe { core::Vector::<core::Rect2d>::opencv_from_extern(ret) };
			ret
		}

		/// List of randomly generated colors for bounding boxes display
		#[inline]
		fn colors(&self) -> core::Vector<core::Scalar> {
			let ret = unsafe { sys::cv_legacy_MultiTracker_Alt_propColors_const(self.as_raw_Legacy_MultiTracker_Alt()) };
			let ret = unsafe { core::Vector::<core::Scalar>::opencv_from_extern(ret) };
			ret
		}

	}

	/// Mutable methods for [crate::tracking::Legacy_MultiTracker_Alt]
	pub trait Legacy_MultiTracker_AltTrait: crate::tracking::Legacy_MultiTracker_AltTraitConst {
		fn as_raw_mut_Legacy_MultiTracker_Alt(&mut self) -> *mut c_void;

		/// Current number of targets in tracking-list
		#[inline]
		fn set_target_num(&mut self, val: i32) {
			let ret = unsafe { sys::cv_legacy_MultiTracker_Alt_propTargetNum_const_int(self.as_raw_mut_Legacy_MultiTracker_Alt(), val) };
			ret
		}

		/// Trackers list for Multi-Object-Tracker
		#[inline]
		fn set_trackers(&mut self, val: core::Vector<core::Ptr<crate::tracking::Legacy_Tracker>>) {
			let ret = unsafe { sys::cv_legacy_MultiTracker_Alt_propTrackers_const_vectorLPtrLTrackerGG(self.as_raw_mut_Legacy_MultiTracker_Alt(), val.as_raw_VectorOfPtrOfLegacy_Tracker()) };
			ret
		}

		/// Bounding Boxes list for Multi-Object-Tracker
		#[inline]
		fn set_bounding_boxes(&mut self, val: core::Vector<core::Rect2d>) {
			let ret = unsafe { sys::cv_legacy_MultiTracker_Alt_propBoundingBoxes_const_vectorLRect2dG(self.as_raw_mut_Legacy_MultiTracker_Alt(), val.as_raw_VectorOfRect2d()) };
			ret
		}

		/// List of randomly generated colors for bounding boxes display
		#[inline]
		fn set_colors(&mut self, val: core::Vector<core::Scalar>) {
			let ret = unsafe { sys::cv_legacy_MultiTracker_Alt_propColors_const_vectorLScalarG(self.as_raw_mut_Legacy_MultiTracker_Alt(), val.as_raw_VectorOfScalar()) };
			ret
		}

		/// Add a new target to a tracking-list and initialize the tracker with a known bounding box that surrounded the target
		/// ## Parameters
		/// * image: The initial frame
		/// * boundingBox: The initial bounding box of target
		/// * tracker_algorithm: Multi-tracker algorithm
		///
		/// ## Returns
		/// True if new target initialization went succesfully, false otherwise
		#[inline]
		fn add_target(&mut self, image: &impl ToInputArray, bounding_box: core::Rect2d, mut tracker_algorithm: core::Ptr<crate::tracking::Legacy_Tracker>) -> Result<bool> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_MultiTracker_Alt_addTarget_const__InputArrayR_const_Rect2dR_PtrLTrackerG(self.as_raw_mut_Legacy_MultiTracker_Alt(), image.as_raw__InputArray(), &bounding_box, tracker_algorithm.as_raw_mut_PtrOfLegacy_Tracker(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Update all trackers from the tracking-list, find a new most likely bounding boxes for the targets
		/// ## Parameters
		/// * image: The current frame
		///
		/// ## Returns
		/// True means that all targets were located and false means that tracker couldn't locate one of the targets in
		/// current frame. Note, that latter *does not* imply that tracker has failed, maybe target is indeed
		/// missing from the frame (say, out of sight)
		#[inline]
		fn update(&mut self, image: &impl ToInputArray) -> Result<bool> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_MultiTracker_Alt_update_const__InputArrayR(self.as_raw_mut_Legacy_MultiTracker_Alt(), image.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Legacy_MultiTracker_Alt {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Legacy_MultiTracker_Alt")
				.field("target_num", &crate::tracking::Legacy_MultiTracker_AltTraitConst::target_num(self))
				.field("trackers", &crate::tracking::Legacy_MultiTracker_AltTraitConst::trackers(self))
				.field("bounding_boxes", &crate::tracking::Legacy_MultiTracker_AltTraitConst::bounding_boxes(self))
				.field("colors", &crate::tracking::Legacy_MultiTracker_AltTraitConst::colors(self))
				.finish()
		}
	}

	impl crate::tracking::Legacy_MultiTracker_AltTraitConst for Legacy_MultiTracker_Alt {
		#[inline] fn as_raw_Legacy_MultiTracker_Alt(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Legacy_MultiTracker_AltTrait for Legacy_MultiTracker_Alt {
		#[inline] fn as_raw_mut_Legacy_MultiTracker_Alt(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_MultiTracker_Alt, crate::tracking::Legacy_MultiTracker_AltTraitConst, as_raw_Legacy_MultiTracker_Alt, crate::tracking::Legacy_MultiTracker_AltTrait, as_raw_mut_Legacy_MultiTracker_Alt }

	/// Base abstract class for the long-term tracker:
	pub struct Legacy_Tracker {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Legacy_Tracker }

	impl Drop for Legacy_Tracker {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_legacy_Tracker_delete(self.as_raw_mut_Legacy_Tracker()) };
		}
	}

	unsafe impl Send for Legacy_Tracker {}

	/// Constant methods for [crate::tracking::Legacy_Tracker]
	pub trait Legacy_TrackerTraitConst: core::AlgorithmTraitConst {
		fn as_raw_Legacy_Tracker(&self) -> *const c_void;

		#[inline]
		fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_Tracker_write_const_FileStorageR(self.as_raw_Legacy_Tracker(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::tracking::Legacy_Tracker]
	pub trait Legacy_TrackerTrait: core::AlgorithmTrait + crate::tracking::Legacy_TrackerTraitConst {
		fn as_raw_mut_Legacy_Tracker(&mut self) -> *mut c_void;

		/// Initialize the tracker with a known bounding box that surrounded the target
		/// ## Parameters
		/// * image: The initial frame
		/// * boundingBox: The initial bounding box
		///
		/// ## Returns
		/// True if initialization went succesfully, false otherwise
		#[inline]
		fn init(&mut self, image: &impl ToInputArray, bounding_box: core::Rect2d) -> Result<bool> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_Tracker_init_const__InputArrayR_const_Rect2dR(self.as_raw_mut_Legacy_Tracker(), image.as_raw__InputArray(), &bounding_box, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Update the tracker, find the new most likely bounding box for the target
		/// ## Parameters
		/// * image: The current frame
		/// * boundingBox: The bounding box that represent the new target location, if true was returned, not
		/// modified otherwise
		///
		/// ## Returns
		/// True means that target was located and false means that tracker cannot locate target in
		/// current frame. Note, that latter *does not* imply that tracker has failed, maybe target is indeed
		/// missing from the frame (say, out of sight)
		#[inline]
		fn update(&mut self, image: &impl ToInputArray, bounding_box: &mut core::Rect2d) -> Result<bool> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_Tracker_update_const__InputArrayR_Rect2dR(self.as_raw_mut_Legacy_Tracker(), image.as_raw__InputArray(), bounding_box, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn read(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_Tracker_read_const_FileNodeR(self.as_raw_mut_Legacy_Tracker(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Legacy_Tracker {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Legacy_Tracker")
				.finish()
		}
	}

	boxed_cast_base! { Legacy_Tracker, core::Algorithm, cv_legacy_Tracker_to_Algorithm }

	boxed_cast_descendant! { Legacy_Tracker, crate::tracking::Legacy_TrackerBoosting, cv_legacy_Tracker_to_Legacy_TrackerBoosting }

	boxed_cast_descendant! { Legacy_Tracker, crate::tracking::Legacy_TrackerCSRT, cv_legacy_Tracker_to_Legacy_TrackerCSRT }

	boxed_cast_descendant! { Legacy_Tracker, crate::tracking::Legacy_TrackerKCF, cv_legacy_Tracker_to_Legacy_TrackerKCF }

	boxed_cast_descendant! { Legacy_Tracker, crate::tracking::Legacy_TrackerMIL, cv_legacy_Tracker_to_Legacy_TrackerMIL }

	boxed_cast_descendant! { Legacy_Tracker, crate::tracking::Legacy_TrackerMOSSE, cv_legacy_Tracker_to_Legacy_TrackerMOSSE }

	boxed_cast_descendant! { Legacy_Tracker, crate::tracking::Legacy_TrackerMedianFlow, cv_legacy_Tracker_to_Legacy_TrackerMedianFlow }

	boxed_cast_descendant! { Legacy_Tracker, crate::tracking::Legacy_TrackerTLD, cv_legacy_Tracker_to_Legacy_TrackerTLD }

	impl core::AlgorithmTraitConst for Legacy_Tracker {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for Legacy_Tracker {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_Tracker, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::tracking::Legacy_TrackerTraitConst for Legacy_Tracker {
		#[inline] fn as_raw_Legacy_Tracker(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Legacy_TrackerTrait for Legacy_Tracker {
		#[inline] fn as_raw_mut_Legacy_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_Tracker, crate::tracking::Legacy_TrackerTraitConst, as_raw_Legacy_Tracker, crate::tracking::Legacy_TrackerTrait, as_raw_mut_Legacy_Tracker }

	/// the Boosting tracker
	///
	/// This is a real-time object tracking based on a novel on-line version of the AdaBoost algorithm.
	/// The classifier uses the surrounding background as negative examples in update step to avoid the
	/// drifting problem. The implementation is based on [OLB](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_OLB) .
	pub struct Legacy_TrackerBoosting {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Legacy_TrackerBoosting }

	impl Drop for Legacy_TrackerBoosting {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_legacy_TrackerBoosting_delete(self.as_raw_mut_Legacy_TrackerBoosting()) };
		}
	}

	unsafe impl Send for Legacy_TrackerBoosting {}

	impl Legacy_TrackerBoosting {
		/// Constructor
		/// ## Parameters
		/// * parameters: BOOSTING parameters TrackerBoosting::Params
		#[inline]
		pub fn create(parameters: &impl crate::tracking::Legacy_TrackerBoosting_ParamsTraitConst) -> Result<core::Ptr<crate::tracking::Legacy_TrackerBoosting>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerBoosting_create_const_ParamsR(parameters.as_raw_Legacy_TrackerBoosting_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::tracking::Legacy_TrackerBoosting>::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		pub fn create_1() -> Result<core::Ptr<crate::tracking::Legacy_TrackerBoosting>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerBoosting_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::tracking::Legacy_TrackerBoosting>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Legacy_TrackerBoosting]
	pub trait Legacy_TrackerBoostingTraitConst: crate::tracking::Legacy_TrackerTraitConst {
		fn as_raw_Legacy_TrackerBoosting(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::tracking::Legacy_TrackerBoosting]
	pub trait Legacy_TrackerBoostingTrait: crate::tracking::Legacy_TrackerBoostingTraitConst + crate::tracking::Legacy_TrackerTrait {
		fn as_raw_mut_Legacy_TrackerBoosting(&mut self) -> *mut c_void;

	}

	impl std::fmt::Debug for Legacy_TrackerBoosting {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Legacy_TrackerBoosting")
				.finish()
		}
	}

	boxed_cast_base! { Legacy_TrackerBoosting, core::Algorithm, cv_legacy_TrackerBoosting_to_Algorithm }

	boxed_cast_base! { Legacy_TrackerBoosting, crate::tracking::Legacy_Tracker, cv_legacy_TrackerBoosting_to_Legacy_Tracker }

	impl core::AlgorithmTraitConst for Legacy_TrackerBoosting {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for Legacy_TrackerBoosting {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerBoosting, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::tracking::Legacy_TrackerTraitConst for Legacy_TrackerBoosting {
		#[inline] fn as_raw_Legacy_Tracker(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Legacy_TrackerTrait for Legacy_TrackerBoosting {
		#[inline] fn as_raw_mut_Legacy_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerBoosting, crate::tracking::Legacy_TrackerTraitConst, as_raw_Legacy_Tracker, crate::tracking::Legacy_TrackerTrait, as_raw_mut_Legacy_Tracker }

	impl crate::tracking::Legacy_TrackerBoostingTraitConst for Legacy_TrackerBoosting {
		#[inline] fn as_raw_Legacy_TrackerBoosting(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Legacy_TrackerBoostingTrait for Legacy_TrackerBoosting {
		#[inline] fn as_raw_mut_Legacy_TrackerBoosting(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerBoosting, crate::tracking::Legacy_TrackerBoostingTraitConst, as_raw_Legacy_TrackerBoosting, crate::tracking::Legacy_TrackerBoostingTrait, as_raw_mut_Legacy_TrackerBoosting }

	pub struct Legacy_TrackerBoosting_Params {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Legacy_TrackerBoosting_Params }

	impl Drop for Legacy_TrackerBoosting_Params {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_legacy_TrackerBoosting_Params_delete(self.as_raw_mut_Legacy_TrackerBoosting_Params()) };
		}
	}

	unsafe impl Send for Legacy_TrackerBoosting_Params {}

	impl Legacy_TrackerBoosting_Params {
		#[inline]
		pub fn default() -> Result<crate::tracking::Legacy_TrackerBoosting_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerBoosting_Params_Params(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Legacy_TrackerBoosting_Params::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Legacy_TrackerBoosting_Params]
	pub trait Legacy_TrackerBoosting_ParamsTraitConst {
		fn as_raw_Legacy_TrackerBoosting_Params(&self) -> *const c_void;

		/// the number of classifiers to use in a OnlineBoosting algorithm
		#[inline]
		fn num_classifiers(&self) -> i32 {
			let ret = unsafe { sys::cv_legacy_TrackerBoosting_Params_propNumClassifiers_const(self.as_raw_Legacy_TrackerBoosting_Params()) };
			ret
		}

		/// search region parameters to use in a OnlineBoosting algorithm
		#[inline]
		fn sampler_overlap(&self) -> f32 {
			let ret = unsafe { sys::cv_legacy_TrackerBoosting_Params_propSamplerOverlap_const(self.as_raw_Legacy_TrackerBoosting_Params()) };
			ret
		}

		/// search region parameters to use in a OnlineBoosting algorithm
		#[inline]
		fn sampler_search_factor(&self) -> f32 {
			let ret = unsafe { sys::cv_legacy_TrackerBoosting_Params_propSamplerSearchFactor_const(self.as_raw_Legacy_TrackerBoosting_Params()) };
			ret
		}

		/// the initial iterations
		#[inline]
		fn iteration_init(&self) -> i32 {
			let ret = unsafe { sys::cv_legacy_TrackerBoosting_Params_propIterationInit_const(self.as_raw_Legacy_TrackerBoosting_Params()) };
			ret
		}

		/// # features
		#[inline]
		fn feature_set_num_features(&self) -> i32 {
			let ret = unsafe { sys::cv_legacy_TrackerBoosting_Params_propFeatureSetNumFeatures_const(self.as_raw_Legacy_TrackerBoosting_Params()) };
			ret
		}

		/// \brief Write parameters to a file
		#[inline]
		fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerBoosting_Params_write_const_FileStorageR(self.as_raw_Legacy_TrackerBoosting_Params(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::tracking::Legacy_TrackerBoosting_Params]
	pub trait Legacy_TrackerBoosting_ParamsTrait: crate::tracking::Legacy_TrackerBoosting_ParamsTraitConst {
		fn as_raw_mut_Legacy_TrackerBoosting_Params(&mut self) -> *mut c_void;

		/// the number of classifiers to use in a OnlineBoosting algorithm
		#[inline]
		fn set_num_classifiers(&mut self, val: i32) {
			let ret = unsafe { sys::cv_legacy_TrackerBoosting_Params_propNumClassifiers_const_int(self.as_raw_mut_Legacy_TrackerBoosting_Params(), val) };
			ret
		}

		/// search region parameters to use in a OnlineBoosting algorithm
		#[inline]
		fn set_sampler_overlap(&mut self, val: f32) {
			let ret = unsafe { sys::cv_legacy_TrackerBoosting_Params_propSamplerOverlap_const_float(self.as_raw_mut_Legacy_TrackerBoosting_Params(), val) };
			ret
		}

		/// search region parameters to use in a OnlineBoosting algorithm
		#[inline]
		fn set_sampler_search_factor(&mut self, val: f32) {
			let ret = unsafe { sys::cv_legacy_TrackerBoosting_Params_propSamplerSearchFactor_const_float(self.as_raw_mut_Legacy_TrackerBoosting_Params(), val) };
			ret
		}

		/// the initial iterations
		#[inline]
		fn set_iteration_init(&mut self, val: i32) {
			let ret = unsafe { sys::cv_legacy_TrackerBoosting_Params_propIterationInit_const_int(self.as_raw_mut_Legacy_TrackerBoosting_Params(), val) };
			ret
		}

		/// # features
		#[inline]
		fn set_feature_set_num_features(&mut self, val: i32) {
			let ret = unsafe { sys::cv_legacy_TrackerBoosting_Params_propFeatureSetNumFeatures_const_int(self.as_raw_mut_Legacy_TrackerBoosting_Params(), val) };
			ret
		}

		/// \brief Read parameters from a file
		#[inline]
		fn read(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerBoosting_Params_read_const_FileNodeR(self.as_raw_mut_Legacy_TrackerBoosting_Params(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Legacy_TrackerBoosting_Params {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Legacy_TrackerBoosting_Params")
				.field("num_classifiers", &crate::tracking::Legacy_TrackerBoosting_ParamsTraitConst::num_classifiers(self))
				.field("sampler_overlap", &crate::tracking::Legacy_TrackerBoosting_ParamsTraitConst::sampler_overlap(self))
				.field("sampler_search_factor", &crate::tracking::Legacy_TrackerBoosting_ParamsTraitConst::sampler_search_factor(self))
				.field("iteration_init", &crate::tracking::Legacy_TrackerBoosting_ParamsTraitConst::iteration_init(self))
				.field("feature_set_num_features", &crate::tracking::Legacy_TrackerBoosting_ParamsTraitConst::feature_set_num_features(self))
				.finish()
		}
	}

	impl crate::tracking::Legacy_TrackerBoosting_ParamsTraitConst for Legacy_TrackerBoosting_Params {
		#[inline] fn as_raw_Legacy_TrackerBoosting_Params(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Legacy_TrackerBoosting_ParamsTrait for Legacy_TrackerBoosting_Params {
		#[inline] fn as_raw_mut_Legacy_TrackerBoosting_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerBoosting_Params, crate::tracking::Legacy_TrackerBoosting_ParamsTraitConst, as_raw_Legacy_TrackerBoosting_Params, crate::tracking::Legacy_TrackerBoosting_ParamsTrait, as_raw_mut_Legacy_TrackerBoosting_Params }

	/// ********************************* CSRT ***********************************
	/// the CSRT tracker
	///
	/// The implementation is based on [Lukezic_IJCV2018](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_Lukezic_IJCV2018) Discriminative Correlation Filter with Channel and Spatial Reliability
	pub struct Legacy_TrackerCSRT {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Legacy_TrackerCSRT }

	impl Drop for Legacy_TrackerCSRT {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_legacy_TrackerCSRT_delete(self.as_raw_mut_Legacy_TrackerCSRT()) };
		}
	}

	unsafe impl Send for Legacy_TrackerCSRT {}

	impl Legacy_TrackerCSRT {
		/// Constructor
		/// ## Parameters
		/// * parameters: CSRT parameters TrackerCSRT::Params
		#[inline]
		pub fn create(parameters: &impl crate::tracking::Legacy_TrackerCSRT_ParamsTraitConst) -> Result<core::Ptr<crate::tracking::Legacy_TrackerCSRT>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerCSRT_create_const_ParamsR(parameters.as_raw_Legacy_TrackerCSRT_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::tracking::Legacy_TrackerCSRT>::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		pub fn create_1() -> Result<core::Ptr<crate::tracking::Legacy_TrackerCSRT>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerCSRT_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::tracking::Legacy_TrackerCSRT>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Legacy_TrackerCSRT]
	pub trait Legacy_TrackerCSRTTraitConst: crate::tracking::Legacy_TrackerTraitConst {
		fn as_raw_Legacy_TrackerCSRT(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::tracking::Legacy_TrackerCSRT]
	pub trait Legacy_TrackerCSRTTrait: crate::tracking::Legacy_TrackerCSRTTraitConst + crate::tracking::Legacy_TrackerTrait {
		fn as_raw_mut_Legacy_TrackerCSRT(&mut self) -> *mut c_void;

		#[inline]
		fn set_initial_mask(&mut self, mask: &impl ToInputArray) -> Result<()> {
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerCSRT_setInitialMask_const__InputArrayR(self.as_raw_mut_Legacy_TrackerCSRT(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Legacy_TrackerCSRT {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Legacy_TrackerCSRT")
				.finish()
		}
	}

	boxed_cast_base! { Legacy_TrackerCSRT, core::Algorithm, cv_legacy_TrackerCSRT_to_Algorithm }

	boxed_cast_base! { Legacy_TrackerCSRT, crate::tracking::Legacy_Tracker, cv_legacy_TrackerCSRT_to_Legacy_Tracker }

	impl core::AlgorithmTraitConst for Legacy_TrackerCSRT {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for Legacy_TrackerCSRT {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerCSRT, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::tracking::Legacy_TrackerTraitConst for Legacy_TrackerCSRT {
		#[inline] fn as_raw_Legacy_Tracker(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Legacy_TrackerTrait for Legacy_TrackerCSRT {
		#[inline] fn as_raw_mut_Legacy_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerCSRT, crate::tracking::Legacy_TrackerTraitConst, as_raw_Legacy_Tracker, crate::tracking::Legacy_TrackerTrait, as_raw_mut_Legacy_Tracker }

	impl crate::tracking::Legacy_TrackerCSRTTraitConst for Legacy_TrackerCSRT {
		#[inline] fn as_raw_Legacy_TrackerCSRT(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Legacy_TrackerCSRTTrait for Legacy_TrackerCSRT {
		#[inline] fn as_raw_mut_Legacy_TrackerCSRT(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerCSRT, crate::tracking::Legacy_TrackerCSRTTraitConst, as_raw_Legacy_TrackerCSRT, crate::tracking::Legacy_TrackerCSRTTrait, as_raw_mut_Legacy_TrackerCSRT }

	pub struct Legacy_TrackerCSRT_Params {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Legacy_TrackerCSRT_Params }

	impl Drop for Legacy_TrackerCSRT_Params {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_legacy_TrackerCSRT_Params_delete(self.as_raw_mut_Legacy_TrackerCSRT_Params()) };
		}
	}

	unsafe impl Send for Legacy_TrackerCSRT_Params {}

	impl Legacy_TrackerCSRT_Params {
		/// Creates a default instance of the class by calling the default constructor
		#[inline]
		pub fn default() -> crate::tracking::Legacy_TrackerCSRT_Params {
			let ret = unsafe { sys::cv_legacy_TrackerCSRT_Params_defaultNew_const() };
			let ret = unsafe { crate::tracking::Legacy_TrackerCSRT_Params::opencv_from_extern(ret) };
			ret
		}

	}

	/// Constant methods for [crate::tracking::Legacy_TrackerCSRT_Params]
	pub trait Legacy_TrackerCSRT_ParamsTraitConst: crate::tracking::TrackerCSRT_ParamsTraitConst {
		fn as_raw_Legacy_TrackerCSRT_Params(&self) -> *const c_void;

		/// \brief Write parameters to a file
		#[inline]
		fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerCSRT_Params_write_const_FileStorageR(self.as_raw_Legacy_TrackerCSRT_Params(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::tracking::Legacy_TrackerCSRT_Params]
	pub trait Legacy_TrackerCSRT_ParamsTrait: crate::tracking::Legacy_TrackerCSRT_ParamsTraitConst + crate::tracking::TrackerCSRT_ParamsTrait {
		fn as_raw_mut_Legacy_TrackerCSRT_Params(&mut self) -> *mut c_void;

		/// \brief Read parameters from a file
		#[inline]
		fn read(&mut self, unnamed: &impl core::FileNodeTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerCSRT_Params_read_const_FileNodeR(self.as_raw_mut_Legacy_TrackerCSRT_Params(), unnamed.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl Default for Legacy_TrackerCSRT_Params {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
		}
	}

	impl std::fmt::Debug for Legacy_TrackerCSRT_Params {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Legacy_TrackerCSRT_Params")
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

	boxed_cast_base! { Legacy_TrackerCSRT_Params, crate::tracking::TrackerCSRT_Params, cv_legacy_TrackerCSRT_Params_to_TrackerCSRT_Params }

	impl crate::tracking::TrackerCSRT_ParamsTraitConst for Legacy_TrackerCSRT_Params {
		#[inline] fn as_raw_TrackerCSRT_Params(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::TrackerCSRT_ParamsTrait for Legacy_TrackerCSRT_Params {
		#[inline] fn as_raw_mut_TrackerCSRT_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerCSRT_Params, crate::tracking::TrackerCSRT_ParamsTraitConst, as_raw_TrackerCSRT_Params, crate::tracking::TrackerCSRT_ParamsTrait, as_raw_mut_TrackerCSRT_Params }

	impl crate::tracking::Legacy_TrackerCSRT_ParamsTraitConst for Legacy_TrackerCSRT_Params {
		#[inline] fn as_raw_Legacy_TrackerCSRT_Params(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Legacy_TrackerCSRT_ParamsTrait for Legacy_TrackerCSRT_Params {
		#[inline] fn as_raw_mut_Legacy_TrackerCSRT_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerCSRT_Params, crate::tracking::Legacy_TrackerCSRT_ParamsTraitConst, as_raw_Legacy_TrackerCSRT_Params, crate::tracking::Legacy_TrackerCSRT_ParamsTrait, as_raw_mut_Legacy_TrackerCSRT_Params }

	/// the KCF (Kernelized Correlation Filter) tracker
	///
	/// * KCF is a novel tracking framework that utilizes properties of circulant matrix to enhance the processing speed.
	/// * This tracking method is an implementation of [KCF_ECCV](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_KCF_ECCV) which is extended to KCF with color-names features ([KCF_CN](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_KCF_CN)).
	/// * The original paper of KCF is available at <http://www.robots.ox.ac.uk/~joao/publications/henriques_tpami2015.pdf>
	/// * as well as the matlab implementation. For more information about KCF with color-names features, please refer to
	/// * <http://www.cvl.isy.liu.se/research/objrec/visualtracking/colvistrack/index.html>.
	pub struct Legacy_TrackerKCF {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Legacy_TrackerKCF }

	impl Drop for Legacy_TrackerKCF {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_legacy_TrackerKCF_delete(self.as_raw_mut_Legacy_TrackerKCF()) };
		}
	}

	unsafe impl Send for Legacy_TrackerKCF {}

	impl Legacy_TrackerKCF {
		/// Constructor
		/// ## Parameters
		/// * parameters: KCF parameters TrackerKCF::Params
		#[inline]
		pub fn create(parameters: &impl crate::tracking::Legacy_TrackerKCF_ParamsTraitConst) -> Result<core::Ptr<crate::tracking::Legacy_TrackerKCF>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerKCF_create_const_ParamsR(parameters.as_raw_Legacy_TrackerKCF_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::tracking::Legacy_TrackerKCF>::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		pub fn create_1() -> Result<core::Ptr<crate::tracking::Legacy_TrackerKCF>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerKCF_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::tracking::Legacy_TrackerKCF>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Legacy_TrackerKCF]
	pub trait Legacy_TrackerKCFTraitConst: crate::tracking::Legacy_TrackerTraitConst {
		fn as_raw_Legacy_TrackerKCF(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::tracking::Legacy_TrackerKCF]
	pub trait Legacy_TrackerKCFTrait: crate::tracking::Legacy_TrackerKCFTraitConst + crate::tracking::Legacy_TrackerTrait {
		fn as_raw_mut_Legacy_TrackerKCF(&mut self) -> *mut c_void;

		/// ## C++ default parameters
		/// * pca_func: false
		#[inline]
		fn set_feature_extractor(&mut self, unnamed: Option<unsafe extern "C" fn(*const c_void, core::Rect, *mut c_void) -> ()>, pca_func: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerKCF_setFeatureExtractor_void__X__const_cv_Mat__const_cv_Rect__cv_MatR__bool(self.as_raw_mut_Legacy_TrackerKCF(), unnamed, pca_func, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// ## Note
		/// This alternative version of [Legacy_TrackerKCFTrait::set_feature_extractor] function uses the following default values for its arguments:
		/// * pca_func: false
		#[inline]
		fn set_feature_extractor_def(&mut self, unnamed: Option<unsafe extern "C" fn(*const c_void, core::Rect, *mut c_void) -> ()>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerKCF_setFeatureExtractor_void__X__const_cv_Mat__const_cv_Rect__cv_MatR_(self.as_raw_mut_Legacy_TrackerKCF(), unnamed, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Legacy_TrackerKCF {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Legacy_TrackerKCF")
				.finish()
		}
	}

	boxed_cast_base! { Legacy_TrackerKCF, core::Algorithm, cv_legacy_TrackerKCF_to_Algorithm }

	boxed_cast_base! { Legacy_TrackerKCF, crate::tracking::Legacy_Tracker, cv_legacy_TrackerKCF_to_Legacy_Tracker }

	impl core::AlgorithmTraitConst for Legacy_TrackerKCF {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for Legacy_TrackerKCF {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerKCF, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::tracking::Legacy_TrackerTraitConst for Legacy_TrackerKCF {
		#[inline] fn as_raw_Legacy_Tracker(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Legacy_TrackerTrait for Legacy_TrackerKCF {
		#[inline] fn as_raw_mut_Legacy_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerKCF, crate::tracking::Legacy_TrackerTraitConst, as_raw_Legacy_Tracker, crate::tracking::Legacy_TrackerTrait, as_raw_mut_Legacy_Tracker }

	impl crate::tracking::Legacy_TrackerKCFTraitConst for Legacy_TrackerKCF {
		#[inline] fn as_raw_Legacy_TrackerKCF(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Legacy_TrackerKCFTrait for Legacy_TrackerKCF {
		#[inline] fn as_raw_mut_Legacy_TrackerKCF(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerKCF, crate::tracking::Legacy_TrackerKCFTraitConst, as_raw_Legacy_TrackerKCF, crate::tracking::Legacy_TrackerKCFTrait, as_raw_mut_Legacy_TrackerKCF }

	pub struct Legacy_TrackerKCF_Params {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Legacy_TrackerKCF_Params }

	impl Drop for Legacy_TrackerKCF_Params {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_legacy_TrackerKCF_Params_delete(self.as_raw_mut_Legacy_TrackerKCF_Params()) };
		}
	}

	unsafe impl Send for Legacy_TrackerKCF_Params {}

	impl Legacy_TrackerKCF_Params {
		/// Creates a default instance of the class by calling the default constructor
		#[inline]
		pub fn default() -> crate::tracking::Legacy_TrackerKCF_Params {
			let ret = unsafe { sys::cv_legacy_TrackerKCF_Params_defaultNew_const() };
			let ret = unsafe { crate::tracking::Legacy_TrackerKCF_Params::opencv_from_extern(ret) };
			ret
		}

	}

	/// Constant methods for [crate::tracking::Legacy_TrackerKCF_Params]
	pub trait Legacy_TrackerKCF_ParamsTraitConst {
		fn as_raw_Legacy_TrackerKCF_Params(&self) -> *const c_void;

		#[inline]
		fn write(&self, unnamed: &mut impl core::FileStorageTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerKCF_Params_write_const_FileStorageR(self.as_raw_Legacy_TrackerKCF_Params(), unnamed.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::tracking::Legacy_TrackerKCF_Params]
	pub trait Legacy_TrackerKCF_ParamsTrait: crate::tracking::Legacy_TrackerKCF_ParamsTraitConst {
		fn as_raw_mut_Legacy_TrackerKCF_Params(&mut self) -> *mut c_void;

		#[inline]
		fn read(&mut self, unnamed: &impl core::FileNodeTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerKCF_Params_read_const_FileNodeR(self.as_raw_mut_Legacy_TrackerKCF_Params(), unnamed.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl Default for Legacy_TrackerKCF_Params {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
		}
	}

	impl std::fmt::Debug for Legacy_TrackerKCF_Params {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Legacy_TrackerKCF_Params")
				.finish()
		}
	}

	impl crate::tracking::Legacy_TrackerKCF_ParamsTraitConst for Legacy_TrackerKCF_Params {
		#[inline] fn as_raw_Legacy_TrackerKCF_Params(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Legacy_TrackerKCF_ParamsTrait for Legacy_TrackerKCF_Params {
		#[inline] fn as_raw_mut_Legacy_TrackerKCF_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerKCF_Params, crate::tracking::Legacy_TrackerKCF_ParamsTraitConst, as_raw_Legacy_TrackerKCF_Params, crate::tracking::Legacy_TrackerKCF_ParamsTrait, as_raw_mut_Legacy_TrackerKCF_Params }

	/// The MIL algorithm trains a classifier in an online manner to separate the object from the
	/// background.
	///
	/// Multiple Instance Learning avoids the drift problem for a robust tracking. The implementation is
	/// based on [MIL](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_MIL) .
	///
	/// Original code can be found here <http://vision.ucsd.edu/~bbabenko/project_miltrack.shtml>
	pub struct Legacy_TrackerMIL {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Legacy_TrackerMIL }

	impl Drop for Legacy_TrackerMIL {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_legacy_TrackerMIL_delete(self.as_raw_mut_Legacy_TrackerMIL()) };
		}
	}

	unsafe impl Send for Legacy_TrackerMIL {}

	impl Legacy_TrackerMIL {
		/// Constructor
		/// ## Parameters
		/// * parameters: MIL parameters TrackerMIL::Params
		#[inline]
		pub fn create(parameters: &impl crate::tracking::Legacy_TrackerMIL_ParamsTraitConst) -> Result<core::Ptr<crate::tracking::Legacy_TrackerMIL>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerMIL_create_const_ParamsR(parameters.as_raw_Legacy_TrackerMIL_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::tracking::Legacy_TrackerMIL>::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		pub fn create_1() -> Result<core::Ptr<crate::tracking::Legacy_TrackerMIL>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerMIL_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::tracking::Legacy_TrackerMIL>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Legacy_TrackerMIL]
	pub trait Legacy_TrackerMILTraitConst: crate::tracking::Legacy_TrackerTraitConst {
		fn as_raw_Legacy_TrackerMIL(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::tracking::Legacy_TrackerMIL]
	pub trait Legacy_TrackerMILTrait: crate::tracking::Legacy_TrackerMILTraitConst + crate::tracking::Legacy_TrackerTrait {
		fn as_raw_mut_Legacy_TrackerMIL(&mut self) -> *mut c_void;

	}

	impl std::fmt::Debug for Legacy_TrackerMIL {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Legacy_TrackerMIL")
				.finish()
		}
	}

	boxed_cast_base! { Legacy_TrackerMIL, core::Algorithm, cv_legacy_TrackerMIL_to_Algorithm }

	boxed_cast_base! { Legacy_TrackerMIL, crate::tracking::Legacy_Tracker, cv_legacy_TrackerMIL_to_Legacy_Tracker }

	impl core::AlgorithmTraitConst for Legacy_TrackerMIL {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for Legacy_TrackerMIL {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerMIL, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::tracking::Legacy_TrackerTraitConst for Legacy_TrackerMIL {
		#[inline] fn as_raw_Legacy_Tracker(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Legacy_TrackerTrait for Legacy_TrackerMIL {
		#[inline] fn as_raw_mut_Legacy_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerMIL, crate::tracking::Legacy_TrackerTraitConst, as_raw_Legacy_Tracker, crate::tracking::Legacy_TrackerTrait, as_raw_mut_Legacy_Tracker }

	impl crate::tracking::Legacy_TrackerMILTraitConst for Legacy_TrackerMIL {
		#[inline] fn as_raw_Legacy_TrackerMIL(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Legacy_TrackerMILTrait for Legacy_TrackerMIL {
		#[inline] fn as_raw_mut_Legacy_TrackerMIL(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerMIL, crate::tracking::Legacy_TrackerMILTraitConst, as_raw_Legacy_TrackerMIL, crate::tracking::Legacy_TrackerMILTrait, as_raw_mut_Legacy_TrackerMIL }

	pub struct Legacy_TrackerMIL_Params {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Legacy_TrackerMIL_Params }

	impl Drop for Legacy_TrackerMIL_Params {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_legacy_TrackerMIL_Params_delete(self.as_raw_mut_Legacy_TrackerMIL_Params()) };
		}
	}

	unsafe impl Send for Legacy_TrackerMIL_Params {}

	impl Legacy_TrackerMIL_Params {
		/// Creates a default instance of the class by calling the default constructor
		#[inline]
		pub fn default() -> crate::tracking::Legacy_TrackerMIL_Params {
			let ret = unsafe { sys::cv_legacy_TrackerMIL_Params_defaultNew_const() };
			let ret = unsafe { crate::tracking::Legacy_TrackerMIL_Params::opencv_from_extern(ret) };
			ret
		}

	}

	/// Constant methods for [crate::tracking::Legacy_TrackerMIL_Params]
	pub trait Legacy_TrackerMIL_ParamsTraitConst {
		fn as_raw_Legacy_TrackerMIL_Params(&self) -> *const c_void;

		#[inline]
		fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerMIL_Params_write_const_FileStorageR(self.as_raw_Legacy_TrackerMIL_Params(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::tracking::Legacy_TrackerMIL_Params]
	pub trait Legacy_TrackerMIL_ParamsTrait: crate::tracking::Legacy_TrackerMIL_ParamsTraitConst {
		fn as_raw_mut_Legacy_TrackerMIL_Params(&mut self) -> *mut c_void;

		#[inline]
		fn read(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerMIL_Params_read_const_FileNodeR(self.as_raw_mut_Legacy_TrackerMIL_Params(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl Default for Legacy_TrackerMIL_Params {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
		}
	}

	impl std::fmt::Debug for Legacy_TrackerMIL_Params {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Legacy_TrackerMIL_Params")
				.finish()
		}
	}

	impl crate::tracking::Legacy_TrackerMIL_ParamsTraitConst for Legacy_TrackerMIL_Params {
		#[inline] fn as_raw_Legacy_TrackerMIL_Params(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Legacy_TrackerMIL_ParamsTrait for Legacy_TrackerMIL_Params {
		#[inline] fn as_raw_mut_Legacy_TrackerMIL_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerMIL_Params, crate::tracking::Legacy_TrackerMIL_ParamsTraitConst, as_raw_Legacy_TrackerMIL_Params, crate::tracking::Legacy_TrackerMIL_ParamsTrait, as_raw_mut_Legacy_TrackerMIL_Params }

	/// the MOSSE (Minimum Output Sum of Squared %Error) tracker
	///
	/// The implementation is based on [MOSSE](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_MOSSE) Visual Object Tracking using Adaptive Correlation Filters
	///
	/// Note: this tracker works with grayscale images, if passed bgr ones, they will get converted internally.
	pub struct Legacy_TrackerMOSSE {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Legacy_TrackerMOSSE }

	impl Drop for Legacy_TrackerMOSSE {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_legacy_TrackerMOSSE_delete(self.as_raw_mut_Legacy_TrackerMOSSE()) };
		}
	}

	unsafe impl Send for Legacy_TrackerMOSSE {}

	impl Legacy_TrackerMOSSE {
		/// Constructor
		#[inline]
		pub fn create() -> Result<core::Ptr<crate::tracking::Legacy_TrackerMOSSE>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerMOSSE_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::tracking::Legacy_TrackerMOSSE>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Legacy_TrackerMOSSE]
	pub trait Legacy_TrackerMOSSETraitConst: crate::tracking::Legacy_TrackerTraitConst {
		fn as_raw_Legacy_TrackerMOSSE(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::tracking::Legacy_TrackerMOSSE]
	pub trait Legacy_TrackerMOSSETrait: crate::tracking::Legacy_TrackerMOSSETraitConst + crate::tracking::Legacy_TrackerTrait {
		fn as_raw_mut_Legacy_TrackerMOSSE(&mut self) -> *mut c_void;

	}

	impl std::fmt::Debug for Legacy_TrackerMOSSE {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Legacy_TrackerMOSSE")
				.finish()
		}
	}

	boxed_cast_base! { Legacy_TrackerMOSSE, core::Algorithm, cv_legacy_TrackerMOSSE_to_Algorithm }

	boxed_cast_base! { Legacy_TrackerMOSSE, crate::tracking::Legacy_Tracker, cv_legacy_TrackerMOSSE_to_Legacy_Tracker }

	impl core::AlgorithmTraitConst for Legacy_TrackerMOSSE {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for Legacy_TrackerMOSSE {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerMOSSE, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::tracking::Legacy_TrackerTraitConst for Legacy_TrackerMOSSE {
		#[inline] fn as_raw_Legacy_Tracker(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Legacy_TrackerTrait for Legacy_TrackerMOSSE {
		#[inline] fn as_raw_mut_Legacy_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerMOSSE, crate::tracking::Legacy_TrackerTraitConst, as_raw_Legacy_Tracker, crate::tracking::Legacy_TrackerTrait, as_raw_mut_Legacy_Tracker }

	impl crate::tracking::Legacy_TrackerMOSSETraitConst for Legacy_TrackerMOSSE {
		#[inline] fn as_raw_Legacy_TrackerMOSSE(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Legacy_TrackerMOSSETrait for Legacy_TrackerMOSSE {
		#[inline] fn as_raw_mut_Legacy_TrackerMOSSE(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerMOSSE, crate::tracking::Legacy_TrackerMOSSETraitConst, as_raw_Legacy_TrackerMOSSE, crate::tracking::Legacy_TrackerMOSSETrait, as_raw_mut_Legacy_TrackerMOSSE }

	/// the Median Flow tracker
	///
	/// Implementation of a paper [MedianFlow](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_MedianFlow) .
	///
	/// The tracker is suitable for very smooth and predictable movements when object is visible throughout
	/// the whole sequence. It's quite and accurate for this type of problems (in particular, it was shown
	/// by authors to outperform MIL). During the implementation period the code at
	/// <http://www.aonsquared.co.uk/node/5>, the courtesy of the author Arthur Amarra, was used for the
	/// reference purpose.
	pub struct Legacy_TrackerMedianFlow {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Legacy_TrackerMedianFlow }

	impl Drop for Legacy_TrackerMedianFlow {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_legacy_TrackerMedianFlow_delete(self.as_raw_mut_Legacy_TrackerMedianFlow()) };
		}
	}

	unsafe impl Send for Legacy_TrackerMedianFlow {}

	impl Legacy_TrackerMedianFlow {
		/// Constructor
		/// ## Parameters
		/// * parameters: Median Flow parameters TrackerMedianFlow::Params
		#[inline]
		pub fn create(parameters: &impl crate::tracking::Legacy_TrackerMedianFlow_ParamsTraitConst) -> Result<core::Ptr<crate::tracking::Legacy_TrackerMedianFlow>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerMedianFlow_create_const_ParamsR(parameters.as_raw_Legacy_TrackerMedianFlow_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::tracking::Legacy_TrackerMedianFlow>::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		pub fn create_1() -> Result<core::Ptr<crate::tracking::Legacy_TrackerMedianFlow>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerMedianFlow_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::tracking::Legacy_TrackerMedianFlow>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Legacy_TrackerMedianFlow]
	pub trait Legacy_TrackerMedianFlowTraitConst: crate::tracking::Legacy_TrackerTraitConst {
		fn as_raw_Legacy_TrackerMedianFlow(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::tracking::Legacy_TrackerMedianFlow]
	pub trait Legacy_TrackerMedianFlowTrait: crate::tracking::Legacy_TrackerMedianFlowTraitConst + crate::tracking::Legacy_TrackerTrait {
		fn as_raw_mut_Legacy_TrackerMedianFlow(&mut self) -> *mut c_void;

	}

	impl std::fmt::Debug for Legacy_TrackerMedianFlow {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Legacy_TrackerMedianFlow")
				.finish()
		}
	}

	boxed_cast_base! { Legacy_TrackerMedianFlow, core::Algorithm, cv_legacy_TrackerMedianFlow_to_Algorithm }

	boxed_cast_base! { Legacy_TrackerMedianFlow, crate::tracking::Legacy_Tracker, cv_legacy_TrackerMedianFlow_to_Legacy_Tracker }

	impl core::AlgorithmTraitConst for Legacy_TrackerMedianFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for Legacy_TrackerMedianFlow {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerMedianFlow, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::tracking::Legacy_TrackerTraitConst for Legacy_TrackerMedianFlow {
		#[inline] fn as_raw_Legacy_Tracker(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Legacy_TrackerTrait for Legacy_TrackerMedianFlow {
		#[inline] fn as_raw_mut_Legacy_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerMedianFlow, crate::tracking::Legacy_TrackerTraitConst, as_raw_Legacy_Tracker, crate::tracking::Legacy_TrackerTrait, as_raw_mut_Legacy_Tracker }

	impl crate::tracking::Legacy_TrackerMedianFlowTraitConst for Legacy_TrackerMedianFlow {
		#[inline] fn as_raw_Legacy_TrackerMedianFlow(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Legacy_TrackerMedianFlowTrait for Legacy_TrackerMedianFlow {
		#[inline] fn as_raw_mut_Legacy_TrackerMedianFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerMedianFlow, crate::tracking::Legacy_TrackerMedianFlowTraitConst, as_raw_Legacy_TrackerMedianFlow, crate::tracking::Legacy_TrackerMedianFlowTrait, as_raw_mut_Legacy_TrackerMedianFlow }

	pub struct Legacy_TrackerMedianFlow_Params {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Legacy_TrackerMedianFlow_Params }

	impl Drop for Legacy_TrackerMedianFlow_Params {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_legacy_TrackerMedianFlow_Params_delete(self.as_raw_mut_Legacy_TrackerMedianFlow_Params()) };
		}
	}

	unsafe impl Send for Legacy_TrackerMedianFlow_Params {}

	impl Legacy_TrackerMedianFlow_Params {
		#[inline]
		pub fn default() -> Result<crate::tracking::Legacy_TrackerMedianFlow_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerMedianFlow_Params_Params(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Legacy_TrackerMedianFlow_Params::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Legacy_TrackerMedianFlow_Params]
	pub trait Legacy_TrackerMedianFlow_ParamsTraitConst {
		fn as_raw_Legacy_TrackerMedianFlow_Params(&self) -> *const c_void;

		/// square root of number of keypoints used; increase it to trade
		/// accurateness for speed
		#[inline]
		fn points_in_grid(&self) -> i32 {
			let ret = unsafe { sys::cv_legacy_TrackerMedianFlow_Params_propPointsInGrid_const(self.as_raw_Legacy_TrackerMedianFlow_Params()) };
			ret
		}

		/// window size parameter for Lucas-Kanade optical flow
		#[inline]
		fn win_size(&self) -> core::Size {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerMedianFlow_Params_propWinSize_const(self.as_raw_Legacy_TrackerMedianFlow_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			ret
		}

		/// maximal pyramid level number for Lucas-Kanade optical flow
		#[inline]
		fn max_level(&self) -> i32 {
			let ret = unsafe { sys::cv_legacy_TrackerMedianFlow_Params_propMaxLevel_const(self.as_raw_Legacy_TrackerMedianFlow_Params()) };
			ret
		}

		/// termination criteria for Lucas-Kanade optical flow
		#[inline]
		fn term_criteria(&self) -> core::TermCriteria {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerMedianFlow_Params_propTermCriteria_const(self.as_raw_Legacy_TrackerMedianFlow_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			ret
		}

		/// window size around a point for normalized cross-correlation check
		#[inline]
		fn win_size_ncc(&self) -> core::Size {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerMedianFlow_Params_propWinSizeNCC_const(self.as_raw_Legacy_TrackerMedianFlow_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			ret
		}

		/// criterion for loosing the tracked object
		#[inline]
		fn max_median_length_of_displacement_difference(&self) -> f64 {
			let ret = unsafe { sys::cv_legacy_TrackerMedianFlow_Params_propMaxMedianLengthOfDisplacementDifference_const(self.as_raw_Legacy_TrackerMedianFlow_Params()) };
			ret
		}

		#[inline]
		fn write(&self, unnamed: &mut impl core::FileStorageTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerMedianFlow_Params_write_const_FileStorageR(self.as_raw_Legacy_TrackerMedianFlow_Params(), unnamed.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::tracking::Legacy_TrackerMedianFlow_Params]
	pub trait Legacy_TrackerMedianFlow_ParamsTrait: crate::tracking::Legacy_TrackerMedianFlow_ParamsTraitConst {
		fn as_raw_mut_Legacy_TrackerMedianFlow_Params(&mut self) -> *mut c_void;

		/// square root of number of keypoints used; increase it to trade
		/// accurateness for speed
		#[inline]
		fn set_points_in_grid(&mut self, val: i32) {
			let ret = unsafe { sys::cv_legacy_TrackerMedianFlow_Params_propPointsInGrid_const_int(self.as_raw_mut_Legacy_TrackerMedianFlow_Params(), val) };
			ret
		}

		/// window size parameter for Lucas-Kanade optical flow
		#[inline]
		fn set_win_size(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_legacy_TrackerMedianFlow_Params_propWinSize_const_Size(self.as_raw_mut_Legacy_TrackerMedianFlow_Params(), &val) };
			ret
		}

		/// maximal pyramid level number for Lucas-Kanade optical flow
		#[inline]
		fn set_max_level(&mut self, val: i32) {
			let ret = unsafe { sys::cv_legacy_TrackerMedianFlow_Params_propMaxLevel_const_int(self.as_raw_mut_Legacy_TrackerMedianFlow_Params(), val) };
			ret
		}

		/// termination criteria for Lucas-Kanade optical flow
		#[inline]
		fn set_term_criteria(&mut self, val: core::TermCriteria) {
			let ret = unsafe { sys::cv_legacy_TrackerMedianFlow_Params_propTermCriteria_const_TermCriteria(self.as_raw_mut_Legacy_TrackerMedianFlow_Params(), &val) };
			ret
		}

		/// window size around a point for normalized cross-correlation check
		#[inline]
		fn set_win_size_ncc(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_legacy_TrackerMedianFlow_Params_propWinSizeNCC_const_Size(self.as_raw_mut_Legacy_TrackerMedianFlow_Params(), &val) };
			ret
		}

		/// criterion for loosing the tracked object
		#[inline]
		fn set_max_median_length_of_displacement_difference(&mut self, val: f64) {
			let ret = unsafe { sys::cv_legacy_TrackerMedianFlow_Params_propMaxMedianLengthOfDisplacementDifference_const_double(self.as_raw_mut_Legacy_TrackerMedianFlow_Params(), val) };
			ret
		}

		#[inline]
		fn read(&mut self, unnamed: &impl core::FileNodeTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerMedianFlow_Params_read_const_FileNodeR(self.as_raw_mut_Legacy_TrackerMedianFlow_Params(), unnamed.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Legacy_TrackerMedianFlow_Params {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Legacy_TrackerMedianFlow_Params")
				.field("points_in_grid", &crate::tracking::Legacy_TrackerMedianFlow_ParamsTraitConst::points_in_grid(self))
				.field("win_size", &crate::tracking::Legacy_TrackerMedianFlow_ParamsTraitConst::win_size(self))
				.field("max_level", &crate::tracking::Legacy_TrackerMedianFlow_ParamsTraitConst::max_level(self))
				.field("term_criteria", &crate::tracking::Legacy_TrackerMedianFlow_ParamsTraitConst::term_criteria(self))
				.field("win_size_ncc", &crate::tracking::Legacy_TrackerMedianFlow_ParamsTraitConst::win_size_ncc(self))
				.field("max_median_length_of_displacement_difference", &crate::tracking::Legacy_TrackerMedianFlow_ParamsTraitConst::max_median_length_of_displacement_difference(self))
				.finish()
		}
	}

	impl crate::tracking::Legacy_TrackerMedianFlow_ParamsTraitConst for Legacy_TrackerMedianFlow_Params {
		#[inline] fn as_raw_Legacy_TrackerMedianFlow_Params(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Legacy_TrackerMedianFlow_ParamsTrait for Legacy_TrackerMedianFlow_Params {
		#[inline] fn as_raw_mut_Legacy_TrackerMedianFlow_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerMedianFlow_Params, crate::tracking::Legacy_TrackerMedianFlow_ParamsTraitConst, as_raw_Legacy_TrackerMedianFlow_Params, crate::tracking::Legacy_TrackerMedianFlow_ParamsTrait, as_raw_mut_Legacy_TrackerMedianFlow_Params }

	/// the TLD (Tracking, learning and detection) tracker
	///
	/// TLD is a novel tracking framework that explicitly decomposes the long-term tracking task into
	/// tracking, learning and detection.
	///
	/// The tracker follows the object from frame to frame. The detector localizes all appearances that
	/// have been observed so far and corrects the tracker if necessary. The learning estimates detector's
	/// errors and updates it to avoid these errors in the future. The implementation is based on [TLD](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_TLD) .
	///
	/// The Median Flow algorithm (see cv::TrackerMedianFlow) was chosen as a tracking component in this
	/// implementation, following authors. The tracker is supposed to be able to handle rapid motions, partial
	/// occlusions, object absence etc.
	pub struct Legacy_TrackerTLD {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Legacy_TrackerTLD }

	impl Drop for Legacy_TrackerTLD {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_legacy_TrackerTLD_delete(self.as_raw_mut_Legacy_TrackerTLD()) };
		}
	}

	unsafe impl Send for Legacy_TrackerTLD {}

	impl Legacy_TrackerTLD {
		/// Constructor
		/// ## Parameters
		/// * parameters: TLD parameters TrackerTLD::Params
		#[inline]
		pub fn create(parameters: &impl crate::tracking::Legacy_TrackerTLD_ParamsTraitConst) -> Result<core::Ptr<crate::tracking::Legacy_TrackerTLD>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerTLD_create_const_ParamsR(parameters.as_raw_Legacy_TrackerTLD_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::tracking::Legacy_TrackerTLD>::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		pub fn create_1() -> Result<core::Ptr<crate::tracking::Legacy_TrackerTLD>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerTLD_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::tracking::Legacy_TrackerTLD>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Legacy_TrackerTLD]
	pub trait Legacy_TrackerTLDTraitConst: crate::tracking::Legacy_TrackerTraitConst {
		fn as_raw_Legacy_TrackerTLD(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::tracking::Legacy_TrackerTLD]
	pub trait Legacy_TrackerTLDTrait: crate::tracking::Legacy_TrackerTLDTraitConst + crate::tracking::Legacy_TrackerTrait {
		fn as_raw_mut_Legacy_TrackerTLD(&mut self) -> *mut c_void;

	}

	impl std::fmt::Debug for Legacy_TrackerTLD {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Legacy_TrackerTLD")
				.finish()
		}
	}

	boxed_cast_base! { Legacy_TrackerTLD, core::Algorithm, cv_legacy_TrackerTLD_to_Algorithm }

	boxed_cast_base! { Legacy_TrackerTLD, crate::tracking::Legacy_Tracker, cv_legacy_TrackerTLD_to_Legacy_Tracker }

	impl core::AlgorithmTraitConst for Legacy_TrackerTLD {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for Legacy_TrackerTLD {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerTLD, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::tracking::Legacy_TrackerTraitConst for Legacy_TrackerTLD {
		#[inline] fn as_raw_Legacy_Tracker(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Legacy_TrackerTrait for Legacy_TrackerTLD {
		#[inline] fn as_raw_mut_Legacy_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerTLD, crate::tracking::Legacy_TrackerTraitConst, as_raw_Legacy_Tracker, crate::tracking::Legacy_TrackerTrait, as_raw_mut_Legacy_Tracker }

	impl crate::tracking::Legacy_TrackerTLDTraitConst for Legacy_TrackerTLD {
		#[inline] fn as_raw_Legacy_TrackerTLD(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Legacy_TrackerTLDTrait for Legacy_TrackerTLD {
		#[inline] fn as_raw_mut_Legacy_TrackerTLD(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerTLD, crate::tracking::Legacy_TrackerTLDTraitConst, as_raw_Legacy_TrackerTLD, crate::tracking::Legacy_TrackerTLDTrait, as_raw_mut_Legacy_TrackerTLD }

	pub struct Legacy_TrackerTLD_Params {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Legacy_TrackerTLD_Params }

	impl Drop for Legacy_TrackerTLD_Params {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_legacy_TrackerTLD_Params_delete(self.as_raw_mut_Legacy_TrackerTLD_Params()) };
		}
	}

	unsafe impl Send for Legacy_TrackerTLD_Params {}

	impl Legacy_TrackerTLD_Params {
		#[inline]
		pub fn default() -> Result<crate::tracking::Legacy_TrackerTLD_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerTLD_Params_Params(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::tracking::Legacy_TrackerTLD_Params::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::tracking::Legacy_TrackerTLD_Params]
	pub trait Legacy_TrackerTLD_ParamsTraitConst {
		fn as_raw_Legacy_TrackerTLD_Params(&self) -> *const c_void;

		#[inline]
		fn write(&self, unnamed: &mut impl core::FileStorageTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerTLD_Params_write_const_FileStorageR(self.as_raw_Legacy_TrackerTLD_Params(), unnamed.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::tracking::Legacy_TrackerTLD_Params]
	pub trait Legacy_TrackerTLD_ParamsTrait: crate::tracking::Legacy_TrackerTLD_ParamsTraitConst {
		fn as_raw_mut_Legacy_TrackerTLD_Params(&mut self) -> *mut c_void;

		#[inline]
		fn read(&mut self, unnamed: &impl core::FileNodeTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_legacy_TrackerTLD_Params_read_const_FileNodeR(self.as_raw_mut_Legacy_TrackerTLD_Params(), unnamed.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Legacy_TrackerTLD_Params {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Legacy_TrackerTLD_Params")
				.finish()
		}
	}

	impl crate::tracking::Legacy_TrackerTLD_ParamsTraitConst for Legacy_TrackerTLD_Params {
		#[inline] fn as_raw_Legacy_TrackerTLD_Params(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::tracking::Legacy_TrackerTLD_ParamsTrait for Legacy_TrackerTLD_Params {
		#[inline] fn as_raw_mut_Legacy_TrackerTLD_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Legacy_TrackerTLD_Params, crate::tracking::Legacy_TrackerTLD_ParamsTraitConst, as_raw_Legacy_TrackerTLD_Params, crate::tracking::Legacy_TrackerTLD_ParamsTrait, as_raw_mut_Legacy_TrackerTLD_Params }

}
