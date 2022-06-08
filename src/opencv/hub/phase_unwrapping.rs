#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
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
//! In this module, a quality-guided phase unwrapping is implemented following the approach described in [histogramUnwrapping](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_histogramUnwrapping) .
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::PhaseUnwrappingConst, super::PhaseUnwrapping, super::HistogramPhaseUnwrappingConst, super::HistogramPhaseUnwrapping };
}

/// Class implementing two-dimensional phase unwrapping based on [histogramUnwrapping](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_histogramUnwrapping)
/// This algorithm belongs to the quality-guided phase unwrapping methods.
/// First, it computes a reliability map from second differences between a pixel and its eight neighbours.
/// Reliability values lie between 0 and 16*pi*pi. Then, this reliability map is used to compute
/// the reliabilities of "edges". An edge is an entity defined by two pixels that are connected
/// horizontally or vertically. Its reliability is found by adding the the reliabilities of the
/// two pixels connected through it. Edges are sorted in a histogram based on their reliability values.
/// This histogram is then used to unwrap pixels, starting from the highest quality pixel.
/// 
/// The wrapped phase map and the unwrapped result are stored in CV_32FC1 Mat.
pub trait HistogramPhaseUnwrappingConst: crate::phase_unwrapping::PhaseUnwrappingConst {
	fn as_raw_HistogramPhaseUnwrapping(&self) -> *const c_void;

}

pub trait HistogramPhaseUnwrapping: crate::phase_unwrapping::HistogramPhaseUnwrappingConst + crate::phase_unwrapping::PhaseUnwrapping {
	fn as_raw_mut_HistogramPhaseUnwrapping(&mut self) -> *mut c_void;

	/// Get the reliability map computed from the wrapped phase map.
	/// 
	/// ## Parameters
	/// * reliabilityMap: Image where the reliability map is stored.
	#[inline]
	fn get_inverse_reliability_map(&mut self, reliability_map: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(reliability_map);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_phase_unwrapping_HistogramPhaseUnwrapping_getInverseReliabilityMap_const__OutputArrayR(self.as_raw_mut_HistogramPhaseUnwrapping(), reliability_map.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn HistogramPhaseUnwrapping + '_ {
	/// Constructor
	/// 
	/// ## Parameters
	/// * parameters: HistogramPhaseUnwrapping parameters HistogramPhaseUnwrapping::Params: width,height of the phase map and histogram characteristics.
	/// 
	/// ## C++ default parameters
	/// * parameters: HistogramPhaseUnwrapping::Params()
	#[inline]
	pub fn create(parameters: crate::phase_unwrapping::HistogramPhaseUnwrapping_Params) -> Result<core::Ptr<dyn crate::phase_unwrapping::HistogramPhaseUnwrapping>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_phase_unwrapping_HistogramPhaseUnwrapping_create_const_ParamsR(&parameters, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::phase_unwrapping::HistogramPhaseUnwrapping>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
/// Parameters of phaseUnwrapping constructor.
/// 
/// ## Parameters
/// * width: Phase map width.
/// * height: Phase map height.
/// * histThresh: Bins in the histogram are not of equal size. Default value is 3*pi*pi. The one before "histThresh" value are smaller.
/// * nbrOfSmallBins: Number of bins between 0 and "histThresh". Default value is 10.
/// * nbrOfLargeBins: Number of bins between "histThresh" and 32*pi*pi (highest edge reliability value). Default value is 5.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HistogramPhaseUnwrapping_Params {
	pub width: i32,
	pub height: i32,
	pub hist_thresh: f32,
	pub nbr_of_small_bins: i32,
	pub nbr_of_large_bins: i32,
}

opencv_type_simple! { crate::phase_unwrapping::HistogramPhaseUnwrapping_Params }

impl HistogramPhaseUnwrapping_Params {
	#[inline]
	pub fn default() -> Result<crate::phase_unwrapping::HistogramPhaseUnwrapping_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_phase_unwrapping_HistogramPhaseUnwrapping_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Abstract base class for phase unwrapping.
pub trait PhaseUnwrappingConst: core::AlgorithmTraitConst {
	fn as_raw_PhaseUnwrapping(&self) -> *const c_void;

}

pub trait PhaseUnwrapping: core::AlgorithmTrait + crate::phase_unwrapping::PhaseUnwrappingConst {
	fn as_raw_mut_PhaseUnwrapping(&mut self) -> *mut c_void;

	/// Unwraps a 2D phase map.
	/// 
	/// ## Parameters
	/// * wrappedPhaseMap: The wrapped phase map of type CV_32FC1 that needs to be unwrapped.
	/// * unwrappedPhaseMap: The unwrapped phase map.
	/// * shadowMask: Optional CV_8UC1 mask image used when some pixels do not hold any phase information in the wrapped phase map.
	/// 
	/// ## C++ default parameters
	/// * shadow_mask: noArray()
	#[inline]
	fn unwrap_phase_map(&mut self, wrapped_phase_map: &dyn core::ToInputArray, unwrapped_phase_map: &mut dyn core::ToOutputArray, shadow_mask: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(wrapped_phase_map);
		output_array_arg!(unwrapped_phase_map);
		input_array_arg!(shadow_mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_phase_unwrapping_PhaseUnwrapping_unwrapPhaseMap_const__InputArrayR_const__OutputArrayR_const__InputArrayR(self.as_raw_mut_PhaseUnwrapping(), wrapped_phase_map.as_raw__InputArray(), unwrapped_phase_map.as_raw__OutputArray(), shadow_mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}
