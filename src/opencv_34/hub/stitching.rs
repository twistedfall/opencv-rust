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

#[repr(C)]
#[derive(Debug)]
pub enum Stitcher_Mode {
    PANORAMA = Stitcher_PANORAMA as isize,
    SCANS = Stitcher_SCANS as isize,
}

#[repr(C)]
#[derive(Debug)]
pub enum Stitcher_Status {
    OK = Stitcher_OK as isize,
    ERR_NEED_MORE_IMGS = Stitcher_ERR_NEED_MORE_IMGS as isize,
    ERR_HOMOGRAPHY_EST_FAIL = Stitcher_ERR_HOMOGRAPHY_EST_FAIL as isize,
    ERR_CAMERA_PARAMS_ADJUST_FAIL = Stitcher_ERR_CAMERA_PARAMS_ADJUST_FAIL as isize,
}

///
/// ## C++ default parameters
/// * try_use_gpu: false
pub fn create_stitcher_scans(try_use_gpu: bool) -> Result<types::PtrOfStitcher> {
    unsafe { sys::cv_createStitcherScans_bool(try_use_gpu) }.into_result().map(|ptr| types::PtrOfStitcher { ptr })
}

///
/// ## C++ default parameters
/// * try_use_gpu: false
pub fn create_stitcher(try_use_gpu: bool) -> Result<types::PtrOfStitcher> {
    unsafe { sys::cv_createStitcher_bool(try_use_gpu) }.into_result().map(|ptr| types::PtrOfStitcher { ptr })
}

// boxed class cv::AffineWarper
/// Affine warper factory class.
/// ## See also
/// detail::AffineWarper
pub struct AffineWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::stitching::AffineWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_AffineWarper_delete(self.ptr) };
    }
}
impl crate::stitching::AffineWarper {
    #[inline(always)] pub fn as_raw_AffineWarper(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for AffineWarper {}

impl crate::stitching::WarperCreator for AffineWarper {
    #[inline(always)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

impl AffineWarper {

}

// boxed class cv::CompressedRectilinearPortraitWarper
pub struct CompressedRectilinearPortraitWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::stitching::CompressedRectilinearPortraitWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_CompressedRectilinearPortraitWarper_delete(self.ptr) };
    }
}
impl crate::stitching::CompressedRectilinearPortraitWarper {
    #[inline(always)] pub fn as_raw_CompressedRectilinearPortraitWarper(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for CompressedRectilinearPortraitWarper {}

impl crate::stitching::WarperCreator for CompressedRectilinearPortraitWarper {
    #[inline(always)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

impl CompressedRectilinearPortraitWarper {

    ///
    /// ## C++ default parameters
    /// * a: 1
    /// * b: 1
    pub fn new(a: f32, b: f32) -> Result<crate::stitching::CompressedRectilinearPortraitWarper> {
        unsafe { sys::cv_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper_float_float(a, b) }.into_result().map(|ptr| crate::stitching::CompressedRectilinearPortraitWarper { ptr })
    }
    
}

// boxed class cv::CompressedRectilinearWarper
pub struct CompressedRectilinearWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::stitching::CompressedRectilinearWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_CompressedRectilinearWarper_delete(self.ptr) };
    }
}
impl crate::stitching::CompressedRectilinearWarper {
    #[inline(always)] pub fn as_raw_CompressedRectilinearWarper(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for CompressedRectilinearWarper {}

impl crate::stitching::WarperCreator for CompressedRectilinearWarper {
    #[inline(always)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

impl CompressedRectilinearWarper {

    ///
    /// ## C++ default parameters
    /// * a: 1
    /// * b: 1
    pub fn new(a: f32, b: f32) -> Result<crate::stitching::CompressedRectilinearWarper> {
        unsafe { sys::cv_CompressedRectilinearWarper_CompressedRectilinearWarper_float_float(a, b) }.into_result().map(|ptr| crate::stitching::CompressedRectilinearWarper { ptr })
    }
    
}

// boxed class cv::CylindricalWarper
/// Cylindrical warper factory class.
/// ## See also
/// detail::CylindricalWarper
pub struct CylindricalWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::stitching::CylindricalWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_CylindricalWarper_delete(self.ptr) };
    }
}
impl crate::stitching::CylindricalWarper {
    #[inline(always)] pub fn as_raw_CylindricalWarper(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for CylindricalWarper {}

impl crate::stitching::WarperCreator for CylindricalWarper {
    #[inline(always)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

impl CylindricalWarper {

}

// boxed class cv::FisheyeWarper
pub struct FisheyeWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::stitching::FisheyeWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_FisheyeWarper_delete(self.ptr) };
    }
}
impl crate::stitching::FisheyeWarper {
    #[inline(always)] pub fn as_raw_FisheyeWarper(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for FisheyeWarper {}

impl crate::stitching::WarperCreator for FisheyeWarper {
    #[inline(always)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

impl FisheyeWarper {

}

// boxed class cv::MercatorWarper
pub struct MercatorWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::stitching::MercatorWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_MercatorWarper_delete(self.ptr) };
    }
}
impl crate::stitching::MercatorWarper {
    #[inline(always)] pub fn as_raw_MercatorWarper(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for MercatorWarper {}

impl crate::stitching::WarperCreator for MercatorWarper {
    #[inline(always)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

impl MercatorWarper {

}

// boxed class cv::PaniniPortraitWarper
pub struct PaniniPortraitWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::stitching::PaniniPortraitWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_PaniniPortraitWarper_delete(self.ptr) };
    }
}
impl crate::stitching::PaniniPortraitWarper {
    #[inline(always)] pub fn as_raw_PaniniPortraitWarper(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for PaniniPortraitWarper {}

impl crate::stitching::WarperCreator for PaniniPortraitWarper {
    #[inline(always)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

impl PaniniPortraitWarper {

    ///
    /// ## C++ default parameters
    /// * a: 1
    /// * b: 1
    pub fn new(a: f32, b: f32) -> Result<crate::stitching::PaniniPortraitWarper> {
        unsafe { sys::cv_PaniniPortraitWarper_PaniniPortraitWarper_float_float(a, b) }.into_result().map(|ptr| crate::stitching::PaniniPortraitWarper { ptr })
    }
    
}

// boxed class cv::PaniniWarper
pub struct PaniniWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::stitching::PaniniWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_PaniniWarper_delete(self.ptr) };
    }
}
impl crate::stitching::PaniniWarper {
    #[inline(always)] pub fn as_raw_PaniniWarper(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for PaniniWarper {}

impl crate::stitching::WarperCreator for PaniniWarper {
    #[inline(always)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

impl PaniniWarper {

    ///
    /// ## C++ default parameters
    /// * a: 1
    /// * b: 1
    pub fn new(a: f32, b: f32) -> Result<crate::stitching::PaniniWarper> {
        unsafe { sys::cv_PaniniWarper_PaniniWarper_float_float(a, b) }.into_result().map(|ptr| crate::stitching::PaniniWarper { ptr })
    }
    
}

// boxed class cv::PlaneWarper
/// Plane warper factory class.
/// ## See also
/// detail::PlaneWarper
pub struct PlaneWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::stitching::PlaneWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_PlaneWarper_delete(self.ptr) };
    }
}
impl crate::stitching::PlaneWarper {
    #[inline(always)] pub fn as_raw_PlaneWarper(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for PlaneWarper {}

impl crate::stitching::WarperCreator for PlaneWarper {
    #[inline(always)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

impl PlaneWarper {

}

// boxed class cv::SphericalWarper
/// Spherical warper factory class
pub struct SphericalWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::stitching::SphericalWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_SphericalWarper_delete(self.ptr) };
    }
}
impl crate::stitching::SphericalWarper {
    #[inline(always)] pub fn as_raw_SphericalWarper(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for SphericalWarper {}

impl crate::stitching::WarperCreator for SphericalWarper {
    #[inline(always)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

impl SphericalWarper {

}

// boxed class cv::StereographicWarper
pub struct StereographicWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::stitching::StereographicWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_StereographicWarper_delete(self.ptr) };
    }
}
impl crate::stitching::StereographicWarper {
    #[inline(always)] pub fn as_raw_StereographicWarper(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for StereographicWarper {}

impl crate::stitching::WarperCreator for StereographicWarper {
    #[inline(always)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
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
pub struct Stitcher {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::stitching::Stitcher {
    fn drop(&mut self) {
        unsafe { sys::cv_Stitcher_delete(self.ptr) };
    }
}
impl crate::stitching::Stitcher {
    #[inline(always)] pub fn as_raw_Stitcher(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for Stitcher {}

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
        unsafe { sys::cv_Stitcher_createDefault_bool(try_use_gpu) }.into_result().map(|ptr| crate::stitching::Stitcher { ptr })
    }
    
    /// Creates a Stitcher configured in one of the stitching modes.
    ///
    /// ## Parameters
    /// * mode: Scenario for stitcher operation. This is usually determined by source of images
    /// to stitch and their transformation. Default parameters will be chosen for operation in given
    /// scenario.
    /// * try_use_gpu: Flag indicating whether GPU should be used whenever it's possible.
    /// ## Returns
    /// Stitcher class instance.
    ///
    /// ## C++ default parameters
    /// * mode: PANORAMA
    /// * try_use_gpu: false
    pub fn create(mode: crate::stitching::Stitcher_Mode, try_use_gpu: bool) -> Result<types::PtrOfStitcher> {
        unsafe { sys::cv_Stitcher_create_Stitcher_Mode_bool(mode, try_use_gpu) }.into_result().map(|ptr| types::PtrOfStitcher { ptr })
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
    
    pub fn estimate_transform(&mut self, images: &types::VectorOfMat) -> Result<crate::stitching::Stitcher_Status> {
        unsafe { sys::cv_Stitcher_estimateTransform_VectorOfMat(self.as_raw_Stitcher(), images.as_raw_VectorOfMat()) }.into_result()
    }
    
    /// These functions try to match the given images and to estimate rotations of each camera.
    ///
    ///
    /// Note: Use the functions only if you're aware of the stitching pipeline, otherwise use
    /// Stitcher::stitch.
    ///
    /// ## Parameters
    /// * images: Input images.
    /// * rois: Region of interest rectangles.
    /// ## Returns
    /// Status code.
    pub fn estimate_transform_1(&mut self, images: &types::VectorOfMat, rois: &types::VectorOfVectorOfRect) -> Result<crate::stitching::Stitcher_Status> {
        unsafe { sys::cv_Stitcher_estimateTransform_VectorOfMat_VectorOfVectorOfRect(self.as_raw_Stitcher(), images.as_raw_VectorOfMat(), rois.as_raw_VectorOfVectorOfRect()) }.into_result()
    }
    
    pub fn compose_panorama(&mut self, pano: &mut core::Mat) -> Result<crate::stitching::Stitcher_Status> {
        unsafe { sys::cv_Stitcher_composePanorama_Mat(self.as_raw_Stitcher(), pano.as_raw_Mat()) }.into_result()
    }
    
    /// These functions try to compose the given images (or images stored internally from the other function
    /// calls) into the final pano under the assumption that the image transformations were estimated
    /// before.
    ///
    ///
    /// Note: Use the functions only if you're aware of the stitching pipeline, otherwise use
    /// Stitcher::stitch.
    ///
    /// ## Parameters
    /// * images: Input images.
    /// * pano: Final pano.
    /// ## Returns
    /// Status code.
    pub fn compose_panorama_1(&mut self, images: &types::VectorOfMat, pano: &mut core::Mat) -> Result<crate::stitching::Stitcher_Status> {
        unsafe { sys::cv_Stitcher_composePanorama_VectorOfMat_Mat(self.as_raw_Stitcher(), images.as_raw_VectorOfMat(), pano.as_raw_Mat()) }.into_result()
    }
    
    pub fn stitch(&mut self, images: &types::VectorOfMat, pano: &mut core::Mat) -> Result<crate::stitching::Stitcher_Status> {
        unsafe { sys::cv_Stitcher_stitch_VectorOfMat_Mat(self.as_raw_Stitcher(), images.as_raw_VectorOfMat(), pano.as_raw_Mat()) }.into_result()
    }
    
    /// These functions try to stitch the given images.
    ///
    /// ## Parameters
    /// * images: Input images.
    /// * rois: Region of interest rectangles.
    /// * pano: Final pano.
    /// ## Returns
    /// Status code.
    pub fn stitch_1(&mut self, images: &types::VectorOfMat, rois: &types::VectorOfVectorOfRect, pano: &mut core::Mat) -> Result<crate::stitching::Stitcher_Status> {
        unsafe { sys::cv_Stitcher_stitch_VectorOfMat_VectorOfVectorOfRect_Mat(self.as_raw_Stitcher(), images.as_raw_VectorOfMat(), rois.as_raw_VectorOfVectorOfRect(), pano.as_raw_Mat()) }.into_result()
    }
    
    pub fn component(&self) -> Result<types::VectorOfint> {
        unsafe { sys::cv_Stitcher_component_const(self.as_raw_Stitcher()) }.into_result().map(|ptr| types::VectorOfint { ptr })
    }
    
    pub fn work_scale(&self) -> Result<f64> {
        unsafe { sys::cv_Stitcher_workScale_const(self.as_raw_Stitcher()) }.into_result()
    }
    
}

// boxed class cv::TransverseMercatorWarper
pub struct TransverseMercatorWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::stitching::TransverseMercatorWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_TransverseMercatorWarper_delete(self.ptr) };
    }
}
impl crate::stitching::TransverseMercatorWarper {
    #[inline(always)] pub fn as_raw_TransverseMercatorWarper(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for TransverseMercatorWarper {}

impl crate::stitching::WarperCreator for TransverseMercatorWarper {
    #[inline(always)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

impl TransverseMercatorWarper {

}

// Generating impl for trait cv::WarperCreator (trait)
/// Image warper factories base class.
pub trait WarperCreator {
    #[inline(always)] fn as_raw_WarperCreator(&self) -> *mut c_void;
}

