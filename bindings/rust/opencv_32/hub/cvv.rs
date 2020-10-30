#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # GUI for Interactive Visual Debugging of Computer Vision Programs
//! 
//! Namespace for all functions is **cvv**, i.e. *cvv::showImage()*.
//! 
//! Compilation:
//! 
//! *   For development, i.e. for cvv GUI to show up, compile your code using cvv with
//!    *g++ -DCVVISUAL_DEBUGMODE*.
//! *   For release, i.e. cvv calls doing nothing, compile your code without above flag.
//! 
//! See cvv tutorial for a commented example application using cvv.
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::CallMetaDataTrait };
}

pub fn debug_d_match(img1: &dyn core::ToInputArray, mut keypoints1: core::Vector::<core::KeyPoint>, img2: &dyn core::ToInputArray, mut keypoints2: core::Vector::<core::KeyPoint>, mut matches: core::Vector::<core::DMatch>, data: &crate::cvv::CallMetaData, description: &str, view: &str, use_train_descriptor: bool) -> Result<()> {
	input_array_arg!(img1);
	input_array_arg!(img2);
	extern_container_arg!(description);
	extern_container_arg!(view);
	unsafe { sys::cvv_impl_debugDMatch_const__InputArrayR_vector_KeyPoint__const__InputArrayR_vector_KeyPoint__vector_DMatch__const_CallMetaDataR_const_charX_const_charX_bool(img1.as_raw__InputArray(), keypoints1.as_raw_mut_VectorOfKeyPoint(), img2.as_raw__InputArray(), keypoints2.as_raw_mut_VectorOfKeyPoint(), matches.as_raw_mut_VectorOfDMatch(), data.as_raw_CallMetaData(), description.opencv_as_extern(), view.opencv_as_extern(), use_train_descriptor) }.into_result()
}

pub fn debug_filter(original: &dyn core::ToInputArray, result: &dyn core::ToInputArray, data: &crate::cvv::CallMetaData, description: &str, view: &str) -> Result<()> {
	input_array_arg!(original);
	input_array_arg!(result);
	extern_container_arg!(description);
	extern_container_arg!(view);
	unsafe { sys::cvv_impl_debugFilter_const__InputArrayR_const__InputArrayR_const_CallMetaDataR_const_charX_const_charX(original.as_raw__InputArray(), result.as_raw__InputArray(), data.as_raw_CallMetaData(), description.opencv_as_extern(), view.opencv_as_extern()) }.into_result()
}

pub fn final_show() -> Result<()> {
	unsafe { sys::cvv_impl_finalShow() }.into_result()
}

pub fn show_image(img: &dyn core::ToInputArray, data: &crate::cvv::CallMetaData, description: &str, view: &str) -> Result<()> {
	input_array_arg!(img);
	extern_container_arg!(description);
	extern_container_arg!(view);
	unsafe { sys::cvv_impl_showImage_const__InputArrayR_const_CallMetaDataR_const_charX_const_charX(img.as_raw__InputArray(), data.as_raw_CallMetaData(), description.opencv_as_extern(), view.opencv_as_extern()) }.into_result()
}

/// Optional information about a location in Code.
pub trait CallMetaDataTrait {
	fn as_raw_CallMetaData(&self) -> *const c_void;
	fn as_raw_mut_CallMetaData(&mut self) -> *mut c_void;

	fn file(&self) -> String {
		unsafe { sys::cvv_impl_CallMetaData_getPropFile_const(self.as_raw_CallMetaData()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: file")
	}
	
	fn line(&self) -> size_t {
		unsafe { sys::cvv_impl_CallMetaData_getPropLine_const(self.as_raw_CallMetaData()) }.into_result().expect("Infallible function failed: line")
	}
	
	fn function(&self) -> String {
		unsafe { sys::cvv_impl_CallMetaData_getPropFunction_const(self.as_raw_CallMetaData()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: function")
	}
	
	/// Whether *this holds actual data.
	fn is_known(&self) -> bool {
		unsafe { sys::cvv_impl_CallMetaData_getPropIsKnown_const(self.as_raw_CallMetaData()) }.into_result().expect("Infallible function failed: is_known")
	}
	
	fn to_bool(&mut self) -> Result<bool> {
		unsafe { sys::cvv_impl_CallMetaData_operator_bool(self.as_raw_mut_CallMetaData()) }.into_result()
	}
	
}

/// Optional information about a location in Code.
pub struct CallMetaData {
	ptr: *mut c_void
}

opencv_type_boxed! { CallMetaData }

impl Drop for CallMetaData {
	fn drop(&mut self) {
		extern "C" { fn cv_CallMetaData_delete(instance: *mut c_void); }
		unsafe { cv_CallMetaData_delete(self.as_raw_mut_CallMetaData()) };
	}
}

impl CallMetaData {
	#[inline] pub fn as_raw_CallMetaData(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_CallMetaData(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for CallMetaData {}

impl crate::cvv::CallMetaDataTrait for CallMetaData {
	#[inline] fn as_raw_CallMetaData(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_CallMetaData(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CallMetaData {
	/// Creates an unknown location.
	pub fn default() -> Result<crate::cvv::CallMetaData> {
		unsafe { sys::cvv_impl_CallMetaData_CallMetaData() }.into_result().map(|r| unsafe { crate::cvv::CallMetaData::opencv_from_extern(r) } )
	}
	
	/// Creates the provided location.
	/// 
	/// Argument should be self-explaining.
	pub fn new(file: &str, line: size_t, function: &str) -> Result<crate::cvv::CallMetaData> {
		extern_container_arg!(file);
		extern_container_arg!(function);
		unsafe { sys::cvv_impl_CallMetaData_CallMetaData_const_charX_size_t_const_charX(file.opencv_as_extern(), line, function.opencv_as_extern()) }.into_result().map(|r| unsafe { crate::cvv::CallMetaData::opencv_from_extern(r) } )
	}
	
}
