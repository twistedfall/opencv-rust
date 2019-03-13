//! <script type="text/javascript" src="http://latex.codecogs.com/latexit.js"></script>

use libc::{c_void, c_char, size_t};
use std::ffi::{CStr, CString};
use crate::{core, sys, types};
use crate::{Error, Result};
pub const MM_AFFINE: i32 = 5;
pub const MM_HOMOGRAPHY: i32 = 6;
pub const MM_RIGID: i32 = 3;
pub const MM_ROTATION: i32 = 2;
pub const MM_SIMILARITY: i32 = 4;
pub const MM_TRANSLATION: i32 = 0;
pub const MM_TRANSLATION_AND_SCALE: i32 = 1;
pub const MM_UNKNOWN: i32 = 7;

pub fn calc_blurriness(frame: &core::Mat) -> Result<f32> {
// identifier: cv_videostab_calcBlurriness_Mat_frame
  unsafe {
    let rv = sys::cv_videostab_cv_videostab_calcBlurriness_Mat_frame(frame.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

pub fn calc_flow_mask(flow_x: &core::Mat, flow_y: &core::Mat, errors: &core::Mat, max_error: f32, mask0: &core::Mat, mask1: &core::Mat, flow_mask: &core::Mat) -> Result<()> {
// identifier: cv_videostab_calcFlowMask_Mat_flowX_Mat_flowY_Mat_errors_float_maxError_Mat_mask0_Mat_mask1_Mat_flowMask
  unsafe {
    let rv = sys::cv_videostab_cv_videostab_calcFlowMask_Mat_flowX_Mat_flowY_Mat_errors_float_maxError_Mat_mask0_Mat_mask1_Mat_flowMask(flow_x.as_raw_Mat(), flow_y.as_raw_Mat(), errors.as_raw_Mat(), max_error, mask0.as_raw_Mat(), mask1.as_raw_Mat(), flow_mask.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

pub fn complete_frame_according_to_flow(flow_mask: &core::Mat, flow_x: &core::Mat, flow_y: &core::Mat, frame1: &core::Mat, mask1: &core::Mat, dist_thresh: f32, frame0: &core::Mat, mask0: &core::Mat) -> Result<()> {
// identifier: cv_videostab_completeFrameAccordingToFlow_Mat_flowMask_Mat_flowX_Mat_flowY_Mat_frame1_Mat_mask1_float_distThresh_Mat_frame0_Mat_mask0
  unsafe {
    let rv = sys::cv_videostab_cv_videostab_completeFrameAccordingToFlow_Mat_flowMask_Mat_flowX_Mat_flowY_Mat_frame1_Mat_mask1_float_distThresh_Mat_frame0_Mat_mask0(flow_mask.as_raw_Mat(), flow_x.as_raw_Mat(), flow_y.as_raw_Mat(), frame1.as_raw_Mat(), mask1.as_raw_Mat(), dist_thresh, frame0.as_raw_Mat(), mask0.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

pub fn ensure_inclusion_constraint(m: &core::Mat, size: core::Size, trim_ratio: f32) -> Result<core::Mat> {
// identifier: cv_videostab_ensureInclusionConstraint_Mat_M_Size_size_float_trimRatio
  unsafe {
    let rv = sys::cv_videostab_cv_videostab_ensureInclusionConstraint_Mat_M_Size_size_float_trimRatio(m.as_raw_Mat(), size, trim_ratio);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(core::Mat { ptr: rv.result })
    }
  }
}

pub fn estimate_optimal_trim_ratio(m: &core::Mat, size: core::Size) -> Result<f32> {
// identifier: cv_videostab_estimateOptimalTrimRatio_Mat_M_Size_size
  unsafe {
    let rv = sys::cv_videostab_cv_videostab_estimateOptimalTrimRatio_Mat_M_Size_size(m.as_raw_Mat(), size);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Computes motion between two frames assuming that all the intermediate motions are known.
/// 
/// ## Parameters
/// * from: Source frame index.
/// * to: Destination frame index.
/// * motions: Pair-wise motions. motions[i] denotes motion from the frame i to the frame i+1
/// @return Motion from the Source frame to the Destination frame.
pub fn get_motion(from: i32, to: i32, motions: &types::VectorOfMat) -> Result<core::Mat> {
// identifier: cv_videostab_getMotion_int_from_int_to_VectorOfMat_motions
  unsafe {
    let rv = sys::cv_videostab_cv_videostab_getMotion_int_from_int_to_VectorOfMat_motions(from, to, motions.as_raw_VectorOfMat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(core::Mat { ptr: rv.result })
    }
  }
}

// boxed class cv::videostab::ColorAverageInpainter

#[allow(dead_code)]
pub struct ColorAverageInpainter {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::ColorAverageInpainter {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_ColorAverageInpainter(self.ptr) };
    }
}
impl super::videostab::ColorAverageInpainter {
    #[doc(hidden)] pub fn as_raw_ColorAverageInpainter(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::InpainterBase for ColorAverageInpainter {
    #[doc(hidden)] fn as_raw_InpainterBase(&self) -> *mut c_void { self.ptr }
}
impl ColorAverageInpainter {

  pub fn inpaint(&mut self, idx: i32, frame: &core::Mat, mask: &core::Mat) -> Result<()> {
  // identifier: cv_videostab_ColorAverageInpainter_inpaint_int_idx_Mat_frame_Mat_mask
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_ColorAverageInpainter_inpaint_int_idx_Mat_frame_Mat_mask(self.as_raw_ColorAverageInpainter(), idx, frame.as_raw_Mat(), mask.as_raw_Mat());
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
// boxed class cv::videostab::ColorInpainter

#[allow(dead_code)]
pub struct ColorInpainter {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::ColorInpainter {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_ColorInpainter(self.ptr) };
    }
}
impl super::videostab::ColorInpainter {
    #[doc(hidden)] pub fn as_raw_ColorInpainter(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::InpainterBase for ColorInpainter {
    #[doc(hidden)] fn as_raw_InpainterBase(&self) -> *mut c_void { self.ptr }
}
impl ColorInpainter {

  ///
  /// ## C++ default parameters:
  /// * method: INPAINT_TELEA
  /// * radius: 2.
  pub fn new(method: i32, radius: f64) -> Result<super::videostab::ColorInpainter> {
  // identifier: cv_videostab_ColorInpainter_ColorInpainter_int_method_double_radius
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_ColorInpainter_ColorInpainter_int_method_double_radius(method, radius);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(super::videostab::ColorInpainter { ptr: rv.result })
      }
    }
  }

  pub fn inpaint(&mut self, idx: i32, frame: &core::Mat, mask: &core::Mat) -> Result<()> {
  // identifier: cv_videostab_ColorInpainter_inpaint_int_idx_Mat_frame_Mat_mask
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_ColorInpainter_inpaint_int_idx_Mat_frame_Mat_mask(self.as_raw_ColorInpainter(), idx, frame.as_raw_Mat(), mask.as_raw_Mat());
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
// boxed class cv::videostab::ConsistentMosaicInpainter

#[allow(dead_code)]
pub struct ConsistentMosaicInpainter {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::ConsistentMosaicInpainter {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_ConsistentMosaicInpainter(self.ptr) };
    }
}
impl super::videostab::ConsistentMosaicInpainter {
    #[doc(hidden)] pub fn as_raw_ConsistentMosaicInpainter(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::InpainterBase for ConsistentMosaicInpainter {
    #[doc(hidden)] fn as_raw_InpainterBase(&self) -> *mut c_void { self.ptr }
}
impl ConsistentMosaicInpainter {

  pub fn new() -> Result<super::videostab::ConsistentMosaicInpainter> {
  // identifier: cv_videostab_ConsistentMosaicInpainter_ConsistentMosaicInpainter
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_ConsistentMosaicInpainter_ConsistentMosaicInpainter();
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(super::videostab::ConsistentMosaicInpainter { ptr: rv.result })
      }
    }
  }

  pub fn set_stdev_thresh(&mut self, val: f32) -> Result<()> {
  // identifier: cv_videostab_ConsistentMosaicInpainter_setStdevThresh_float_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_ConsistentMosaicInpainter_setStdevThresh_float_val(self.as_raw_ConsistentMosaicInpainter(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn stdev_thresh(&self) -> Result<f32> {
  // identifier: cv_videostab_ConsistentMosaicInpainter_stdevThresh
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_ConsistentMosaicInpainter_stdevThresh(self.as_raw_ConsistentMosaicInpainter());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn inpaint(&mut self, idx: i32, frame: &core::Mat, mask: &core::Mat) -> Result<()> {
  // identifier: cv_videostab_ConsistentMosaicInpainter_inpaint_int_idx_Mat_frame_Mat_mask
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_ConsistentMosaicInpainter_inpaint_int_idx_Mat_frame_Mat_mask(self.as_raw_ConsistentMosaicInpainter(), idx, frame.as_raw_Mat(), mask.as_raw_Mat());
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
// Generating impl for trait cv::videostab::DeblurerBase (trait)
pub trait DeblurerBase {
  #[doc(hidden)] fn as_raw_DeblurerBase(&self) -> *mut c_void;
  fn set_radius(&mut self, val: i32) -> Result<()> {
  // identifier: cv_videostab_DeblurerBase_setRadius_int_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_DeblurerBase_setRadius_int_val(self.as_raw_DeblurerBase(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn radius(&self) -> Result<i32> {
  // identifier: cv_videostab_DeblurerBase_radius
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_DeblurerBase_radius(self.as_raw_DeblurerBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn deblur(&mut self, idx: i32, frame: &core::Mat) -> Result<()> {
  // identifier: cv_videostab_DeblurerBase_deblur_int_idx_Mat_frame
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_DeblurerBase_deblur_int_idx_Mat_frame(self.as_raw_DeblurerBase(), idx, frame.as_raw_Mat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn set_frames(&mut self, val: &types::VectorOfMat) -> Result<()> {
  // identifier: cv_videostab_DeblurerBase_setFrames_VectorOfMat_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_DeblurerBase_setFrames_VectorOfMat_val(self.as_raw_DeblurerBase(), val.as_raw_VectorOfMat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn frames(&self) -> Result<types::VectorOfMat> {
  // identifier: cv_videostab_DeblurerBase_frames
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_DeblurerBase_frames(self.as_raw_DeblurerBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::VectorOfMat { ptr: rv.result })
      }
    }
  }

  fn set_motions(&mut self, val: &types::VectorOfMat) -> Result<()> {
  // identifier: cv_videostab_DeblurerBase_setMotions_VectorOfMat_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_DeblurerBase_setMotions_VectorOfMat_val(self.as_raw_DeblurerBase(), val.as_raw_VectorOfMat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn motions(&self) -> Result<types::VectorOfMat> {
  // identifier: cv_videostab_DeblurerBase_motions
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_DeblurerBase_motions(self.as_raw_DeblurerBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::VectorOfMat { ptr: rv.result })
      }
    }
  }

  fn set_blurriness_rates(&mut self, val: &types::VectorOffloat) -> Result<()> {
  // identifier: cv_videostab_DeblurerBase_setBlurrinessRates_VectorOffloat_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_DeblurerBase_setBlurrinessRates_VectorOffloat_val(self.as_raw_DeblurerBase(), val.as_raw_VectorOffloat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn blurriness_rates(&self) -> Result<types::VectorOffloat> {
  // identifier: cv_videostab_DeblurerBase_blurrinessRates
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_DeblurerBase_blurrinessRates(self.as_raw_DeblurerBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::VectorOffloat { ptr: rv.result })
      }
    }
  }

}
impl<'a> DeblurerBase + 'a {

}

// boxed class cv::videostab::FastMarchingMethod
/// Describes the Fast Marching Method implementation.
/// 
/// See http://iwi.eldoc.ub.rug.nl/FILES/root/2004/JGraphToolsTelea/2004JGraphToolsTelea.pdf

#[allow(dead_code)]
pub struct FastMarchingMethod {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::FastMarchingMethod {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_FastMarchingMethod(self.ptr) };
    }
}
impl super::videostab::FastMarchingMethod {
    #[doc(hidden)] pub fn as_raw_FastMarchingMethod(&self) -> *mut c_void { self.ptr }
}
impl FastMarchingMethod {

  /// @return Distance map that's created during working of the method.
  pub fn distance_map(&self) -> Result<core::Mat> {
  // identifier: cv_videostab_FastMarchingMethod_distanceMap
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_FastMarchingMethod_distanceMap(self.as_raw_FastMarchingMethod());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(core::Mat { ptr: rv.result })
      }
    }
  }

}
// boxed class cv::videostab::FromFileMotionReader

#[allow(dead_code)]
pub struct FromFileMotionReader {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::FromFileMotionReader {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_FromFileMotionReader(self.ptr) };
    }
}
impl super::videostab::FromFileMotionReader {
    #[doc(hidden)] pub fn as_raw_FromFileMotionReader(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::ImageMotionEstimatorBase for FromFileMotionReader {
    #[doc(hidden)] fn as_raw_ImageMotionEstimatorBase(&self) -> *mut c_void { self.ptr }
}
impl FromFileMotionReader {

  pub fn new(path:&str) -> Result<super::videostab::FromFileMotionReader> {
  // identifier: cv_videostab_FromFileMotionReader_FromFileMotionReader_String_path
    unsafe {
      let path = CString::new(path).unwrap();
      let rv = sys::cv_videostab_cv_videostab_FromFileMotionReader_FromFileMotionReader_String_path(path.as_ptr() as _);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(super::videostab::FromFileMotionReader { ptr: rv.result })
      }
    }
  }

}
// boxed class cv::videostab::GaussianMotionFilter

#[allow(dead_code)]
pub struct GaussianMotionFilter {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::GaussianMotionFilter {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_GaussianMotionFilter(self.ptr) };
    }
}
impl super::videostab::GaussianMotionFilter {
    #[doc(hidden)] pub fn as_raw_GaussianMotionFilter(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::IMotionStabilizer for GaussianMotionFilter {
    #[doc(hidden)] fn as_raw_IMotionStabilizer(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::MotionFilterBase for GaussianMotionFilter {
    #[doc(hidden)] fn as_raw_MotionFilterBase(&self) -> *mut c_void { self.ptr }
}
impl GaussianMotionFilter {

  ///
  /// ## C++ default parameters:
  /// * radius: 15
  /// * stdev: -1.f
  pub fn new(radius: i32, stdev: f32) -> Result<super::videostab::GaussianMotionFilter> {
  // identifier: cv_videostab_GaussianMotionFilter_GaussianMotionFilter_int_radius_float_stdev
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_GaussianMotionFilter_GaussianMotionFilter_int_radius_float_stdev(radius, stdev);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(super::videostab::GaussianMotionFilter { ptr: rv.result })
      }
    }
  }

  ///
  /// ## C++ default parameters:
  /// * stdev: -1.f
  pub fn set_params(&mut self, radius: i32, stdev: f32) -> Result<()> {
  // identifier: cv_videostab_GaussianMotionFilter_setParams_int_radius_float_stdev
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_GaussianMotionFilter_setParams_int_radius_float_stdev(self.as_raw_GaussianMotionFilter(), radius, stdev);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn radius(&self) -> Result<i32> {
  // identifier: cv_videostab_GaussianMotionFilter_radius
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_GaussianMotionFilter_radius(self.as_raw_GaussianMotionFilter());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn stdev(&self) -> Result<f32> {
  // identifier: cv_videostab_GaussianMotionFilter_stdev
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_GaussianMotionFilter_stdev(self.as_raw_GaussianMotionFilter());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn new_v0(_radius: i32, _stdev: f32) -> Result<super::videostab::GaussianMotionFilter> {
  // identifier: cv_videostab_GaussianMotionFilter_GaussianMotionFilter_int__radius_float__stdev
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_GaussianMotionFilter_GaussianMotionFilter_int__radius_float__stdev(_radius, _stdev);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(super::videostab::GaussianMotionFilter { ptr: rv.result })
      }
    }
  }

}
// Generating impl for trait cv::videostab::IDenseOptFlowEstimator (trait)
pub trait IDenseOptFlowEstimator {
  #[doc(hidden)] fn as_raw_IDenseOptFlowEstimator(&self) -> *mut c_void;
  fn run(&mut self, frame0: &core::Mat, frame1: &core::Mat, flow_x: &mut core::Mat, flow_y: &mut core::Mat, errors: &mut core::Mat) -> Result<()> {
  // identifier: cv_videostab_IDenseOptFlowEstimator_run_Mat_frame0_Mat_frame1_Mat_flowX_Mat_flowY_Mat_errors
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_IDenseOptFlowEstimator_run_Mat_frame0_Mat_frame1_Mat_flowX_Mat_flowY_Mat_errors(self.as_raw_IDenseOptFlowEstimator(), frame0.as_raw_Mat(), frame1.as_raw_Mat(), flow_x.as_raw_Mat(), flow_y.as_raw_Mat(), errors.as_raw_Mat());
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
impl<'a> IDenseOptFlowEstimator + 'a {

}

// Generating impl for trait cv::videostab::IFrameSource (trait)
pub trait IFrameSource {
  #[doc(hidden)] fn as_raw_IFrameSource(&self) -> *mut c_void;
  fn reset(&mut self) -> Result<()> {
  // identifier: cv_videostab_IFrameSource_reset
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_IFrameSource_reset(self.as_raw_IFrameSource());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn next_frame(&mut self) -> Result<core::Mat> {
  // identifier: cv_videostab_IFrameSource_nextFrame
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_IFrameSource_nextFrame(self.as_raw_IFrameSource());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(core::Mat { ptr: rv.result })
      }
    }
  }

}
impl<'a> IFrameSource + 'a {

}

// Generating impl for trait cv::videostab::ILog (trait)
pub trait ILog {
  #[doc(hidden)] fn as_raw_ILog(&self) -> *mut c_void;
}
impl<'a> ILog + 'a {

}

// Generating impl for trait cv::videostab::IMotionStabilizer (trait)
pub trait IMotionStabilizer {
  #[doc(hidden)] fn as_raw_IMotionStabilizer(&self) -> *mut c_void;
}
impl<'a> IMotionStabilizer + 'a {

}

// Generating impl for trait cv::videostab::IOutlierRejector (trait)
pub trait IOutlierRejector {
  #[doc(hidden)] fn as_raw_IOutlierRejector(&self) -> *mut c_void;
  fn process(&mut self, frame_size: core::Size, points0: &core::Mat, points1: &core::Mat, mask: &mut core::Mat) -> Result<()> {
  // identifier: cv_videostab_IOutlierRejector_process_Size_frameSize_Mat_points0_Mat_points1_Mat_mask
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_IOutlierRejector_process_Size_frameSize_Mat_points0_Mat_points1_Mat_mask(self.as_raw_IOutlierRejector(), frame_size, points0.as_raw_Mat(), points1.as_raw_Mat(), mask.as_raw_Mat());
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
impl<'a> IOutlierRejector + 'a {

}

// Generating impl for trait cv::videostab::ISparseOptFlowEstimator (trait)
pub trait ISparseOptFlowEstimator {
  #[doc(hidden)] fn as_raw_ISparseOptFlowEstimator(&self) -> *mut c_void;
  fn run(&mut self, frame0: &core::Mat, frame1: &core::Mat, points0: &core::Mat, points1: &mut core::Mat, status: &mut core::Mat, errors: &mut core::Mat) -> Result<()> {
  // identifier: cv_videostab_ISparseOptFlowEstimator_run_Mat_frame0_Mat_frame1_Mat_points0_Mat_points1_Mat_status_Mat_errors
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_ISparseOptFlowEstimator_run_Mat_frame0_Mat_frame1_Mat_points0_Mat_points1_Mat_status_Mat_errors(self.as_raw_ISparseOptFlowEstimator(), frame0.as_raw_Mat(), frame1.as_raw_Mat(), points0.as_raw_Mat(), points1.as_raw_Mat(), status.as_raw_Mat(), errors.as_raw_Mat());
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
impl<'a> ISparseOptFlowEstimator + 'a {

}

// Generating impl for trait cv::videostab::ImageMotionEstimatorBase (trait)
/// Base class for global 2D motion estimation methods which take frames as input.
pub trait ImageMotionEstimatorBase {
  #[doc(hidden)] fn as_raw_ImageMotionEstimatorBase(&self) -> *mut c_void;
}
impl<'a> ImageMotionEstimatorBase + 'a {

}

// Generating impl for trait cv::videostab::InpainterBase (trait)
pub trait InpainterBase {
  #[doc(hidden)] fn as_raw_InpainterBase(&self) -> *mut c_void;
  fn set_radius(&mut self, val: i32) -> Result<()> {
  // identifier: cv_videostab_InpainterBase_setRadius_int_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_InpainterBase_setRadius_int_val(self.as_raw_InpainterBase(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn radius(&self) -> Result<i32> {
  // identifier: cv_videostab_InpainterBase_radius
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_InpainterBase_radius(self.as_raw_InpainterBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn inpaint(&mut self, idx: i32, frame: &core::Mat, mask: &core::Mat) -> Result<()> {
  // identifier: cv_videostab_InpainterBase_inpaint_int_idx_Mat_frame_Mat_mask
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_InpainterBase_inpaint_int_idx_Mat_frame_Mat_mask(self.as_raw_InpainterBase(), idx, frame.as_raw_Mat(), mask.as_raw_Mat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn set_frames(&mut self, val: &types::VectorOfMat) -> Result<()> {
  // identifier: cv_videostab_InpainterBase_setFrames_VectorOfMat_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_InpainterBase_setFrames_VectorOfMat_val(self.as_raw_InpainterBase(), val.as_raw_VectorOfMat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn frames(&self) -> Result<types::VectorOfMat> {
  // identifier: cv_videostab_InpainterBase_frames
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_InpainterBase_frames(self.as_raw_InpainterBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::VectorOfMat { ptr: rv.result })
      }
    }
  }

  fn set_motions(&mut self, val: &types::VectorOfMat) -> Result<()> {
  // identifier: cv_videostab_InpainterBase_setMotions_VectorOfMat_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_InpainterBase_setMotions_VectorOfMat_val(self.as_raw_InpainterBase(), val.as_raw_VectorOfMat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn motions(&self) -> Result<types::VectorOfMat> {
  // identifier: cv_videostab_InpainterBase_motions
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_InpainterBase_motions(self.as_raw_InpainterBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::VectorOfMat { ptr: rv.result })
      }
    }
  }

  fn set_stabilized_frames(&mut self, val: &types::VectorOfMat) -> Result<()> {
  // identifier: cv_videostab_InpainterBase_setStabilizedFrames_VectorOfMat_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_InpainterBase_setStabilizedFrames_VectorOfMat_val(self.as_raw_InpainterBase(), val.as_raw_VectorOfMat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn stabilized_frames(&self) -> Result<types::VectorOfMat> {
  // identifier: cv_videostab_InpainterBase_stabilizedFrames
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_InpainterBase_stabilizedFrames(self.as_raw_InpainterBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::VectorOfMat { ptr: rv.result })
      }
    }
  }

  fn set_stabilization_motions(&mut self, val: &types::VectorOfMat) -> Result<()> {
  // identifier: cv_videostab_InpainterBase_setStabilizationMotions_VectorOfMat_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_InpainterBase_setStabilizationMotions_VectorOfMat_val(self.as_raw_InpainterBase(), val.as_raw_VectorOfMat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn stabilization_motions(&self) -> Result<types::VectorOfMat> {
  // identifier: cv_videostab_InpainterBase_stabilizationMotions
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_InpainterBase_stabilizationMotions(self.as_raw_InpainterBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::VectorOfMat { ptr: rv.result })
      }
    }
  }

}
impl<'a> InpainterBase + 'a {

}

// boxed class cv::videostab::InpaintingPipeline

#[allow(dead_code)]
pub struct InpaintingPipeline {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::InpaintingPipeline {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_InpaintingPipeline(self.ptr) };
    }
}
impl super::videostab::InpaintingPipeline {
    #[doc(hidden)] pub fn as_raw_InpaintingPipeline(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::InpainterBase for InpaintingPipeline {
    #[doc(hidden)] fn as_raw_InpainterBase(&self) -> *mut c_void { self.ptr }
}
impl InpaintingPipeline {

  pub fn push_back(&mut self, inpainter: &types::PtrOfInpainterBase) -> Result<()> {
  // identifier: cv_videostab_InpaintingPipeline_pushBack_PtrOfInpainterBase_inpainter
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_InpaintingPipeline_pushBack_PtrOfInpainterBase_inpainter(self.as_raw_InpaintingPipeline(), inpainter.as_raw_PtrOfInpainterBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn empty(&self) -> Result<bool> {
  // identifier: cv_videostab_InpaintingPipeline_empty
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_InpaintingPipeline_empty(self.as_raw_InpaintingPipeline());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn set_radius(&mut self, val: i32) -> Result<()> {
  // identifier: cv_videostab_InpaintingPipeline_setRadius_int_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_InpaintingPipeline_setRadius_int_val(self.as_raw_InpaintingPipeline(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn set_frames(&mut self, val: &types::VectorOfMat) -> Result<()> {
  // identifier: cv_videostab_InpaintingPipeline_setFrames_VectorOfMat_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_InpaintingPipeline_setFrames_VectorOfMat_val(self.as_raw_InpaintingPipeline(), val.as_raw_VectorOfMat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn set_motions(&mut self, val: &types::VectorOfMat) -> Result<()> {
  // identifier: cv_videostab_InpaintingPipeline_setMotions_VectorOfMat_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_InpaintingPipeline_setMotions_VectorOfMat_val(self.as_raw_InpaintingPipeline(), val.as_raw_VectorOfMat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn set_stabilized_frames(&mut self, val: &types::VectorOfMat) -> Result<()> {
  // identifier: cv_videostab_InpaintingPipeline_setStabilizedFrames_VectorOfMat_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_InpaintingPipeline_setStabilizedFrames_VectorOfMat_val(self.as_raw_InpaintingPipeline(), val.as_raw_VectorOfMat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn set_stabilization_motions(&mut self, val: &types::VectorOfMat) -> Result<()> {
  // identifier: cv_videostab_InpaintingPipeline_setStabilizationMotions_VectorOfMat_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_InpaintingPipeline_setStabilizationMotions_VectorOfMat_val(self.as_raw_InpaintingPipeline(), val.as_raw_VectorOfMat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn inpaint(&mut self, idx: i32, frame: &core::Mat, mask: &core::Mat) -> Result<()> {
  // identifier: cv_videostab_InpaintingPipeline_inpaint_int_idx_Mat_frame_Mat_mask
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_InpaintingPipeline_inpaint_int_idx_Mat_frame_Mat_mask(self.as_raw_InpaintingPipeline(), idx, frame.as_raw_Mat(), mask.as_raw_Mat());
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
// boxed class cv::videostab::KeypointBasedMotionEstimator
/// Describes a global 2D motion estimation method which uses keypoints detection and optical flow for
/// matching.

#[allow(dead_code)]
pub struct KeypointBasedMotionEstimator {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::KeypointBasedMotionEstimator {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_KeypointBasedMotionEstimator(self.ptr) };
    }
}
impl super::videostab::KeypointBasedMotionEstimator {
    #[doc(hidden)] pub fn as_raw_KeypointBasedMotionEstimator(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::ImageMotionEstimatorBase for KeypointBasedMotionEstimator {
    #[doc(hidden)] fn as_raw_ImageMotionEstimatorBase(&self) -> *mut c_void { self.ptr }
}
impl KeypointBasedMotionEstimator {

  pub fn new(estimator: &types::PtrOfMotionEstimatorBase) -> Result<super::videostab::KeypointBasedMotionEstimator> {
  // identifier: cv_videostab_KeypointBasedMotionEstimator_KeypointBasedMotionEstimator_PtrOfMotionEstimatorBase_estimator
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_KeypointBasedMotionEstimator_KeypointBasedMotionEstimator_PtrOfMotionEstimatorBase_estimator(estimator.as_raw_PtrOfMotionEstimatorBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(super::videostab::KeypointBasedMotionEstimator { ptr: rv.result })
      }
    }
  }

}
// boxed class cv::videostab::LogToStdout

#[allow(dead_code)]
pub struct LogToStdout {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::LogToStdout {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_LogToStdout(self.ptr) };
    }
}
impl super::videostab::LogToStdout {
    #[doc(hidden)] pub fn as_raw_LogToStdout(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::ILog for LogToStdout {
    #[doc(hidden)] fn as_raw_ILog(&self) -> *mut c_void { self.ptr }
}
impl LogToStdout {

}
// boxed class cv::videostab::LpMotionStabilizer

#[allow(dead_code)]
pub struct LpMotionStabilizer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::LpMotionStabilizer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_LpMotionStabilizer(self.ptr) };
    }
}
impl super::videostab::LpMotionStabilizer {
    #[doc(hidden)] pub fn as_raw_LpMotionStabilizer(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::IMotionStabilizer for LpMotionStabilizer {
    #[doc(hidden)] fn as_raw_IMotionStabilizer(&self) -> *mut c_void { self.ptr }
}
impl LpMotionStabilizer {

  pub fn set_frame_size(&mut self, val: core::Size) -> Result<()> {
  // identifier: cv_videostab_LpMotionStabilizer_setFrameSize_Size_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_LpMotionStabilizer_setFrameSize_Size_val(self.as_raw_LpMotionStabilizer(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn frame_size(&self) -> Result<core::Size> {
  // identifier: cv_videostab_LpMotionStabilizer_frameSize
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_LpMotionStabilizer_frameSize(self.as_raw_LpMotionStabilizer());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn set_trim_ratio(&mut self, val: f32) -> Result<()> {
  // identifier: cv_videostab_LpMotionStabilizer_setTrimRatio_float_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_LpMotionStabilizer_setTrimRatio_float_val(self.as_raw_LpMotionStabilizer(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn trim_ratio(&self) -> Result<f32> {
  // identifier: cv_videostab_LpMotionStabilizer_trimRatio
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_LpMotionStabilizer_trimRatio(self.as_raw_LpMotionStabilizer());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn set_weight1(&mut self, val: f32) -> Result<()> {
  // identifier: cv_videostab_LpMotionStabilizer_setWeight1_float_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_LpMotionStabilizer_setWeight1_float_val(self.as_raw_LpMotionStabilizer(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn weight1(&self) -> Result<f32> {
  // identifier: cv_videostab_LpMotionStabilizer_weight1
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_LpMotionStabilizer_weight1(self.as_raw_LpMotionStabilizer());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn set_weight2(&mut self, val: f32) -> Result<()> {
  // identifier: cv_videostab_LpMotionStabilizer_setWeight2_float_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_LpMotionStabilizer_setWeight2_float_val(self.as_raw_LpMotionStabilizer(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn weight2(&self) -> Result<f32> {
  // identifier: cv_videostab_LpMotionStabilizer_weight2
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_LpMotionStabilizer_weight2(self.as_raw_LpMotionStabilizer());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn set_weight3(&mut self, val: f32) -> Result<()> {
  // identifier: cv_videostab_LpMotionStabilizer_setWeight3_float_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_LpMotionStabilizer_setWeight3_float_val(self.as_raw_LpMotionStabilizer(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn weight3(&self) -> Result<f32> {
  // identifier: cv_videostab_LpMotionStabilizer_weight3
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_LpMotionStabilizer_weight3(self.as_raw_LpMotionStabilizer());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn set_weight4(&mut self, val: f32) -> Result<()> {
  // identifier: cv_videostab_LpMotionStabilizer_setWeight4_float_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_LpMotionStabilizer_setWeight4_float_val(self.as_raw_LpMotionStabilizer(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn weight4(&self) -> Result<f32> {
  // identifier: cv_videostab_LpMotionStabilizer_weight4
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_LpMotionStabilizer_weight4(self.as_raw_LpMotionStabilizer());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

}
// boxed class cv::videostab::MoreAccurateMotionWobbleSuppressor

#[allow(dead_code)]
pub struct MoreAccurateMotionWobbleSuppressor {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::MoreAccurateMotionWobbleSuppressor {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_MoreAccurateMotionWobbleSuppressor(self.ptr) };
    }
}
impl super::videostab::MoreAccurateMotionWobbleSuppressor {
    #[doc(hidden)] pub fn as_raw_MoreAccurateMotionWobbleSuppressor(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::WobbleSuppressorBase for MoreAccurateMotionWobbleSuppressor {
    #[doc(hidden)] fn as_raw_WobbleSuppressorBase(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::MoreAccurateMotionWobbleSuppressorBase for MoreAccurateMotionWobbleSuppressor {
    #[doc(hidden)] fn as_raw_MoreAccurateMotionWobbleSuppressorBase(&self) -> *mut c_void { self.ptr }
}
impl MoreAccurateMotionWobbleSuppressor {

  pub fn suppress(&mut self, idx: i32, frame: &core::Mat, result: &core::Mat) -> Result<()> {
  // identifier: cv_videostab_MoreAccurateMotionWobbleSuppressor_suppress_int_idx_Mat_frame_Mat_result
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_MoreAccurateMotionWobbleSuppressor_suppress_int_idx_Mat_frame_Mat_result(self.as_raw_MoreAccurateMotionWobbleSuppressor(), idx, frame.as_raw_Mat(), result.as_raw_Mat());
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
// Generating impl for trait cv::videostab::MoreAccurateMotionWobbleSuppressorBase (trait)
pub trait MoreAccurateMotionWobbleSuppressorBase : super::videostab::WobbleSuppressorBase {
  #[doc(hidden)] fn as_raw_MoreAccurateMotionWobbleSuppressorBase(&self) -> *mut c_void;
  fn set_period(&mut self, val: i32) -> Result<()> {
  // identifier: cv_videostab_MoreAccurateMotionWobbleSuppressorBase_setPeriod_int_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_MoreAccurateMotionWobbleSuppressorBase_setPeriod_int_val(self.as_raw_MoreAccurateMotionWobbleSuppressorBase(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn period(&self) -> Result<i32> {
  // identifier: cv_videostab_MoreAccurateMotionWobbleSuppressorBase_period
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_MoreAccurateMotionWobbleSuppressorBase_period(self.as_raw_MoreAccurateMotionWobbleSuppressorBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

}
impl<'a> MoreAccurateMotionWobbleSuppressorBase + 'a {

}

// Generating impl for trait cv::videostab::MotionEstimatorBase (trait)
/// Base class for all global motion estimation methods.
pub trait MotionEstimatorBase {
  #[doc(hidden)] fn as_raw_MotionEstimatorBase(&self) -> *mut c_void;
}
impl<'a> MotionEstimatorBase + 'a {

}

// boxed class cv::videostab::MotionEstimatorL1
/// Describes a global 2D motion estimation method which minimizes L1 error.
/// 
/// 
/// Note: To be able to use this method you must build OpenCV with CLP library support. :

#[allow(dead_code)]
pub struct MotionEstimatorL1 {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::MotionEstimatorL1 {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_MotionEstimatorL1(self.ptr) };
    }
}
impl super::videostab::MotionEstimatorL1 {
    #[doc(hidden)] pub fn as_raw_MotionEstimatorL1(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::MotionEstimatorBase for MotionEstimatorL1 {
    #[doc(hidden)] fn as_raw_MotionEstimatorBase(&self) -> *mut c_void { self.ptr }
}
impl MotionEstimatorL1 {

}
// boxed class cv::videostab::MotionEstimatorRansacL2
/// Describes a robust RANSAC-based global 2D motion estimation method which minimizes L2 error.

#[allow(dead_code)]
pub struct MotionEstimatorRansacL2 {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::MotionEstimatorRansacL2 {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_MotionEstimatorRansacL2(self.ptr) };
    }
}
impl super::videostab::MotionEstimatorRansacL2 {
    #[doc(hidden)] pub fn as_raw_MotionEstimatorRansacL2(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::MotionEstimatorBase for MotionEstimatorRansacL2 {
    #[doc(hidden)] fn as_raw_MotionEstimatorBase(&self) -> *mut c_void { self.ptr }
}
impl MotionEstimatorRansacL2 {

  pub fn set_min_inlier_ratio(&mut self, val: f32) -> Result<()> {
  // identifier: cv_videostab_MotionEstimatorRansacL2_setMinInlierRatio_float_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_MotionEstimatorRansacL2_setMinInlierRatio_float_val(self.as_raw_MotionEstimatorRansacL2(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn min_inlier_ratio(&self) -> Result<f32> {
  // identifier: cv_videostab_MotionEstimatorRansacL2_minInlierRatio
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_MotionEstimatorRansacL2_minInlierRatio(self.as_raw_MotionEstimatorRansacL2());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

}
// Generating impl for trait cv::videostab::MotionFilterBase (trait)
pub trait MotionFilterBase : super::videostab::IMotionStabilizer {
  #[doc(hidden)] fn as_raw_MotionFilterBase(&self) -> *mut c_void;
}
impl<'a> MotionFilterBase + 'a {

}

// boxed class cv::videostab::MotionInpainter

#[allow(dead_code)]
pub struct MotionInpainter {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::MotionInpainter {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_MotionInpainter(self.ptr) };
    }
}
impl super::videostab::MotionInpainter {
    #[doc(hidden)] pub fn as_raw_MotionInpainter(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::InpainterBase for MotionInpainter {
    #[doc(hidden)] fn as_raw_InpainterBase(&self) -> *mut c_void { self.ptr }
}
impl MotionInpainter {

  pub fn new() -> Result<super::videostab::MotionInpainter> {
  // identifier: cv_videostab_MotionInpainter_MotionInpainter
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_MotionInpainter_MotionInpainter();
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(super::videostab::MotionInpainter { ptr: rv.result })
      }
    }
  }

  pub fn set_flow_error_threshold(&mut self, val: f32) -> Result<()> {
  // identifier: cv_videostab_MotionInpainter_setFlowErrorThreshold_float_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_MotionInpainter_setFlowErrorThreshold_float_val(self.as_raw_MotionInpainter(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn flow_error_threshold(&self) -> Result<f32> {
  // identifier: cv_videostab_MotionInpainter_flowErrorThreshold
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_MotionInpainter_flowErrorThreshold(self.as_raw_MotionInpainter());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn set_dist_threshold(&mut self, val: f32) -> Result<()> {
  // identifier: cv_videostab_MotionInpainter_setDistThreshold_float_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_MotionInpainter_setDistThreshold_float_val(self.as_raw_MotionInpainter(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn dist_thresh(&self) -> Result<f32> {
  // identifier: cv_videostab_MotionInpainter_distThresh
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_MotionInpainter_distThresh(self.as_raw_MotionInpainter());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn set_border_mode(&mut self, val: i32) -> Result<()> {
  // identifier: cv_videostab_MotionInpainter_setBorderMode_int_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_MotionInpainter_setBorderMode_int_val(self.as_raw_MotionInpainter(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn border_mode(&self) -> Result<i32> {
  // identifier: cv_videostab_MotionInpainter_borderMode
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_MotionInpainter_borderMode(self.as_raw_MotionInpainter());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn inpaint(&mut self, idx: i32, frame: &core::Mat, mask: &core::Mat) -> Result<()> {
  // identifier: cv_videostab_MotionInpainter_inpaint_int_idx_Mat_frame_Mat_mask
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_MotionInpainter_inpaint_int_idx_Mat_frame_Mat_mask(self.as_raw_MotionInpainter(), idx, frame.as_raw_Mat(), mask.as_raw_Mat());
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
// boxed class cv::videostab::MotionStabilizationPipeline

#[allow(dead_code)]
pub struct MotionStabilizationPipeline {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::MotionStabilizationPipeline {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_MotionStabilizationPipeline(self.ptr) };
    }
}
impl super::videostab::MotionStabilizationPipeline {
    #[doc(hidden)] pub fn as_raw_MotionStabilizationPipeline(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::IMotionStabilizer for MotionStabilizationPipeline {
    #[doc(hidden)] fn as_raw_IMotionStabilizer(&self) -> *mut c_void { self.ptr }
}
impl MotionStabilizationPipeline {

  pub fn push_back(&mut self, stabilizer: &types::PtrOfIMotionStabilizer) -> Result<()> {
  // identifier: cv_videostab_MotionStabilizationPipeline_pushBack_PtrOfIMotionStabilizer_stabilizer
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_MotionStabilizationPipeline_pushBack_PtrOfIMotionStabilizer_stabilizer(self.as_raw_MotionStabilizationPipeline(), stabilizer.as_raw_PtrOfIMotionStabilizer());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn empty(&self) -> Result<bool> {
  // identifier: cv_videostab_MotionStabilizationPipeline_empty
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_MotionStabilizationPipeline_empty(self.as_raw_MotionStabilizationPipeline());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

}
// boxed class cv::videostab::NullDeblurer

#[allow(dead_code)]
pub struct NullDeblurer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::NullDeblurer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_NullDeblurer(self.ptr) };
    }
}
impl super::videostab::NullDeblurer {
    #[doc(hidden)] pub fn as_raw_NullDeblurer(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::DeblurerBase for NullDeblurer {
    #[doc(hidden)] fn as_raw_DeblurerBase(&self) -> *mut c_void { self.ptr }
}
impl NullDeblurer {

  pub fn deblur(&mut self, unnamed_arg: i32, unnamed_arg_1: &core::Mat) -> Result<()> {
  // identifier: cv_videostab_NullDeblurer_deblur_int_unnamed_arg_Mat_unnamed_arg_1
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_NullDeblurer_deblur_int_unnamed_arg_Mat_unnamed_arg_1(self.as_raw_NullDeblurer(), unnamed_arg, unnamed_arg_1.as_raw_Mat());
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
// boxed class cv::videostab::NullFrameSource

#[allow(dead_code)]
pub struct NullFrameSource {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::NullFrameSource {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_NullFrameSource(self.ptr) };
    }
}
impl super::videostab::NullFrameSource {
    #[doc(hidden)] pub fn as_raw_NullFrameSource(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::IFrameSource for NullFrameSource {
    #[doc(hidden)] fn as_raw_IFrameSource(&self) -> *mut c_void { self.ptr }
}
impl NullFrameSource {

  pub fn reset(&mut self) -> Result<()> {
  // identifier: cv_videostab_NullFrameSource_reset
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_NullFrameSource_reset(self.as_raw_NullFrameSource());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn next_frame(&mut self) -> Result<core::Mat> {
  // identifier: cv_videostab_NullFrameSource_nextFrame
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_NullFrameSource_nextFrame(self.as_raw_NullFrameSource());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(core::Mat { ptr: rv.result })
      }
    }
  }

}
// boxed class cv::videostab::NullInpainter

#[allow(dead_code)]
pub struct NullInpainter {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::NullInpainter {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_NullInpainter(self.ptr) };
    }
}
impl super::videostab::NullInpainter {
    #[doc(hidden)] pub fn as_raw_NullInpainter(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::InpainterBase for NullInpainter {
    #[doc(hidden)] fn as_raw_InpainterBase(&self) -> *mut c_void { self.ptr }
}
impl NullInpainter {

  pub fn inpaint(&mut self, unnamed_arg: i32, unnamed_arg_1: &core::Mat, unnamed_arg_2: &core::Mat) -> Result<()> {
  // identifier: cv_videostab_NullInpainter_inpaint_int_unnamed_arg_Mat_unnamed_arg_1_Mat_unnamed_arg_2
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_NullInpainter_inpaint_int_unnamed_arg_Mat_unnamed_arg_1_Mat_unnamed_arg_2(self.as_raw_NullInpainter(), unnamed_arg, unnamed_arg_1.as_raw_Mat(), unnamed_arg_2.as_raw_Mat());
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
// boxed class cv::videostab::NullLog

#[allow(dead_code)]
pub struct NullLog {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::NullLog {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_NullLog(self.ptr) };
    }
}
impl super::videostab::NullLog {
    #[doc(hidden)] pub fn as_raw_NullLog(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::ILog for NullLog {
    #[doc(hidden)] fn as_raw_ILog(&self) -> *mut c_void { self.ptr }
}
impl NullLog {

}
// boxed class cv::videostab::NullOutlierRejector

#[allow(dead_code)]
pub struct NullOutlierRejector {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::NullOutlierRejector {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_NullOutlierRejector(self.ptr) };
    }
}
impl super::videostab::NullOutlierRejector {
    #[doc(hidden)] pub fn as_raw_NullOutlierRejector(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::IOutlierRejector for NullOutlierRejector {
    #[doc(hidden)] fn as_raw_IOutlierRejector(&self) -> *mut c_void { self.ptr }
}
impl NullOutlierRejector {

  pub fn process(&mut self, frame_size: core::Size, points0: &core::Mat, points1: &core::Mat, mask: &mut core::Mat) -> Result<()> {
  // identifier: cv_videostab_NullOutlierRejector_process_Size_frameSize_Mat_points0_Mat_points1_Mat_mask
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_NullOutlierRejector_process_Size_frameSize_Mat_points0_Mat_points1_Mat_mask(self.as_raw_NullOutlierRejector(), frame_size, points0.as_raw_Mat(), points1.as_raw_Mat(), mask.as_raw_Mat());
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
// boxed class cv::videostab::NullWobbleSuppressor

#[allow(dead_code)]
pub struct NullWobbleSuppressor {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::NullWobbleSuppressor {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_NullWobbleSuppressor(self.ptr) };
    }
}
impl super::videostab::NullWobbleSuppressor {
    #[doc(hidden)] pub fn as_raw_NullWobbleSuppressor(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::WobbleSuppressorBase for NullWobbleSuppressor {
    #[doc(hidden)] fn as_raw_WobbleSuppressorBase(&self) -> *mut c_void { self.ptr }
}
impl NullWobbleSuppressor {

  pub fn suppress(&mut self, idx: i32, frame: &core::Mat, result: &core::Mat) -> Result<()> {
  // identifier: cv_videostab_NullWobbleSuppressor_suppress_int_idx_Mat_frame_Mat_result
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_NullWobbleSuppressor_suppress_int_idx_Mat_frame_Mat_result(self.as_raw_NullWobbleSuppressor(), idx, frame.as_raw_Mat(), result.as_raw_Mat());
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
// boxed class cv::videostab::OnePassStabilizer

#[allow(dead_code)]
pub struct OnePassStabilizer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::OnePassStabilizer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_OnePassStabilizer(self.ptr) };
    }
}
impl super::videostab::OnePassStabilizer {
    #[doc(hidden)] pub fn as_raw_OnePassStabilizer(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::IFrameSource for OnePassStabilizer {
    #[doc(hidden)] fn as_raw_IFrameSource(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::StabilizerBase for OnePassStabilizer {
    #[doc(hidden)] fn as_raw_StabilizerBase(&self) -> *mut c_void { self.ptr }
}
impl OnePassStabilizer {

  pub fn new() -> Result<super::videostab::OnePassStabilizer> {
  // identifier: cv_videostab_OnePassStabilizer_OnePassStabilizer
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_OnePassStabilizer_OnePassStabilizer();
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(super::videostab::OnePassStabilizer { ptr: rv.result })
      }
    }
  }

  pub fn set_motion_filter(&mut self, val: &types::PtrOfMotionFilterBase) -> Result<()> {
  // identifier: cv_videostab_OnePassStabilizer_setMotionFilter_PtrOfMotionFilterBase_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_OnePassStabilizer_setMotionFilter_PtrOfMotionFilterBase_val(self.as_raw_OnePassStabilizer(), val.as_raw_PtrOfMotionFilterBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn motion_filter(&self) -> Result<types::PtrOfMotionFilterBase> {
  // identifier: cv_videostab_OnePassStabilizer_motionFilter
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_OnePassStabilizer_motionFilter(self.as_raw_OnePassStabilizer());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::PtrOfMotionFilterBase { ptr: rv.result })
      }
    }
  }

  pub fn reset(&mut self) -> Result<()> {
  // identifier: cv_videostab_OnePassStabilizer_reset
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_OnePassStabilizer_reset(self.as_raw_OnePassStabilizer());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn next_frame(&mut self) -> Result<core::Mat> {
  // identifier: cv_videostab_OnePassStabilizer_nextFrame
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_OnePassStabilizer_nextFrame(self.as_raw_OnePassStabilizer());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(core::Mat { ptr: rv.result })
      }
    }
  }

}
// Generating impl for trait cv::videostab::PyrLkOptFlowEstimatorBase (trait)
pub trait PyrLkOptFlowEstimatorBase {
  #[doc(hidden)] fn as_raw_PyrLkOptFlowEstimatorBase(&self) -> *mut c_void;
  fn set_win_size(&mut self, val: core::Size) -> Result<()> {
  // identifier: cv_videostab_PyrLkOptFlowEstimatorBase_setWinSize_Size_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_PyrLkOptFlowEstimatorBase_setWinSize_Size_val(self.as_raw_PyrLkOptFlowEstimatorBase(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn win_size(&self) -> Result<core::Size> {
  // identifier: cv_videostab_PyrLkOptFlowEstimatorBase_winSize
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_PyrLkOptFlowEstimatorBase_winSize(self.as_raw_PyrLkOptFlowEstimatorBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn set_max_level(&mut self, val: i32) -> Result<()> {
  // identifier: cv_videostab_PyrLkOptFlowEstimatorBase_setMaxLevel_int_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_PyrLkOptFlowEstimatorBase_setMaxLevel_int_val(self.as_raw_PyrLkOptFlowEstimatorBase(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn max_level(&self) -> Result<i32> {
  // identifier: cv_videostab_PyrLkOptFlowEstimatorBase_maxLevel
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_PyrLkOptFlowEstimatorBase_maxLevel(self.as_raw_PyrLkOptFlowEstimatorBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

}
impl<'a> PyrLkOptFlowEstimatorBase + 'a {

}

// boxed class cv::videostab::RansacParams
/// Describes RANSAC method parameters.

#[allow(dead_code)]
pub struct RansacParams {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::RansacParams {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_RansacParams(self.ptr) };
    }
}
impl super::videostab::RansacParams {
    #[doc(hidden)] pub fn as_raw_RansacParams(&self) -> *mut c_void { self.ptr }
}
impl RansacParams {

  /// @return Number of iterations that'll be performed by RANSAC method.
  pub fn niters(&self) -> Result<i32> {
  // identifier: cv_videostab_RansacParams_niters
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_RansacParams_niters(self.as_raw_RansacParams());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

}
// boxed class cv::videostab::SparsePyrLkOptFlowEstimator

#[allow(dead_code)]
pub struct SparsePyrLkOptFlowEstimator {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::SparsePyrLkOptFlowEstimator {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_SparsePyrLkOptFlowEstimator(self.ptr) };
    }
}
impl super::videostab::SparsePyrLkOptFlowEstimator {
    #[doc(hidden)] pub fn as_raw_SparsePyrLkOptFlowEstimator(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::ISparseOptFlowEstimator for SparsePyrLkOptFlowEstimator {
    #[doc(hidden)] fn as_raw_ISparseOptFlowEstimator(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::PyrLkOptFlowEstimatorBase for SparsePyrLkOptFlowEstimator {
    #[doc(hidden)] fn as_raw_PyrLkOptFlowEstimatorBase(&self) -> *mut c_void { self.ptr }
}
impl SparsePyrLkOptFlowEstimator {

  pub fn run(&mut self, frame0: &core::Mat, frame1: &core::Mat, points0: &core::Mat, points1: &mut core::Mat, status: &mut core::Mat, errors: &mut core::Mat) -> Result<()> {
  // identifier: cv_videostab_SparsePyrLkOptFlowEstimator_run_Mat_frame0_Mat_frame1_Mat_points0_Mat_points1_Mat_status_Mat_errors
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_SparsePyrLkOptFlowEstimator_run_Mat_frame0_Mat_frame1_Mat_points0_Mat_points1_Mat_status_Mat_errors(self.as_raw_SparsePyrLkOptFlowEstimator(), frame0.as_raw_Mat(), frame1.as_raw_Mat(), points0.as_raw_Mat(), points1.as_raw_Mat(), status.as_raw_Mat(), errors.as_raw_Mat());
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
// Generating impl for trait cv::videostab::StabilizerBase (trait)
pub trait StabilizerBase {
  #[doc(hidden)] fn as_raw_StabilizerBase(&self) -> *mut c_void;
  fn set_log(&mut self, ilog: &types::PtrOfILog) -> Result<()> {
  // identifier: cv_videostab_StabilizerBase_setLog_PtrOfILog_ilog
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_StabilizerBase_setLog_PtrOfILog_ilog(self.as_raw_StabilizerBase(), ilog.as_raw_PtrOfILog());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn log(&self) -> Result<types::PtrOfILog> {
  // identifier: cv_videostab_StabilizerBase_log
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_StabilizerBase_log(self.as_raw_StabilizerBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::PtrOfILog { ptr: rv.result })
      }
    }
  }

  fn set_radius(&mut self, val: i32) -> Result<()> {
  // identifier: cv_videostab_StabilizerBase_setRadius_int_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_StabilizerBase_setRadius_int_val(self.as_raw_StabilizerBase(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn radius(&self) -> Result<i32> {
  // identifier: cv_videostab_StabilizerBase_radius
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_StabilizerBase_radius(self.as_raw_StabilizerBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn set_frame_source(&mut self, val: &types::PtrOfIFrameSource) -> Result<()> {
  // identifier: cv_videostab_StabilizerBase_setFrameSource_PtrOfIFrameSource_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_StabilizerBase_setFrameSource_PtrOfIFrameSource_val(self.as_raw_StabilizerBase(), val.as_raw_PtrOfIFrameSource());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn frame_source(&self) -> Result<types::PtrOfIFrameSource> {
  // identifier: cv_videostab_StabilizerBase_frameSource
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_StabilizerBase_frameSource(self.as_raw_StabilizerBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::PtrOfIFrameSource { ptr: rv.result })
      }
    }
  }

  fn set_motion_estimator(&mut self, val: &types::PtrOfImageMotionEstimatorBase) -> Result<()> {
  // identifier: cv_videostab_StabilizerBase_setMotionEstimator_PtrOfImageMotionEstimatorBase_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_StabilizerBase_setMotionEstimator_PtrOfImageMotionEstimatorBase_val(self.as_raw_StabilizerBase(), val.as_raw_PtrOfImageMotionEstimatorBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn motion_estimator(&self) -> Result<types::PtrOfImageMotionEstimatorBase> {
  // identifier: cv_videostab_StabilizerBase_motionEstimator
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_StabilizerBase_motionEstimator(self.as_raw_StabilizerBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::PtrOfImageMotionEstimatorBase { ptr: rv.result })
      }
    }
  }

  fn set_deblurer(&mut self, val: &types::PtrOfDeblurerBase) -> Result<()> {
  // identifier: cv_videostab_StabilizerBase_setDeblurer_PtrOfDeblurerBase_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_StabilizerBase_setDeblurer_PtrOfDeblurerBase_val(self.as_raw_StabilizerBase(), val.as_raw_PtrOfDeblurerBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn deblurrer(&self) -> Result<types::PtrOfDeblurerBase> {
  // identifier: cv_videostab_StabilizerBase_deblurrer
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_StabilizerBase_deblurrer(self.as_raw_StabilizerBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::PtrOfDeblurerBase { ptr: rv.result })
      }
    }
  }

  fn set_trim_ratio(&mut self, val: f32) -> Result<()> {
  // identifier: cv_videostab_StabilizerBase_setTrimRatio_float_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_StabilizerBase_setTrimRatio_float_val(self.as_raw_StabilizerBase(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn trim_ratio(&self) -> Result<f32> {
  // identifier: cv_videostab_StabilizerBase_trimRatio
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_StabilizerBase_trimRatio(self.as_raw_StabilizerBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn set_correction_for_inclusion(&mut self, val: bool) -> Result<()> {
  // identifier: cv_videostab_StabilizerBase_setCorrectionForInclusion_bool_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_StabilizerBase_setCorrectionForInclusion_bool_val(self.as_raw_StabilizerBase(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn do_correction_for_inclusion(&self) -> Result<bool> {
  // identifier: cv_videostab_StabilizerBase_doCorrectionForInclusion
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_StabilizerBase_doCorrectionForInclusion(self.as_raw_StabilizerBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn set_border_mode(&mut self, val: i32) -> Result<()> {
  // identifier: cv_videostab_StabilizerBase_setBorderMode_int_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_StabilizerBase_setBorderMode_int_val(self.as_raw_StabilizerBase(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn border_mode(&self) -> Result<i32> {
  // identifier: cv_videostab_StabilizerBase_borderMode
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_StabilizerBase_borderMode(self.as_raw_StabilizerBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn set_inpainter(&mut self, val: &types::PtrOfInpainterBase) -> Result<()> {
  // identifier: cv_videostab_StabilizerBase_setInpainter_PtrOfInpainterBase_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_StabilizerBase_setInpainter_PtrOfInpainterBase_val(self.as_raw_StabilizerBase(), val.as_raw_PtrOfInpainterBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn inpainter(&self) -> Result<types::PtrOfInpainterBase> {
  // identifier: cv_videostab_StabilizerBase_inpainter
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_StabilizerBase_inpainter(self.as_raw_StabilizerBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::PtrOfInpainterBase { ptr: rv.result })
      }
    }
  }

}
impl<'a> StabilizerBase + 'a {

}

// boxed class cv::videostab::ToFileMotionWriter

#[allow(dead_code)]
pub struct ToFileMotionWriter {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::ToFileMotionWriter {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_ToFileMotionWriter(self.ptr) };
    }
}
impl super::videostab::ToFileMotionWriter {
    #[doc(hidden)] pub fn as_raw_ToFileMotionWriter(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::ImageMotionEstimatorBase for ToFileMotionWriter {
    #[doc(hidden)] fn as_raw_ImageMotionEstimatorBase(&self) -> *mut c_void { self.ptr }
}
impl ToFileMotionWriter {

  pub fn new(path:&str, estimator: &types::PtrOfImageMotionEstimatorBase) -> Result<super::videostab::ToFileMotionWriter> {
  // identifier: cv_videostab_ToFileMotionWriter_ToFileMotionWriter_String_path_PtrOfImageMotionEstimatorBase_estimator
    unsafe {
      let path = CString::new(path).unwrap();
      let rv = sys::cv_videostab_cv_videostab_ToFileMotionWriter_ToFileMotionWriter_String_path_PtrOfImageMotionEstimatorBase_estimator(path.as_ptr() as _, estimator.as_raw_PtrOfImageMotionEstimatorBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(super::videostab::ToFileMotionWriter { ptr: rv.result })
      }
    }
  }

}
// boxed class cv::videostab::TranslationBasedLocalOutlierRejector

#[allow(dead_code)]
pub struct TranslationBasedLocalOutlierRejector {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::TranslationBasedLocalOutlierRejector {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_TranslationBasedLocalOutlierRejector(self.ptr) };
    }
}
impl super::videostab::TranslationBasedLocalOutlierRejector {
    #[doc(hidden)] pub fn as_raw_TranslationBasedLocalOutlierRejector(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::IOutlierRejector for TranslationBasedLocalOutlierRejector {
    #[doc(hidden)] fn as_raw_IOutlierRejector(&self) -> *mut c_void { self.ptr }
}
impl TranslationBasedLocalOutlierRejector {

  pub fn new() -> Result<super::videostab::TranslationBasedLocalOutlierRejector> {
  // identifier: cv_videostab_TranslationBasedLocalOutlierRejector_TranslationBasedLocalOutlierRejector
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_TranslationBasedLocalOutlierRejector_TranslationBasedLocalOutlierRejector();
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(super::videostab::TranslationBasedLocalOutlierRejector { ptr: rv.result })
      }
    }
  }

  pub fn set_cell_size(&mut self, val: core::Size) -> Result<()> {
  // identifier: cv_videostab_TranslationBasedLocalOutlierRejector_setCellSize_Size_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_TranslationBasedLocalOutlierRejector_setCellSize_Size_val(self.as_raw_TranslationBasedLocalOutlierRejector(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn cell_size(&self) -> Result<core::Size> {
  // identifier: cv_videostab_TranslationBasedLocalOutlierRejector_cellSize
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_TranslationBasedLocalOutlierRejector_cellSize(self.as_raw_TranslationBasedLocalOutlierRejector());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn process(&mut self, frame_size: core::Size, points0: &core::Mat, points1: &core::Mat, mask: &mut core::Mat) -> Result<()> {
  // identifier: cv_videostab_TranslationBasedLocalOutlierRejector_process_Size_frameSize_Mat_points0_Mat_points1_Mat_mask
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_TranslationBasedLocalOutlierRejector_process_Size_frameSize_Mat_points0_Mat_points1_Mat_mask(self.as_raw_TranslationBasedLocalOutlierRejector(), frame_size, points0.as_raw_Mat(), points1.as_raw_Mat(), mask.as_raw_Mat());
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
// boxed class cv::videostab::TwoPassStabilizer

#[allow(dead_code)]
pub struct TwoPassStabilizer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::TwoPassStabilizer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_TwoPassStabilizer(self.ptr) };
    }
}
impl super::videostab::TwoPassStabilizer {
    #[doc(hidden)] pub fn as_raw_TwoPassStabilizer(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::IFrameSource for TwoPassStabilizer {
    #[doc(hidden)] fn as_raw_IFrameSource(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::StabilizerBase for TwoPassStabilizer {
    #[doc(hidden)] fn as_raw_StabilizerBase(&self) -> *mut c_void { self.ptr }
}
impl TwoPassStabilizer {

  pub fn new() -> Result<super::videostab::TwoPassStabilizer> {
  // identifier: cv_videostab_TwoPassStabilizer_TwoPassStabilizer
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_TwoPassStabilizer_TwoPassStabilizer();
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(super::videostab::TwoPassStabilizer { ptr: rv.result })
      }
    }
  }

  pub fn set_motion_stabilizer(&mut self, val: &types::PtrOfIMotionStabilizer) -> Result<()> {
  // identifier: cv_videostab_TwoPassStabilizer_setMotionStabilizer_PtrOfIMotionStabilizer_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_TwoPassStabilizer_setMotionStabilizer_PtrOfIMotionStabilizer_val(self.as_raw_TwoPassStabilizer(), val.as_raw_PtrOfIMotionStabilizer());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn motion_stabilizer(&self) -> Result<types::PtrOfIMotionStabilizer> {
  // identifier: cv_videostab_TwoPassStabilizer_motionStabilizer
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_TwoPassStabilizer_motionStabilizer(self.as_raw_TwoPassStabilizer());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::PtrOfIMotionStabilizer { ptr: rv.result })
      }
    }
  }

  pub fn set_estimate_trim_ratio(&mut self, val: bool) -> Result<()> {
  // identifier: cv_videostab_TwoPassStabilizer_setEstimateTrimRatio_bool_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_TwoPassStabilizer_setEstimateTrimRatio_bool_val(self.as_raw_TwoPassStabilizer(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn must_estimate_trima_ratio(&self) -> Result<bool> {
  // identifier: cv_videostab_TwoPassStabilizer_mustEstimateTrimaRatio
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_TwoPassStabilizer_mustEstimateTrimaRatio(self.as_raw_TwoPassStabilizer());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn reset(&mut self) -> Result<()> {
  // identifier: cv_videostab_TwoPassStabilizer_reset
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_TwoPassStabilizer_reset(self.as_raw_TwoPassStabilizer());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn next_frame(&mut self) -> Result<core::Mat> {
  // identifier: cv_videostab_TwoPassStabilizer_nextFrame
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_TwoPassStabilizer_nextFrame(self.as_raw_TwoPassStabilizer());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(core::Mat { ptr: rv.result })
      }
    }
  }

}
// boxed class cv::videostab::VideoFileSource

#[allow(dead_code)]
pub struct VideoFileSource {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::VideoFileSource {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_VideoFileSource(self.ptr) };
    }
}
impl super::videostab::VideoFileSource {
    #[doc(hidden)] pub fn as_raw_VideoFileSource(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::IFrameSource for VideoFileSource {
    #[doc(hidden)] fn as_raw_IFrameSource(&self) -> *mut c_void { self.ptr }
}
impl VideoFileSource {

  ///
  /// ## C++ default parameters:
  /// * volatile_frame: false
  pub fn new(path:&str, volatile_frame: bool) -> Result<super::videostab::VideoFileSource> {
  // identifier: cv_videostab_VideoFileSource_VideoFileSource_String_path_bool_volatileFrame
    unsafe {
      let path = CString::new(path).unwrap();
      let rv = sys::cv_videostab_cv_videostab_VideoFileSource_VideoFileSource_String_path_bool_volatileFrame(path.as_ptr() as _, volatile_frame);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(super::videostab::VideoFileSource { ptr: rv.result })
      }
    }
  }

  pub fn reset(&mut self) -> Result<()> {
  // identifier: cv_videostab_VideoFileSource_reset
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_VideoFileSource_reset(self.as_raw_VideoFileSource());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn next_frame(&mut self) -> Result<core::Mat> {
  // identifier: cv_videostab_VideoFileSource_nextFrame
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_VideoFileSource_nextFrame(self.as_raw_VideoFileSource());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(core::Mat { ptr: rv.result })
      }
    }
  }

  pub fn width(&mut self) -> Result<i32> {
  // identifier: cv_videostab_VideoFileSource_width
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_VideoFileSource_width(self.as_raw_VideoFileSource());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn height(&mut self) -> Result<i32> {
  // identifier: cv_videostab_VideoFileSource_height
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_VideoFileSource_height(self.as_raw_VideoFileSource());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn count(&mut self) -> Result<i32> {
  // identifier: cv_videostab_VideoFileSource_count
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_VideoFileSource_count(self.as_raw_VideoFileSource());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn fps(&mut self) -> Result<f64> {
  // identifier: cv_videostab_VideoFileSource_fps
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_VideoFileSource_fps(self.as_raw_VideoFileSource());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

}
// boxed class cv::videostab::WeightingDeblurer

#[allow(dead_code)]
pub struct WeightingDeblurer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::videostab::WeightingDeblurer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_WeightingDeblurer(self.ptr) };
    }
}
impl super::videostab::WeightingDeblurer {
    #[doc(hidden)] pub fn as_raw_WeightingDeblurer(&self) -> *mut c_void { self.ptr }
}

impl super::videostab::DeblurerBase for WeightingDeblurer {
    #[doc(hidden)] fn as_raw_DeblurerBase(&self) -> *mut c_void { self.ptr }
}
impl WeightingDeblurer {

  pub fn new() -> Result<super::videostab::WeightingDeblurer> {
  // identifier: cv_videostab_WeightingDeblurer_WeightingDeblurer
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_WeightingDeblurer_WeightingDeblurer();
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(super::videostab::WeightingDeblurer { ptr: rv.result })
      }
    }
  }

  pub fn set_sensitivity(&mut self, val: f32) -> Result<()> {
  // identifier: cv_videostab_WeightingDeblurer_setSensitivity_float_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_WeightingDeblurer_setSensitivity_float_val(self.as_raw_WeightingDeblurer(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn sensitivity(&self) -> Result<f32> {
  // identifier: cv_videostab_WeightingDeblurer_sensitivity
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_WeightingDeblurer_sensitivity(self.as_raw_WeightingDeblurer());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn deblur(&mut self, idx: i32, frame: &core::Mat) -> Result<()> {
  // identifier: cv_videostab_WeightingDeblurer_deblur_int_idx_Mat_frame
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_WeightingDeblurer_deblur_int_idx_Mat_frame(self.as_raw_WeightingDeblurer(), idx, frame.as_raw_Mat());
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
// Generating impl for trait cv::videostab::WobbleSuppressorBase (trait)
pub trait WobbleSuppressorBase {
  #[doc(hidden)] fn as_raw_WobbleSuppressorBase(&self) -> *mut c_void;
  fn set_motion_estimator(&mut self, val: &types::PtrOfImageMotionEstimatorBase) -> Result<()> {
  // identifier: cv_videostab_WobbleSuppressorBase_setMotionEstimator_PtrOfImageMotionEstimatorBase_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_WobbleSuppressorBase_setMotionEstimator_PtrOfImageMotionEstimatorBase_val(self.as_raw_WobbleSuppressorBase(), val.as_raw_PtrOfImageMotionEstimatorBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn motion_estimator(&self) -> Result<types::PtrOfImageMotionEstimatorBase> {
  // identifier: cv_videostab_WobbleSuppressorBase_motionEstimator
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_WobbleSuppressorBase_motionEstimator(self.as_raw_WobbleSuppressorBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::PtrOfImageMotionEstimatorBase { ptr: rv.result })
      }
    }
  }

  fn suppress(&mut self, idx: i32, frame: &core::Mat, result: &core::Mat) -> Result<()> {
  // identifier: cv_videostab_WobbleSuppressorBase_suppress_int_idx_Mat_frame_Mat_result
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_WobbleSuppressorBase_suppress_int_idx_Mat_frame_Mat_result(self.as_raw_WobbleSuppressorBase(), idx, frame.as_raw_Mat(), result.as_raw_Mat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn set_frame_count(&mut self, val: i32) -> Result<()> {
  // identifier: cv_videostab_WobbleSuppressorBase_setFrameCount_int_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_WobbleSuppressorBase_setFrameCount_int_val(self.as_raw_WobbleSuppressorBase(), val);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn frame_count(&self) -> Result<i32> {
  // identifier: cv_videostab_WobbleSuppressorBase_frameCount
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_WobbleSuppressorBase_frameCount(self.as_raw_WobbleSuppressorBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn set_motions(&mut self, val: &types::VectorOfMat) -> Result<()> {
  // identifier: cv_videostab_WobbleSuppressorBase_setMotions_VectorOfMat_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_WobbleSuppressorBase_setMotions_VectorOfMat_val(self.as_raw_WobbleSuppressorBase(), val.as_raw_VectorOfMat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn motions(&self) -> Result<types::VectorOfMat> {
  // identifier: cv_videostab_WobbleSuppressorBase_motions
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_WobbleSuppressorBase_motions(self.as_raw_WobbleSuppressorBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::VectorOfMat { ptr: rv.result })
      }
    }
  }

  fn set_motions2(&mut self, val: &types::VectorOfMat) -> Result<()> {
  // identifier: cv_videostab_WobbleSuppressorBase_setMotions2_VectorOfMat_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_WobbleSuppressorBase_setMotions2_VectorOfMat_val(self.as_raw_WobbleSuppressorBase(), val.as_raw_VectorOfMat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn motions2(&self) -> Result<types::VectorOfMat> {
  // identifier: cv_videostab_WobbleSuppressorBase_motions2
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_WobbleSuppressorBase_motions2(self.as_raw_WobbleSuppressorBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::VectorOfMat { ptr: rv.result })
      }
    }
  }

  fn set_stabilization_motions(&mut self, val: &types::VectorOfMat) -> Result<()> {
  // identifier: cv_videostab_WobbleSuppressorBase_setStabilizationMotions_VectorOfMat_val
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_WobbleSuppressorBase_setStabilizationMotions_VectorOfMat_val(self.as_raw_WobbleSuppressorBase(), val.as_raw_VectorOfMat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn stabilization_motions(&self) -> Result<types::VectorOfMat> {
  // identifier: cv_videostab_WobbleSuppressorBase_stabilizationMotions
    unsafe {
      let rv = sys::cv_videostab_cv_videostab_WobbleSuppressorBase_stabilizationMotions(self.as_raw_WobbleSuppressorBase());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::VectorOfMat { ptr: rv.result })
      }
    }
  }

}
impl<'a> WobbleSuppressorBase + 'a {

}

