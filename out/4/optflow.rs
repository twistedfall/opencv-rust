//! # Optical Flow Algorithms
//!
//! Dense optical flow algorithms compute motion for each point:
//!
//! - cv::optflow::calcOpticalFlowSF
//! - cv::optflow::createOptFlow_DeepFlow
//!
//! Motion templates is alternative technique for detecting motion and computing its direction.
//! See samples/motempl.py.
//!
//! - cv::motempl::updateMotionHistory
//! - cv::motempl::calcMotionGradient
//! - cv::motempl::calcGlobalOrientation
//! - cv::motempl::segmentMotion
//!
//! Functions reading and writing .flo files in "Middlebury" format, see: <http://vision.middlebury.edu/flow/code/flow-code/README.txt>
//!
//! - cv::optflow::readOpticalFlow
//! - cv::optflow::writeOpticalFlow
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{DenseRLOFOpticalFlowTrait, DenseRLOFOpticalFlowTraitConst, DualTVL1OpticalFlowTrait, DualTVL1OpticalFlowTraitConst, GPCDetailsTrait, GPCDetailsTraitConst, GPCPatchDescriptorTrait, GPCPatchDescriptorTraitConst, GPCPatchSampleTrait, GPCPatchSampleTraitConst, GPCTrainingSamplesTrait, GPCTrainingSamplesTraitConst, GPCTreeTrait, GPCTreeTraitConst, OpticalFlowPCAFlowTrait, OpticalFlowPCAFlowTraitConst, PCAPriorTrait, PCAPriorTraitConst, RLOFOpticalFlowParameterTrait, RLOFOpticalFlowParameterTraitConst, SparseRLOFOpticalFlowTrait, SparseRLOFOpticalFlowTraitConst};
}

/// Better quality but slow
// GPC_DESCRIPTOR_DCT /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:92
pub const GPC_DESCRIPTOR_DCT: i32 = 0;
/// Worse quality but much faster
// GPC_DESCRIPTOR_WHT /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:93
pub const GPC_DESCRIPTOR_WHT: i32 = 1;
/// <  Edge-preserving interpolation using ximgproc::EdgeAwareInterpolator, see [Revaud2015](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Revaud2015),Geistert2016.
// INTERP_EPIC /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:33
pub const INTERP_EPIC: i32 = 1;
/// <  Fast geodesic interpolation, see [Geistert2016](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Geistert2016)
// INTERP_GEO /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:32
pub const INTERP_GEO: i32 = 0;
/// <  SLIC based robust interpolation using ximgproc::RICInterpolator, see [Hu2017](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Hu2017).
// INTERP_RIC /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:34
pub const INTERP_RIC: i32 = 2;
/// <  Apply a adaptive support region obtained by cross-based segmentation
/// as described in [Senst2014](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2014)
// SR_CROSS /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:19
pub const SR_CROSS: i32 = 1;
/// <  Apply a constant support region
// SR_FIXED /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:18
pub const SR_FIXED: i32 = 0;
/// < Apply optimized iterative refinement based bilinear equation solutions
/// as described in [Senst2013](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2013)
// ST_BILINEAR /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:25
pub const ST_BILINEAR: i32 = 1;
/// < Apply standard iterative refinement
// ST_STANDART /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:24
pub const ST_STANDART: i32 = 0;
/// Descriptor types for the Global Patch Collider.
// GPCDescType /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:90
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GPCDescType {
	/// Better quality but slow
	GPC_DESCRIPTOR_DCT = 0,
	/// Worse quality but much faster
	GPC_DESCRIPTOR_WHT = 1,
}

impl TryFrom<i32> for GPCDescType {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::GPC_DESCRIPTOR_DCT),
			1 => Ok(Self::GPC_DESCRIPTOR_WHT),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::optflow::GPCDescType"))),
		}
	}
}

opencv_type_enum! { crate::optflow::GPCDescType }

// InterpolationType /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:30
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InterpolationType {
	/// <  Fast geodesic interpolation, see [Geistert2016](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Geistert2016)
	INTERP_GEO = 0,
	/// <  Edge-preserving interpolation using ximgproc::EdgeAwareInterpolator, see [Revaud2015](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Revaud2015),Geistert2016.
	INTERP_EPIC = 1,
	/// <  SLIC based robust interpolation using ximgproc::RICInterpolator, see [Hu2017](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Hu2017).
	INTERP_RIC = 2,
}

impl TryFrom<i32> for InterpolationType {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::INTERP_GEO),
			1 => Ok(Self::INTERP_EPIC),
			2 => Ok(Self::INTERP_RIC),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::optflow::InterpolationType"))),
		}
	}
}

opencv_type_enum! { crate::optflow::InterpolationType }

// SolverType /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:23
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SolverType {
	/// < Apply standard iterative refinement
	ST_STANDART = 0,
	/// < Apply optimized iterative refinement based bilinear equation solutions
	/// as described in [Senst2013](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2013)
	ST_BILINEAR = 1,
}

impl TryFrom<i32> for SolverType {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::ST_STANDART),
			1 => Ok(Self::ST_BILINEAR),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::optflow::SolverType"))),
		}
	}
}

opencv_type_enum! { crate::optflow::SolverType }

// SupportRegionType /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:17
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SupportRegionType {
	/// <  Apply a constant support region
	SR_FIXED = 0,
	/// <  Apply a adaptive support region obtained by cross-based segmentation
	/// as described in [Senst2014](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2014)
	SR_CROSS = 1,
}

impl TryFrom<i32> for SupportRegionType {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::SR_FIXED),
			1 => Ok(Self::SR_CROSS),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::optflow::SupportRegionType"))),
		}
	}
}

opencv_type_enum! { crate::optflow::SupportRegionType }

// GPCSamplesVector /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:86
pub type GPCSamplesVector = core::Vector<crate::optflow::GPCPatchSample>;
/// Calculates a global motion orientation in a selected region.
///
/// ## Parameters
/// * orientation: Motion gradient orientation image calculated by the function calcMotionGradient
/// * mask: Mask image. It may be a conjunction of a valid gradient mask, also calculated by
/// calcMotionGradient , and the mask of a region whose direction needs to be calculated.
/// * mhi: Motion history image calculated by updateMotionHistory .
/// * timestamp: Timestamp passed to updateMotionHistory .
/// * duration: Maximum duration of a motion track in milliseconds, passed to updateMotionHistory
///
/// The function calculates an average motion direction in the selected region and returns the angle
/// between 0 degrees and 360 degrees. The average direction is computed from the weighted orientation
/// histogram, where a recent motion has a larger weight and the motion occurred in the past has a
/// smaller weight, as recorded in mhi .
// calcGlobalOrientation(InputArray, InputArray, InputArray, double, double)(InputArray, InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/motempl.hpp:119
// ("cv::motempl::calcGlobalOrientation", vec![(pred!(mut, ["orientation", "mask", "mhi", "timestamp", "duration"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "double", "double"]), _)]),
#[inline]
pub fn calc_global_orientation(orientation: &impl ToInputArray, mask: &impl ToInputArray, mhi: &impl ToInputArray, timestamp: f64, duration: f64) -> Result<f64> {
	input_array_arg!(orientation);
	input_array_arg!(mask);
	input_array_arg!(mhi);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_motempl_calcGlobalOrientation_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_double(orientation.as_raw__InputArray(), mask.as_raw__InputArray(), mhi.as_raw__InputArray(), timestamp, duration, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Calculates a gradient orientation of a motion history image.
///
/// ## Parameters
/// * mhi: Motion history single-channel floating-point image.
/// * mask: Output mask image that has the type CV_8UC1 and the same size as mhi . Its non-zero
/// elements mark pixels where the motion gradient data is correct.
/// * orientation: Output motion gradient orientation image that has the same type and the same
/// size as mhi . Each pixel of the image is a motion orientation, from 0 to 360 degrees.
/// * delta1: Minimal (or maximal) allowed difference between mhi values within a pixel
/// neighborhood.
/// * delta2: Maximal (or minimal) allowed difference between mhi values within a pixel
/// neighborhood. That is, the function finds the minimum ( ![inline formula](https://latex.codecogs.com/png.latex?m%28x%2Cy%29) ) and maximum ( ![inline formula](https://latex.codecogs.com/png.latex?M%28x%2Cy%29) ) mhi
/// values over ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%203) neighborhood of each pixel and marks the motion orientation at ![inline formula](https://latex.codecogs.com/png.latex?%28x%2C%20y%29)
/// as valid only if
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cmin%20%28%20%5Ctexttt%7Bdelta1%7D%20%20%2C%20%20%5Ctexttt%7Bdelta2%7D%20%20%29%20%20%5Cle%20%20M%28x%2Cy%29%2Dm%28x%2Cy%29%20%20%5Cle%20%20%20%5Cmax%20%28%20%5Ctexttt%7Bdelta1%7D%20%20%2C%20%5Ctexttt%7Bdelta2%7D%20%29%2E)
/// * apertureSize: Aperture size of the Sobel operator.
///
/// The function calculates a gradient orientation at each pixel ![inline formula](https://latex.codecogs.com/png.latex?%28x%2C%20y%29) as:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Borientation%7D%20%28x%2Cy%29%3D%20%5Carctan%7B%5Cfrac%7Bd%5Ctexttt%7Bmhi%7D%2Fdy%7D%7Bd%5Ctexttt%7Bmhi%7D%2Fdx%7D%7D)
///
/// In fact, fastAtan2 and phase are used so that the computed angle is measured in degrees and covers
/// the full range 0..360. Also, the mask is filled to indicate pixels where the computed angle is
/// valid.
///
///
/// Note:
///    *   (Python) An example on how to perform a motion template technique can be found at
///        opencv_source_code/samples/python2/motempl.py
///
/// ## Note
/// This alternative version of [calc_motion_gradient] function uses the following default values for its arguments:
/// * aperture_size: 3
// cv::motempl::calcMotionGradient(InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/motempl.hpp:102
// ("cv::motempl::calcMotionGradient", vec![(pred!(mut, ["mhi", "mask", "orientation", "delta1", "delta2"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "double"]), _)]),
#[inline]
pub fn calc_motion_gradient_def(mhi: &impl ToInputArray, mask: &mut impl ToOutputArray, orientation: &mut impl ToOutputArray, delta1: f64, delta2: f64) -> Result<()> {
	input_array_arg!(mhi);
	output_array_arg!(mask);
	output_array_arg!(orientation);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_motempl_calcMotionGradient_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double(mhi.as_raw__InputArray(), mask.as_raw__OutputArray(), orientation.as_raw__OutputArray(), delta1, delta2, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Calculates a gradient orientation of a motion history image.
///
/// ## Parameters
/// * mhi: Motion history single-channel floating-point image.
/// * mask: Output mask image that has the type CV_8UC1 and the same size as mhi . Its non-zero
/// elements mark pixels where the motion gradient data is correct.
/// * orientation: Output motion gradient orientation image that has the same type and the same
/// size as mhi . Each pixel of the image is a motion orientation, from 0 to 360 degrees.
/// * delta1: Minimal (or maximal) allowed difference between mhi values within a pixel
/// neighborhood.
/// * delta2: Maximal (or minimal) allowed difference between mhi values within a pixel
/// neighborhood. That is, the function finds the minimum ( ![inline formula](https://latex.codecogs.com/png.latex?m%28x%2Cy%29) ) and maximum ( ![inline formula](https://latex.codecogs.com/png.latex?M%28x%2Cy%29) ) mhi
/// values over ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%203) neighborhood of each pixel and marks the motion orientation at ![inline formula](https://latex.codecogs.com/png.latex?%28x%2C%20y%29)
/// as valid only if
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cmin%20%28%20%5Ctexttt%7Bdelta1%7D%20%20%2C%20%20%5Ctexttt%7Bdelta2%7D%20%20%29%20%20%5Cle%20%20M%28x%2Cy%29%2Dm%28x%2Cy%29%20%20%5Cle%20%20%20%5Cmax%20%28%20%5Ctexttt%7Bdelta1%7D%20%20%2C%20%5Ctexttt%7Bdelta2%7D%20%29%2E)
/// * apertureSize: Aperture size of the Sobel operator.
///
/// The function calculates a gradient orientation at each pixel ![inline formula](https://latex.codecogs.com/png.latex?%28x%2C%20y%29) as:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Borientation%7D%20%28x%2Cy%29%3D%20%5Carctan%7B%5Cfrac%7Bd%5Ctexttt%7Bmhi%7D%2Fdy%7D%7Bd%5Ctexttt%7Bmhi%7D%2Fdx%7D%7D)
///
/// In fact, fastAtan2 and phase are used so that the computed angle is measured in degrees and covers
/// the full range 0..360. Also, the mask is filled to indicate pixels where the computed angle is
/// valid.
///
///
/// Note:
///    *   (Python) An example on how to perform a motion template technique can be found at
///        opencv_source_code/samples/python2/motempl.py
///
/// ## C++ default parameters
/// * aperture_size: 3
// calcMotionGradient(InputArray, OutputArray, OutputArray, double, double, int)(InputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/motempl.hpp:102
// ("cv::motempl::calcMotionGradient", vec![(pred!(mut, ["mhi", "mask", "orientation", "delta1", "delta2", "apertureSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double", "double", "int"]), _)]),
#[inline]
pub fn calc_motion_gradient(mhi: &impl ToInputArray, mask: &mut impl ToOutputArray, orientation: &mut impl ToOutputArray, delta1: f64, delta2: f64, aperture_size: i32) -> Result<()> {
	input_array_arg!(mhi);
	output_array_arg!(mask);
	output_array_arg!(orientation);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_motempl_calcMotionGradient_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double_int(mhi.as_raw__InputArray(), mask.as_raw__OutputArray(), orientation.as_raw__OutputArray(), delta1, delta2, aperture_size, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Splits a motion history image into a few parts corresponding to separate independent motions (for
/// example, left hand, right hand).
///
/// ## Parameters
/// * mhi: Motion history image.
/// * segmask: Image where the found mask should be stored, single-channel, 32-bit floating-point.
/// * boundingRects: Vector containing ROIs of motion connected components.
/// * timestamp: Current time in milliseconds or other units.
/// * segThresh: Segmentation threshold that is recommended to be equal to the interval between
/// motion history "steps" or greater.
///
/// The function finds all of the motion segments and marks them in segmask with individual values
/// (1,2,...). It also computes a vector with ROIs of motion connected components. After that the motion
/// direction for every component can be calculated with calcGlobalOrientation using the extracted mask
/// of the particular component.
// segmentMotion(InputArray, OutputArray, std::vector<Rect> &, double, double)(InputArray, OutputArray, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/motempl.hpp:137
// ("cv::motempl::segmentMotion", vec![(pred!(mut, ["mhi", "segmask", "boundingRects", "timestamp", "segThresh"], ["const cv::_InputArray*", "const cv::_OutputArray*", "std::vector<cv::Rect>*", "double", "double"]), _)]),
#[inline]
pub fn segment_motion(mhi: &impl ToInputArray, segmask: &mut impl ToOutputArray, bounding_rects: &mut core::Vector<core::Rect>, timestamp: f64, seg_thresh: f64) -> Result<()> {
	input_array_arg!(mhi);
	output_array_arg!(segmask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_motempl_segmentMotion_const__InputArrayR_const__OutputArrayR_vectorLRectGR_double_double(mhi.as_raw__InputArray(), segmask.as_raw__OutputArray(), bounding_rects.as_raw_mut_VectorOfRect(), timestamp, seg_thresh, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Updates the motion history image by a moving silhouette.
///
/// ## Parameters
/// * silhouette: Silhouette mask that has non-zero pixels where the motion occurs.
/// * mhi: Motion history image that is updated by the function (single-channel, 32-bit
/// floating-point).
/// * timestamp: Current time in milliseconds or other units.
/// * duration: Maximal duration of the motion track in the same units as timestamp .
///
/// The function updates the motion history image as follows:
///
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bmhi%7D%20%28x%2Cy%29%3D%20%5Cforkthree%7B%5Ctexttt%7Btimestamp%7D%7D%7Bif%20%5C%28%5Ctexttt%7Bsilhouette%7D%28x%2Cy%29%20%5Cne%200%5C%29%7D%7B0%7D%7Bif%20%5C%28%5Ctexttt%7Bsilhouette%7D%28x%2Cy%29%20%3D%200%5C%29%20and%20%5C%28%5Ctexttt%7Bmhi%7D%20%3C%20%28%5Ctexttt%7Btimestamp%7D%20%2D%20%5Ctexttt%7Bduration%7D%29%5C%29%7D%7B%5Ctexttt%7Bmhi%7D%28x%2Cy%29%7D%7Botherwise%7D)
///
/// That is, MHI pixels where the motion occurs are set to the current timestamp , while the pixels
/// where the motion happened last time a long time ago are cleared.
///
/// The function, together with calcMotionGradient and calcGlobalOrientation , implements a motion
/// templates technique described in [Davis97](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Davis97) and [Bradski00](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Bradski00) .
// updateMotionHistory(InputArray, InputOutputArray, double, double)(InputArray, InputOutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/motempl.hpp:71
// ("cv::motempl::updateMotionHistory", vec![(pred!(mut, ["silhouette", "mhi", "timestamp", "duration"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "double", "double"]), _)]),
#[inline]
pub fn update_motion_history(silhouette: &impl ToInputArray, mhi: &mut impl ToInputOutputArray, timestamp: f64, duration: f64) -> Result<()> {
	input_array_arg!(silhouette);
	input_output_array_arg!(mhi);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_motempl_updateMotionHistory_const__InputArrayR_const__InputOutputArrayR_double_double(silhouette.as_raw__InputArray(), mhi.as_raw__InputOutputArray(), timestamp, duration, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Fast dense optical flow computation based on robust local optical flow (RLOF) algorithms and sparse-to-dense interpolation scheme.
///
/// The RLOF is a fast local optical flow approach described in [Senst2012](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2012) [Senst2013](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2013) [Senst2014](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2014)
/// and [Senst2016](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2016) similar to the pyramidal iterative Lucas-Kanade method as
/// proposed by [Bouguet00](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Bouguet00). More details and experiments can be found in the following thesis [Senst2019](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2019).
/// The implementation is derived from optflow::calcOpticalFlowPyrLK().
///
/// The sparse-to-dense interpolation scheme allows for fast computation of dense optical flow using RLOF (see [Geistert2016](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Geistert2016)).
/// For this scheme the following steps are applied:
/// -# motion vector seeded at a regular sampled grid are computed. The sparsity of this grid can be configured with setGridStep
/// -# (optinally) errornous motion vectors are filter based on the forward backward confidence. The threshold can be configured
/// with setForwardBackward. The filter is only applied if the threshold >0 but than the runtime is doubled due to the estimation
/// of the backward flow.
/// -# Vector field interpolation is applied to the motion vector set to obtain a dense vector field.
///
/// ## Parameters
/// * I0: first 8-bit input image. If The cross-based RLOF is used (by selecting optflow::RLOFOpticalFlowParameter::supportRegionType
/// = SupportRegionType::SR_CROSS) image has to be a 8-bit 3 channel image.
/// * I1: second 8-bit input image. If The cross-based RLOF is used (by selecting optflow::RLOFOpticalFlowParameter::supportRegionType
/// = SupportRegionType::SR_CROSS) image has to be a 8-bit 3 channel image.
/// * flow: computed flow image that has the same size as I0 and type CV_32FC2.
/// * rlofParam: see optflow::RLOFOpticalFlowParameter
/// * forwardBackwardThreshold: Threshold for the forward backward confidence check.
/// For each grid point ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cmathbf%7Bx%7D%20) a motion vector ![inline formula](https://latex.codecogs.com/png.latex?%20d%5F%7BI0%2CI1%7D%28%5Cmathbf%7Bx%7D%29%20) is computed.
/// If the forward backward error ![block formula](https://latex.codecogs.com/png.latex?%20EP%5F%7BFB%7D%20%3D%20%7C%7C%20d%5F%7BI0%2CI1%7D%20%2B%20d%5F%7BI1%2CI0%7D%20%7C%7C%20)
/// is larger than threshold given by this function then the motion vector will not be used by the following
/// vector field interpolation. ![inline formula](https://latex.codecogs.com/png.latex?%20d%5F%7BI1%2CI0%7D%20) denotes the backward flow. Note, the forward backward test
///    will only be applied if the threshold > 0. This may results into a doubled runtime for the motion estimation.
/// * gridStep: Size of the grid to spawn the motion vectors. For each grid point a motion vector is computed.
/// Some motion vectors will be removed due to the forwatd backward threshold (if set >0). The rest will be the
/// base of the vector field interpolation.
/// * interp_type: interpolation method used to compute the dense optical flow. Two interpolation algorithms are
/// supported:
/// - **INTERP_GEO** applies the fast geodesic interpolation, see [Geistert2016](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Geistert2016).
/// - **INTERP_EPIC_RESIDUAL** applies the edge-preserving interpolation, see [Revaud2015](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Revaud2015),Geistert2016.
/// * epicK: see ximgproc::EdgeAwareInterpolator sets the respective parameter.
/// * epicSigma: see ximgproc::EdgeAwareInterpolator sets the respective parameter.
/// * epicLambda: see ximgproc::EdgeAwareInterpolator sets the respective parameter.
/// * ricSPSize: see ximgproc::RICInterpolator sets the respective parameter.
/// * ricSLICType: see ximgproc::RICInterpolator sets the respective parameter.
/// * use_post_proc: enables ximgproc::fastGlobalSmootherFilter() parameter.
/// * fgsLambda: sets the respective ximgproc::fastGlobalSmootherFilter() parameter.
/// * fgsSigma: sets the respective ximgproc::fastGlobalSmootherFilter() parameter.
/// * use_variational_refinement: enables VariationalRefinement
///
/// Parameters have been described in [Senst2012](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2012), [Senst2013](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2013), [Senst2014](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2014), [Senst2016](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2016).
/// For the RLOF configuration see optflow::RLOFOpticalFlowParameter for further details.
///
/// Note: If the grid size is set to (1,1) and the forward backward threshold <= 0 that the dense optical flow field is purely
/// computed with the RLOF.
///
///
/// Note: SIMD parallelization is only available when compiling with SSE4.1.
///
/// Note: Note that in output, if no correspondences are found between \a I0 and \a I1, the \a flow is set to 0.
/// ## See also
/// optflow::DenseRLOFOpticalFlow, optflow::RLOFOpticalFlowParameter
///
/// ## Note
/// This alternative version of [calc_optical_flow_dense_rlof] function uses the following default values for its arguments:
/// * rlof_param: Ptr<RLOFOpticalFlowParameter>()
/// * forward_backward_threshold: 0
/// * grid_step: Size(6,6)
/// * interp_type: InterpolationType::INTERP_EPIC
/// * epic_k: 128
/// * epic_sigma: 0.05f
/// * epic_lambda: 100.f
/// * ric_sp_size: 15
/// * ric_slic_type: 100
/// * use_post_proc: true
/// * fgs_lambda: 500.0f
/// * fgs_sigma: 1.5f
/// * use_variational_refinement: false
// cv::optflow::calcOpticalFlowDenseRLOF(InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:501
// ("cv::optflow::calcOpticalFlowDenseRLOF", vec![(pred!(mut, ["I0", "I1", "flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
#[inline]
pub fn calc_optical_flow_dense_rlof_def(i0: &impl ToInputArray, i1: &impl ToInputArray, flow: &mut impl ToInputOutputArray) -> Result<()> {
	input_array_arg!(i0);
	input_array_arg!(i1);
	input_output_array_arg!(flow);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_calcOpticalFlowDenseRLOF_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(i0.as_raw__InputArray(), i1.as_raw__InputArray(), flow.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Fast dense optical flow computation based on robust local optical flow (RLOF) algorithms and sparse-to-dense interpolation scheme.
///
/// The RLOF is a fast local optical flow approach described in [Senst2012](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2012) [Senst2013](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2013) [Senst2014](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2014)
/// and [Senst2016](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2016) similar to the pyramidal iterative Lucas-Kanade method as
/// proposed by [Bouguet00](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Bouguet00). More details and experiments can be found in the following thesis [Senst2019](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2019).
/// The implementation is derived from optflow::calcOpticalFlowPyrLK().
///
/// The sparse-to-dense interpolation scheme allows for fast computation of dense optical flow using RLOF (see [Geistert2016](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Geistert2016)).
/// For this scheme the following steps are applied:
/// -# motion vector seeded at a regular sampled grid are computed. The sparsity of this grid can be configured with setGridStep
/// -# (optinally) errornous motion vectors are filter based on the forward backward confidence. The threshold can be configured
/// with setForwardBackward. The filter is only applied if the threshold >0 but than the runtime is doubled due to the estimation
/// of the backward flow.
/// -# Vector field interpolation is applied to the motion vector set to obtain a dense vector field.
///
/// ## Parameters
/// * I0: first 8-bit input image. If The cross-based RLOF is used (by selecting optflow::RLOFOpticalFlowParameter::supportRegionType
/// = SupportRegionType::SR_CROSS) image has to be a 8-bit 3 channel image.
/// * I1: second 8-bit input image. If The cross-based RLOF is used (by selecting optflow::RLOFOpticalFlowParameter::supportRegionType
/// = SupportRegionType::SR_CROSS) image has to be a 8-bit 3 channel image.
/// * flow: computed flow image that has the same size as I0 and type CV_32FC2.
/// * rlofParam: see optflow::RLOFOpticalFlowParameter
/// * forwardBackwardThreshold: Threshold for the forward backward confidence check.
/// For each grid point ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cmathbf%7Bx%7D%20) a motion vector ![inline formula](https://latex.codecogs.com/png.latex?%20d%5F%7BI0%2CI1%7D%28%5Cmathbf%7Bx%7D%29%20) is computed.
/// If the forward backward error ![block formula](https://latex.codecogs.com/png.latex?%20EP%5F%7BFB%7D%20%3D%20%7C%7C%20d%5F%7BI0%2CI1%7D%20%2B%20d%5F%7BI1%2CI0%7D%20%7C%7C%20)
/// is larger than threshold given by this function then the motion vector will not be used by the following
/// vector field interpolation. ![inline formula](https://latex.codecogs.com/png.latex?%20d%5F%7BI1%2CI0%7D%20) denotes the backward flow. Note, the forward backward test
///    will only be applied if the threshold > 0. This may results into a doubled runtime for the motion estimation.
/// * gridStep: Size of the grid to spawn the motion vectors. For each grid point a motion vector is computed.
/// Some motion vectors will be removed due to the forwatd backward threshold (if set >0). The rest will be the
/// base of the vector field interpolation.
/// * interp_type: interpolation method used to compute the dense optical flow. Two interpolation algorithms are
/// supported:
/// - **INTERP_GEO** applies the fast geodesic interpolation, see [Geistert2016](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Geistert2016).
/// - **INTERP_EPIC_RESIDUAL** applies the edge-preserving interpolation, see [Revaud2015](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Revaud2015),Geistert2016.
/// * epicK: see ximgproc::EdgeAwareInterpolator sets the respective parameter.
/// * epicSigma: see ximgproc::EdgeAwareInterpolator sets the respective parameter.
/// * epicLambda: see ximgproc::EdgeAwareInterpolator sets the respective parameter.
/// * ricSPSize: see ximgproc::RICInterpolator sets the respective parameter.
/// * ricSLICType: see ximgproc::RICInterpolator sets the respective parameter.
/// * use_post_proc: enables ximgproc::fastGlobalSmootherFilter() parameter.
/// * fgsLambda: sets the respective ximgproc::fastGlobalSmootherFilter() parameter.
/// * fgsSigma: sets the respective ximgproc::fastGlobalSmootherFilter() parameter.
/// * use_variational_refinement: enables VariationalRefinement
///
/// Parameters have been described in [Senst2012](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2012), [Senst2013](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2013), [Senst2014](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2014), [Senst2016](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2016).
/// For the RLOF configuration see optflow::RLOFOpticalFlowParameter for further details.
///
/// Note: If the grid size is set to (1,1) and the forward backward threshold <= 0 that the dense optical flow field is purely
/// computed with the RLOF.
///
///
/// Note: SIMD parallelization is only available when compiling with SSE4.1.
///
/// Note: Note that in output, if no correspondences are found between \a I0 and \a I1, the \a flow is set to 0.
/// ## See also
/// optflow::DenseRLOFOpticalFlow, optflow::RLOFOpticalFlowParameter
///
/// ## C++ default parameters
/// * rlof_param: Ptr<RLOFOpticalFlowParameter>()
/// * forward_backward_threshold: 0
/// * grid_step: Size(6,6)
/// * interp_type: InterpolationType::INTERP_EPIC
/// * epic_k: 128
/// * epic_sigma: 0.05f
/// * epic_lambda: 100.f
/// * ric_sp_size: 15
/// * ric_slic_type: 100
/// * use_post_proc: true
/// * fgs_lambda: 500.0f
/// * fgs_sigma: 1.5f
/// * use_variational_refinement: false
// calcOpticalFlowDenseRLOF(InputArray, InputArray, InputOutputArray, Ptr<RLOFOpticalFlowParameter>, float, Size, InterpolationType, int, float, float, int, int, bool, float, float, bool)(InputArray, InputArray, InputOutputArray, CppPassByVoidPtr, Primitive, SimpleClass, Enum, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:501
// ("cv::optflow::calcOpticalFlowDenseRLOF", vec![(pred!(mut, ["I0", "I1", "flow", "rlofParam", "forwardBackwardThreshold", "gridStep", "interp_type", "epicK", "epicSigma", "epicLambda", "ricSPSize", "ricSLICType", "use_post_proc", "fgsLambda", "fgsSigma", "use_variational_refinement"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>", "float", "cv::Size", "cv::optflow::InterpolationType", "int", "float", "float", "int", "int", "bool", "float", "float", "bool"]), _)]),
#[inline]
pub fn calc_optical_flow_dense_rlof(i0: &impl ToInputArray, i1: &impl ToInputArray, flow: &mut impl ToInputOutputArray, mut rlof_param: core::Ptr<crate::optflow::RLOFOpticalFlowParameter>, forward_backward_threshold: f32, grid_step: core::Size, interp_type: crate::optflow::InterpolationType, epic_k: i32, epic_sigma: f32, epic_lambda: f32, ric_sp_size: i32, ric_slic_type: i32, use_post_proc: bool, fgs_lambda: f32, fgs_sigma: f32, use_variational_refinement: bool) -> Result<()> {
	input_array_arg!(i0);
	input_array_arg!(i1);
	input_output_array_arg!(flow);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_calcOpticalFlowDenseRLOF_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_PtrLRLOFOpticalFlowParameterG_float_Size_InterpolationType_int_float_float_int_int_bool_float_float_bool(i0.as_raw__InputArray(), i1.as_raw__InputArray(), flow.as_raw__InputOutputArray(), rlof_param.as_raw_mut_PtrOfRLOFOpticalFlowParameter(), forward_backward_threshold, &grid_step, interp_type, epic_k, epic_sigma, epic_lambda, ric_sp_size, ric_slic_type, use_post_proc, fgs_lambda, fgs_sigma, use_variational_refinement, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Calculate an optical flow using "SimpleFlow" algorithm.
///
/// ## Parameters
/// * from: First 8-bit 3-channel image.
/// * to: Second 8-bit 3-channel image of the same size as prev
/// * flow: computed flow image that has the same size as prev and type CV_32FC2
/// * layers: Number of layers
/// * averaging_block_size: Size of block through which we sum up when calculate cost function
/// for pixel
/// * max_flow: maximal flow that we search at each level
/// * sigma_dist: vector smooth spatial sigma parameter
/// * sigma_color: vector smooth color sigma parameter
/// * postprocess_window: window size for postprocess cross bilateral filter
/// * sigma_dist_fix: spatial sigma for postprocess cross bilateralf filter
/// * sigma_color_fix: color sigma for postprocess cross bilateral filter
/// * occ_thr: threshold for detecting occlusions
/// * upscale_averaging_radius: window size for bilateral upscale operation
/// * upscale_sigma_dist: spatial sigma for bilateral upscale operation
/// * upscale_sigma_color: color sigma for bilateral upscale operation
/// * speed_up_thr: threshold to detect point with irregular flow - where flow should be
/// recalculated after upscale
///
/// See [Tao2012](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Tao2012) . And site of project - <http://graphics.berkeley.edu/papers/Tao-SAN-2012-05/>.
///
///
/// Note:
///    *   An example using the simpleFlow algorithm can be found at samples/simpleflow_demo.cpp
///
/// ## Overloaded parameters
// calcOpticalFlowSF(InputArray, InputArray, OutputArray, int, int, int)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:81
// ("cv::optflow::calcOpticalFlowSF", vec![(pred!(mut, ["from", "to", "flow", "layers", "averaging_block_size", "max_flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
#[inline]
pub fn calc_optical_flow_sf(from: &impl ToInputArray, to: &impl ToInputArray, flow: &mut impl ToOutputArray, layers: i32, averaging_block_size: i32, max_flow: i32) -> Result<()> {
	input_array_arg!(from);
	input_array_arg!(to);
	output_array_arg!(flow);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_calcOpticalFlowSF_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int(from.as_raw__InputArray(), to.as_raw__InputArray(), flow.as_raw__OutputArray(), layers, averaging_block_size, max_flow, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Calculate an optical flow using "SimpleFlow" algorithm.
///
/// ## Parameters
/// * from: First 8-bit 3-channel image.
/// * to: Second 8-bit 3-channel image of the same size as prev
/// * flow: computed flow image that has the same size as prev and type CV_32FC2
/// * layers: Number of layers
/// * averaging_block_size: Size of block through which we sum up when calculate cost function
/// for pixel
/// * max_flow: maximal flow that we search at each level
/// * sigma_dist: vector smooth spatial sigma parameter
/// * sigma_color: vector smooth color sigma parameter
/// * postprocess_window: window size for postprocess cross bilateral filter
/// * sigma_dist_fix: spatial sigma for postprocess cross bilateralf filter
/// * sigma_color_fix: color sigma for postprocess cross bilateral filter
/// * occ_thr: threshold for detecting occlusions
/// * upscale_averaging_radius: window size for bilateral upscale operation
/// * upscale_sigma_dist: spatial sigma for bilateral upscale operation
/// * upscale_sigma_color: color sigma for bilateral upscale operation
/// * speed_up_thr: threshold to detect point with irregular flow - where flow should be
/// recalculated after upscale
///
/// See [Tao2012](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Tao2012) . And site of project - <http://graphics.berkeley.edu/papers/Tao-SAN-2012-05/>.
///
///
/// Note:
///    *   An example using the simpleFlow algorithm can be found at samples/simpleflow_demo.cpp
// calcOpticalFlowSF(InputArray, InputArray, OutputArray, int, int, int, double, double, int, double, double, double, int, double, double, double)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:110
// ("cv::optflow::calcOpticalFlowSF", vec![(pred!(mut, ["from", "to", "flow", "layers", "averaging_block_size", "max_flow", "sigma_dist", "sigma_color", "postprocess_window", "sigma_dist_fix", "sigma_color_fix", "occ_thr", "upscale_averaging_radius", "upscale_sigma_dist", "upscale_sigma_color", "speed_up_thr"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int", "double", "double", "int", "double", "double", "double", "int", "double", "double", "double"]), _)]),
#[inline]
pub fn calc_optical_flow_sf_1(from: &impl ToInputArray, to: &impl ToInputArray, flow: &mut impl ToOutputArray, layers: i32, averaging_block_size: i32, max_flow: i32, sigma_dist: f64, sigma_color: f64, postprocess_window: i32, sigma_dist_fix: f64, sigma_color_fix: f64, occ_thr: f64, upscale_averaging_radius: i32, upscale_sigma_dist: f64, upscale_sigma_color: f64, speed_up_thr: f64) -> Result<()> {
	input_array_arg!(from);
	input_array_arg!(to);
	output_array_arg!(flow);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_calcOpticalFlowSF_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int_double_double_int_double_double_double_int_double_double_double(from.as_raw__InputArray(), to.as_raw__InputArray(), flow.as_raw__OutputArray(), layers, averaging_block_size, max_flow, sigma_dist, sigma_color, postprocess_window, sigma_dist_fix, sigma_color_fix, occ_thr, upscale_averaging_radius, upscale_sigma_dist, upscale_sigma_color, speed_up_thr, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Calculates fast optical flow for a sparse feature set using the robust local optical flow (RLOF) similar
/// to optflow::calcOpticalFlowPyrLK().
///
/// The RLOF is a fast local optical flow approach described in [Senst2012](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2012) [Senst2013](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2013) [Senst2014](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2014)
/// and [Senst2016](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2016) similar to the pyramidal iterative Lucas-Kanade method as
/// proposed by [Bouguet00](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Bouguet00). More details and experiments can be found in the following thesis [Senst2019](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2019).
/// The implementation is derived from optflow::calcOpticalFlowPyrLK().
///
/// ## Parameters
/// * prevImg: first 8-bit input image. If The cross-based RLOF is used (by selecting optflow::RLOFOpticalFlowParameter::supportRegionType
/// = SupportRegionType::SR_CROSS) image has to be a 8-bit 3 channel image.
/// * nextImg: second 8-bit input image. If The cross-based RLOF is used (by selecting optflow::RLOFOpticalFlowParameter::supportRegionType
/// = SupportRegionType::SR_CROSS) image has to be a 8-bit 3 channel image.
/// * prevPts: vector of 2D points for which the flow needs to be found; point coordinates must be single-precision
/// floating-point numbers.
/// * nextPts: output vector of 2D points (with single-precision floating-point coordinates) containing the calculated
/// new positions of input features in the second image; when optflow::RLOFOpticalFlowParameter::useInitialFlow variable is true  the vector must
/// have the same size as in the input and contain the initialization point correspondences.
/// * status: output status vector (of unsigned chars); each element of the vector is set to 1 if the flow for the
/// corresponding features has passed the forward backward check.
/// * err: output vector of errors; each element of the vector is set to the forward backward error for the corresponding feature.
/// * rlofParam: see optflow::RLOFOpticalFlowParameter
/// * forwardBackwardThreshold: Threshold for the forward backward confidence check. If forewardBackwardThreshold <=0 the forward
///
///
/// Note: SIMD parallelization is only available when compiling with SSE4.1.
///
/// Parameters have been described in [Senst2012](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2012), [Senst2013](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2013), [Senst2014](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2014) and [Senst2016](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2016).
/// For the RLOF configuration see optflow::RLOFOpticalFlowParameter for further details.
///
/// ## Note
/// This alternative version of [calc_optical_flow_sparse_rlof] function uses the following default values for its arguments:
/// * rlof_param: Ptr<RLOFOpticalFlowParameter>()
/// * forward_backward_threshold: 0
// cv::optflow::calcOpticalFlowSparseRLOF(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:538
// ("cv::optflow::calcOpticalFlowSparseRLOF", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status", "err"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn calc_optical_flow_sparse_rlof_def(prev_img: &impl ToInputArray, next_img: &impl ToInputArray, prev_pts: &impl ToInputArray, next_pts: &mut impl ToInputOutputArray, status: &mut impl ToOutputArray, err: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(prev_img);
	input_array_arg!(next_img);
	input_array_arg!(prev_pts);
	input_output_array_arg!(next_pts);
	output_array_arg!(status);
	output_array_arg!(err);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_calcOpticalFlowSparseRLOF_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(prev_img.as_raw__InputArray(), next_img.as_raw__InputArray(), prev_pts.as_raw__InputArray(), next_pts.as_raw__InputOutputArray(), status.as_raw__OutputArray(), err.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Calculates fast optical flow for a sparse feature set using the robust local optical flow (RLOF) similar
/// to optflow::calcOpticalFlowPyrLK().
///
/// The RLOF is a fast local optical flow approach described in [Senst2012](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2012) [Senst2013](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2013) [Senst2014](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2014)
/// and [Senst2016](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2016) similar to the pyramidal iterative Lucas-Kanade method as
/// proposed by [Bouguet00](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Bouguet00). More details and experiments can be found in the following thesis [Senst2019](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2019).
/// The implementation is derived from optflow::calcOpticalFlowPyrLK().
///
/// ## Parameters
/// * prevImg: first 8-bit input image. If The cross-based RLOF is used (by selecting optflow::RLOFOpticalFlowParameter::supportRegionType
/// = SupportRegionType::SR_CROSS) image has to be a 8-bit 3 channel image.
/// * nextImg: second 8-bit input image. If The cross-based RLOF is used (by selecting optflow::RLOFOpticalFlowParameter::supportRegionType
/// = SupportRegionType::SR_CROSS) image has to be a 8-bit 3 channel image.
/// * prevPts: vector of 2D points for which the flow needs to be found; point coordinates must be single-precision
/// floating-point numbers.
/// * nextPts: output vector of 2D points (with single-precision floating-point coordinates) containing the calculated
/// new positions of input features in the second image; when optflow::RLOFOpticalFlowParameter::useInitialFlow variable is true  the vector must
/// have the same size as in the input and contain the initialization point correspondences.
/// * status: output status vector (of unsigned chars); each element of the vector is set to 1 if the flow for the
/// corresponding features has passed the forward backward check.
/// * err: output vector of errors; each element of the vector is set to the forward backward error for the corresponding feature.
/// * rlofParam: see optflow::RLOFOpticalFlowParameter
/// * forwardBackwardThreshold: Threshold for the forward backward confidence check. If forewardBackwardThreshold <=0 the forward
///
///
/// Note: SIMD parallelization is only available when compiling with SSE4.1.
///
/// Parameters have been described in [Senst2012](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2012), [Senst2013](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2013), [Senst2014](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2014) and [Senst2016](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2016).
/// For the RLOF configuration see optflow::RLOFOpticalFlowParameter for further details.
///
/// ## C++ default parameters
/// * rlof_param: Ptr<RLOFOpticalFlowParameter>()
/// * forward_backward_threshold: 0
// calcOpticalFlowSparseRLOF(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray, Ptr<RLOFOpticalFlowParameter>, float)(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:538
// ("cv::optflow::calcOpticalFlowSparseRLOF", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status", "err", "rlofParam", "forwardBackwardThreshold"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>", "float"]), _)]),
#[inline]
pub fn calc_optical_flow_sparse_rlof(prev_img: &impl ToInputArray, next_img: &impl ToInputArray, prev_pts: &impl ToInputArray, next_pts: &mut impl ToInputOutputArray, status: &mut impl ToOutputArray, err: &mut impl ToOutputArray, mut rlof_param: core::Ptr<crate::optflow::RLOFOpticalFlowParameter>, forward_backward_threshold: f32) -> Result<()> {
	input_array_arg!(prev_img);
	input_array_arg!(next_img);
	input_array_arg!(prev_pts);
	input_output_array_arg!(next_pts);
	output_array_arg!(status);
	output_array_arg!(err);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_calcOpticalFlowSparseRLOF_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_PtrLRLOFOpticalFlowParameterG_float(prev_img.as_raw__InputArray(), next_img.as_raw__InputArray(), prev_pts.as_raw__InputArray(), next_pts.as_raw__InputOutputArray(), status.as_raw__OutputArray(), err.as_raw__OutputArray(), rlof_param.as_raw_mut_PtrOfRLOFOpticalFlowParameter(), forward_backward_threshold, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Fast dense optical flow based on PyrLK sparse matches interpolation.
///
/// ## Parameters
/// * from: first 8-bit 3-channel or 1-channel image.
/// * to: second 8-bit 3-channel or 1-channel image of the same size as from
/// * flow: computed flow image that has the same size as from and CV_32FC2 type
/// * grid_step: stride used in sparse match computation. Lower values usually
///        result in higher quality but slow down the algorithm.
/// * k: number of nearest-neighbor matches considered, when fitting a locally affine
///        model. Lower values can make the algorithm noticeably faster at the cost of
///        some quality degradation.
/// * sigma: parameter defining how fast the weights decrease in the locally-weighted affine
///        fitting. Higher values can help preserve fine details, lower values can help to get rid
///        of the noise in the output flow.
/// * use_post_proc: defines whether the ximgproc::fastGlobalSmootherFilter() is used
///        for post-processing after interpolation
/// * fgs_lambda: see the respective parameter of the ximgproc::fastGlobalSmootherFilter()
/// * fgs_sigma: see the respective parameter of the ximgproc::fastGlobalSmootherFilter()
///
/// ## Note
/// This alternative version of [calc_optical_flow_sparse_to_dense] function uses the following default values for its arguments:
/// * grid_step: 8
/// * k: 128
/// * sigma: 0.05f
/// * use_post_proc: true
/// * fgs_lambda: 500.0f
/// * fgs_sigma: 1.5f
// cv::optflow::calcOpticalFlowSparseToDense(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:135
// ("cv::optflow::calcOpticalFlowSparseToDense", vec![(pred!(mut, ["from", "to", "flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn calc_optical_flow_sparse_to_dense_def(from: &impl ToInputArray, to: &impl ToInputArray, flow: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(from);
	input_array_arg!(to);
	output_array_arg!(flow);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_calcOpticalFlowSparseToDense_const__InputArrayR_const__InputArrayR_const__OutputArrayR(from.as_raw__InputArray(), to.as_raw__InputArray(), flow.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Fast dense optical flow based on PyrLK sparse matches interpolation.
///
/// ## Parameters
/// * from: first 8-bit 3-channel or 1-channel image.
/// * to: second 8-bit 3-channel or 1-channel image of the same size as from
/// * flow: computed flow image that has the same size as from and CV_32FC2 type
/// * grid_step: stride used in sparse match computation. Lower values usually
///        result in higher quality but slow down the algorithm.
/// * k: number of nearest-neighbor matches considered, when fitting a locally affine
///        model. Lower values can make the algorithm noticeably faster at the cost of
///        some quality degradation.
/// * sigma: parameter defining how fast the weights decrease in the locally-weighted affine
///        fitting. Higher values can help preserve fine details, lower values can help to get rid
///        of the noise in the output flow.
/// * use_post_proc: defines whether the ximgproc::fastGlobalSmootherFilter() is used
///        for post-processing after interpolation
/// * fgs_lambda: see the respective parameter of the ximgproc::fastGlobalSmootherFilter()
/// * fgs_sigma: see the respective parameter of the ximgproc::fastGlobalSmootherFilter()
///
/// ## C++ default parameters
/// * grid_step: 8
/// * k: 128
/// * sigma: 0.05f
/// * use_post_proc: true
/// * fgs_lambda: 500.0f
/// * fgs_sigma: 1.5f
// calcOpticalFlowSparseToDense(InputArray, InputArray, OutputArray, int, int, float, bool, float, float)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:135
// ("cv::optflow::calcOpticalFlowSparseToDense", vec![(pred!(mut, ["from", "to", "flow", "grid_step", "k", "sigma", "use_post_proc", "fgs_lambda", "fgs_sigma"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "float", "bool", "float", "float"]), _)]),
#[inline]
pub fn calc_optical_flow_sparse_to_dense(from: &impl ToInputArray, to: &impl ToInputArray, flow: &mut impl ToOutputArray, grid_step: i32, k: i32, sigma: f32, use_post_proc: bool, fgs_lambda: f32, fgs_sigma: f32) -> Result<()> {
	input_array_arg!(from);
	input_array_arg!(to);
	output_array_arg!(flow);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_calcOpticalFlowSparseToDense_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_float_bool_float_float(from.as_raw__InputArray(), to.as_raw__InputArray(), flow.as_raw__OutputArray(), grid_step, k, sigma, use_post_proc, fgs_lambda, fgs_sigma, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// DeepFlow optical flow algorithm implementation.
///
/// The class implements the DeepFlow optical flow algorithm described in [Weinzaepfel2013](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Weinzaepfel2013) . See
/// also <http://lear.inrialpes.fr/src/deepmatching/> .
/// Parameters - class fields - that may be modified after creating a class instance:
/// *   member float alpha
/// Smoothness assumption weight
/// *   member float delta
/// Color constancy assumption weight
/// *   member float gamma
/// Gradient constancy weight
/// *   member float sigma
/// Gaussian smoothing parameter
/// *   member int minSize
/// Minimal dimension of an image in the pyramid (next, smaller images in the pyramid are generated
/// until one of the dimensions reaches this size)
/// *   member float downscaleFactor
/// Scaling factor in the image pyramid (must be \< 1)
/// *   member int fixedPointIterations
/// How many iterations on each level of the pyramid
/// *   member int sorIterations
/// Iterations of Succesive Over-Relaxation (solver)
/// *   member float omega
/// Relaxation factor in SOR
// createOptFlow_DeepFlow()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:165
// ("cv::optflow::createOptFlow_DeepFlow", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn create_opt_flow_deep_flow() -> Result<core::Ptr<crate::video::DenseOpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_createOptFlow_DeepFlow(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::video::DenseOpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Additional interface to the Dense RLOF algorithm - optflow::calcOpticalFlowDenseRLOF()
// createOptFlow_DenseRLOF()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:545
// ("cv::optflow::createOptFlow_DenseRLOF", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn create_opt_flow_dense_rlof() -> Result<core::Ptr<crate::video::DenseOpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_createOptFlow_DenseRLOF(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::video::DenseOpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Creates instance of cv::DenseOpticalFlow
// createOptFlow_DualTVL1()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:300
// ("cv::optflow::createOptFlow_DualTVL1", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn create_opt_flow_dual_tvl1() -> Result<core::Ptr<crate::optflow::DualTVL1OpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_createOptFlow_DualTVL1(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::optflow::DualTVL1OpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Additional interface to the Farneback's algorithm - calcOpticalFlowFarneback()
// createOptFlow_Farneback()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:171
// ("cv::optflow::createOptFlow_Farneback", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn create_opt_flow_farneback() -> Result<core::Ptr<crate::video::DenseOpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_createOptFlow_Farneback(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::video::DenseOpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Creates an instance of PCAFlow
// createOptFlow_PCAFlow()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/pcaflow.hpp:142
// ("cv::optflow::createOptFlow_PCAFlow", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn create_opt_flow_pca_flow() -> Result<core::Ptr<crate::video::DenseOpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_createOptFlow_PCAFlow(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::video::DenseOpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Additional interface to the SimpleFlow algorithm - calcOpticalFlowSF()
// createOptFlow_SimpleFlow()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:168
// ("cv::optflow::createOptFlow_SimpleFlow", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn create_opt_flow_simple_flow() -> Result<core::Ptr<crate::video::DenseOpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_createOptFlow_SimpleFlow(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::video::DenseOpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Additional interface to the Sparse RLOF algorithm - optflow::calcOpticalFlowSparseRLOF()
// createOptFlow_SparseRLOF()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:548
// ("cv::optflow::createOptFlow_SparseRLOF", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn create_opt_flow_sparse_rlof() -> Result<core::Ptr<crate::video::SparseOpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_createOptFlow_SparseRLOF(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::video::SparseOpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Additional interface to the SparseToDenseFlow algorithm - calcOpticalFlowSparseToDense()
// createOptFlow_SparseToDense()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:174
// ("cv::optflow::createOptFlow_SparseToDense", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn create_opt_flow_sparse_to_dense() -> Result<core::Ptr<crate::video::DenseOpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_createOptFlow_SparseToDense(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::video::DenseOpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

// read(const FileNode &, optflow::GPCTree::Node &, optflow::GPCTree::Node)(TraitClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:369
// ("cv::read", vec![(pred!(mut, ["fn", "node", "unnamed"], ["const cv::FileNode*", "cv::optflow::GPCTree::Node*", "cv::optflow::GPCTree::Node"]), _)]),
#[inline]
pub fn read(fn_: &impl core::FileNodeTraitConst, node: &mut crate::optflow::GPCTree_Node, unnamed: crate::optflow::GPCTree_Node) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_read_const_FileNodeR_NodeR_Node(fn_.as_raw_FileNode(), node, &unnamed, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// write(FileStorage &, const String &, const optflow::GPCTree::Node &)(TraitClass, InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:367
// ("cv::write", vec![(pred!(mut, ["fs", "name", "node"], ["cv::FileStorage*", "const cv::String*", "const cv::optflow::GPCTree::Node*"]), _)]),
#[inline]
pub fn write(fs: &mut impl core::FileStorageTrait, name: &str, node: crate::optflow::GPCTree_Node) -> Result<()> {
	extern_container_arg!(name);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_write_FileStorageR_const_StringR_const_NodeR(fs.as_raw_mut_FileStorage(), name.opencv_as_extern(), &node, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Constant methods for [crate::optflow::DenseRLOFOpticalFlow]
// DenseRLOFOpticalFlow /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:233
pub trait DenseRLOFOpticalFlowTraitConst: crate::video::DenseOpticalFlowTraitConst {
	fn as_raw_DenseRLOFOpticalFlow(&self) -> *const c_void;

	/// Configuration of the RLOF alogrithm.
	/// ## See also
	/// optflow::RLOFOpticalFlowParameter, getRLOFOpticalFlowParameter
	/// optflow::RLOFOpticalFlowParameter, setRLOFOpticalFlowParameter
	// getRLOFOpticalFlowParameter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:244
	// ("cv::optflow::DenseRLOFOpticalFlow::getRLOFOpticalFlowParameter", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_rlof_optical_flow_parameter(&self) -> Result<core::Ptr<crate::optflow::RLOFOpticalFlowParameter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getRLOFOpticalFlowParameter_const(self.as_raw_DenseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::optflow::RLOFOpticalFlowParameter>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Threshold for the forward backward confidence check
	/// For each grid point ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cmathbf%7Bx%7D%20) a motion vector ![inline formula](https://latex.codecogs.com/png.latex?%20d%5F%7BI0%2CI1%7D%28%5Cmathbf%7Bx%7D%29%20) is computed.
	///      *     If the forward backward error ![block formula](https://latex.codecogs.com/png.latex?%20EP%5F%7BFB%7D%20%3D%20%7C%7C%20d%5F%7BI0%2CI1%7D%20%2B%20d%5F%7BI1%2CI0%7D%20%7C%7C%20)
	///      *     is larger than threshold given by this function then the motion vector will not be used by the following
	///      *    vector field interpolation. ![inline formula](https://latex.codecogs.com/png.latex?%20d%5F%7BI1%2CI0%7D%20) denotes the backward flow. Note, the forward backward test
	///      *    will only be applied if the threshold > 0. This may results into a doubled runtime for the motion estimation.
	///      *    getForwardBackward, setGridStep
	/// ## See also
	/// setForwardBackward
	// getForwardBackward()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:257
	// ("cv::optflow::DenseRLOFOpticalFlow::getForwardBackward", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_forward_backward(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getForwardBackward_const(self.as_raw_DenseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Size of the grid to spawn the motion vectors.
	/// For each grid point a motion vector is computed. Some motion vectors will be removed due to the forwatd backward
	///      *  threshold (if set >0). The rest will be the base of the vector field interpolation.
	///      *    see also: getForwardBackward, setGridStep
	// getGridStep()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:263
	// ("cv::optflow::DenseRLOFOpticalFlow::getGridStep", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_grid_step(&self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getGridStep_const(self.as_raw_DenseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Interpolation used to compute the dense optical flow.
	/// Two interpolation algorithms are supported
	///      * - **INTERP_GEO** applies the fast geodesic interpolation, see [Geistert2016](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Geistert2016).
	///      * - **INTERP_EPIC_RESIDUAL** applies the edge-preserving interpolation, see [Revaud2015](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Revaud2015),Geistert2016.
	///      * ximgproc::EdgeAwareInterpolator, getInterpolation
	/// ## See also
	/// ximgproc::EdgeAwareInterpolator, setInterpolation
	// getInterpolation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:279
	// ("cv::optflow::DenseRLOFOpticalFlow::getInterpolation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_interpolation(&self) -> Result<crate::optflow::InterpolationType> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getInterpolation_const(self.as_raw_DenseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// see ximgproc::EdgeAwareInterpolator() K value.
	/// K is a number of nearest-neighbor matches considered, when fitting a locally affine
	///      *    model. Usually it should be around 128. However, lower values would make the interpolation noticeably faster.
	///      *    see also: ximgproc::EdgeAwareInterpolator,  setEPICK
	// getEPICK()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:285
	// ("cv::optflow::DenseRLOFOpticalFlow::getEPICK", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_epick(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getEPICK_const(self.as_raw_DenseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// see ximgproc::EdgeAwareInterpolator() sigma value.
	/// Sigma is a parameter defining how fast the weights decrease in the locally-weighted affine
	///      *  fitting. Higher values can help preserve fine details, lower values can help to get rid of noise in the
	///      *  output flow.
	///      *    see also: ximgproc::EdgeAwareInterpolator, setEPICSigma
	// getEPICSigma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:296
	// ("cv::optflow::DenseRLOFOpticalFlow::getEPICSigma", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_epic_sigma(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getEPICSigma_const(self.as_raw_DenseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// see ximgproc::EdgeAwareInterpolator() lambda value.
	/// Lambda is a parameter defining the weight of the edge-aware term in geodesic distance,
	///      *    should be in the range of 0 to 1000.
	///      *    see also: ximgproc::EdgeAwareInterpolator, setEPICSigma
	// getEPICLambda()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:306
	// ("cv::optflow::DenseRLOFOpticalFlow::getEPICLambda", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_epic_lambda(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getEPICLambda_const(self.as_raw_DenseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// see ximgproc::EdgeAwareInterpolator().
	/// Sets the respective fastGlobalSmootherFilter() parameter.
	///      *    see also: ximgproc::EdgeAwareInterpolator, setFgsLambda
	// getFgsLambda()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:315
	// ("cv::optflow::DenseRLOFOpticalFlow::getFgsLambda", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_fgs_lambda(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getFgsLambda_const(self.as_raw_DenseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// see ximgproc::EdgeAwareInterpolator().
	/// Sets the respective fastGlobalSmootherFilter() parameter.
	///      *    see also: ximgproc::EdgeAwareInterpolator, ximgproc::fastGlobalSmootherFilter, setFgsSigma
	// getFgsSigma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:324
	// ("cv::optflow::DenseRLOFOpticalFlow::getFgsSigma", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_fgs_sigma(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getFgsSigma_const(self.as_raw_DenseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// enables ximgproc::fastGlobalSmootherFilter
	///
	/// * getUsePostProc
	/// ## See also
	/// ximgproc::fastGlobalSmootherFilter, setUsePostProc
	// getUsePostProc()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:337
	// ("cv::optflow::DenseRLOFOpticalFlow::getUsePostProc", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_use_post_proc(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getUsePostProc_const(self.as_raw_DenseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// enables VariationalRefinement
	///
	/// * getUseVariationalRefinement
	/// ## See also
	/// ximgproc::fastGlobalSmootherFilter, setUsePostProc
	// getUseVariationalRefinement()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:346
	// ("cv::optflow::DenseRLOFOpticalFlow::getUseVariationalRefinement", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_use_variational_refinement(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getUseVariationalRefinement_const(self.as_raw_DenseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Parameter to tune the approximate size of the superpixel used for oversegmentation.
	///
	/// * cv::ximgproc::createSuperpixelSLIC, cv::ximgproc::RICInterpolator
	/// ## See also
	/// setRICSPSize
	// getRICSPSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:355
	// ("cv::optflow::DenseRLOFOpticalFlow::getRICSPSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_ricsp_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getRICSPSize_const(self.as_raw_DenseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Parameter to choose superpixel algorithm variant to use:
	/// - cv::ximgproc::SLICType SLIC segments image using a desired region_size (value: 100)
	/// - cv::ximgproc::SLICType SLICO will optimize using adaptive compactness factor (value: 101)
	/// - cv::ximgproc::SLICType MSLIC will optimize using manifold methods resulting in more content-sensitive superpixels (value: 102).
	/// ## See also
	/// cv::ximgproc::createSuperpixelSLIC, cv::ximgproc::RICInterpolator
	/// setRICSLICType
	// getRICSLICType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:366
	// ("cv::optflow::DenseRLOFOpticalFlow::getRICSLICType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_ricslic_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getRICSLICType_const(self.as_raw_DenseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::optflow::DenseRLOFOpticalFlow]
pub trait DenseRLOFOpticalFlowTrait: crate::optflow::DenseRLOFOpticalFlowTraitConst + crate::video::DenseOpticalFlowTrait {
	fn as_raw_mut_DenseRLOFOpticalFlow(&mut self) -> *mut c_void;

	/// Configuration of the RLOF alogrithm.
	/// ## See also
	/// optflow::RLOFOpticalFlowParameter, getRLOFOpticalFlowParameter
	// setRLOFOpticalFlowParameter(Ptr<RLOFOpticalFlowParameter>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:240
	// ("cv::optflow::DenseRLOFOpticalFlow::setRLOFOpticalFlowParameter", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>"]), _)]),
	#[inline]
	fn set_rlof_optical_flow_parameter(&mut self, mut val: core::Ptr<crate::optflow::RLOFOpticalFlowParameter>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setRLOFOpticalFlowParameter_PtrLRLOFOpticalFlowParameterG(self.as_raw_mut_DenseRLOFOpticalFlow(), val.as_raw_mut_PtrOfRLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Threshold for the forward backward confidence check
	/// For each grid point ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cmathbf%7Bx%7D%20) a motion vector ![inline formula](https://latex.codecogs.com/png.latex?%20d%5F%7BI0%2CI1%7D%28%5Cmathbf%7Bx%7D%29%20) is computed.
	///      *     If the forward backward error ![block formula](https://latex.codecogs.com/png.latex?%20EP%5F%7BFB%7D%20%3D%20%7C%7C%20d%5F%7BI0%2CI1%7D%20%2B%20d%5F%7BI1%2CI0%7D%20%7C%7C%20)
	///      *     is larger than threshold given by this function then the motion vector will not be used by the following
	///      *    vector field interpolation. ![inline formula](https://latex.codecogs.com/png.latex?%20d%5F%7BI1%2CI0%7D%20) denotes the backward flow. Note, the forward backward test
	///      *    will only be applied if the threshold > 0. This may results into a doubled runtime for the motion estimation.
	///      *    see also: getForwardBackward, setGridStep
	// setForwardBackward(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:253
	// ("cv::optflow::DenseRLOFOpticalFlow::setForwardBackward", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_forward_backward(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setForwardBackward_float(self.as_raw_mut_DenseRLOFOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Size of the grid to spawn the motion vectors.
	/// For each grid point a motion vector is computed. Some motion vectors will be removed due to the forwatd backward
	///      *  threshold (if set >0). The rest will be the base of the vector field interpolation.
	///      *    getForwardBackward, setGridStep
	/// ## See also
	/// getGridStep
	// setGridStep(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:267
	// ("cv::optflow::DenseRLOFOpticalFlow::setGridStep", vec![(pred!(mut, ["val"], ["cv::Size"]), _)]),
	#[inline]
	fn set_grid_step(&mut self, val: core::Size) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setGridStep_Size(self.as_raw_mut_DenseRLOFOpticalFlow(), &val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Interpolation used to compute the dense optical flow.
	/// Two interpolation algorithms are supported
	///      * - **INTERP_GEO** applies the fast geodesic interpolation, see [Geistert2016](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Geistert2016).
	///      * - **INTERP_EPIC_RESIDUAL** applies the edge-preserving interpolation, see [Revaud2015](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Revaud2015),Geistert2016.
	///      * see also: ximgproc::EdgeAwareInterpolator, getInterpolation
	// setInterpolation(InterpolationType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:275
	// ("cv::optflow::DenseRLOFOpticalFlow::setInterpolation", vec![(pred!(mut, ["val"], ["cv::optflow::InterpolationType"]), _)]),
	#[inline]
	fn set_interpolation(&mut self, val: crate::optflow::InterpolationType) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setInterpolation_InterpolationType(self.as_raw_mut_DenseRLOFOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// see ximgproc::EdgeAwareInterpolator() K value.
	/// K is a number of nearest-neighbor matches considered, when fitting a locally affine
	///      *    model. Usually it should be around 128. However, lower values would make the interpolation noticeably faster.
	///      *    ximgproc::EdgeAwareInterpolator,  setEPICK
	/// ## See also
	/// ximgproc::EdgeAwareInterpolator, getEPICK
	// setEPICK(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:289
	// ("cv::optflow::DenseRLOFOpticalFlow::setEPICK", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_epick(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setEPICK_int(self.as_raw_mut_DenseRLOFOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// see ximgproc::EdgeAwareInterpolator() sigma value.
	/// Sigma is a parameter defining how fast the weights decrease in the locally-weighted affine
	///      *  fitting. Higher values can help preserve fine details, lower values can help to get rid of noise in the
	///      *  output flow.
	///      *    ximgproc::EdgeAwareInterpolator, setEPICSigma
	/// ## See also
	/// ximgproc::EdgeAwareInterpolator, getEPICSigma
	// setEPICSigma(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:300
	// ("cv::optflow::DenseRLOFOpticalFlow::setEPICSigma", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_epic_sigma(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setEPICSigma_float(self.as_raw_mut_DenseRLOFOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// see ximgproc::EdgeAwareInterpolator() lambda value.
	/// Lambda is a parameter defining the weight of the edge-aware term in geodesic distance,
	///      *    should be in the range of 0 to 1000.
	///      *    ximgproc::EdgeAwareInterpolator, setEPICSigma
	/// ## See also
	/// ximgproc::EdgeAwareInterpolator, getEPICLambda
	// setEPICLambda(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:310
	// ("cv::optflow::DenseRLOFOpticalFlow::setEPICLambda", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_epic_lambda(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setEPICLambda_float(self.as_raw_mut_DenseRLOFOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// see ximgproc::EdgeAwareInterpolator().
	/// Sets the respective fastGlobalSmootherFilter() parameter.
	///      *    ximgproc::EdgeAwareInterpolator, setFgsLambda
	/// ## See also
	/// ximgproc::EdgeAwareInterpolator, ximgproc::fastGlobalSmootherFilter, getFgsLambda
	// setFgsLambda(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:319
	// ("cv::optflow::DenseRLOFOpticalFlow::setFgsLambda", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_fgs_lambda(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setFgsLambda_float(self.as_raw_mut_DenseRLOFOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// see ximgproc::EdgeAwareInterpolator().
	/// Sets the respective fastGlobalSmootherFilter() parameter.
	///      *    ximgproc::EdgeAwareInterpolator, ximgproc::fastGlobalSmootherFilter, setFgsSigma
	/// ## See also
	/// ximgproc::EdgeAwareInterpolator, ximgproc::fastGlobalSmootherFilter, getFgsSigma
	// setFgsSigma(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:328
	// ("cv::optflow::DenseRLOFOpticalFlow::setFgsSigma", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_fgs_sigma(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setFgsSigma_float(self.as_raw_mut_DenseRLOFOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// enables ximgproc::fastGlobalSmootherFilter
	///
	/// * see also: getUsePostProc
	// setUsePostProc(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:333
	// ("cv::optflow::DenseRLOFOpticalFlow::setUsePostProc", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	#[inline]
	fn set_use_post_proc(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setUsePostProc_bool(self.as_raw_mut_DenseRLOFOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// enables VariationalRefinement
	///
	/// * see also: getUseVariationalRefinement
	// setUseVariationalRefinement(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:342
	// ("cv::optflow::DenseRLOFOpticalFlow::setUseVariationalRefinement", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	#[inline]
	fn set_use_variational_refinement(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setUseVariationalRefinement_bool(self.as_raw_mut_DenseRLOFOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Parameter to tune the approximate size of the superpixel used for oversegmentation.
	///
	/// * see also: cv::ximgproc::createSuperpixelSLIC, cv::ximgproc::RICInterpolator
	// setRICSPSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:351
	// ("cv::optflow::DenseRLOFOpticalFlow::setRICSPSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_ricsp_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setRICSPSize_int(self.as_raw_mut_DenseRLOFOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Parameter to choose superpixel algorithm variant to use:
	/// - cv::ximgproc::SLICType SLIC segments image using a desired region_size (value: 100)
	/// - cv::ximgproc::SLICType SLICO will optimize using adaptive compactness factor (value: 101)
	/// - cv::ximgproc::SLICType MSLIC will optimize using manifold methods resulting in more content-sensitive superpixels (value: 102).
	/// ## See also
	/// cv::ximgproc::createSuperpixelSLIC, cv::ximgproc::RICInterpolator
	// setRICSLICType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:362
	// ("cv::optflow::DenseRLOFOpticalFlow::setRICSLICType", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_ricslic_type(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setRICSLICType_int(self.as_raw_mut_DenseRLOFOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Fast dense optical flow computation based on robust local optical flow (RLOF) algorithms and sparse-to-dense interpolation
/// scheme.
///
/// The RLOF is a fast local optical flow approach described in [Senst2012](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2012) [Senst2013](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2013) [Senst2014](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2014)
/// and [Senst2016](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2016) similar to the pyramidal iterative Lucas-Kanade method as
/// proposed by [Bouguet00](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Bouguet00). More details and experiments can be found in the following thesis [Senst2019](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2019).
/// The implementation is derived from optflow::calcOpticalFlowPyrLK().
///
/// The sparse-to-dense interpolation scheme allows for fast computation of dense optical flow using RLOF (see [Geistert2016](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Geistert2016)).
/// For this scheme the following steps are applied:
/// -# motion vector seeded at a regular sampled grid are computed. The sparsity of this grid can be configured with setGridStep
/// -# (optinally) errornous motion vectors are filter based on the forward backward confidence. The threshold can be configured
/// with setForwardBackward. The filter is only applied if the threshold >0 but than the runtime is doubled due to the estimation
/// of the backward flow.
/// -# Vector field interpolation is applied to the motion vector set to obtain a dense vector field.
///
/// For the RLOF configuration see optflow::RLOFOpticalFlowParameter for further details.
/// Parameters have been described in [Senst2012](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2012) [Senst2013](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2013) [Senst2014](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2014) and [Senst2016](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2016).
///
///
/// Note: If the grid size is set to (1,1) and the forward backward threshold <= 0 than pixelwise dense optical flow field is
/// computed by RLOF without using interpolation.
///
///
/// Note: Note that in output, if no correspondences are found between \a I0 and \a I1, the \a flow is set to 0.
/// ## See also
/// optflow::calcOpticalFlowDenseRLOF(), optflow::RLOFOpticalFlowParameter
// DenseRLOFOpticalFlow /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:233
pub struct DenseRLOFOpticalFlow {
	ptr: *mut c_void,
}

opencv_type_boxed! { DenseRLOFOpticalFlow }

impl Drop for DenseRLOFOpticalFlow {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_delete(self.as_raw_mut_DenseRLOFOpticalFlow()) };
	}
}

unsafe impl Send for DenseRLOFOpticalFlow {}

impl core::AlgorithmTraitConst for DenseRLOFOpticalFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for DenseRLOFOpticalFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DenseRLOFOpticalFlow, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::video::DenseOpticalFlowTraitConst for DenseRLOFOpticalFlow {
	#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::DenseOpticalFlowTrait for DenseRLOFOpticalFlow {
	#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DenseRLOFOpticalFlow, crate::video::DenseOpticalFlowTraitConst, as_raw_DenseOpticalFlow, crate::video::DenseOpticalFlowTrait, as_raw_mut_DenseOpticalFlow }

impl crate::optflow::DenseRLOFOpticalFlowTraitConst for DenseRLOFOpticalFlow {
	#[inline] fn as_raw_DenseRLOFOpticalFlow(&self) -> *const c_void { self.as_raw() }
}

impl crate::optflow::DenseRLOFOpticalFlowTrait for DenseRLOFOpticalFlow {
	#[inline] fn as_raw_mut_DenseRLOFOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DenseRLOFOpticalFlow, crate::optflow::DenseRLOFOpticalFlowTraitConst, as_raw_DenseRLOFOpticalFlow, crate::optflow::DenseRLOFOpticalFlowTrait, as_raw_mut_DenseRLOFOpticalFlow }

impl DenseRLOFOpticalFlow {
	/// Creates instance of optflow::DenseRLOFOpticalFlow
	///
	/// ## Parameters
	/// * rlofParam: see optflow::RLOFOpticalFlowParameter
	/// * forwardBackwardThreshold: see setForwardBackward
	/// * gridStep: see setGridStep
	/// * interp_type: see setInterpolation
	/// * epicK: see setEPICK
	/// * epicSigma: see setEPICSigma
	/// * epicLambda: see setEPICLambda
	/// * ricSPSize: see setRICSPSize
	/// * ricSLICType: see setRICSLICType
	/// * use_post_proc: see setUsePostProc
	/// * fgsLambda: see setFgsLambda
	/// * fgsSigma: see setFgsSigma
	/// * use_variational_refinement: see setUseVariationalRefinement
	///
	/// ## C++ default parameters
	/// * rlof_param: Ptr<RLOFOpticalFlowParameter>()
	/// * forward_backward_threshold: 1.f
	/// * grid_step: Size(6,6)
	/// * interp_type: InterpolationType::INTERP_EPIC
	/// * epic_k: 128
	/// * epic_sigma: 0.05f
	/// * epic_lambda: 999.0f
	/// * ric_sp_size: 15
	/// * ric_slic_type: 100
	/// * use_post_proc: true
	/// * fgs_lambda: 500.0f
	/// * fgs_sigma: 1.5f
	/// * use_variational_refinement: false
	// create(Ptr<RLOFOpticalFlowParameter>, float, Size, InterpolationType, int, float, float, int, int, bool, float, float, bool)(CppPassByVoidPtr, Primitive, SimpleClass, Enum, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:383
	// ("cv::optflow::DenseRLOFOpticalFlow::create", vec![(pred!(mut, ["rlofParam", "forwardBackwardThreshold", "gridStep", "interp_type", "epicK", "epicSigma", "epicLambda", "ricSPSize", "ricSLICType", "use_post_proc", "fgsLambda", "fgsSigma", "use_variational_refinement"], ["cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>", "float", "cv::Size", "cv::optflow::InterpolationType", "int", "float", "float", "int", "int", "bool", "float", "float", "bool"]), _)]),
	#[inline]
	pub fn create(mut rlof_param: core::Ptr<crate::optflow::RLOFOpticalFlowParameter>, forward_backward_threshold: f32, grid_step: core::Size, interp_type: crate::optflow::InterpolationType, epic_k: i32, epic_sigma: f32, epic_lambda: f32, ric_sp_size: i32, ric_slic_type: i32, use_post_proc: bool, fgs_lambda: f32, fgs_sigma: f32, use_variational_refinement: bool) -> Result<core::Ptr<crate::optflow::DenseRLOFOpticalFlow>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_create_PtrLRLOFOpticalFlowParameterG_float_Size_InterpolationType_int_float_float_int_int_bool_float_float_bool(rlof_param.as_raw_mut_PtrOfRLOFOpticalFlowParameter(), forward_backward_threshold, &grid_step, interp_type, epic_k, epic_sigma, epic_lambda, ric_sp_size, ric_slic_type, use_post_proc, fgs_lambda, fgs_sigma, use_variational_refinement, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::optflow::DenseRLOFOpticalFlow>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates instance of optflow::DenseRLOFOpticalFlow
	///
	/// ## Parameters
	/// * rlofParam: see optflow::RLOFOpticalFlowParameter
	/// * forwardBackwardThreshold: see setForwardBackward
	/// * gridStep: see setGridStep
	/// * interp_type: see setInterpolation
	/// * epicK: see setEPICK
	/// * epicSigma: see setEPICSigma
	/// * epicLambda: see setEPICLambda
	/// * ricSPSize: see setRICSPSize
	/// * ricSLICType: see setRICSLICType
	/// * use_post_proc: see setUsePostProc
	/// * fgsLambda: see setFgsLambda
	/// * fgsSigma: see setFgsSigma
	/// * use_variational_refinement: see setUseVariationalRefinement
	///
	/// ## Note
	/// This alternative version of [DenseRLOFOpticalFlow::create] function uses the following default values for its arguments:
	/// * rlof_param: Ptr<RLOFOpticalFlowParameter>()
	/// * forward_backward_threshold: 1.f
	/// * grid_step: Size(6,6)
	/// * interp_type: InterpolationType::INTERP_EPIC
	/// * epic_k: 128
	/// * epic_sigma: 0.05f
	/// * epic_lambda: 999.0f
	/// * ric_sp_size: 15
	/// * ric_slic_type: 100
	/// * use_post_proc: true
	/// * fgs_lambda: 500.0f
	/// * fgs_sigma: 1.5f
	/// * use_variational_refinement: false
	// cv::optflow::DenseRLOFOpticalFlow::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:383
	// ("cv::optflow::DenseRLOFOpticalFlow::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::optflow::DenseRLOFOpticalFlow>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::optflow::DenseRLOFOpticalFlow>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { DenseRLOFOpticalFlow, core::Algorithm, cv_optflow_DenseRLOFOpticalFlow_to_Algorithm }

boxed_cast_base! { DenseRLOFOpticalFlow, crate::video::DenseOpticalFlow, cv_optflow_DenseRLOFOpticalFlow_to_DenseOpticalFlow }

impl std::fmt::Debug for DenseRLOFOpticalFlow {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("DenseRLOFOpticalFlow")
			.finish()
	}
}

/// Constant methods for [crate::optflow::DualTVL1OpticalFlow]
// DualTVL1OpticalFlow /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:218
pub trait DualTVL1OpticalFlowTraitConst: crate::video::DenseOpticalFlowTraitConst {
	fn as_raw_DualTVL1OpticalFlow(&self) -> *const c_void;

	/// Time step of the numerical scheme
	/// ## See also
	/// setTau
	// getTau()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:223
	// ("cv::optflow::DualTVL1OpticalFlow::getTau", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_tau(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getTau_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weight parameter for the data term, attachment parameter
	/// ## See also
	/// setLambda
	// getLambda()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:228
	// ("cv::optflow::DualTVL1OpticalFlow::getLambda", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_lambda(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getLambda_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weight parameter for (u - v)^2, tightness parameter
	/// ## See also
	/// setTheta
	// getTheta()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:233
	// ("cv::optflow::DualTVL1OpticalFlow::getTheta", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_theta(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getTheta_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// coefficient for additional illumination variation term
	/// ## See also
	/// setGamma
	// getGamma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:238
	// ("cv::optflow::DualTVL1OpticalFlow::getGamma", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_gamma(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getGamma_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Number of scales used to create the pyramid of images
	/// ## See also
	/// setScalesNumber
	// getScalesNumber()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:243
	// ("cv::optflow::DualTVL1OpticalFlow::getScalesNumber", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_scales_number(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getScalesNumber_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Number of warpings per scale
	/// ## See also
	/// setWarpingsNumber
	// getWarpingsNumber()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:248
	// ("cv::optflow::DualTVL1OpticalFlow::getWarpingsNumber", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_warpings_number(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getWarpingsNumber_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Stopping criterion threshold used in the numerical scheme, which is a trade-off between precision and running time
	/// ## See also
	/// setEpsilon
	// getEpsilon()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:253
	// ("cv::optflow::DualTVL1OpticalFlow::getEpsilon", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_epsilon(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getEpsilon_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Inner iterations (between outlier filtering) used in the numerical scheme
	/// ## See also
	/// setInnerIterations
	// getInnerIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:258
	// ("cv::optflow::DualTVL1OpticalFlow::getInnerIterations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_inner_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getInnerIterations_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Outer iterations (number of inner loops) used in the numerical scheme
	/// ## See also
	/// setOuterIterations
	// getOuterIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:263
	// ("cv::optflow::DualTVL1OpticalFlow::getOuterIterations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_outer_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getOuterIterations_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Use initial flow
	/// ## See also
	/// setUseInitialFlow
	// getUseInitialFlow()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:268
	// ("cv::optflow::DualTVL1OpticalFlow::getUseInitialFlow", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_use_initial_flow(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getUseInitialFlow_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Step between scales (<1)
	/// ## See also
	/// setScaleStep
	// getScaleStep()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:273
	// ("cv::optflow::DualTVL1OpticalFlow::getScaleStep", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_scale_step(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getScaleStep_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Median filter kernel size (1 = no filter) (3 or 5)
	/// ## See also
	/// setMedianFiltering
	// getMedianFiltering()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:278
	// ("cv::optflow::DualTVL1OpticalFlow::getMedianFiltering", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_median_filtering(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getMedianFiltering_const(self.as_raw_DualTVL1OpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::optflow::DualTVL1OpticalFlow]
pub trait DualTVL1OpticalFlowTrait: crate::optflow::DualTVL1OpticalFlowTraitConst + crate::video::DenseOpticalFlowTrait {
	fn as_raw_mut_DualTVL1OpticalFlow(&mut self) -> *mut c_void;

	/// Time step of the numerical scheme
	/// ## See also
	/// setTau getTau
	// setTau(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:225
	// ("cv::optflow::DualTVL1OpticalFlow::setTau", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_tau(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setTau_double(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weight parameter for the data term, attachment parameter
	/// ## See also
	/// setLambda getLambda
	// setLambda(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:230
	// ("cv::optflow::DualTVL1OpticalFlow::setLambda", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_lambda(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setLambda_double(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weight parameter for (u - v)^2, tightness parameter
	/// ## See also
	/// setTheta getTheta
	// setTheta(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:235
	// ("cv::optflow::DualTVL1OpticalFlow::setTheta", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_theta(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setTheta_double(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// coefficient for additional illumination variation term
	/// ## See also
	/// setGamma getGamma
	// setGamma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:240
	// ("cv::optflow::DualTVL1OpticalFlow::setGamma", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_gamma(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setGamma_double(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Number of scales used to create the pyramid of images
	/// ## See also
	/// setScalesNumber getScalesNumber
	// setScalesNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:245
	// ("cv::optflow::DualTVL1OpticalFlow::setScalesNumber", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_scales_number(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setScalesNumber_int(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Number of warpings per scale
	/// ## See also
	/// setWarpingsNumber getWarpingsNumber
	// setWarpingsNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:250
	// ("cv::optflow::DualTVL1OpticalFlow::setWarpingsNumber", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_warpings_number(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setWarpingsNumber_int(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Stopping criterion threshold used in the numerical scheme, which is a trade-off between precision and running time
	/// ## See also
	/// setEpsilon getEpsilon
	// setEpsilon(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:255
	// ("cv::optflow::DualTVL1OpticalFlow::setEpsilon", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_epsilon(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setEpsilon_double(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Inner iterations (between outlier filtering) used in the numerical scheme
	/// ## See also
	/// setInnerIterations getInnerIterations
	// setInnerIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:260
	// ("cv::optflow::DualTVL1OpticalFlow::setInnerIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_inner_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setInnerIterations_int(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Outer iterations (number of inner loops) used in the numerical scheme
	/// ## See also
	/// setOuterIterations getOuterIterations
	// setOuterIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:265
	// ("cv::optflow::DualTVL1OpticalFlow::setOuterIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_outer_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setOuterIterations_int(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Use initial flow
	/// ## See also
	/// setUseInitialFlow getUseInitialFlow
	// setUseInitialFlow(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:270
	// ("cv::optflow::DualTVL1OpticalFlow::setUseInitialFlow", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	#[inline]
	fn set_use_initial_flow(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setUseInitialFlow_bool(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Step between scales (<1)
	/// ## See also
	/// setScaleStep getScaleStep
	// setScaleStep(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:275
	// ("cv::optflow::DualTVL1OpticalFlow::setScaleStep", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_scale_step(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setScaleStep_double(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Median filter kernel size (1 = no filter) (3 or 5)
	/// ## See also
	/// setMedianFiltering getMedianFiltering
	// setMedianFiltering(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:280
	// ("cv::optflow::DualTVL1OpticalFlow::setMedianFiltering", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_median_filtering(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setMedianFiltering_int(self.as_raw_mut_DualTVL1OpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// "Dual TV L1" Optical Flow Algorithm.
///
/// The class implements the "Dual TV L1" optical flow algorithm described in [Zach2007](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Zach2007) and
/// [Javier2012](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Javier2012) .
/// Here are important members of the class that control the algorithm, which you can set after
/// constructing the class instance:
///
/// *   member double tau
///    Time step of the numerical scheme.
///
/// *   member double lambda
///    Weight parameter for the data term, attachment parameter. This is the most relevant
///    parameter, which determines the smoothness of the output. The smaller this parameter is,
///    the smoother the solutions we obtain. It depends on the range of motions of the images, so
///    its value should be adapted to each image sequence.
///
/// *   member double theta
///    Weight parameter for (u - v)\^2, tightness parameter. It serves as a link between the
///    attachment and the regularization terms. In theory, it should have a small value in order
///    to maintain both parts in correspondence. The method is stable for a large range of values
///    of this parameter.
///
/// *   member int nscales
///    Number of scales used to create the pyramid of images.
///
/// *   member int warps
///    Number of warpings per scale. Represents the number of times that I1(x+u0) and grad(
///    I1(x+u0) ) are computed per scale. This is a parameter that assures the stability of the
///    method. It also affects the running time, so it is a compromise between speed and
///    accuracy.
///
/// *   member double epsilon
///    Stopping criterion threshold used in the numerical scheme, which is a trade-off between
///    precision and running time. A small value will yield more accurate solutions at the
///    expense of a slower convergence.
///
/// *   member int iterations
///    Stopping criterion iterations number used in the numerical scheme.
///
/// C. Zach, T. Pock and H. Bischof, "A Duality Based Approach for Realtime TV-L1 Optical Flow".
/// Javier Sanchez, Enric Meinhardt-Llopis and Gabriele Facciolo. "TV-L1 Optical Flow Estimation".
// DualTVL1OpticalFlow /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:218
pub struct DualTVL1OpticalFlow {
	ptr: *mut c_void,
}

opencv_type_boxed! { DualTVL1OpticalFlow }

impl Drop for DualTVL1OpticalFlow {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_delete(self.as_raw_mut_DualTVL1OpticalFlow()) };
	}
}

unsafe impl Send for DualTVL1OpticalFlow {}

impl core::AlgorithmTraitConst for DualTVL1OpticalFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for DualTVL1OpticalFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DualTVL1OpticalFlow, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::video::DenseOpticalFlowTraitConst for DualTVL1OpticalFlow {
	#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::DenseOpticalFlowTrait for DualTVL1OpticalFlow {
	#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DualTVL1OpticalFlow, crate::video::DenseOpticalFlowTraitConst, as_raw_DenseOpticalFlow, crate::video::DenseOpticalFlowTrait, as_raw_mut_DenseOpticalFlow }

impl crate::optflow::DualTVL1OpticalFlowTraitConst for DualTVL1OpticalFlow {
	#[inline] fn as_raw_DualTVL1OpticalFlow(&self) -> *const c_void { self.as_raw() }
}

impl crate::optflow::DualTVL1OpticalFlowTrait for DualTVL1OpticalFlow {
	#[inline] fn as_raw_mut_DualTVL1OpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DualTVL1OpticalFlow, crate::optflow::DualTVL1OpticalFlowTraitConst, as_raw_DualTVL1OpticalFlow, crate::optflow::DualTVL1OpticalFlowTrait, as_raw_mut_DualTVL1OpticalFlow }

impl DualTVL1OpticalFlow {
	/// Creates instance of cv::DualTVL1OpticalFlow
	///
	/// ## C++ default parameters
	/// * tau: 0.25
	/// * lambda: 0.15
	/// * theta: 0.3
	/// * nscales: 5
	/// * warps: 5
	/// * epsilon: 0.01
	/// * innner_iterations: 30
	/// * outer_iterations: 10
	/// * scale_step: 0.8
	/// * gamma: 0.0
	/// * median_filtering: 5
	/// * use_initial_flow: false
	// create(double, double, double, int, int, double, int, int, double, double, int, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:283
	// ("cv::optflow::DualTVL1OpticalFlow::create", vec![(pred!(mut, ["tau", "lambda", "theta", "nscales", "warps", "epsilon", "innnerIterations", "outerIterations", "scaleStep", "gamma", "medianFiltering", "useInitialFlow"], ["double", "double", "double", "int", "int", "double", "int", "int", "double", "double", "int", "bool"]), _)]),
	#[inline]
	pub fn create(tau: f64, lambda: f64, theta: f64, nscales: i32, warps: i32, epsilon: f64, innner_iterations: i32, outer_iterations: i32, scale_step: f64, gamma: f64, median_filtering: i32, use_initial_flow: bool) -> Result<core::Ptr<crate::optflow::DualTVL1OpticalFlow>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_create_double_double_double_int_int_double_int_int_double_double_int_bool(tau, lambda, theta, nscales, warps, epsilon, innner_iterations, outer_iterations, scale_step, gamma, median_filtering, use_initial_flow, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::optflow::DualTVL1OpticalFlow>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates instance of cv::DualTVL1OpticalFlow
	///
	/// ## Note
	/// This alternative version of [DualTVL1OpticalFlow::create] function uses the following default values for its arguments:
	/// * tau: 0.25
	/// * lambda: 0.15
	/// * theta: 0.3
	/// * nscales: 5
	/// * warps: 5
	/// * epsilon: 0.01
	/// * innner_iterations: 30
	/// * outer_iterations: 10
	/// * scale_step: 0.8
	/// * gamma: 0.0
	/// * median_filtering: 5
	/// * use_initial_flow: false
	// cv::optflow::DualTVL1OpticalFlow::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow.hpp:283
	// ("cv::optflow::DualTVL1OpticalFlow::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::optflow::DualTVL1OpticalFlow>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::optflow::DualTVL1OpticalFlow>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { DualTVL1OpticalFlow, core::Algorithm, cv_optflow_DualTVL1OpticalFlow_to_Algorithm }

boxed_cast_base! { DualTVL1OpticalFlow, crate::video::DenseOpticalFlow, cv_optflow_DualTVL1OpticalFlow_to_DenseOpticalFlow }

impl std::fmt::Debug for DualTVL1OpticalFlow {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("DualTVL1OpticalFlow")
			.finish()
	}
}

/// Constant methods for [crate::optflow::GPCDetails]
// GPCDetails /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:301
pub trait GPCDetailsTraitConst {
	fn as_raw_GPCDetails(&self) -> *const c_void;

}

/// Mutable methods for [crate::optflow::GPCDetails]
pub trait GPCDetailsTrait: crate::optflow::GPCDetailsTraitConst {
	fn as_raw_mut_GPCDetails(&mut self) -> *mut c_void;

}

// GPCDetails /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:301
pub struct GPCDetails {
	ptr: *mut c_void,
}

opencv_type_boxed! { GPCDetails }

impl Drop for GPCDetails {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_optflow_GPCDetails_delete(self.as_raw_mut_GPCDetails()) };
	}
}

unsafe impl Send for GPCDetails {}

impl crate::optflow::GPCDetailsTraitConst for GPCDetails {
	#[inline] fn as_raw_GPCDetails(&self) -> *const c_void { self.as_raw() }
}

impl crate::optflow::GPCDetailsTrait for GPCDetails {
	#[inline] fn as_raw_mut_GPCDetails(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { GPCDetails, crate::optflow::GPCDetailsTraitConst, as_raw_GPCDetails, crate::optflow::GPCDetailsTrait, as_raw_mut_GPCDetails }

impl GPCDetails {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_optflow_GPCDetails_defaultNew_const()) }
	}

	// dropOutliers(std::vector<std::pair<Point2i, Point2i>> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:304
	// ("cv::optflow::GPCDetails::dropOutliers", vec![(pred!(mut, ["corr"], ["std::vector<std::pair<cv::Point2i, cv::Point2i>>*"]), _)]),
	#[inline]
	pub fn drop_outliers(corr: &mut core::Vector<core::Tuple<(core::Point2i, core::Point2i)>>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCDetails_dropOutliers_vectorLpairLcv_Point2i__cv_Point2iGGR(corr.as_raw_mut_VectorOfTupleOfPoint2i_Point2i(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getAllDescriptorsForImage(const Mat *, std::vector<GPCPatchDescriptor> &, const GPCMatchingParams &, int)(TraitClass, CppPassByVoidPtr, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:306
	// ("cv::optflow::GPCDetails::getAllDescriptorsForImage", vec![(pred!(mut, ["imgCh", "descr", "mp", "type"], ["const cv::Mat*", "std::vector<cv::optflow::GPCPatchDescriptor>*", "const cv::optflow::GPCMatchingParams*", "int"]), _)]),
	#[inline]
	pub fn get_all_descriptors_for_image(img_ch: &impl core::MatTraitConst, descr: &mut core::Vector<crate::optflow::GPCPatchDescriptor>, mp: crate::optflow::GPCMatchingParams, typ: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCDetails_getAllDescriptorsForImage_const_MatX_vectorLGPCPatchDescriptorGR_const_GPCMatchingParamsR_int(img_ch.as_raw_Mat(), descr.as_raw_mut_VectorOfGPCPatchDescriptor(), &mp, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getCoordinatesFromIndex(size_t, Size, int &, int &)(Primitive, SimpleClass, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:309
	// ("cv::optflow::GPCDetails::getCoordinatesFromIndex", vec![(pred!(mut, ["index", "sz", "x", "y"], ["size_t", "cv::Size", "int*", "int*"]), _)]),
	#[inline]
	pub fn get_coordinates_from_index(index: size_t, sz: core::Size, x: &mut i32, y: &mut i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCDetails_getCoordinatesFromIndex_size_t_Size_intR_intR(index, &sz, x, y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

impl std::fmt::Debug for GPCDetails {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("GPCDetails")
			.finish()
	}
}

impl Default for GPCDetails {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Class encapsulating matching parameters.
// GPCMatchingParams /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:143
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GPCMatchingParams {
	/// Whether to use OpenCL to speed up the matching.
	pub use_opencl: bool,
}

opencv_type_simple! { crate::optflow::GPCMatchingParams }

impl GPCMatchingParams {
	/// ## C++ default parameters
	/// * _use_opencl: false
	// GPCMatchingParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:147
	// ("cv::optflow::GPCMatchingParams::GPCMatchingParams", vec![(pred!(mut, ["_useOpenCL"], ["bool"]), _)]),
	#[inline]
	pub fn new(_use_opencl: bool) -> Result<crate::optflow::GPCMatchingParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCMatchingParams_GPCMatchingParams_bool(_use_opencl, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * _use_opencl: false
	// cv::optflow::GPCMatchingParams::GPCMatchingParams() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:147
	// ("cv::optflow::GPCMatchingParams::GPCMatchingParams", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::optflow::GPCMatchingParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCMatchingParams_GPCMatchingParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// GPCMatchingParams(const GPCMatchingParams &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:149
	// ("cv::optflow::GPCMatchingParams::GPCMatchingParams", vec![(pred!(mut, ["params"], ["const cv::optflow::GPCMatchingParams*"]), _)]),
	#[inline]
	pub fn copy(params: crate::optflow::GPCMatchingParams) -> Result<crate::optflow::GPCMatchingParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCMatchingParams_GPCMatchingParams_const_GPCMatchingParamsR(&params, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Constant methods for [crate::optflow::GPCPatchDescriptor]
// GPCPatchDescriptor /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:65
pub trait GPCPatchDescriptorTraitConst {
	fn as_raw_GPCPatchDescriptor(&self) -> *const c_void;

	// cv::optflow::GPCPatchDescriptor::feature() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:68
	// ("cv::optflow::GPCPatchDescriptor::feature", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn feature(&self) -> core::VecN<f64, 18> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCPatchDescriptor_propFeature_const(self.as_raw_GPCPatchDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	// dot(const Vec<double, nFeatures> &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:70
	// ("cv::optflow::GPCPatchDescriptor::dot", vec![(pred!(const, ["coef"], ["const cv::Vec<double, 18>*"]), _)]),
	#[inline]
	fn dot(&self, coef: core::VecN<f64, 18>) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCPatchDescriptor_dot_const_const_VecLdouble__18GR(self.as_raw_GPCPatchDescriptor(), &coef, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// isSeparated()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:74
	// ("cv::optflow::GPCPatchDescriptor::isSeparated", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn is_separated(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCPatchDescriptor_isSeparated_const(self.as_raw_GPCPatchDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::optflow::GPCPatchDescriptor]
pub trait GPCPatchDescriptorTrait: crate::optflow::GPCPatchDescriptorTraitConst {
	fn as_raw_mut_GPCPatchDescriptor(&mut self) -> *mut c_void;

	// cv::optflow::GPCPatchDescriptor::setFeature(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:68
	// ("cv::optflow::GPCPatchDescriptor::setFeature", vec![(pred!(mut, ["val"], ["const cv::Vec<double, 18>"]), _)]),
	#[inline]
	fn set_feature(&mut self, val: core::VecN<f64, 18>) {
		let ret = unsafe { sys::cv_optflow_GPCPatchDescriptor_propFeature_const_VecLdouble__18G(self.as_raw_mut_GPCPatchDescriptor(), &val) };
		ret
	}

	// markAsSeparated()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:72
	// ("cv::optflow::GPCPatchDescriptor::markAsSeparated", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn mark_as_separated(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCPatchDescriptor_markAsSeparated(self.as_raw_mut_GPCPatchDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// GPCPatchDescriptor /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:65
pub struct GPCPatchDescriptor {
	ptr: *mut c_void,
}

opencv_type_boxed! { GPCPatchDescriptor }

impl Drop for GPCPatchDescriptor {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_optflow_GPCPatchDescriptor_delete(self.as_raw_mut_GPCPatchDescriptor()) };
	}
}

unsafe impl Send for GPCPatchDescriptor {}

impl crate::optflow::GPCPatchDescriptorTraitConst for GPCPatchDescriptor {
	#[inline] fn as_raw_GPCPatchDescriptor(&self) -> *const c_void { self.as_raw() }
}

impl crate::optflow::GPCPatchDescriptorTrait for GPCPatchDescriptor {
	#[inline] fn as_raw_mut_GPCPatchDescriptor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { GPCPatchDescriptor, crate::optflow::GPCPatchDescriptorTraitConst, as_raw_GPCPatchDescriptor, crate::optflow::GPCPatchDescriptorTrait, as_raw_mut_GPCPatchDescriptor }

impl GPCPatchDescriptor {
	/// number of features in a patch descriptor
	// nFeatures /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:67
	pub const nFeatures: u32 = 18;
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_optflow_GPCPatchDescriptor_defaultNew_const()) }
	}

}

impl std::fmt::Debug for GPCPatchDescriptor {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("GPCPatchDescriptor")
			.field("feature", &crate::optflow::GPCPatchDescriptorTraitConst::feature(self))
			.finish()
	}
}

impl Default for GPCPatchDescriptor {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::optflow::GPCPatchSample]
// GPCPatchSample /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:77
pub trait GPCPatchSampleTraitConst {
	fn as_raw_GPCPatchSample(&self) -> *const c_void;

	// cv::optflow::GPCPatchSample::ref() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:79
	// ("cv::optflow::GPCPatchSample::ref", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn ref_(&self) -> crate::optflow::GPCPatchDescriptor {
		let ret = unsafe { sys::cv_optflow_GPCPatchSample_propRef_const(self.as_raw_GPCPatchSample()) };
		let ret = unsafe { crate::optflow::GPCPatchDescriptor::opencv_from_extern(ret) };
		ret
	}

	// cv::optflow::GPCPatchSample::pos() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:80
	// ("cv::optflow::GPCPatchSample::pos", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pos(&self) -> crate::optflow::GPCPatchDescriptor {
		let ret = unsafe { sys::cv_optflow_GPCPatchSample_propPos_const(self.as_raw_GPCPatchSample()) };
		let ret = unsafe { crate::optflow::GPCPatchDescriptor::opencv_from_extern(ret) };
		ret
	}

	// cv::optflow::GPCPatchSample::neg() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:81
	// ("cv::optflow::GPCPatchSample::neg", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn neg(&self) -> crate::optflow::GPCPatchDescriptor {
		let ret = unsafe { sys::cv_optflow_GPCPatchSample_propNeg_const(self.as_raw_GPCPatchSample()) };
		let ret = unsafe { crate::optflow::GPCPatchDescriptor::opencv_from_extern(ret) };
		ret
	}

	// getDirections(bool &, bool &, bool &, const Vec<double, GPCPatchDescriptor::nFeatures> &, double)(Indirect, Indirect, Indirect, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:83
	// ("cv::optflow::GPCPatchSample::getDirections", vec![(pred!(const, ["refdir", "posdir", "negdir", "coef", "rhs"], ["bool*", "bool*", "bool*", "const cv::Vec<double, 18>*", "double"]), _)]),
	#[inline]
	fn get_directions(&self, refdir: &mut bool, posdir: &mut bool, negdir: &mut bool, coef: core::VecN<f64, 18>, rhs: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCPatchSample_getDirections_const_boolR_boolR_boolR_const_VecLdouble__18GR_double(self.as_raw_GPCPatchSample(), refdir, posdir, negdir, &coef, rhs, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::optflow::GPCPatchSample]
pub trait GPCPatchSampleTrait: crate::optflow::GPCPatchSampleTraitConst {
	fn as_raw_mut_GPCPatchSample(&mut self) -> *mut c_void;

	// cv::optflow::GPCPatchSample::setRef(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:79
	// ("cv::optflow::GPCPatchSample::setRef", vec![(pred!(mut, ["val"], ["const cv::optflow::GPCPatchDescriptor"]), _)]),
	#[inline]
	fn set_ref(&mut self, val: crate::optflow::GPCPatchDescriptor) {
		let ret = unsafe { sys::cv_optflow_GPCPatchSample_propRef_const_GPCPatchDescriptor(self.as_raw_mut_GPCPatchSample(), val.as_raw_GPCPatchDescriptor()) };
		ret
	}

	// cv::optflow::GPCPatchSample::setPos(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:80
	// ("cv::optflow::GPCPatchSample::setPos", vec![(pred!(mut, ["val"], ["const cv::optflow::GPCPatchDescriptor"]), _)]),
	#[inline]
	fn set_pos(&mut self, val: crate::optflow::GPCPatchDescriptor) {
		let ret = unsafe { sys::cv_optflow_GPCPatchSample_propPos_const_GPCPatchDescriptor(self.as_raw_mut_GPCPatchSample(), val.as_raw_GPCPatchDescriptor()) };
		ret
	}

	// cv::optflow::GPCPatchSample::setNeg(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:81
	// ("cv::optflow::GPCPatchSample::setNeg", vec![(pred!(mut, ["val"], ["const cv::optflow::GPCPatchDescriptor"]), _)]),
	#[inline]
	fn set_neg(&mut self, val: crate::optflow::GPCPatchDescriptor) {
		let ret = unsafe { sys::cv_optflow_GPCPatchSample_propNeg_const_GPCPatchDescriptor(self.as_raw_mut_GPCPatchSample(), val.as_raw_GPCPatchDescriptor()) };
		ret
	}

}

// GPCPatchSample /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:77
pub struct GPCPatchSample {
	ptr: *mut c_void,
}

opencv_type_boxed! { GPCPatchSample }

impl Drop for GPCPatchSample {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_optflow_GPCPatchSample_delete(self.as_raw_mut_GPCPatchSample()) };
	}
}

unsafe impl Send for GPCPatchSample {}

impl crate::optflow::GPCPatchSampleTraitConst for GPCPatchSample {
	#[inline] fn as_raw_GPCPatchSample(&self) -> *const c_void { self.as_raw() }
}

impl crate::optflow::GPCPatchSampleTrait for GPCPatchSample {
	#[inline] fn as_raw_mut_GPCPatchSample(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { GPCPatchSample, crate::optflow::GPCPatchSampleTraitConst, as_raw_GPCPatchSample, crate::optflow::GPCPatchSampleTrait, as_raw_mut_GPCPatchSample }

impl GPCPatchSample {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_optflow_GPCPatchSample_defaultNew_const()) }
	}

}

impl std::fmt::Debug for GPCPatchSample {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("GPCPatchSample")
			.field("ref_", &crate::optflow::GPCPatchSampleTraitConst::ref_(self))
			.field("pos", &crate::optflow::GPCPatchSampleTraitConst::pos(self))
			.field("neg", &crate::optflow::GPCPatchSampleTraitConst::neg(self))
			.finish()
	}
}

impl Default for GPCPatchSample {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Class encapsulating training parameters.
// GPCTrainingParams /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:123
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GPCTrainingParams {
	/// Maximum tree depth to stop partitioning.
	pub max_tree_depth: u32,
	/// Minimum number of samples in the node to stop partitioning.
	pub min_number_of_samples: i32,
	/// Type of descriptors to use.
	pub descriptor_type: i32,
	/// Print progress to stdout.
	pub print_progress: bool,
}

opencv_type_simple! { crate::optflow::GPCTrainingParams }

impl GPCTrainingParams {
	// check()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:138
	// ("cv::optflow::GPCTrainingParams::check", vec![(pred!(const, [], []), _)]),
	#[inline]
	pub fn check(self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTrainingParams_check_const(&self, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * _max_tree_depth: 20
	/// * _min_number_of_samples: 3
	/// * _descriptor_type: GPC_DESCRIPTOR_DCT
	/// * _print_progress: true
	// GPCTrainingParams(unsigned int, int, GPCDescType, bool)(Primitive, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:130
	// ("cv::optflow::GPCTrainingParams::GPCTrainingParams", vec![(pred!(mut, ["_maxTreeDepth", "_minNumberOfSamples", "_descriptorType", "_printProgress"], ["unsigned int", "int", "cv::optflow::GPCDescType", "bool"]), _)]),
	#[inline]
	pub fn new(_max_tree_depth: u32, _min_number_of_samples: i32, _descriptor_type: crate::optflow::GPCDescType, _print_progress: bool) -> Result<crate::optflow::GPCTrainingParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTrainingParams_GPCTrainingParams_unsigned_int_int_GPCDescType_bool(_max_tree_depth, _min_number_of_samples, _descriptor_type, _print_progress, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * _max_tree_depth: 20
	/// * _min_number_of_samples: 3
	/// * _descriptor_type: GPC_DESCRIPTOR_DCT
	/// * _print_progress: true
	// cv::optflow::GPCTrainingParams::GPCTrainingParams() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:130
	// ("cv::optflow::GPCTrainingParams::GPCTrainingParams", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::optflow::GPCTrainingParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTrainingParams_GPCTrainingParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Constant methods for [crate::optflow::GPCTrainingSamples]
// GPCTrainingSamples /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:98
pub trait GPCTrainingSamplesTraitConst {
	fn as_raw_GPCTrainingSamples(&self) -> *const c_void;

	// size()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:114
	// ("cv::optflow::GPCTrainingSamples::size", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn size(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTrainingSamples_size_const(self.as_raw_GPCTrainingSamples(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// type()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:116
	// ("cv::optflow::GPCTrainingSamples::type", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn typ(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTrainingSamples_type_const(self.as_raw_GPCTrainingSamples(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::optflow::GPCTrainingSamples]
pub trait GPCTrainingSamplesTrait: crate::optflow::GPCTrainingSamplesTraitConst {
	fn as_raw_mut_GPCTrainingSamples(&mut self) -> *mut c_void;

}

/// Class encapsulating training samples.
// GPCTrainingSamples /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:98
pub struct GPCTrainingSamples {
	ptr: *mut c_void,
}

opencv_type_boxed! { GPCTrainingSamples }

impl Drop for GPCTrainingSamples {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_optflow_GPCTrainingSamples_delete(self.as_raw_mut_GPCTrainingSamples()) };
	}
}

unsafe impl Send for GPCTrainingSamples {}

impl crate::optflow::GPCTrainingSamplesTraitConst for GPCTrainingSamples {
	#[inline] fn as_raw_GPCTrainingSamples(&self) -> *const c_void { self.as_raw() }
}

impl crate::optflow::GPCTrainingSamplesTrait for GPCTrainingSamples {
	#[inline] fn as_raw_mut_GPCTrainingSamples(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { GPCTrainingSamples, crate::optflow::GPCTrainingSamplesTraitConst, as_raw_GPCTrainingSamples, crate::optflow::GPCTrainingSamplesTrait, as_raw_mut_GPCTrainingSamples }

impl GPCTrainingSamples {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_optflow_GPCTrainingSamples_defaultNew_const()) }
	}

	/// This function can be used to extract samples from a pair of images and a ground truth flow.
	/// Sizes of all the provided vectors must be equal.
	// create(const std::vector<String> &, const std::vector<String> &, const std::vector<String> &, int)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:108
	// ("cv::optflow::GPCTrainingSamples::create", vec![(pred!(mut, ["imagesFrom", "imagesTo", "gt", "descriptorType"], ["const std::vector<cv::String>*", "const std::vector<cv::String>*", "const std::vector<cv::String>*", "int"]), _)]),
	#[inline]
	pub fn create(images_from: &core::Vector<String>, images_to: &core::Vector<String>, gt: &core::Vector<String>, descriptor_type: i32) -> Result<core::Ptr<crate::optflow::GPCTrainingSamples>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTrainingSamples_create_const_vectorLStringGR_const_vectorLStringGR_const_vectorLStringGR_int(images_from.as_raw_VectorOfString(), images_to.as_raw_VectorOfString(), gt.as_raw_VectorOfString(), descriptor_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::optflow::GPCTrainingSamples>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// create(InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, int)(InputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:111
	// ("cv::optflow::GPCTrainingSamples::create", vec![(pred!(mut, ["imagesFrom", "imagesTo", "gt", "descriptorType"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
	#[inline]
	pub fn create_1(images_from: &impl ToInputArray, images_to: &impl ToInputArray, gt: &impl ToInputArray, descriptor_type: i32) -> Result<core::Ptr<crate::optflow::GPCTrainingSamples>> {
		input_array_arg!(images_from);
		input_array_arg!(images_to);
		input_array_arg!(gt);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTrainingSamples_create_const__InputArrayR_const__InputArrayR_const__InputArrayR_int(images_from.as_raw__InputArray(), images_to.as_raw__InputArray(), gt.as_raw__InputArray(), descriptor_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::optflow::GPCTrainingSamples>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for GPCTrainingSamples {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("GPCTrainingSamples")
			.finish()
	}
}

impl Default for GPCTrainingSamples {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::optflow::GPCTree]
// GPCTree /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:154
pub trait GPCTreeTraitConst: core::AlgorithmTraitConst {
	fn as_raw_GPCTree(&self) -> *const c_void;

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:178
	// ("cv::optflow::GPCTree::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	#[inline]
	fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTree_write_const_FileStorageR(self.as_raw_GPCTree(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// findLeafForPatch(const GPCPatchDescriptor &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:182
	// ("cv::optflow::GPCTree::findLeafForPatch", vec![(pred!(const, ["descr"], ["const cv::optflow::GPCPatchDescriptor*"]), _)]),
	#[inline]
	fn find_leaf_for_patch(&self, descr: &impl crate::optflow::GPCPatchDescriptorTraitConst) -> Result<u32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTree_findLeafForPatch_const_const_GPCPatchDescriptorR(self.as_raw_GPCTree(), descr.as_raw_GPCPatchDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// operator==(const GPCTree &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:186
	// ("cv::optflow::GPCTree::operator==", vec![(pred!(const, ["t"], ["const cv::optflow::GPCTree*"]), _)]),
	#[inline]
	fn equals(&self, t: &impl crate::optflow::GPCTreeTraitConst) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTree_operatorEQ_const_const_GPCTreeR(self.as_raw_GPCTree(), t.as_raw_GPCTree(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDescriptorType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:188
	// ("cv::optflow::GPCTree::getDescriptorType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_descriptor_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTree_getDescriptorType_const(self.as_raw_GPCTree(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::optflow::GPCTree]
pub trait GPCTreeTrait: core::AlgorithmTrait + crate::optflow::GPCTreeTraitConst {
	fn as_raw_mut_GPCTree(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * params: GPCTrainingParams()
	// train(GPCTrainingSamples &, const GPCTrainingParams)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:176
	// ("cv::optflow::GPCTree::train", vec![(pred!(mut, ["samples", "params"], ["cv::optflow::GPCTrainingSamples*", "const cv::optflow::GPCTrainingParams"]), _)]),
	#[inline]
	fn train(&mut self, samples: &mut impl crate::optflow::GPCTrainingSamplesTrait, params: crate::optflow::GPCTrainingParams) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTree_train_GPCTrainingSamplesR_const_GPCTrainingParams(self.as_raw_mut_GPCTree(), samples.as_raw_mut_GPCTrainingSamples(), &params, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [GPCTreeTrait::train] function uses the following default values for its arguments:
	/// * params: GPCTrainingParams()
	// cv::optflow::GPCTree::train(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:176
	// ("cv::optflow::GPCTree::train", vec![(pred!(mut, ["samples"], ["cv::optflow::GPCTrainingSamples*"]), _)]),
	#[inline]
	fn train_def(&mut self, samples: &mut impl crate::optflow::GPCTrainingSamplesTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTree_train_GPCTrainingSamplesR(self.as_raw_mut_GPCTree(), samples.as_raw_mut_GPCTrainingSamples(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:180
	// ("cv::optflow::GPCTree::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	#[inline]
	fn read(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTree_read_const_FileNodeR(self.as_raw_mut_GPCTree(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class for individual tree.
// GPCTree /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:154
pub struct GPCTree {
	ptr: *mut c_void,
}

opencv_type_boxed! { GPCTree }

impl Drop for GPCTree {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_optflow_GPCTree_delete(self.as_raw_mut_GPCTree()) };
	}
}

unsafe impl Send for GPCTree {}

impl core::AlgorithmTraitConst for GPCTree {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for GPCTree {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { GPCTree, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::optflow::GPCTreeTraitConst for GPCTree {
	#[inline] fn as_raw_GPCTree(&self) -> *const c_void { self.as_raw() }
}

impl crate::optflow::GPCTreeTrait for GPCTree {
	#[inline] fn as_raw_mut_GPCTree(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { GPCTree, crate::optflow::GPCTreeTraitConst, as_raw_GPCTree, crate::optflow::GPCTreeTrait, as_raw_mut_GPCTree }

impl GPCTree {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_optflow_GPCTree_defaultNew_const()) }
	}

	// create()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:184
	// ("cv::optflow::GPCTree::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::optflow::GPCTree>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTree_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::optflow::GPCTree>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { GPCTree, core::Algorithm, cv_optflow_GPCTree_to_Algorithm }

impl std::fmt::Debug for GPCTree {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("GPCTree")
			.finish()
	}
}

impl Default for GPCTree {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

// Node /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:157
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GPCTree_Node {
	/// Hyperplane coefficients
	pub coef: core::VecN<f64, 18>,
	/// Bias term of the hyperplane
	pub rhs: f64,
	pub left: u32,
	pub right: u32,
}

opencv_type_simple! { crate::optflow::GPCTree_Node }

impl GPCTree_Node {
	// operator==(const Node &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/sparse_matching_gpc.hpp:164
	// ("cv::optflow::GPCTree::Node::operator==", vec![(pred!(const, ["n"], ["const cv::optflow::GPCTree::Node*"]), _)]),
	#[inline]
	pub fn equals(self, n: crate::optflow::GPCTree_Node) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTree_Node_operatorEQ_const_const_NodeR(&self, &n, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Constant methods for [crate::optflow::OpticalFlowPCAFlow]
// OpticalFlowPCAFlow /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/pcaflow.hpp:94
pub trait OpticalFlowPCAFlowTraitConst: crate::video::DenseOpticalFlowTraitConst {
	fn as_raw_OpticalFlowPCAFlow(&self) -> *const c_void;

}

/// Mutable methods for [crate::optflow::OpticalFlowPCAFlow]
pub trait OpticalFlowPCAFlowTrait: crate::optflow::OpticalFlowPCAFlowTraitConst + crate::video::DenseOpticalFlowTrait {
	fn as_raw_mut_OpticalFlowPCAFlow(&mut self) -> *mut c_void;

	// calc(InputArray, InputArray, InputOutputArray)(InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/pcaflow.hpp:120
	// ("cv::optflow::OpticalFlowPCAFlow::calc", vec![(pred!(mut, ["I0", "I1", "flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
	#[inline]
	fn calc(&mut self, i0: &impl ToInputArray, i1: &impl ToInputArray, flow: &mut impl ToInputOutputArray) -> Result<()> {
		input_array_arg!(i0);
		input_array_arg!(i1);
		input_output_array_arg!(flow);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_OpticalFlowPCAFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(self.as_raw_mut_OpticalFlowPCAFlow(), i0.as_raw__InputArray(), i1.as_raw__InputArray(), flow.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// collectGarbage()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/pcaflow.hpp:121
	// ("cv::optflow::OpticalFlowPCAFlow::collectGarbage", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn collect_garbage(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_OpticalFlowPCAFlow_collectGarbage(self.as_raw_mut_OpticalFlowPCAFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// PCAFlow algorithm.
// OpticalFlowPCAFlow /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/pcaflow.hpp:94
pub struct OpticalFlowPCAFlow {
	ptr: *mut c_void,
}

opencv_type_boxed! { OpticalFlowPCAFlow }

impl Drop for OpticalFlowPCAFlow {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_optflow_OpticalFlowPCAFlow_delete(self.as_raw_mut_OpticalFlowPCAFlow()) };
	}
}

unsafe impl Send for OpticalFlowPCAFlow {}

impl core::AlgorithmTraitConst for OpticalFlowPCAFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for OpticalFlowPCAFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { OpticalFlowPCAFlow, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::video::DenseOpticalFlowTraitConst for OpticalFlowPCAFlow {
	#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::DenseOpticalFlowTrait for OpticalFlowPCAFlow {
	#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { OpticalFlowPCAFlow, crate::video::DenseOpticalFlowTraitConst, as_raw_DenseOpticalFlow, crate::video::DenseOpticalFlowTrait, as_raw_mut_DenseOpticalFlow }

impl crate::optflow::OpticalFlowPCAFlowTraitConst for OpticalFlowPCAFlow {
	#[inline] fn as_raw_OpticalFlowPCAFlow(&self) -> *const c_void { self.as_raw() }
}

impl crate::optflow::OpticalFlowPCAFlowTrait for OpticalFlowPCAFlow {
	#[inline] fn as_raw_mut_OpticalFlowPCAFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { OpticalFlowPCAFlow, crate::optflow::OpticalFlowPCAFlowTraitConst, as_raw_OpticalFlowPCAFlow, crate::optflow::OpticalFlowPCAFlowTrait, as_raw_mut_OpticalFlowPCAFlow }

impl OpticalFlowPCAFlow {
	/// Creates an instance of PCAFlow algorithm.
	/// ## Parameters
	/// * _prior: Learned prior or no prior (default). see also: cv::optflow::PCAPrior
	/// * _basisSize: Number of basis vectors.
	/// * _sparseRate: Controls density of sparse matches.
	/// * _retainedCornersFraction: Retained corners fraction.
	/// * _occlusionsThreshold: Occlusion threshold.
	/// * _dampingFactor: Regularization term for solving least-squares. It is not related to the prior regularization.
	/// * _claheClip: Clip parameter for CLAHE.
	///
	/// ## C++ default parameters
	/// * _prior: Ptr<constPCAPrior>()
	/// * _basis_size: Size(18,14)
	/// * _sparse_rate: 0.024
	/// * _retained_corners_fraction: 0.2
	/// * _occlusions_threshold: 0.0003
	/// * _damping_factor: 0.00002
	/// * _clahe_clip: 14
	// OpticalFlowPCAFlow(Ptr<const PCAPrior>, const Size, float, float, float, float, float)(CppPassByVoidPtr, SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/pcaflow.hpp:116
	// ("cv::optflow::OpticalFlowPCAFlow::OpticalFlowPCAFlow", vec![(pred!(mut, ["_prior", "_basisSize", "_sparseRate", "_retainedCornersFraction", "_occlusionsThreshold", "_dampingFactor", "_claheClip"], ["cv::Ptr<const cv::optflow::PCAPrior>", "const cv::Size", "float", "float", "float", "float", "float"]), _)]),
	#[inline]
	pub fn new(_prior: core::Ptr<crate::optflow::PCAPrior>, _basis_size: core::Size, _sparse_rate: f32, _retained_corners_fraction: f32, _occlusions_threshold: f32, _damping_factor: f32, _clahe_clip: f32) -> Result<crate::optflow::OpticalFlowPCAFlow> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_OpticalFlowPCAFlow_OpticalFlowPCAFlow_PtrLconst_PCAPriorG_const_Size_float_float_float_float_float(_prior.as_raw_PtrOfPCAPrior(), &_basis_size, _sparse_rate, _retained_corners_fraction, _occlusions_threshold, _damping_factor, _clahe_clip, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::optflow::OpticalFlowPCAFlow::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates an instance of PCAFlow algorithm.
	/// ## Parameters
	/// * _prior: Learned prior or no prior (default). see also: cv::optflow::PCAPrior
	/// * _basisSize: Number of basis vectors.
	/// * _sparseRate: Controls density of sparse matches.
	/// * _retainedCornersFraction: Retained corners fraction.
	/// * _occlusionsThreshold: Occlusion threshold.
	/// * _dampingFactor: Regularization term for solving least-squares. It is not related to the prior regularization.
	/// * _claheClip: Clip parameter for CLAHE.
	///
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * _prior: Ptr<constPCAPrior>()
	/// * _basis_size: Size(18,14)
	/// * _sparse_rate: 0.024
	/// * _retained_corners_fraction: 0.2
	/// * _occlusions_threshold: 0.0003
	/// * _damping_factor: 0.00002
	/// * _clahe_clip: 14
	// cv::optflow::OpticalFlowPCAFlow::OpticalFlowPCAFlow() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/pcaflow.hpp:116
	// ("cv::optflow::OpticalFlowPCAFlow::OpticalFlowPCAFlow", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::optflow::OpticalFlowPCAFlow> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_OpticalFlowPCAFlow_OpticalFlowPCAFlow(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::optflow::OpticalFlowPCAFlow::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { OpticalFlowPCAFlow, core::Algorithm, cv_optflow_OpticalFlowPCAFlow_to_Algorithm }

boxed_cast_base! { OpticalFlowPCAFlow, crate::video::DenseOpticalFlow, cv_optflow_OpticalFlowPCAFlow_to_DenseOpticalFlow }

impl std::fmt::Debug for OpticalFlowPCAFlow {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("OpticalFlowPCAFlow")
			.finish()
	}
}

/// Constant methods for [crate::optflow::PCAPrior]
// PCAPrior /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/pcaflow.hpp:74
pub trait PCAPriorTraitConst {
	fn as_raw_PCAPrior(&self) -> *const c_void;

	// getPadding()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/pcaflow.hpp:85
	// ("cv::optflow::PCAPrior::getPadding", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_padding(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_PCAPrior_getPadding_const(self.as_raw_PCAPrior(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getBasisSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/pcaflow.hpp:87
	// ("cv::optflow::PCAPrior::getBasisSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_basis_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_PCAPrior_getBasisSize_const(self.as_raw_PCAPrior(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// fillConstraints(float *, float *, float *, float *)(Indirect, Indirect, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/pcaflow.hpp:89
	// ("cv::optflow::PCAPrior::fillConstraints", vec![(pred!(const, ["A1", "A2", "b1", "b2"], ["float*", "float*", "float*", "float*"]), _)]),
	#[inline]
	fn fill_constraints(&self, a1: &mut f32, a2: &mut f32, b1: &mut f32, b2: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_PCAPrior_fillConstraints_const_floatX_floatX_floatX_floatX(self.as_raw_PCAPrior(), a1, a2, b1, b2, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::optflow::PCAPrior]
pub trait PCAPriorTrait: crate::optflow::PCAPriorTraitConst {
	fn as_raw_mut_PCAPrior(&mut self) -> *mut c_void;

}

///
/// This class can be used for imposing a learned prior on the resulting optical flow.
/// Solution will be regularized according to this prior.
/// You need to generate appropriate prior file with "learn_prior.py" script beforehand.
// PCAPrior /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/pcaflow.hpp:74
pub struct PCAPrior {
	ptr: *mut c_void,
}

opencv_type_boxed! { PCAPrior }

impl Drop for PCAPrior {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_optflow_PCAPrior_delete(self.as_raw_mut_PCAPrior()) };
	}
}

unsafe impl Send for PCAPrior {}

impl crate::optflow::PCAPriorTraitConst for PCAPrior {
	#[inline] fn as_raw_PCAPrior(&self) -> *const c_void { self.as_raw() }
}

impl crate::optflow::PCAPriorTrait for PCAPrior {
	#[inline] fn as_raw_mut_PCAPrior(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PCAPrior, crate::optflow::PCAPriorTraitConst, as_raw_PCAPrior, crate::optflow::PCAPriorTrait, as_raw_mut_PCAPrior }

impl PCAPrior {
	// PCAPrior(const char *)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/pcaflow.hpp:83
	// ("cv::optflow::PCAPrior::PCAPrior", vec![(pred!(mut, ["pathToPrior"], ["const char*"]), _)]),
	#[inline]
	pub fn new(path_to_prior: &str) -> Result<crate::optflow::PCAPrior> {
		extern_container_arg!(path_to_prior);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_PCAPrior_PCAPrior_const_charX(path_to_prior.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::optflow::PCAPrior::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for PCAPrior {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PCAPrior")
			.finish()
	}
}

/// Constant methods for [crate::optflow::RLOFOpticalFlowParameter]
// RLOFOpticalFlowParameter /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:64
pub trait RLOFOpticalFlowParameterTraitConst {
	fn as_raw_RLOFOpticalFlowParameter(&self) -> *const c_void;

	// cv::optflow::RLOFOpticalFlowParameter::solverType() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:83
	// ("cv::optflow::RLOFOpticalFlowParameter::solverType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn solver_type(&self) -> crate::optflow::SolverType {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propSolverType_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	// cv::optflow::RLOFOpticalFlowParameter::supportRegionType() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:88
	// ("cv::optflow::RLOFOpticalFlowParameter::supportRegionType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn support_region_type(&self) -> crate::optflow::SupportRegionType {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propSupportRegionType_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	// cv::optflow::RLOFOpticalFlowParameter::normSigma0() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:92
	// ("cv::optflow::RLOFOpticalFlowParameter::normSigma0", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn norm_sigma0(&self) -> f32 {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propNormSigma0_const(self.as_raw_RLOFOpticalFlowParameter()) };
		ret
	}

	// cv::optflow::RLOFOpticalFlowParameter::normSigma1() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:98
	// ("cv::optflow::RLOFOpticalFlowParameter::normSigma1", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn norm_sigma1(&self) -> f32 {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propNormSigma1_const(self.as_raw_RLOFOpticalFlowParameter()) };
		ret
	}

	// cv::optflow::RLOFOpticalFlowParameter::smallWinSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:104
	// ("cv::optflow::RLOFOpticalFlowParameter::smallWinSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn small_win_size(&self) -> i32 {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propSmallWinSize_const(self.as_raw_RLOFOpticalFlowParameter()) };
		ret
	}

	// cv::optflow::RLOFOpticalFlowParameter::largeWinSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:107
	// ("cv::optflow::RLOFOpticalFlowParameter::largeWinSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn large_win_size(&self) -> i32 {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propLargeWinSize_const(self.as_raw_RLOFOpticalFlowParameter()) };
		ret
	}

	// cv::optflow::RLOFOpticalFlowParameter::crossSegmentationThreshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:112
	// ("cv::optflow::RLOFOpticalFlowParameter::crossSegmentationThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn cross_segmentation_threshold(&self) -> i32 {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propCrossSegmentationThreshold_const(self.as_raw_RLOFOpticalFlowParameter()) };
		ret
	}

	// cv::optflow::RLOFOpticalFlowParameter::maxLevel() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:117
	// ("cv::optflow::RLOFOpticalFlowParameter::maxLevel", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn max_level(&self) -> i32 {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propMaxLevel_const(self.as_raw_RLOFOpticalFlowParameter()) };
		ret
	}

	// cv::optflow::RLOFOpticalFlowParameter::useInitialFlow() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:122
	// ("cv::optflow::RLOFOpticalFlowParameter::useInitialFlow", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn use_initial_flow(&self) -> bool {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propUseInitialFlow_const(self.as_raw_RLOFOpticalFlowParameter()) };
		ret
	}

	// cv::optflow::RLOFOpticalFlowParameter::useIlluminationModel() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:126
	// ("cv::optflow::RLOFOpticalFlowParameter::useIlluminationModel", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn use_illumination_model(&self) -> bool {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propUseIlluminationModel_const(self.as_raw_RLOFOpticalFlowParameter()) };
		ret
	}

	// cv::optflow::RLOFOpticalFlowParameter::useGlobalMotionPrior() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:134
	// ("cv::optflow::RLOFOpticalFlowParameter::useGlobalMotionPrior", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn use_global_motion_prior(&self) -> bool {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propUseGlobalMotionPrior_const(self.as_raw_RLOFOpticalFlowParameter()) };
		ret
	}

	// cv::optflow::RLOFOpticalFlowParameter::maxIteration() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:139
	// ("cv::optflow::RLOFOpticalFlowParameter::maxIteration", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn max_iteration(&self) -> i32 {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propMaxIteration_const(self.as_raw_RLOFOpticalFlowParameter()) };
		ret
	}

	// cv::optflow::RLOFOpticalFlowParameter::minEigenValue() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:143
	// ("cv::optflow::RLOFOpticalFlowParameter::minEigenValue", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn min_eigen_value(&self) -> f32 {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propMinEigenValue_const(self.as_raw_RLOFOpticalFlowParameter()) };
		ret
	}

	// cv::optflow::RLOFOpticalFlowParameter::globalMotionRansacThreshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:147
	// ("cv::optflow::RLOFOpticalFlowParameter::globalMotionRansacThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn global_motion_ransac_threshold(&self) -> f32 {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propGlobalMotionRansacThreshold_const(self.as_raw_RLOFOpticalFlowParameter()) };
		ret
	}

	// getSolverType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:163
	// ("cv::optflow::RLOFOpticalFlowParameter::getSolverType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_solver_type(&self) -> Result<crate::optflow::SolverType> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getSolverType_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getSupportRegionType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:166
	// ("cv::optflow::RLOFOpticalFlowParameter::getSupportRegionType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_support_region_type(&self) -> Result<crate::optflow::SupportRegionType> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getSupportRegionType_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNormSigma0()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:169
	// ("cv::optflow::RLOFOpticalFlowParameter::getNormSigma0", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_norm_sigma0(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getNormSigma0_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNormSigma1()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:172
	// ("cv::optflow::RLOFOpticalFlowParameter::getNormSigma1", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_norm_sigma1(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getNormSigma1_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getSmallWinSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:175
	// ("cv::optflow::RLOFOpticalFlowParameter::getSmallWinSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_small_win_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getSmallWinSize_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getLargeWinSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:178
	// ("cv::optflow::RLOFOpticalFlowParameter::getLargeWinSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_large_win_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getLargeWinSize_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getCrossSegmentationThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:181
	// ("cv::optflow::RLOFOpticalFlowParameter::getCrossSegmentationThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_cross_segmentation_threshold(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getCrossSegmentationThreshold_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxLevel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:184
	// ("cv::optflow::RLOFOpticalFlowParameter::getMaxLevel", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_level(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getMaxLevel_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getUseInitialFlow()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:187
	// ("cv::optflow::RLOFOpticalFlowParameter::getUseInitialFlow", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_use_initial_flow(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getUseInitialFlow_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getUseIlluminationModel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:190
	// ("cv::optflow::RLOFOpticalFlowParameter::getUseIlluminationModel", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_use_illumination_model(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getUseIlluminationModel_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getUseGlobalMotionPrior()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:193
	// ("cv::optflow::RLOFOpticalFlowParameter::getUseGlobalMotionPrior", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_use_global_motion_prior(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getUseGlobalMotionPrior_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxIteration()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:196
	// ("cv::optflow::RLOFOpticalFlowParameter::getMaxIteration", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_iteration(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getMaxIteration_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMinEigenValue()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:199
	// ("cv::optflow::RLOFOpticalFlowParameter::getMinEigenValue", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_eigen_value(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getMinEigenValue_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getGlobalMotionRansacThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:202
	// ("cv::optflow::RLOFOpticalFlowParameter::getGlobalMotionRansacThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_global_motion_ransac_threshold(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getGlobalMotionRansacThreshold_const(self.as_raw_RLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::optflow::RLOFOpticalFlowParameter]
pub trait RLOFOpticalFlowParameterTrait: crate::optflow::RLOFOpticalFlowParameterTraitConst {
	fn as_raw_mut_RLOFOpticalFlowParameter(&mut self) -> *mut c_void;

	// cv::optflow::RLOFOpticalFlowParameter::setSolverType(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:83
	// ("cv::optflow::RLOFOpticalFlowParameter::setSolverType", vec![(pred!(mut, ["val"], ["const cv::optflow::SolverType"]), _)]),
	#[inline]
	fn set_solver_type(&mut self, val: crate::optflow::SolverType) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propSolverType_const_SolverType(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}

	// cv::optflow::RLOFOpticalFlowParameter::setSupportRegionType(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:88
	// ("cv::optflow::RLOFOpticalFlowParameter::setSupportRegionType", vec![(pred!(mut, ["val"], ["const cv::optflow::SupportRegionType"]), _)]),
	#[inline]
	fn set_support_region_type(&mut self, val: crate::optflow::SupportRegionType) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propSupportRegionType_const_SupportRegionType(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}

	// cv::optflow::RLOFOpticalFlowParameter::setNormSigma0(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:92
	// ("cv::optflow::RLOFOpticalFlowParameter::setNormSigma0", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_norm_sigma0(&mut self, val: f32) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propNormSigma0_const_float(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}

	// cv::optflow::RLOFOpticalFlowParameter::setNormSigma1(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:98
	// ("cv::optflow::RLOFOpticalFlowParameter::setNormSigma1", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_norm_sigma1(&mut self, val: f32) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propNormSigma1_const_float(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}

	// cv::optflow::RLOFOpticalFlowParameter::setSmallWinSize(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:104
	// ("cv::optflow::RLOFOpticalFlowParameter::setSmallWinSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_small_win_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propSmallWinSize_const_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}

	// cv::optflow::RLOFOpticalFlowParameter::setLargeWinSize(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:107
	// ("cv::optflow::RLOFOpticalFlowParameter::setLargeWinSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_large_win_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propLargeWinSize_const_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}

	// cv::optflow::RLOFOpticalFlowParameter::setCrossSegmentationThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:112
	// ("cv::optflow::RLOFOpticalFlowParameter::setCrossSegmentationThreshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_cross_segmentation_threshold(&mut self, val: i32) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propCrossSegmentationThreshold_const_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}

	// cv::optflow::RLOFOpticalFlowParameter::setMaxLevel(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:117
	// ("cv::optflow::RLOFOpticalFlowParameter::setMaxLevel", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_max_level(&mut self, val: i32) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propMaxLevel_const_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}

	// cv::optflow::RLOFOpticalFlowParameter::setUseInitialFlow(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:122
	// ("cv::optflow::RLOFOpticalFlowParameter::setUseInitialFlow", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_use_initial_flow(&mut self, val: bool) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propUseInitialFlow_const_bool(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}

	// cv::optflow::RLOFOpticalFlowParameter::setUseIlluminationModel(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:126
	// ("cv::optflow::RLOFOpticalFlowParameter::setUseIlluminationModel", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_use_illumination_model(&mut self, val: bool) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propUseIlluminationModel_const_bool(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}

	// cv::optflow::RLOFOpticalFlowParameter::setUseGlobalMotionPrior(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:134
	// ("cv::optflow::RLOFOpticalFlowParameter::setUseGlobalMotionPrior", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_use_global_motion_prior(&mut self, val: bool) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propUseGlobalMotionPrior_const_bool(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}

	// cv::optflow::RLOFOpticalFlowParameter::setMaxIteration(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:139
	// ("cv::optflow::RLOFOpticalFlowParameter::setMaxIteration", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_max_iteration(&mut self, val: i32) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propMaxIteration_const_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}

	// cv::optflow::RLOFOpticalFlowParameter::setMinEigenValue(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:143
	// ("cv::optflow::RLOFOpticalFlowParameter::setMinEigenValue", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_min_eigen_value(&mut self, val: f32) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propMinEigenValue_const_float(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}

	// cv::optflow::RLOFOpticalFlowParameter::setGlobalMotionRansacThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:147
	// ("cv::optflow::RLOFOpticalFlowParameter::setGlobalMotionRansacThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_global_motion_ransac_threshold(&mut self, val: f32) {
		let ret = unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_propGlobalMotionRansacThreshold_const_float(self.as_raw_mut_RLOFOpticalFlowParameter(), val) };
		ret
	}

	/// Enable M-estimator or disable and use least-square estimator.
	/// Enables M-estimator by setting sigma parameters to (3.2, 7.0). Disabling M-estimator can reduce
	///      *  runtime, while enabling can improve the accuracy.
	/// ## Parameters
	/// * val: If true M-estimator is used. If false least-square estimator is used.
	///      *    see also: setNormSigma0, setNormSigma1
	// setUseMEstimator(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:160
	// ("cv::optflow::RLOFOpticalFlowParameter::setUseMEstimator", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	#[inline]
	fn set_use_m_estimator(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setUseMEstimator_bool(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setSolverType(SolverType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:162
	// ("cv::optflow::RLOFOpticalFlowParameter::setSolverType", vec![(pred!(mut, ["val"], ["cv::optflow::SolverType"]), _)]),
	#[inline]
	fn set_solver_type_1(&mut self, val: crate::optflow::SolverType) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setSolverType_SolverType(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setSupportRegionType(SupportRegionType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:165
	// ("cv::optflow::RLOFOpticalFlowParameter::setSupportRegionType", vec![(pred!(mut, ["val"], ["cv::optflow::SupportRegionType"]), _)]),
	#[inline]
	fn set_support_region_type_1(&mut self, val: crate::optflow::SupportRegionType) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setSupportRegionType_SupportRegionType(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNormSigma0(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:168
	// ("cv::optflow::RLOFOpticalFlowParameter::setNormSigma0", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_norm_sigma0_1(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setNormSigma0_float(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNormSigma1(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:171
	// ("cv::optflow::RLOFOpticalFlowParameter::setNormSigma1", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_norm_sigma1_1(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setNormSigma1_float(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setSmallWinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:174
	// ("cv::optflow::RLOFOpticalFlowParameter::setSmallWinSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_small_win_size_1(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setSmallWinSize_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setLargeWinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:177
	// ("cv::optflow::RLOFOpticalFlowParameter::setLargeWinSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_large_win_size_1(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setLargeWinSize_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setCrossSegmentationThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:180
	// ("cv::optflow::RLOFOpticalFlowParameter::setCrossSegmentationThreshold", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_cross_segmentation_threshold_1(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setCrossSegmentationThreshold_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxLevel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:183
	// ("cv::optflow::RLOFOpticalFlowParameter::setMaxLevel", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_max_level_1(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setMaxLevel_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setUseInitialFlow(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:186
	// ("cv::optflow::RLOFOpticalFlowParameter::setUseInitialFlow", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	#[inline]
	fn set_use_initial_flow_1(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setUseInitialFlow_bool(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setUseIlluminationModel(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:189
	// ("cv::optflow::RLOFOpticalFlowParameter::setUseIlluminationModel", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	#[inline]
	fn set_use_illumination_model_1(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setUseIlluminationModel_bool(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setUseGlobalMotionPrior(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:192
	// ("cv::optflow::RLOFOpticalFlowParameter::setUseGlobalMotionPrior", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	#[inline]
	fn set_use_global_motion_prior_1(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setUseGlobalMotionPrior_bool(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxIteration(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:195
	// ("cv::optflow::RLOFOpticalFlowParameter::setMaxIteration", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_max_iteration_1(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setMaxIteration_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinEigenValue(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:198
	// ("cv::optflow::RLOFOpticalFlowParameter::setMinEigenValue", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_min_eigen_value_1(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setMinEigenValue_float(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setGlobalMotionRansacThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:201
	// ("cv::optflow::RLOFOpticalFlowParameter::setGlobalMotionRansacThreshold", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_global_motion_ransac_threshold_1(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setGlobalMotionRansacThreshold_float(self.as_raw_mut_RLOFOpticalFlowParameter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// This is used store and set up the parameters of the robust local optical flow (RLOF) algoritm.
///
/// The RLOF is a fast local optical flow approach described in [Senst2012](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2012) [Senst2013](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2013) [Senst2014](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2014)
/// and [Senst2016](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2016) similar to the pyramidal iterative Lucas-Kanade method as
/// proposed by [Bouguet00](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Bouguet00). More details and experiments can be found in the following thesis [Senst2019](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2019).
/// The implementation is derived from optflow::calcOpticalFlowPyrLK().
/// This RLOF implementation can be seen as an improved pyramidal iterative Lucas-Kanade and includes
/// a set of improving modules. The main improvements in respect to the pyramidal iterative Lucas-Kanade
/// are:
///  - A more robust redecending M-estimator framework (see [Senst2012](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2012)) to improve the accuracy at
///    motion boundaries and appearing and disappearing pixels.
///  - an adaptive support region strategies to improve the accuracy at motion boundaries to reduce the
///    corona effect, i.e oversmoothing of the PLK at motion/object boundaries. The cross-based segementation
///    strategy (SR_CROSS) proposed in [Senst2014](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2014) uses a simple segmenation approach to obtain the optimal
///    shape of the support region.
///  - To deal with illumination changes (outdoor sequences and shadow) the intensity constancy assumption
///    based optical flow equation has been adopt with the Gennert and Negahdaripour illumination model
///    (see [Senst2016](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2016)). This model can be switched on/off with the useIlluminationModel variable.
///  - By using a global motion prior initialization (see [Senst2016](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2016)) of the iterative refinement
///    the accuracy could be significantly improved for large displacements. This initialization can be
///    switched on and of with useGlobalMotionPrior variable.
///
/// The RLOF can be computed with the SparseOpticalFlow class or function interface to track a set of features
/// or with the DenseOpticalFlow class or function interface to compute dense optical flow.
/// ## See also
/// optflow::DenseRLOFOpticalFlow, optflow::calcOpticalFlowDenseRLOF(), optflow::SparseRLOFOpticalFlow, optflow::calcOpticalFlowSparseRLOF()
// RLOFOpticalFlowParameter /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:64
pub struct RLOFOpticalFlowParameter {
	ptr: *mut c_void,
}

opencv_type_boxed! { RLOFOpticalFlowParameter }

impl Drop for RLOFOpticalFlowParameter {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_delete(self.as_raw_mut_RLOFOpticalFlowParameter()) };
	}
}

unsafe impl Send for RLOFOpticalFlowParameter {}

impl crate::optflow::RLOFOpticalFlowParameterTraitConst for RLOFOpticalFlowParameter {
	#[inline] fn as_raw_RLOFOpticalFlowParameter(&self) -> *const c_void { self.as_raw() }
}

impl crate::optflow::RLOFOpticalFlowParameterTrait for RLOFOpticalFlowParameter {
	#[inline] fn as_raw_mut_RLOFOpticalFlowParameter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { RLOFOpticalFlowParameter, crate::optflow::RLOFOpticalFlowParameterTraitConst, as_raw_RLOFOpticalFlowParameter, crate::optflow::RLOFOpticalFlowParameterTrait, as_raw_mut_RLOFOpticalFlowParameter }

impl RLOFOpticalFlowParameter {
	// RLOFOpticalFlowParameter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:66
	// ("cv::optflow::RLOFOpticalFlowParameter::RLOFOpticalFlowParameter", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::optflow::RLOFOpticalFlowParameter> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_RLOFOpticalFlowParameter(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::optflow::RLOFOpticalFlowParameter::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates instance of optflow::RLOFOpticalFlowParameter
	// create()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:205
	// ("cv::optflow::RLOFOpticalFlowParameter::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::optflow::RLOFOpticalFlowParameter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::optflow::RLOFOpticalFlowParameter>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for RLOFOpticalFlowParameter {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("RLOFOpticalFlowParameter")
			.field("solver_type", &crate::optflow::RLOFOpticalFlowParameterTraitConst::solver_type(self))
			.field("support_region_type", &crate::optflow::RLOFOpticalFlowParameterTraitConst::support_region_type(self))
			.field("norm_sigma0", &crate::optflow::RLOFOpticalFlowParameterTraitConst::norm_sigma0(self))
			.field("norm_sigma1", &crate::optflow::RLOFOpticalFlowParameterTraitConst::norm_sigma1(self))
			.field("small_win_size", &crate::optflow::RLOFOpticalFlowParameterTraitConst::small_win_size(self))
			.field("large_win_size", &crate::optflow::RLOFOpticalFlowParameterTraitConst::large_win_size(self))
			.field("cross_segmentation_threshold", &crate::optflow::RLOFOpticalFlowParameterTraitConst::cross_segmentation_threshold(self))
			.field("max_level", &crate::optflow::RLOFOpticalFlowParameterTraitConst::max_level(self))
			.field("use_initial_flow", &crate::optflow::RLOFOpticalFlowParameterTraitConst::use_initial_flow(self))
			.field("use_illumination_model", &crate::optflow::RLOFOpticalFlowParameterTraitConst::use_illumination_model(self))
			.field("use_global_motion_prior", &crate::optflow::RLOFOpticalFlowParameterTraitConst::use_global_motion_prior(self))
			.field("max_iteration", &crate::optflow::RLOFOpticalFlowParameterTraitConst::max_iteration(self))
			.field("min_eigen_value", &crate::optflow::RLOFOpticalFlowParameterTraitConst::min_eigen_value(self))
			.field("global_motion_ransac_threshold", &crate::optflow::RLOFOpticalFlowParameterTraitConst::global_motion_ransac_threshold(self))
			.finish()
	}
}

/// Constant methods for [crate::optflow::SparseRLOFOpticalFlow]
// SparseRLOFOpticalFlow /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:412
pub trait SparseRLOFOpticalFlowTraitConst: crate::video::SparseOpticalFlowTraitConst {
	fn as_raw_SparseRLOFOpticalFlow(&self) -> *const c_void;

	/// @copydoc DenseRLOFOpticalFlow::setRLOFOpticalFlowParameter
	/// ## See also
	/// setRLOFOpticalFlowParameter
	// getRLOFOpticalFlowParameter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:421
	// ("cv::optflow::SparseRLOFOpticalFlow::getRLOFOpticalFlowParameter", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_rlof_optical_flow_parameter(&self) -> Result<core::Ptr<crate::optflow::RLOFOpticalFlowParameter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_SparseRLOFOpticalFlow_getRLOFOpticalFlowParameter_const(self.as_raw_SparseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::optflow::RLOFOpticalFlowParameter>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Threshold for the forward backward confidence check
	/// For each feature point a motion vector ![inline formula](https://latex.codecogs.com/png.latex?%20d%5F%7BI0%2CI1%7D%28%5Cmathbf%7Bx%7D%29%20) is computed.
	///      *     If the forward backward error ![block formula](https://latex.codecogs.com/png.latex?%20EP%5F%7BFB%7D%20%3D%20%7C%7C%20d%5F%7BI0%2CI1%7D%20%2B%20d%5F%7BI1%2CI0%7D%20%7C%7C%20)
	///      *     is larger than threshold given by this function then the status  will not be used by the following
	///      *    vector field interpolation. ![inline formula](https://latex.codecogs.com/png.latex?%20d%5F%7BI1%2CI0%7D%20) denotes the backward flow. Note, the forward backward test
	///      *    will only be applied if the threshold > 0. This may results into a doubled runtime for the motion estimation.
	///      *    setForwardBackward
	/// ## See also
	/// setForwardBackward
	// getForwardBackward()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:434
	// ("cv::optflow::SparseRLOFOpticalFlow::getForwardBackward", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_forward_backward(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_SparseRLOFOpticalFlow_getForwardBackward_const(self.as_raw_SparseRLOFOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::optflow::SparseRLOFOpticalFlow]
pub trait SparseRLOFOpticalFlowTrait: crate::optflow::SparseRLOFOpticalFlowTraitConst + crate::video::SparseOpticalFlowTrait {
	fn as_raw_mut_SparseRLOFOpticalFlow(&mut self) -> *mut c_void;

	/// @copydoc DenseRLOFOpticalFlow::setRLOFOpticalFlowParameter
	// setRLOFOpticalFlowParameter(Ptr<RLOFOpticalFlowParameter>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:417
	// ("cv::optflow::SparseRLOFOpticalFlow::setRLOFOpticalFlowParameter", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>"]), _)]),
	#[inline]
	fn set_rlof_optical_flow_parameter(&mut self, mut val: core::Ptr<crate::optflow::RLOFOpticalFlowParameter>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_SparseRLOFOpticalFlow_setRLOFOpticalFlowParameter_PtrLRLOFOpticalFlowParameterG(self.as_raw_mut_SparseRLOFOpticalFlow(), val.as_raw_mut_PtrOfRLOFOpticalFlowParameter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Threshold for the forward backward confidence check
	/// For each feature point a motion vector ![inline formula](https://latex.codecogs.com/png.latex?%20d%5F%7BI0%2CI1%7D%28%5Cmathbf%7Bx%7D%29%20) is computed.
	///      *     If the forward backward error ![block formula](https://latex.codecogs.com/png.latex?%20EP%5F%7BFB%7D%20%3D%20%7C%7C%20d%5F%7BI0%2CI1%7D%20%2B%20d%5F%7BI1%2CI0%7D%20%7C%7C%20)
	///      *     is larger than threshold given by this function then the status  will not be used by the following
	///      *    vector field interpolation. ![inline formula](https://latex.codecogs.com/png.latex?%20d%5F%7BI1%2CI0%7D%20) denotes the backward flow. Note, the forward backward test
	///      *    will only be applied if the threshold > 0. This may results into a doubled runtime for the motion estimation.
	///      *    see also: setForwardBackward
	// setForwardBackward(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:430
	// ("cv::optflow::SparseRLOFOpticalFlow::setForwardBackward", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_forward_backward(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_SparseRLOFOpticalFlow_setForwardBackward_float(self.as_raw_mut_SparseRLOFOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class used for calculation sparse optical flow and feature tracking with robust local optical flow (RLOF) algorithms.
///
/// The RLOF is a fast local optical flow approach described in [Senst2012](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2012) [Senst2013](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2013) [Senst2014](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2014)
/// and [Senst2016](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2016) similar to the pyramidal iterative Lucas-Kanade method as
/// proposed by [Bouguet00](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Bouguet00). More details and experiments can be found in the following thesis [Senst2019](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2019).
/// The implementation is derived from optflow::calcOpticalFlowPyrLK().
///
/// For the RLOF configuration see optflow::RLOFOpticalFlowParameter for further details.
/// Parameters have been described in [Senst2012](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2012), [Senst2013](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2013), [Senst2014](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2014) and [Senst2016](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Senst2016).
///
///
/// Note: SIMD parallelization is only available when compiling with SSE4.1.
/// ## See also
/// optflow::calcOpticalFlowSparseRLOF(), optflow::RLOFOpticalFlowParameter
// SparseRLOFOpticalFlow /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:412
pub struct SparseRLOFOpticalFlow {
	ptr: *mut c_void,
}

opencv_type_boxed! { SparseRLOFOpticalFlow }

impl Drop for SparseRLOFOpticalFlow {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_optflow_SparseRLOFOpticalFlow_delete(self.as_raw_mut_SparseRLOFOpticalFlow()) };
	}
}

unsafe impl Send for SparseRLOFOpticalFlow {}

impl core::AlgorithmTraitConst for SparseRLOFOpticalFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SparseRLOFOpticalFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SparseRLOFOpticalFlow, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::video::SparseOpticalFlowTraitConst for SparseRLOFOpticalFlow {
	#[inline] fn as_raw_SparseOpticalFlow(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::SparseOpticalFlowTrait for SparseRLOFOpticalFlow {
	#[inline] fn as_raw_mut_SparseOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SparseRLOFOpticalFlow, crate::video::SparseOpticalFlowTraitConst, as_raw_SparseOpticalFlow, crate::video::SparseOpticalFlowTrait, as_raw_mut_SparseOpticalFlow }

impl crate::optflow::SparseRLOFOpticalFlowTraitConst for SparseRLOFOpticalFlow {
	#[inline] fn as_raw_SparseRLOFOpticalFlow(&self) -> *const c_void { self.as_raw() }
}

impl crate::optflow::SparseRLOFOpticalFlowTrait for SparseRLOFOpticalFlow {
	#[inline] fn as_raw_mut_SparseRLOFOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SparseRLOFOpticalFlow, crate::optflow::SparseRLOFOpticalFlowTraitConst, as_raw_SparseRLOFOpticalFlow, crate::optflow::SparseRLOFOpticalFlowTrait, as_raw_mut_SparseRLOFOpticalFlow }

impl SparseRLOFOpticalFlow {
	/// Creates instance of SparseRLOFOpticalFlow
	///
	/// ## Parameters
	/// * rlofParam: see setRLOFOpticalFlowParameter
	/// * forwardBackwardThreshold: see setForwardBackward
	///
	/// ## C++ default parameters
	/// * rlof_param: Ptr<RLOFOpticalFlowParameter>()
	/// * forward_backward_threshold: 1.f
	// create(Ptr<RLOFOpticalFlowParameter>, float)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:441
	// ("cv::optflow::SparseRLOFOpticalFlow::create", vec![(pred!(mut, ["rlofParam", "forwardBackwardThreshold"], ["cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>", "float"]), _)]),
	#[inline]
	pub fn create(mut rlof_param: core::Ptr<crate::optflow::RLOFOpticalFlowParameter>, forward_backward_threshold: f32) -> Result<core::Ptr<crate::optflow::SparseRLOFOpticalFlow>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_SparseRLOFOpticalFlow_create_PtrLRLOFOpticalFlowParameterG_float(rlof_param.as_raw_mut_PtrOfRLOFOpticalFlowParameter(), forward_backward_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::optflow::SparseRLOFOpticalFlow>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates instance of SparseRLOFOpticalFlow
	///
	/// ## Parameters
	/// * rlofParam: see setRLOFOpticalFlowParameter
	/// * forwardBackwardThreshold: see setForwardBackward
	///
	/// ## Note
	/// This alternative version of [SparseRLOFOpticalFlow::create] function uses the following default values for its arguments:
	/// * rlof_param: Ptr<RLOFOpticalFlowParameter>()
	/// * forward_backward_threshold: 1.f
	// cv::optflow::SparseRLOFOpticalFlow::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/optflow/rlofflow.hpp:441
	// ("cv::optflow::SparseRLOFOpticalFlow::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::optflow::SparseRLOFOpticalFlow>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_SparseRLOFOpticalFlow_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::optflow::SparseRLOFOpticalFlow>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { SparseRLOFOpticalFlow, core::Algorithm, cv_optflow_SparseRLOFOpticalFlow_to_Algorithm }

boxed_cast_base! { SparseRLOFOpticalFlow, crate::video::SparseOpticalFlow, cv_optflow_SparseRLOFOpticalFlow_to_SparseOpticalFlow }

impl std::fmt::Debug for SparseRLOFOpticalFlow {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SparseRLOFOpticalFlow")
			.finish()
	}
}
