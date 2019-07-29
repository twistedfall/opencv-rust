//! # Video Stabilization
//!
//! The video stabilization module contains a set of functions and classes that can be used to solve the
//! problem of video stabilization. There are a few methods implemented, most of them are described in
//! the papers [OF06](https://docs.opencv.org/3.4.7/d0/de3/citelist.html#CITEREF_OF06) and [G11](https://docs.opencv.org/3.4.7/d0/de3/citelist.html#CITEREF_G11) . However, there are some extensions and deviations from the original
//! paper methods.
//!
//! ### References
//!
//! 1. "Full-Frame Video Stabilization with Motion Inpainting"
//! Yasuyuki Matsushita, Eyal Ofek, Weina Ge, Xiaoou Tang, Senior Member, and Heung-Yeung Shum
//! 2. "Auto-Directed Video Stabilization with Robust L1 Optimal Camera Paths"
//! Matthias Grundmann, Vivek Kwatra, Irfan Essa
//! # Global Motion Estimation
//!
//! The video stabilization module contains a set of functions and classes for global motion estimation
//! between point clouds or between images. In the last case features are extracted and matched
//! internally. For the sake of convenience the motion estimation functions are wrapped into classes.
//! Both the functions and the classes are available.
//!
//! # Fast Marching Method
//!
//! The Fast Marching Method [Telea04](https://docs.opencv.org/3.4.7/d0/de3/citelist.html#CITEREF_Telea04) is used in of the video stabilization routines to do motion and
//! color inpainting. The method is implemented is a flexible way and it's made public for other users.
use std::os::raw::{c_char, c_void};
use libc::{ptrdiff_t, size_t};
use crate::{Error, Result, core, sys, types};

pub const MM_AFFINE: i32 = 5;
pub const MM_HOMOGRAPHY: i32 = 6;
pub const MM_RIGID: i32 = 3;
pub const MM_ROTATION: i32 = 2;
pub const MM_SIMILARITY: i32 = 4;
pub const MM_TRANSLATION: i32 = 0;
pub const MM_TRANSLATION_AND_SCALE: i32 = 1;
pub const MM_UNKNOWN: i32 = 7;

pub fn calc_blurriness(frame: &core::Mat) -> Result<f32> {
    unsafe { sys::cv_videostab_calcBlurriness_Mat(frame.as_raw_Mat()) }.into_result()
}

pub fn calc_flow_mask(flow_x: &core::Mat, flow_y: &core::Mat, errors: &core::Mat, max_error: f32, mask0: &core::Mat, mask1: &core::Mat, flow_mask: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_videostab_calcFlowMask_Mat_Mat_Mat_float_Mat_Mat_Mat(flow_x.as_raw_Mat(), flow_y.as_raw_Mat(), errors.as_raw_Mat(), max_error, mask0.as_raw_Mat(), mask1.as_raw_Mat(), flow_mask.as_raw_Mat()) }.into_result()
}

pub fn complete_frame_according_to_flow(flow_mask: &core::Mat, flow_x: &core::Mat, flow_y: &core::Mat, frame1: &core::Mat, mask1: &core::Mat, dist_thresh: f32, frame0: &mut core::Mat, mask0: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_videostab_completeFrameAccordingToFlow_Mat_Mat_Mat_Mat_Mat_float_Mat_Mat(flow_mask.as_raw_Mat(), flow_x.as_raw_Mat(), flow_y.as_raw_Mat(), frame1.as_raw_Mat(), mask1.as_raw_Mat(), dist_thresh, frame0.as_raw_Mat(), mask0.as_raw_Mat()) }.into_result()
}

pub fn ensure_inclusion_constraint(m: &core::Mat, size: core::Size, trim_ratio: f32) -> Result<core::Mat> {
    unsafe { sys::cv_videostab_ensureInclusionConstraint_Mat_Size_float(m.as_raw_Mat(), size, trim_ratio) }.into_result().map(|ptr| core::Mat { ptr })
}

/// Estimates best global motion between two 2D point clouds in the least-squares sense.
///
///
/// Note: Works in-place and changes input point arrays.
///
/// ## Parameters
/// * points0: Source set of 2D points (32F).
/// * points1: Destination set of 2D points (32F).
/// * model: Motion model (up to MM_AFFINE).
/// * rmse: Final root-mean-square error.
/// ## Returns
/// 3x3 2D transformation matrix (32F).
///
/// ## C++ default parameters
/// * model: MM_AFFINE
/// * rmse: 0
pub fn estimate_global_motion_least_squares(points0: &mut core::Mat, points1: &mut core::Mat, model: i32, rmse: &mut f32) -> Result<core::Mat> {
    unsafe { sys::cv_videostab_estimateGlobalMotionLeastSquares_Mat_Mat_int_float_X(points0.as_raw_Mat(), points1.as_raw_Mat(), model, rmse) }.into_result().map(|ptr| core::Mat { ptr })
}

pub fn estimate_optimal_trim_ratio(m: &core::Mat, size: core::Size) -> Result<f32> {
    unsafe { sys::cv_videostab_estimateOptimalTrimRatio_Mat_Size(m.as_raw_Mat(), size) }.into_result()
}

/// Computes motion between two frames assuming that all the intermediate motions are known.
///
/// ## Parameters
/// * from: Source frame index.
/// * to: Destination frame index.
/// * motions: Pair-wise motions. motions[i] denotes motion from the frame i to the frame i+1
/// ## Returns
/// Motion from the Source frame to the Destination frame.
pub fn get_motion(from: i32, to: i32, motions: &types::VectorOfMat) -> Result<core::Mat> {
    unsafe { sys::cv_videostab_getMotion_int_int_VectorOfMat(from, to, motions.as_raw_VectorOfMat()) }.into_result().map(|ptr| core::Mat { ptr })
}

// boxed class cv::videostab::ColorAverageInpainter
pub struct ColorAverageInpainter {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::ColorAverageInpainter {
    fn drop(&mut self) {
        unsafe { sys::cv_ColorAverageInpainter_delete(self.ptr) };
    }
}
impl crate::videostab::ColorAverageInpainter {
    #[inline(always)] pub fn as_raw_ColorAverageInpainter(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for ColorAverageInpainter {}

impl crate::videostab::InpainterBase for ColorAverageInpainter {
    #[inline(always)] fn as_raw_InpainterBase(&self) -> *mut c_void { self.ptr }
}

impl ColorAverageInpainter {

    pub fn inpaint(&mut self, idx: i32, frame: &mut core::Mat, mask: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_videostab_ColorAverageInpainter_inpaint_int_Mat_Mat(self.as_raw_ColorAverageInpainter(), idx, frame.as_raw_Mat(), mask.as_raw_Mat()) }.into_result()
    }
    
}

// boxed class cv::videostab::ColorInpainter
pub struct ColorInpainter {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::ColorInpainter {
    fn drop(&mut self) {
        unsafe { sys::cv_ColorInpainter_delete(self.ptr) };
    }
}
impl crate::videostab::ColorInpainter {
    #[inline(always)] pub fn as_raw_ColorInpainter(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for ColorInpainter {}

impl crate::videostab::InpainterBase for ColorInpainter {
    #[inline(always)] fn as_raw_InpainterBase(&self) -> *mut c_void { self.ptr }
}

impl ColorInpainter {

    ///
    /// ## C++ default parameters
    /// * method: INPAINT_TELEA
    /// * radius: 2.
    pub fn new(method: i32, radius: f64) -> Result<crate::videostab::ColorInpainter> {
        unsafe { sys::cv_videostab_ColorInpainter_ColorInpainter_int_double(method, radius) }.into_result().map(|ptr| crate::videostab::ColorInpainter { ptr })
    }
    
    pub fn inpaint(&mut self, idx: i32, frame: &mut core::Mat, mask: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_videostab_ColorInpainter_inpaint_int_Mat_Mat(self.as_raw_ColorInpainter(), idx, frame.as_raw_Mat(), mask.as_raw_Mat()) }.into_result()
    }
    
}

// boxed class cv::videostab::ConsistentMosaicInpainter
pub struct ConsistentMosaicInpainter {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::ConsistentMosaicInpainter {
    fn drop(&mut self) {
        unsafe { sys::cv_ConsistentMosaicInpainter_delete(self.ptr) };
    }
}
impl crate::videostab::ConsistentMosaicInpainter {
    #[inline(always)] pub fn as_raw_ConsistentMosaicInpainter(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for ConsistentMosaicInpainter {}

impl crate::videostab::InpainterBase for ConsistentMosaicInpainter {
    #[inline(always)] fn as_raw_InpainterBase(&self) -> *mut c_void { self.ptr }
}

impl ConsistentMosaicInpainter {

    pub fn default() -> Result<crate::videostab::ConsistentMosaicInpainter> {
        unsafe { sys::cv_videostab_ConsistentMosaicInpainter_ConsistentMosaicInpainter() }.into_result().map(|ptr| crate::videostab::ConsistentMosaicInpainter { ptr })
    }
    
    pub fn set_stdev_thresh(&mut self, val: f32) -> Result<()> {
        unsafe { sys::cv_videostab_ConsistentMosaicInpainter_setStdevThresh_float(self.as_raw_ConsistentMosaicInpainter(), val) }.into_result()
    }
    
    pub fn stdev_thresh(&self) -> Result<f32> {
        unsafe { sys::cv_videostab_ConsistentMosaicInpainter_stdevThresh_const(self.as_raw_ConsistentMosaicInpainter()) }.into_result()
    }
    
    pub fn inpaint(&mut self, idx: i32, frame: &mut core::Mat, mask: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_videostab_ConsistentMosaicInpainter_inpaint_int_Mat_Mat(self.as_raw_ConsistentMosaicInpainter(), idx, frame.as_raw_Mat(), mask.as_raw_Mat()) }.into_result()
    }
    
}

// Generating impl for trait cv::videostab::DeblurerBase (trait)
pub trait DeblurerBase {
    #[inline(always)] fn as_raw_DeblurerBase(&self) -> *mut c_void;
    fn set_radius(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_videostab_DeblurerBase_setRadius_int(self.as_raw_DeblurerBase(), val) }.into_result()
    }
    
    fn radius(&self) -> Result<i32> {
        unsafe { sys::cv_videostab_DeblurerBase_radius_const(self.as_raw_DeblurerBase()) }.into_result()
    }
    
    fn deblur(&mut self, idx: i32, frame: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_videostab_DeblurerBase_deblur_int_Mat(self.as_raw_DeblurerBase(), idx, frame.as_raw_Mat()) }.into_result()
    }
    
    fn set_frames(&mut self, val: &types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_videostab_DeblurerBase_setFrames_VectorOfMat(self.as_raw_DeblurerBase(), val.as_raw_VectorOfMat()) }.into_result()
    }
    
    fn frames(&self) -> Result<types::VectorOfMat> {
        unsafe { sys::cv_videostab_DeblurerBase_frames_const(self.as_raw_DeblurerBase()) }.into_result().map(|ptr| types::VectorOfMat { ptr })
    }
    
    fn set_motions(&mut self, val: &types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_videostab_DeblurerBase_setMotions_VectorOfMat(self.as_raw_DeblurerBase(), val.as_raw_VectorOfMat()) }.into_result()
    }
    
    fn motions(&self) -> Result<types::VectorOfMat> {
        unsafe { sys::cv_videostab_DeblurerBase_motions_const(self.as_raw_DeblurerBase()) }.into_result().map(|ptr| types::VectorOfMat { ptr })
    }
    
    fn set_blurriness_rates(&mut self, val: &types::VectorOffloat) -> Result<()> {
        unsafe { sys::cv_videostab_DeblurerBase_setBlurrinessRates_VectorOffloat(self.as_raw_DeblurerBase(), val.as_raw_VectorOffloat()) }.into_result()
    }
    
    fn blurriness_rates(&self) -> Result<types::VectorOffloat> {
        unsafe { sys::cv_videostab_DeblurerBase_blurrinessRates_const(self.as_raw_DeblurerBase()) }.into_result().map(|ptr| types::VectorOffloat { ptr })
    }
    
}

// boxed class cv::videostab::FastMarchingMethod
/// Describes the Fast Marching Method implementation.
///
/// See http://iwi.eldoc.ub.rug.nl/FILES/root/2004/JGraphToolsTelea/2004JGraphToolsTelea.pdf
pub struct FastMarchingMethod {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::FastMarchingMethod {
    fn drop(&mut self) {
        unsafe { sys::cv_FastMarchingMethod_delete(self.ptr) };
    }
}
impl crate::videostab::FastMarchingMethod {
    #[inline(always)] pub fn as_raw_FastMarchingMethod(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for FastMarchingMethod {}

impl FastMarchingMethod {

    pub fn default() -> Result<crate::videostab::FastMarchingMethod> {
        unsafe { sys::cv_videostab_FastMarchingMethod_FastMarchingMethod() }.into_result().map(|ptr| crate::videostab::FastMarchingMethod { ptr })
    }
    
    /// ## Returns
    /// Distance map that's created during working of the method.
    pub fn distance_map(&self) -> Result<core::Mat> {
        unsafe { sys::cv_videostab_FastMarchingMethod_distanceMap_const(self.as_raw_FastMarchingMethod()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
}

// boxed class cv::videostab::FromFileMotionReader
pub struct FromFileMotionReader {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::FromFileMotionReader {
    fn drop(&mut self) {
        unsafe { sys::cv_FromFileMotionReader_delete(self.ptr) };
    }
}
impl crate::videostab::FromFileMotionReader {
    #[inline(always)] pub fn as_raw_FromFileMotionReader(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for FromFileMotionReader {}

impl crate::videostab::ImageMotionEstimatorBase for FromFileMotionReader {
    #[inline(always)] fn as_raw_ImageMotionEstimatorBase(&self) -> *mut c_void { self.ptr }
}

impl FromFileMotionReader {

    pub fn new(path: &str) -> Result<crate::videostab::FromFileMotionReader> {
        string_arg!(path);
        unsafe { sys::cv_videostab_FromFileMotionReader_FromFileMotionReader_String(path.as_ptr()) }.into_result().map(|ptr| crate::videostab::FromFileMotionReader { ptr })
    }
    
    ///
    /// ## C++ default parameters
    /// * ok: 0
    pub fn estimate(&mut self, frame0: &core::Mat, frame1: &core::Mat, ok: &mut bool) -> Result<core::Mat> {
        unsafe { sys::cv_videostab_FromFileMotionReader_estimate_Mat_Mat_bool_X(self.as_raw_FromFileMotionReader(), frame0.as_raw_Mat(), frame1.as_raw_Mat(), ok) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
}

// boxed class cv::videostab::GaussianMotionFilter
pub struct GaussianMotionFilter {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::GaussianMotionFilter {
    fn drop(&mut self) {
        unsafe { sys::cv_GaussianMotionFilter_delete(self.ptr) };
    }
}
impl crate::videostab::GaussianMotionFilter {
    #[inline(always)] pub fn as_raw_GaussianMotionFilter(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for GaussianMotionFilter {}

impl crate::videostab::IMotionStabilizer for GaussianMotionFilter {
    #[inline(always)] fn as_raw_IMotionStabilizer(&self) -> *mut c_void { self.ptr }
}

impl crate::videostab::MotionFilterBase for GaussianMotionFilter {
    #[inline(always)] fn as_raw_MotionFilterBase(&self) -> *mut c_void { self.ptr }
}

impl GaussianMotionFilter {

    ///
    /// ## C++ default parameters
    /// * radius: 15
    /// * stdev: -1.f
    pub fn new(radius: i32, stdev: f32) -> Result<crate::videostab::GaussianMotionFilter> {
        unsafe { sys::cv_videostab_GaussianMotionFilter_GaussianMotionFilter_int_float(radius, stdev) }.into_result().map(|ptr| crate::videostab::GaussianMotionFilter { ptr })
    }
    
    ///
    /// ## C++ default parameters
    /// * stdev: -1.f
    pub fn set_params(&mut self, radius: i32, stdev: f32) -> Result<()> {
        unsafe { sys::cv_videostab_GaussianMotionFilter_setParams_int_float(self.as_raw_GaussianMotionFilter(), radius, stdev) }.into_result()
    }
    
    pub fn radius(&self) -> Result<i32> {
        unsafe { sys::cv_videostab_GaussianMotionFilter_radius_const(self.as_raw_GaussianMotionFilter()) }.into_result()
    }
    
    pub fn stdev(&self) -> Result<f32> {
        unsafe { sys::cv_videostab_GaussianMotionFilter_stdev_const(self.as_raw_GaussianMotionFilter()) }.into_result()
    }
    
}

// Generating impl for trait cv::videostab::IDenseOptFlowEstimator (trait)
pub trait IDenseOptFlowEstimator {
    #[inline(always)] fn as_raw_IDenseOptFlowEstimator(&self) -> *mut c_void;
    fn run(&mut self, frame0: &core::Mat, frame1: &core::Mat, flow_x: &mut core::Mat, flow_y: &mut core::Mat, errors: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_videostab_IDenseOptFlowEstimator_run_Mat_Mat_Mat_Mat_Mat(self.as_raw_IDenseOptFlowEstimator(), frame0.as_raw_Mat(), frame1.as_raw_Mat(), flow_x.as_raw_Mat(), flow_y.as_raw_Mat(), errors.as_raw_Mat()) }.into_result()
    }
    
}

// Generating impl for trait cv::videostab::IFrameSource (trait)
pub trait IFrameSource {
    #[inline(always)] fn as_raw_IFrameSource(&self) -> *mut c_void;
    fn reset(&mut self) -> Result<()> {
        unsafe { sys::cv_videostab_IFrameSource_reset(self.as_raw_IFrameSource()) }.into_result()
    }
    
    fn next_frame(&mut self) -> Result<core::Mat> {
        unsafe { sys::cv_videostab_IFrameSource_nextFrame(self.as_raw_IFrameSource()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
}

// Generating impl for trait cv::videostab::ILog (trait)
pub trait ILog {
    #[inline(always)] fn as_raw_ILog(&self) -> *mut c_void;
}

// Generating impl for trait cv::videostab::IMotionStabilizer (trait)
pub trait IMotionStabilizer {
    #[inline(always)] fn as_raw_IMotionStabilizer(&self) -> *mut c_void;
}

// Generating impl for trait cv::videostab::IOutlierRejector (trait)
pub trait IOutlierRejector {
    #[inline(always)] fn as_raw_IOutlierRejector(&self) -> *mut c_void;
    fn process(&mut self, frame_size: core::Size, points0: &core::Mat, points1: &core::Mat, mask: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_videostab_IOutlierRejector_process_Size_Mat_Mat_Mat(self.as_raw_IOutlierRejector(), frame_size, points0.as_raw_Mat(), points1.as_raw_Mat(), mask.as_raw_Mat()) }.into_result()
    }
    
}

// Generating impl for trait cv::videostab::ISparseOptFlowEstimator (trait)
pub trait ISparseOptFlowEstimator {
    #[inline(always)] fn as_raw_ISparseOptFlowEstimator(&self) -> *mut c_void;
    fn run(&mut self, frame0: &core::Mat, frame1: &core::Mat, points0: &core::Mat, points1: &mut core::Mat, status: &mut core::Mat, errors: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_videostab_ISparseOptFlowEstimator_run_Mat_Mat_Mat_Mat_Mat_Mat(self.as_raw_ISparseOptFlowEstimator(), frame0.as_raw_Mat(), frame1.as_raw_Mat(), points0.as_raw_Mat(), points1.as_raw_Mat(), status.as_raw_Mat(), errors.as_raw_Mat()) }.into_result()
    }
    
}

// Generating impl for trait cv::videostab::ImageMotionEstimatorBase (trait)
/// Base class for global 2D motion estimation methods which take frames as input.
pub trait ImageMotionEstimatorBase {
    #[inline(always)] fn as_raw_ImageMotionEstimatorBase(&self) -> *mut c_void;
    ///
    /// ## C++ default parameters
    /// * ok: 0
    fn estimate(&mut self, frame0: &core::Mat, frame1: &core::Mat, ok: &mut bool) -> Result<core::Mat> {
        unsafe { sys::cv_videostab_ImageMotionEstimatorBase_estimate_Mat_Mat_bool_X(self.as_raw_ImageMotionEstimatorBase(), frame0.as_raw_Mat(), frame1.as_raw_Mat(), ok) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
}

// Generating impl for trait cv::videostab::InpainterBase (trait)
pub trait InpainterBase {
    #[inline(always)] fn as_raw_InpainterBase(&self) -> *mut c_void;
    fn set_radius(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_videostab_InpainterBase_setRadius_int(self.as_raw_InpainterBase(), val) }.into_result()
    }
    
    fn radius(&self) -> Result<i32> {
        unsafe { sys::cv_videostab_InpainterBase_radius_const(self.as_raw_InpainterBase()) }.into_result()
    }
    
    fn inpaint(&mut self, idx: i32, frame: &mut core::Mat, mask: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_videostab_InpainterBase_inpaint_int_Mat_Mat(self.as_raw_InpainterBase(), idx, frame.as_raw_Mat(), mask.as_raw_Mat()) }.into_result()
    }
    
    fn set_frames(&mut self, val: &types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_videostab_InpainterBase_setFrames_VectorOfMat(self.as_raw_InpainterBase(), val.as_raw_VectorOfMat()) }.into_result()
    }
    
    fn frames(&self) -> Result<types::VectorOfMat> {
        unsafe { sys::cv_videostab_InpainterBase_frames_const(self.as_raw_InpainterBase()) }.into_result().map(|ptr| types::VectorOfMat { ptr })
    }
    
    fn set_motions(&mut self, val: &types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_videostab_InpainterBase_setMotions_VectorOfMat(self.as_raw_InpainterBase(), val.as_raw_VectorOfMat()) }.into_result()
    }
    
    fn motions(&self) -> Result<types::VectorOfMat> {
        unsafe { sys::cv_videostab_InpainterBase_motions_const(self.as_raw_InpainterBase()) }.into_result().map(|ptr| types::VectorOfMat { ptr })
    }
    
    fn set_stabilized_frames(&mut self, val: &types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_videostab_InpainterBase_setStabilizedFrames_VectorOfMat(self.as_raw_InpainterBase(), val.as_raw_VectorOfMat()) }.into_result()
    }
    
    fn stabilized_frames(&self) -> Result<types::VectorOfMat> {
        unsafe { sys::cv_videostab_InpainterBase_stabilizedFrames_const(self.as_raw_InpainterBase()) }.into_result().map(|ptr| types::VectorOfMat { ptr })
    }
    
    fn set_stabilization_motions(&mut self, val: &types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_videostab_InpainterBase_setStabilizationMotions_VectorOfMat(self.as_raw_InpainterBase(), val.as_raw_VectorOfMat()) }.into_result()
    }
    
    fn stabilization_motions(&self) -> Result<types::VectorOfMat> {
        unsafe { sys::cv_videostab_InpainterBase_stabilizationMotions_const(self.as_raw_InpainterBase()) }.into_result().map(|ptr| types::VectorOfMat { ptr })
    }
    
}

// boxed class cv::videostab::InpaintingPipeline
pub struct InpaintingPipeline {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::InpaintingPipeline {
    fn drop(&mut self) {
        unsafe { sys::cv_InpaintingPipeline_delete(self.ptr) };
    }
}
impl crate::videostab::InpaintingPipeline {
    #[inline(always)] pub fn as_raw_InpaintingPipeline(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for InpaintingPipeline {}

impl crate::videostab::InpainterBase for InpaintingPipeline {
    #[inline(always)] fn as_raw_InpainterBase(&self) -> *mut c_void { self.ptr }
}

impl InpaintingPipeline {

    pub fn push_back(&mut self, inpainter: &types::PtrOfInpainterBase) -> Result<()> {
        unsafe { sys::cv_videostab_InpaintingPipeline_pushBack_PtrOfInpainterBase(self.as_raw_InpaintingPipeline(), inpainter.as_raw_PtrOfInpainterBase()) }.into_result()
    }
    
    pub fn empty(&self) -> Result<bool> {
        unsafe { sys::cv_videostab_InpaintingPipeline_empty_const(self.as_raw_InpaintingPipeline()) }.into_result()
    }
    
    pub fn set_radius(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_videostab_InpaintingPipeline_setRadius_int(self.as_raw_InpaintingPipeline(), val) }.into_result()
    }
    
    pub fn set_frames(&mut self, val: &types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_videostab_InpaintingPipeline_setFrames_VectorOfMat(self.as_raw_InpaintingPipeline(), val.as_raw_VectorOfMat()) }.into_result()
    }
    
    pub fn set_motions(&mut self, val: &types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_videostab_InpaintingPipeline_setMotions_VectorOfMat(self.as_raw_InpaintingPipeline(), val.as_raw_VectorOfMat()) }.into_result()
    }
    
    pub fn set_stabilized_frames(&mut self, val: &types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_videostab_InpaintingPipeline_setStabilizedFrames_VectorOfMat(self.as_raw_InpaintingPipeline(), val.as_raw_VectorOfMat()) }.into_result()
    }
    
    pub fn set_stabilization_motions(&mut self, val: &types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_videostab_InpaintingPipeline_setStabilizationMotions_VectorOfMat(self.as_raw_InpaintingPipeline(), val.as_raw_VectorOfMat()) }.into_result()
    }
    
    pub fn inpaint(&mut self, idx: i32, frame: &mut core::Mat, mask: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_videostab_InpaintingPipeline_inpaint_int_Mat_Mat(self.as_raw_InpaintingPipeline(), idx, frame.as_raw_Mat(), mask.as_raw_Mat()) }.into_result()
    }
    
}

// boxed class cv::videostab::KeypointBasedMotionEstimator
/// Describes a global 2D motion estimation method which uses keypoints detection and optical flow for
/// matching.
pub struct KeypointBasedMotionEstimator {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::KeypointBasedMotionEstimator {
    fn drop(&mut self) {
        unsafe { sys::cv_KeypointBasedMotionEstimator_delete(self.ptr) };
    }
}
impl crate::videostab::KeypointBasedMotionEstimator {
    #[inline(always)] pub fn as_raw_KeypointBasedMotionEstimator(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for KeypointBasedMotionEstimator {}

impl crate::videostab::ImageMotionEstimatorBase for KeypointBasedMotionEstimator {
    #[inline(always)] fn as_raw_ImageMotionEstimatorBase(&self) -> *mut c_void { self.ptr }
}

impl KeypointBasedMotionEstimator {

    pub fn new(estimator: &types::PtrOfMotionEstimatorBase) -> Result<crate::videostab::KeypointBasedMotionEstimator> {
        unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_KeypointBasedMotionEstimator_PtrOfMotionEstimatorBase(estimator.as_raw_PtrOfMotionEstimatorBase()) }.into_result().map(|ptr| crate::videostab::KeypointBasedMotionEstimator { ptr })
    }
    
    ///
    /// ## C++ default parameters
    /// * ok: 0
    pub fn estimate(&mut self, frame0: &core::Mat, frame1: &core::Mat, ok: &mut bool) -> Result<core::Mat> {
        unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_estimate_Mat_Mat_bool_X(self.as_raw_KeypointBasedMotionEstimator(), frame0.as_raw_Mat(), frame1.as_raw_Mat(), ok) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
}

// boxed class cv::videostab::LogToStdout
pub struct LogToStdout {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::LogToStdout {
    fn drop(&mut self) {
        unsafe { sys::cv_LogToStdout_delete(self.ptr) };
    }
}
impl crate::videostab::LogToStdout {
    #[inline(always)] pub fn as_raw_LogToStdout(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for LogToStdout {}

impl crate::videostab::ILog for LogToStdout {
    #[inline(always)] fn as_raw_ILog(&self) -> *mut c_void { self.ptr }
}

impl LogToStdout {

}

// boxed class cv::videostab::LpMotionStabilizer
pub struct LpMotionStabilizer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::LpMotionStabilizer {
    fn drop(&mut self) {
        unsafe { sys::cv_LpMotionStabilizer_delete(self.ptr) };
    }
}
impl crate::videostab::LpMotionStabilizer {
    #[inline(always)] pub fn as_raw_LpMotionStabilizer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for LpMotionStabilizer {}

impl crate::videostab::IMotionStabilizer for LpMotionStabilizer {
    #[inline(always)] fn as_raw_IMotionStabilizer(&self) -> *mut c_void { self.ptr }
}

impl LpMotionStabilizer {

    pub fn set_frame_size(&mut self, val: core::Size) -> Result<()> {
        unsafe { sys::cv_videostab_LpMotionStabilizer_setFrameSize_Size(self.as_raw_LpMotionStabilizer(), val) }.into_result()
    }
    
    pub fn frame_size(&self) -> Result<core::Size> {
        unsafe { sys::cv_videostab_LpMotionStabilizer_frameSize_const(self.as_raw_LpMotionStabilizer()) }.into_result()
    }
    
    pub fn set_trim_ratio(&mut self, val: f32) -> Result<()> {
        unsafe { sys::cv_videostab_LpMotionStabilizer_setTrimRatio_float(self.as_raw_LpMotionStabilizer(), val) }.into_result()
    }
    
    pub fn trim_ratio(&self) -> Result<f32> {
        unsafe { sys::cv_videostab_LpMotionStabilizer_trimRatio_const(self.as_raw_LpMotionStabilizer()) }.into_result()
    }
    
    pub fn set_weight1(&mut self, val: f32) -> Result<()> {
        unsafe { sys::cv_videostab_LpMotionStabilizer_setWeight1_float(self.as_raw_LpMotionStabilizer(), val) }.into_result()
    }
    
    pub fn weight1(&self) -> Result<f32> {
        unsafe { sys::cv_videostab_LpMotionStabilizer_weight1_const(self.as_raw_LpMotionStabilizer()) }.into_result()
    }
    
    pub fn set_weight2(&mut self, val: f32) -> Result<()> {
        unsafe { sys::cv_videostab_LpMotionStabilizer_setWeight2_float(self.as_raw_LpMotionStabilizer(), val) }.into_result()
    }
    
    pub fn weight2(&self) -> Result<f32> {
        unsafe { sys::cv_videostab_LpMotionStabilizer_weight2_const(self.as_raw_LpMotionStabilizer()) }.into_result()
    }
    
    pub fn set_weight3(&mut self, val: f32) -> Result<()> {
        unsafe { sys::cv_videostab_LpMotionStabilizer_setWeight3_float(self.as_raw_LpMotionStabilizer(), val) }.into_result()
    }
    
    pub fn weight3(&self) -> Result<f32> {
        unsafe { sys::cv_videostab_LpMotionStabilizer_weight3_const(self.as_raw_LpMotionStabilizer()) }.into_result()
    }
    
    pub fn set_weight4(&mut self, val: f32) -> Result<()> {
        unsafe { sys::cv_videostab_LpMotionStabilizer_setWeight4_float(self.as_raw_LpMotionStabilizer(), val) }.into_result()
    }
    
    pub fn weight4(&self) -> Result<f32> {
        unsafe { sys::cv_videostab_LpMotionStabilizer_weight4_const(self.as_raw_LpMotionStabilizer()) }.into_result()
    }
    
}

// boxed class cv::videostab::MoreAccurateMotionWobbleSuppressor
pub struct MoreAccurateMotionWobbleSuppressor {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::MoreAccurateMotionWobbleSuppressor {
    fn drop(&mut self) {
        unsafe { sys::cv_MoreAccurateMotionWobbleSuppressor_delete(self.ptr) };
    }
}
impl crate::videostab::MoreAccurateMotionWobbleSuppressor {
    #[inline(always)] pub fn as_raw_MoreAccurateMotionWobbleSuppressor(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for MoreAccurateMotionWobbleSuppressor {}

impl crate::videostab::MoreAccurateMotionWobbleSuppressorBase for MoreAccurateMotionWobbleSuppressor {
    #[inline(always)] fn as_raw_MoreAccurateMotionWobbleSuppressorBase(&self) -> *mut c_void { self.ptr }
}

impl crate::videostab::WobbleSuppressorBase for MoreAccurateMotionWobbleSuppressor {
    #[inline(always)] fn as_raw_WobbleSuppressorBase(&self) -> *mut c_void { self.ptr }
}

impl MoreAccurateMotionWobbleSuppressor {

    pub fn suppress(&mut self, idx: i32, frame: &core::Mat, result: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_videostab_MoreAccurateMotionWobbleSuppressor_suppress_int_Mat_Mat(self.as_raw_MoreAccurateMotionWobbleSuppressor(), idx, frame.as_raw_Mat(), result.as_raw_Mat()) }.into_result()
    }
    
}

// Generating impl for trait cv::videostab::MoreAccurateMotionWobbleSuppressorBase (trait)
pub trait MoreAccurateMotionWobbleSuppressorBase: crate::videostab::WobbleSuppressorBase {
    #[inline(always)] fn as_raw_MoreAccurateMotionWobbleSuppressorBase(&self) -> *mut c_void;
    fn set_period(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_videostab_MoreAccurateMotionWobbleSuppressorBase_setPeriod_int(self.as_raw_MoreAccurateMotionWobbleSuppressorBase(), val) }.into_result()
    }
    
    fn period(&self) -> Result<i32> {
        unsafe { sys::cv_videostab_MoreAccurateMotionWobbleSuppressorBase_period_const(self.as_raw_MoreAccurateMotionWobbleSuppressorBase()) }.into_result()
    }
    
}

// Generating impl for trait cv::videostab::MotionEstimatorBase (trait)
/// Base class for all global motion estimation methods.
pub trait MotionEstimatorBase {
    #[inline(always)] fn as_raw_MotionEstimatorBase(&self) -> *mut c_void;
    /// Estimates global motion between two 2D point clouds.
    ///
    /// ## Parameters
    /// * points0: Source set of 2D points (32F).
    /// * points1: Destination set of 2D points (32F).
    /// * ok: Indicates whether motion was estimated successfully.
    /// ## Returns
    /// 3x3 2D transformation matrix (32F).
    ///
    /// ## C++ default parameters
    /// * ok: 0
    fn estimate(&mut self, points0: &core::Mat, points1: &core::Mat, ok: &mut bool) -> Result<core::Mat> {
        unsafe { sys::cv_videostab_MotionEstimatorBase_estimate_Mat_Mat_bool_X(self.as_raw_MotionEstimatorBase(), points0.as_raw_Mat(), points1.as_raw_Mat(), ok) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
}

// boxed class cv::videostab::MotionEstimatorL1
/// Describes a global 2D motion estimation method which minimizes L1 error.
///
///
/// Note: To be able to use this method you must build OpenCV with CLP library support. :
pub struct MotionEstimatorL1 {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::MotionEstimatorL1 {
    fn drop(&mut self) {
        unsafe { sys::cv_MotionEstimatorL1_delete(self.ptr) };
    }
}
impl crate::videostab::MotionEstimatorL1 {
    #[inline(always)] pub fn as_raw_MotionEstimatorL1(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for MotionEstimatorL1 {}

impl crate::videostab::MotionEstimatorBase for MotionEstimatorL1 {
    #[inline(always)] fn as_raw_MotionEstimatorBase(&self) -> *mut c_void { self.ptr }
}

impl MotionEstimatorL1 {

    ///
    /// ## C++ default parameters
    /// * ok: 0
    pub fn estimate(&mut self, points0: &core::Mat, points1: &core::Mat, ok: &mut bool) -> Result<core::Mat> {
        unsafe { sys::cv_videostab_MotionEstimatorL1_estimate_Mat_Mat_bool_X(self.as_raw_MotionEstimatorL1(), points0.as_raw_Mat(), points1.as_raw_Mat(), ok) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
}

// boxed class cv::videostab::MotionEstimatorRansacL2
/// Describes a robust RANSAC-based global 2D motion estimation method which minimizes L2 error.
pub struct MotionEstimatorRansacL2 {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::MotionEstimatorRansacL2 {
    fn drop(&mut self) {
        unsafe { sys::cv_MotionEstimatorRansacL2_delete(self.ptr) };
    }
}
impl crate::videostab::MotionEstimatorRansacL2 {
    #[inline(always)] pub fn as_raw_MotionEstimatorRansacL2(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for MotionEstimatorRansacL2 {}

impl crate::videostab::MotionEstimatorBase for MotionEstimatorRansacL2 {
    #[inline(always)] fn as_raw_MotionEstimatorBase(&self) -> *mut c_void { self.ptr }
}

impl MotionEstimatorRansacL2 {

    pub fn set_min_inlier_ratio(&mut self, val: f32) -> Result<()> {
        unsafe { sys::cv_videostab_MotionEstimatorRansacL2_setMinInlierRatio_float(self.as_raw_MotionEstimatorRansacL2(), val) }.into_result()
    }
    
    pub fn min_inlier_ratio(&self) -> Result<f32> {
        unsafe { sys::cv_videostab_MotionEstimatorRansacL2_minInlierRatio_const(self.as_raw_MotionEstimatorRansacL2()) }.into_result()
    }
    
    ///
    /// ## C++ default parameters
    /// * ok: 0
    pub fn estimate(&mut self, points0: &core::Mat, points1: &core::Mat, ok: &mut bool) -> Result<core::Mat> {
        unsafe { sys::cv_videostab_MotionEstimatorRansacL2_estimate_Mat_Mat_bool_X(self.as_raw_MotionEstimatorRansacL2(), points0.as_raw_Mat(), points1.as_raw_Mat(), ok) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
}

// Generating impl for trait cv::videostab::MotionFilterBase (trait)
pub trait MotionFilterBase: crate::videostab::IMotionStabilizer {
    #[inline(always)] fn as_raw_MotionFilterBase(&self) -> *mut c_void;
}

// boxed class cv::videostab::MotionInpainter
pub struct MotionInpainter {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::MotionInpainter {
    fn drop(&mut self) {
        unsafe { sys::cv_MotionInpainter_delete(self.ptr) };
    }
}
impl crate::videostab::MotionInpainter {
    #[inline(always)] pub fn as_raw_MotionInpainter(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for MotionInpainter {}

impl crate::videostab::InpainterBase for MotionInpainter {
    #[inline(always)] fn as_raw_InpainterBase(&self) -> *mut c_void { self.ptr }
}

impl MotionInpainter {

    pub fn default() -> Result<crate::videostab::MotionInpainter> {
        unsafe { sys::cv_videostab_MotionInpainter_MotionInpainter() }.into_result().map(|ptr| crate::videostab::MotionInpainter { ptr })
    }
    
    pub fn set_flow_error_threshold(&mut self, val: f32) -> Result<()> {
        unsafe { sys::cv_videostab_MotionInpainter_setFlowErrorThreshold_float(self.as_raw_MotionInpainter(), val) }.into_result()
    }
    
    pub fn flow_error_threshold(&self) -> Result<f32> {
        unsafe { sys::cv_videostab_MotionInpainter_flowErrorThreshold_const(self.as_raw_MotionInpainter()) }.into_result()
    }
    
    pub fn set_dist_threshold(&mut self, val: f32) -> Result<()> {
        unsafe { sys::cv_videostab_MotionInpainter_setDistThreshold_float(self.as_raw_MotionInpainter(), val) }.into_result()
    }
    
    pub fn dist_thresh(&self) -> Result<f32> {
        unsafe { sys::cv_videostab_MotionInpainter_distThresh_const(self.as_raw_MotionInpainter()) }.into_result()
    }
    
    pub fn set_border_mode(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_videostab_MotionInpainter_setBorderMode_int(self.as_raw_MotionInpainter(), val) }.into_result()
    }
    
    pub fn border_mode(&self) -> Result<i32> {
        unsafe { sys::cv_videostab_MotionInpainter_borderMode_const(self.as_raw_MotionInpainter()) }.into_result()
    }
    
    pub fn inpaint(&mut self, idx: i32, frame: &mut core::Mat, mask: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_videostab_MotionInpainter_inpaint_int_Mat_Mat(self.as_raw_MotionInpainter(), idx, frame.as_raw_Mat(), mask.as_raw_Mat()) }.into_result()
    }
    
}

// boxed class cv::videostab::MotionStabilizationPipeline
pub struct MotionStabilizationPipeline {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::MotionStabilizationPipeline {
    fn drop(&mut self) {
        unsafe { sys::cv_MotionStabilizationPipeline_delete(self.ptr) };
    }
}
impl crate::videostab::MotionStabilizationPipeline {
    #[inline(always)] pub fn as_raw_MotionStabilizationPipeline(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for MotionStabilizationPipeline {}

impl crate::videostab::IMotionStabilizer for MotionStabilizationPipeline {
    #[inline(always)] fn as_raw_IMotionStabilizer(&self) -> *mut c_void { self.ptr }
}

impl MotionStabilizationPipeline {

    pub fn push_back(&mut self, stabilizer: &types::PtrOfIMotionStabilizer) -> Result<()> {
        unsafe { sys::cv_videostab_MotionStabilizationPipeline_pushBack_PtrOfIMotionStabilizer(self.as_raw_MotionStabilizationPipeline(), stabilizer.as_raw_PtrOfIMotionStabilizer()) }.into_result()
    }
    
    pub fn empty(&self) -> Result<bool> {
        unsafe { sys::cv_videostab_MotionStabilizationPipeline_empty_const(self.as_raw_MotionStabilizationPipeline()) }.into_result()
    }
    
}

// boxed class cv::videostab::NullDeblurer
pub struct NullDeblurer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::NullDeblurer {
    fn drop(&mut self) {
        unsafe { sys::cv_NullDeblurer_delete(self.ptr) };
    }
}
impl crate::videostab::NullDeblurer {
    #[inline(always)] pub fn as_raw_NullDeblurer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for NullDeblurer {}

impl crate::videostab::DeblurerBase for NullDeblurer {
    #[inline(always)] fn as_raw_DeblurerBase(&self) -> *mut c_void { self.ptr }
}

impl NullDeblurer {

    pub fn deblur(&mut self, unnamed_arg: i32, unnamed_arg_1: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_videostab_NullDeblurer_deblur_int_Mat(self.as_raw_NullDeblurer(), unnamed_arg, unnamed_arg_1.as_raw_Mat()) }.into_result()
    }
    
}

// boxed class cv::videostab::NullFrameSource
pub struct NullFrameSource {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::NullFrameSource {
    fn drop(&mut self) {
        unsafe { sys::cv_NullFrameSource_delete(self.ptr) };
    }
}
impl crate::videostab::NullFrameSource {
    #[inline(always)] pub fn as_raw_NullFrameSource(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for NullFrameSource {}

impl crate::videostab::IFrameSource for NullFrameSource {
    #[inline(always)] fn as_raw_IFrameSource(&self) -> *mut c_void { self.ptr }
}

impl NullFrameSource {

    pub fn reset(&mut self) -> Result<()> {
        unsafe { sys::cv_videostab_NullFrameSource_reset(self.as_raw_NullFrameSource()) }.into_result()
    }
    
    pub fn next_frame(&mut self) -> Result<core::Mat> {
        unsafe { sys::cv_videostab_NullFrameSource_nextFrame(self.as_raw_NullFrameSource()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
}

// boxed class cv::videostab::NullInpainter
pub struct NullInpainter {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::NullInpainter {
    fn drop(&mut self) {
        unsafe { sys::cv_NullInpainter_delete(self.ptr) };
    }
}
impl crate::videostab::NullInpainter {
    #[inline(always)] pub fn as_raw_NullInpainter(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for NullInpainter {}

impl crate::videostab::InpainterBase for NullInpainter {
    #[inline(always)] fn as_raw_InpainterBase(&self) -> *mut c_void { self.ptr }
}

impl NullInpainter {

    pub fn inpaint(&mut self, unnamed_arg: i32, unnamed_arg_1: &mut core::Mat, unnamed_arg_2: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_videostab_NullInpainter_inpaint_int_Mat_Mat(self.as_raw_NullInpainter(), unnamed_arg, unnamed_arg_1.as_raw_Mat(), unnamed_arg_2.as_raw_Mat()) }.into_result()
    }
    
}

// boxed class cv::videostab::NullLog
pub struct NullLog {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::NullLog {
    fn drop(&mut self) {
        unsafe { sys::cv_NullLog_delete(self.ptr) };
    }
}
impl crate::videostab::NullLog {
    #[inline(always)] pub fn as_raw_NullLog(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for NullLog {}

impl crate::videostab::ILog for NullLog {
    #[inline(always)] fn as_raw_ILog(&self) -> *mut c_void { self.ptr }
}

impl NullLog {

}

// boxed class cv::videostab::NullOutlierRejector
pub struct NullOutlierRejector {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::NullOutlierRejector {
    fn drop(&mut self) {
        unsafe { sys::cv_NullOutlierRejector_delete(self.ptr) };
    }
}
impl crate::videostab::NullOutlierRejector {
    #[inline(always)] pub fn as_raw_NullOutlierRejector(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for NullOutlierRejector {}

impl crate::videostab::IOutlierRejector for NullOutlierRejector {
    #[inline(always)] fn as_raw_IOutlierRejector(&self) -> *mut c_void { self.ptr }
}

impl NullOutlierRejector {

    pub fn process(&mut self, frame_size: core::Size, points0: &core::Mat, points1: &core::Mat, mask: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_videostab_NullOutlierRejector_process_Size_Mat_Mat_Mat(self.as_raw_NullOutlierRejector(), frame_size, points0.as_raw_Mat(), points1.as_raw_Mat(), mask.as_raw_Mat()) }.into_result()
    }
    
}

// boxed class cv::videostab::NullWobbleSuppressor
pub struct NullWobbleSuppressor {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::NullWobbleSuppressor {
    fn drop(&mut self) {
        unsafe { sys::cv_NullWobbleSuppressor_delete(self.ptr) };
    }
}
impl crate::videostab::NullWobbleSuppressor {
    #[inline(always)] pub fn as_raw_NullWobbleSuppressor(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for NullWobbleSuppressor {}

impl crate::videostab::WobbleSuppressorBase for NullWobbleSuppressor {
    #[inline(always)] fn as_raw_WobbleSuppressorBase(&self) -> *mut c_void { self.ptr }
}

impl NullWobbleSuppressor {

    pub fn suppress(&mut self, idx: i32, frame: &core::Mat, result: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_videostab_NullWobbleSuppressor_suppress_int_Mat_Mat(self.as_raw_NullWobbleSuppressor(), idx, frame.as_raw_Mat(), result.as_raw_Mat()) }.into_result()
    }
    
}

// boxed class cv::videostab::OnePassStabilizer
pub struct OnePassStabilizer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::OnePassStabilizer {
    fn drop(&mut self) {
        unsafe { sys::cv_OnePassStabilizer_delete(self.ptr) };
    }
}
impl crate::videostab::OnePassStabilizer {
    #[inline(always)] pub fn as_raw_OnePassStabilizer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for OnePassStabilizer {}

impl crate::videostab::IFrameSource for OnePassStabilizer {
    #[inline(always)] fn as_raw_IFrameSource(&self) -> *mut c_void { self.ptr }
}

impl crate::videostab::StabilizerBase for OnePassStabilizer {
    #[inline(always)] fn as_raw_StabilizerBase(&self) -> *mut c_void { self.ptr }
}

impl OnePassStabilizer {

    pub fn default() -> Result<crate::videostab::OnePassStabilizer> {
        unsafe { sys::cv_videostab_OnePassStabilizer_OnePassStabilizer() }.into_result().map(|ptr| crate::videostab::OnePassStabilizer { ptr })
    }
    
    pub fn set_motion_filter(&mut self, val: &types::PtrOfMotionFilterBase) -> Result<()> {
        unsafe { sys::cv_videostab_OnePassStabilizer_setMotionFilter_PtrOfMotionFilterBase(self.as_raw_OnePassStabilizer(), val.as_raw_PtrOfMotionFilterBase()) }.into_result()
    }
    
    pub fn motion_filter(&self) -> Result<types::PtrOfMotionFilterBase> {
        unsafe { sys::cv_videostab_OnePassStabilizer_motionFilter_const(self.as_raw_OnePassStabilizer()) }.into_result().map(|ptr| types::PtrOfMotionFilterBase { ptr })
    }
    
    pub fn reset(&mut self) -> Result<()> {
        unsafe { sys::cv_videostab_OnePassStabilizer_reset(self.as_raw_OnePassStabilizer()) }.into_result()
    }
    
    pub fn next_frame(&mut self) -> Result<core::Mat> {
        unsafe { sys::cv_videostab_OnePassStabilizer_nextFrame(self.as_raw_OnePassStabilizer()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
}

// Generating impl for trait cv::videostab::PyrLkOptFlowEstimatorBase (trait)
pub trait PyrLkOptFlowEstimatorBase {
    #[inline(always)] fn as_raw_PyrLkOptFlowEstimatorBase(&self) -> *mut c_void;
    fn set_win_size(&mut self, val: core::Size) -> Result<()> {
        unsafe { sys::cv_videostab_PyrLkOptFlowEstimatorBase_setWinSize_Size(self.as_raw_PyrLkOptFlowEstimatorBase(), val) }.into_result()
    }
    
    fn win_size(&self) -> Result<core::Size> {
        unsafe { sys::cv_videostab_PyrLkOptFlowEstimatorBase_winSize_const(self.as_raw_PyrLkOptFlowEstimatorBase()) }.into_result()
    }
    
    fn set_max_level(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_videostab_PyrLkOptFlowEstimatorBase_setMaxLevel_int(self.as_raw_PyrLkOptFlowEstimatorBase(), val) }.into_result()
    }
    
    fn max_level(&self) -> Result<i32> {
        unsafe { sys::cv_videostab_PyrLkOptFlowEstimatorBase_maxLevel_const(self.as_raw_PyrLkOptFlowEstimatorBase()) }.into_result()
    }
    
}

// boxed class cv::videostab::RansacParams
/// Describes RANSAC method parameters.
pub struct RansacParams {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::RansacParams {
    fn drop(&mut self) {
        unsafe { sys::cv_RansacParams_delete(self.ptr) };
    }
}
impl crate::videostab::RansacParams {
    #[inline(always)] pub fn as_raw_RansacParams(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for RansacParams {}

impl RansacParams {

    /// ## Returns
    /// Number of iterations that'll be performed by RANSAC method.
    pub fn niters(&self) -> Result<i32> {
        unsafe { sys::cv_videostab_RansacParams_niters_const(self.as_raw_RansacParams()) }.into_result()
    }
    
}

// boxed class cv::videostab::SparsePyrLkOptFlowEstimator
pub struct SparsePyrLkOptFlowEstimator {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::SparsePyrLkOptFlowEstimator {
    fn drop(&mut self) {
        unsafe { sys::cv_SparsePyrLkOptFlowEstimator_delete(self.ptr) };
    }
}
impl crate::videostab::SparsePyrLkOptFlowEstimator {
    #[inline(always)] pub fn as_raw_SparsePyrLkOptFlowEstimator(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for SparsePyrLkOptFlowEstimator {}

impl crate::videostab::ISparseOptFlowEstimator for SparsePyrLkOptFlowEstimator {
    #[inline(always)] fn as_raw_ISparseOptFlowEstimator(&self) -> *mut c_void { self.ptr }
}

impl crate::videostab::PyrLkOptFlowEstimatorBase for SparsePyrLkOptFlowEstimator {
    #[inline(always)] fn as_raw_PyrLkOptFlowEstimatorBase(&self) -> *mut c_void { self.ptr }
}

impl SparsePyrLkOptFlowEstimator {

    pub fn run(&mut self, frame0: &core::Mat, frame1: &core::Mat, points0: &core::Mat, points1: &mut core::Mat, status: &mut core::Mat, errors: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_videostab_SparsePyrLkOptFlowEstimator_run_Mat_Mat_Mat_Mat_Mat_Mat(self.as_raw_SparsePyrLkOptFlowEstimator(), frame0.as_raw_Mat(), frame1.as_raw_Mat(), points0.as_raw_Mat(), points1.as_raw_Mat(), status.as_raw_Mat(), errors.as_raw_Mat()) }.into_result()
    }
    
}

// Generating impl for trait cv::videostab::StabilizerBase (trait)
pub trait StabilizerBase {
    #[inline(always)] fn as_raw_StabilizerBase(&self) -> *mut c_void;
    fn set_log(&mut self, ilog: &types::PtrOfILog) -> Result<()> {
        unsafe { sys::cv_videostab_StabilizerBase_setLog_PtrOfILog(self.as_raw_StabilizerBase(), ilog.as_raw_PtrOfILog()) }.into_result()
    }
    
    fn log(&self) -> Result<types::PtrOfILog> {
        unsafe { sys::cv_videostab_StabilizerBase_log_const(self.as_raw_StabilizerBase()) }.into_result().map(|ptr| types::PtrOfILog { ptr })
    }
    
    fn set_radius(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_videostab_StabilizerBase_setRadius_int(self.as_raw_StabilizerBase(), val) }.into_result()
    }
    
    fn radius(&self) -> Result<i32> {
        unsafe { sys::cv_videostab_StabilizerBase_radius_const(self.as_raw_StabilizerBase()) }.into_result()
    }
    
    fn set_frame_source(&mut self, val: &types::PtrOfIFrameSource) -> Result<()> {
        unsafe { sys::cv_videostab_StabilizerBase_setFrameSource_PtrOfIFrameSource(self.as_raw_StabilizerBase(), val.as_raw_PtrOfIFrameSource()) }.into_result()
    }
    
    fn frame_source(&self) -> Result<types::PtrOfIFrameSource> {
        unsafe { sys::cv_videostab_StabilizerBase_frameSource_const(self.as_raw_StabilizerBase()) }.into_result().map(|ptr| types::PtrOfIFrameSource { ptr })
    }
    
    fn set_motion_estimator(&mut self, val: &types::PtrOfImageMotionEstimatorBase) -> Result<()> {
        unsafe { sys::cv_videostab_StabilizerBase_setMotionEstimator_PtrOfImageMotionEstimatorBase(self.as_raw_StabilizerBase(), val.as_raw_PtrOfImageMotionEstimatorBase()) }.into_result()
    }
    
    fn motion_estimator(&self) -> Result<types::PtrOfImageMotionEstimatorBase> {
        unsafe { sys::cv_videostab_StabilizerBase_motionEstimator_const(self.as_raw_StabilizerBase()) }.into_result().map(|ptr| types::PtrOfImageMotionEstimatorBase { ptr })
    }
    
    fn set_deblurer(&mut self, val: &types::PtrOfDeblurerBase) -> Result<()> {
        unsafe { sys::cv_videostab_StabilizerBase_setDeblurer_PtrOfDeblurerBase(self.as_raw_StabilizerBase(), val.as_raw_PtrOfDeblurerBase()) }.into_result()
    }
    
    fn deblurrer(&self) -> Result<types::PtrOfDeblurerBase> {
        unsafe { sys::cv_videostab_StabilizerBase_deblurrer_const(self.as_raw_StabilizerBase()) }.into_result().map(|ptr| types::PtrOfDeblurerBase { ptr })
    }
    
    fn set_trim_ratio(&mut self, val: f32) -> Result<()> {
        unsafe { sys::cv_videostab_StabilizerBase_setTrimRatio_float(self.as_raw_StabilizerBase(), val) }.into_result()
    }
    
    fn trim_ratio(&self) -> Result<f32> {
        unsafe { sys::cv_videostab_StabilizerBase_trimRatio_const(self.as_raw_StabilizerBase()) }.into_result()
    }
    
    fn set_correction_for_inclusion(&mut self, val: bool) -> Result<()> {
        unsafe { sys::cv_videostab_StabilizerBase_setCorrectionForInclusion_bool(self.as_raw_StabilizerBase(), val) }.into_result()
    }
    
    fn do_correction_for_inclusion(&self) -> Result<bool> {
        unsafe { sys::cv_videostab_StabilizerBase_doCorrectionForInclusion_const(self.as_raw_StabilizerBase()) }.into_result()
    }
    
    fn set_border_mode(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_videostab_StabilizerBase_setBorderMode_int(self.as_raw_StabilizerBase(), val) }.into_result()
    }
    
    fn border_mode(&self) -> Result<i32> {
        unsafe { sys::cv_videostab_StabilizerBase_borderMode_const(self.as_raw_StabilizerBase()) }.into_result()
    }
    
    fn set_inpainter(&mut self, val: &types::PtrOfInpainterBase) -> Result<()> {
        unsafe { sys::cv_videostab_StabilizerBase_setInpainter_PtrOfInpainterBase(self.as_raw_StabilizerBase(), val.as_raw_PtrOfInpainterBase()) }.into_result()
    }
    
    fn inpainter(&self) -> Result<types::PtrOfInpainterBase> {
        unsafe { sys::cv_videostab_StabilizerBase_inpainter_const(self.as_raw_StabilizerBase()) }.into_result().map(|ptr| types::PtrOfInpainterBase { ptr })
    }
    
}

// boxed class cv::videostab::ToFileMotionWriter
pub struct ToFileMotionWriter {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::ToFileMotionWriter {
    fn drop(&mut self) {
        unsafe { sys::cv_ToFileMotionWriter_delete(self.ptr) };
    }
}
impl crate::videostab::ToFileMotionWriter {
    #[inline(always)] pub fn as_raw_ToFileMotionWriter(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for ToFileMotionWriter {}

impl crate::videostab::ImageMotionEstimatorBase for ToFileMotionWriter {
    #[inline(always)] fn as_raw_ImageMotionEstimatorBase(&self) -> *mut c_void { self.ptr }
}

impl ToFileMotionWriter {

    pub fn new(path: &str, estimator: &types::PtrOfImageMotionEstimatorBase) -> Result<crate::videostab::ToFileMotionWriter> {
        string_arg!(path);
        unsafe { sys::cv_videostab_ToFileMotionWriter_ToFileMotionWriter_String_PtrOfImageMotionEstimatorBase(path.as_ptr(), estimator.as_raw_PtrOfImageMotionEstimatorBase()) }.into_result().map(|ptr| crate::videostab::ToFileMotionWriter { ptr })
    }
    
    ///
    /// ## C++ default parameters
    /// * ok: 0
    pub fn estimate(&mut self, frame0: &core::Mat, frame1: &core::Mat, ok: &mut bool) -> Result<core::Mat> {
        unsafe { sys::cv_videostab_ToFileMotionWriter_estimate_Mat_Mat_bool_X(self.as_raw_ToFileMotionWriter(), frame0.as_raw_Mat(), frame1.as_raw_Mat(), ok) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
}

// boxed class cv::videostab::TranslationBasedLocalOutlierRejector
pub struct TranslationBasedLocalOutlierRejector {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::TranslationBasedLocalOutlierRejector {
    fn drop(&mut self) {
        unsafe { sys::cv_TranslationBasedLocalOutlierRejector_delete(self.ptr) };
    }
}
impl crate::videostab::TranslationBasedLocalOutlierRejector {
    #[inline(always)] pub fn as_raw_TranslationBasedLocalOutlierRejector(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for TranslationBasedLocalOutlierRejector {}

impl crate::videostab::IOutlierRejector for TranslationBasedLocalOutlierRejector {
    #[inline(always)] fn as_raw_IOutlierRejector(&self) -> *mut c_void { self.ptr }
}

impl TranslationBasedLocalOutlierRejector {

    pub fn default() -> Result<crate::videostab::TranslationBasedLocalOutlierRejector> {
        unsafe { sys::cv_videostab_TranslationBasedLocalOutlierRejector_TranslationBasedLocalOutlierRejector() }.into_result().map(|ptr| crate::videostab::TranslationBasedLocalOutlierRejector { ptr })
    }
    
    pub fn set_cell_size(&mut self, val: core::Size) -> Result<()> {
        unsafe { sys::cv_videostab_TranslationBasedLocalOutlierRejector_setCellSize_Size(self.as_raw_TranslationBasedLocalOutlierRejector(), val) }.into_result()
    }
    
    pub fn cell_size(&self) -> Result<core::Size> {
        unsafe { sys::cv_videostab_TranslationBasedLocalOutlierRejector_cellSize_const(self.as_raw_TranslationBasedLocalOutlierRejector()) }.into_result()
    }
    
    pub fn process(&mut self, frame_size: core::Size, points0: &core::Mat, points1: &core::Mat, mask: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_videostab_TranslationBasedLocalOutlierRejector_process_Size_Mat_Mat_Mat(self.as_raw_TranslationBasedLocalOutlierRejector(), frame_size, points0.as_raw_Mat(), points1.as_raw_Mat(), mask.as_raw_Mat()) }.into_result()
    }
    
}

// boxed class cv::videostab::TwoPassStabilizer
pub struct TwoPassStabilizer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::TwoPassStabilizer {
    fn drop(&mut self) {
        unsafe { sys::cv_TwoPassStabilizer_delete(self.ptr) };
    }
}
impl crate::videostab::TwoPassStabilizer {
    #[inline(always)] pub fn as_raw_TwoPassStabilizer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for TwoPassStabilizer {}

impl crate::videostab::IFrameSource for TwoPassStabilizer {
    #[inline(always)] fn as_raw_IFrameSource(&self) -> *mut c_void { self.ptr }
}

impl crate::videostab::StabilizerBase for TwoPassStabilizer {
    #[inline(always)] fn as_raw_StabilizerBase(&self) -> *mut c_void { self.ptr }
}

impl TwoPassStabilizer {

    pub fn default() -> Result<crate::videostab::TwoPassStabilizer> {
        unsafe { sys::cv_videostab_TwoPassStabilizer_TwoPassStabilizer() }.into_result().map(|ptr| crate::videostab::TwoPassStabilizer { ptr })
    }
    
    pub fn set_motion_stabilizer(&mut self, val: &types::PtrOfIMotionStabilizer) -> Result<()> {
        unsafe { sys::cv_videostab_TwoPassStabilizer_setMotionStabilizer_PtrOfIMotionStabilizer(self.as_raw_TwoPassStabilizer(), val.as_raw_PtrOfIMotionStabilizer()) }.into_result()
    }
    
    pub fn motion_stabilizer(&self) -> Result<types::PtrOfIMotionStabilizer> {
        unsafe { sys::cv_videostab_TwoPassStabilizer_motionStabilizer_const(self.as_raw_TwoPassStabilizer()) }.into_result().map(|ptr| types::PtrOfIMotionStabilizer { ptr })
    }
    
    pub fn set_estimate_trim_ratio(&mut self, val: bool) -> Result<()> {
        unsafe { sys::cv_videostab_TwoPassStabilizer_setEstimateTrimRatio_bool(self.as_raw_TwoPassStabilizer(), val) }.into_result()
    }
    
    pub fn must_estimate_trima_ratio(&self) -> Result<bool> {
        unsafe { sys::cv_videostab_TwoPassStabilizer_mustEstimateTrimaRatio_const(self.as_raw_TwoPassStabilizer()) }.into_result()
    }
    
    pub fn reset(&mut self) -> Result<()> {
        unsafe { sys::cv_videostab_TwoPassStabilizer_reset(self.as_raw_TwoPassStabilizer()) }.into_result()
    }
    
    pub fn next_frame(&mut self) -> Result<core::Mat> {
        unsafe { sys::cv_videostab_TwoPassStabilizer_nextFrame(self.as_raw_TwoPassStabilizer()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
}

// boxed class cv::videostab::VideoFileSource
pub struct VideoFileSource {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::VideoFileSource {
    fn drop(&mut self) {
        unsafe { sys::cv_VideoFileSource_delete(self.ptr) };
    }
}
impl crate::videostab::VideoFileSource {
    #[inline(always)] pub fn as_raw_VideoFileSource(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for VideoFileSource {}

impl crate::videostab::IFrameSource for VideoFileSource {
    #[inline(always)] fn as_raw_IFrameSource(&self) -> *mut c_void { self.ptr }
}

impl VideoFileSource {

    ///
    /// ## C++ default parameters
    /// * volatile_frame: false
    pub fn new(path: &str, volatile_frame: bool) -> Result<crate::videostab::VideoFileSource> {
        string_arg!(path);
        unsafe { sys::cv_videostab_VideoFileSource_VideoFileSource_String_bool(path.as_ptr(), volatile_frame) }.into_result().map(|ptr| crate::videostab::VideoFileSource { ptr })
    }
    
    pub fn reset(&mut self) -> Result<()> {
        unsafe { sys::cv_videostab_VideoFileSource_reset(self.as_raw_VideoFileSource()) }.into_result()
    }
    
    pub fn next_frame(&mut self) -> Result<core::Mat> {
        unsafe { sys::cv_videostab_VideoFileSource_nextFrame(self.as_raw_VideoFileSource()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    pub fn width(&mut self) -> Result<i32> {
        unsafe { sys::cv_videostab_VideoFileSource_width(self.as_raw_VideoFileSource()) }.into_result()
    }
    
    pub fn height(&mut self) -> Result<i32> {
        unsafe { sys::cv_videostab_VideoFileSource_height(self.as_raw_VideoFileSource()) }.into_result()
    }
    
    pub fn count(&mut self) -> Result<i32> {
        unsafe { sys::cv_videostab_VideoFileSource_count(self.as_raw_VideoFileSource()) }.into_result()
    }
    
    pub fn fps(&mut self) -> Result<f64> {
        unsafe { sys::cv_videostab_VideoFileSource_fps(self.as_raw_VideoFileSource()) }.into_result()
    }
    
}

// boxed class cv::videostab::WeightingDeblurer
pub struct WeightingDeblurer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::videostab::WeightingDeblurer {
    fn drop(&mut self) {
        unsafe { sys::cv_WeightingDeblurer_delete(self.ptr) };
    }
}
impl crate::videostab::WeightingDeblurer {
    #[inline(always)] pub fn as_raw_WeightingDeblurer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for WeightingDeblurer {}

impl crate::videostab::DeblurerBase for WeightingDeblurer {
    #[inline(always)] fn as_raw_DeblurerBase(&self) -> *mut c_void { self.ptr }
}

impl WeightingDeblurer {

    pub fn default() -> Result<crate::videostab::WeightingDeblurer> {
        unsafe { sys::cv_videostab_WeightingDeblurer_WeightingDeblurer() }.into_result().map(|ptr| crate::videostab::WeightingDeblurer { ptr })
    }
    
    pub fn set_sensitivity(&mut self, val: f32) -> Result<()> {
        unsafe { sys::cv_videostab_WeightingDeblurer_setSensitivity_float(self.as_raw_WeightingDeblurer(), val) }.into_result()
    }
    
    pub fn sensitivity(&self) -> Result<f32> {
        unsafe { sys::cv_videostab_WeightingDeblurer_sensitivity_const(self.as_raw_WeightingDeblurer()) }.into_result()
    }
    
    pub fn deblur(&mut self, idx: i32, frame: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_videostab_WeightingDeblurer_deblur_int_Mat(self.as_raw_WeightingDeblurer(), idx, frame.as_raw_Mat()) }.into_result()
    }
    
}

// Generating impl for trait cv::videostab::WobbleSuppressorBase (trait)
pub trait WobbleSuppressorBase {
    #[inline(always)] fn as_raw_WobbleSuppressorBase(&self) -> *mut c_void;
    fn set_motion_estimator(&mut self, val: &types::PtrOfImageMotionEstimatorBase) -> Result<()> {
        unsafe { sys::cv_videostab_WobbleSuppressorBase_setMotionEstimator_PtrOfImageMotionEstimatorBase(self.as_raw_WobbleSuppressorBase(), val.as_raw_PtrOfImageMotionEstimatorBase()) }.into_result()
    }
    
    fn motion_estimator(&self) -> Result<types::PtrOfImageMotionEstimatorBase> {
        unsafe { sys::cv_videostab_WobbleSuppressorBase_motionEstimator_const(self.as_raw_WobbleSuppressorBase()) }.into_result().map(|ptr| types::PtrOfImageMotionEstimatorBase { ptr })
    }
    
    fn suppress(&mut self, idx: i32, frame: &core::Mat, result: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_videostab_WobbleSuppressorBase_suppress_int_Mat_Mat(self.as_raw_WobbleSuppressorBase(), idx, frame.as_raw_Mat(), result.as_raw_Mat()) }.into_result()
    }
    
    fn set_frame_count(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_videostab_WobbleSuppressorBase_setFrameCount_int(self.as_raw_WobbleSuppressorBase(), val) }.into_result()
    }
    
    fn frame_count(&self) -> Result<i32> {
        unsafe { sys::cv_videostab_WobbleSuppressorBase_frameCount_const(self.as_raw_WobbleSuppressorBase()) }.into_result()
    }
    
    fn set_motions(&mut self, val: &types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_videostab_WobbleSuppressorBase_setMotions_VectorOfMat(self.as_raw_WobbleSuppressorBase(), val.as_raw_VectorOfMat()) }.into_result()
    }
    
    fn motions(&self) -> Result<types::VectorOfMat> {
        unsafe { sys::cv_videostab_WobbleSuppressorBase_motions_const(self.as_raw_WobbleSuppressorBase()) }.into_result().map(|ptr| types::VectorOfMat { ptr })
    }
    
    fn set_motions2(&mut self, val: &types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_videostab_WobbleSuppressorBase_setMotions2_VectorOfMat(self.as_raw_WobbleSuppressorBase(), val.as_raw_VectorOfMat()) }.into_result()
    }
    
    fn motions2(&self) -> Result<types::VectorOfMat> {
        unsafe { sys::cv_videostab_WobbleSuppressorBase_motions2_const(self.as_raw_WobbleSuppressorBase()) }.into_result().map(|ptr| types::VectorOfMat { ptr })
    }
    
    fn set_stabilization_motions(&mut self, val: &types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_videostab_WobbleSuppressorBase_setStabilizationMotions_VectorOfMat(self.as_raw_WobbleSuppressorBase(), val.as_raw_VectorOfMat()) }.into_result()
    }
    
    fn stabilization_motions(&self) -> Result<types::VectorOfMat> {
        unsafe { sys::cv_videostab_WobbleSuppressorBase_stabilizationMotions_const(self.as_raw_WobbleSuppressorBase()) }.into_result().map(|ptr| types::VectorOfMat { ptr })
    }
    
}

