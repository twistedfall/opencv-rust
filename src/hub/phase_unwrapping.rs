//! # Phase Unwrapping API
//! 
//! Two-dimensional phase unwrapping is found in different applications like terrain elevation estimation
//! in synthetic aperture radar (SAR), field mapping in magnetic resonance imaging or as a way of finding
//! corresponding pixels in structured light reconstruction with sinusoidal patterns.
//! 
//! Given a phase map, wrapped between [-pi; pi], phase unwrapping aims at finding the "true" phase map
//! by adding the right number of 2*pi to each pixel.
//! 
//! The problem is straightforward for perfect wrapped phase map, but real data are usually not noise-free.
//! Among the different algorithms that were developed, quality-guided phase unwrapping methods are fast
//! and efficient. They follow a path that unwraps high quality pixels first,
//! avoiding error propagation from the start.
//! 
//! In this module, a quality-guided phase unwrapping is implemented following the approach described in [histogramUnwrapping](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_histogramUnwrapping) .
use std::os::raw::{c_char, c_void};
use libc::size_t;
use crate::{Error, Result, core, sys, types};


// Generating impl for trait cv::phase_unwrapping::HistogramPhaseUnwrapping (trait)
/// Class implementing two-dimensional phase unwrapping based on [histogramUnwrapping](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_histogramUnwrapping)
/// This algorithm belongs to the quality-guided phase unwrapping methods.
/// First, it computes a reliability map from second differences between a pixel and its eight neighbours.
/// Reliability values lie between 0 and 16*pi*pi. Then, this reliability map is used to compute
/// the reliabilities of "edges". An edge is an entity defined by two pixels that are connected
/// horizontally or vertically. Its reliability is found by adding the the reliabilities of the
/// two pixels connected through it. Edges are sorted in a histogram based on their reliability values.
/// This histogram is then used to unwrap pixels, starting from the highest quality pixel.
/// 
/// The wrapped phase map and the unwrapped result are stored in CV_32FC1 Mat.
pub trait HistogramPhaseUnwrapping : crate::phase_unwrapping::PhaseUnwrapping {
    #[doc(hidden)] fn as_raw_HistogramPhaseUnwrapping(&self) -> *mut c_void;
    /// Get the reliability map computed from the wrapped phase map.
    /// 
    /// ## Parameters
    /// * reliabilityMap: Image where the reliability map is stored.
    fn get_inverse_reliability_map(&mut self, reliability_map: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_phase_unwrapping_HistogramPhaseUnwrapping_getInverseReliabilityMap_Mat(self.as_raw_HistogramPhaseUnwrapping(), reliability_map.as_raw_Mat()) }.into_result()
    }
    
}

impl<'a> HistogramPhaseUnwrapping + 'a {

    /// Constructor
    /// 
    /// ## Parameters
    /// * parameters: HistogramPhaseUnwrapping parameters HistogramPhaseUnwrapping::Params: width,height of the phase map and histogram characteristics.
    ///
    /// ## C++ default parameters
    /// * parameters: HistogramPhaseUnwrapping::Params()
    pub fn create(parameters: &crate::phase_unwrapping::HistogramPhaseUnwrapping_Params) -> Result<types::PtrOfHistogramPhaseUnwrapping> {
        unsafe { sys::cv_phase_unwrapping_HistogramPhaseUnwrapping_create_Params(parameters.as_raw_HistogramPhaseUnwrapping_Params()) }.into_result().map(|x| types::PtrOfHistogramPhaseUnwrapping { ptr: x })
    }
    
}

// boxed class cv::phase_unwrapping::HistogramPhaseUnwrapping::Params
/// Parameters of phaseUnwrapping constructor.
/// 
/// ## Parameters
/// * width: Phase map width.
/// * height: Phase map height.
/// * histThresh: Bins in the histogram are not of equal size. Default value is 3*pi*pi. The one before "histThresh" value are smaller.
/// * nbrOfSmallBins: Number of bins between 0 and "histThresh". Default value is 10.
/// * nbrOfLargeBins: Number of bins between "histThresh" and 32*pi*pi (highest edge reliability value). Default value is 5.
#[allow(dead_code)]
pub struct HistogramPhaseUnwrapping_Params {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::phase_unwrapping::HistogramPhaseUnwrapping_Params {
    fn drop(&mut self) {
        unsafe { sys::cv_HistogramPhaseUnwrapping_Params_delete(self.ptr) };
    }
}
impl crate::phase_unwrapping::HistogramPhaseUnwrapping_Params {
    #[doc(hidden)] pub fn as_raw_HistogramPhaseUnwrapping_Params(&self) -> *mut c_void { self.ptr }
}

impl HistogramPhaseUnwrapping_Params {

    pub fn new() -> Result<crate::phase_unwrapping::HistogramPhaseUnwrapping_Params> {
        unsafe { sys::cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_Params() }.into_result().map(|x| crate::phase_unwrapping::HistogramPhaseUnwrapping_Params { ptr: x })
    }
    
}

// Generating impl for trait cv::phase_unwrapping::PhaseUnwrapping (trait)
/// Abstract base class for phase unwrapping.
pub trait PhaseUnwrapping : core::Algorithm {
    #[doc(hidden)] fn as_raw_PhaseUnwrapping(&self) -> *mut c_void;
    /// Unwraps a 2D phase map.
    /// 
    /// ## Parameters
    /// * wrappedPhaseMap: The wrapped phase map that needs to be unwrapped.
    /// * unwrappedPhaseMap: The unwrapped phase map.
    /// * shadowMask: Optional parameter used when some pixels do not hold any phase information in the wrapped phase map.
    ///
    /// ## C++ default parameters
    /// * shadow_mask: noArray()
    fn unwrap_phase_map(&mut self, wrapped_phase_map: &core::Mat, unwrapped_phase_map: &mut core::Mat, shadow_mask: &core::Mat) -> Result<()> {
        unsafe { sys::cv_phase_unwrapping_PhaseUnwrapping_unwrapPhaseMap_Mat_Mat_Mat(self.as_raw_PhaseUnwrapping(), wrapped_phase_map.as_raw_Mat(), unwrapped_phase_map.as_raw_Mat(), shadow_mask.as_raw_Mat()) }.into_result()
    }
    
}

impl<'a> PhaseUnwrapping + 'a {

}

