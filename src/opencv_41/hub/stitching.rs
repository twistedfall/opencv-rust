//! # Images stitching
//!
//! This figure illustrates the stitching module pipeline implemented in the Stitcher class. Using that
//! class it's possible to configure/remove some steps, i.e. adjust the stitching pipeline according to
//! the particular needs. All building blocks from the pipeline are available in the detail namespace,
//! one can combine and use them separately.
//!
//! The implemented stitching pipeline is very similar to the one proposed in [BL07](https://docs.opencv.org/4.1.2/d0/de3/citelist.html#CITEREF_BL07) .
//!
//! ![stitching pipeline](https://docs.opencv.org/4.1.2/StitchingPipeline.jpg)
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
use libc::{ptrdiff_t, size_t};
use crate::{Error, Result, core, sys, types};
use crate::core::{_InputArrayTrait, _OutputArrayTrait};

pub const Stitcher_ERR_CAMERA_PARAMS_ADJUST_FAIL: i32 = 3;
pub const Stitcher_ERR_HOMOGRAPHY_EST_FAIL: i32 = 2;
pub const Stitcher_ERR_NEED_MORE_IMGS: i32 = 1;
pub const Stitcher_OK: i32 = 0;
pub const Stitcher_PANORAMA: i32 = 0;
pub const Stitcher_SCANS: i32 = 1;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Stitcher_Mode {
    PANORAMA = Stitcher_PANORAMA as isize,
    SCANS = Stitcher_SCANS as isize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Stitcher_Status {
    OK = Stitcher_OK as isize,
    ERR_NEED_MORE_IMGS = Stitcher_ERR_NEED_MORE_IMGS as isize,
    ERR_HOMOGRAPHY_EST_FAIL = Stitcher_ERR_HOMOGRAPHY_EST_FAIL as isize,
    ERR_CAMERA_PARAMS_ADJUST_FAIL = Stitcher_ERR_CAMERA_PARAMS_ADJUST_FAIL as isize,
}

/// **Deprecated**: use Stitcher::create
///
/// ## C++ default parameters
/// * try_use_gpu: false
#[deprecated = "use Stitcher::create"]
pub fn create_stitcher_scans(try_use_gpu: bool) -> Result<types::PtrOfStitcher> {
    unsafe { sys::cv_createStitcherScans_bool(try_use_gpu) }.into_result().map(|ptr| types::PtrOfStitcher { ptr })
}

/// **Deprecated**: use Stitcher::create
///
/// ## C++ default parameters
/// * try_use_gpu: false
#[deprecated = "use Stitcher::create"]
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

impl Drop for AffineWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_AffineWarper_delete(self.ptr) };
    }
}

impl AffineWarper {
    #[inline(always)] pub fn as_raw_AffineWarper(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for AffineWarper {}

impl crate::stitching::WarperCreator for AffineWarper {
    #[inline(always)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

// boxed class cv::CompressedRectilinearPortraitWarper
pub struct CompressedRectilinearPortraitWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for CompressedRectilinearPortraitWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_CompressedRectilinearPortraitWarper_delete(self.ptr) };
    }
}

impl CompressedRectilinearPortraitWarper {
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

impl Drop for CompressedRectilinearWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_CompressedRectilinearWarper_delete(self.ptr) };
    }
}

impl CompressedRectilinearWarper {
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

impl Drop for CylindricalWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_CylindricalWarper_delete(self.ptr) };
    }
}

impl CylindricalWarper {
    #[inline(always)] pub fn as_raw_CylindricalWarper(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for CylindricalWarper {}

impl crate::stitching::WarperCreator for CylindricalWarper {
    #[inline(always)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

// boxed class cv::FisheyeWarper
pub struct FisheyeWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for FisheyeWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_FisheyeWarper_delete(self.ptr) };
    }
}

impl FisheyeWarper {
    #[inline(always)] pub fn as_raw_FisheyeWarper(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for FisheyeWarper {}

impl crate::stitching::WarperCreator for FisheyeWarper {
    #[inline(always)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

// boxed class cv::MercatorWarper
pub struct MercatorWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for MercatorWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_MercatorWarper_delete(self.ptr) };
    }
}

impl MercatorWarper {
    #[inline(always)] pub fn as_raw_MercatorWarper(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for MercatorWarper {}

impl crate::stitching::WarperCreator for MercatorWarper {
    #[inline(always)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

// boxed class cv::PaniniPortraitWarper
pub struct PaniniPortraitWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for PaniniPortraitWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_PaniniPortraitWarper_delete(self.ptr) };
    }
}

impl PaniniPortraitWarper {
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

impl Drop for PaniniWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_PaniniWarper_delete(self.ptr) };
    }
}

impl PaniniWarper {
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

impl Drop for PlaneWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_PlaneWarper_delete(self.ptr) };
    }
}

impl PlaneWarper {
    #[inline(always)] pub fn as_raw_PlaneWarper(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for PlaneWarper {}

impl crate::stitching::WarperCreator for PlaneWarper {
    #[inline(always)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

// boxed class cv::PyRotationWarper
pub struct PyRotationWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for PyRotationWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_PyRotationWarper_delete(self.ptr) };
    }
}

impl PyRotationWarper {
    #[inline(always)] pub fn as_raw_PyRotationWarper(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for PyRotationWarper {}

impl PyRotationWarper {
    pub fn new(_type: &str, scale: f32) -> Result<crate::stitching::PyRotationWarper> {
        string_arg!(mut _type);
        unsafe { sys::cv_PyRotationWarper_PyRotationWarper_String_float(_type.as_ptr() as _, scale) }.into_result().map(|ptr| crate::stitching::PyRotationWarper { ptr })
    }
    
    pub fn default() -> Result<crate::stitching::PyRotationWarper> {
        unsafe { sys::cv_PyRotationWarper_PyRotationWarper() }.into_result().map(|ptr| crate::stitching::PyRotationWarper { ptr })
    }
    
    /// Projects the image point.
    ///
    /// ## Parameters
    /// * pt: Source point
    /// * K: Camera intrinsic parameters
    /// * R: Camera rotation matrix
    /// ## Returns
    /// Projected point
    pub fn warp_point(&mut self, pt: core::Point2f, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray) -> Result<core::Point2f> {
        input_array_arg!(k);
        input_array_arg!(r);
        unsafe { sys::cv_PyRotationWarper_warpPoint_Point2f__InputArray__InputArray(self.as_raw_PyRotationWarper(), pt, k.as_raw__InputArray(), r.as_raw__InputArray()) }.into_result()
    }
    
    /// Builds the projection maps according to the given camera data.
    ///
    /// ## Parameters
    /// * src_size: Source image size
    /// * K: Camera intrinsic parameters
    /// * R: Camera rotation matrix
    /// * xmap: Projection map for the x axis
    /// * ymap: Projection map for the y axis
    /// ## Returns
    /// Projected image minimum bounding box
    pub fn build_maps(&mut self, src_size: core::Size, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, xmap: &mut dyn core::ToOutputArray, ymap: &mut dyn core::ToOutputArray) -> Result<core::Rect> {
        input_array_arg!(k);
        input_array_arg!(r);
        output_array_arg!(xmap);
        output_array_arg!(ymap);
        unsafe { sys::cv_PyRotationWarper_buildMaps_Size__InputArray__InputArray__OutputArray__OutputArray(self.as_raw_PyRotationWarper(), src_size, k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray()) }.into_result()
    }
    
    /// Projects the image.
    ///
    /// ## Parameters
    /// * src: Source image
    /// * K: Camera intrinsic parameters
    /// * R: Camera rotation matrix
    /// * interp_mode: Interpolation mode
    /// * border_mode: Border extrapolation mode
    /// * dst: Projected image
    /// ## Returns
    /// Project image top-left corner
    pub fn warp(&mut self, src: &dyn core::ToInputArray, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut dyn core::ToOutputArray) -> Result<core::Point> {
        input_array_arg!(src);
        input_array_arg!(k);
        input_array_arg!(r);
        output_array_arg!(dst);
        unsafe { sys::cv_PyRotationWarper_warp__InputArray__InputArray__InputArray_int_int__OutputArray(self.as_raw_PyRotationWarper(), src.as_raw__InputArray(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw__OutputArray()) }.into_result()
    }
    
    /// Projects the image backward.
    ///
    /// ## Parameters
    /// * src: Projected image
    /// * K: Camera intrinsic parameters
    /// * R: Camera rotation matrix
    /// * interp_mode: Interpolation mode
    /// * border_mode: Border extrapolation mode
    /// * dst_size: Backward-projected image size
    /// * dst: Backward-projected image
    pub fn warp_backward(&mut self, src: &dyn core::ToInputArray, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, interp_mode: i32, border_mode: i32, dst_size: core::Size, dst: &mut dyn core::ToOutputArray) -> Result<()> {
        input_array_arg!(src);
        input_array_arg!(k);
        input_array_arg!(r);
        output_array_arg!(dst);
        unsafe { sys::cv_PyRotationWarper_warpBackward__InputArray__InputArray__InputArray_int_int_Size__OutputArray(self.as_raw_PyRotationWarper(), src.as_raw__InputArray(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst_size, dst.as_raw__OutputArray()) }.into_result()
    }
    
    /// ## Parameters
    /// * src_size: Source image bounding box
    /// * K: Camera intrinsic parameters
    /// * R: Camera rotation matrix
    /// ## Returns
    /// Projected image minimum bounding box
    pub fn warp_roi(&mut self, src_size: core::Size, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray) -> Result<core::Rect> {
        input_array_arg!(k);
        input_array_arg!(r);
        unsafe { sys::cv_PyRotationWarper_warpRoi_Size__InputArray__InputArray(self.as_raw_PyRotationWarper(), src_size, k.as_raw__InputArray(), r.as_raw__InputArray()) }.into_result()
    }
    
    pub fn get_scale(&self) -> Result<f32> {
        unsafe { sys::cv_PyRotationWarper_getScale_const(self.as_raw_PyRotationWarper()) }.into_result()
    }
    
    pub fn set_scale(&mut self, unnamed_arg: f32) -> Result<()> {
        unsafe { sys::cv_PyRotationWarper_setScale_float(self.as_raw_PyRotationWarper(), unnamed_arg) }.into_result()
    }
    
}

// boxed class cv::SphericalWarper
/// Spherical warper factory class
pub struct SphericalWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for SphericalWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_SphericalWarper_delete(self.ptr) };
    }
}

impl SphericalWarper {
    #[inline(always)] pub fn as_raw_SphericalWarper(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for SphericalWarper {}

impl crate::stitching::WarperCreator for SphericalWarper {
    #[inline(always)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

// boxed class cv::StereographicWarper
pub struct StereographicWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for StereographicWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_StereographicWarper_delete(self.ptr) };
    }
}

impl StereographicWarper {
    #[inline(always)] pub fn as_raw_StereographicWarper(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for StereographicWarper {}

impl crate::stitching::WarperCreator for StereographicWarper {
    #[inline(always)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
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
/// *   A basic example on image stitching in Python can be found at
/// opencv_source_code/samples/python/stitching.py
/// *   A detailed example on image stitching can be found at
/// opencv_source_code/samples/cpp/stitching_detailed.cpp
pub struct Stitcher {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for Stitcher {
    fn drop(&mut self) {
        unsafe { sys::cv_Stitcher_delete(self.ptr) };
    }
}

impl Stitcher {
    #[inline(always)] pub fn as_raw_Stitcher(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for Stitcher {}

impl Stitcher {
    /// Creates a Stitcher configured in one of the stitching modes.
    ///
    /// ## Parameters
    /// * mode: Scenario for stitcher operation. This is usually determined by source of images
    /// to stitch and their transformation. Default parameters will be chosen for operation in given
    /// scenario.
    /// ## Returns
    /// Stitcher class instance.
    ///
    /// ## C++ default parameters
    /// * mode: Stitcher::PANORAMA
    pub fn create(mode: crate::stitching::Stitcher_Mode) -> Result<types::PtrOfStitcher> {
        unsafe { sys::cv_Stitcher_create_Stitcher_Mode(mode) }.into_result().map(|ptr| types::PtrOfStitcher { ptr })
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
    
    pub fn interpolation_flags(&self) -> Result<crate::imgproc::InterpolationFlags> {
        unsafe { sys::cv_Stitcher_interpolationFlags_const(self.as_raw_Stitcher()) }.into_result()
    }
    
    pub fn set_interpolation_flags(&mut self, interp_flags: crate::imgproc::InterpolationFlags) -> Result<()> {
        unsafe { sys::cv_Stitcher_setInterpolationFlags_InterpolationFlags(self.as_raw_Stitcher(), interp_flags) }.into_result()
    }
    
    pub fn features_finder(&mut self) -> Result<types::PtrOfFeature2D> {
        unsafe { sys::cv_Stitcher_featuresFinder(self.as_raw_Stitcher()) }.into_result().map(|ptr| types::PtrOfFeature2D { ptr })
    }
    
    pub fn features_finder_1(&self) -> Result<types::PtrOfFeature2D> {
        unsafe { sys::cv_Stitcher_featuresFinder_const(self.as_raw_Stitcher()) }.into_result().map(|ptr| types::PtrOfFeature2D { ptr })
    }
    
    pub fn set_features_finder(&mut self, features_finder: &types::PtrOfFeature2D) -> Result<()> {
        unsafe { sys::cv_Stitcher_setFeaturesFinder_PtrOfFeature2D(self.as_raw_Stitcher(), features_finder.as_raw_PtrOfFeature2D()) }.into_result()
    }
    
    pub fn matching_mask(&self) -> Result<core::UMat> {
        unsafe { sys::cv_Stitcher_matchingMask_const(self.as_raw_Stitcher()) }.into_result().map(|ptr| core::UMat { ptr })
    }
    
    pub fn set_matching_mask(&mut self, mask: &core::UMat) -> Result<()> {
        unsafe { sys::cv_Stitcher_setMatchingMask_UMat(self.as_raw_Stitcher(), mask.as_raw_UMat()) }.into_result()
    }
    
    /// These functions try to match the given images and to estimate rotations of each camera.
    ///
    ///
    /// Note: Use the functions only if you're aware of the stitching pipeline, otherwise use
    /// Stitcher::stitch.
    ///
    /// ## Parameters
    /// * images: Input images.
    /// * masks: Masks for each input image specifying where to look for keypoints (optional).
    /// ## Returns
    /// Status code.
    ///
    /// ## C++ default parameters
    /// * masks: noArray()
    pub fn estimate_transform(&mut self, images: &dyn core::ToInputArray, masks: &dyn core::ToInputArray) -> Result<crate::stitching::Stitcher_Status> {
        input_array_arg!(images);
        input_array_arg!(masks);
        unsafe { sys::cv_Stitcher_estimateTransform__InputArray__InputArray(self.as_raw_Stitcher(), images.as_raw__InputArray(), masks.as_raw__InputArray()) }.into_result()
    }
    
    pub fn compose_panorama(&mut self, pano: &mut dyn core::ToOutputArray) -> Result<crate::stitching::Stitcher_Status> {
        output_array_arg!(pano);
        unsafe { sys::cv_Stitcher_composePanorama__OutputArray(self.as_raw_Stitcher(), pano.as_raw__OutputArray()) }.into_result()
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
    pub fn compose_panorama_images(&mut self, images: &dyn core::ToInputArray, pano: &mut dyn core::ToOutputArray) -> Result<crate::stitching::Stitcher_Status> {
        input_array_arg!(images);
        output_array_arg!(pano);
        unsafe { sys::cv_Stitcher_composePanorama__InputArray__OutputArray(self.as_raw_Stitcher(), images.as_raw__InputArray(), pano.as_raw__OutputArray()) }.into_result()
    }
    
    pub fn stitch(&mut self, images: &dyn core::ToInputArray, pano: &mut dyn core::ToOutputArray) -> Result<crate::stitching::Stitcher_Status> {
        input_array_arg!(images);
        output_array_arg!(pano);
        unsafe { sys::cv_Stitcher_stitch__InputArray__OutputArray(self.as_raw_Stitcher(), images.as_raw__InputArray(), pano.as_raw__OutputArray()) }.into_result()
    }
    
    /// These functions try to stitch the given images.
    ///
    /// ## Parameters
    /// * images: Input images.
    /// * masks: Masks for each input image specifying where to look for keypoints (optional).
    /// * pano: Final pano.
    /// ## Returns
    /// Status code.
    pub fn stitch_mask(&mut self, images: &dyn core::ToInputArray, masks: &dyn core::ToInputArray, pano: &mut dyn core::ToOutputArray) -> Result<crate::stitching::Stitcher_Status> {
        input_array_arg!(images);
        input_array_arg!(masks);
        output_array_arg!(pano);
        unsafe { sys::cv_Stitcher_stitch__InputArray__InputArray__OutputArray(self.as_raw_Stitcher(), images.as_raw__InputArray(), masks.as_raw__InputArray(), pano.as_raw__OutputArray()) }.into_result()
    }
    
    pub fn component(&self) -> Result<types::VectorOfint> {
        unsafe { sys::cv_Stitcher_component_const(self.as_raw_Stitcher()) }.into_result().map(|ptr| types::VectorOfint { ptr })
    }
    
    pub fn work_scale(&self) -> Result<f64> {
        unsafe { sys::cv_Stitcher_workScale_const(self.as_raw_Stitcher()) }.into_result()
    }
    
    pub fn result_mask(&self) -> Result<core::UMat> {
        unsafe { sys::cv_Stitcher_resultMask_const(self.as_raw_Stitcher()) }.into_result().map(|ptr| core::UMat { ptr })
    }
    
}

// boxed class cv::TransverseMercatorWarper
pub struct TransverseMercatorWarper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for TransverseMercatorWarper {
    fn drop(&mut self) {
        unsafe { sys::cv_TransverseMercatorWarper_delete(self.ptr) };
    }
}

impl TransverseMercatorWarper {
    #[inline(always)] pub fn as_raw_TransverseMercatorWarper(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for TransverseMercatorWarper {}

impl crate::stitching::WarperCreator for TransverseMercatorWarper {
    #[inline(always)] fn as_raw_WarperCreator(&self) -> *mut c_void { self.ptr }
}

// Generating impl for trait crate::stitching::WarperCreator
/// Image warper factories base class.
pub trait WarperCreator {
    fn as_raw_WarperCreator(&self) -> *mut c_void;
}

