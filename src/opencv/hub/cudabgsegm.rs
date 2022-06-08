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
	pub use { super::CUDA_BackgroundSubtractorMOGConst, super::CUDA_BackgroundSubtractorMOG, super::CUDA_BackgroundSubtractorMOG2Const, super::CUDA_BackgroundSubtractorMOG2 };
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
#[inline]
pub fn create_background_subtractor_mog2(history: i32, var_threshold: f64, detect_shadows: bool) -> Result<core::Ptr<dyn crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_createBackgroundSubtractorMOG2_int_double_bool(history, var_threshold, detect_shadows, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2>::opencv_from_extern(ret) };
	Ok(ret)
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
#[inline]
pub fn create_background_subtractor_mog(history: i32, nmixtures: i32, background_ratio: f64, noise_sigma: f64) -> Result<core::Ptr<dyn crate::cudabgsegm::CUDA_BackgroundSubtractorMOG>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_createBackgroundSubtractorMOG_int_int_double_double(history, nmixtures, background_ratio, noise_sigma, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::cudabgsegm::CUDA_BackgroundSubtractorMOG>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Gaussian Mixture-based Background/Foreground Segmentation Algorithm.
/// 
/// The class discriminates between foreground and background pixels by building and maintaining a model
/// of the background. Any pixel which does not fit this model is then deemed to be foreground. The
/// class implements algorithm described in [MOG2001](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_MOG2001) .
/// ## See also
/// BackgroundSubtractorMOG
/// 
/// 
/// Note:
///    *   An example on gaussian mixture based background/foreground segmantation can be found at
///        opencv_source_code/samples/gpu/bgfg_segm.cpp
pub trait CUDA_BackgroundSubtractorMOGConst: crate::video::BackgroundSubtractorConst {
	fn as_raw_CUDA_BackgroundSubtractorMOG(&self) -> *const c_void;

	#[inline]
	fn get_background_image(&self, background_image: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		output_array_arg!(background_image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_getBackgroundImage_const_const__OutputArrayR_StreamR(self.as_raw_CUDA_BackgroundSubtractorMOG(), background_image.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_history(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_getHistory_const(self.as_raw_CUDA_BackgroundSubtractorMOG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_n_mixtures(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_getNMixtures_const(self.as_raw_CUDA_BackgroundSubtractorMOG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_background_ratio(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_getBackgroundRatio_const(self.as_raw_CUDA_BackgroundSubtractorMOG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_noise_sigma(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_getNoiseSigma_const(self.as_raw_CUDA_BackgroundSubtractorMOG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait CUDA_BackgroundSubtractorMOG: crate::cudabgsegm::CUDA_BackgroundSubtractorMOGConst + crate::video::BackgroundSubtractor {
	fn as_raw_mut_CUDA_BackgroundSubtractorMOG(&mut self) -> *mut c_void;

	#[inline]
	fn apply(&mut self, image: &dyn core::ToInputArray, fgmask: &mut dyn core::ToOutputArray, learning_rate: f64, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(fgmask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_apply_const__InputArrayR_const__OutputArrayR_double_StreamR(self.as_raw_mut_CUDA_BackgroundSubtractorMOG(), image.as_raw__InputArray(), fgmask.as_raw__OutputArray(), learning_rate, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_background_image_1(&mut self, background_image: &mut core::GpuMat, stream: &mut core::Stream) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_getBackgroundImage_GpuMatR_StreamR(self.as_raw_mut_CUDA_BackgroundSubtractorMOG(), background_image.as_raw_mut_GpuMat(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_history(&mut self, nframes: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_setHistory_int(self.as_raw_mut_CUDA_BackgroundSubtractorMOG(), nframes, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_n_mixtures(&mut self, nmix: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_setNMixtures_int(self.as_raw_mut_CUDA_BackgroundSubtractorMOG(), nmix, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_background_ratio(&mut self, background_ratio: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_setBackgroundRatio_double(self.as_raw_mut_CUDA_BackgroundSubtractorMOG(), background_ratio, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_noise_sigma(&mut self, noise_sigma: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_setNoiseSigma_double(self.as_raw_mut_CUDA_BackgroundSubtractorMOG(), noise_sigma, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Gaussian Mixture-based Background/Foreground Segmentation Algorithm.
/// 
/// The class discriminates between foreground and background pixels by building and maintaining a model
/// of the background. Any pixel which does not fit this model is then deemed to be foreground. The
/// class implements algorithm described in [Zivkovic2004](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Zivkovic2004) .
/// ## See also
/// BackgroundSubtractorMOG2
pub trait CUDA_BackgroundSubtractorMOG2Const: crate::video::BackgroundSubtractorMOG2Const {
	fn as_raw_CUDA_BackgroundSubtractorMOG2(&self) -> *const c_void;

	#[inline]
	fn get_background_image(&self, background_image: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		output_array_arg!(background_image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG2_getBackgroundImage_const_const__OutputArrayR_StreamR(self.as_raw_CUDA_BackgroundSubtractorMOG2(), background_image.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait CUDA_BackgroundSubtractorMOG2: crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2Const + crate::video::BackgroundSubtractorMOG2 {
	fn as_raw_mut_CUDA_BackgroundSubtractorMOG2(&mut self) -> *mut c_void;

	#[inline]
	fn apply(&mut self, image: &dyn core::ToInputArray, fgmask: &mut dyn core::ToOutputArray, learning_rate: f64, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(fgmask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG2_apply_const__InputArrayR_const__OutputArrayR_double_StreamR(self.as_raw_mut_CUDA_BackgroundSubtractorMOG2(), image.as_raw__InputArray(), fgmask.as_raw__OutputArray(), learning_rate, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_background_image_1(&mut self, background_image: &mut core::GpuMat, stream: &mut core::Stream) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG2_getBackgroundImage_GpuMatR_StreamR(self.as_raw_mut_CUDA_BackgroundSubtractorMOG2(), background_image.as_raw_mut_GpuMat(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}
