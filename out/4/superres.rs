//! # Super Resolution
//!
//! The Super Resolution module contains a set of functions and classes that can be used to solve the
//! problem of resolution enhancement. There are a few methods implemented, most of them are described in
//! the papers [Farsiu03](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Farsiu03) and [Mitzel09](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Mitzel09) .
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{SuperRes_BroxOpticalFlowTrait, SuperRes_BroxOpticalFlowTraitConst, SuperRes_DenseOpticalFlowExtTrait, SuperRes_DenseOpticalFlowExtTraitConst, SuperRes_DualTVL1OpticalFlowTrait, SuperRes_DualTVL1OpticalFlowTraitConst, SuperRes_FarnebackOpticalFlowTrait, SuperRes_FarnebackOpticalFlowTraitConst, SuperRes_FrameSourceTrait, SuperRes_FrameSourceTraitConst, SuperRes_PyrLKOpticalFlowTrait, SuperRes_PyrLKOpticalFlowTraitConst, SuperRes_SuperResolutionTrait, SuperRes_SuperResolutionTraitConst};
}

/// ## Note
/// This alternative version of [create_frame_source_camera] function uses the following default values for its arguments:
/// * device_id: 0
// cv::superres::createFrameSource_Camera() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:80
// ("cv::superres::createFrameSource_Camera", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn create_frame_source_camera_def() -> Result<core::Ptr<crate::superres::SuperRes_FrameSource>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_superres_createFrameSource_Camera(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::superres::SuperRes_FrameSource>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * device_id: 0
// createFrameSource_Camera(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:80
// ("cv::superres::createFrameSource_Camera", vec![(pred!(mut, ["deviceId"], ["int"]), _)]),
#[inline]
pub fn create_frame_source_camera(device_id: i32) -> Result<core::Ptr<crate::superres::SuperRes_FrameSource>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_superres_createFrameSource_Camera_int(device_id, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::superres::SuperRes_FrameSource>::opencv_from_extern(ret) };
	Ok(ret)
}

// createFrameSource_Empty()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:75
// ("cv::superres::createFrameSource_Empty", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn create_frame_source_empty() -> Result<core::Ptr<crate::superres::SuperRes_FrameSource>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_superres_createFrameSource_Empty(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::superres::SuperRes_FrameSource>::opencv_from_extern(ret) };
	Ok(ret)
}

// createFrameSource_Video_CUDA(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:78
// ("cv::superres::createFrameSource_Video_CUDA", vec![(pred!(mut, ["fileName"], ["const cv::String*"]), _)]),
#[inline]
pub fn create_frame_source_video_cuda(file_name: &str) -> Result<core::Ptr<crate::superres::SuperRes_FrameSource>> {
	extern_container_arg!(file_name);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_superres_createFrameSource_Video_CUDA_const_StringR(file_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::superres::SuperRes_FrameSource>::opencv_from_extern(ret) };
	Ok(ret)
}

// createFrameSource_Video(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:77
// ("cv::superres::createFrameSource_Video", vec![(pred!(mut, ["fileName"], ["const cv::String*"]), _)]),
#[inline]
pub fn create_frame_source_video(file_name: &str) -> Result<core::Ptr<crate::superres::SuperRes_FrameSource>> {
	extern_container_arg!(file_name);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_superres_createFrameSource_Video_const_StringR(file_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::superres::SuperRes_FrameSource>::opencv_from_extern(ret) };
	Ok(ret)
}

// createOptFlow_Brox_CUDA()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:177
// ("cv::superres::createOptFlow_Brox_CUDA", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn create_opt_flow_brox_cuda() -> Result<core::Ptr<crate::superres::SuperRes_BroxOpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_superres_createOptFlow_Brox_CUDA(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::superres::SuperRes_BroxOpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

// createOptFlow_DualTVL1()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:139
// ("cv::superres::createOptFlow_DualTVL1", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn create_opt_flow_dual_tvl1() -> Result<core::Ptr<crate::superres::SuperRes_DualTVL1OpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_superres_createOptFlow_DualTVL1(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::superres::SuperRes_DualTVL1OpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

// createOptFlow_DualTVL1_CUDA()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:140
// ("cv::superres::createOptFlow_DualTVL1_CUDA", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn create_opt_flow_dual_tvl1_cuda() -> Result<core::Ptr<crate::superres::SuperRes_DualTVL1OpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_superres_createOptFlow_DualTVL1_CUDA(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::superres::SuperRes_DualTVL1OpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

// createOptFlow_Farneback()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:96
// ("cv::superres::createOptFlow_Farneback", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn create_opt_flow_farneback() -> Result<core::Ptr<crate::superres::SuperRes_FarnebackOpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_superres_createOptFlow_Farneback(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::superres::SuperRes_FarnebackOpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

// createOptFlow_Farneback_CUDA()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:97
// ("cv::superres::createOptFlow_Farneback_CUDA", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn create_opt_flow_farneback_cuda() -> Result<core::Ptr<crate::superres::SuperRes_FarnebackOpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_superres_createOptFlow_Farneback_CUDA(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::superres::SuperRes_FarnebackOpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

// createOptFlow_PyrLK_CUDA()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:196
// ("cv::superres::createOptFlow_PyrLK_CUDA", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn create_opt_flow_pyr_lk_cuda() -> Result<core::Ptr<crate::superres::SuperRes_PyrLKOpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_superres_createOptFlow_PyrLK_CUDA(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::superres::SuperRes_PyrLKOpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Create Bilateral TV-L1 Super Resolution.
///
/// This class implements Super Resolution algorithm described in the papers [Farsiu03](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Farsiu03) and
/// [Mitzel09](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Mitzel09) .
///
/// Here are important members of the class that control the algorithm, which you can set after
/// constructing the class instance:
///
/// *   **int scale** Scale factor.
/// *   **int iterations** Iteration count.
/// *   **double tau** Asymptotic value of steepest descent method.
/// *   **double lambda** Weight parameter to balance data term and smoothness term.
/// *   **double alpha** Parameter of spacial distribution in Bilateral-TV.
/// *   **int btvKernelSize** Kernel size of Bilateral-TV filter.
/// *   **int blurKernelSize** Gaussian blur kernel size.
/// *   **double blurSigma** Gaussian blur sigma.
/// *   **int temporalAreaRadius** Radius of the temporal search area.
/// *   **Ptr\<DenseOpticalFlowExt\> opticalFlow** Dense optical flow algorithm.
// createSuperResolution_BTVL1()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:199
// ("cv::superres::createSuperResolution_BTVL1", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn create_super_resolution_btvl1() -> Result<core::Ptr<crate::superres::SuperRes_SuperResolution>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_superres_createSuperResolution_BTVL1(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::superres::SuperRes_SuperResolution>::opencv_from_extern(ret) };
	Ok(ret)
}

// createSuperResolution_BTVL1_CUDA()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:200
// ("cv::superres::createSuperResolution_BTVL1_CUDA", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn create_super_resolution_btvl1_cuda() -> Result<core::Ptr<crate::superres::SuperRes_SuperResolution>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_superres_createSuperResolution_BTVL1_CUDA(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::superres::SuperRes_SuperResolution>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Constant methods for [crate::superres::SuperRes_BroxOpticalFlow]
// BroxOpticalFlow /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:143
pub trait SuperRes_BroxOpticalFlowTraitConst: crate::superres::SuperRes_DenseOpticalFlowExtTraitConst {
	fn as_raw_SuperRes_BroxOpticalFlow(&self) -> *const c_void;

	/// Flow smoothness
	/// ## See also
	/// setAlpha
	// getAlpha()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:148
	// ("cv::superres::BroxOpticalFlow::getAlpha", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_alpha(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_BroxOpticalFlow_getAlpha_const(self.as_raw_SuperRes_BroxOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Gradient constancy importance
	/// ## See also
	/// setGamma
	// getGamma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:153
	// ("cv::superres::BroxOpticalFlow::getGamma", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_gamma(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_BroxOpticalFlow_getGamma_const(self.as_raw_SuperRes_BroxOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Pyramid scale factor
	/// ## See also
	/// setScaleFactor
	// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:158
	// ("cv::superres::BroxOpticalFlow::getScaleFactor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_scale_factor(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_BroxOpticalFlow_getScaleFactor_const(self.as_raw_SuperRes_BroxOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Number of lagged non-linearity iterations (inner loop)
	/// ## See also
	/// setInnerIterations
	// getInnerIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:163
	// ("cv::superres::BroxOpticalFlow::getInnerIterations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_inner_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_BroxOpticalFlow_getInnerIterations_const(self.as_raw_SuperRes_BroxOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Number of warping iterations (number of pyramid levels)
	/// ## See also
	/// setOuterIterations
	// getOuterIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:168
	// ("cv::superres::BroxOpticalFlow::getOuterIterations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_outer_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_BroxOpticalFlow_getOuterIterations_const(self.as_raw_SuperRes_BroxOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Number of linear system solver iterations
	/// ## See also
	/// setSolverIterations
	// getSolverIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:173
	// ("cv::superres::BroxOpticalFlow::getSolverIterations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_solver_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_BroxOpticalFlow_getSolverIterations_const(self.as_raw_SuperRes_BroxOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::superres::SuperRes_BroxOpticalFlow]
pub trait SuperRes_BroxOpticalFlowTrait: crate::superres::SuperRes_BroxOpticalFlowTraitConst + crate::superres::SuperRes_DenseOpticalFlowExtTrait {
	fn as_raw_mut_SuperRes_BroxOpticalFlow(&mut self) -> *mut c_void;

	/// Flow smoothness
	/// ## See also
	/// setAlpha getAlpha
	// setAlpha(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:150
	// ("cv::superres::BroxOpticalFlow::setAlpha", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_alpha(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_BroxOpticalFlow_setAlpha_double(self.as_raw_mut_SuperRes_BroxOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Gradient constancy importance
	/// ## See also
	/// setGamma getGamma
	// setGamma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:155
	// ("cv::superres::BroxOpticalFlow::setGamma", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_gamma(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_BroxOpticalFlow_setGamma_double(self.as_raw_mut_SuperRes_BroxOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Pyramid scale factor
	/// ## See also
	/// setScaleFactor getScaleFactor
	// setScaleFactor(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:160
	// ("cv::superres::BroxOpticalFlow::setScaleFactor", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_scale_factor(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_BroxOpticalFlow_setScaleFactor_double(self.as_raw_mut_SuperRes_BroxOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Number of lagged non-linearity iterations (inner loop)
	/// ## See also
	/// setInnerIterations getInnerIterations
	// setInnerIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:165
	// ("cv::superres::BroxOpticalFlow::setInnerIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_inner_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_BroxOpticalFlow_setInnerIterations_int(self.as_raw_mut_SuperRes_BroxOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Number of warping iterations (number of pyramid levels)
	/// ## See also
	/// setOuterIterations getOuterIterations
	// setOuterIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:170
	// ("cv::superres::BroxOpticalFlow::setOuterIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_outer_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_BroxOpticalFlow_setOuterIterations_int(self.as_raw_mut_SuperRes_BroxOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Number of linear system solver iterations
	/// ## See also
	/// setSolverIterations getSolverIterations
	// setSolverIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:175
	// ("cv::superres::BroxOpticalFlow::setSolverIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_solver_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_BroxOpticalFlow_setSolverIterations_int(self.as_raw_mut_SuperRes_BroxOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// BroxOpticalFlow /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:143
pub struct SuperRes_BroxOpticalFlow {
	ptr: *mut c_void,
}

opencv_type_boxed! { SuperRes_BroxOpticalFlow }

impl Drop for SuperRes_BroxOpticalFlow {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_superres_BroxOpticalFlow_delete(self.as_raw_mut_SuperRes_BroxOpticalFlow()) };
	}
}

unsafe impl Send for SuperRes_BroxOpticalFlow {}

impl core::AlgorithmTraitConst for SuperRes_BroxOpticalFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SuperRes_BroxOpticalFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SuperRes_BroxOpticalFlow, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::superres::SuperRes_DenseOpticalFlowExtTraitConst for SuperRes_BroxOpticalFlow {
	#[inline] fn as_raw_SuperRes_DenseOpticalFlowExt(&self) -> *const c_void { self.as_raw() }
}

impl crate::superres::SuperRes_DenseOpticalFlowExtTrait for SuperRes_BroxOpticalFlow {
	#[inline] fn as_raw_mut_SuperRes_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SuperRes_BroxOpticalFlow, crate::superres::SuperRes_DenseOpticalFlowExtTraitConst, as_raw_SuperRes_DenseOpticalFlowExt, crate::superres::SuperRes_DenseOpticalFlowExtTrait, as_raw_mut_SuperRes_DenseOpticalFlowExt }

impl crate::superres::SuperRes_BroxOpticalFlowTraitConst for SuperRes_BroxOpticalFlow {
	#[inline] fn as_raw_SuperRes_BroxOpticalFlow(&self) -> *const c_void { self.as_raw() }
}

impl crate::superres::SuperRes_BroxOpticalFlowTrait for SuperRes_BroxOpticalFlow {
	#[inline] fn as_raw_mut_SuperRes_BroxOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SuperRes_BroxOpticalFlow, crate::superres::SuperRes_BroxOpticalFlowTraitConst, as_raw_SuperRes_BroxOpticalFlow, crate::superres::SuperRes_BroxOpticalFlowTrait, as_raw_mut_SuperRes_BroxOpticalFlow }

impl SuperRes_BroxOpticalFlow {
}

boxed_cast_base! { SuperRes_BroxOpticalFlow, core::Algorithm, cv_superres_BroxOpticalFlow_to_Algorithm }

boxed_cast_base! { SuperRes_BroxOpticalFlow, crate::superres::SuperRes_DenseOpticalFlowExt, cv_superres_BroxOpticalFlow_to_SuperRes_DenseOpticalFlowExt }

impl std::fmt::Debug for SuperRes_BroxOpticalFlow {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SuperRes_BroxOpticalFlow")
			.finish()
	}
}

/// Constant methods for [crate::superres::SuperRes_DenseOpticalFlowExt]
// DenseOpticalFlowExt /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:56
pub trait SuperRes_DenseOpticalFlowExtTraitConst: core::AlgorithmTraitConst {
	fn as_raw_SuperRes_DenseOpticalFlowExt(&self) -> *const c_void;

}

/// Mutable methods for [crate::superres::SuperRes_DenseOpticalFlowExt]
pub trait SuperRes_DenseOpticalFlowExtTrait: core::AlgorithmTrait + crate::superres::SuperRes_DenseOpticalFlowExtTraitConst {
	fn as_raw_mut_SuperRes_DenseOpticalFlowExt(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * flow2: noArray()
	// calc(InputArray, InputArray, OutputArray, OutputArray)(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:59
	// ("cv::superres::DenseOpticalFlowExt::calc", vec![(pred!(mut, ["frame0", "frame1", "flow1", "flow2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn calc(&mut self, frame0: &impl ToInputArray, frame1: &impl ToInputArray, flow1: &mut impl ToOutputArray, flow2: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(frame0);
		input_array_arg!(frame1);
		output_array_arg!(flow1);
		output_array_arg!(flow2);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DenseOpticalFlowExt_calc_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_SuperRes_DenseOpticalFlowExt(), frame0.as_raw__InputArray(), frame1.as_raw__InputArray(), flow1.as_raw__OutputArray(), flow2.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [SuperRes_DenseOpticalFlowExtTrait::calc] function uses the following default values for its arguments:
	/// * flow2: noArray()
	// cv::superres::DenseOpticalFlowExt::calc(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:59
	// ("cv::superres::DenseOpticalFlowExt::calc", vec![(pred!(mut, ["frame0", "frame1", "flow1"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn calc_def(&mut self, frame0: &impl ToInputArray, frame1: &impl ToInputArray, flow1: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(frame0);
		input_array_arg!(frame1);
		output_array_arg!(flow1);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DenseOpticalFlowExt_calc_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_SuperRes_DenseOpticalFlowExt(), frame0.as_raw__InputArray(), frame1.as_raw__InputArray(), flow1.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// collectGarbage()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:60
	// ("cv::superres::DenseOpticalFlowExt::collectGarbage", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn collect_garbage(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DenseOpticalFlowExt_collectGarbage(self.as_raw_mut_SuperRes_DenseOpticalFlowExt(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// DenseOpticalFlowExt /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:56
pub struct SuperRes_DenseOpticalFlowExt {
	ptr: *mut c_void,
}

opencv_type_boxed! { SuperRes_DenseOpticalFlowExt }

impl Drop for SuperRes_DenseOpticalFlowExt {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_superres_DenseOpticalFlowExt_delete(self.as_raw_mut_SuperRes_DenseOpticalFlowExt()) };
	}
}

unsafe impl Send for SuperRes_DenseOpticalFlowExt {}

impl core::AlgorithmTraitConst for SuperRes_DenseOpticalFlowExt {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SuperRes_DenseOpticalFlowExt {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SuperRes_DenseOpticalFlowExt, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::superres::SuperRes_DenseOpticalFlowExtTraitConst for SuperRes_DenseOpticalFlowExt {
	#[inline] fn as_raw_SuperRes_DenseOpticalFlowExt(&self) -> *const c_void { self.as_raw() }
}

impl crate::superres::SuperRes_DenseOpticalFlowExtTrait for SuperRes_DenseOpticalFlowExt {
	#[inline] fn as_raw_mut_SuperRes_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SuperRes_DenseOpticalFlowExt, crate::superres::SuperRes_DenseOpticalFlowExtTraitConst, as_raw_SuperRes_DenseOpticalFlowExt, crate::superres::SuperRes_DenseOpticalFlowExtTrait, as_raw_mut_SuperRes_DenseOpticalFlowExt }

impl SuperRes_DenseOpticalFlowExt {
}

boxed_cast_descendant! { SuperRes_DenseOpticalFlowExt, crate::superres::SuperRes_BroxOpticalFlow, cv_superres_DenseOpticalFlowExt_to_SuperRes_BroxOpticalFlow }

boxed_cast_descendant! { SuperRes_DenseOpticalFlowExt, crate::superres::SuperRes_DualTVL1OpticalFlow, cv_superres_DenseOpticalFlowExt_to_SuperRes_DualTVL1OpticalFlow }

boxed_cast_descendant! { SuperRes_DenseOpticalFlowExt, crate::superres::SuperRes_FarnebackOpticalFlow, cv_superres_DenseOpticalFlowExt_to_SuperRes_FarnebackOpticalFlow }

boxed_cast_descendant! { SuperRes_DenseOpticalFlowExt, crate::superres::SuperRes_PyrLKOpticalFlow, cv_superres_DenseOpticalFlowExt_to_SuperRes_PyrLKOpticalFlow }

boxed_cast_base! { SuperRes_DenseOpticalFlowExt, core::Algorithm, cv_superres_DenseOpticalFlowExt_to_Algorithm }

impl std::fmt::Debug for SuperRes_DenseOpticalFlowExt {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SuperRes_DenseOpticalFlowExt")
			.finish()
	}
}

/// Constant methods for [crate::superres::SuperRes_DualTVL1OpticalFlow]
// DualTVL1OpticalFlow /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:103
pub trait SuperRes_DualTVL1OpticalFlowTraitConst: crate::superres::SuperRes_DenseOpticalFlowExtTraitConst {
	fn as_raw_SuperRes_DualTVL1OpticalFlow(&self) -> *const c_void;

	/// ## See also
	/// setTau
	// getTau()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:107
	// ("cv::superres::DualTVL1OpticalFlow::getTau", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_tau(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_getTau_const(self.as_raw_SuperRes_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setLambda
	// getLambda()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:111
	// ("cv::superres::DualTVL1OpticalFlow::getLambda", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_lambda(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_getLambda_const(self.as_raw_SuperRes_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setTheta
	// getTheta()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:115
	// ("cv::superres::DualTVL1OpticalFlow::getTheta", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_theta(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_getTheta_const(self.as_raw_SuperRes_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setScalesNumber
	// getScalesNumber()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:119
	// ("cv::superres::DualTVL1OpticalFlow::getScalesNumber", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_scales_number(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_getScalesNumber_const(self.as_raw_SuperRes_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setWarpingsNumber
	// getWarpingsNumber()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:123
	// ("cv::superres::DualTVL1OpticalFlow::getWarpingsNumber", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_warpings_number(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_getWarpingsNumber_const(self.as_raw_SuperRes_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setEpsilon
	// getEpsilon()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:127
	// ("cv::superres::DualTVL1OpticalFlow::getEpsilon", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_epsilon(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_getEpsilon_const(self.as_raw_SuperRes_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setIterations
	// getIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:131
	// ("cv::superres::DualTVL1OpticalFlow::getIterations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_getIterations_const(self.as_raw_SuperRes_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setUseInitialFlow
	// getUseInitialFlow()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:135
	// ("cv::superres::DualTVL1OpticalFlow::getUseInitialFlow", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_use_initial_flow(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_getUseInitialFlow_const(self.as_raw_SuperRes_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::superres::SuperRes_DualTVL1OpticalFlow]
pub trait SuperRes_DualTVL1OpticalFlowTrait: crate::superres::SuperRes_DenseOpticalFlowExtTrait + crate::superres::SuperRes_DualTVL1OpticalFlowTraitConst {
	fn as_raw_mut_SuperRes_DualTVL1OpticalFlow(&mut self) -> *mut c_void;

	/// ## See also
	/// setTau getTau
	// setTau(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:109
	// ("cv::superres::DualTVL1OpticalFlow::setTau", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_tau(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_setTau_double(self.as_raw_mut_SuperRes_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setLambda getLambda
	// setLambda(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:113
	// ("cv::superres::DualTVL1OpticalFlow::setLambda", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_lambda(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_setLambda_double(self.as_raw_mut_SuperRes_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setTheta getTheta
	// setTheta(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:117
	// ("cv::superres::DualTVL1OpticalFlow::setTheta", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_theta(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_setTheta_double(self.as_raw_mut_SuperRes_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setScalesNumber getScalesNumber
	// setScalesNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:121
	// ("cv::superres::DualTVL1OpticalFlow::setScalesNumber", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_scales_number(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_setScalesNumber_int(self.as_raw_mut_SuperRes_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setWarpingsNumber getWarpingsNumber
	// setWarpingsNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:125
	// ("cv::superres::DualTVL1OpticalFlow::setWarpingsNumber", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_warpings_number(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_setWarpingsNumber_int(self.as_raw_mut_SuperRes_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setEpsilon getEpsilon
	// setEpsilon(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:129
	// ("cv::superres::DualTVL1OpticalFlow::setEpsilon", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_epsilon(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_setEpsilon_double(self.as_raw_mut_SuperRes_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setIterations getIterations
	// setIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:133
	// ("cv::superres::DualTVL1OpticalFlow::setIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_setIterations_int(self.as_raw_mut_SuperRes_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setUseInitialFlow getUseInitialFlow
	// setUseInitialFlow(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:137
	// ("cv::superres::DualTVL1OpticalFlow::setUseInitialFlow", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	#[inline]
	fn set_use_initial_flow(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_setUseInitialFlow_bool(self.as_raw_mut_SuperRes_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// DualTVL1OpticalFlow /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:103
pub struct SuperRes_DualTVL1OpticalFlow {
	ptr: *mut c_void,
}

opencv_type_boxed! { SuperRes_DualTVL1OpticalFlow }

impl Drop for SuperRes_DualTVL1OpticalFlow {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_superres_DualTVL1OpticalFlow_delete(self.as_raw_mut_SuperRes_DualTVL1OpticalFlow()) };
	}
}

unsafe impl Send for SuperRes_DualTVL1OpticalFlow {}

impl core::AlgorithmTraitConst for SuperRes_DualTVL1OpticalFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SuperRes_DualTVL1OpticalFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SuperRes_DualTVL1OpticalFlow, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::superres::SuperRes_DenseOpticalFlowExtTraitConst for SuperRes_DualTVL1OpticalFlow {
	#[inline] fn as_raw_SuperRes_DenseOpticalFlowExt(&self) -> *const c_void { self.as_raw() }
}

impl crate::superres::SuperRes_DenseOpticalFlowExtTrait for SuperRes_DualTVL1OpticalFlow {
	#[inline] fn as_raw_mut_SuperRes_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SuperRes_DualTVL1OpticalFlow, crate::superres::SuperRes_DenseOpticalFlowExtTraitConst, as_raw_SuperRes_DenseOpticalFlowExt, crate::superres::SuperRes_DenseOpticalFlowExtTrait, as_raw_mut_SuperRes_DenseOpticalFlowExt }

impl crate::superres::SuperRes_DualTVL1OpticalFlowTraitConst for SuperRes_DualTVL1OpticalFlow {
	#[inline] fn as_raw_SuperRes_DualTVL1OpticalFlow(&self) -> *const c_void { self.as_raw() }
}

impl crate::superres::SuperRes_DualTVL1OpticalFlowTrait for SuperRes_DualTVL1OpticalFlow {
	#[inline] fn as_raw_mut_SuperRes_DualTVL1OpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SuperRes_DualTVL1OpticalFlow, crate::superres::SuperRes_DualTVL1OpticalFlowTraitConst, as_raw_SuperRes_DualTVL1OpticalFlow, crate::superres::SuperRes_DualTVL1OpticalFlowTrait, as_raw_mut_SuperRes_DualTVL1OpticalFlow }

impl SuperRes_DualTVL1OpticalFlow {
}

boxed_cast_base! { SuperRes_DualTVL1OpticalFlow, core::Algorithm, cv_superres_DualTVL1OpticalFlow_to_Algorithm }

boxed_cast_base! { SuperRes_DualTVL1OpticalFlow, crate::superres::SuperRes_DenseOpticalFlowExt, cv_superres_DualTVL1OpticalFlow_to_SuperRes_DenseOpticalFlowExt }

impl std::fmt::Debug for SuperRes_DualTVL1OpticalFlow {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SuperRes_DualTVL1OpticalFlow")
			.finish()
	}
}

/// Constant methods for [crate::superres::SuperRes_FarnebackOpticalFlow]
// FarnebackOpticalFlow /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:64
pub trait SuperRes_FarnebackOpticalFlowTraitConst: crate::superres::SuperRes_DenseOpticalFlowExtTraitConst {
	fn as_raw_SuperRes_FarnebackOpticalFlow(&self) -> *const c_void;

	/// ## See also
	/// setPyrScale
	// getPyrScale()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:68
	// ("cv::superres::FarnebackOpticalFlow::getPyrScale", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_pyr_scale(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_getPyrScale_const(self.as_raw_SuperRes_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setLevelsNumber
	// getLevelsNumber()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:72
	// ("cv::superres::FarnebackOpticalFlow::getLevelsNumber", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_levels_number(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_getLevelsNumber_const(self.as_raw_SuperRes_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setWindowSize
	// getWindowSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:76
	// ("cv::superres::FarnebackOpticalFlow::getWindowSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_window_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_getWindowSize_const(self.as_raw_SuperRes_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setIterations
	// getIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:80
	// ("cv::superres::FarnebackOpticalFlow::getIterations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_getIterations_const(self.as_raw_SuperRes_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setPolyN
	// getPolyN()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:84
	// ("cv::superres::FarnebackOpticalFlow::getPolyN", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_poly_n(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_getPolyN_const(self.as_raw_SuperRes_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setPolySigma
	// getPolySigma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:88
	// ("cv::superres::FarnebackOpticalFlow::getPolySigma", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_poly_sigma(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_getPolySigma_const(self.as_raw_SuperRes_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setFlags
	// getFlags()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:92
	// ("cv::superres::FarnebackOpticalFlow::getFlags", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_flags(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_getFlags_const(self.as_raw_SuperRes_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::superres::SuperRes_FarnebackOpticalFlow]
pub trait SuperRes_FarnebackOpticalFlowTrait: crate::superres::SuperRes_DenseOpticalFlowExtTrait + crate::superres::SuperRes_FarnebackOpticalFlowTraitConst {
	fn as_raw_mut_SuperRes_FarnebackOpticalFlow(&mut self) -> *mut c_void;

	/// ## See also
	/// setPyrScale getPyrScale
	// setPyrScale(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:70
	// ("cv::superres::FarnebackOpticalFlow::setPyrScale", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_pyr_scale(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_setPyrScale_double(self.as_raw_mut_SuperRes_FarnebackOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setLevelsNumber getLevelsNumber
	// setLevelsNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:74
	// ("cv::superres::FarnebackOpticalFlow::setLevelsNumber", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_levels_number(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_setLevelsNumber_int(self.as_raw_mut_SuperRes_FarnebackOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setWindowSize getWindowSize
	// setWindowSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:78
	// ("cv::superres::FarnebackOpticalFlow::setWindowSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_window_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_setWindowSize_int(self.as_raw_mut_SuperRes_FarnebackOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setIterations getIterations
	// setIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:82
	// ("cv::superres::FarnebackOpticalFlow::setIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_setIterations_int(self.as_raw_mut_SuperRes_FarnebackOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setPolyN getPolyN
	// setPolyN(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:86
	// ("cv::superres::FarnebackOpticalFlow::setPolyN", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_poly_n(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_setPolyN_int(self.as_raw_mut_SuperRes_FarnebackOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setPolySigma getPolySigma
	// setPolySigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:90
	// ("cv::superres::FarnebackOpticalFlow::setPolySigma", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_poly_sigma(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_setPolySigma_double(self.as_raw_mut_SuperRes_FarnebackOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setFlags getFlags
	// setFlags(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:94
	// ("cv::superres::FarnebackOpticalFlow::setFlags", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_flags(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FarnebackOpticalFlow_setFlags_int(self.as_raw_mut_SuperRes_FarnebackOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// FarnebackOpticalFlow /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:64
pub struct SuperRes_FarnebackOpticalFlow {
	ptr: *mut c_void,
}

opencv_type_boxed! { SuperRes_FarnebackOpticalFlow }

impl Drop for SuperRes_FarnebackOpticalFlow {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_superres_FarnebackOpticalFlow_delete(self.as_raw_mut_SuperRes_FarnebackOpticalFlow()) };
	}
}

unsafe impl Send for SuperRes_FarnebackOpticalFlow {}

impl core::AlgorithmTraitConst for SuperRes_FarnebackOpticalFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SuperRes_FarnebackOpticalFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SuperRes_FarnebackOpticalFlow, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::superres::SuperRes_DenseOpticalFlowExtTraitConst for SuperRes_FarnebackOpticalFlow {
	#[inline] fn as_raw_SuperRes_DenseOpticalFlowExt(&self) -> *const c_void { self.as_raw() }
}

impl crate::superres::SuperRes_DenseOpticalFlowExtTrait for SuperRes_FarnebackOpticalFlow {
	#[inline] fn as_raw_mut_SuperRes_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SuperRes_FarnebackOpticalFlow, crate::superres::SuperRes_DenseOpticalFlowExtTraitConst, as_raw_SuperRes_DenseOpticalFlowExt, crate::superres::SuperRes_DenseOpticalFlowExtTrait, as_raw_mut_SuperRes_DenseOpticalFlowExt }

impl crate::superres::SuperRes_FarnebackOpticalFlowTraitConst for SuperRes_FarnebackOpticalFlow {
	#[inline] fn as_raw_SuperRes_FarnebackOpticalFlow(&self) -> *const c_void { self.as_raw() }
}

impl crate::superres::SuperRes_FarnebackOpticalFlowTrait for SuperRes_FarnebackOpticalFlow {
	#[inline] fn as_raw_mut_SuperRes_FarnebackOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SuperRes_FarnebackOpticalFlow, crate::superres::SuperRes_FarnebackOpticalFlowTraitConst, as_raw_SuperRes_FarnebackOpticalFlow, crate::superres::SuperRes_FarnebackOpticalFlowTrait, as_raw_mut_SuperRes_FarnebackOpticalFlow }

impl SuperRes_FarnebackOpticalFlow {
}

boxed_cast_base! { SuperRes_FarnebackOpticalFlow, core::Algorithm, cv_superres_FarnebackOpticalFlow_to_Algorithm }

boxed_cast_base! { SuperRes_FarnebackOpticalFlow, crate::superres::SuperRes_DenseOpticalFlowExt, cv_superres_FarnebackOpticalFlow_to_SuperRes_DenseOpticalFlowExt }

impl std::fmt::Debug for SuperRes_FarnebackOpticalFlow {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SuperRes_FarnebackOpticalFlow")
			.finish()
	}
}

/// Constant methods for [crate::superres::SuperRes_FrameSource]
// FrameSource /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:66
pub trait SuperRes_FrameSourceTraitConst {
	fn as_raw_SuperRes_FrameSource(&self) -> *const c_void;

}

/// Mutable methods for [crate::superres::SuperRes_FrameSource]
pub trait SuperRes_FrameSourceTrait: crate::superres::SuperRes_FrameSourceTraitConst {
	fn as_raw_mut_SuperRes_FrameSource(&mut self) -> *mut c_void;

	// nextFrame(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:71
	// ("cv::superres::FrameSource::nextFrame", vec![(pred!(mut, ["frame"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn next_frame(&mut self, frame: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(frame);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FrameSource_nextFrame_const__OutputArrayR(self.as_raw_mut_SuperRes_FrameSource(), frame.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:72
	// ("cv::superres::FrameSource::reset", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn reset(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_FrameSource_reset(self.as_raw_mut_SuperRes_FrameSource(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// FrameSource /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:66
pub struct SuperRes_FrameSource {
	ptr: *mut c_void,
}

opencv_type_boxed! { SuperRes_FrameSource }

impl Drop for SuperRes_FrameSource {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_superres_FrameSource_delete(self.as_raw_mut_SuperRes_FrameSource()) };
	}
}

unsafe impl Send for SuperRes_FrameSource {}

impl crate::superres::SuperRes_FrameSourceTraitConst for SuperRes_FrameSource {
	#[inline] fn as_raw_SuperRes_FrameSource(&self) -> *const c_void { self.as_raw() }
}

impl crate::superres::SuperRes_FrameSourceTrait for SuperRes_FrameSource {
	#[inline] fn as_raw_mut_SuperRes_FrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SuperRes_FrameSource, crate::superres::SuperRes_FrameSourceTraitConst, as_raw_SuperRes_FrameSource, crate::superres::SuperRes_FrameSourceTrait, as_raw_mut_SuperRes_FrameSource }

impl SuperRes_FrameSource {
}

boxed_cast_descendant! { SuperRes_FrameSource, crate::superres::SuperRes_SuperResolution, cv_superres_FrameSource_to_SuperRes_SuperResolution }

impl std::fmt::Debug for SuperRes_FrameSource {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SuperRes_FrameSource")
			.finish()
	}
}

/// Constant methods for [crate::superres::SuperRes_PyrLKOpticalFlow]
// PyrLKOpticalFlow /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:180
pub trait SuperRes_PyrLKOpticalFlowTraitConst: crate::superres::SuperRes_DenseOpticalFlowExtTraitConst {
	fn as_raw_SuperRes_PyrLKOpticalFlow(&self) -> *const c_void;

	/// ## See also
	/// setWindowSize
	// getWindowSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:184
	// ("cv::superres::PyrLKOpticalFlow::getWindowSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_window_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_PyrLKOpticalFlow_getWindowSize_const(self.as_raw_SuperRes_PyrLKOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setMaxLevel
	// getMaxLevel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:188
	// ("cv::superres::PyrLKOpticalFlow::getMaxLevel", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_level(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_PyrLKOpticalFlow_getMaxLevel_const(self.as_raw_SuperRes_PyrLKOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setIterations
	// getIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:192
	// ("cv::superres::PyrLKOpticalFlow::getIterations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_PyrLKOpticalFlow_getIterations_const(self.as_raw_SuperRes_PyrLKOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::superres::SuperRes_PyrLKOpticalFlow]
pub trait SuperRes_PyrLKOpticalFlowTrait: crate::superres::SuperRes_DenseOpticalFlowExtTrait + crate::superres::SuperRes_PyrLKOpticalFlowTraitConst {
	fn as_raw_mut_SuperRes_PyrLKOpticalFlow(&mut self) -> *mut c_void;

	/// ## See also
	/// setWindowSize getWindowSize
	// setWindowSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:186
	// ("cv::superres::PyrLKOpticalFlow::setWindowSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_window_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_PyrLKOpticalFlow_setWindowSize_int(self.as_raw_mut_SuperRes_PyrLKOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setMaxLevel getMaxLevel
	// setMaxLevel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:190
	// ("cv::superres::PyrLKOpticalFlow::setMaxLevel", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_max_level(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_PyrLKOpticalFlow_setMaxLevel_int(self.as_raw_mut_SuperRes_PyrLKOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setIterations getIterations
	// setIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:194
	// ("cv::superres::PyrLKOpticalFlow::setIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_PyrLKOpticalFlow_setIterations_int(self.as_raw_mut_SuperRes_PyrLKOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// PyrLKOpticalFlow /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres/optical_flow.hpp:180
pub struct SuperRes_PyrLKOpticalFlow {
	ptr: *mut c_void,
}

opencv_type_boxed! { SuperRes_PyrLKOpticalFlow }

impl Drop for SuperRes_PyrLKOpticalFlow {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_superres_PyrLKOpticalFlow_delete(self.as_raw_mut_SuperRes_PyrLKOpticalFlow()) };
	}
}

unsafe impl Send for SuperRes_PyrLKOpticalFlow {}

impl core::AlgorithmTraitConst for SuperRes_PyrLKOpticalFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SuperRes_PyrLKOpticalFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SuperRes_PyrLKOpticalFlow, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::superres::SuperRes_DenseOpticalFlowExtTraitConst for SuperRes_PyrLKOpticalFlow {
	#[inline] fn as_raw_SuperRes_DenseOpticalFlowExt(&self) -> *const c_void { self.as_raw() }
}

impl crate::superres::SuperRes_DenseOpticalFlowExtTrait for SuperRes_PyrLKOpticalFlow {
	#[inline] fn as_raw_mut_SuperRes_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SuperRes_PyrLKOpticalFlow, crate::superres::SuperRes_DenseOpticalFlowExtTraitConst, as_raw_SuperRes_DenseOpticalFlowExt, crate::superres::SuperRes_DenseOpticalFlowExtTrait, as_raw_mut_SuperRes_DenseOpticalFlowExt }

impl crate::superres::SuperRes_PyrLKOpticalFlowTraitConst for SuperRes_PyrLKOpticalFlow {
	#[inline] fn as_raw_SuperRes_PyrLKOpticalFlow(&self) -> *const c_void { self.as_raw() }
}

impl crate::superres::SuperRes_PyrLKOpticalFlowTrait for SuperRes_PyrLKOpticalFlow {
	#[inline] fn as_raw_mut_SuperRes_PyrLKOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SuperRes_PyrLKOpticalFlow, crate::superres::SuperRes_PyrLKOpticalFlowTraitConst, as_raw_SuperRes_PyrLKOpticalFlow, crate::superres::SuperRes_PyrLKOpticalFlowTrait, as_raw_mut_SuperRes_PyrLKOpticalFlow }

impl SuperRes_PyrLKOpticalFlow {
}

boxed_cast_base! { SuperRes_PyrLKOpticalFlow, core::Algorithm, cv_superres_PyrLKOpticalFlow_to_Algorithm }

boxed_cast_base! { SuperRes_PyrLKOpticalFlow, crate::superres::SuperRes_DenseOpticalFlowExt, cv_superres_PyrLKOpticalFlow_to_SuperRes_DenseOpticalFlowExt }

impl std::fmt::Debug for SuperRes_PyrLKOpticalFlow {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SuperRes_PyrLKOpticalFlow")
			.finish()
	}
}

/// Constant methods for [crate::superres::SuperRes_SuperResolution]
// SuperResolution /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:87
pub trait SuperRes_SuperResolutionTraitConst: core::AlgorithmTraitConst + crate::superres::SuperRes_FrameSourceTraitConst {
	fn as_raw_SuperRes_SuperResolution(&self) -> *const c_void;

	/// Scale factor
	/// ## See also
	/// setScale
	// getScale()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:109
	// ("cv::superres::SuperResolution::getScale", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_scale(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_getScale_const(self.as_raw_SuperRes_SuperResolution(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Iterations count
	/// ## See also
	/// setIterations
	// getIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:115
	// ("cv::superres::SuperResolution::getIterations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_getIterations_const(self.as_raw_SuperRes_SuperResolution(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Asymptotic value of steepest descent method
	/// ## See also
	/// setTau
	// getTau()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:121
	// ("cv::superres::SuperResolution::getTau", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_tau(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_getTau_const(self.as_raw_SuperRes_SuperResolution(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weight parameter to balance data term and smoothness term
	/// ## See also
	/// setLambda
	// getLambda()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:127
	// ("cv::superres::SuperResolution::getLambda", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_lambda(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_getLambda_const(self.as_raw_SuperRes_SuperResolution(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Parameter of spacial distribution in Bilateral-TV
	/// ## See also
	/// setAlpha
	// getAlpha()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:133
	// ("cv::superres::SuperResolution::getAlpha", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_alpha(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_getAlpha_const(self.as_raw_SuperRes_SuperResolution(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Kernel size of Bilateral-TV filter
	/// ## See also
	/// setKernelSize
	// getKernelSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:139
	// ("cv::superres::SuperResolution::getKernelSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_kernel_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_getKernelSize_const(self.as_raw_SuperRes_SuperResolution(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Gaussian blur kernel size
	/// ## See also
	/// setBlurKernelSize
	// getBlurKernelSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:145
	// ("cv::superres::SuperResolution::getBlurKernelSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_blur_kernel_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_getBlurKernelSize_const(self.as_raw_SuperRes_SuperResolution(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Gaussian blur sigma
	/// ## See also
	/// setBlurSigma
	// getBlurSigma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:151
	// ("cv::superres::SuperResolution::getBlurSigma", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_blur_sigma(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_getBlurSigma_const(self.as_raw_SuperRes_SuperResolution(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Radius of the temporal search area
	/// ## See also
	/// setTemporalAreaRadius
	// getTemporalAreaRadius()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:157
	// ("cv::superres::SuperResolution::getTemporalAreaRadius", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_temporal_area_radius(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_getTemporalAreaRadius_const(self.as_raw_SuperRes_SuperResolution(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Dense optical flow algorithm
	/// ## See also
	/// setOpticalFlow
	// getOpticalFlow()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:163
	// ("cv::superres::SuperResolution::getOpticalFlow", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_optical_flow(&self) -> Result<core::Ptr<crate::superres::SuperRes_DenseOpticalFlowExt>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_getOpticalFlow_const(self.as_raw_SuperRes_SuperResolution(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::superres::SuperRes_DenseOpticalFlowExt>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::superres::SuperRes_SuperResolution]
pub trait SuperRes_SuperResolutionTrait: core::AlgorithmTrait + crate::superres::SuperRes_FrameSourceTrait + crate::superres::SuperRes_SuperResolutionTraitConst {
	fn as_raw_mut_SuperRes_SuperResolution(&mut self) -> *mut c_void;

	/// Set input frame source for Super Resolution algorithm.
	///
	/// ## Parameters
	/// * frameSource: Input frame source
	// setInput(const Ptr<FrameSource> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:94
	// ("cv::superres::SuperResolution::setInput", vec![(pred!(mut, ["frameSource"], ["const cv::Ptr<cv::superres::FrameSource>*"]), _)]),
	#[inline]
	fn set_input(&mut self, frame_source: &core::Ptr<crate::superres::SuperRes_FrameSource>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_setInput_const_PtrLFrameSourceGR(self.as_raw_mut_SuperRes_SuperResolution(), frame_source.as_raw_PtrOfSuperRes_FrameSource(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Process next frame from input and return output result.
	///
	/// ## Parameters
	/// * frame: Output result
	// nextFrame(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:100
	// ("cv::superres::SuperResolution::nextFrame", vec![(pred!(mut, ["frame"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn next_frame(&mut self, frame: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(frame);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_nextFrame_const__OutputArrayR(self.as_raw_mut_SuperRes_SuperResolution(), frame.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:101
	// ("cv::superres::SuperResolution::reset", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn reset(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_reset(self.as_raw_mut_SuperRes_SuperResolution(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Clear all inner buffers.
	// collectGarbage()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:105
	// ("cv::superres::SuperResolution::collectGarbage", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn collect_garbage(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_collectGarbage(self.as_raw_mut_SuperRes_SuperResolution(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Scale factor
	/// ## See also
	/// setScale getScale
	// setScale(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:111
	// ("cv::superres::SuperResolution::setScale", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_scale(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_setScale_int(self.as_raw_mut_SuperRes_SuperResolution(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Iterations count
	/// ## See also
	/// setIterations getIterations
	// setIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:117
	// ("cv::superres::SuperResolution::setIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_setIterations_int(self.as_raw_mut_SuperRes_SuperResolution(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Asymptotic value of steepest descent method
	/// ## See also
	/// setTau getTau
	// setTau(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:123
	// ("cv::superres::SuperResolution::setTau", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_tau(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_setTau_double(self.as_raw_mut_SuperRes_SuperResolution(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weight parameter to balance data term and smoothness term
	/// ## See also
	/// setLambda getLambda
	// setLambda(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:129
	// ("cv::superres::SuperResolution::setLambda", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_lambda(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_setLambda_double(self.as_raw_mut_SuperRes_SuperResolution(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Parameter of spacial distribution in Bilateral-TV
	/// ## See also
	/// setAlpha getAlpha
	// setAlpha(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:135
	// ("cv::superres::SuperResolution::setAlpha", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_alpha(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_setAlpha_double(self.as_raw_mut_SuperRes_SuperResolution(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Kernel size of Bilateral-TV filter
	/// ## See also
	/// setKernelSize getKernelSize
	// setKernelSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:141
	// ("cv::superres::SuperResolution::setKernelSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_kernel_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_setKernelSize_int(self.as_raw_mut_SuperRes_SuperResolution(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Gaussian blur kernel size
	/// ## See also
	/// setBlurKernelSize getBlurKernelSize
	// setBlurKernelSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:147
	// ("cv::superres::SuperResolution::setBlurKernelSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_blur_kernel_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_setBlurKernelSize_int(self.as_raw_mut_SuperRes_SuperResolution(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Gaussian blur sigma
	/// ## See also
	/// setBlurSigma getBlurSigma
	// setBlurSigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:153
	// ("cv::superres::SuperResolution::setBlurSigma", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_blur_sigma(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_setBlurSigma_double(self.as_raw_mut_SuperRes_SuperResolution(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Radius of the temporal search area
	/// ## See also
	/// setTemporalAreaRadius getTemporalAreaRadius
	// setTemporalAreaRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:159
	// ("cv::superres::SuperResolution::setTemporalAreaRadius", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_temporal_area_radius(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_setTemporalAreaRadius_int(self.as_raw_mut_SuperRes_SuperResolution(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Dense optical flow algorithm
	/// ## See also
	/// setOpticalFlow getOpticalFlow
	// setOpticalFlow(const Ptr<cv::superres::DenseOpticalFlowExt> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:165
	// ("cv::superres::SuperResolution::setOpticalFlow", vec![(pred!(mut, ["val"], ["const cv::Ptr<cv::superres::DenseOpticalFlowExt>*"]), _)]),
	#[inline]
	fn set_optical_flow(&mut self, val: &core::Ptr<crate::superres::SuperRes_DenseOpticalFlowExt>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_superres_SuperResolution_setOpticalFlow_const_PtrLDenseOpticalFlowExtGR(self.as_raw_mut_SuperRes_SuperResolution(), val.as_raw_PtrOfSuperRes_DenseOpticalFlowExt(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Base class for Super Resolution algorithms.
///
/// The class is only used to define the common interface for the whole family of Super Resolution
/// algorithms.
// SuperResolution /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/superres.hpp:87
pub struct SuperRes_SuperResolution {
	ptr: *mut c_void,
}

opencv_type_boxed! { SuperRes_SuperResolution }

impl Drop for SuperRes_SuperResolution {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_superres_SuperResolution_delete(self.as_raw_mut_SuperRes_SuperResolution()) };
	}
}

unsafe impl Send for SuperRes_SuperResolution {}

impl core::AlgorithmTraitConst for SuperRes_SuperResolution {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SuperRes_SuperResolution {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SuperRes_SuperResolution, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::superres::SuperRes_FrameSourceTraitConst for SuperRes_SuperResolution {
	#[inline] fn as_raw_SuperRes_FrameSource(&self) -> *const c_void { self.as_raw() }
}

impl crate::superres::SuperRes_FrameSourceTrait for SuperRes_SuperResolution {
	#[inline] fn as_raw_mut_SuperRes_FrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SuperRes_SuperResolution, crate::superres::SuperRes_FrameSourceTraitConst, as_raw_SuperRes_FrameSource, crate::superres::SuperRes_FrameSourceTrait, as_raw_mut_SuperRes_FrameSource }

impl crate::superres::SuperRes_SuperResolutionTraitConst for SuperRes_SuperResolution {
	#[inline] fn as_raw_SuperRes_SuperResolution(&self) -> *const c_void { self.as_raw() }
}

impl crate::superres::SuperRes_SuperResolutionTrait for SuperRes_SuperResolution {
	#[inline] fn as_raw_mut_SuperRes_SuperResolution(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SuperRes_SuperResolution, crate::superres::SuperRes_SuperResolutionTraitConst, as_raw_SuperRes_SuperResolution, crate::superres::SuperRes_SuperResolutionTrait, as_raw_mut_SuperRes_SuperResolution }

impl SuperRes_SuperResolution {
}

boxed_cast_base! { SuperRes_SuperResolution, core::Algorithm, cv_superres_SuperResolution_to_Algorithm }

boxed_cast_base! { SuperRes_SuperResolution, crate::superres::SuperRes_FrameSource, cv_superres_SuperResolution_to_SuperRes_FrameSource }

impl std::fmt::Debug for SuperRes_SuperResolution {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SuperRes_SuperResolution")
			.finish()
	}
}
