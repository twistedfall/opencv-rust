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
	pub use super::{DISOpticalFlowTrait, DISOpticalFlowTraitConst, GPCDetailsTrait, GPCDetailsTraitConst, GPCPatchDescriptorTrait, GPCPatchDescriptorTraitConst, GPCPatchSampleTrait, GPCPatchSampleTraitConst, GPCTrainingSamplesTrait, GPCTrainingSamplesTraitConst, GPCTreeTrait, GPCTreeTraitConst, OpticalFlowPCAFlowTrait, OpticalFlowPCAFlowTraitConst, PCAPriorTrait, PCAPriorTraitConst, VariationalRefinementTrait, VariationalRefinementTraitConst};
}

// PRESET_FAST /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:273
pub const DISOpticalFlow_PRESET_FAST: i32 = 1;
// PRESET_MEDIUM /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:274
pub const DISOpticalFlow_PRESET_MEDIUM: i32 = 2;
// PRESET_ULTRAFAST /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:272
pub const DISOpticalFlow_PRESET_ULTRAFAST: i32 = 0;
/// Better quality but slow
// GPC_DESCRIPTOR_DCT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:92
pub const GPC_DESCRIPTOR_DCT: i32 = 0;
/// Worse quality but much faster
// GPC_DESCRIPTOR_WHT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:93
pub const GPC_DESCRIPTOR_WHT: i32 = 1;
/// Descriptor types for the Global Patch Collider.
// GPCDescType /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:90
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

// GPCSamplesVector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:86
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
// calcGlobalOrientation(InputArray, InputArray, InputArray, double, double)(InputArray, InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/motempl.hpp:119
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
// cv::motempl::calcMotionGradient(InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/motempl.hpp:102
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
// calcMotionGradient(InputArray, OutputArray, OutputArray, double, double, int)(InputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/motempl.hpp:102
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
// segmentMotion(InputArray, OutputArray, std::vector<Rect> &, double, double)(InputArray, OutputArray, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/motempl.hpp:137
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
/// templates technique described in [Davis97](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Davis97) and [Bradski00](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Bradski00) .
// updateMotionHistory(InputArray, InputOutputArray, double, double)(InputArray, InputOutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/motempl.hpp:71
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
/// See [Tao2012](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Tao2012) . And site of project - <http://graphics.berkeley.edu/papers/Tao-SAN-2012-05/>.
///
///
/// Note:
///    *   An example using the simpleFlow algorithm can be found at samples/simpleflow_demo.cpp
///
/// ## Overloaded parameters
// calcOpticalFlowSF(InputArray, InputArray, OutputArray, int, int, int)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:81
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
/// See [Tao2012](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Tao2012) . And site of project - <http://graphics.berkeley.edu/papers/Tao-SAN-2012-05/>.
///
///
/// Note:
///    *   An example using the simpleFlow algorithm can be found at samples/simpleflow_demo.cpp
// calcOpticalFlowSF(InputArray, InputArray, OutputArray, int, int, int, double, double, int, double, double, double, int, double, double, double)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:110
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
// cv::optflow::calcOpticalFlowSparseToDense(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:135
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
// calcOpticalFlowSparseToDense(InputArray, InputArray, OutputArray, int, int, float, bool, float, float)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:135
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

/// Creates an instance of DISOpticalFlow
///
/// ## Parameters
/// * preset: one of PRESET_ULTRAFAST, PRESET_FAST and PRESET_MEDIUM
///
/// ## Note
/// This alternative version of [create_opt_flow_dis] function uses the following default values for its arguments:
/// * preset: DISOpticalFlow::PRESET_FAST
// cv::optflow::createOptFlow_DIS() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:355
// ("cv::optflow::createOptFlow_DIS", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn create_opt_flow_dis_def() -> Result<core::Ptr<crate::optflow::DISOpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_createOptFlow_DIS(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::optflow::DISOpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Creates an instance of DISOpticalFlow
///
/// ## Parameters
/// * preset: one of PRESET_ULTRAFAST, PRESET_FAST and PRESET_MEDIUM
///
/// ## C++ default parameters
/// * preset: DISOpticalFlow::PRESET_FAST
// createOptFlow_DIS(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:355
// ("cv::optflow::createOptFlow_DIS", vec![(pred!(mut, ["preset"], ["int"]), _)]),
#[inline]
pub fn create_opt_flow_dis(preset: i32) -> Result<core::Ptr<crate::optflow::DISOpticalFlow>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_createOptFlow_DIS_int(preset, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::optflow::DISOpticalFlow>::opencv_from_extern(ret) };
	Ok(ret)
}

/// DeepFlow optical flow algorithm implementation.
///
/// The class implements the DeepFlow optical flow algorithm described in [Weinzaepfel2013](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Weinzaepfel2013) . See
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
// createOptFlow_DeepFlow()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:244
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

/// Additional interface to the Farneback's algorithm - calcOpticalFlowFarneback()
// createOptFlow_Farneback()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:250
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
// createOptFlow_PCAFlow()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:142
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
// createOptFlow_SimpleFlow()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:247
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

/// Additional interface to the SparseToDenseFlow algorithm - calcOpticalFlowSparseToDense()
// createOptFlow_SparseToDense()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:253
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

/// Creates an instance of VariationalRefinement
// createVariationalFlowRefinement()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:217
// ("cv::optflow::createVariationalFlowRefinement", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn create_variational_flow_refinement() -> Result<core::Ptr<crate::optflow::VariationalRefinement>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_createVariationalFlowRefinement(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::optflow::VariationalRefinement>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Read a .flo file
///
/// ## Parameters
/// * path: Path to the file to be loaded
///
/// The function readOpticalFlow loads a flow field from a file and returns it as a single matrix.
/// Resulting Mat has a type CV_32FC2 - floating-point, 2-channel. First channel corresponds to the
/// flow in the horizontal direction (u), second - vertical (v).
// readOpticalFlow(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:148
// ("cv::optflow::readOpticalFlow", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
#[inline]
pub fn read_optical_flow(path: &str) -> Result<core::Mat> {
	extern_container_arg!(path);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_readOpticalFlow_const_StringR(path.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Write a .flo to disk
///
/// ## Parameters
/// * path: Path to the file to be written
/// * flow: Flow field to be stored
///
/// The function stores a flow field in a file, returns true on success, false otherwise.
/// The flow field must be a 2-channel, floating-point matrix (CV_32FC2). First channel corresponds
/// to the flow in the horizontal direction (u), second - vertical (v).
// writeOpticalFlow(const String &, InputArray)(InString, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:158
// ("cv::optflow::writeOpticalFlow", vec![(pred!(mut, ["path", "flow"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn write_optical_flow(path: &str, flow: &impl ToInputArray) -> Result<bool> {
	extern_container_arg!(path);
	input_array_arg!(flow);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_optflow_writeOpticalFlow_const_StringR_const__InputArrayR(path.opencv_as_extern(), flow.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// read(const FileNode &, optflow::GPCTree::Node &, optflow::GPCTree::Node)(TraitClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:369
// ("cv::read", vec![(pred!(mut, ["fn", "node", "unnamed"], ["const cv::FileNode*", "cv::optflow::GPCTree::Node*", "cv::optflow::GPCTree::Node"]), _)]),
#[inline]
pub fn read(fn_: &impl core::FileNodeTraitConst, node: &mut crate::optflow::GPCTree_Node, unnamed: crate::optflow::GPCTree_Node) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_read_const_FileNodeR_NodeR_Node(fn_.as_raw_FileNode(), node, &unnamed, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// write(FileStorage &, const String &, const optflow::GPCTree::Node &)(TraitClass, InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:367
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

/// Constant methods for [crate::optflow::DISOpticalFlow]
// DISOpticalFlow /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:267
pub trait DISOpticalFlowTraitConst: crate::video::DenseOpticalFlowTraitConst {
	fn as_raw_DISOpticalFlow(&self) -> *const c_void;

	/// Finest level of the Gaussian pyramid on which the flow is computed (zero level
	/// corresponds to the original image resolution). The final flow is obtained by bilinear upscaling.
	/// ## See also
	/// setFinestScale
	// getFinestScale()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:280
	// ("cv::optflow::DISOpticalFlow::getFinestScale", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_finest_scale(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DISOpticalFlow_getFinestScale_const(self.as_raw_DISOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Size of an image patch for matching (in pixels). Normally, default 8x8 patches work well
	/// enough in most cases.
	/// ## See also
	/// setPatchSize
	// getPatchSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:287
	// ("cv::optflow::DISOpticalFlow::getPatchSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_patch_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DISOpticalFlow_getPatchSize_const(self.as_raw_DISOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Stride between neighbor patches. Must be less than patch size. Lower values correspond
	/// to higher flow quality.
	/// ## See also
	/// setPatchStride
	// getPatchStride()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:294
	// ("cv::optflow::DISOpticalFlow::getPatchStride", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_patch_stride(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DISOpticalFlow_getPatchStride_const(self.as_raw_DISOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Maximum number of gradient descent iterations in the patch inverse search stage. Higher values
	/// may improve quality in some cases.
	/// ## See also
	/// setGradientDescentIterations
	// getGradientDescentIterations()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:301
	// ("cv::optflow::DISOpticalFlow::getGradientDescentIterations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_gradient_descent_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DISOpticalFlow_getGradientDescentIterations_const(self.as_raw_DISOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Number of fixed point iterations of variational refinement per scale. Set to zero to
	///    disable variational refinement completely. Higher values will typically result in more smooth and
	///    high-quality flow.
	/// ## See also
	/// setGradientDescentIterations
	// getVariationalRefinementIterations()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:309
	// ("cv::optflow::DISOpticalFlow::getVariationalRefinementIterations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_variational_refinement_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DISOpticalFlow_getVariationalRefinementIterations_const(self.as_raw_DISOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weight of the smoothness term
	/// ## See also
	/// setVariationalRefinementAlpha
	// getVariationalRefinementAlpha()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:315
	// ("cv::optflow::DISOpticalFlow::getVariationalRefinementAlpha", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_variational_refinement_alpha(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DISOpticalFlow_getVariationalRefinementAlpha_const(self.as_raw_DISOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weight of the color constancy term
	/// ## See also
	/// setVariationalRefinementDelta
	// getVariationalRefinementDelta()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:321
	// ("cv::optflow::DISOpticalFlow::getVariationalRefinementDelta", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_variational_refinement_delta(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DISOpticalFlow_getVariationalRefinementDelta_const(self.as_raw_DISOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weight of the gradient constancy term
	/// ## See also
	/// setVariationalRefinementGamma
	// getVariationalRefinementGamma()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:327
	// ("cv::optflow::DISOpticalFlow::getVariationalRefinementGamma", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_variational_refinement_gamma(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DISOpticalFlow_getVariationalRefinementGamma_const(self.as_raw_DISOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Whether to use mean-normalization of patches when computing patch distance. It is turned on
	///    by default as it typically provides a noticeable quality boost because of increased robustness to
	///    illumination variations. Turn it off if you are certain that your sequence doesn't contain any changes
	///    in illumination.
	/// ## See also
	/// setUseMeanNormalization
	// getUseMeanNormalization()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:337
	// ("cv::optflow::DISOpticalFlow::getUseMeanNormalization", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_use_mean_normalization(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DISOpticalFlow_getUseMeanNormalization_const(self.as_raw_DISOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Whether to use spatial propagation of good optical flow vectors. This option is turned on by
	///    default, as it tends to work better on average and can sometimes help recover from major errors
	///    introduced by the coarse-to-fine scheme employed by the DIS optical flow algorithm. Turning this
	///    option off can make the output flow field a bit smoother, however.
	/// ## See also
	/// setUseSpatialPropagation
	// getUseSpatialPropagation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:346
	// ("cv::optflow::DISOpticalFlow::getUseSpatialPropagation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_use_spatial_propagation(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DISOpticalFlow_getUseSpatialPropagation_const(self.as_raw_DISOpticalFlow(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::optflow::DISOpticalFlow]
pub trait DISOpticalFlowTrait: crate::optflow::DISOpticalFlowTraitConst + crate::video::DenseOpticalFlowTrait {
	fn as_raw_mut_DISOpticalFlow(&mut self) -> *mut c_void;

	/// Finest level of the Gaussian pyramid on which the flow is computed (zero level
	/// corresponds to the original image resolution). The final flow is obtained by bilinear upscaling.
	/// ## See also
	/// setFinestScale getFinestScale
	// setFinestScale(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:282
	// ("cv::optflow::DISOpticalFlow::setFinestScale", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_finest_scale(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DISOpticalFlow_setFinestScale_int(self.as_raw_mut_DISOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Size of an image patch for matching (in pixels). Normally, default 8x8 patches work well
	/// enough in most cases.
	/// ## See also
	/// setPatchSize getPatchSize
	// setPatchSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:289
	// ("cv::optflow::DISOpticalFlow::setPatchSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_patch_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DISOpticalFlow_setPatchSize_int(self.as_raw_mut_DISOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Stride between neighbor patches. Must be less than patch size. Lower values correspond
	/// to higher flow quality.
	/// ## See also
	/// setPatchStride getPatchStride
	// setPatchStride(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:296
	// ("cv::optflow::DISOpticalFlow::setPatchStride", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_patch_stride(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DISOpticalFlow_setPatchStride_int(self.as_raw_mut_DISOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Maximum number of gradient descent iterations in the patch inverse search stage. Higher values
	/// may improve quality in some cases.
	/// ## See also
	/// setGradientDescentIterations getGradientDescentIterations
	// setGradientDescentIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:303
	// ("cv::optflow::DISOpticalFlow::setGradientDescentIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_gradient_descent_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DISOpticalFlow_setGradientDescentIterations_int(self.as_raw_mut_DISOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Maximum number of gradient descent iterations in the patch inverse search stage. Higher values
	/// may improve quality in some cases.
	/// ## See also
	/// setGradientDescentIterations getGradientDescentIterations
	// setVariationalRefinementIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:311
	// ("cv::optflow::DISOpticalFlow::setVariationalRefinementIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_variational_refinement_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DISOpticalFlow_setVariationalRefinementIterations_int(self.as_raw_mut_DISOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weight of the smoothness term
	/// ## See also
	/// setVariationalRefinementAlpha getVariationalRefinementAlpha
	// setVariationalRefinementAlpha(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:317
	// ("cv::optflow::DISOpticalFlow::setVariationalRefinementAlpha", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_variational_refinement_alpha(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DISOpticalFlow_setVariationalRefinementAlpha_float(self.as_raw_mut_DISOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weight of the color constancy term
	/// ## See also
	/// setVariationalRefinementDelta getVariationalRefinementDelta
	// setVariationalRefinementDelta(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:323
	// ("cv::optflow::DISOpticalFlow::setVariationalRefinementDelta", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_variational_refinement_delta(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DISOpticalFlow_setVariationalRefinementDelta_float(self.as_raw_mut_DISOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weight of the gradient constancy term
	/// ## See also
	/// setVariationalRefinementGamma getVariationalRefinementGamma
	// setVariationalRefinementGamma(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:329
	// ("cv::optflow::DISOpticalFlow::setVariationalRefinementGamma", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_variational_refinement_gamma(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DISOpticalFlow_setVariationalRefinementGamma_float(self.as_raw_mut_DISOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Whether to use mean-normalization of patches when computing patch distance. It is turned on
	///    by default as it typically provides a noticeable quality boost because of increased robustness to
	///    illumination variations. Turn it off if you are certain that your sequence doesn't contain any changes
	///    in illumination.
	/// ## See also
	/// setUseMeanNormalization getUseMeanNormalization
	// setUseMeanNormalization(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:339
	// ("cv::optflow::DISOpticalFlow::setUseMeanNormalization", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	#[inline]
	fn set_use_mean_normalization(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DISOpticalFlow_setUseMeanNormalization_bool(self.as_raw_mut_DISOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Whether to use spatial propagation of good optical flow vectors. This option is turned on by
	///    default, as it tends to work better on average and can sometimes help recover from major errors
	///    introduced by the coarse-to-fine scheme employed by the DIS optical flow algorithm. Turning this
	///    option off can make the output flow field a bit smoother, however.
	/// ## See also
	/// setUseSpatialPropagation getUseSpatialPropagation
	// setUseSpatialPropagation(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:348
	// ("cv::optflow::DISOpticalFlow::setUseSpatialPropagation", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	#[inline]
	fn set_use_spatial_propagation(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_DISOpticalFlow_setUseSpatialPropagation_bool(self.as_raw_mut_DISOpticalFlow(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// DIS optical flow algorithm.
///
/// This class implements the Dense Inverse Search (DIS) optical flow algorithm. More
/// details about the algorithm can be found at [Kroeger2016](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Kroeger2016) . Includes three presets with preselected
/// parameters to provide reasonable trade-off between speed and quality. However, even the slowest preset is
/// still relatively fast, use DeepFlow if you need better quality and don't care about speed.
///
/// This implementation includes several additional features compared to the algorithm described in the paper,
/// including spatial propagation of flow vectors ([getUseSpatialPropagation]), as well as an option to
/// utilize an initial flow approximation passed to [calc] (which is, essentially, temporal propagation,
/// if the previous frame's flow field is passed).
// DISOpticalFlow /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:267
pub struct DISOpticalFlow {
	ptr: *mut c_void,
}

opencv_type_boxed! { DISOpticalFlow }

impl Drop for DISOpticalFlow {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_optflow_DISOpticalFlow_delete(self.as_raw_mut_DISOpticalFlow()) };
	}
}

unsafe impl Send for DISOpticalFlow {}

impl core::AlgorithmTraitConst for DISOpticalFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for DISOpticalFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DISOpticalFlow, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::video::DenseOpticalFlowTraitConst for DISOpticalFlow {
	#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::DenseOpticalFlowTrait for DISOpticalFlow {
	#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DISOpticalFlow, crate::video::DenseOpticalFlowTraitConst, as_raw_DenseOpticalFlow, crate::video::DenseOpticalFlowTrait, as_raw_mut_DenseOpticalFlow }

impl crate::optflow::DISOpticalFlowTraitConst for DISOpticalFlow {
	#[inline] fn as_raw_DISOpticalFlow(&self) -> *const c_void { self.as_raw() }
}

impl crate::optflow::DISOpticalFlowTrait for DISOpticalFlow {
	#[inline] fn as_raw_mut_DISOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DISOpticalFlow, crate::optflow::DISOpticalFlowTraitConst, as_raw_DISOpticalFlow, crate::optflow::DISOpticalFlowTrait, as_raw_mut_DISOpticalFlow }

impl DISOpticalFlow {
}

boxed_cast_base! { DISOpticalFlow, core::Algorithm, cv_optflow_DISOpticalFlow_to_Algorithm }

boxed_cast_base! { DISOpticalFlow, crate::video::DenseOpticalFlow, cv_optflow_DISOpticalFlow_to_DenseOpticalFlow }

impl std::fmt::Debug for DISOpticalFlow {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("DISOpticalFlow")
			.finish()
	}
}

/// Constant methods for [crate::optflow::GPCDetails]
// GPCDetails /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:301
pub trait GPCDetailsTraitConst {
	fn as_raw_GPCDetails(&self) -> *const c_void;

}

/// Mutable methods for [crate::optflow::GPCDetails]
pub trait GPCDetailsTrait: crate::optflow::GPCDetailsTraitConst {
	fn as_raw_mut_GPCDetails(&mut self) -> *mut c_void;

}

// GPCDetails /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:301
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

	// dropOutliers(std::vector<std::pair<Point2i, Point2i>> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:304
	// ("cv::optflow::GPCDetails::dropOutliers", vec![(pred!(mut, ["corr"], ["std::vector<std::pair<cv::Point2i, cv::Point2i>>*"]), _)]),
	#[inline]
	pub fn drop_outliers(corr: &mut core::Vector<core::Tuple<(core::Point2i, core::Point2i)>>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCDetails_dropOutliers_vectorLpairLcv_Point2i__cv_Point2iGGR(corr.as_raw_mut_VectorOfTupleOfPoint2i_Point2i(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getAllDescriptorsForImage(const Mat *, std::vector<GPCPatchDescriptor> &, const GPCMatchingParams &, int)(TraitClass, CppPassByVoidPtr, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:306
	// ("cv::optflow::GPCDetails::getAllDescriptorsForImage", vec![(pred!(mut, ["imgCh", "descr", "mp", "type"], ["const cv::Mat*", "std::vector<cv::optflow::GPCPatchDescriptor>*", "const cv::optflow::GPCMatchingParams*", "int"]), _)]),
	#[inline]
	pub fn get_all_descriptors_for_image(img_ch: &impl core::MatTraitConst, descr: &mut core::Vector<crate::optflow::GPCPatchDescriptor>, mp: crate::optflow::GPCMatchingParams, typ: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCDetails_getAllDescriptorsForImage_const_MatX_vectorLGPCPatchDescriptorGR_const_GPCMatchingParamsR_int(img_ch.as_raw_Mat(), descr.as_raw_mut_VectorOfGPCPatchDescriptor(), &mp, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getCoordinatesFromIndex(size_t, Size, int &, int &)(Primitive, SimpleClass, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:309
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
// GPCMatchingParams /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:143
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
	// GPCMatchingParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:147
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
	// cv::optflow::GPCMatchingParams::GPCMatchingParams() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:147
	// ("cv::optflow::GPCMatchingParams::GPCMatchingParams", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::optflow::GPCMatchingParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCMatchingParams_GPCMatchingParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// GPCMatchingParams(const GPCMatchingParams &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:149
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
// GPCPatchDescriptor /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:65
pub trait GPCPatchDescriptorTraitConst {
	fn as_raw_GPCPatchDescriptor(&self) -> *const c_void;

	// cv::optflow::GPCPatchDescriptor::feature() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:68
	// ("cv::optflow::GPCPatchDescriptor::feature", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn feature(&self) -> core::VecN<f64, 18> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCPatchDescriptor_propFeature_const(self.as_raw_GPCPatchDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	// dot(const Vec<double, nFeatures> &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:70
	// ("cv::optflow::GPCPatchDescriptor::dot", vec![(pred!(const, ["coef"], ["const cv::Vec<double, 18>*"]), _)]),
	#[inline]
	fn dot(&self, coef: core::VecN<f64, 18>) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCPatchDescriptor_dot_const_const_VecLdouble__18GR(self.as_raw_GPCPatchDescriptor(), &coef, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// isSeparated()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:74
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

	// cv::optflow::GPCPatchDescriptor::setFeature(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:68
	// ("cv::optflow::GPCPatchDescriptor::setFeature", vec![(pred!(mut, ["val"], ["const cv::Vec<double, 18>"]), _)]),
	#[inline]
	fn set_feature(&mut self, val: core::VecN<f64, 18>) {
		let ret = unsafe { sys::cv_optflow_GPCPatchDescriptor_propFeature_const_VecLdouble__18G(self.as_raw_mut_GPCPatchDescriptor(), &val) };
		ret
	}

	// markAsSeparated()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:72
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

// GPCPatchDescriptor /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:65
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
	// nFeatures /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:67
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
// GPCPatchSample /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:77
pub trait GPCPatchSampleTraitConst {
	fn as_raw_GPCPatchSample(&self) -> *const c_void;

	// cv::optflow::GPCPatchSample::ref() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:79
	// ("cv::optflow::GPCPatchSample::ref", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn ref_(&self) -> crate::optflow::GPCPatchDescriptor {
		let ret = unsafe { sys::cv_optflow_GPCPatchSample_propRef_const(self.as_raw_GPCPatchSample()) };
		let ret = unsafe { crate::optflow::GPCPatchDescriptor::opencv_from_extern(ret) };
		ret
	}

	// cv::optflow::GPCPatchSample::pos() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:80
	// ("cv::optflow::GPCPatchSample::pos", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pos(&self) -> crate::optflow::GPCPatchDescriptor {
		let ret = unsafe { sys::cv_optflow_GPCPatchSample_propPos_const(self.as_raw_GPCPatchSample()) };
		let ret = unsafe { crate::optflow::GPCPatchDescriptor::opencv_from_extern(ret) };
		ret
	}

	// cv::optflow::GPCPatchSample::neg() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:81
	// ("cv::optflow::GPCPatchSample::neg", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn neg(&self) -> crate::optflow::GPCPatchDescriptor {
		let ret = unsafe { sys::cv_optflow_GPCPatchSample_propNeg_const(self.as_raw_GPCPatchSample()) };
		let ret = unsafe { crate::optflow::GPCPatchDescriptor::opencv_from_extern(ret) };
		ret
	}

	// getDirections(bool &, bool &, bool &, const Vec<double, GPCPatchDescriptor::nFeatures> &, double)(Indirect, Indirect, Indirect, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:83
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

	// cv::optflow::GPCPatchSample::setRef(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:79
	// ("cv::optflow::GPCPatchSample::setRef", vec![(pred!(mut, ["val"], ["const cv::optflow::GPCPatchDescriptor"]), _)]),
	#[inline]
	fn set_ref(&mut self, val: crate::optflow::GPCPatchDescriptor) {
		let ret = unsafe { sys::cv_optflow_GPCPatchSample_propRef_const_GPCPatchDescriptor(self.as_raw_mut_GPCPatchSample(), val.as_raw_GPCPatchDescriptor()) };
		ret
	}

	// cv::optflow::GPCPatchSample::setPos(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:80
	// ("cv::optflow::GPCPatchSample::setPos", vec![(pred!(mut, ["val"], ["const cv::optflow::GPCPatchDescriptor"]), _)]),
	#[inline]
	fn set_pos(&mut self, val: crate::optflow::GPCPatchDescriptor) {
		let ret = unsafe { sys::cv_optflow_GPCPatchSample_propPos_const_GPCPatchDescriptor(self.as_raw_mut_GPCPatchSample(), val.as_raw_GPCPatchDescriptor()) };
		ret
	}

	// cv::optflow::GPCPatchSample::setNeg(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:81
	// ("cv::optflow::GPCPatchSample::setNeg", vec![(pred!(mut, ["val"], ["const cv::optflow::GPCPatchDescriptor"]), _)]),
	#[inline]
	fn set_neg(&mut self, val: crate::optflow::GPCPatchDescriptor) {
		let ret = unsafe { sys::cv_optflow_GPCPatchSample_propNeg_const_GPCPatchDescriptor(self.as_raw_mut_GPCPatchSample(), val.as_raw_GPCPatchDescriptor()) };
		ret
	}

}

// GPCPatchSample /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:77
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
// GPCTrainingParams /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:123
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
	// check()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:138
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
	// GPCTrainingParams(unsigned int, int, GPCDescType, bool)(Primitive, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:130
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
	// cv::optflow::GPCTrainingParams::GPCTrainingParams() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:130
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
// GPCTrainingSamples /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:98
pub trait GPCTrainingSamplesTraitConst {
	fn as_raw_GPCTrainingSamples(&self) -> *const c_void;

	// size()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:114
	// ("cv::optflow::GPCTrainingSamples::size", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn size(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTrainingSamples_size_const(self.as_raw_GPCTrainingSamples(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// type()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:116
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
// GPCTrainingSamples /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:98
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
	// create(const std::vector<String> &, const std::vector<String> &, const std::vector<String> &, int)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:108
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

	// create(InputArrayOfArrays, InputArrayOfArrays, InputArrayOfArrays, int)(InputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:111
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
// GPCTree /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:154
pub trait GPCTreeTraitConst: core::AlgorithmTraitConst {
	fn as_raw_GPCTree(&self) -> *const c_void;

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:178
	// ("cv::optflow::GPCTree::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	#[inline]
	fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTree_write_const_FileStorageR(self.as_raw_GPCTree(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// findLeafForPatch(const GPCPatchDescriptor &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:182
	// ("cv::optflow::GPCTree::findLeafForPatch", vec![(pred!(const, ["descr"], ["const cv::optflow::GPCPatchDescriptor*"]), _)]),
	#[inline]
	fn find_leaf_for_patch(&self, descr: &impl crate::optflow::GPCPatchDescriptorTraitConst) -> Result<u32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTree_findLeafForPatch_const_const_GPCPatchDescriptorR(self.as_raw_GPCTree(), descr.as_raw_GPCPatchDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// operator==(const GPCTree &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:186
	// ("cv::optflow::GPCTree::operator==", vec![(pred!(const, ["t"], ["const cv::optflow::GPCTree*"]), _)]),
	#[inline]
	fn equals(&self, t: &impl crate::optflow::GPCTreeTraitConst) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTree_operatorEQ_const_const_GPCTreeR(self.as_raw_GPCTree(), t.as_raw_GPCTree(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDescriptorType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:188
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
	// train(GPCTrainingSamples &, const GPCTrainingParams)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:176
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
	// cv::optflow::GPCTree::train(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:176
	// ("cv::optflow::GPCTree::train", vec![(pred!(mut, ["samples"], ["cv::optflow::GPCTrainingSamples*"]), _)]),
	#[inline]
	fn train_def(&mut self, samples: &mut impl crate::optflow::GPCTrainingSamplesTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_GPCTree_train_GPCTrainingSamplesR(self.as_raw_mut_GPCTree(), samples.as_raw_mut_GPCTrainingSamples(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:180
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
// GPCTree /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:154
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

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:184
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

// Node /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:157
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
	// operator==(const Node &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/sparse_matching_gpc.hpp:164
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
// OpticalFlowPCAFlow /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:94
pub trait OpticalFlowPCAFlowTraitConst: crate::video::DenseOpticalFlowTraitConst {
	fn as_raw_OpticalFlowPCAFlow(&self) -> *const c_void;

}

/// Mutable methods for [crate::optflow::OpticalFlowPCAFlow]
pub trait OpticalFlowPCAFlowTrait: crate::optflow::OpticalFlowPCAFlowTraitConst + crate::video::DenseOpticalFlowTrait {
	fn as_raw_mut_OpticalFlowPCAFlow(&mut self) -> *mut c_void;

	// calc(InputArray, InputArray, InputOutputArray)(InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:120
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

	// collectGarbage()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:121
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
// OpticalFlowPCAFlow /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:94
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
	// OpticalFlowPCAFlow(Ptr<const PCAPrior>, const Size, float, float, float, float, float)(CppPassByVoidPtr, SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:116
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
	// cv::optflow::OpticalFlowPCAFlow::OpticalFlowPCAFlow() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:116
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
// PCAPrior /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:74
pub trait PCAPriorTraitConst {
	fn as_raw_PCAPrior(&self) -> *const c_void;

	// getPadding()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:85
	// ("cv::optflow::PCAPrior::getPadding", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_padding(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_PCAPrior_getPadding_const(self.as_raw_PCAPrior(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getBasisSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:87
	// ("cv::optflow::PCAPrior::getBasisSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_basis_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_PCAPrior_getBasisSize_const(self.as_raw_PCAPrior(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// fillConstraints(float *, float *, float *, float *)(Indirect, Indirect, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:89
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
// PCAPrior /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:74
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
	// PCAPrior(const char *)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow/pcaflow.hpp:83
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

/// Constant methods for [crate::optflow::VariationalRefinement]
// VariationalRefinement /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:170
pub trait VariationalRefinementTraitConst: crate::video::DenseOpticalFlowTraitConst {
	fn as_raw_VariationalRefinement(&self) -> *const c_void;

	/// Number of outer (fixed-point) iterations in the minimization procedure.
	/// ## See also
	/// setFixedPointIterations
	// getFixedPointIterations()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:179
	// ("cv::optflow::VariationalRefinement::getFixedPointIterations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_fixed_point_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_VariationalRefinement_getFixedPointIterations_const(self.as_raw_VariationalRefinement(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Number of inner successive over-relaxation (SOR) iterations
	///    in the minimization procedure to solve the respective linear system.
	/// ## See also
	/// setSorIterations
	// getSorIterations()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:186
	// ("cv::optflow::VariationalRefinement::getSorIterations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_sor_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_VariationalRefinement_getSorIterations_const(self.as_raw_VariationalRefinement(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Relaxation factor in SOR
	/// ## See also
	/// setOmega
	// getOmega()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:192
	// ("cv::optflow::VariationalRefinement::getOmega", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_omega(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_VariationalRefinement_getOmega_const(self.as_raw_VariationalRefinement(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weight of the smoothness term
	/// ## See also
	/// setAlpha
	// getAlpha()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:198
	// ("cv::optflow::VariationalRefinement::getAlpha", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_alpha(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_VariationalRefinement_getAlpha_const(self.as_raw_VariationalRefinement(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weight of the color constancy term
	/// ## See also
	/// setDelta
	// getDelta()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:204
	// ("cv::optflow::VariationalRefinement::getDelta", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_delta(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_VariationalRefinement_getDelta_const(self.as_raw_VariationalRefinement(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weight of the gradient constancy term
	/// ## See also
	/// setGamma
	// getGamma()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:210
	// ("cv::optflow::VariationalRefinement::getGamma", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_gamma(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_VariationalRefinement_getGamma_const(self.as_raw_VariationalRefinement(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::optflow::VariationalRefinement]
pub trait VariationalRefinementTrait: crate::optflow::VariationalRefinementTraitConst + crate::video::DenseOpticalFlowTrait {
	fn as_raw_mut_VariationalRefinement(&mut self) -> *mut c_void;

	/// [calc] function overload to handle separate horizontal (u) and vertical (v) flow components
	/// (to avoid extra splits/merges)
	// calcUV(InputArray, InputArray, InputOutputArray, InputOutputArray)(InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:175
	// ("cv::optflow::VariationalRefinement::calcUV", vec![(pred!(mut, ["I0", "I1", "flow_u", "flow_v"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	#[inline]
	fn calc_uv(&mut self, i0: &impl ToInputArray, i1: &impl ToInputArray, flow_u: &mut impl ToInputOutputArray, flow_v: &mut impl ToInputOutputArray) -> Result<()> {
		input_array_arg!(i0);
		input_array_arg!(i1);
		input_output_array_arg!(flow_u);
		input_output_array_arg!(flow_v);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_VariationalRefinement_calcUV_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(self.as_raw_mut_VariationalRefinement(), i0.as_raw__InputArray(), i1.as_raw__InputArray(), flow_u.as_raw__InputOutputArray(), flow_v.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Number of outer (fixed-point) iterations in the minimization procedure.
	/// ## See also
	/// setFixedPointIterations getFixedPointIterations
	// setFixedPointIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:181
	// ("cv::optflow::VariationalRefinement::setFixedPointIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_fixed_point_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_VariationalRefinement_setFixedPointIterations_int(self.as_raw_mut_VariationalRefinement(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Number of inner successive over-relaxation (SOR) iterations
	///    in the minimization procedure to solve the respective linear system.
	/// ## See also
	/// setSorIterations getSorIterations
	// setSorIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:188
	// ("cv::optflow::VariationalRefinement::setSorIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_sor_iterations(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_VariationalRefinement_setSorIterations_int(self.as_raw_mut_VariationalRefinement(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Relaxation factor in SOR
	/// ## See also
	/// setOmega getOmega
	// setOmega(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:194
	// ("cv::optflow::VariationalRefinement::setOmega", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_omega(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_VariationalRefinement_setOmega_float(self.as_raw_mut_VariationalRefinement(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weight of the smoothness term
	/// ## See also
	/// setAlpha getAlpha
	// setAlpha(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:200
	// ("cv::optflow::VariationalRefinement::setAlpha", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_alpha(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_VariationalRefinement_setAlpha_float(self.as_raw_mut_VariationalRefinement(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weight of the color constancy term
	/// ## See also
	/// setDelta getDelta
	// setDelta(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:206
	// ("cv::optflow::VariationalRefinement::setDelta", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_delta(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_VariationalRefinement_setDelta_float(self.as_raw_mut_VariationalRefinement(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weight of the gradient constancy term
	/// ## See also
	/// setGamma getGamma
	// setGamma(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:212
	// ("cv::optflow::VariationalRefinement::setGamma", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_gamma(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_optflow_VariationalRefinement_setGamma_float(self.as_raw_mut_VariationalRefinement(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Variational optical flow refinement
///
/// This class implements variational refinement of the input flow field, i.e.
/// it uses input flow to initialize the minimization of the following functional:
/// ![inline formula](https://latex.codecogs.com/png.latex?E%28U%29%20%3D%20%5Cint%5F%7B%5COmega%7D%20%5Cdelta%20%5CPsi%28E%5FI%29%20%2B%20%5Cgamma%20%5CPsi%28E%5FG%29%20%2B%20%5Calpha%20%5CPsi%28E%5FS%29%20),
/// where ![inline formula](https://latex.codecogs.com/png.latex?E%5FI%2CE%5FG%2CE%5FS) are color constancy, gradient constancy and smoothness terms
/// respectively. ![inline formula](https://latex.codecogs.com/png.latex?%5CPsi%28s%5E2%29%3D%5Csqrt%7Bs%5E2%2B%5Cepsilon%5E2%7D) is a robust penalizer to limit the
/// influence of outliers. A complete formulation and a description of the minimization
/// procedure can be found in [Brox2004](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Brox2004)
// VariationalRefinement /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/optflow.hpp:170
pub struct VariationalRefinement {
	ptr: *mut c_void,
}

opencv_type_boxed! { VariationalRefinement }

impl Drop for VariationalRefinement {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_optflow_VariationalRefinement_delete(self.as_raw_mut_VariationalRefinement()) };
	}
}

unsafe impl Send for VariationalRefinement {}

impl core::AlgorithmTraitConst for VariationalRefinement {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for VariationalRefinement {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { VariationalRefinement, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::video::DenseOpticalFlowTraitConst for VariationalRefinement {
	#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.as_raw() }
}

impl crate::video::DenseOpticalFlowTrait for VariationalRefinement {
	#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { VariationalRefinement, crate::video::DenseOpticalFlowTraitConst, as_raw_DenseOpticalFlow, crate::video::DenseOpticalFlowTrait, as_raw_mut_DenseOpticalFlow }

impl crate::optflow::VariationalRefinementTraitConst for VariationalRefinement {
	#[inline] fn as_raw_VariationalRefinement(&self) -> *const c_void { self.as_raw() }
}

impl crate::optflow::VariationalRefinementTrait for VariationalRefinement {
	#[inline] fn as_raw_mut_VariationalRefinement(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { VariationalRefinement, crate::optflow::VariationalRefinementTraitConst, as_raw_VariationalRefinement, crate::optflow::VariationalRefinementTrait, as_raw_mut_VariationalRefinement }

impl VariationalRefinement {
}

boxed_cast_base! { VariationalRefinement, core::Algorithm, cv_optflow_VariationalRefinement_to_Algorithm }

boxed_cast_base! { VariationalRefinement, crate::video::DenseOpticalFlow, cv_optflow_VariationalRefinement_to_DenseOpticalFlow }

impl std::fmt::Debug for VariationalRefinement {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("VariationalRefinement")
			.finish()
	}
}
