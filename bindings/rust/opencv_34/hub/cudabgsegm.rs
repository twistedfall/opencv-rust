#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Background Segmentation
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::CUDA_BackgroundSubtractorMOG, super::CUDA_BackgroundSubtractorMOG2 };
}

/// Creates MOG2 Background Subtractor
/// 
/// ## Parameters
/// * history: Length of the history.
/// * varThreshold: Threshold on the squared Mahalanobis distance between the pixel and the model
/// to decide whether a pixel is well described by the background model. This parameter does not
/// affect the background update.
/// * detectShadows: If true, the algorithm will detect shadows and mark them. It decreases the
/// speed a bit, so if you do not need this feature, set the parameter to false.
/// 
/// ## C++ default parameters
/// * history: 500
/// * var_threshold: 16
/// * detect_shadows: true
pub fn create_background_subtractor_mog2(history: i32, var_threshold: f64, detect_shadows: bool) -> Result<core::Ptr::<dyn crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2>> {
	unsafe { sys::cv_cuda_createBackgroundSubtractorMOG2_int_double_bool(history, var_threshold, detect_shadows) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2>::opencv_from_extern(r) } )
}

/// Creates mixture-of-gaussian background subtractor
/// 
/// ## Parameters
/// * history: Length of the history.
/// * nmixtures: Number of Gaussian mixtures.
/// * backgroundRatio: Background ratio.
/// * noiseSigma: Noise strength (standard deviation of the brightness or each color channel). 0
/// means some automatic value.
/// 
/// ## C++ default parameters
/// * history: 200
/// * nmixtures: 5
/// * background_ratio: 0.7
/// * noise_sigma: 0
pub fn create_background_subtractor_mog(history: i32, nmixtures: i32, background_ratio: f64, noise_sigma: f64) -> Result<core::Ptr::<dyn crate::cudabgsegm::CUDA_BackgroundSubtractorMOG>> {
	unsafe { sys::cv_cuda_createBackgroundSubtractorMOG_int_int_double_double(history, nmixtures, background_ratio, noise_sigma) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudabgsegm::CUDA_BackgroundSubtractorMOG>::opencv_from_extern(r) } )
}

/// Gaussian Mixture-based Background/Foreground Segmentation Algorithm.
/// 
/// The class discriminates between foreground and background pixels by building and maintaining a model
/// of the background. Any pixel which does not fit this model is then deemed to be foreground. The
/// class implements algorithm described in [MOG2001](https://docs.opencv.org/3.4.10/d0/de3/citelist.html#CITEREF_MOG2001) .
/// ## See also
/// BackgroundSubtractorMOG
/// 
/// 
/// Note:
///    *   An example on gaussian mixture based background/foreground segmantation can be found at
///        opencv_source_code/samples/gpu/bgfg_segm.cpp
pub trait CUDA_BackgroundSubtractorMOG: crate::video::BackgroundSubtractor {
	fn as_raw_CUDA_BackgroundSubtractorMOG(&self) -> *const c_void;
	fn as_raw_mut_CUDA_BackgroundSubtractorMOG(&mut self) -> *mut c_void;

	fn apply(&mut self, image: &dyn core::ToInputArray, fgmask: &mut dyn core::ToOutputArray, learning_rate: f64, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(fgmask);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_apply_const__InputArrayR_const__OutputArrayR_double_StreamR(self.as_raw_mut_CUDA_BackgroundSubtractorMOG(), image.as_raw__InputArray(), fgmask.as_raw__OutputArray(), learning_rate, stream.as_raw_mut_Stream()) }.into_result()
	}
	
	fn get_background_image(&self, background_image: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		output_array_arg!(background_image);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_getBackgroundImage_const_const__OutputArrayR_StreamR(self.as_raw_CUDA_BackgroundSubtractorMOG(), background_image.as_raw__OutputArray(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
	fn get_history(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_getHistory_const(self.as_raw_CUDA_BackgroundSubtractorMOG()) }.into_result()
	}
	
	fn set_history(&mut self, nframes: i32) -> Result<()> {
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_setHistory_int(self.as_raw_mut_CUDA_BackgroundSubtractorMOG(), nframes) }.into_result()
	}
	
	fn get_n_mixtures(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_getNMixtures_const(self.as_raw_CUDA_BackgroundSubtractorMOG()) }.into_result()
	}
	
	fn set_n_mixtures(&mut self, nmix: i32) -> Result<()> {
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_setNMixtures_int(self.as_raw_mut_CUDA_BackgroundSubtractorMOG(), nmix) }.into_result()
	}
	
	fn get_background_ratio(&self) -> Result<f64> {
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_getBackgroundRatio_const(self.as_raw_CUDA_BackgroundSubtractorMOG()) }.into_result()
	}
	
	fn set_background_ratio(&mut self, background_ratio: f64) -> Result<()> {
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_setBackgroundRatio_double(self.as_raw_mut_CUDA_BackgroundSubtractorMOG(), background_ratio) }.into_result()
	}
	
	fn get_noise_sigma(&self) -> Result<f64> {
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_getNoiseSigma_const(self.as_raw_CUDA_BackgroundSubtractorMOG()) }.into_result()
	}
	
	fn set_noise_sigma(&mut self, noise_sigma: f64) -> Result<()> {
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_setNoiseSigma_double(self.as_raw_mut_CUDA_BackgroundSubtractorMOG(), noise_sigma) }.into_result()
	}
	
}

/// Gaussian Mixture-based Background/Foreground Segmentation Algorithm.
/// 
/// The class discriminates between foreground and background pixels by building and maintaining a model
/// of the background. Any pixel which does not fit this model is then deemed to be foreground. The
/// class implements algorithm described in [Zivkovic2004](https://docs.opencv.org/3.4.10/d0/de3/citelist.html#CITEREF_Zivkovic2004) .
/// ## See also
/// BackgroundSubtractorMOG2
pub trait CUDA_BackgroundSubtractorMOG2: crate::video::BackgroundSubtractorMOG2 {
	fn as_raw_CUDA_BackgroundSubtractorMOG2(&self) -> *const c_void;
	fn as_raw_mut_CUDA_BackgroundSubtractorMOG2(&mut self) -> *mut c_void;

	fn apply(&mut self, image: &dyn core::ToInputArray, fgmask: &mut dyn core::ToOutputArray, learning_rate: f64, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(fgmask);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG2_apply_const__InputArrayR_const__OutputArrayR_double_StreamR(self.as_raw_mut_CUDA_BackgroundSubtractorMOG2(), image.as_raw__InputArray(), fgmask.as_raw__OutputArray(), learning_rate, stream.as_raw_mut_Stream()) }.into_result()
	}
	
	fn get_background_image(&self, background_image: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		output_array_arg!(background_image);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG2_getBackgroundImage_const_const__OutputArrayR_StreamR(self.as_raw_CUDA_BackgroundSubtractorMOG2(), background_image.as_raw__OutputArray(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
}
