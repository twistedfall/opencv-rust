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
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{CallMetaDataTrait, CallMetaDataTraitConst};
}

// debugDMatch(cv::InputArray, std::vector<cv::KeyPoint>, cv::InputArray, std::vector<cv::KeyPoint>, std::vector<cv::DMatch>, const CallMetaData &, const char *, const char *, bool)(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, TraitClass, InString, InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/dmatch.hpp:24
// ("cvv::impl::debugDMatch", vec![(pred!(mut, ["img1", "keypoints1", "img2", "keypoints2", "matches", "data", "description", "view", "useTrainDescriptor"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>", "const cv::_InputArray*", "std::vector<cv::KeyPoint>", "std::vector<cv::DMatch>", "const cvv::impl::CallMetaData*", "const char*", "const char*", "bool"]), _)]),
#[inline]
pub fn debug_d_match(img1: &impl ToInputArray, mut keypoints1: core::Vector<core::KeyPoint>, img2: &impl ToInputArray, mut keypoints2: core::Vector<core::KeyPoint>, mut matches: core::Vector<core::DMatch>, data: &impl crate::cvv::CallMetaDataTraitConst, description: &str, view: &str, use_train_descriptor: bool) -> Result<()> {
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

// debugFilter(cv::InputArray, cv::InputArray, const CallMetaData &, const char *, const char *)(InputArray, InputArray, TraitClass, InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/filter.hpp:24
// ("cvv::impl::debugFilter", vec![(pred!(mut, ["original", "result", "data", "description", "view"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cvv::impl::CallMetaData*", "const char*", "const char*"]), _)]),
#[inline]
pub fn debug_filter(original: &impl ToInputArray, result: &impl ToInputArray, data: &impl crate::cvv::CallMetaDataTraitConst, description: &str, view: &str) -> Result<()> {
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

// finalShow()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/final_show.hpp:15
// ("cvv::impl::finalShow", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn final_show() -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cvv_impl_finalShow(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// showImage(cv::InputArray, const CallMetaData &, const char *, const char *)(InputArray, TraitClass, InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/show_image.hpp:24
// ("cvv::impl::showImage", vec![(pred!(mut, ["img", "data", "description", "view"], ["const cv::_InputArray*", "const cvv::impl::CallMetaData*", "const char*", "const char*"]), _)]),
#[inline]
pub fn show_image(img: &impl ToInputArray, data: &impl crate::cvv::CallMetaDataTraitConst, description: &str, view: &str) -> Result<()> {
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
// CallMetaData /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/call_meta_data.hpp:20
pub trait CallMetaDataTraitConst {
	fn as_raw_CallMetaData(&self) -> *const c_void;

	// cvv::impl::CallMetaData::file() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/call_meta_data.hpp:46
	// ("cvv::impl::CallMetaData::file", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn file(&self) -> String {
		let ret = unsafe { sys::cvv_impl_CallMetaData_propFile_const(self.as_raw_CallMetaData()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}

	// cvv::impl::CallMetaData::line() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/call_meta_data.hpp:47
	// ("cvv::impl::CallMetaData::line", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn line(&self) -> size_t {
		let ret = unsafe { sys::cvv_impl_CallMetaData_propLine_const(self.as_raw_CallMetaData()) };
		ret
	}

	// cvv::impl::CallMetaData::function() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/call_meta_data.hpp:48
	// ("cvv::impl::CallMetaData::function", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn function(&self) -> String {
		let ret = unsafe { sys::cvv_impl_CallMetaData_propFunction_const(self.as_raw_CallMetaData()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}

	/// Whether *this holds actual data.
	// cvv::impl::CallMetaData::isKnown() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/call_meta_data.hpp:53
	// ("cvv::impl::CallMetaData::isKnown", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn is_known(&self) -> bool {
		let ret = unsafe { sys::cvv_impl_CallMetaData_propIsKnown_const(self.as_raw_CallMetaData()) };
		ret
	}

}

/// Mutable methods for [crate::cvv::CallMetaData]
pub trait CallMetaDataTrait: crate::cvv::CallMetaDataTraitConst {
	fn as_raw_mut_CallMetaData(&mut self) -> *mut c_void;

	// operator bool()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/call_meta_data.hpp:40
	// ("cvv::impl::CallMetaData::operator bool", vec![(pred!(mut, [], []), _)]),
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
// CallMetaData /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/call_meta_data.hpp:20
pub struct CallMetaData {
	ptr: *mut c_void,
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

boxed_ref! { CallMetaData, crate::cvv::CallMetaDataTraitConst, as_raw_CallMetaData, crate::cvv::CallMetaDataTrait, as_raw_mut_CallMetaData }

impl CallMetaData {
	/// Creates an unknown location.
	// CallMetaData()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/call_meta_data.hpp:26
	// ("cvv::impl::CallMetaData::CallMetaData", vec![(pred!(mut, [], []), _)]),
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
	// CallMetaData(const char *, size_t, const char *)(InString, Primitive, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cvv/call_meta_data.hpp:36
	// ("cvv::impl::CallMetaData::CallMetaData", vec![(pred!(mut, ["file", "line", "function"], ["const char*", "size_t", "const char*"]), _)]),
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
