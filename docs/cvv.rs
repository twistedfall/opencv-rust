pub mod cvv {
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
		pub use { super::CallMetaDataTraitConst, super::CallMetaDataTrait };
	}
	
	#[inline]
	pub fn debug_d_match(img1: &impl core::ToInputArray, mut keypoints1: core::Vector<core::KeyPoint>, img2: &impl core::ToInputArray, mut keypoints2: core::Vector<core::KeyPoint>, mut matches: core::Vector<core::DMatch>, data: &crate::cvv::CallMetaData, description: &str, view: &str, use_train_descriptor: bool) -> Result<()> {
		input_array_arg!(img1);
		input_array_arg!(img2);
		extern_container_arg!(description);
		extern_container_arg!(view);
		return_send!(via ocvrs_return);
		unsafe { sys::cvv_impl_debugDMatch_const__InputArrayR_vectorLKeyPointG_const__InputArrayR_vectorLKeyPointG_vectorLDMatchG_const_CallMetaDataR_const_charX_const_charX_bool(img1.as_raw__InputArray(), keypoints1.as_raw_mut_VectorOfKeyPoint(), img2.as_raw__InputArray(), keypoints2.as_raw_mut_VectorOfKeyPoint(), matches.as_raw_mut_VectorOfDMatch(), data.as_raw_CallMetaData(), description.opencv_as_extern(), view.opencv_as_extern(), use_train_descriptor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn debug_filter(original: &impl core::ToInputArray, result: &impl core::ToInputArray, data: &crate::cvv::CallMetaData, description: &str, view: &str) -> Result<()> {
		input_array_arg!(original);
		input_array_arg!(result);
		extern_container_arg!(description);
		extern_container_arg!(view);
		return_send!(via ocvrs_return);
		unsafe { sys::cvv_impl_debugFilter_const__InputArrayR_const__InputArrayR_const_CallMetaDataR_const_charX_const_charX(original.as_raw__InputArray(), result.as_raw__InputArray(), data.as_raw_CallMetaData(), description.opencv_as_extern(), view.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn final_show() -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cvv_impl_finalShow(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn show_image(img: &impl core::ToInputArray, data: &crate::cvv::CallMetaData, description: &str, view: &str) -> Result<()> {
		input_array_arg!(img);
		extern_container_arg!(description);
		extern_container_arg!(view);
		return_send!(via ocvrs_return);
		unsafe { sys::cvv_impl_showImage_const__InputArrayR_const_CallMetaDataR_const_charX_const_charX(img.as_raw__InputArray(), data.as_raw_CallMetaData(), description.opencv_as_extern(), view.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Constant methods for [crate::cvv::CallMetaData]
	pub trait CallMetaDataTraitConst {
		fn as_raw_CallMetaData(&self) -> *const c_void;
	
		#[inline]
		fn file(&self) -> String {
			let ret = unsafe { sys::cvv_impl_CallMetaData_propFile_const(self.as_raw_CallMetaData()) };
			let ret = unsafe { String::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn line(&self) -> size_t {
			let ret = unsafe { sys::cvv_impl_CallMetaData_propLine_const(self.as_raw_CallMetaData()) };
			ret
		}
		
		#[inline]
		fn function(&self) -> String {
			let ret = unsafe { sys::cvv_impl_CallMetaData_propFunction_const(self.as_raw_CallMetaData()) };
			let ret = unsafe { String::opencv_from_extern(ret) };
			ret
		}
		
		/// Whether *this holds actual data.
		#[inline]
		fn is_known(&self) -> bool {
			let ret = unsafe { sys::cvv_impl_CallMetaData_propIsKnown_const(self.as_raw_CallMetaData()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::cvv::CallMetaData]
	pub trait CallMetaDataTrait: crate::cvv::CallMetaDataTraitConst {
		fn as_raw_mut_CallMetaData(&mut self) -> *mut c_void;
	
		#[inline]
		fn to_bool(&mut self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cvv_impl_CallMetaData_operator_bool(self.as_raw_mut_CallMetaData(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Optional information about a location in Code.
	pub struct CallMetaData {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CallMetaData }
	
	impl Drop for CallMetaData {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cvv_impl_CallMetaData_delete(self.as_raw_mut_CallMetaData()) };
		}
	}
	
	unsafe impl Send for CallMetaData {}
	
	impl crate::cvv::CallMetaDataTraitConst for CallMetaData {
		#[inline] fn as_raw_CallMetaData(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cvv::CallMetaDataTrait for CallMetaData {
		#[inline] fn as_raw_mut_CallMetaData(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CallMetaData {
		/// Creates an unknown location.
		#[inline]
		pub fn default() -> Result<crate::cvv::CallMetaData> {
			return_send!(via ocvrs_return);
			unsafe { sys::cvv_impl_CallMetaData_CallMetaData(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::cvv::CallMetaData::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Creates the provided location.
		/// 
		/// Argument should be self-explaining.
		#[inline]
		pub fn new(file: &str, line: size_t, function: &str) -> Result<crate::cvv::CallMetaData> {
			extern_container_arg!(file);
			extern_container_arg!(function);
			return_send!(via ocvrs_return);
			unsafe { sys::cvv_impl_CallMetaData_CallMetaData_const_charX_size_t_const_charX(file.opencv_as_extern(), line, function.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::cvv::CallMetaData::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl Clone for CallMetaData {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cvv_impl_CallMetaData_implicitClone_const(self.as_raw_CallMetaData())) }
		}
	}
	
	impl std::fmt::Debug for CallMetaData {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CallMetaData")
				.field("file", &crate::cvv::CallMetaDataTraitConst::file(self))
				.field("line", &crate::cvv::CallMetaDataTraitConst::line(self))
				.field("function", &crate::cvv::CallMetaDataTraitConst::function(self))
				.field("is_known", &crate::cvv::CallMetaDataTraitConst::is_known(self))
				.finish()
		}
	}
}
