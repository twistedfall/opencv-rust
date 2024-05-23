//! # Background Segmentation
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{CUDA_BackgroundSubtractorMOG2Trait, CUDA_BackgroundSubtractorMOG2TraitConst, CUDA_BackgroundSubtractorMOGTrait, CUDA_BackgroundSubtractorMOGTraitConst};
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
/// ## Note
/// This alternative version of [create_background_subtractor_mog] function uses the following default values for its arguments:
/// * history: 200
/// * nmixtures: 5
/// * background_ratio: 0.7
/// * noise_sigma: 0
// cv::cuda::createBackgroundSubtractorMOG() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudabgsegm.hpp:116
// ("cv::cuda::createBackgroundSubtractorMOG", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn create_background_subtractor_mog_def() -> Result<core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_createBackgroundSubtractorMOG(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG>::opencv_from_extern(ret) };
	Ok(ret)
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
/// ## Note
/// This alternative version of [create_background_subtractor_mog2] function uses the following default values for its arguments:
/// * history: 500
/// * var_threshold: 16
/// * detect_shadows: true
// cv::cuda::createBackgroundSubtractorMOG2() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudabgsegm.hpp:155
// ("cv::cuda::createBackgroundSubtractorMOG2", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn create_background_subtractor_mog2_def() -> Result<core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_createBackgroundSubtractorMOG2(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2>::opencv_from_extern(ret) };
	Ok(ret)
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
// createBackgroundSubtractorMOG2(int, double, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudabgsegm.hpp:155
// ("cv::cuda::createBackgroundSubtractorMOG2", vec![(pred!(mut, ["history", "varThreshold", "detectShadows"], ["int", "double", "bool"]), _)]),
#[inline]
pub fn create_background_subtractor_mog2(history: i32, var_threshold: f64, detect_shadows: bool) -> Result<core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_createBackgroundSubtractorMOG2_int_double_bool(history, var_threshold, detect_shadows, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2>::opencv_from_extern(ret) };
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
// createBackgroundSubtractorMOG(int, int, double, double)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudabgsegm.hpp:116
// ("cv::cuda::createBackgroundSubtractorMOG", vec![(pred!(mut, ["history", "nmixtures", "backgroundRatio", "noiseSigma"], ["int", "int", "double", "double"]), _)]),
#[inline]
pub fn create_background_subtractor_mog(history: i32, nmixtures: i32, background_ratio: f64, noise_sigma: f64) -> Result<core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cuda_createBackgroundSubtractorMOG_int_int_double_double(history, nmixtures, background_ratio, noise_sigma, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Constant methods for [crate::cudabgsegm::CUDA_BackgroundSubtractorMOG]
// BackgroundSubtractorMOG /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudabgsegm.hpp:80
pub trait CUDA_BackgroundSubtractorMOGTraitConst: crate::video::BackgroundSubtractorTraitConst {
	fn as_raw_CUDA_BackgroundSubtractorMOG(&self) -> *const c_void;

	// getBackgroundImage(OutputArray, Stream &)(OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudabgsegm.hpp:88
	// ("cv::cuda::BackgroundSubtractorMOG::getBackgroundImage", vec![(pred!(const, ["backgroundImage", "stream"], ["const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	#[inline]
	fn get_background_image(&self, background_image: &mut impl ToOutputArray, stream: &mut impl core::StreamTrait) -> Result<()> {
		output_array_arg!(background_image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_getBackgroundImage_const_const__OutputArrayR_StreamR(self.as_raw_CUDA_BackgroundSubtractorMOG(), background_image.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getHistory()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudabgsegm.hpp:94
	// ("cv::cuda::BackgroundSubtractorMOG::getHistory", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_history(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_getHistory_const(self.as_raw_CUDA_BackgroundSubtractorMOG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNMixtures()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudabgsegm.hpp:97
	// ("cv::cuda::BackgroundSubtractorMOG::getNMixtures", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_n_mixtures(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_getNMixtures_const(self.as_raw_CUDA_BackgroundSubtractorMOG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getBackgroundRatio()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudabgsegm.hpp:100
	// ("cv::cuda::BackgroundSubtractorMOG::getBackgroundRatio", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_background_ratio(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_getBackgroundRatio_const(self.as_raw_CUDA_BackgroundSubtractorMOG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNoiseSigma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudabgsegm.hpp:103
	// ("cv::cuda::BackgroundSubtractorMOG::getNoiseSigma", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_noise_sigma(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_getNoiseSigma_const(self.as_raw_CUDA_BackgroundSubtractorMOG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::cudabgsegm::CUDA_BackgroundSubtractorMOG]
pub trait CUDA_BackgroundSubtractorMOGTrait: crate::cudabgsegm::CUDA_BackgroundSubtractorMOGTraitConst + crate::video::BackgroundSubtractorTrait {
	fn as_raw_mut_CUDA_BackgroundSubtractorMOG(&mut self) -> *mut c_void;

	// apply(InputArray, OutputArray, double, Stream &)(InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudabgsegm.hpp:85
	// ("cv::cuda::BackgroundSubtractorMOG::apply", vec![(pred!(mut, ["image", "fgmask", "learningRate", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "cv::cuda::Stream*"]), _)]),
	#[inline]
	fn apply(&mut self, image: &impl ToInputArray, fgmask: &mut impl ToOutputArray, learning_rate: f64, stream: &mut impl core::StreamTrait) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(fgmask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_apply_const__InputArrayR_const__OutputArrayR_double_StreamR(self.as_raw_mut_CUDA_BackgroundSubtractorMOG(), image.as_raw__InputArray(), fgmask.as_raw__OutputArray(), learning_rate, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getBackgroundImage(GpuMat &, Stream &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudabgsegm.hpp:90
	// ("cv::cuda::BackgroundSubtractorMOG::getBackgroundImage", vec![(pred!(mut, ["backgroundImage", "stream"], ["cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
	#[inline]
	fn get_background_image_1(&mut self, background_image: &mut impl core::GpuMatTrait, stream: &mut impl core::StreamTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_getBackgroundImage_GpuMatR_StreamR(self.as_raw_mut_CUDA_BackgroundSubtractorMOG(), background_image.as_raw_mut_GpuMat(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setHistory(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudabgsegm.hpp:95
	// ("cv::cuda::BackgroundSubtractorMOG::setHistory", vec![(pred!(mut, ["nframes"], ["int"]), _)]),
	#[inline]
	fn set_history(&mut self, nframes: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_setHistory_int(self.as_raw_mut_CUDA_BackgroundSubtractorMOG(), nframes, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNMixtures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudabgsegm.hpp:98
	// ("cv::cuda::BackgroundSubtractorMOG::setNMixtures", vec![(pred!(mut, ["nmix"], ["int"]), _)]),
	#[inline]
	fn set_n_mixtures(&mut self, nmix: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_setNMixtures_int(self.as_raw_mut_CUDA_BackgroundSubtractorMOG(), nmix, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setBackgroundRatio(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudabgsegm.hpp:101
	// ("cv::cuda::BackgroundSubtractorMOG::setBackgroundRatio", vec![(pred!(mut, ["backgroundRatio"], ["double"]), _)]),
	#[inline]
	fn set_background_ratio(&mut self, background_ratio: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_setBackgroundRatio_double(self.as_raw_mut_CUDA_BackgroundSubtractorMOG(), background_ratio, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNoiseSigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudabgsegm.hpp:104
	// ("cv::cuda::BackgroundSubtractorMOG::setNoiseSigma", vec![(pred!(mut, ["noiseSigma"], ["double"]), _)]),
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
/// class implements algorithm described in [MOG2001](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_MOG2001) .
/// ## See also
/// BackgroundSubtractorMOG
///
///
/// Note:
///    *   An example on gaussian mixture based background/foreground segmantation can be found at
///        opencv_source_code/samples/gpu/bgfg_segm.cpp
// BackgroundSubtractorMOG /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudabgsegm.hpp:80
pub struct CUDA_BackgroundSubtractorMOG {
	ptr: *mut c_void,
}

opencv_type_boxed! { CUDA_BackgroundSubtractorMOG }

impl Drop for CUDA_BackgroundSubtractorMOG {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG_delete(self.as_raw_mut_CUDA_BackgroundSubtractorMOG()) };
	}
}

unsafe impl Send for CUDA_BackgroundSubtractorMOG {}

impl core::AlgorithmTraitConst for CUDA_BackgroundSubtractorMOG {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for CUDA_BackgroundSubtractorMOG {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CUDA_BackgroundSubtractorMOG, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::video::BackgroundSubtractorTraitConst for CUDA_BackgroundSubtractorMOG {
	#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::BackgroundSubtractorTrait for CUDA_BackgroundSubtractorMOG {
	#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CUDA_BackgroundSubtractorMOG, crate::video::BackgroundSubtractorTraitConst, as_raw_BackgroundSubtractor, crate::video::BackgroundSubtractorTrait, as_raw_mut_BackgroundSubtractor }

impl crate::cudabgsegm::CUDA_BackgroundSubtractorMOGTraitConst for CUDA_BackgroundSubtractorMOG {
	#[inline] fn as_raw_CUDA_BackgroundSubtractorMOG(&self) -> *const c_void { self.as_raw() }
}

impl crate::cudabgsegm::CUDA_BackgroundSubtractorMOGTrait for CUDA_BackgroundSubtractorMOG {
	#[inline] fn as_raw_mut_CUDA_BackgroundSubtractorMOG(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CUDA_BackgroundSubtractorMOG, crate::cudabgsegm::CUDA_BackgroundSubtractorMOGTraitConst, as_raw_CUDA_BackgroundSubtractorMOG, crate::cudabgsegm::CUDA_BackgroundSubtractorMOGTrait, as_raw_mut_CUDA_BackgroundSubtractorMOG }

impl CUDA_BackgroundSubtractorMOG {
}

boxed_cast_base! { CUDA_BackgroundSubtractorMOG, core::Algorithm, cv_cuda_BackgroundSubtractorMOG_to_Algorithm }

boxed_cast_base! { CUDA_BackgroundSubtractorMOG, crate::video::BackgroundSubtractor, cv_cuda_BackgroundSubtractorMOG_to_BackgroundSubtractor }

impl std::fmt::Debug for CUDA_BackgroundSubtractorMOG {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("CUDA_BackgroundSubtractorMOG")
			.finish()
	}
}

/// Constant methods for [crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2]
// BackgroundSubtractorMOG2 /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudabgsegm.hpp:130
pub trait CUDA_BackgroundSubtractorMOG2TraitConst: crate::video::BackgroundSubtractorMOG2TraitConst {
	fn as_raw_CUDA_BackgroundSubtractorMOG2(&self) -> *const c_void;

	// getBackgroundImage(OutputArray, Stream &)(OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudabgsegm.hpp:138
	// ("cv::cuda::BackgroundSubtractorMOG2::getBackgroundImage", vec![(pred!(const, ["backgroundImage", "stream"], ["const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	#[inline]
	fn get_background_image(&self, background_image: &mut impl ToOutputArray, stream: &mut impl core::StreamTrait) -> Result<()> {
		output_array_arg!(background_image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG2_getBackgroundImage_const_const__OutputArrayR_StreamR(self.as_raw_CUDA_BackgroundSubtractorMOG2(), background_image.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2]
pub trait CUDA_BackgroundSubtractorMOG2Trait: crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2TraitConst + crate::video::BackgroundSubtractorMOG2Trait {
	fn as_raw_mut_CUDA_BackgroundSubtractorMOG2(&mut self) -> *mut c_void;

	// apply(InputArray, OutputArray, double, Stream &)(InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudabgsegm.hpp:136
	// ("cv::cuda::BackgroundSubtractorMOG2::apply", vec![(pred!(mut, ["image", "fgmask", "learningRate", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "cv::cuda::Stream*"]), _)]),
	#[inline]
	fn apply(&mut self, image: &impl ToInputArray, fgmask: &mut impl ToOutputArray, learning_rate: f64, stream: &mut impl core::StreamTrait) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(fgmask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG2_apply_const__InputArrayR_const__OutputArrayR_double_StreamR(self.as_raw_mut_CUDA_BackgroundSubtractorMOG2(), image.as_raw__InputArray(), fgmask.as_raw__OutputArray(), learning_rate, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getBackgroundImage(GpuMat &, Stream &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudabgsegm.hpp:140
	// ("cv::cuda::BackgroundSubtractorMOG2::getBackgroundImage", vec![(pred!(mut, ["backgroundImage", "stream"], ["cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
	#[inline]
	fn get_background_image_1(&mut self, background_image: &mut impl core::GpuMatTrait, stream: &mut impl core::StreamTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG2_getBackgroundImage_GpuMatR_StreamR(self.as_raw_mut_CUDA_BackgroundSubtractorMOG2(), background_image.as_raw_mut_GpuMat(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Gaussian Mixture-based Background/Foreground Segmentation Algorithm.
///
/// The class discriminates between foreground and background pixels by building and maintaining a model
/// of the background. Any pixel which does not fit this model is then deemed to be foreground. The
/// class implements algorithm described in [Zivkovic2004](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Zivkovic2004) .
/// ## See also
/// BackgroundSubtractorMOG2
// BackgroundSubtractorMOG2 /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudabgsegm.hpp:130
pub struct CUDA_BackgroundSubtractorMOG2 {
	ptr: *mut c_void,
}

opencv_type_boxed! { CUDA_BackgroundSubtractorMOG2 }

impl Drop for CUDA_BackgroundSubtractorMOG2 {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_cuda_BackgroundSubtractorMOG2_delete(self.as_raw_mut_CUDA_BackgroundSubtractorMOG2()) };
	}
}

unsafe impl Send for CUDA_BackgroundSubtractorMOG2 {}

impl core::AlgorithmTraitConst for CUDA_BackgroundSubtractorMOG2 {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for CUDA_BackgroundSubtractorMOG2 {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CUDA_BackgroundSubtractorMOG2, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::video::BackgroundSubtractorTraitConst for CUDA_BackgroundSubtractorMOG2 {
	#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::BackgroundSubtractorTrait for CUDA_BackgroundSubtractorMOG2 {
	#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CUDA_BackgroundSubtractorMOG2, crate::video::BackgroundSubtractorTraitConst, as_raw_BackgroundSubtractor, crate::video::BackgroundSubtractorTrait, as_raw_mut_BackgroundSubtractor }

impl crate::video::BackgroundSubtractorMOG2TraitConst for CUDA_BackgroundSubtractorMOG2 {
	#[inline] fn as_raw_BackgroundSubtractorMOG2(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::BackgroundSubtractorMOG2Trait for CUDA_BackgroundSubtractorMOG2 {
	#[inline] fn as_raw_mut_BackgroundSubtractorMOG2(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CUDA_BackgroundSubtractorMOG2, crate::video::BackgroundSubtractorMOG2TraitConst, as_raw_BackgroundSubtractorMOG2, crate::video::BackgroundSubtractorMOG2Trait, as_raw_mut_BackgroundSubtractorMOG2 }

impl crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2TraitConst for CUDA_BackgroundSubtractorMOG2 {
	#[inline] fn as_raw_CUDA_BackgroundSubtractorMOG2(&self) -> *const c_void { self.as_raw() }
}

impl crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2Trait for CUDA_BackgroundSubtractorMOG2 {
	#[inline] fn as_raw_mut_CUDA_BackgroundSubtractorMOG2(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CUDA_BackgroundSubtractorMOG2, crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2TraitConst, as_raw_CUDA_BackgroundSubtractorMOG2, crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2Trait, as_raw_mut_CUDA_BackgroundSubtractorMOG2 }

impl CUDA_BackgroundSubtractorMOG2 {
}

boxed_cast_base! { CUDA_BackgroundSubtractorMOG2, core::Algorithm, cv_cuda_BackgroundSubtractorMOG2_to_Algorithm }

boxed_cast_base! { CUDA_BackgroundSubtractorMOG2, crate::video::BackgroundSubtractor, cv_cuda_BackgroundSubtractorMOG2_to_BackgroundSubtractor }

boxed_cast_base! { CUDA_BackgroundSubtractorMOG2, crate::video::BackgroundSubtractorMOG2, cv_cuda_BackgroundSubtractorMOG2_to_BackgroundSubtractorMOG2 }

impl std::fmt::Debug for CUDA_BackgroundSubtractorMOG2 {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("CUDA_BackgroundSubtractorMOG2")
			.finish()
	}
}
