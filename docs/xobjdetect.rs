pub mod xobjdetect {
	//! # Extended object detection
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::WBDetectorTraitConst, super::WBDetectorTrait };
	}
	
	/// Constant methods for [crate::xobjdetect::WBDetector]
	pub trait WBDetectorTraitConst {
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
	
	/// Mutable methods for [crate::xobjdetect::WBDetector]
	pub trait WBDetectorTrait: crate::xobjdetect::WBDetectorTraitConst {
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
			unsafe { sys::cv_xobjdetect_WBDetector_detect_const_MatR_vectorLRectGR_vectorLdoubleGR(self.as_raw_mut_WBDetector(), img.as_raw_Mat(), bboxes.as_raw_mut_VectorOfRect(), confidences.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// WaldBoost detector
	pub struct WBDetector {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { WBDetector }
	
	impl Drop for WBDetector {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_xobjdetect_WBDetector_delete(self.as_raw_mut_WBDetector()) };
		}
	}
	
	unsafe impl Send for WBDetector {}
	
	impl crate::xobjdetect::WBDetectorTraitConst for WBDetector {
		#[inline] fn as_raw_WBDetector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::xobjdetect::WBDetectorTrait for WBDetector {
		#[inline] fn as_raw_mut_WBDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl WBDetector {
		/// Create instance of WBDetector
		#[inline]
		pub fn create() -> Result<core::Ptr<crate::xobjdetect::WBDetector>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_xobjdetect_WBDetector_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::xobjdetect::WBDetector>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl std::fmt::Debug for WBDetector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("WBDetector")
				.finish()
		}
	}
}
