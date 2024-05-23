//! # DNN used for super resolution
//!
//! This module contains functionality for upscaling an image via convolutional neural networks.
//! The following four models are implemented:
//!
//! - EDSR <https://arxiv.org/abs/1707.02921>
//! - ESPCN <https://arxiv.org/abs/1609.05158>
//! - FSRCNN <https://arxiv.org/abs/1608.00367>
//! - LapSRN <https://arxiv.org/abs/1710.01992>
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{DnnSuperResImplTrait, DnnSuperResImplTraitConst};
}

/// Constant methods for [crate::dnn_superres::DnnSuperResImpl]
// DnnSuperResImpl /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn_superres.hpp:40
pub trait DnnSuperResImplTraitConst {
	fn as_raw_DnnSuperResImpl(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn_superres::DnnSuperResImpl]
pub trait DnnSuperResImplTrait: crate::dnn_superres::DnnSuperResImplTraitConst {
	fn as_raw_mut_DnnSuperResImpl(&mut self) -> *mut c_void;

	/// Read the model from the given path
	/// ## Parameters
	/// * path: Path to the model file.
	// readModel(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn_superres.hpp:79
	// ("cv::dnn_superres::DnnSuperResImpl::readModel", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
	#[inline]
	fn read_model(&mut self, path: &str) -> Result<()> {
		extern_container_arg!(path);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_superres_DnnSuperResImpl_readModel_const_StringR(self.as_raw_mut_DnnSuperResImpl(), path.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Read the model from the given path
	/// ## Parameters
	/// * weights: Path to the model weights file.
	/// * definition: Path to the model definition file.
	// readModel(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn_superres.hpp:85
	// ("cv::dnn_superres::DnnSuperResImpl::readModel", vec![(pred!(mut, ["weights", "definition"], ["const cv::String*", "const cv::String*"]), _)]),
	#[inline]
	fn read_model_1(&mut self, weights: &str, definition: &str) -> Result<()> {
		extern_container_arg!(weights);
		extern_container_arg!(definition);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_superres_DnnSuperResImpl_readModel_const_StringR_const_StringR(self.as_raw_mut_DnnSuperResImpl(), weights.opencv_as_extern(), definition.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set desired model
	/// ## Parameters
	/// * algo: String containing one of the desired models:
	///    - __edsr__
	///    - __espcn__
	///    - __fsrcnn__
	///    - __lapsrn__
	/// * scale: Integer specifying the upscale factor
	// setModel(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn_superres.hpp:95
	// ("cv::dnn_superres::DnnSuperResImpl::setModel", vec![(pred!(mut, ["algo", "scale"], ["const cv::String*", "int"]), _)]),
	#[inline]
	fn set_model(&mut self, algo: &str, scale: i32) -> Result<()> {
		extern_container_arg!(algo);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_superres_DnnSuperResImpl_setModel_const_StringR_int(self.as_raw_mut_DnnSuperResImpl(), algo.opencv_as_extern(), scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set computation backend
	// setPreferableBackend(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn_superres.hpp:99
	// ("cv::dnn_superres::DnnSuperResImpl::setPreferableBackend", vec![(pred!(mut, ["backendId"], ["int"]), _)]),
	#[inline]
	fn set_preferable_backend(&mut self, backend_id: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_superres_DnnSuperResImpl_setPreferableBackend_int(self.as_raw_mut_DnnSuperResImpl(), backend_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set computation target
	// setPreferableTarget(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn_superres.hpp:103
	// ("cv::dnn_superres::DnnSuperResImpl::setPreferableTarget", vec![(pred!(mut, ["targetId"], ["int"]), _)]),
	#[inline]
	fn set_preferable_target(&mut self, target_id: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_superres_DnnSuperResImpl_setPreferableTarget_int(self.as_raw_mut_DnnSuperResImpl(), target_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Upsample via neural network
	/// ## Parameters
	/// * img: Image to upscale
	/// * result: Destination upscaled image
	// upsample(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn_superres.hpp:109
	// ("cv::dnn_superres::DnnSuperResImpl::upsample", vec![(pred!(mut, ["img", "result"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn upsample(&mut self, img: &impl ToInputArray, result: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(img);
		output_array_arg!(result);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_superres_DnnSuperResImpl_upsample_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_DnnSuperResImpl(), img.as_raw__InputArray(), result.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Upsample via neural network of multiple outputs
	/// ## Parameters
	/// * img: Image to upscale
	/// * imgs_new: Destination upscaled images
	/// * scale_factors: Scaling factors of the output nodes
	/// * node_names: Names of the output nodes in the neural network
	// upsampleMultioutput(InputArray, std::vector<Mat> &, const std::vector<int> &, const std::vector<String> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn_superres.hpp:117
	// ("cv::dnn_superres::DnnSuperResImpl::upsampleMultioutput", vec![(pred!(mut, ["img", "imgs_new", "scale_factors", "node_names"], ["const cv::_InputArray*", "std::vector<cv::Mat>*", "const std::vector<int>*", "const std::vector<cv::String>*"]), _)]),
	#[inline]
	fn upsample_multioutput(&mut self, img: &impl ToInputArray, imgs_new: &mut core::Vector<core::Mat>, scale_factors: &core::Vector<i32>, node_names: &core::Vector<String>) -> Result<()> {
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_superres_DnnSuperResImpl_upsampleMultioutput_const__InputArrayR_vectorLMatGR_const_vectorLintGR_const_vectorLStringGR(self.as_raw_mut_DnnSuperResImpl(), img.as_raw__InputArray(), imgs_new.as_raw_mut_VectorOfMat(), scale_factors.as_raw_VectorOfi32(), node_names.as_raw_VectorOfString(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the scale factor of the model:
	/// ## Returns
	/// Current scale factor.
	// getScale()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn_superres.hpp:122
	// ("cv::dnn_superres::DnnSuperResImpl::getScale", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_scale(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_superres_DnnSuperResImpl_getScale(self.as_raw_mut_DnnSuperResImpl(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the scale factor of the model:
	/// ## Returns
	/// Current algorithm.
	// getAlgorithm()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn_superres.hpp:127
	// ("cv::dnn_superres::DnnSuperResImpl::getAlgorithm", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_algorithm(&mut self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_superres_DnnSuperResImpl_getAlgorithm(self.as_raw_mut_DnnSuperResImpl(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// A class to upscale images via convolutional neural networks.
/// The following four models are implemented:
///
/// - edsr
/// - espcn
/// - fsrcnn
/// - lapsrn
// DnnSuperResImpl /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn_superres.hpp:40
pub struct DnnSuperResImpl {
	ptr: *mut c_void,
}

opencv_type_boxed! { DnnSuperResImpl }

impl Drop for DnnSuperResImpl {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_superres_DnnSuperResImpl_delete(self.as_raw_mut_DnnSuperResImpl()) };
	}
}

unsafe impl Send for DnnSuperResImpl {}

impl crate::dnn_superres::DnnSuperResImplTraitConst for DnnSuperResImpl {
	#[inline] fn as_raw_DnnSuperResImpl(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn_superres::DnnSuperResImplTrait for DnnSuperResImpl {
	#[inline] fn as_raw_mut_DnnSuperResImpl(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DnnSuperResImpl, crate::dnn_superres::DnnSuperResImplTraitConst, as_raw_DnnSuperResImpl, crate::dnn_superres::DnnSuperResImplTrait, as_raw_mut_DnnSuperResImpl }

impl DnnSuperResImpl {
	/// Empty constructor for python
	// create()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn_superres.hpp:60
	// ("cv::dnn_superres::DnnSuperResImpl::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::dnn_superres::DnnSuperResImpl>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_superres_DnnSuperResImpl_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn_superres::DnnSuperResImpl>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// DnnSuperResImpl()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn_superres.hpp:64
	// ("cv::dnn_superres::DnnSuperResImpl::DnnSuperResImpl", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::dnn_superres::DnnSuperResImpl> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_superres_DnnSuperResImpl_DnnSuperResImpl(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn_superres::DnnSuperResImpl::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor which immediately sets the desired model
	/// ## Parameters
	/// * algo: String containing one of the desired models:
	///    - __edsr__
	///    - __espcn__
	///    - __fsrcnn__
	///    - __lapsrn__
	/// * scale: Integer specifying the upscale factor
	// DnnSuperResImpl(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn_superres.hpp:74
	// ("cv::dnn_superres::DnnSuperResImpl::DnnSuperResImpl", vec![(pred!(mut, ["algo", "scale"], ["const cv::String*", "int"]), _)]),
	#[inline]
	pub fn new(algo: &str, scale: i32) -> Result<crate::dnn_superres::DnnSuperResImpl> {
		extern_container_arg!(algo);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_superres_DnnSuperResImpl_DnnSuperResImpl_const_StringR_int(algo.opencv_as_extern(), scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn_superres::DnnSuperResImpl::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for DnnSuperResImpl {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("DnnSuperResImpl")
			.finish()
	}
}
