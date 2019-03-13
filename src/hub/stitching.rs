//! <script type="text/javascript" src="http://latex.codecogs.com/latexit.js"></script>
//! Images stitching
//! 
//! # Images stitching
//! 
//! This figure illustrates the stitching module pipeline implemented in the Stitcher class. Using that
//! class it's possible to configure/remove some steps, i.e. adjust the stitching pipeline according to
//! the particular needs. All building blocks from the pipeline are available in the detail namespace,
//! one can combine and use them separately.
//! 
//! The implemented stitching pipeline is very similar to the one proposed in @cite BL07 .
//! 
//! ![stitching pipeline](StitchingPipeline.jpg)
//! 
//! Camera models
//! -------------
//! 
//! There are currently 2 camera models implemented in stitching pipeline.
//! 
//! - _Homography model_ expecting perspective transformations between images
//! implemented in @ref cv::detail::BestOf2NearestMatcher cv::detail::HomographyBasedEstimator
//! cv::detail::BundleAdjusterReproj cv::detail::BundleAdjusterRay
//! - _Affine model_ expecting affine transformation with 6 DOF or 4 DOF implemented in
//! @ref cv::detail::AffineBestOf2NearestMatcher cv::detail::AffineBasedEstimator
//! cv::detail::BundleAdjusterAffine cv::detail::BundleAdjusterAffinePartial cv::AffineWarper
//! 
//! Homography model is useful for creating photo panoramas captured by camera,
//! while affine-based model can be used to stitch scans and object captured by
//! specialized devices. Use @ref cv::Stitcher::create to get preconfigured pipeline for one
//! of those models.
//! 
//! 
//! Note:
//! Certain detailed settings of @ref cv::Stitcher might not make sense. Especially
//! you should not mix classes implementing affine model and classes implementing
//! Homography model, as they work with different transformations.
//! 
//! @{
//! Features Finding and Images Matching
//! 
//! # Features Finding and Images Matching
//! Rotation Estimation
//! 
//! # Rotation Estimation
//! Autocalibration
//! 
//! # Autocalibration
//! Images Warping
//! 
//! # Images Warping
//! Seam Estimation
//! 
//! # Seam Estimation
//! Exposure Compensation
//! 
//! # Exposure Compensation
//! Image Blenders
//! 
//! # Image Blenders
//! @}

use libc::{c_void, c_char, size_t};
use std::ffi::{CStr, CString};
use crate::{core, sys, types};
use crate::{Error, Result};
pub const Stitcher_ERR_CAMERA_PARAMS_ADJUST_FAIL: i32 = 3;
pub const Stitcher_ERR_HOMOGRAPHY_EST_FAIL: i32 = 2;
pub const Stitcher_ERR_NEED_MORE_IMGS: i32 = 1;
pub const Stitcher_OK: i32 = 0;
pub const Stitcher_ORIG_RESOL: i32 = -1;
pub const Stitcher_PANORAMA: i32 = 0;
pub const Stitcher_SCANS: i32 = 1;

///
/// ## C++ default parameters:
/// * try_use_gpu: false
pub fn create_stitcher_scans(try_use_gpu: bool) -> Result<types::PtrOfStitcher> {
// identifier: cv_createStitcherScans_bool_try_use_gpu
  unsafe {
    let rv = sys::cv_stitching_cv_createStitcherScans_bool_try_use_gpu(try_use_gpu);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(types::PtrOfStitcher { ptr: rv.result })
    }
  }
}

///
/// ## C++ default parameters:
/// * try_use_gpu: false
pub fn create_stitcher(try_use_gpu: bool) -> Result<types::PtrOfStitcher> {
// identifier: cv_createStitcher_bool_try_use_gpu
  unsafe {
    let rv = sys::cv_stitching_cv_createStitcher_bool_try_use_gpu(try_use_gpu);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(types::PtrOfStitcher { ptr: rv.result })
    }
  }
}

// boxed class cv::AffineWarper
/// Affine warper factory class.
/// @sa detail::AffineWarper

#[allow(dead_code)]
pub struct AffineWarper {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::stitching::AffineWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_AffineWarper(self.ptr) };
    }
}
impl super::stitching::AffineWarper {
    #[doc(hidden)] pub fn as_raw_AffineWarper(&self) -> *mut c_void { self.ptr }
}

impl super::stitching::WarperCreator for AffineWarper {
    #[doc(hidden)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}
impl AffineWarper {

}
// boxed class cv::CompressedRectilinearPortraitWarper

#[allow(dead_code)]
pub struct CompressedRectilinearPortraitWarper {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::stitching::CompressedRectilinearPortraitWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_CompressedRectilinearPortraitWarper(self.ptr) };
    }
}
impl super::stitching::CompressedRectilinearPortraitWarper {
    #[doc(hidden)] pub fn as_raw_CompressedRectilinearPortraitWarper(&self) -> *mut c_void { self.ptr }
}

impl super::stitching::WarperCreator for CompressedRectilinearPortraitWarper {
    #[doc(hidden)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}
impl CompressedRectilinearPortraitWarper {

  ///
  /// ## C++ default parameters:
  /// * a: 1
  /// * b: 1
  pub fn new(a: f32, b: f32) -> Result<super::stitching::CompressedRectilinearPortraitWarper> {
  // identifier: cv_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper_float_A_float_B
    unsafe {
      let rv = sys::cv_stitching_cv_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper_float_A_float_B(a, b);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(super::stitching::CompressedRectilinearPortraitWarper { ptr: rv.result })
      }
    }
  }

}
// boxed class cv::CompressedRectilinearWarper

#[allow(dead_code)]
pub struct CompressedRectilinearWarper {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::stitching::CompressedRectilinearWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_CompressedRectilinearWarper(self.ptr) };
    }
}
impl super::stitching::CompressedRectilinearWarper {
    #[doc(hidden)] pub fn as_raw_CompressedRectilinearWarper(&self) -> *mut c_void { self.ptr }
}

impl super::stitching::WarperCreator for CompressedRectilinearWarper {
    #[doc(hidden)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}
impl CompressedRectilinearWarper {

  ///
  /// ## C++ default parameters:
  /// * a: 1
  /// * b: 1
  pub fn new(a: f32, b: f32) -> Result<super::stitching::CompressedRectilinearWarper> {
  // identifier: cv_CompressedRectilinearWarper_CompressedRectilinearWarper_float_A_float_B
    unsafe {
      let rv = sys::cv_stitching_cv_CompressedRectilinearWarper_CompressedRectilinearWarper_float_A_float_B(a, b);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(super::stitching::CompressedRectilinearWarper { ptr: rv.result })
      }
    }
  }

}
// boxed class cv::CylindricalWarper
/// Cylindrical warper factory class.
/// @sa detail::CylindricalWarper

#[allow(dead_code)]
pub struct CylindricalWarper {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::stitching::CylindricalWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_CylindricalWarper(self.ptr) };
    }
}
impl super::stitching::CylindricalWarper {
    #[doc(hidden)] pub fn as_raw_CylindricalWarper(&self) -> *mut c_void { self.ptr }
}

impl super::stitching::WarperCreator for CylindricalWarper {
    #[doc(hidden)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}
impl CylindricalWarper {

}
// boxed class cv::FisheyeWarper

#[allow(dead_code)]
pub struct FisheyeWarper {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::stitching::FisheyeWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_FisheyeWarper(self.ptr) };
    }
}
impl super::stitching::FisheyeWarper {
    #[doc(hidden)] pub fn as_raw_FisheyeWarper(&self) -> *mut c_void { self.ptr }
}

impl super::stitching::WarperCreator for FisheyeWarper {
    #[doc(hidden)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}
impl FisheyeWarper {

}
// boxed class cv::MercatorWarper

#[allow(dead_code)]
pub struct MercatorWarper {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::stitching::MercatorWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_MercatorWarper(self.ptr) };
    }
}
impl super::stitching::MercatorWarper {
    #[doc(hidden)] pub fn as_raw_MercatorWarper(&self) -> *mut c_void { self.ptr }
}

impl super::stitching::WarperCreator for MercatorWarper {
    #[doc(hidden)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}
impl MercatorWarper {

}
// boxed class cv::PaniniPortraitWarper

#[allow(dead_code)]
pub struct PaniniPortraitWarper {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::stitching::PaniniPortraitWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_PaniniPortraitWarper(self.ptr) };
    }
}
impl super::stitching::PaniniPortraitWarper {
    #[doc(hidden)] pub fn as_raw_PaniniPortraitWarper(&self) -> *mut c_void { self.ptr }
}

impl super::stitching::WarperCreator for PaniniPortraitWarper {
    #[doc(hidden)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}
impl PaniniPortraitWarper {

  ///
  /// ## C++ default parameters:
  /// * a: 1
  /// * b: 1
  pub fn new(a: f32, b: f32) -> Result<super::stitching::PaniniPortraitWarper> {
  // identifier: cv_PaniniPortraitWarper_PaniniPortraitWarper_float_A_float_B
    unsafe {
      let rv = sys::cv_stitching_cv_PaniniPortraitWarper_PaniniPortraitWarper_float_A_float_B(a, b);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(super::stitching::PaniniPortraitWarper { ptr: rv.result })
      }
    }
  }

}
// boxed class cv::PaniniWarper

#[allow(dead_code)]
pub struct PaniniWarper {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::stitching::PaniniWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_PaniniWarper(self.ptr) };
    }
}
impl super::stitching::PaniniWarper {
    #[doc(hidden)] pub fn as_raw_PaniniWarper(&self) -> *mut c_void { self.ptr }
}

impl super::stitching::WarperCreator for PaniniWarper {
    #[doc(hidden)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}
impl PaniniWarper {

  ///
  /// ## C++ default parameters:
  /// * a: 1
  /// * b: 1
  pub fn new(a: f32, b: f32) -> Result<super::stitching::PaniniWarper> {
  // identifier: cv_PaniniWarper_PaniniWarper_float_A_float_B
    unsafe {
      let rv = sys::cv_stitching_cv_PaniniWarper_PaniniWarper_float_A_float_B(a, b);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(super::stitching::PaniniWarper { ptr: rv.result })
      }
    }
  }

}
// boxed class cv::PlaneWarper
/// Plane warper factory class.
/// @sa detail::PlaneWarper

#[allow(dead_code)]
pub struct PlaneWarper {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::stitching::PlaneWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_PlaneWarper(self.ptr) };
    }
}
impl super::stitching::PlaneWarper {
    #[doc(hidden)] pub fn as_raw_PlaneWarper(&self) -> *mut c_void { self.ptr }
}

impl super::stitching::WarperCreator for PlaneWarper {
    #[doc(hidden)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}
impl PlaneWarper {

}
// boxed class cv::SphericalWarper
/// Spherical warper factory class

#[allow(dead_code)]
pub struct SphericalWarper {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::stitching::SphericalWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_SphericalWarper(self.ptr) };
    }
}
impl super::stitching::SphericalWarper {
    #[doc(hidden)] pub fn as_raw_SphericalWarper(&self) -> *mut c_void { self.ptr }
}

impl super::stitching::WarperCreator for SphericalWarper {
    #[doc(hidden)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}
impl SphericalWarper {

}
// boxed class cv::StereographicWarper

#[allow(dead_code)]
pub struct StereographicWarper {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::stitching::StereographicWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_StereographicWarper(self.ptr) };
    }
}
impl super::stitching::StereographicWarper {
    #[doc(hidden)] pub fn as_raw_StereographicWarper(&self) -> *mut c_void { self.ptr }
}

impl super::stitching::WarperCreator for StereographicWarper {
    #[doc(hidden)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}
impl StereographicWarper {

}
// boxed class cv::Stitcher
/// High level image stitcher.
/// 
/// It's possible to use this class without being aware of the entire stitching pipeline. However, to
/// be able to achieve higher stitching stability and quality of the final images at least being
/// familiar with the theory is recommended.
/// 
/// 
/// Note:
/// *   A basic example on image stitching can be found at
/// opencv_source_code/samples/cpp/stitching.cpp
/// *   A detailed example on image stitching can be found at
/// opencv_source_code/samples/cpp/stitching_detailed.cpp

#[allow(dead_code)]
pub struct Stitcher {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::stitching::Stitcher {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_Stitcher(self.ptr) };
    }
}
impl super::stitching::Stitcher {
    #[doc(hidden)] pub fn as_raw_Stitcher(&self) -> *mut c_void { self.ptr }
}
impl Stitcher {

  /// Creates a stitcher with the default parameters.
  /// 
  /// ## Parameters
  /// * try_use_gpu: Flag indicating whether GPU should be used whenever it's possible.
  /// @return Stitcher class instance.
  ///
  /// ## C++ default parameters:
  /// * try_use_gpu: false
  pub fn create_default(try_use_gpu: bool) -> Result<super::stitching::Stitcher> {
  // identifier: cv_Stitcher_createDefault_bool_try_use_gpu
    unsafe {
      let rv = sys::cv_stitching_cv_Stitcher_createDefault_bool_try_use_gpu(try_use_gpu);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(super::stitching::Stitcher { ptr: rv.result })
      }
    }
  }

  pub fn registration_resol(&self) -> Result<f64> {
  // identifier: cv_Stitcher_registrationResol
    unsafe {
      let rv = sys::cv_stitching_cv_Stitcher_registrationResol(self.as_raw_Stitcher());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn set_registration_resol(&mut self, resol_mpx: f64) -> Result<()> {
  // identifier: cv_Stitcher_setRegistrationResol_double_resol_mpx
    unsafe {
      let rv = sys::cv_stitching_cv_Stitcher_setRegistrationResol_double_resol_mpx(self.as_raw_Stitcher(), resol_mpx);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn seam_estimation_resol(&self) -> Result<f64> {
  // identifier: cv_Stitcher_seamEstimationResol
    unsafe {
      let rv = sys::cv_stitching_cv_Stitcher_seamEstimationResol(self.as_raw_Stitcher());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn set_seam_estimation_resol(&mut self, resol_mpx: f64) -> Result<()> {
  // identifier: cv_Stitcher_setSeamEstimationResol_double_resol_mpx
    unsafe {
      let rv = sys::cv_stitching_cv_Stitcher_setSeamEstimationResol_double_resol_mpx(self.as_raw_Stitcher(), resol_mpx);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn compositing_resol(&self) -> Result<f64> {
  // identifier: cv_Stitcher_compositingResol
    unsafe {
      let rv = sys::cv_stitching_cv_Stitcher_compositingResol(self.as_raw_Stitcher());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn set_compositing_resol(&mut self, resol_mpx: f64) -> Result<()> {
  // identifier: cv_Stitcher_setCompositingResol_double_resol_mpx
    unsafe {
      let rv = sys::cv_stitching_cv_Stitcher_setCompositingResol_double_resol_mpx(self.as_raw_Stitcher(), resol_mpx);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn pano_confidence_thresh(&self) -> Result<f64> {
  // identifier: cv_Stitcher_panoConfidenceThresh
    unsafe {
      let rv = sys::cv_stitching_cv_Stitcher_panoConfidenceThresh(self.as_raw_Stitcher());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn set_pano_confidence_thresh(&mut self, conf_thresh: f64) -> Result<()> {
  // identifier: cv_Stitcher_setPanoConfidenceThresh_double_conf_thresh
    unsafe {
      let rv = sys::cv_stitching_cv_Stitcher_setPanoConfidenceThresh_double_conf_thresh(self.as_raw_Stitcher(), conf_thresh);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn wave_correction(&self) -> Result<bool> {
  // identifier: cv_Stitcher_waveCorrection
    unsafe {
      let rv = sys::cv_stitching_cv_Stitcher_waveCorrection(self.as_raw_Stitcher());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn set_wave_correction(&mut self, flag: bool) -> Result<()> {
  // identifier: cv_Stitcher_setWaveCorrection_bool_flag
    unsafe {
      let rv = sys::cv_stitching_cv_Stitcher_setWaveCorrection_bool_flag(self.as_raw_Stitcher(), flag);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  pub fn component(&self) -> Result<types::VectorOfint> {
  // identifier: cv_Stitcher_component
    unsafe {
      let rv = sys::cv_stitching_cv_Stitcher_component(self.as_raw_Stitcher());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::VectorOfint { ptr: rv.result })
      }
    }
  }

  pub fn work_scale(&self) -> Result<f64> {
  // identifier: cv_Stitcher_workScale
    unsafe {
      let rv = sys::cv_stitching_cv_Stitcher_workScale(self.as_raw_Stitcher());
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
// boxed class cv::TransverseMercatorWarper

#[allow(dead_code)]
pub struct TransverseMercatorWarper {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::stitching::TransverseMercatorWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_TransverseMercatorWarper(self.ptr) };
    }
}
impl super::stitching::TransverseMercatorWarper {
    #[doc(hidden)] pub fn as_raw_TransverseMercatorWarper(&self) -> *mut c_void { self.ptr }
}

impl super::stitching::WarperCreator for TransverseMercatorWarper {
    #[doc(hidden)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}
impl TransverseMercatorWarper {

}
// Generating impl for trait cv::WarperCreator (trait)
/// Image warper factories base class.
pub trait WarperCreator {
  #[doc(hidden)] fn as_raw_WarperCreator(&self) -> *mut c_void;
}
impl<'a> WarperCreator + 'a {

}

