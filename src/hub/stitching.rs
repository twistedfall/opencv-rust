//! # Images stitching
//! 
//! This figure illustrates the stitching module pipeline implemented in the Stitcher class. Using that
//! class it's possible to configure/remove some steps, i.e. adjust the stitching pipeline according to
//! the particular needs. All building blocks from the pipeline are available in the detail namespace,
//! one can combine and use them separately.
//! 
//! The implemented stitching pipeline is very similar to the one proposed in [BL07](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_BL07) .
//! 
//! ![stitching pipeline](https://docs.opencv.org/3.4.6/StitchingPipeline.jpg)
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
//! # Features Finding and Images Matching
//! # Rotation Estimation
//! # Autocalibration
//! # Images Warping
//! # Seam Estimation
//! # Exposure Compensation
//! # Image Blenders
use std::os::raw::{c_char, c_void};
use libc::size_t;
use crate::{Error, Result, core, sys, types};

pub const Stitcher_ERR_CAMERA_PARAMS_ADJUST_FAIL: i32 = 3;
pub const Stitcher_ERR_HOMOGRAPHY_EST_FAIL: i32 = 2;
pub const Stitcher_ERR_NEED_MORE_IMGS: i32 = 1;
pub const Stitcher_OK: i32 = 0;
pub const Stitcher_ORIG_RESOL: i32 = -1;
pub const Stitcher_PANORAMA: i32 = 0;
pub const Stitcher_SCANS: i32 = 1;

///
/// ## C++ default parameters
/// * try_use_gpu: false
pub fn create_stitcher_scans(try_use_gpu: bool) -> Result<types::PtrOfStitcher> {
    unsafe { sys::cv_createStitcherScans_bool(try_use_gpu) }.into_result().map(|x| types::PtrOfStitcher { ptr: x })
}

///
/// ## C++ default parameters
/// * try_use_gpu: false
pub fn create_stitcher(try_use_gpu: bool) -> Result<types::PtrOfStitcher> {
    unsafe { sys::cv_createStitcher_bool(try_use_gpu) }.into_result().map(|x| types::PtrOfStitcher { ptr: x })
}

// boxed class cv::AffineWarper
/// Affine warper factory class.
/// ## See also
/// detail::AffineWarper
#[allow(dead_code)]
pub struct AffineWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::stitching::AffineWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_AffineWarper_delete(self.ptr) };
    }
}
impl crate::stitching::AffineWarper {
    #[doc(hidden)] pub fn as_raw_AffineWarper(&self) -> *mut c_void { self.ptr }
}

impl crate::stitching::WarperCreator for AffineWarper {
    #[doc(hidden)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

impl AffineWarper {

}

// boxed class cv::CompressedRectilinearPortraitWarper
#[allow(dead_code)]
pub struct CompressedRectilinearPortraitWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::stitching::CompressedRectilinearPortraitWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_CompressedRectilinearPortraitWarper_delete(self.ptr) };
    }
}
impl crate::stitching::CompressedRectilinearPortraitWarper {
    #[doc(hidden)] pub fn as_raw_CompressedRectilinearPortraitWarper(&self) -> *mut c_void { self.ptr }
}

impl crate::stitching::WarperCreator for CompressedRectilinearPortraitWarper {
    #[doc(hidden)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

impl CompressedRectilinearPortraitWarper {

    ///
    /// ## C++ default parameters
    /// * a: 1
    /// * b: 1
    pub fn new(a: f32, b: f32) -> Result<crate::stitching::CompressedRectilinearPortraitWarper> {
        unsafe { sys::cv_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper_float_float(a, b) }.into_result().map(|x| crate::stitching::CompressedRectilinearPortraitWarper { ptr: x })
    }
    
}

// boxed class cv::CompressedRectilinearWarper
#[allow(dead_code)]
pub struct CompressedRectilinearWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::stitching::CompressedRectilinearWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_CompressedRectilinearWarper_delete(self.ptr) };
    }
}
impl crate::stitching::CompressedRectilinearWarper {
    #[doc(hidden)] pub fn as_raw_CompressedRectilinearWarper(&self) -> *mut c_void { self.ptr }
}

impl crate::stitching::WarperCreator for CompressedRectilinearWarper {
    #[doc(hidden)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

impl CompressedRectilinearWarper {

    ///
    /// ## C++ default parameters
    /// * a: 1
    /// * b: 1
    pub fn new(a: f32, b: f32) -> Result<crate::stitching::CompressedRectilinearWarper> {
        unsafe { sys::cv_CompressedRectilinearWarper_CompressedRectilinearWarper_float_float(a, b) }.into_result().map(|x| crate::stitching::CompressedRectilinearWarper { ptr: x })
    }
    
}

// boxed class cv::CylindricalWarper
/// Cylindrical warper factory class.
/// ## See also
/// detail::CylindricalWarper
#[allow(dead_code)]
pub struct CylindricalWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::stitching::CylindricalWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_CylindricalWarper_delete(self.ptr) };
    }
}
impl crate::stitching::CylindricalWarper {
    #[doc(hidden)] pub fn as_raw_CylindricalWarper(&self) -> *mut c_void { self.ptr }
}

impl crate::stitching::WarperCreator for CylindricalWarper {
    #[doc(hidden)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

impl CylindricalWarper {

}

// boxed class cv::FisheyeWarper
#[allow(dead_code)]
pub struct FisheyeWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::stitching::FisheyeWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_FisheyeWarper_delete(self.ptr) };
    }
}
impl crate::stitching::FisheyeWarper {
    #[doc(hidden)] pub fn as_raw_FisheyeWarper(&self) -> *mut c_void { self.ptr }
}

impl crate::stitching::WarperCreator for FisheyeWarper {
    #[doc(hidden)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

impl FisheyeWarper {

}

// boxed class cv::MercatorWarper
#[allow(dead_code)]
pub struct MercatorWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::stitching::MercatorWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_MercatorWarper_delete(self.ptr) };
    }
}
impl crate::stitching::MercatorWarper {
    #[doc(hidden)] pub fn as_raw_MercatorWarper(&self) -> *mut c_void { self.ptr }
}

impl crate::stitching::WarperCreator for MercatorWarper {
    #[doc(hidden)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

impl MercatorWarper {

}

// boxed class cv::PaniniPortraitWarper
#[allow(dead_code)]
pub struct PaniniPortraitWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::stitching::PaniniPortraitWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_PaniniPortraitWarper_delete(self.ptr) };
    }
}
impl crate::stitching::PaniniPortraitWarper {
    #[doc(hidden)] pub fn as_raw_PaniniPortraitWarper(&self) -> *mut c_void { self.ptr }
}

impl crate::stitching::WarperCreator for PaniniPortraitWarper {
    #[doc(hidden)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

impl PaniniPortraitWarper {

    ///
    /// ## C++ default parameters
    /// * a: 1
    /// * b: 1
    pub fn new(a: f32, b: f32) -> Result<crate::stitching::PaniniPortraitWarper> {
        unsafe { sys::cv_PaniniPortraitWarper_PaniniPortraitWarper_float_float(a, b) }.into_result().map(|x| crate::stitching::PaniniPortraitWarper { ptr: x })
    }
    
}

// boxed class cv::PaniniWarper
#[allow(dead_code)]
pub struct PaniniWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::stitching::PaniniWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_PaniniWarper_delete(self.ptr) };
    }
}
impl crate::stitching::PaniniWarper {
    #[doc(hidden)] pub fn as_raw_PaniniWarper(&self) -> *mut c_void { self.ptr }
}

impl crate::stitching::WarperCreator for PaniniWarper {
    #[doc(hidden)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

impl PaniniWarper {

    ///
    /// ## C++ default parameters
    /// * a: 1
    /// * b: 1
    pub fn new(a: f32, b: f32) -> Result<crate::stitching::PaniniWarper> {
        unsafe { sys::cv_PaniniWarper_PaniniWarper_float_float(a, b) }.into_result().map(|x| crate::stitching::PaniniWarper { ptr: x })
    }
    
}

// boxed class cv::PlaneWarper
/// Plane warper factory class.
/// ## See also
/// detail::PlaneWarper
#[allow(dead_code)]
pub struct PlaneWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::stitching::PlaneWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_PlaneWarper_delete(self.ptr) };
    }
}
impl crate::stitching::PlaneWarper {
    #[doc(hidden)] pub fn as_raw_PlaneWarper(&self) -> *mut c_void { self.ptr }
}

impl crate::stitching::WarperCreator for PlaneWarper {
    #[doc(hidden)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

impl PlaneWarper {

}

// boxed class cv::SphericalWarper
/// Spherical warper factory class
#[allow(dead_code)]
pub struct SphericalWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::stitching::SphericalWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_SphericalWarper_delete(self.ptr) };
    }
}
impl crate::stitching::SphericalWarper {
    #[doc(hidden)] pub fn as_raw_SphericalWarper(&self) -> *mut c_void { self.ptr }
}

impl crate::stitching::WarperCreator for SphericalWarper {
    #[doc(hidden)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

impl SphericalWarper {

}

// boxed class cv::StereographicWarper
#[allow(dead_code)]
pub struct StereographicWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::stitching::StereographicWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_StereographicWarper_delete(self.ptr) };
    }
}
impl crate::stitching::StereographicWarper {
    #[doc(hidden)] pub fn as_raw_StereographicWarper(&self) -> *mut c_void { self.ptr }
}

impl crate::stitching::WarperCreator for StereographicWarper {
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
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::stitching::Stitcher {
    fn drop(&mut self) {
        unsafe { sys::cv_Stitcher_delete(self.ptr) };
    }
}
impl crate::stitching::Stitcher {
    #[doc(hidden)] pub fn as_raw_Stitcher(&self) -> *mut c_void { self.ptr }
}

impl Stitcher {

    /// Creates a stitcher with the default parameters.
    /// 
    /// ## Parameters
    /// * try_use_gpu: Flag indicating whether GPU should be used whenever it's possible.
    /// ## Returns
    /// Stitcher class instance.
    ///
    /// ## C++ default parameters
    /// * try_use_gpu: false
    pub fn create_default(try_use_gpu: bool) -> Result<crate::stitching::Stitcher> {
        unsafe { sys::cv_Stitcher_createDefault_bool(try_use_gpu) }.into_result().map(|x| crate::stitching::Stitcher { ptr: x })
    }
    
    pub fn registration_resol(&self) -> Result<f64> {
        unsafe { sys::cv_Stitcher_registrationResol_const(self.as_raw_Stitcher()) }.into_result()
    }
    
    pub fn set_registration_resol(&mut self, resol_mpx: f64) -> Result<()> {
        unsafe { sys::cv_Stitcher_setRegistrationResol_double(self.as_raw_Stitcher(), resol_mpx) }.into_result()
    }
    
    pub fn seam_estimation_resol(&self) -> Result<f64> {
        unsafe { sys::cv_Stitcher_seamEstimationResol_const(self.as_raw_Stitcher()) }.into_result()
    }
    
    pub fn set_seam_estimation_resol(&mut self, resol_mpx: f64) -> Result<()> {
        unsafe { sys::cv_Stitcher_setSeamEstimationResol_double(self.as_raw_Stitcher(), resol_mpx) }.into_result()
    }
    
    pub fn compositing_resol(&self) -> Result<f64> {
        unsafe { sys::cv_Stitcher_compositingResol_const(self.as_raw_Stitcher()) }.into_result()
    }
    
    pub fn set_compositing_resol(&mut self, resol_mpx: f64) -> Result<()> {
        unsafe { sys::cv_Stitcher_setCompositingResol_double(self.as_raw_Stitcher(), resol_mpx) }.into_result()
    }
    
    pub fn pano_confidence_thresh(&self) -> Result<f64> {
        unsafe { sys::cv_Stitcher_panoConfidenceThresh_const(self.as_raw_Stitcher()) }.into_result()
    }
    
    pub fn set_pano_confidence_thresh(&mut self, conf_thresh: f64) -> Result<()> {
        unsafe { sys::cv_Stitcher_setPanoConfidenceThresh_double(self.as_raw_Stitcher(), conf_thresh) }.into_result()
    }
    
    pub fn wave_correction(&self) -> Result<bool> {
        unsafe { sys::cv_Stitcher_waveCorrection_const(self.as_raw_Stitcher()) }.into_result()
    }
    
    pub fn set_wave_correction(&mut self, flag: bool) -> Result<()> {
        unsafe { sys::cv_Stitcher_setWaveCorrection_bool(self.as_raw_Stitcher(), flag) }.into_result()
    }
    
    pub fn component(&self) -> Result<types::VectorOfint> {
        unsafe { sys::cv_Stitcher_component_const(self.as_raw_Stitcher()) }.into_result().map(|x| types::VectorOfint { ptr: x })
    }
    
    pub fn work_scale(&self) -> Result<f64> {
        unsafe { sys::cv_Stitcher_workScale_const(self.as_raw_Stitcher()) }.into_result()
    }
    
}

// boxed class cv::TransverseMercatorWarper
#[allow(dead_code)]
pub struct TransverseMercatorWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::stitching::TransverseMercatorWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_TransverseMercatorWarper_delete(self.ptr) };
    }
}
impl crate::stitching::TransverseMercatorWarper {
    #[doc(hidden)] pub fn as_raw_TransverseMercatorWarper(&self) -> *mut c_void { self.ptr }
}

impl crate::stitching::WarperCreator for TransverseMercatorWarper {
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

