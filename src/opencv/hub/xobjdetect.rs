#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Extended object detection
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::WBDetectorConst, super::WBDetector };
}

/// WaldBoost detector
pub trait WBDetectorConst {
	fn as_raw_WBDetector(&self) -> *const c_void;

	/// Write detector to FileStorage.
	/// ## Parameters
	/// * fs: FileStorage for output
	#[inline]
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xobjdetect_WBDetector_write_const_FileStorageR(self.as_raw_WBDetector(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait WBDetector: crate::xobjdetect::WBDetectorConst {
	fn as_raw_mut_WBDetector(&mut self) -> *mut c_void;

	/// Read detector from FileNode.
	/// ## Parameters
	/// * node: FileNode for input
	#[inline]
	fn read(&mut self, node: &core::FileNode) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xobjdetect_WBDetector_read_const_FileNodeR(self.as_raw_mut_WBDetector(), node.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Train WaldBoost detector
	/// ## Parameters
	/// * pos_samples: Path to directory with cropped positive samples
	/// * neg_imgs: Path to directory with negative (background) images
	#[inline]
	fn train(&mut self, pos_samples: &str, neg_imgs: &str) -> Result<()> {
		extern_container_arg!(pos_samples);
		extern_container_arg!(neg_imgs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xobjdetect_WBDetector_train_const_stringR_const_stringR(self.as_raw_mut_WBDetector(), pos_samples.opencv_as_extern(), neg_imgs.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Detect objects on image using WaldBoost detector
	/// ## Parameters
	/// * img: Input image for detection
	/// * bboxes: Bounding boxes coordinates output vector
	/// * confidences: Confidence values for bounding boxes output vector
	#[inline]
	fn detect(&mut self, img: &core::Mat, bboxes: &mut core::Vector<core::Rect>, confidences: &mut core::Vector<f64>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xobjdetect_WBDetector_detect_const_MatR_vector_Rect_R_vector_double_R(self.as_raw_mut_WBDetector(), img.as_raw_Mat(), bboxes.as_raw_mut_VectorOfRect(), confidences.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn WBDetector + '_ {
	/// Create instance of WBDetector
	#[inline]
	pub fn create() -> Result<core::Ptr<dyn crate::xobjdetect::WBDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xobjdetect_WBDetector_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::xobjdetect::WBDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}