#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # DNN used for super resolution
//! 
//! This module contains functionality for upscaling an image via convolutional neural networks.
//! The following four models are implemented:
//! 
//! - EDSR <https://arxiv.org/abs/1707.02921>
//! - ESPCN <https://arxiv.org/abs/1609.05158>
//! - FSRCNN <https://arxiv.org/abs/1608.00367>
//! - LapSRN <https://arxiv.org/abs/1710.01992>
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::DnnSuperResImplTraitConst, super::DnnSuperResImplTrait };
}

/// A class to upscale images via convolutional neural networks.
/// The following four models are implemented:
/// 
/// - edsr
/// - espcn
/// - fsrcnn
/// - lapsrn
pub trait DnnSuperResImplTraitConst {
	fn as_raw_DnnSuperResImpl(&self) -> *const c_void;

}

pub trait DnnSuperResImplTrait: crate::dnn_superres::DnnSuperResImplTraitConst {
	fn as_raw_mut_DnnSuperResImpl(&mut self) -> *mut c_void;

	/// Read the model from the given path
	/// ## Parameters
	/// * path: Path to the model file.
	#[inline]
	fn read_model(&mut self, path: &str) -> Result<()> {
		extern_container_arg!(path);
		let ret = unsafe { sys::cv_dnn_superres_DnnSuperResImpl_readModel_const_StringR(self.as_raw_mut_DnnSuperResImpl(), path.opencv_as_extern()) }.into_result()?;
		Ok(ret)
	}
	
	/// Read the model from the given path
	/// ## Parameters
	/// * weights: Path to the model weights file.
	/// * definition: Path to the model definition file.
	#[inline]
	fn read_model_1(&mut self, weights: &str, definition: &str) -> Result<()> {
		extern_container_arg!(weights);
		extern_container_arg!(definition);
		let ret = unsafe { sys::cv_dnn_superres_DnnSuperResImpl_readModel_const_StringR_const_StringR(self.as_raw_mut_DnnSuperResImpl(), weights.opencv_as_extern(), definition.opencv_as_extern()) }.into_result()?;
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
	#[inline]
	fn set_model(&mut self, algo: &str, scale: i32) -> Result<()> {
		extern_container_arg!(algo);
		let ret = unsafe { sys::cv_dnn_superres_DnnSuperResImpl_setModel_const_StringR_int(self.as_raw_mut_DnnSuperResImpl(), algo.opencv_as_extern(), scale) }.into_result()?;
		Ok(ret)
	}
	
	/// Set computation backend
	#[inline]
	fn set_preferable_backend(&mut self, backend_id: i32) -> Result<()> {
		let ret = unsafe { sys::cv_dnn_superres_DnnSuperResImpl_setPreferableBackend_int(self.as_raw_mut_DnnSuperResImpl(), backend_id) }.into_result()?;
		Ok(ret)
	}
	
	/// Set computation target
	#[inline]
	fn set_preferable_target(&mut self, target_id: i32) -> Result<()> {
		let ret = unsafe { sys::cv_dnn_superres_DnnSuperResImpl_setPreferableTarget_int(self.as_raw_mut_DnnSuperResImpl(), target_id) }.into_result()?;
		Ok(ret)
	}
	
	/// Upsample via neural network
	/// ## Parameters
	/// * img: Image to upscale
	/// * result: Destination upscaled image
	#[inline]
	fn upsample(&mut self, img: &dyn core::ToInputArray, result: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(img);
		output_array_arg!(result);
		let ret = unsafe { sys::cv_dnn_superres_DnnSuperResImpl_upsample_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_DnnSuperResImpl(), img.as_raw__InputArray(), result.as_raw__OutputArray()) }.into_result()?;
		Ok(ret)
	}
	
	/// Upsample via neural network of multiple outputs
	/// ## Parameters
	/// * img: Image to upscale
	/// * imgs_new: Destination upscaled images
	/// * scale_factors: Scaling factors of the output nodes
	/// * node_names: Names of the output nodes in the neural network
	#[inline]
	fn upsample_multioutput(&mut self, img: &dyn core::ToInputArray, imgs_new: &mut core::Vector<core::Mat>, scale_factors: &core::Vector<i32>, node_names: &core::Vector<String>) -> Result<()> {
		input_array_arg!(img);
		let ret = unsafe { sys::cv_dnn_superres_DnnSuperResImpl_upsampleMultioutput_const__InputArrayR_vector_Mat_R_const_vector_int_R_const_vector_String_R(self.as_raw_mut_DnnSuperResImpl(), img.as_raw__InputArray(), imgs_new.as_raw_mut_VectorOfMat(), scale_factors.as_raw_VectorOfi32(), node_names.as_raw_VectorOfString()) }.into_result()?;
		Ok(ret)
	}
	
	/// Returns the scale factor of the model:
	/// ## Returns
	/// Current scale factor.
	#[inline]
	fn get_scale(&mut self) -> Result<i32> {
		let ret = unsafe { sys::cv_dnn_superres_DnnSuperResImpl_getScale(self.as_raw_mut_DnnSuperResImpl()) }.into_result()?;
		Ok(ret)
	}
	
	/// Returns the scale factor of the model:
	/// ## Returns
	/// Current algorithm.
	#[inline]
	fn get_algorithm(&mut self) -> Result<String> {
		let ret = unsafe { sys::cv_dnn_superres_DnnSuperResImpl_getAlgorithm(self.as_raw_mut_DnnSuperResImpl()) }.into_result()?;
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
pub struct DnnSuperResImpl {
	ptr: *mut c_void
}

opencv_type_boxed! { DnnSuperResImpl }

impl Drop for DnnSuperResImpl {
	fn drop(&mut self) {
		extern "C" { fn cv_DnnSuperResImpl_delete(instance: *mut c_void); }
		unsafe { cv_DnnSuperResImpl_delete(self.as_raw_mut_DnnSuperResImpl()) };
	}
}

unsafe impl Send for DnnSuperResImpl {}

impl crate::dnn_superres::DnnSuperResImplTraitConst for DnnSuperResImpl {
	#[inline] fn as_raw_DnnSuperResImpl(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn_superres::DnnSuperResImplTrait for DnnSuperResImpl {
	#[inline] fn as_raw_mut_DnnSuperResImpl(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl DnnSuperResImpl {
	/// Empty constructor for python
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::dnn_superres::DnnSuperResImpl>> {
		let ret = unsafe { sys::cv_dnn_superres_DnnSuperResImpl_create() }.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn_superres::DnnSuperResImpl>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn default() -> Result<crate::dnn_superres::DnnSuperResImpl> {
		let ret = unsafe { sys::cv_dnn_superres_DnnSuperResImpl_DnnSuperResImpl() }.into_result()?;
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
	#[inline]
	pub fn new(algo: &str, scale: i32) -> Result<crate::dnn_superres::DnnSuperResImpl> {
		extern_container_arg!(algo);
		let ret = unsafe { sys::cv_dnn_superres_DnnSuperResImpl_DnnSuperResImpl_const_StringR_int(algo.opencv_as_extern(), scale) }.into_result()?;
		let ret = unsafe { crate::dnn_superres::DnnSuperResImpl::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
