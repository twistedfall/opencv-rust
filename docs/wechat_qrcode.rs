pub mod wechat_qrcode {
	//! # WeChat QR code detector for detecting and parsing QR code.
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::WeChatQRCodeTraitConst, super::WeChatQRCodeTrait };
	}
	
	/// Constant methods for [crate::wechat_qrcode::WeChatQRCode]
	pub trait WeChatQRCodeTraitConst {
		fn as_raw_WeChatQRCode(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::wechat_qrcode::WeChatQRCode]
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
		fn detect_and_decode(&mut self, img: &impl core::ToInputArray, points: &mut impl core::ToOutputArray) -> Result<core::Vector<String>> {
			input_array_arg!(img);
			output_array_arg!(points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_wechat_qrcode_WeChatQRCode_detectAndDecode_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_WeChatQRCode(), img.as_raw__InputArray(), points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<String>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
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
		/// ## Note
		/// This alternative version of [detect_and_decode] function uses the following default values for its arguments:
		/// * points: noArray()
		#[inline]
		fn detect_and_decode_def(&mut self, img: &impl core::ToInputArray) -> Result<core::Vector<String>> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_wechat_qrcode_WeChatQRCode_detectAndDecode_const__InputArrayR(self.as_raw_mut_WeChatQRCode(), img.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<String>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// set scale factor
		/// QR code detector use neural network to detect QR.
		/// Before running the neural network, the input image is pre-processed by scaling.
		/// By default, the input image is scaled to an image with an area of 160000 pixels.
		/// The scale factor allows to use custom scale the input image:
		/// width = scaleFactor*width
		/// height = scaleFactor*width
		/// 
		/// scaleFactor valuse must be > 0 and <= 1, otherwise the scaleFactor value is set to -1
		/// and use default scaled to an image with an area of 160000 pixels.
		#[inline]
		fn set_scale_factor(&mut self, _scaling_factor: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_wechat_qrcode_WeChatQRCode_setScaleFactor_float(self.as_raw_mut_WeChatQRCode(), _scaling_factor, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_scale_factor(&mut self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_wechat_qrcode_WeChatQRCode_getScaleFactor(self.as_raw_mut_WeChatQRCode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
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
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_wechat_qrcode_WeChatQRCode_delete(self.as_raw_mut_WeChatQRCode()) };
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
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * detector_prototxt_path: ""
		/// * detector_caffe_model_path: ""
		/// * super_resolution_prototxt_path: ""
		/// * super_resolution_caffe_model_path: ""
		#[inline]
		pub fn new_def() -> Result<crate::wechat_qrcode::WeChatQRCode> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_wechat_qrcode_WeChatQRCode_WeChatQRCode(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::wechat_qrcode::WeChatQRCode::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl std::fmt::Debug for WeChatQRCode {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("WeChatQRCode")
				.finish()
		}
	}
}
