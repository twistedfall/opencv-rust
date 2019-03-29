//! <script type="text/javascript" src="http://latex.codecogs.com/latexit.js"></script>
//! Super Resolution
//! 
//! # Super Resolution
//! 
//! The Super Resolution module contains a set of functions and classes that can be used to solve the
//! problem of resolution enhancement. There are a few methods implemented, most of them are described in
//! the papers @cite Farsiu03 and @cite Mitzel09 .

use libc::{c_void, c_char, size_t};
use std::ffi::{CStr, CString};
use crate::{core, sys, types};
use crate::{Error, Result};

///
/// ## C++ default parameters:
/// * device_id: 0
pub fn create_frame_source__camera(device_id: i32) -> Result<types::PtrOfFrameSource> {
// identifier: cv_superres_createFrameSource_Camera_int_deviceId
  unsafe {
    let rv = sys::cv_superres_cv_superres_createFrameSource_Camera_int_deviceId(device_id);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(types::PtrOfFrameSource { ptr: rv.result })
    }
  }
}

pub fn create_frame_source__empty() -> Result<types::PtrOfFrameSource> {
// identifier: cv_superres_createFrameSource_Empty
  unsafe {
    let rv = sys::cv_superres_cv_superres_createFrameSource_Empty();
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(types::PtrOfFrameSource { ptr: rv.result })
    }
  }
}

pub fn create_frame_source__video_cuda(file_name:&str) -> Result<types::PtrOfFrameSource> {
// identifier: cv_superres_createFrameSource_Video_CUDA_String_fileName
  unsafe {
    let file_name = CString::new(file_name).unwrap();
    let rv = sys::cv_superres_cv_superres_createFrameSource_Video_CUDA_String_fileName(file_name.as_ptr() as _);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(types::PtrOfFrameSource { ptr: rv.result })
    }
  }
}

pub fn create_frame_source__video(file_name:&str) -> Result<types::PtrOfFrameSource> {
// identifier: cv_superres_createFrameSource_Video_String_fileName
  unsafe {
    let file_name = CString::new(file_name).unwrap();
    let rv = sys::cv_superres_cv_superres_createFrameSource_Video_String_fileName(file_name.as_ptr() as _);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(types::PtrOfFrameSource { ptr: rv.result })
    }
  }
}

/// Create Bilateral TV-L1 Super Resolution.
/// 
/// This class implements Super Resolution algorithm described in the papers @cite Farsiu03 and
/// @cite Mitzel09 .
/// 
/// Here are important members of the class that control the algorithm, which you can set after
/// constructing the class instance:
/// 
/// *   **int scale** Scale factor.
/// *   **int iterations** Iteration count.
/// *   **double tau** Asymptotic value of steepest descent method.
/// *   **double lambda** Weight parameter to balance data term and smoothness term.
/// *   **double alpha** Parameter of spacial distribution in Bilateral-TV.
/// *   **int btvKernelSize** Kernel size of Bilateral-TV filter.
/// *   **int blurKernelSize** Gaussian blur kernel size.
/// *   **double blurSigma** Gaussian blur sigma.
/// *   **int temporalAreaRadius** Radius of the temporal search area.
/// *   **Ptr\<DenseOpticalFlowExt\> opticalFlow** Dense optical flow algorithm.
pub fn create_super_resolution_btvl1() -> Result<types::PtrOfSuperResolution> {
// identifier: cv_superres_createSuperResolution_BTVL1
  unsafe {
    let rv = sys::cv_superres_cv_superres_createSuperResolution_BTVL1();
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(types::PtrOfSuperResolution { ptr: rv.result })
    }
  }
}

pub fn create_super_resolution_btvl1_cuda() -> Result<types::PtrOfSuperResolution> {
// identifier: cv_superres_createSuperResolution_BTVL1_CUDA
  unsafe {
    let rv = sys::cv_superres_cv_superres_createSuperResolution_BTVL1_CUDA();
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(types::PtrOfSuperResolution { ptr: rv.result })
    }
  }
}

// Generating impl for trait cv::superres::FrameSource (trait)
/// Super Resolution
/// 
/// # Super Resolution
/// 
/// The Super Resolution module contains a set of functions and classes that can be used to solve the
/// problem of resolution enhancement. There are a few methods implemented, most of them are described in
/// the papers @cite Farsiu03 and @cite Mitzel09 .
pub trait FrameSource {
  #[doc(hidden)] fn as_raw_FrameSource(&self) -> *mut c_void;
  fn next_frame(&mut self, frame: &mut core::Mat) -> Result<()> {
  // identifier: cv_superres_FrameSource_nextFrame_Mat_frame
    unsafe {
      let rv = sys::cv_superres_cv_superres_FrameSource_nextFrame_Mat_frame(self.as_raw_FrameSource(), frame.as_raw_Mat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn reset(&mut self) -> Result<()> {
  // identifier: cv_superres_FrameSource_reset
    unsafe {
      let rv = sys::cv_superres_cv_superres_FrameSource_reset(self.as_raw_FrameSource());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

}
impl<'a> FrameSource + 'a {

}

// Generating impl for trait cv::superres::SuperResolution (trait)
/// Base class for Super Resolution algorithms.
/// 
/// The class is only used to define the common interface for the whole family of Super Resolution
/// algorithms.
pub trait SuperResolution : core::Algorithm + super::superres::FrameSource {
  #[doc(hidden)] fn as_raw_SuperResolution(&self) -> *mut c_void;
  /// Set input frame source for Super Resolution algorithm.
  /// 
  /// ## Parameters
  /// * frameSource: Input frame source
  fn set_input(&mut self, frame_source: &types::PtrOfFrameSource) -> Result<()> {
  // identifier: cv_superres_SuperResolution_setInput_PtrOfFrameSource_frameSource
    unsafe {
      let rv = sys::cv_superres_cv_superres_SuperResolution_setInput_PtrOfFrameSource_frameSource(self.as_raw_SuperResolution(), frame_source.as_raw_PtrOfFrameSource());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Process next frame from input and return output result.
  /// 
  /// ## Parameters
  /// * frame: Output result
  fn next_frame(&mut self, frame: &mut core::Mat) -> Result<()> {
  // identifier: cv_superres_SuperResolution_nextFrame_Mat_frame
    unsafe {
      let rv = sys::cv_superres_cv_superres_SuperResolution_nextFrame_Mat_frame(self.as_raw_SuperResolution(), frame.as_raw_Mat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn reset(&mut self) -> Result<()> {
  // identifier: cv_superres_SuperResolution_reset
    unsafe {
      let rv = sys::cv_superres_cv_superres_SuperResolution_reset(self.as_raw_SuperResolution());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Clear all inner buffers.
  fn collect_garbage(&mut self) -> Result<()> {
  // identifier: cv_superres_SuperResolution_collectGarbage
    unsafe {
      let rv = sys::cv_superres_cv_superres_SuperResolution_collectGarbage(self.as_raw_SuperResolution());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// @see setScale
  fn get_scale(&self) -> Result<i32> {
  // identifier: cv_superres_SuperResolution_getScale
    unsafe {
      let rv = sys::cv_superres_cv_superres_SuperResolution_getScale(self.as_raw_SuperResolution());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// @copybrief getScale @see getScale
  fn set_scale(&mut self, val: i32) -> Result<()> {
  // identifier: cv_superres_SuperResolution_setScale_int_val
    unsafe {
      let rv = sys::cv_superres_cv_superres_SuperResolution_setScale_int_val(self.as_raw_SuperResolution(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// @see setIterations
  fn get_iterations(&self) -> Result<i32> {
  // identifier: cv_superres_SuperResolution_getIterations
    unsafe {
      let rv = sys::cv_superres_cv_superres_SuperResolution_getIterations(self.as_raw_SuperResolution());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// @copybrief getIterations @see getIterations
  fn set_iterations(&mut self, val: i32) -> Result<()> {
  // identifier: cv_superres_SuperResolution_setIterations_int_val
    unsafe {
      let rv = sys::cv_superres_cv_superres_SuperResolution_setIterations_int_val(self.as_raw_SuperResolution(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// @see setTau
  fn get_tau(&self) -> Result<f64> {
  // identifier: cv_superres_SuperResolution_getTau
    unsafe {
      let rv = sys::cv_superres_cv_superres_SuperResolution_getTau(self.as_raw_SuperResolution());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// @copybrief getTau @see getTau
  fn set_tau(&mut self, val: f64) -> Result<()> {
  // identifier: cv_superres_SuperResolution_setTau_double_val
    unsafe {
      let rv = sys::cv_superres_cv_superres_SuperResolution_setTau_double_val(self.as_raw_SuperResolution(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// @see setLabmda
  fn get_labmda(&self) -> Result<f64> {
  // identifier: cv_superres_SuperResolution_getLabmda
    unsafe {
      let rv = sys::cv_superres_cv_superres_SuperResolution_getLabmda(self.as_raw_SuperResolution());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// @copybrief getLabmda @see getLabmda
  fn set_labmda(&mut self, val: f64) -> Result<()> {
  // identifier: cv_superres_SuperResolution_setLabmda_double_val
    unsafe {
      let rv = sys::cv_superres_cv_superres_SuperResolution_setLabmda_double_val(self.as_raw_SuperResolution(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// @see setAlpha
  fn get_alpha(&self) -> Result<f64> {
  // identifier: cv_superres_SuperResolution_getAlpha
    unsafe {
      let rv = sys::cv_superres_cv_superres_SuperResolution_getAlpha(self.as_raw_SuperResolution());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// @copybrief getAlpha @see getAlpha
  fn set_alpha(&mut self, val: f64) -> Result<()> {
  // identifier: cv_superres_SuperResolution_setAlpha_double_val
    unsafe {
      let rv = sys::cv_superres_cv_superres_SuperResolution_setAlpha_double_val(self.as_raw_SuperResolution(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// @see setKernelSize
  fn get_kernel_size(&self) -> Result<i32> {
  // identifier: cv_superres_SuperResolution_getKernelSize
    unsafe {
      let rv = sys::cv_superres_cv_superres_SuperResolution_getKernelSize(self.as_raw_SuperResolution());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// @copybrief getKernelSize @see getKernelSize
  fn set_kernel_size(&mut self, val: i32) -> Result<()> {
  // identifier: cv_superres_SuperResolution_setKernelSize_int_val
    unsafe {
      let rv = sys::cv_superres_cv_superres_SuperResolution_setKernelSize_int_val(self.as_raw_SuperResolution(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// @see setBlurKernelSize
  fn get_blur_kernel_size(&self) -> Result<i32> {
  // identifier: cv_superres_SuperResolution_getBlurKernelSize
    unsafe {
      let rv = sys::cv_superres_cv_superres_SuperResolution_getBlurKernelSize(self.as_raw_SuperResolution());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// @copybrief getBlurKernelSize @see getBlurKernelSize
  fn set_blur_kernel_size(&mut self, val: i32) -> Result<()> {
  // identifier: cv_superres_SuperResolution_setBlurKernelSize_int_val
    unsafe {
      let rv = sys::cv_superres_cv_superres_SuperResolution_setBlurKernelSize_int_val(self.as_raw_SuperResolution(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// @see setBlurSigma
  fn get_blur_sigma(&self) -> Result<f64> {
  // identifier: cv_superres_SuperResolution_getBlurSigma
    unsafe {
      let rv = sys::cv_superres_cv_superres_SuperResolution_getBlurSigma(self.as_raw_SuperResolution());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// @copybrief getBlurSigma @see getBlurSigma
  fn set_blur_sigma(&mut self, val: f64) -> Result<()> {
  // identifier: cv_superres_SuperResolution_setBlurSigma_double_val
    unsafe {
      let rv = sys::cv_superres_cv_superres_SuperResolution_setBlurSigma_double_val(self.as_raw_SuperResolution(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// @see setTemporalAreaRadius
  fn get_temporal_area_radius(&self) -> Result<i32> {
  // identifier: cv_superres_SuperResolution_getTemporalAreaRadius
    unsafe {
      let rv = sys::cv_superres_cv_superres_SuperResolution_getTemporalAreaRadius(self.as_raw_SuperResolution());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// @copybrief getTemporalAreaRadius @see getTemporalAreaRadius
  fn set_temporal_area_radius(&mut self, val: i32) -> Result<()> {
  // identifier: cv_superres_SuperResolution_setTemporalAreaRadius_int_val
    unsafe {
      let rv = sys::cv_superres_cv_superres_SuperResolution_setTemporalAreaRadius_int_val(self.as_raw_SuperResolution(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

}
impl<'a> SuperResolution + 'a {

}

