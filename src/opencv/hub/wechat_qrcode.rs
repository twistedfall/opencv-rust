#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # WeChat QR code detector for detecting and parsing QR code.
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::WeChatQRCodeTraitConst, super::WeChatQRCodeTrait };
}

/// * WeChat QRCode includes two CNN-based models:
/// * A object detection model and a super resolution model.
/// * Object detection model is applied to detect QRCode with the bounding box.
/// * super resolution model is applied to zoom in QRCode when it is small.
/// *
pub trait WeChatQRCodeTraitConst {
	fn as_raw_WeChatQRCode(&self) -> *const c_void;

}

pub trait WeChatQRCodeTrait: crate::wechat_qrcode::WeChatQRCodeTraitConst {
	fn as_raw_mut_WeChatQRCode(&mut self) -> *mut c_void;

	/// Both detects and decodes QR code.
	/// To simplify the usage, there is a only API: detectAndDecode
	/// 
	/// ## Parameters
	/// * img: supports grayscale or color (BGR) image.
	/// * points: optional output array of vertices of the found QR code quadrangle. Will be
	/// empty if not found.
	/// ## Returns
	/// list of decoded string.
	/// 
	/// ## C++ default parameters
	/// * points: noArray()
	#[inline]
	fn detect_and_decode(&mut self, img: &dyn core::ToInputArray, points: &mut dyn core::ToOutputArray) -> Result<core::Vector<String>> {
		input_array_arg!(img);
		output_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_wechat_qrcode_WeChatQRCode_detectAndDecode_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_WeChatQRCode(), img.as_raw__InputArray(), points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<String>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

/// * WeChat QRCode includes two CNN-based models:
/// * A object detection model and a super resolution model.
/// * Object detection model is applied to detect QRCode with the bounding box.
/// * super resolution model is applied to zoom in QRCode when it is small.
/// *
pub struct WeChatQRCode {
	ptr: *mut c_void
}

opencv_type_boxed! { WeChatQRCode }

impl Drop for WeChatQRCode {
	fn drop(&mut self) {
		extern "C" { fn cv_WeChatQRCode_delete(instance: *mut c_void); }
		unsafe { cv_WeChatQRCode_delete(self.as_raw_mut_WeChatQRCode()) };
	}
}

unsafe impl Send for WeChatQRCode {}

impl crate::wechat_qrcode::WeChatQRCodeTraitConst for WeChatQRCode {
	#[inline] fn as_raw_WeChatQRCode(&self) -> *const c_void { self.as_raw() }
}

impl crate::wechat_qrcode::WeChatQRCodeTrait for WeChatQRCode {
	#[inline] fn as_raw_mut_WeChatQRCode(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WeChatQRCode {
	/// Initialize the WeChatQRCode.
	/// It includes two models, which are packaged with caffe format.
	/// Therefore, there are prototxt and caffe models (In total, four paramenters).
	/// 
	/// ## Parameters
	/// * detector_prototxt_path: prototxt file path for the detector
	/// * detector_caffe_model_path: caffe model file path for the detector
	/// * super_resolution_prototxt_path: prototxt file path for the super resolution model
	/// * super_resolution_caffe_model_path: caffe file path for the super resolution model
	/// 
	/// ## C++ default parameters
	/// * detector_prototxt_path: ""
	/// * detector_caffe_model_path: ""
	/// * super_resolution_prototxt_path: ""
	/// * super_resolution_caffe_model_path: ""
	#[inline]
	pub fn new(detector_prototxt_path: &str, detector_caffe_model_path: &str, super_resolution_prototxt_path: &str, super_resolution_caffe_model_path: &str) -> Result<crate::wechat_qrcode::WeChatQRCode> {
		extern_container_arg!(detector_prototxt_path);
		extern_container_arg!(detector_caffe_model_path);
		extern_container_arg!(super_resolution_prototxt_path);
		extern_container_arg!(super_resolution_caffe_model_path);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_wechat_qrcode_WeChatQRCode_WeChatQRCode_const_stringR_const_stringR_const_stringR_const_stringR(detector_prototxt_path.opencv_as_extern(), detector_caffe_model_path.opencv_as_extern(), super_resolution_prototxt_path.opencv_as_extern(), super_resolution_caffe_model_path.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::wechat_qrcode::WeChatQRCode::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
