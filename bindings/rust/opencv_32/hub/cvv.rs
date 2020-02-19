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

pub fn debug_d_match(img1: &dyn core::ToInputArray, keypoints1: types::VectorOfKeyPoint, img2: &dyn core::ToInputArray, keypoints2: types::VectorOfKeyPoint, matches: types::VectorOfDMatch, data: &crate::cvv::CallMetaData, description: &str, view: &str, use_train_descriptor: bool) -> Result<()> {
	input_array_arg!(img1);
	input_array_arg!(img2);
	string_arg!(description);
	string_arg!(view);
	unsafe { sys::cvv_impl_debugDMatch_const__InputArrayX_vector_KeyPoint__const__InputArrayX_vector_KeyPoint__vector_DMatch__const_CallMetaDataX_const_charX_const_charX_bool(img1.as_raw__InputArray(), keypoints1.as_raw_VectorOfKeyPoint(), img2.as_raw__InputArray(), keypoints2.as_raw_VectorOfKeyPoint(), matches.as_raw_VectorOfDMatch(), data.as_raw_CallMetaData(), description.as_ptr(), view.as_ptr(), use_train_descriptor) }.into_result()
}

pub fn debug_filter(original: &dyn core::ToInputArray, result: &dyn core::ToInputArray, data: &crate::cvv::CallMetaData, description: &str, view: &str) -> Result<()> {
	input_array_arg!(original);
	input_array_arg!(result);
	string_arg!(description);
	string_arg!(view);
	unsafe { sys::cvv_impl_debugFilter_const__InputArrayX_const__InputArrayX_const_CallMetaDataX_const_charX_const_charX(original.as_raw__InputArray(), result.as_raw__InputArray(), data.as_raw_CallMetaData(), description.as_ptr(), view.as_ptr()) }.into_result()
}

pub fn final_show() -> Result<()> {
	unsafe { sys::cvv_impl_finalShow() }.into_result()
}

pub fn show_image(img: &dyn core::ToInputArray, data: &crate::cvv::CallMetaData, description: &str, view: &str) -> Result<()> {
	input_array_arg!(img);
	string_arg!(description);
	string_arg!(view);
	unsafe { sys::cvv_impl_showImage_const__InputArrayX_const_CallMetaDataX_const_charX_const_charX(img.as_raw__InputArray(), data.as_raw_CallMetaData(), description.as_ptr(), view.as_ptr()) }.into_result()
}

/// Optional information about a location in Code.
pub trait CallMetaDataTrait {
	fn as_raw_CallMetaData(&self) -> *mut c_void;
	fn file(&self) -> String {
		unsafe { sys::cvv_impl_CallMetaData_file_const(self.as_raw_CallMetaData()) }.into_result().map(crate::templ::receive_string).expect("Infallible function failed: file")
	}
	
	fn line(&self) -> size_t {
		unsafe { sys::cvv_impl_CallMetaData_line_const(self.as_raw_CallMetaData()) }.into_result().expect("Infallible function failed: line")
	}
	
	fn function(&self) -> String {
		unsafe { sys::cvv_impl_CallMetaData_function_const(self.as_raw_CallMetaData()) }.into_result().map(crate::templ::receive_string).expect("Infallible function failed: function")
	}
	
	/// Whether *this holds actual data.
	fn is_known(&self) -> bool {
		unsafe { sys::cvv_impl_CallMetaData_isKnown_const(self.as_raw_CallMetaData()) }.into_result().expect("Infallible function failed: is_known")
	}
	
	fn to_bool(&mut self) -> Result<bool> {
		unsafe { sys::cvv_impl_CallMetaData_operator_bool(self.as_raw_CallMetaData()) }.into_result()
	}
	
}

/// Optional information about a location in Code.
pub struct CallMetaData {
	pub(crate) ptr: *mut c_void
}

impl Drop for CallMetaData {
	fn drop(&mut self) {
		extern "C" { fn cv_CallMetaData_delete(instance: *mut c_void); }
		unsafe { cv_CallMetaData_delete(self.as_raw_CallMetaData()) };
	}
}

impl CallMetaData {
	pub fn as_raw_CallMetaData(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for CallMetaData {}

impl crate::cvv::CallMetaDataTrait for CallMetaData {
	fn as_raw_CallMetaData(&self) -> *mut c_void { self.ptr }
}

impl CallMetaData {
	/// Creates an unknown location.
	pub fn default() -> Result<crate::cvv::CallMetaData> {
		unsafe { sys::cvv_impl_CallMetaData_CallMetaData() }.into_result().map(|ptr| crate::cvv::CallMetaData { ptr })
	}
	
	/// Creates the provided location.
	/// 
	/// Argument should be self-explaining.
	pub fn new(file: &str, line: size_t, function: &str) -> Result<crate::cvv::CallMetaData> {
		string_arg!(file);
		string_arg!(function);
		unsafe { sys::cvv_impl_CallMetaData_CallMetaData_const_charX_size_t_const_charX(file.as_ptr(), line, function.as_ptr()) }.into_result().map(|ptr| crate::cvv::CallMetaData { ptr })
	}
	
}
