pub mod dnn_superres {
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
	
	/// Constant methods for [crate::dnn_superres::DnnSuperResImpl]
	pub trait DnnSuperResImplTraitConst {
		fn as_raw_DnnSuperResImpl(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn_superres::DnnSuperResImpl]
	pub trait DnnSuperResImplTrait: crate::dnn_superres::DnnSuperResImplTraitConst {
		fn as_raw_mut_DnnSuperResImpl(&mut self) -> *mut c_void;
	
		/// Read the model from the given path
		/// ## Parameters
		/// * path: Path to the model file.
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
		#[inline]
		fn set_preferable_backend(&mut self, backend_id: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_superres_DnnSuperResImpl_setPreferableBackend_int(self.as_raw_mut_DnnSuperResImpl(), backend_id, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Set computation target
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
		#[inline]
		fn upsample(&mut self, img: &impl core::ToInputArray, result: &mut impl core::ToOutputArray) -> Result<()> {
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
		#[inline]
		fn upsample_multioutput(&mut self, img: &impl core::ToInputArray, imgs_new: &mut core::Vector<core::Mat>, scale_factors: &core::Vector<i32>, node_names: &core::Vector<String>) -> Result<()> {
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
	pub struct DnnSuperResImpl {
		ptr: *mut c_void
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
	
	impl DnnSuperResImpl {
		/// Empty constructor for python
		#[inline]
		pub fn create() -> Result<core::Ptr<crate::dnn_superres::DnnSuperResImpl>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_superres_DnnSuperResImpl_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn_superres::DnnSuperResImpl>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
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
}
