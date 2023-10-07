pub mod structured_light {
	//! # Structured Light API
	//! 
	//! Structured light is considered one of the most effective techniques to acquire 3D models.
	//! This technique is based on projecting a light pattern and capturing the illuminated scene
	//! from one or more points of view. Since the pattern is coded, correspondences between image
	//! points and points of the projected pattern can be quickly found and 3D information easily
	//! retrieved.
	//! 
	//! One of the most commonly exploited coding strategies is based on trmatime-multiplexing. In this
	//! case, a set of patterns  are successively projected onto the measuring surface.
	//! The codeword for a given pixel is usually formed by  the sequence of illuminance values for that
	//! pixel across the projected patterns. Thus, the codification is called  temporal because the bits
	//! of the codewords are multiplexed in time [pattern](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_pattern) .
	//! 
	//! In this module a time-multiplexing coding strategy based on Gray encoding is implemented following the
	//! (stereo) approach described in 3DUNDERWORLD algorithm [UNDERWORLD](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_UNDERWORLD) .
	//! For more details, see [tutorial_structured_light].
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::StructuredLightPatternTraitConst, super::StructuredLightPatternTrait, super::GrayCodePattern_ParamsTraitConst, super::GrayCodePattern_ParamsTrait, super::GrayCodePatternTraitConst, super::GrayCodePatternTrait, super::SinusoidalPattern_ParamsTraitConst, super::SinusoidalPattern_ParamsTrait, super::SinusoidalPatternTraitConst, super::SinusoidalPatternTrait };
	}
	
	/// Kyriakos Herakleous, Charalambos Poullis. "3DUNDERWORLD-SLS: An Open-Source Structured-Light Scanning System for Rapid Geometry Acquisition", arXiv preprint arXiv:1406.6595 (2014).
	pub const DECODE_3D_UNDERWORLD: i32 = 0;
	pub const FAPS: i32 = 2;
	pub const FTP: i32 = 0;
	pub const PSP: i32 = 1;
	/// Constant methods for [crate::structured_light::GrayCodePattern]
	pub trait GrayCodePatternTraitConst: crate::structured_light::StructuredLightPatternTraitConst {
		fn as_raw_GrayCodePattern(&self) -> *const c_void;
	
		/// Get the number of pattern images needed for the graycode pattern.
		/// 
		/// ## Returns
		/// The number of pattern images needed for the graycode pattern.
		#[inline]
		fn get_number_of_pattern_images(&self) -> Result<size_t> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_structured_light_GrayCodePattern_getNumberOfPatternImages_const(self.as_raw_GrayCodePattern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Generates the all-black and all-white images needed for shadowMasks computation.
		/// 
		/// To identify shadow regions, the regions of two images where the pixels are not lit by projector's light and thus where there is not coded information,
		/// the 3DUNDERWORLD algorithm computes a shadow mask for the two cameras views, starting from a white and a black images captured by each camera.
		/// This method generates these two additional images to project.
		/// 
		/// ## Parameters
		/// * blackImage: The generated all-black CV_8U image, at projector's resolution.
		/// * whiteImage: The generated all-white CV_8U image, at projector's resolution.
		#[inline]
		fn get_images_for_shadow_masks(&self, black_image: &mut impl core::ToInputOutputArray, white_image: &mut impl core::ToInputOutputArray) -> Result<()> {
			input_output_array_arg!(black_image);
			input_output_array_arg!(white_image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_structured_light_GrayCodePattern_getImagesForShadowMasks_const_const__InputOutputArrayR_const__InputOutputArrayR(self.as_raw_GrayCodePattern(), black_image.as_raw__InputOutputArray(), white_image.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// For a (x,y) pixel of a camera returns the corresponding projector pixel.
		/// 
		/// The function decodes each pixel in the pattern images acquired by a camera into their corresponding decimal numbers representing the projector's column and row,
		/// providing a mapping between camera's and projector's pixel.
		/// 
		/// ## Parameters
		/// * patternImages: The pattern images acquired by the camera, stored in a grayscale vector < Mat >.
		/// * x: x coordinate of the image pixel.
		/// * y: y coordinate of the image pixel.
		/// * projPix: Projector's pixel corresponding to the camera's pixel: projPix.x and projPix.y are the image coordinates of the projector's pixel corresponding to the pixel being decoded in a camera.
		#[inline]
		fn get_proj_pixel(&self, pattern_images: &impl core::ToInputArray, x: i32, y: i32, proj_pix: &mut core::Point) -> Result<bool> {
			input_array_arg!(pattern_images);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_structured_light_GrayCodePattern_getProjPixel_const_const__InputArrayR_int_int_PointR(self.as_raw_GrayCodePattern(), pattern_images.as_raw__InputArray(), x, y, proj_pix, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::structured_light::GrayCodePattern]
	pub trait GrayCodePatternTrait: crate::structured_light::GrayCodePatternTraitConst + crate::structured_light::StructuredLightPatternTrait {
		fn as_raw_mut_GrayCodePattern(&mut self) -> *mut c_void;
	
		/// Sets the value for white threshold, needed for decoding.
		/// 
		/// White threshold is a number between 0-255 that represents the minimum brightness difference required for valid pixels, between the graycode pattern and its inverse images; used in getProjPixel method.
		/// 
		/// ## Parameters
		/// * value: The desired white threshold value.
		#[inline]
		fn set_white_threshold(&mut self, value: size_t) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_structured_light_GrayCodePattern_setWhiteThreshold_size_t(self.as_raw_mut_GrayCodePattern(), value, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Sets the value for black threshold, needed for decoding (shadowsmasks computation).
		/// 
		/// Black threshold is a number between 0-255 that represents the minimum brightness difference required for valid pixels, between the fully illuminated (white) and the not illuminated images (black); used in computeShadowMasks method.
		/// 
		/// ## Parameters
		/// * value: The desired black threshold value.
		#[inline]
		fn set_black_threshold(&mut self, value: size_t) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_structured_light_GrayCodePattern_setBlackThreshold_size_t(self.as_raw_mut_GrayCodePattern(), value, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Class implementing the Gray-code pattern, based on [UNDERWORLD](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_UNDERWORLD).
	/// 
	/// The generation of the pattern images is performed with Gray encoding using the traditional white and black colors.
	/// 
	/// The information about the two image axes x, y is encoded separately into two different pattern sequences.
	/// A projector P with resolution (P_res_x, P_res_y) will result in Ncols = log 2 (P_res_x) encoded pattern images representing the columns, and
	/// in Nrows = log 2 (P_res_y) encoded pattern images representing the rows.
	/// For example a projector with resolution 1024x768 will result in Ncols = 10 and Nrows = 10.
	/// 
	/// However, the generated pattern sequence consists of both regular color and color-inverted images: inverted pattern images are images
	/// with the same structure as the original but with inverted colors.
	/// This provides an effective method for easily determining the intensity value of each pixel when it is lit (highest value) and
	/// when it is not lit (lowest value). So for a a projector with resolution 1024x768, the number of pattern images will be Ncols * 2 + Nrows * 2 = 40.
	pub struct GrayCodePattern {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GrayCodePattern }
	
	impl Drop for GrayCodePattern {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_structured_light_GrayCodePattern_delete(self.as_raw_mut_GrayCodePattern()) };
		}
	}
	
	unsafe impl Send for GrayCodePattern {}
	
	impl core::AlgorithmTraitConst for GrayCodePattern {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for GrayCodePattern {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::structured_light::StructuredLightPatternTraitConst for GrayCodePattern {
		#[inline] fn as_raw_StructuredLightPattern(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::structured_light::StructuredLightPatternTrait for GrayCodePattern {
		#[inline] fn as_raw_mut_StructuredLightPattern(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::structured_light::GrayCodePatternTraitConst for GrayCodePattern {
		#[inline] fn as_raw_GrayCodePattern(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::structured_light::GrayCodePatternTrait for GrayCodePattern {
		#[inline] fn as_raw_mut_GrayCodePattern(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GrayCodePattern {
		/// Constructor
		/// ## Parameters
		/// * parameters: GrayCodePattern parameters GrayCodePattern::Params: the width and the height of the projector.
		/// 
		/// ## C++ default parameters
		/// * parameters: GrayCodePattern::Params()
		#[inline]
		pub fn create(parameters: &crate::structured_light::GrayCodePattern_Params) -> Result<core::Ptr<crate::structured_light::GrayCodePattern>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_structured_light_GrayCodePattern_create_const_ParamsR(parameters.as_raw_GrayCodePattern_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::structured_light::GrayCodePattern>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Constructor
		/// ## Parameters
		/// * parameters: GrayCodePattern parameters GrayCodePattern::Params: the width and the height of the projector.
		/// 
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * parameters: GrayCodePattern::Params()
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::structured_light::GrayCodePattern>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_structured_light_GrayCodePattern_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::structured_light::GrayCodePattern>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn create_1(width: i32, height: i32) -> Result<core::Ptr<crate::structured_light::GrayCodePattern>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_structured_light_GrayCodePattern_create_int_int(width, height, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::structured_light::GrayCodePattern>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { GrayCodePattern, core::Algorithm, cv_structured_light_GrayCodePattern_to_Algorithm }
	
	boxed_cast_base! { GrayCodePattern, crate::structured_light::StructuredLightPattern, cv_structured_light_GrayCodePattern_to_StructuredLightPattern }
	
	impl std::fmt::Debug for GrayCodePattern {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GrayCodePattern")
				.finish()
		}
	}
	
	/// Constant methods for [crate::structured_light::GrayCodePattern_Params]
	pub trait GrayCodePattern_ParamsTraitConst {
		fn as_raw_GrayCodePattern_Params(&self) -> *const c_void;
	
		#[inline]
		fn width(&self) -> i32 {
			let ret = unsafe { sys::cv_structured_light_GrayCodePattern_Params_propWidth_const(self.as_raw_GrayCodePattern_Params()) };
			ret
		}
		
		#[inline]
		fn height(&self) -> i32 {
			let ret = unsafe { sys::cv_structured_light_GrayCodePattern_Params_propHeight_const(self.as_raw_GrayCodePattern_Params()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::structured_light::GrayCodePattern_Params]
	pub trait GrayCodePattern_ParamsTrait: crate::structured_light::GrayCodePattern_ParamsTraitConst {
		fn as_raw_mut_GrayCodePattern_Params(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_width(&mut self, val: i32) {
			let ret = unsafe { sys::cv_structured_light_GrayCodePattern_Params_propWidth_int(self.as_raw_mut_GrayCodePattern_Params(), val) };
			ret
		}
		
		#[inline]
		fn set_height(&mut self, val: i32) {
			let ret = unsafe { sys::cv_structured_light_GrayCodePattern_Params_propHeight_int(self.as_raw_mut_GrayCodePattern_Params(), val) };
			ret
		}
		
	}
	
	/// Parameters of StructuredLightPattern constructor.
	/// ## Parameters
	/// * width: Projector's width. Default value is 1024.
	/// * height: Projector's height. Default value is 768.
	pub struct GrayCodePattern_Params {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GrayCodePattern_Params }
	
	impl Drop for GrayCodePattern_Params {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_structured_light_GrayCodePattern_Params_delete(self.as_raw_mut_GrayCodePattern_Params()) };
		}
	}
	
	unsafe impl Send for GrayCodePattern_Params {}
	
	impl crate::structured_light::GrayCodePattern_ParamsTraitConst for GrayCodePattern_Params {
		#[inline] fn as_raw_GrayCodePattern_Params(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::structured_light::GrayCodePattern_ParamsTrait for GrayCodePattern_Params {
		#[inline] fn as_raw_mut_GrayCodePattern_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GrayCodePattern_Params {
		#[inline]
		pub fn default() -> Result<crate::structured_light::GrayCodePattern_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_structured_light_GrayCodePattern_Params_Params(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::structured_light::GrayCodePattern_Params::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl std::fmt::Debug for GrayCodePattern_Params {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GrayCodePattern_Params")
				.field("width", &crate::structured_light::GrayCodePattern_ParamsTraitConst::width(self))
				.field("height", &crate::structured_light::GrayCodePattern_ParamsTraitConst::height(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::structured_light::SinusoidalPattern]
	pub trait SinusoidalPatternTraitConst: crate::structured_light::StructuredLightPatternTraitConst {
		fn as_raw_SinusoidalPattern(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::structured_light::SinusoidalPattern]
	pub trait SinusoidalPatternTrait: crate::structured_light::SinusoidalPatternTraitConst + crate::structured_light::StructuredLightPatternTrait {
		fn as_raw_mut_SinusoidalPattern(&mut self) -> *mut c_void;
	
		/// Compute a wrapped phase map from sinusoidal patterns.
		/// ## Parameters
		/// * patternImages: Input data to compute the wrapped phase map.
		/// * wrappedPhaseMap: Wrapped phase map obtained through one of the three methods.
		/// * shadowMask: Mask used to discard shadow regions.
		/// * fundamental: Fundamental matrix used to compute epipolar lines and ease the matching step.
		/// 
		/// ## C++ default parameters
		/// * shadow_mask: noArray()
		/// * fundamental: noArray()
		#[inline]
		fn compute_phase_map(&mut self, pattern_images: &impl core::ToInputArray, wrapped_phase_map: &mut impl core::ToOutputArray, shadow_mask: &mut impl core::ToOutputArray, fundamental: &impl core::ToInputArray) -> Result<()> {
			input_array_arg!(pattern_images);
			output_array_arg!(wrapped_phase_map);
			output_array_arg!(shadow_mask);
			input_array_arg!(fundamental);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_structured_light_SinusoidalPattern_computePhaseMap_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR(self.as_raw_mut_SinusoidalPattern(), pattern_images.as_raw__InputArray(), wrapped_phase_map.as_raw__OutputArray(), shadow_mask.as_raw__OutputArray(), fundamental.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Compute a wrapped phase map from sinusoidal patterns.
		/// ## Parameters
		/// * patternImages: Input data to compute the wrapped phase map.
		/// * wrappedPhaseMap: Wrapped phase map obtained through one of the three methods.
		/// * shadowMask: Mask used to discard shadow regions.
		/// * fundamental: Fundamental matrix used to compute epipolar lines and ease the matching step.
		/// 
		/// ## Note
		/// This alternative version of [compute_phase_map] function uses the following default values for its arguments:
		/// * shadow_mask: noArray()
		/// * fundamental: noArray()
		#[inline]
		fn compute_phase_map_def(&mut self, pattern_images: &impl core::ToInputArray, wrapped_phase_map: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(pattern_images);
			output_array_arg!(wrapped_phase_map);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_structured_light_SinusoidalPattern_computePhaseMap_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_SinusoidalPattern(), pattern_images.as_raw__InputArray(), wrapped_phase_map.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Unwrap the wrapped phase map to remove phase ambiguities.
		/// ## Parameters
		/// * wrappedPhaseMap: The wrapped phase map computed from the pattern.
		/// * unwrappedPhaseMap: The unwrapped phase map used to find correspondences between the two devices.
		/// * camSize: Resolution of the camera.
		/// * shadowMask: Mask used to discard shadow regions.
		/// 
		/// ## C++ default parameters
		/// * shadow_mask: noArray()
		#[inline]
		fn unwrap_phase_map(&mut self, wrapped_phase_map: &impl core::ToInputArray, unwrapped_phase_map: &mut impl core::ToOutputArray, cam_size: core::Size, shadow_mask: &impl core::ToInputArray) -> Result<()> {
			input_array_arg!(wrapped_phase_map);
			output_array_arg!(unwrapped_phase_map);
			input_array_arg!(shadow_mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_structured_light_SinusoidalPattern_unwrapPhaseMap_const__InputArrayR_const__OutputArrayR_Size_const__InputArrayR(self.as_raw_mut_SinusoidalPattern(), wrapped_phase_map.as_raw__InputArray(), unwrapped_phase_map.as_raw__OutputArray(), cam_size.opencv_as_extern(), shadow_mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Unwrap the wrapped phase map to remove phase ambiguities.
		/// ## Parameters
		/// * wrappedPhaseMap: The wrapped phase map computed from the pattern.
		/// * unwrappedPhaseMap: The unwrapped phase map used to find correspondences between the two devices.
		/// * camSize: Resolution of the camera.
		/// * shadowMask: Mask used to discard shadow regions.
		/// 
		/// ## Note
		/// This alternative version of [unwrap_phase_map] function uses the following default values for its arguments:
		/// * shadow_mask: noArray()
		#[inline]
		fn unwrap_phase_map_def(&mut self, wrapped_phase_map: &impl core::ToInputArray, unwrapped_phase_map: &mut impl core::ToOutputArray, cam_size: core::Size) -> Result<()> {
			input_array_arg!(wrapped_phase_map);
			output_array_arg!(unwrapped_phase_map);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_structured_light_SinusoidalPattern_unwrapPhaseMap_const__InputArrayR_const__OutputArrayR_Size(self.as_raw_mut_SinusoidalPattern(), wrapped_phase_map.as_raw__InputArray(), unwrapped_phase_map.as_raw__OutputArray(), cam_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Find correspondences between the two devices thanks to unwrapped phase maps.
		/// ## Parameters
		/// * projUnwrappedPhaseMap: Projector's unwrapped phase map.
		/// * camUnwrappedPhaseMap: Camera's unwrapped phase map.
		/// * matches: Images used to display correspondences map.
		#[inline]
		fn find_pro_cam_matches(&mut self, proj_unwrapped_phase_map: &impl core::ToInputArray, cam_unwrapped_phase_map: &impl core::ToInputArray, matches: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(proj_unwrapped_phase_map);
			input_array_arg!(cam_unwrapped_phase_map);
			output_array_arg!(matches);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_structured_light_SinusoidalPattern_findProCamMatches_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_SinusoidalPattern(), proj_unwrapped_phase_map.as_raw__InputArray(), cam_unwrapped_phase_map.as_raw__InputArray(), matches.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// compute the data modulation term.
		/// ## Parameters
		/// * patternImages: captured images with projected patterns.
		/// * dataModulationTerm: Mat where the data modulation term is saved.
		/// * shadowMask: Mask used to discard shadow regions.
		#[inline]
		fn compute_data_modulation_term(&mut self, pattern_images: &impl core::ToInputArray, data_modulation_term: &mut impl core::ToOutputArray, shadow_mask: &impl core::ToInputArray) -> Result<()> {
			input_array_arg!(pattern_images);
			output_array_arg!(data_modulation_term);
			input_array_arg!(shadow_mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_structured_light_SinusoidalPattern_computeDataModulationTerm_const__InputArrayR_const__OutputArrayR_const__InputArrayR(self.as_raw_mut_SinusoidalPattern(), pattern_images.as_raw__InputArray(), data_modulation_term.as_raw__OutputArray(), shadow_mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Class implementing Fourier transform profilometry (FTP) , phase-shifting profilometry (PSP)
	/// and Fourier-assisted phase-shifting profilometry (FAPS) based on [faps](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_faps).
	/// 
	/// This class generates sinusoidal patterns that can be used with FTP, PSP and FAPS.
	pub struct SinusoidalPattern {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { SinusoidalPattern }
	
	impl Drop for SinusoidalPattern {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_structured_light_SinusoidalPattern_delete(self.as_raw_mut_SinusoidalPattern()) };
		}
	}
	
	unsafe impl Send for SinusoidalPattern {}
	
	impl core::AlgorithmTraitConst for SinusoidalPattern {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for SinusoidalPattern {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::structured_light::StructuredLightPatternTraitConst for SinusoidalPattern {
		#[inline] fn as_raw_StructuredLightPattern(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::structured_light::StructuredLightPatternTrait for SinusoidalPattern {
		#[inline] fn as_raw_mut_StructuredLightPattern(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::structured_light::SinusoidalPatternTraitConst for SinusoidalPattern {
		#[inline] fn as_raw_SinusoidalPattern(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::structured_light::SinusoidalPatternTrait for SinusoidalPattern {
		#[inline] fn as_raw_mut_SinusoidalPattern(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl SinusoidalPattern {
		/// Constructor.
		/// ## Parameters
		/// * parameters: SinusoidalPattern parameters SinusoidalPattern::Params: width, height of the projector and patterns parameters.
		/// 
		/// ## C++ default parameters
		/// * parameters: makePtr<SinusoidalPattern::Params>()
		#[inline]
		pub fn create(mut parameters: core::Ptr<crate::structured_light::SinusoidalPattern_Params>) -> Result<core::Ptr<crate::structured_light::SinusoidalPattern>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_structured_light_SinusoidalPattern_create_PtrLParamsG(parameters.as_raw_mut_PtrOfSinusoidalPattern_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::structured_light::SinusoidalPattern>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Constructor.
		/// ## Parameters
		/// * parameters: SinusoidalPattern parameters SinusoidalPattern::Params: width, height of the projector and patterns parameters.
		/// 
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * parameters: makePtr<SinusoidalPattern::Params>()
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::structured_light::SinusoidalPattern>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_structured_light_SinusoidalPattern_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::structured_light::SinusoidalPattern>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { SinusoidalPattern, core::Algorithm, cv_structured_light_SinusoidalPattern_to_Algorithm }
	
	boxed_cast_base! { SinusoidalPattern, crate::structured_light::StructuredLightPattern, cv_structured_light_SinusoidalPattern_to_StructuredLightPattern }
	
	impl std::fmt::Debug for SinusoidalPattern {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("SinusoidalPattern")
				.finish()
		}
	}
	
	/// Constant methods for [crate::structured_light::SinusoidalPattern_Params]
	pub trait SinusoidalPattern_ParamsTraitConst {
		fn as_raw_SinusoidalPattern_Params(&self) -> *const c_void;
	
		#[inline]
		fn width(&self) -> i32 {
			let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_propWidth_const(self.as_raw_SinusoidalPattern_Params()) };
			ret
		}
		
		#[inline]
		fn height(&self) -> i32 {
			let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_propHeight_const(self.as_raw_SinusoidalPattern_Params()) };
			ret
		}
		
		#[inline]
		fn nbr_of_periods(&self) -> i32 {
			let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_propNbrOfPeriods_const(self.as_raw_SinusoidalPattern_Params()) };
			ret
		}
		
		#[inline]
		fn shift_value(&self) -> f32 {
			let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_propShiftValue_const(self.as_raw_SinusoidalPattern_Params()) };
			ret
		}
		
		#[inline]
		fn method_id(&self) -> i32 {
			let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_propMethodId_const(self.as_raw_SinusoidalPattern_Params()) };
			ret
		}
		
		#[inline]
		fn nbr_of_pixels_between_markers(&self) -> i32 {
			let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_propNbrOfPixelsBetweenMarkers_const(self.as_raw_SinusoidalPattern_Params()) };
			ret
		}
		
		#[inline]
		fn horizontal(&self) -> bool {
			let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_propHorizontal_const(self.as_raw_SinusoidalPattern_Params()) };
			ret
		}
		
		#[inline]
		fn set_markers(&self) -> bool {
			let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_propSetMarkers_const(self.as_raw_SinusoidalPattern_Params()) };
			ret
		}
		
		#[inline]
		fn markers_location(&self) -> core::Vector<core::Point2f> {
			let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_propMarkersLocation_const(self.as_raw_SinusoidalPattern_Params()) };
			let ret = unsafe { core::Vector::<core::Point2f>::opencv_from_extern(ret) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::structured_light::SinusoidalPattern_Params]
	pub trait SinusoidalPattern_ParamsTrait: crate::structured_light::SinusoidalPattern_ParamsTraitConst {
		fn as_raw_mut_SinusoidalPattern_Params(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_width(&mut self, val: i32) {
			let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_propWidth_int(self.as_raw_mut_SinusoidalPattern_Params(), val) };
			ret
		}
		
		#[inline]
		fn set_height(&mut self, val: i32) {
			let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_propHeight_int(self.as_raw_mut_SinusoidalPattern_Params(), val) };
			ret
		}
		
		#[inline]
		fn set_nbr_of_periods(&mut self, val: i32) {
			let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_propNbrOfPeriods_int(self.as_raw_mut_SinusoidalPattern_Params(), val) };
			ret
		}
		
		#[inline]
		fn set_shift_value(&mut self, val: f32) {
			let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_propShiftValue_float(self.as_raw_mut_SinusoidalPattern_Params(), val) };
			ret
		}
		
		#[inline]
		fn set_method_id(&mut self, val: i32) {
			let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_propMethodId_int(self.as_raw_mut_SinusoidalPattern_Params(), val) };
			ret
		}
		
		#[inline]
		fn set_nbr_of_pixels_between_markers(&mut self, val: i32) {
			let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_propNbrOfPixelsBetweenMarkers_int(self.as_raw_mut_SinusoidalPattern_Params(), val) };
			ret
		}
		
		#[inline]
		fn set_horizontal(&mut self, val: bool) {
			let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_propHorizontal_bool(self.as_raw_mut_SinusoidalPattern_Params(), val) };
			ret
		}
		
		#[inline]
		fn set_set_markers(&mut self, val: bool) {
			let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_propSetMarkers_bool(self.as_raw_mut_SinusoidalPattern_Params(), val) };
			ret
		}
		
		#[inline]
		fn set_markers_location(&mut self, mut val: core::Vector<core::Point2f>) {
			let ret = unsafe { sys::cv_structured_light_SinusoidalPattern_Params_propMarkersLocation_vectorLPoint2fG(self.as_raw_mut_SinusoidalPattern_Params(), val.as_raw_mut_VectorOfPoint2f()) };
			ret
		}
		
	}
	
	/// Parameters of SinusoidalPattern constructor
	/// ## Parameters
	/// * width: Projector's width.
	/// * height: Projector's height.
	/// * nbrOfPeriods: Number of period along the patterns direction.
	/// * shiftValue: Phase shift between two consecutive patterns.
	/// * methodId: Allow to choose between FTP, PSP and FAPS.
	/// * nbrOfPixelsBetweenMarkers: Number of pixels between two consecutive markers on the same row.
	/// * setMarkers: Allow to set markers on the patterns.
	/// * markersLocation: vector used to store markers location on the patterns.
	pub struct SinusoidalPattern_Params {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { SinusoidalPattern_Params }
	
	impl Drop for SinusoidalPattern_Params {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_structured_light_SinusoidalPattern_Params_delete(self.as_raw_mut_SinusoidalPattern_Params()) };
		}
	}
	
	unsafe impl Send for SinusoidalPattern_Params {}
	
	impl crate::structured_light::SinusoidalPattern_ParamsTraitConst for SinusoidalPattern_Params {
		#[inline] fn as_raw_SinusoidalPattern_Params(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::structured_light::SinusoidalPattern_ParamsTrait for SinusoidalPattern_Params {
		#[inline] fn as_raw_mut_SinusoidalPattern_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl SinusoidalPattern_Params {
		#[inline]
		pub fn default() -> Result<crate::structured_light::SinusoidalPattern_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_structured_light_SinusoidalPattern_Params_Params(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::structured_light::SinusoidalPattern_Params::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl std::fmt::Debug for SinusoidalPattern_Params {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("SinusoidalPattern_Params")
				.field("width", &crate::structured_light::SinusoidalPattern_ParamsTraitConst::width(self))
				.field("height", &crate::structured_light::SinusoidalPattern_ParamsTraitConst::height(self))
				.field("nbr_of_periods", &crate::structured_light::SinusoidalPattern_ParamsTraitConst::nbr_of_periods(self))
				.field("shift_value", &crate::structured_light::SinusoidalPattern_ParamsTraitConst::shift_value(self))
				.field("method_id", &crate::structured_light::SinusoidalPattern_ParamsTraitConst::method_id(self))
				.field("nbr_of_pixels_between_markers", &crate::structured_light::SinusoidalPattern_ParamsTraitConst::nbr_of_pixels_between_markers(self))
				.field("horizontal", &crate::structured_light::SinusoidalPattern_ParamsTraitConst::horizontal(self))
				.field("set_markers", &crate::structured_light::SinusoidalPattern_ParamsTraitConst::set_markers(self))
				.field("markers_location", &crate::structured_light::SinusoidalPattern_ParamsTraitConst::markers_location(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::structured_light::StructuredLightPattern]
	pub trait StructuredLightPatternTraitConst: core::AlgorithmTraitConst {
		fn as_raw_StructuredLightPattern(&self) -> *const c_void;
	
		/// Decodes the structured light pattern, generating a disparity map
		/// 
		/// ## Parameters
		/// * patternImages: The acquired pattern images to decode (vector<vector<Mat>>), loaded as grayscale and previously rectified.
		/// * disparityMap: The decoding result: a CV_64F Mat at image resolution, storing the computed disparity map.
		/// * blackImages: The all-black images needed for shadowMasks computation.
		/// * whiteImages: The all-white images needed for shadowMasks computation.
		/// * flags: Flags setting decoding algorithms. Default: DECODE_3D_UNDERWORLD.
		/// 
		/// Note: All the images must be at the same resolution.
		/// 
		/// ## C++ default parameters
		/// * black_images: noArray()
		/// * white_images: noArray()
		/// * flags: DECODE_3D_UNDERWORLD
		#[inline]
		fn decode(&self, pattern_images: &core::Vector<core::Vector<core::Mat>>, disparity_map: &mut impl core::ToOutputArray, black_images: &impl core::ToInputArray, white_images: &impl core::ToInputArray, flags: i32) -> Result<bool> {
			output_array_arg!(disparity_map);
			input_array_arg!(black_images);
			input_array_arg!(white_images);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_structured_light_StructuredLightPattern_decode_const_const_vectorLvectorLMatGGR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int(self.as_raw_StructuredLightPattern(), pattern_images.as_raw_VectorOfVectorOfMat(), disparity_map.as_raw__OutputArray(), black_images.as_raw__InputArray(), white_images.as_raw__InputArray(), flags, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Decodes the structured light pattern, generating a disparity map
		/// 
		/// ## Parameters
		/// * patternImages: The acquired pattern images to decode (vector<vector<Mat>>), loaded as grayscale and previously rectified.
		/// * disparityMap: The decoding result: a CV_64F Mat at image resolution, storing the computed disparity map.
		/// * blackImages: The all-black images needed for shadowMasks computation.
		/// * whiteImages: The all-white images needed for shadowMasks computation.
		/// * flags: Flags setting decoding algorithms. Default: DECODE_3D_UNDERWORLD.
		/// 
		/// Note: All the images must be at the same resolution.
		/// 
		/// ## Note
		/// This alternative version of [decode] function uses the following default values for its arguments:
		/// * black_images: noArray()
		/// * white_images: noArray()
		/// * flags: DECODE_3D_UNDERWORLD
		#[inline]
		fn decode_def(&self, pattern_images: &core::Vector<core::Vector<core::Mat>>, disparity_map: &mut impl core::ToOutputArray) -> Result<bool> {
			output_array_arg!(disparity_map);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_structured_light_StructuredLightPattern_decode_const_const_vectorLvectorLMatGGR_const__OutputArrayR(self.as_raw_StructuredLightPattern(), pattern_images.as_raw_VectorOfVectorOfMat(), disparity_map.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::structured_light::StructuredLightPattern]
	pub trait StructuredLightPatternTrait: core::AlgorithmTrait + crate::structured_light::StructuredLightPatternTraitConst {
		fn as_raw_mut_StructuredLightPattern(&mut self) -> *mut c_void;
	
		/// Generates the structured light pattern to project.
		/// 
		/// ## Parameters
		/// * patternImages: The generated pattern: a vector<Mat>, in which each image is a CV_8U Mat at projector's resolution.
		#[inline]
		fn generate(&mut self, pattern_images: &mut impl core::ToOutputArray) -> Result<bool> {
			output_array_arg!(pattern_images);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_structured_light_StructuredLightPattern_generate_const__OutputArrayR(self.as_raw_mut_StructuredLightPattern(), pattern_images.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Abstract base class for generating and decoding structured light patterns.
	pub struct StructuredLightPattern {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { StructuredLightPattern }
	
	impl Drop for StructuredLightPattern {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_structured_light_StructuredLightPattern_delete(self.as_raw_mut_StructuredLightPattern()) };
		}
	}
	
	unsafe impl Send for StructuredLightPattern {}
	
	impl core::AlgorithmTraitConst for StructuredLightPattern {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for StructuredLightPattern {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::structured_light::StructuredLightPatternTraitConst for StructuredLightPattern {
		#[inline] fn as_raw_StructuredLightPattern(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::structured_light::StructuredLightPatternTrait for StructuredLightPattern {
		#[inline] fn as_raw_mut_StructuredLightPattern(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl StructuredLightPattern {
	}
	
	boxed_cast_descendant! { StructuredLightPattern, crate::structured_light::GrayCodePattern, cv_structured_light_StructuredLightPattern_to_GrayCodePattern }
	
	boxed_cast_descendant! { StructuredLightPattern, crate::structured_light::SinusoidalPattern, cv_structured_light_StructuredLightPattern_to_SinusoidalPattern }
	
	boxed_cast_base! { StructuredLightPattern, core::Algorithm, cv_structured_light_StructuredLightPattern_to_Algorithm }
	
	impl std::fmt::Debug for StructuredLightPattern {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("StructuredLightPattern")
				.finish()
		}
	}
}
