//! # Structure From Motion
//!
//! The opencv_sfm module contains algorithms to perform 3d reconstruction
//! from 2d images.
//!
//! The core of the module is based on a light version of
//! [Libmv](https://developer.blender.org/project/profile/59) originally
//! developed by Sameer Agarwal and Keir Mierle.
//!
//! __Whats is libmv?__ 
//!
//! libmv, also known as the Library for Multiview Reconstruction (or LMV),
//! is the computer vision backend for Blender's motion tracking abilities.
//! Unlike other vision libraries with general ambitions, libmv is focused
//! on algorithms for match moving, specifically targeting [Blender](https://developer.blender.org) as the
//! primary customer. Dense reconstruction, reconstruction from unorganized
//! photo collections, image recognition, and other tasks are not a focus
//! of libmv.
//!
//! __Development__ 
//!
//! libmv is officially under the Blender umbrella, and so is developed
//! on developer.blender.org. The [source repository](https://developer.blender.org/diffusion/LMV) can get checked out
//! independently from Blender.
//!
//! This module has been originally developed as a project for Google Summer of Code 2012-2015.
//!
//!
//! Note:
//!   - Notice that it is compiled only when Eigen, GLog and GFlags are correctly installed.
//!
//!    Check installation instructions in the following tutorial: [tutorial_sfm_installation]
//!    # Conditioning
//!    # Fundamental
//!    # Input/Output
//!    # Numeric
//!    # Projection
//!    # Robust Estimation
//!    # Triangulation
//!
//!    # Reconstruction
//!
//!     
//! Note:
//!    - Notice that it is compiled only when Ceres Solver is correctly installed.
//!
//!        Check installation instructions in the following tutorial: [tutorial_sfm_installation]
//!
//!    # Simple Pipeline
//!
//!     
//! Note:
//!        - Notice that it is compiled only when Ceres Solver is correctly installed.
//!
//!        Check installation instructions in the following tutorial: [tutorial_sfm_installation]
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{BaseSFMTrait, BaseSFMTraitConst, SFMLibmvEuclideanReconstructionTrait, SFMLibmvEuclideanReconstructionTraitConst};
}

// SFM_DISTORTION_MODEL_DIVISION /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:53
pub const SFM_DISTORTION_MODEL_DIVISION: i32 = 1;
// SFM_DISTORTION_MODEL_POLYNOMIAL /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:52
pub const SFM_DISTORTION_MODEL_POLYNOMIAL: i32 = 0;
// SFM_IO_BUNDLER /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/io.hpp:58
pub const SFM_IO_BUNDLER: i32 = 0;
// SFM_IO_OPENMVG /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/io.hpp:61
pub const SFM_IO_OPENMVG: i32 = 3;
// SFM_IO_OPENSFM /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/io.hpp:60
pub const SFM_IO_OPENSFM: i32 = 2;
// SFM_IO_THEIASFM /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/io.hpp:62
pub const SFM_IO_THEIASFM: i32 = 4;
// SFM_IO_VISUALSFM /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/io.hpp:59
pub const SFM_IO_VISUALSFM: i32 = 1;
// SFM_REFINE_FOCAL_LENGTH /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:124
pub const SFM_REFINE_FOCAL_LENGTH: i32 = 1;
// SFM_REFINE_PRINCIPAL_POINT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:125
pub const SFM_REFINE_PRINCIPAL_POINT: i32 = 2;
// SFM_REFINE_RADIAL_DISTORTION_K1 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:126
pub const SFM_REFINE_RADIAL_DISTORTION_K1: i32 = 4;
// SFM_REFINE_RADIAL_DISTORTION_K2 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:127
pub const SFM_REFINE_RADIAL_DISTORTION_K2: i32 = 16;
/// Get K, R and t from projection matrix P, decompose using the RQ decomposition.
/// ## Parameters
/// * P: Input 3x4 projection matrix.
/// * K: Output 3x3 camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D).
/// * R: Output 3x3 rotation matrix.
/// * t: Output 3x1 translation vector.
///
/// Reference: [HartleyZ00](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_HartleyZ00) A4.1.1 pag.579
// KRtFromProjection(InputArray, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/projection.hpp:88
// ("cv::sfm::KRtFromProjection", vec![(pred!(mut, ["P", "K", "R", "t"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn k_rt_from_projection(p: &impl ToInputArray, k: &mut impl ToOutputArray, r: &mut impl ToOutputArray, t: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(p);
	output_array_arg!(k);
	output_array_arg!(r);
	output_array_arg!(t);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_KRtFromProjection_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(p.as_raw__InputArray(), k.as_raw__OutputArray(), r.as_raw__OutputArray(), t.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Apply Transformation to points.
/// ## Parameters
/// * points: Input vector of N-dimensional points.
/// * T: Input 3x3 transformation matrix such that ![inline formula](https://latex.codecogs.com/png.latex?x%20%3D%20T%2AX), where ![inline formula](https://latex.codecogs.com/png.latex?X) are the points to transform and ![inline formula](https://latex.codecogs.com/png.latex?x) the transformed points.
/// * transformed_points: Output vector of N-dimensional transformed points.
// applyTransformationToPoints(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/conditioning.hpp:82
// ("cv::sfm::applyTransformationToPoints", vec![(pred!(mut, ["points", "T", "transformed_points"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn apply_transformation_to_points(points: &impl ToInputArray, t: &impl ToInputArray, transformed_points: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(points);
	input_array_arg!(t);
	output_array_arg!(transformed_points);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_applyTransformationToPoints_const__InputArrayR_const__InputArrayR_const__OutputArrayR(points.as_raw__InputArray(), t.as_raw__InputArray(), transformed_points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes Absolute or Exterior Orientation (Pose Estimation) between 2 sets of 3D point.
/// ## Parameters
/// * x1: Input first 3xN or 2xN array of points.
/// * x2: Input second 3xN or 2xN array of points.
/// * R: Output 3x3 computed rotation matrix.
/// * t: Output 3x1 computed translation vector.
/// * s: Output computed scale factor.
///
/// Find the best transformation such that xp=projection*(s*R*x+t) (same as Pose Estimation, ePNP).
/// The routines below are only for the orthographic case for now.
// computeOrientation(InputArrayOfArrays, InputArrayOfArrays, OutputArray, OutputArray, double)(InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/fundamental.hpp:212
// ("cv::sfm::computeOrientation", vec![(pred!(mut, ["x1", "x2", "R", "t", "s"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
#[inline]
pub fn compute_orientation(x1: &impl ToInputArray, x2: &impl ToInputArray, r: &mut impl ToOutputArray, t: &mut impl ToOutputArray, s: f64) -> Result<()> {
	input_array_arg!(x1);
	input_array_arg!(x2);
	output_array_arg!(r);
	output_array_arg!(t);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_computeOrientation_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double(x1.as_raw__InputArray(), x2.as_raw__InputArray(), r.as_raw__OutputArray(), t.as_raw__OutputArray(), s, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Returns the depth of a point transformed by a rigid transform.
/// ## Parameters
/// * R: Input 3x3 rotation matrix.
/// * t: Input 3x1 translation vector.
/// * X: Input 3x1 or 4x1 vector with the 3d point.
// depth(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/projection.hpp:97
// ("cv::sfm::depth", vec![(pred!(mut, ["R", "t", "X"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn depth(r: &impl ToInputArray, t: &impl ToInputArray, x: &impl ToInputArray) -> Result<f64> {
	input_array_arg!(r);
	input_array_arg!(t);
	input_array_arg!(x);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_depth_const__InputArrayR_const__InputArrayR_const__InputArrayR(r.as_raw__InputArray(), t.as_raw__InputArray(), x.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Get Essential matrix from Fundamental and Camera matrices.
/// ## Parameters
/// * F: Input 3x3 fundamental matrix.
/// * K1: Input 3x3 first camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D).
/// * K2: Input 3x3 second camera matrix. The parameters are similar to K1.
/// * E: Output 3x3 essential matrix.
///
/// Reference: [HartleyZ00](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_HartleyZ00) 9.6 pag 257 (formula 9.12)
// essentialFromFundamental(InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/fundamental.hpp:167
// ("cv::sfm::essentialFromFundamental", vec![(pred!(mut, ["F", "K1", "K2", "E"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn essential_from_fundamental(f: &impl ToInputArray, k1: &impl ToInputArray, k2: &impl ToInputArray, e: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(f);
	input_array_arg!(k1);
	input_array_arg!(k2);
	output_array_arg!(e);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_essentialFromFundamental_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(f.as_raw__InputArray(), k1.as_raw__InputArray(), k2.as_raw__InputArray(), e.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Get Essential matrix from Motion (R's and t's ).
/// ## Parameters
/// * R1: Input 3x3 first camera rotation matrix.
/// * t1: Input 3x1 first camera translation vector.
/// * R2: Input 3x3 second camera rotation matrix.
/// * t2: Input 3x1 second camera translation vector.
/// * E: Output 3x3 essential matrix.
///
/// Reference: [HartleyZ00](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_HartleyZ00) 9.6 pag 257 (formula 9.12)
// essentialFromRt(InputArray, InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/fundamental.hpp:183
// ("cv::sfm::essentialFromRt", vec![(pred!(mut, ["R1", "t1", "R2", "t2", "E"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn essential_from_rt(r1: &impl ToInputArray, t1: &impl ToInputArray, r2: &impl ToInputArray, t2: &impl ToInputArray, e: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(r1);
	input_array_arg!(t1);
	input_array_arg!(r2);
	input_array_arg!(t2);
	output_array_arg!(e);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_essentialFromRt_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(r1.as_raw__InputArray(), t1.as_raw__InputArray(), r2.as_raw__InputArray(), t2.as_raw__InputArray(), e.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Converts points from Euclidean to homogeneous space. E.g., ((x,y)->(x,y,1))
/// ## Parameters
/// * src: Input vector of N-dimensional points.
/// * dst: Output vector of N+1-dimensional points.
// euclideanToHomogeneous(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/projection.hpp:63
// ("cv::sfm::euclideanToHomogeneous", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn euclidean_to_homogeneous(src: &impl ToInputArray, dst: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_euclideanToHomogeneous_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Estimate robustly the fundamental matrix between two dataset of 2D point (image coords space).
/// ## Parameters
/// * x1: Input 2xN Array of 2D points in view 1.
/// * x2: Input 2xN Array of 2D points in view 2.
/// * max_error: maximum error (in pixels).
/// * F: Output 3x3 fundamental matrix such that ![inline formula](https://latex.codecogs.com/png.latex?x%5F2%5ET%20F%20x%5F1%3D0).
/// * inliers: Output 1xN vector that contains the indexes of the detected inliers.
/// * outliers_probability: outliers probability (in ]0,1[).
///          The number of iterations is controlled using the following equation:
///          ![inline formula](https://latex.codecogs.com/png.latex?k%20%3D%20%5Cfrac%7Blog%281%2Dp%29%7D%7Blog%281%2E0%20%2D%20w%5En%20%29%7D) where ![inline formula](https://latex.codecogs.com/png.latex?k), ![inline formula](https://latex.codecogs.com/png.latex?w) and ![inline formula](https://latex.codecogs.com/png.latex?n) are the number of
///          iterations, the inliers ratio and minimun number of selected independent samples.
///          The more this value is high, the less the function selects ramdom samples.
///
/// The fundamental solver relies on the 7 point solution. Returns the best error (in pixels), associated to the solution F.
///
/// ## Note
/// This alternative version of [fundamental_from_correspondences7_point_robust] function uses the following default values for its arguments:
/// * outliers_probability: 1e-2
// cv::sfm::fundamentalFromCorrespondences7PointRobust(InputArray, InputArray, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/robust.hpp:90
// ("cv::sfm::fundamentalFromCorrespondences7PointRobust", vec![(pred!(mut, ["x1", "x2", "max_error", "F", "inliers"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn fundamental_from_correspondences7_point_robust_def(x1: &impl ToInputArray, x2: &impl ToInputArray, max_error: f64, f: &mut impl ToOutputArray, inliers: &mut impl ToOutputArray) -> Result<f64> {
	input_array_arg!(x1);
	input_array_arg!(x2);
	output_array_arg!(f);
	output_array_arg!(inliers);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_fundamentalFromCorrespondences7PointRobust_const__InputArrayR_const__InputArrayR_double_const__OutputArrayR_const__OutputArrayR(x1.as_raw__InputArray(), x2.as_raw__InputArray(), max_error, f.as_raw__OutputArray(), inliers.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Estimate robustly the fundamental matrix between two dataset of 2D point (image coords space).
/// ## Parameters
/// * x1: Input 2xN Array of 2D points in view 1.
/// * x2: Input 2xN Array of 2D points in view 2.
/// * max_error: maximum error (in pixels).
/// * F: Output 3x3 fundamental matrix such that ![inline formula](https://latex.codecogs.com/png.latex?x%5F2%5ET%20F%20x%5F1%3D0).
/// * inliers: Output 1xN vector that contains the indexes of the detected inliers.
/// * outliers_probability: outliers probability (in ]0,1[).
///          The number of iterations is controlled using the following equation:
///          ![inline formula](https://latex.codecogs.com/png.latex?k%20%3D%20%5Cfrac%7Blog%281%2Dp%29%7D%7Blog%281%2E0%20%2D%20w%5En%20%29%7D) where ![inline formula](https://latex.codecogs.com/png.latex?k), ![inline formula](https://latex.codecogs.com/png.latex?w) and ![inline formula](https://latex.codecogs.com/png.latex?n) are the number of
///          iterations, the inliers ratio and minimun number of selected independent samples.
///          The more this value is high, the less the function selects ramdom samples.
///
/// The fundamental solver relies on the 7 point solution. Returns the best error (in pixels), associated to the solution F.
///
/// ## C++ default parameters
/// * outliers_probability: 1e-2
// fundamentalFromCorrespondences7PointRobust(InputArray, InputArray, double, OutputArray, OutputArray, double)(InputArray, InputArray, Primitive, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/robust.hpp:90
// ("cv::sfm::fundamentalFromCorrespondences7PointRobust", vec![(pred!(mut, ["x1", "x2", "max_error", "F", "inliers", "outliers_probability"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
#[inline]
pub fn fundamental_from_correspondences7_point_robust(x1: &impl ToInputArray, x2: &impl ToInputArray, max_error: f64, f: &mut impl ToOutputArray, inliers: &mut impl ToOutputArray, outliers_probability: f64) -> Result<f64> {
	input_array_arg!(x1);
	input_array_arg!(x2);
	output_array_arg!(f);
	output_array_arg!(inliers);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_fundamentalFromCorrespondences7PointRobust_const__InputArrayR_const__InputArrayR_double_const__OutputArrayR_const__OutputArrayR_double(x1.as_raw__InputArray(), x2.as_raw__InputArray(), max_error, f.as_raw__OutputArray(), inliers.as_raw__OutputArray(), outliers_probability, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Estimate robustly the fundamental matrix between two dataset of 2D point (image coords space).
/// ## Parameters
/// * x1: Input 2xN Array of 2D points in view 1.
/// * x2: Input 2xN Array of 2D points in view 2.
/// * max_error: maximum error (in pixels).
/// * F: Output 3x3 fundamental matrix such that ![inline formula](https://latex.codecogs.com/png.latex?x%5F2%5ET%20F%20x%5F1%3D0).
/// * inliers: Output 1xN vector that contains the indexes of the detected inliers.
/// * outliers_probability: outliers probability (in ]0,1[).
///          The number of iterations is controlled using the following equation:
///          ![inline formula](https://latex.codecogs.com/png.latex?k%20%3D%20%5Cfrac%7Blog%281%2Dp%29%7D%7Blog%281%2E0%20%2D%20w%5En%20%29%7D) where ![inline formula](https://latex.codecogs.com/png.latex?k), ![inline formula](https://latex.codecogs.com/png.latex?w) and ![inline formula](https://latex.codecogs.com/png.latex?n) are the number of
///          iterations, the inliers ratio and minimun number of selected independent samples.
///          The more this value is high, the less the function selects ramdom samples.
///
/// The fundamental solver relies on the 8 point solution. Returns the best error (in pixels), associated to the solution F.
///
/// ## Note
/// This alternative version of [fundamental_from_correspondences8_point_robust] function uses the following default values for its arguments:
/// * outliers_probability: 1e-2
// cv::sfm::fundamentalFromCorrespondences8PointRobust(InputArray, InputArray, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/robust.hpp:67
// ("cv::sfm::fundamentalFromCorrespondences8PointRobust", vec![(pred!(mut, ["x1", "x2", "max_error", "F", "inliers"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn fundamental_from_correspondences8_point_robust_def(x1: &impl ToInputArray, x2: &impl ToInputArray, max_error: f64, f: &mut impl ToOutputArray, inliers: &mut impl ToOutputArray) -> Result<f64> {
	input_array_arg!(x1);
	input_array_arg!(x2);
	output_array_arg!(f);
	output_array_arg!(inliers);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_fundamentalFromCorrespondences8PointRobust_const__InputArrayR_const__InputArrayR_double_const__OutputArrayR_const__OutputArrayR(x1.as_raw__InputArray(), x2.as_raw__InputArray(), max_error, f.as_raw__OutputArray(), inliers.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Estimate robustly the fundamental matrix between two dataset of 2D point (image coords space).
/// ## Parameters
/// * x1: Input 2xN Array of 2D points in view 1.
/// * x2: Input 2xN Array of 2D points in view 2.
/// * max_error: maximum error (in pixels).
/// * F: Output 3x3 fundamental matrix such that ![inline formula](https://latex.codecogs.com/png.latex?x%5F2%5ET%20F%20x%5F1%3D0).
/// * inliers: Output 1xN vector that contains the indexes of the detected inliers.
/// * outliers_probability: outliers probability (in ]0,1[).
///          The number of iterations is controlled using the following equation:
///          ![inline formula](https://latex.codecogs.com/png.latex?k%20%3D%20%5Cfrac%7Blog%281%2Dp%29%7D%7Blog%281%2E0%20%2D%20w%5En%20%29%7D) where ![inline formula](https://latex.codecogs.com/png.latex?k), ![inline formula](https://latex.codecogs.com/png.latex?w) and ![inline formula](https://latex.codecogs.com/png.latex?n) are the number of
///          iterations, the inliers ratio and minimun number of selected independent samples.
///          The more this value is high, the less the function selects ramdom samples.
///
/// The fundamental solver relies on the 8 point solution. Returns the best error (in pixels), associated to the solution F.
///
/// ## C++ default parameters
/// * outliers_probability: 1e-2
// fundamentalFromCorrespondences8PointRobust(InputArray, InputArray, double, OutputArray, OutputArray, double)(InputArray, InputArray, Primitive, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/robust.hpp:67
// ("cv::sfm::fundamentalFromCorrespondences8PointRobust", vec![(pred!(mut, ["x1", "x2", "max_error", "F", "inliers", "outliers_probability"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_OutputArray*", "const cv::_OutputArray*", "double"]), _)]),
#[inline]
pub fn fundamental_from_correspondences8_point_robust(x1: &impl ToInputArray, x2: &impl ToInputArray, max_error: f64, f: &mut impl ToOutputArray, inliers: &mut impl ToOutputArray, outliers_probability: f64) -> Result<f64> {
	input_array_arg!(x1);
	input_array_arg!(x2);
	output_array_arg!(f);
	output_array_arg!(inliers);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_fundamentalFromCorrespondences8PointRobust_const__InputArrayR_const__InputArrayR_double_const__OutputArrayR_const__OutputArrayR_double(x1.as_raw__InputArray(), x2.as_raw__InputArray(), max_error, f.as_raw__OutputArray(), inliers.as_raw__OutputArray(), outliers_probability, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Get Essential matrix from Fundamental and Camera matrices.
/// ## Parameters
/// * E: Input 3x3 essential matrix.
/// * K1: Input 3x3 first camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D).
/// * K2: Input 3x3 second camera matrix. The parameters are similar to K1.
/// * F: Output 3x3 fundamental matrix.
///
/// Reference: [HartleyZ00](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_HartleyZ00) 9.6 pag 257 (formula 9.12) or <http://ai.stanford.edu/~birch/projective/node20.html>
// fundamentalFromEssential(InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/fundamental.hpp:152
// ("cv::sfm::fundamentalFromEssential", vec![(pred!(mut, ["E", "K1", "K2", "F"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn fundamental_from_essential(e: &impl ToInputArray, k1: &impl ToInputArray, k2: &impl ToInputArray, f: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(e);
	input_array_arg!(k1);
	input_array_arg!(k2);
	output_array_arg!(f);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_fundamentalFromEssential_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(e.as_raw__InputArray(), k1.as_raw__InputArray(), k2.as_raw__InputArray(), f.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Get Fundamental matrix from Projection matrices.
/// ## Parameters
/// * P1: Input 3x4 first projection matrix.
/// * P2: Input 3x4 second projection matrix.
/// * F: Output 3x3 fundamental matrix.
// fundamentalFromProjections(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/fundamental.hpp:69
// ("cv::sfm::fundamentalFromProjections", vec![(pred!(mut, ["P1", "P2", "F"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn fundamental_from_projections(p1: &impl ToInputArray, p2: &impl ToInputArray, f: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(p1);
	input_array_arg!(p2);
	output_array_arg!(f);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_fundamentalFromProjections_const__InputArrayR_const__InputArrayR_const__OutputArrayR(p1.as_raw__InputArray(), p2.as_raw__InputArray(), f.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Converts point coordinates from homogeneous to euclidean pixel coordinates. E.g., ((x,y,z)->(x/z, y/z))
/// ## Parameters
/// * src: Input vector of N-dimensional points.
/// * dst: Output vector of N-1-dimensional points.
// homogeneousToEuclidean(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/projection.hpp:55
// ("cv::sfm::homogeneousToEuclidean", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn homogeneous_to_euclidean(src: &impl ToInputArray, dst: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(src);
	output_array_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_homogeneousToEuclidean_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Import a reconstruction file.
/// ## Parameters
/// * file: The path to the file.
/// * Rs: Output vector of 3x3 rotations of the camera
/// * Ts: Output vector of 3x1 translations of the camera.
/// * Ks: Output vector of 3x3 instrinsics of the camera.
/// * points3d: Output array with 3d points. Is 3 x N.
/// * file_format: The format of the file to import.
///
/// The function supports reconstructions from Bundler.
///
/// ## Note
/// This alternative version of [import_reconstruction] function uses the following default values for its arguments:
/// * file_format: SFM_IO_BUNDLER
// cv::sfm::importReconstruction(InString, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/io.hpp:77
// ("cv::sfm::importReconstruction", vec![(pred!(mut, ["file", "Rs", "Ts", "Ks", "points3d"], ["const cv::String*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn import_reconstruction_def(file: &str, rs: &mut impl ToOutputArray, ts: &mut impl ToOutputArray, ks: &mut impl ToOutputArray, points3d: &mut impl ToOutputArray) -> Result<()> {
	extern_container_arg!(file);
	output_array_arg!(rs);
	output_array_arg!(ts);
	output_array_arg!(ks);
	output_array_arg!(points3d);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_importReconstruction_const_StringR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(file.opencv_as_extern(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray(), ks.as_raw__OutputArray(), points3d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Import a reconstruction file.
/// ## Parameters
/// * file: The path to the file.
/// * Rs: Output vector of 3x3 rotations of the camera
/// * Ts: Output vector of 3x1 translations of the camera.
/// * Ks: Output vector of 3x3 instrinsics of the camera.
/// * points3d: Output array with 3d points. Is 3 x N.
/// * file_format: The format of the file to import.
///
/// The function supports reconstructions from Bundler.
///
/// ## C++ default parameters
/// * file_format: SFM_IO_BUNDLER
// importReconstruction(const cv::String &, OutputArrayOfArrays, OutputArrayOfArrays, OutputArrayOfArrays, OutputArrayOfArrays, int)(InString, OutputArray, OutputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/io.hpp:77
// ("cv::sfm::importReconstruction", vec![(pred!(mut, ["file", "Rs", "Ts", "Ks", "points3d", "file_format"], ["const cv::String*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
#[inline]
pub fn import_reconstruction(file: &str, rs: &mut impl ToOutputArray, ts: &mut impl ToOutputArray, ks: &mut impl ToOutputArray, points3d: &mut impl ToOutputArray, file_format: i32) -> Result<()> {
	extern_container_arg!(file);
	output_array_arg!(rs);
	output_array_arg!(ts);
	output_array_arg!(ks);
	output_array_arg!(points3d);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_importReconstruction_const_StringR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int(file.opencv_as_extern(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray(), ks.as_raw__OutputArray(), points3d.as_raw__OutputArray(), file_format, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Point conditioning (isotropic).
/// ## Parameters
/// * points: Input vector of N-dimensional points.
/// * T: Output 3x3 transformation matrix.
///
/// Computes the transformation matrix such that each coordinate direction will be scaled equally,
/// bringing the centroid to the origin with an average centroid ![inline formula](https://latex.codecogs.com/png.latex?%281%2C1%2C1%29%5ET).
///
/// Reference: [HartleyZ00](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_HartleyZ00) 4.4.4 pag.107.
// isotropicPreconditionerFromPoints(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/conditioning.hpp:72
// ("cv::sfm::isotropicPreconditionerFromPoints", vec![(pred!(mut, ["points", "T"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn isotropic_preconditioner_from_points(points: &impl ToInputArray, t: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(points);
	output_array_arg!(t);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_isotropicPreconditionerFromPoints_const__InputArrayR_const__OutputArrayR(points.as_raw__InputArray(), t.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes the mean and variance of a given matrix along its rows.
/// ## Parameters
/// * A: Input NxN matrix.
/// * mean: Output Nx1 matrix with computed mean.
/// * variance: Output Nx1 matrix with computed variance.
///
/// It computes in the same way as woud do [reduce] but with \a Variance function.
// meanAndVarianceAlongRows(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/numeric.hpp:58
// ("cv::sfm::meanAndVarianceAlongRows", vec![(pred!(mut, ["A", "mean", "variance"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn mean_and_variance_along_rows(a: &impl ToInputArray, mean: &mut impl ToOutputArray, variance: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(a);
	output_array_arg!(mean);
	output_array_arg!(variance);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_meanAndVarianceAlongRows_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(a.as_raw__InputArray(), mean.as_raw__OutputArray(), variance.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Choose one of the four possible motion solutions from an essential matrix.
/// ## Parameters
/// * Rs: Input vector of 3x3 rotation matrices.
/// * ts: Input vector of 3x1 translation vectors.
/// * K1: Input 3x3 first camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D).
/// * x1: Input 2x1 vector with first 2d point.
/// * K2: Input 3x3 second camera matrix. The parameters are similar to K1.
/// * x2: Input 2x1 vector with second 2d point.
///
/// Decides the right solution by checking that the triangulation of a match
/// x1--x2 lies in front of the cameras. Return index of the right solution or -1 if no solution.
///
/// Reference: See [HartleyZ00](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_HartleyZ00) 9.6 pag 259 (9.6.3 Geometrical interpretation of the 4 solutions).
// motionFromEssentialChooseSolution(InputArrayOfArrays, InputArrayOfArrays, InputArray, InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/fundamental.hpp:135
// ("cv::sfm::motionFromEssentialChooseSolution", vec![(pred!(mut, ["Rs", "ts", "K1", "x1", "K2", "x2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn motion_from_essential_choose_solution(rs: &impl ToInputArray, ts: &impl ToInputArray, k1: &impl ToInputArray, x1: &impl ToInputArray, k2: &impl ToInputArray, x2: &impl ToInputArray) -> Result<i32> {
	input_array_arg!(rs);
	input_array_arg!(ts);
	input_array_arg!(k1);
	input_array_arg!(x1);
	input_array_arg!(k2);
	input_array_arg!(x2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_motionFromEssentialChooseSolution_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(rs.as_raw__InputArray(), ts.as_raw__InputArray(), k1.as_raw__InputArray(), x1.as_raw__InputArray(), k2.as_raw__InputArray(), x2.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Get Motion (R's and t's ) from Essential matrix.
/// ## Parameters
/// * E: Input 3x3 essential matrix.
/// * Rs: Output vector of 3x3 rotation matrices.
/// * ts: Output vector of 3x1 translation vectors.
///
/// Reference: [HartleyZ00](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_HartleyZ00) 9.6 pag 259 (Result 9.19)
// motionFromEssential(InputArray, OutputArrayOfArrays, OutputArrayOfArrays)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/fundamental.hpp:117
// ("cv::sfm::motionFromEssential", vec![(pred!(mut, ["E", "Rs", "ts"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn motion_from_essential(e: &impl ToInputArray, rs: &mut impl ToOutputArray, ts: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(e);
	output_array_arg!(rs);
	output_array_arg!(ts);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_motionFromEssential_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(e.as_raw__InputArray(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Normalizes the Fundamental matrix.
/// ## Parameters
/// * F: Input 3x3 fundamental matrix.
/// * F_normalized: Output 3x3 normalized fundamental matrix.
///
/// By default divides the fundamental matrix by its L2 norm.
// normalizeFundamental(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/fundamental.hpp:197
// ("cv::sfm::normalizeFundamental", vec![(pred!(mut, ["F", "F_normalized"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn normalize_fundamental(f: &impl ToInputArray, f_normalized: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(f);
	output_array_arg!(f_normalized);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_normalizeFundamental_const__InputArrayR_const__OutputArrayR(f.as_raw__InputArray(), f_normalized.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// This function normalizes points. (isotropic).
/// ## Parameters
/// * points: Input vector of N-dimensional points.
/// * normalized_points: Output vector of the same N-dimensional points but with mean 0 and average norm ![inline formula](https://latex.codecogs.com/png.latex?%5Csqrt%7B2%7D).
/// * T: Output 3x3 transform matrix such that ![inline formula](https://latex.codecogs.com/png.latex?x%20%3D%20T%2AX), where ![inline formula](https://latex.codecogs.com/png.latex?X) are the points to normalize and ![inline formula](https://latex.codecogs.com/png.latex?x) the normalized points.
///
/// Internally calls [preconditionerFromPoints] in order to get the scaling matrix before applying [applyTransformationToPoints].
/// This operation is an essential step before applying the DLT algorithm in order to consider the result as optimal.
///
/// Reference: [HartleyZ00](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_HartleyZ00) 4.4.4 pag.107.
// normalizeIsotropicPoints(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/conditioning.hpp:112
// ("cv::sfm::normalizeIsotropicPoints", vec![(pred!(mut, ["points", "normalized_points", "T"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn normalize_isotropic_points(points: &impl ToInputArray, normalized_points: &mut impl ToOutputArray, t: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(points);
	output_array_arg!(normalized_points);
	output_array_arg!(t);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_normalizeIsotropicPoints_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(points.as_raw__InputArray(), normalized_points.as_raw__OutputArray(), t.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// This function normalizes points (non isotropic).
/// ## Parameters
/// * points: Input vector of N-dimensional points.
/// * normalized_points: Output vector of the same N-dimensional points but with mean 0 and average norm ![inline formula](https://latex.codecogs.com/png.latex?%5Csqrt%7B2%7D).
/// * T: Output 3x3 transform matrix such that ![inline formula](https://latex.codecogs.com/png.latex?x%20%3D%20T%2AX), where ![inline formula](https://latex.codecogs.com/png.latex?X) are the points to normalize and ![inline formula](https://latex.codecogs.com/png.latex?x) the normalized points.
///
/// Internally calls [preconditionerFromPoints] in order to get the scaling matrix before applying [applyTransformationToPoints].
/// This operation is an essential step before applying the DLT algorithm in order to consider the result as optimal.
///
/// Reference: [HartleyZ00](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_HartleyZ00) 4.4.4 pag.109
// normalizePoints(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/conditioning.hpp:97
// ("cv::sfm::normalizePoints", vec![(pred!(mut, ["points", "normalized_points", "T"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn normalize_points(points: &impl ToInputArray, normalized_points: &mut impl ToOutputArray, t: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(points);
	output_array_arg!(normalized_points);
	output_array_arg!(t);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_normalizePoints_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(points.as_raw__InputArray(), normalized_points.as_raw__OutputArray(), t.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Estimate the fundamental matrix between two dataset of 2D point (image coords space).
/// ## Parameters
/// * x1: Input 2xN Array of 2D points in view 1.
/// * x2: Input 2xN Array of 2D points in view 2.
/// * F: Output 3x3 fundamental matrix.
///
/// Uses the normalized 8-point fundamental matrix solver.
/// Reference: [HartleyZ00](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_HartleyZ00) 11.2 pag.281 (x1 = x, x2 = x')
// normalizedEightPointSolver(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/fundamental.hpp:83
// ("cv::sfm::normalizedEightPointSolver", vec![(pred!(mut, ["x1", "x2", "F"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn normalized_eight_point_solver(x1: &impl ToInputArray, x2: &impl ToInputArray, f: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(x1);
	input_array_arg!(x2);
	output_array_arg!(f);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_normalizedEightPointSolver_const__InputArrayR_const__InputArrayR_const__OutputArrayR(x1.as_raw__InputArray(), x2.as_raw__InputArray(), f.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Point conditioning (non isotropic).
/// ## Parameters
/// * points: Input vector of N-dimensional points.
/// * T: Output 3x3 transformation matrix.
///
/// Computes the transformation matrix such that the two principal moments of the set of points are equal to unity,
/// forming an approximately symmetric circular cloud of points of radius 1 about the origin.
///
/// Reference: [HartleyZ00](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_HartleyZ00) 4.4.4 pag.109
// preconditionerFromPoints(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/conditioning.hpp:59
// ("cv::sfm::preconditionerFromPoints", vec![(pred!(mut, ["points", "T"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn preconditioner_from_points(points: &impl ToInputArray, t: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(points);
	output_array_arg!(t);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_preconditionerFromPoints_const__InputArrayR_const__OutputArrayR(points.as_raw__InputArray(), t.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Get projection matrix P from K, R and t.
/// ## Parameters
/// * K: Input 3x3 camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D).
/// * R: Input 3x3 rotation matrix.
/// * t: Input 3x1 translation vector.
/// * P: Output 3x4 projection matrix.
///
/// This function estimate the projection matrix by solving the following equation: ![inline formula](https://latex.codecogs.com/png.latex?P%20%3D%20K%20%2A%20%5BR%7Ct%5D)
// projectionFromKRt(InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/projection.hpp:76
// ("cv::sfm::projectionFromKRt", vec![(pred!(mut, ["K", "R", "t", "P"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn projection_from_k_rt(k: &impl ToInputArray, r: &impl ToInputArray, t: &impl ToInputArray, p: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(k);
	input_array_arg!(r);
	input_array_arg!(t);
	output_array_arg!(p);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_projectionFromKRt_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(k.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), p.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Get projection matrices from Fundamental matrix
/// ## Parameters
/// * F: Input 3x3 fundamental matrix.
/// * P1: Output 3x4 one possible projection matrix.
/// * P2: Output 3x4 another possible projection matrix.
// projectionsFromFundamental(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/fundamental.hpp:58
// ("cv::sfm::projectionsFromFundamental", vec![(pred!(mut, ["F", "P1", "P2"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn projections_from_fundamental(f: &impl ToInputArray, p1: &mut impl ToOutputArray, p2: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(f);
	output_array_arg!(p1);
	output_array_arg!(p2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_projectionsFromFundamental_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(f.as_raw__InputArray(), p1.as_raw__OutputArray(), p2.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Reconstruct 3d points from 2d correspondences while performing autocalibration.
/// ## Parameters
/// * points2d: Input vector of vectors of 2d points (the inner vector is per image).
/// * Ps: Output vector with the 3x4 projections matrices of each image.
/// * points3d: Output array with estimated 3d points.
/// * K: Input/Output camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D). Input parameters used as initial guess.
/// * is_projective: if true, the cameras are supposed to be projective.
///
/// This method calls below signature and extracts projection matrices from estimated K, R and t.
///
///  
/// Note:
///   - Tracks must be as precise as possible. It does not handle outliers and is very sensible to them.
///
/// ## Note
/// This alternative version of [reconstruct] function uses the following default values for its arguments:
/// * is_projective: false
// cv::sfm::reconstruct(InputArray, OutputArray, OutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/reconstruct.hpp:74
// ("cv::sfm::reconstruct", vec![(pred!(mut, ["points2d", "Ps", "points3d", "K"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*"]), _)]),
#[inline]
pub fn reconstruct_def(points2d: &impl ToInputArray, ps: &mut impl ToOutputArray, points3d: &mut impl ToOutputArray, k: &mut impl ToInputOutputArray) -> Result<()> {
	input_array_arg!(points2d);
	output_array_arg!(ps);
	output_array_arg!(points3d);
	input_output_array_arg!(k);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_reconstruct_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR(points2d.as_raw__InputArray(), ps.as_raw__OutputArray(), points3d.as_raw__OutputArray(), k.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Reconstruct 3d points from 2d correspondences while performing autocalibration.
/// ## Parameters
/// * points2d: Input vector of vectors of 2d points (the inner vector is per image).
/// * Ps: Output vector with the 3x4 projections matrices of each image.
/// * points3d: Output array with estimated 3d points.
/// * K: Input/Output camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D). Input parameters used as initial guess.
/// * is_projective: if true, the cameras are supposed to be projective.
///
/// This method calls below signature and extracts projection matrices from estimated K, R and t.
///
///  
/// Note:
///   - Tracks must be as precise as possible. It does not handle outliers and is very sensible to them.
///
/// ## C++ default parameters
/// * is_projective: false
// reconstruct(InputArrayOfArrays, OutputArray, OutputArray, InputOutputArray, bool)(InputArray, OutputArray, OutputArray, InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/reconstruct.hpp:74
// ("cv::sfm::reconstruct", vec![(pred!(mut, ["points2d", "Ps", "points3d", "K", "is_projective"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "bool"]), _)]),
#[inline]
pub fn reconstruct(points2d: &impl ToInputArray, ps: &mut impl ToOutputArray, points3d: &mut impl ToOutputArray, k: &mut impl ToInputOutputArray, is_projective: bool) -> Result<()> {
	input_array_arg!(points2d);
	output_array_arg!(ps);
	output_array_arg!(points3d);
	input_output_array_arg!(k);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_reconstruct_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_bool(points2d.as_raw__InputArray(), ps.as_raw__OutputArray(), points3d.as_raw__OutputArray(), k.as_raw__InputOutputArray(), is_projective, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Reconstruct 3d points from 2d correspondences while performing autocalibration.
/// ## Parameters
/// * points2d: Input vector of vectors of 2d points (the inner vector is per image).
/// * Rs: Output vector of 3x3 rotations of the camera.
/// * Ts: Output vector of 3x1 translations of the camera.
/// * points3d: Output array with estimated 3d points.
/// * K: Input/Output camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D). Input parameters used as initial guess.
/// * is_projective: if true, the cameras are supposed to be projective.
///
/// Internally calls libmv simple pipeline routine with some default parameters by instatiating SFMLibmvEuclideanReconstruction class.
///
///
/// Note:
///   - Tracks must be as precise as possible. It does not handle outliers and is very sensible to them.
///   - To see a working example for camera motion reconstruction, check the following tutorial: [tutorial_sfm_trajectory_estimation].
///
/// ## Note
/// This alternative version of [reconstruct_1] function uses the following default values for its arguments:
/// * is_projective: false
// cv::sfm::reconstruct(InputArray, OutputArray, OutputArray, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/reconstruct.hpp:93
// ("cv::sfm::reconstruct", vec![(pred!(mut, ["points2d", "Rs", "Ts", "K", "points3d"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn reconstruct_1_def(points2d: &impl ToInputArray, rs: &mut impl ToOutputArray, ts: &mut impl ToOutputArray, k: &mut impl ToInputOutputArray, points3d: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(points2d);
	output_array_arg!(rs);
	output_array_arg!(ts);
	input_output_array_arg!(k);
	output_array_arg!(points3d);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_reconstruct_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__OutputArrayR(points2d.as_raw__InputArray(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray(), k.as_raw__InputOutputArray(), points3d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Reconstruct 3d points from 2d correspondences while performing autocalibration.
/// ## Parameters
/// * points2d: Input vector of vectors of 2d points (the inner vector is per image).
/// * Rs: Output vector of 3x3 rotations of the camera.
/// * Ts: Output vector of 3x1 translations of the camera.
/// * points3d: Output array with estimated 3d points.
/// * K: Input/Output camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D). Input parameters used as initial guess.
/// * is_projective: if true, the cameras are supposed to be projective.
///
/// Internally calls libmv simple pipeline routine with some default parameters by instatiating SFMLibmvEuclideanReconstruction class.
///
///
/// Note:
///   - Tracks must be as precise as possible. It does not handle outliers and is very sensible to them.
///   - To see a working example for camera motion reconstruction, check the following tutorial: [tutorial_sfm_trajectory_estimation].
///
/// ## C++ default parameters
/// * is_projective: false
// reconstruct(InputArrayOfArrays, OutputArray, OutputArray, InputOutputArray, OutputArray, bool)(InputArray, OutputArray, OutputArray, InputOutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/reconstruct.hpp:93
// ("cv::sfm::reconstruct", vec![(pred!(mut, ["points2d", "Rs", "Ts", "K", "points3d", "is_projective"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "bool"]), _)]),
#[inline]
pub fn reconstruct_1(points2d: &impl ToInputArray, rs: &mut impl ToOutputArray, ts: &mut impl ToOutputArray, k: &mut impl ToInputOutputArray, points3d: &mut impl ToOutputArray, is_projective: bool) -> Result<()> {
	input_array_arg!(points2d);
	output_array_arg!(rs);
	output_array_arg!(ts);
	input_output_array_arg!(k);
	output_array_arg!(points3d);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_reconstruct_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__OutputArrayR_bool(points2d.as_raw__InputArray(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray(), k.as_raw__InputOutputArray(), points3d.as_raw__OutputArray(), is_projective, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Reconstruct 3d points from 2d images while performing autocalibration.
/// ## Parameters
/// * images: a vector of string with the images paths.
/// * Ps: Output vector with the 3x4 projections matrices of each image.
/// * points3d: Output array with estimated 3d points.
/// * K: Input/Output camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D). Input parameters used as initial guess.
/// * is_projective: if true, the cameras are supposed to be projective.
///
/// This method calls below signature and extracts projection matrices from estimated K, R and t.
///
///  
/// Note:
///   - The images must be ordered as they were an image sequence. Additionally, each frame should be as close as posible to the previous and posterior.
///   - For now DAISY features are used in order to compute the 2d points tracks and it only works for 3-4 images.
///
/// ## Note
/// This alternative version of [reconstruct_2] function uses the following default values for its arguments:
/// * is_projective: false
// cv::sfm::reconstruct(CppPassByVoidPtr, OutputArray, OutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/reconstruct.hpp:111
// ("cv::sfm::reconstruct", vec![(pred!(mut, ["images", "Ps", "points3d", "K"], ["const std::vector<cv::String>", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*"]), _)]),
#[inline]
pub fn reconstruct_2_def(images: core::Vector<String>, ps: &mut impl ToOutputArray, points3d: &mut impl ToOutputArray, k: &mut impl ToInputOutputArray) -> Result<()> {
	output_array_arg!(ps);
	output_array_arg!(points3d);
	input_output_array_arg!(k);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_reconstruct_const_vectorLStringG_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR(images.as_raw_VectorOfString(), ps.as_raw__OutputArray(), points3d.as_raw__OutputArray(), k.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Reconstruct 3d points from 2d images while performing autocalibration.
/// ## Parameters
/// * images: a vector of string with the images paths.
/// * Ps: Output vector with the 3x4 projections matrices of each image.
/// * points3d: Output array with estimated 3d points.
/// * K: Input/Output camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D). Input parameters used as initial guess.
/// * is_projective: if true, the cameras are supposed to be projective.
///
/// This method calls below signature and extracts projection matrices from estimated K, R and t.
///
///  
/// Note:
///   - The images must be ordered as they were an image sequence. Additionally, each frame should be as close as posible to the previous and posterior.
///   - For now DAISY features are used in order to compute the 2d points tracks and it only works for 3-4 images.
///
/// ## C++ default parameters
/// * is_projective: false
// reconstruct(const std::vector<String>, OutputArray, OutputArray, InputOutputArray, bool)(CppPassByVoidPtr, OutputArray, OutputArray, InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/reconstruct.hpp:111
// ("cv::sfm::reconstruct", vec![(pred!(mut, ["images", "Ps", "points3d", "K", "is_projective"], ["const std::vector<cv::String>", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "bool"]), _)]),
#[inline]
pub fn reconstruct_2(images: core::Vector<String>, ps: &mut impl ToOutputArray, points3d: &mut impl ToOutputArray, k: &mut impl ToInputOutputArray, is_projective: bool) -> Result<()> {
	output_array_arg!(ps);
	output_array_arg!(points3d);
	input_output_array_arg!(k);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_reconstruct_const_vectorLStringG_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_bool(images.as_raw_VectorOfString(), ps.as_raw__OutputArray(), points3d.as_raw__OutputArray(), k.as_raw__InputOutputArray(), is_projective, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Reconstruct 3d points from 2d images while performing autocalibration.
/// ## Parameters
/// * images: a vector of string with the images paths.
/// * Rs: Output vector of 3x3 rotations of the camera.
/// * Ts: Output vector of 3x1 translations of the camera.
/// * points3d: Output array with estimated 3d points.
/// * K: Input/Output camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D). Input parameters used as initial guess.
/// * is_projective: if true, the cameras are supposed to be projective.
///
/// Internally calls libmv simple pipeline routine with some default parameters by instatiating SFMLibmvEuclideanReconstruction class.
///
///  
/// Note:
///   - The images must be ordered as they were an image sequence. Additionally, each frame should be as close as posible to the previous and posterior.
///   - For now DAISY features are used in order to compute the 2d points tracks and it only works for 3-4 images.
///   - To see a working example for scene reconstruction, check the following tutorial: [tutorial_sfm_scene_reconstruction].
///
/// ## Note
/// This alternative version of [reconstruct_3] function uses the following default values for its arguments:
/// * is_projective: false
// cv::sfm::reconstruct(CppPassByVoidPtr, OutputArray, OutputArray, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/reconstruct.hpp:131
// ("cv::sfm::reconstruct", vec![(pred!(mut, ["images", "Rs", "Ts", "K", "points3d"], ["const std::vector<cv::String>", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn reconstruct_3_def(images: core::Vector<String>, rs: &mut impl ToOutputArray, ts: &mut impl ToOutputArray, k: &mut impl ToInputOutputArray, points3d: &mut impl ToOutputArray) -> Result<()> {
	output_array_arg!(rs);
	output_array_arg!(ts);
	input_output_array_arg!(k);
	output_array_arg!(points3d);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_reconstruct_const_vectorLStringG_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__OutputArrayR(images.as_raw_VectorOfString(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray(), k.as_raw__InputOutputArray(), points3d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Reconstruct 3d points from 2d images while performing autocalibration.
/// ## Parameters
/// * images: a vector of string with the images paths.
/// * Rs: Output vector of 3x3 rotations of the camera.
/// * Ts: Output vector of 3x1 translations of the camera.
/// * points3d: Output array with estimated 3d points.
/// * K: Input/Output camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D). Input parameters used as initial guess.
/// * is_projective: if true, the cameras are supposed to be projective.
///
/// Internally calls libmv simple pipeline routine with some default parameters by instatiating SFMLibmvEuclideanReconstruction class.
///
///  
/// Note:
///   - The images must be ordered as they were an image sequence. Additionally, each frame should be as close as posible to the previous and posterior.
///   - For now DAISY features are used in order to compute the 2d points tracks and it only works for 3-4 images.
///   - To see a working example for scene reconstruction, check the following tutorial: [tutorial_sfm_scene_reconstruction].
///
/// ## C++ default parameters
/// * is_projective: false
// reconstruct(const std::vector<String>, OutputArray, OutputArray, InputOutputArray, OutputArray, bool)(CppPassByVoidPtr, OutputArray, OutputArray, InputOutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/reconstruct.hpp:131
// ("cv::sfm::reconstruct", vec![(pred!(mut, ["images", "Rs", "Ts", "K", "points3d", "is_projective"], ["const std::vector<cv::String>", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "bool"]), _)]),
#[inline]
pub fn reconstruct_3(images: core::Vector<String>, rs: &mut impl ToOutputArray, ts: &mut impl ToOutputArray, k: &mut impl ToInputOutputArray, points3d: &mut impl ToOutputArray, is_projective: bool) -> Result<()> {
	output_array_arg!(rs);
	output_array_arg!(ts);
	input_output_array_arg!(k);
	output_array_arg!(points3d);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_reconstruct_const_vectorLStringG_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__OutputArrayR_bool(images.as_raw_VectorOfString(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray(), k.as_raw__InputOutputArray(), points3d.as_raw__OutputArray(), is_projective, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes the relative camera motion between two cameras.
/// ## Parameters
/// * R1: Input 3x3 first camera rotation matrix.
/// * t1: Input 3x1 first camera translation vector.
/// * R2: Input 3x3 second camera rotation matrix.
/// * t2: Input 3x1 second camera translation vector.
/// * R: Output 3x3 relative rotation matrix.
/// * t: Output 3x1 relative translation vector.
///
/// Given the motion parameters of two cameras, computes the motion parameters
/// of the second one assuming the first one to be at the origin.
/// If T1 and T2 are the camera motions, the computed relative motion is ![inline formula](https://latex.codecogs.com/png.latex?T%20%3D%20T%5F2%20T%5F1%5E%7B%2D1%7D)
// relativeCameraMotion(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/fundamental.hpp:101
// ("cv::sfm::relativeCameraMotion", vec![(pred!(mut, ["R1", "t1", "R2", "t2", "R", "t"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn relative_camera_motion(r1: &impl ToInputArray, t1: &impl ToInputArray, r2: &impl ToInputArray, t2: &impl ToInputArray, r: &mut impl ToOutputArray, t: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(r1);
	input_array_arg!(t1);
	input_array_arg!(r2);
	input_array_arg!(t2);
	output_array_arg!(r);
	output_array_arg!(t);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_relativeCameraMotion_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(r1.as_raw__InputArray(), t1.as_raw__InputArray(), r2.as_raw__InputArray(), t2.as_raw__InputArray(), r.as_raw__OutputArray(), t.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Returns the 3x3 skew symmetric matrix of a vector.
/// ## Parameters
/// * x: Input 3x1 vector.
///
/// Reference: [HartleyZ00](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_HartleyZ00), p581, equation (A4.5).
// skew(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/numeric.hpp:69
// ("cv::sfm::skew", vec![(pred!(mut, ["x"], ["const cv::_InputArray*"]), _)]),
#[inline]
pub fn skew(x: &impl ToInputArray) -> Result<core::Mat> {
	input_array_arg!(x);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_skew_const__InputArrayR(x.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Reconstructs bunch of points by triangulation.
/// ## Parameters
/// * points2d: Input vector of vectors of 2d points (the inner vector is per image). Has to be 2 X N.
/// * projection_matrices: Input vector with 3x4 projections matrices of each image.
/// * points3d: Output array with computed 3d points. Is 3 x N.
///
/// Triangulates the 3d position of 2d correspondences between several images.
/// Reference: Internally it uses DLT method [HartleyZ00](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_HartleyZ00) 12.2 pag.312
// triangulatePoints(InputArrayOfArrays, InputArrayOfArrays, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/triangulation.hpp:59
// ("cv::sfm::triangulatePoints", vec![(pred!(mut, ["points2d", "projection_matrices", "points3d"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn triangulate_points(points2d: &impl ToInputArray, projection_matrices: &impl ToInputArray, points3d: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(points2d);
	input_array_arg!(projection_matrices);
	output_array_arg!(points3d);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_sfm_triangulatePoints_const__InputArrayR_const__InputArrayR_const__OutputArrayR(points2d.as_raw__InputArray(), projection_matrices.as_raw__InputArray(), points3d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Constant methods for [crate::sfm::BaseSFM]
// BaseSFM /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:161
pub trait BaseSFMTraitConst {
	fn as_raw_BaseSFM(&self) -> *const c_void;

	// getError()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:177
	// ("cv::sfm::BaseSFM::getError", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_error(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_BaseSFM_getError_const(self.as_raw_BaseSFM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getIntrinsics()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:179
	// ("cv::sfm::BaseSFM::getIntrinsics", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_intrinsics(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_BaseSFM_getIntrinsics_const(self.as_raw_BaseSFM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::sfm::BaseSFM]
pub trait BaseSFMTrait: crate::sfm::BaseSFMTraitConst {
	fn as_raw_mut_BaseSFM(&mut self) -> *mut c_void;

	// run(InputArrayOfArrays)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:167
	// ("cv::sfm::BaseSFM::run", vec![(pred!(mut, ["points2d"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn run(&mut self, points2d: &impl ToInputArray) -> Result<()> {
		input_array_arg!(points2d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_BaseSFM_run_const__InputArrayR(self.as_raw_mut_BaseSFM(), points2d.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// run(InputArrayOfArrays, InputOutputArray, OutputArray, OutputArray, OutputArray)(InputArray, InputOutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:170
	// ("cv::sfm::BaseSFM::run", vec![(pred!(mut, ["points2d", "K", "Rs", "Ts", "points3d"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn run_1(&mut self, points2d: &impl ToInputArray, k: &mut impl ToInputOutputArray, rs: &mut impl ToOutputArray, ts: &mut impl ToOutputArray, points3d: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(points2d);
		input_output_array_arg!(k);
		output_array_arg!(rs);
		output_array_arg!(ts);
		output_array_arg!(points3d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_BaseSFM_run_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_BaseSFM(), points2d.as_raw__InputArray(), k.as_raw__InputOutputArray(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray(), points3d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// run(const std::vector<String> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:173
	// ("cv::sfm::BaseSFM::run", vec![(pred!(mut, ["images"], ["const std::vector<cv::String>*"]), _)]),
	#[inline]
	fn run_2(&mut self, images: &core::Vector<String>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_BaseSFM_run_const_vectorLStringGR(self.as_raw_mut_BaseSFM(), images.as_raw_VectorOfString(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// run(const std::vector<String> &, InputOutputArray, OutputArray, OutputArray, OutputArray)(CppPassByVoidPtr, InputOutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:174
	// ("cv::sfm::BaseSFM::run", vec![(pred!(mut, ["images", "K", "Rs", "Ts", "points3d"], ["const std::vector<cv::String>*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn run_3(&mut self, images: &core::Vector<String>, k: &mut impl ToInputOutputArray, rs: &mut impl ToOutputArray, ts: &mut impl ToOutputArray, points3d: &mut impl ToOutputArray) -> Result<()> {
		input_output_array_arg!(k);
		output_array_arg!(rs);
		output_array_arg!(ts);
		output_array_arg!(points3d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_BaseSFM_run_const_vectorLStringGR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_BaseSFM(), images.as_raw_VectorOfString(), k.as_raw__InputOutputArray(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray(), points3d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:178
	// ("cv::sfm::BaseSFM::getPoints", vec![(pred!(mut, ["points3d"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_points(&mut self, points3d: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(points3d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_BaseSFM_getPoints_const__OutputArrayR(self.as_raw_mut_BaseSFM(), points3d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getCameras(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:180
	// ("cv::sfm::BaseSFM::getCameras", vec![(pred!(mut, ["Rs", "Ts"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_cameras(&mut self, rs: &mut impl ToOutputArray, ts: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(rs);
		output_array_arg!(ts);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_BaseSFM_getCameras_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_BaseSFM(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setReconstructionOptions(const libmv_ReconstructionOptions &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:184
	// ("cv::sfm::BaseSFM::setReconstructionOptions", vec![(pred!(mut, ["libmv_reconstruction_options"], ["const cv::sfm::libmv_ReconstructionOptions*"]), _)]),
	#[inline]
	fn set_reconstruction_options(&mut self, libmv_reconstruction_options: crate::sfm::libmv_ReconstructionOptions) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_BaseSFM_setReconstructionOptions_const_libmv_ReconstructionOptionsR(self.as_raw_mut_BaseSFM(), &libmv_reconstruction_options, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setCameraIntrinsicOptions(const libmv_CameraIntrinsicsOptions &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:188
	// ("cv::sfm::BaseSFM::setCameraIntrinsicOptions", vec![(pred!(mut, ["libmv_camera_intrinsics_options"], ["const cv::sfm::libmv_CameraIntrinsicsOptions*"]), _)]),
	#[inline]
	fn set_camera_intrinsic_options(&mut self, libmv_camera_intrinsics_options: crate::sfm::libmv_CameraIntrinsicsOptions) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_BaseSFM_setCameraIntrinsicOptions_const_libmv_CameraIntrinsicsOptionsR(self.as_raw_mut_BaseSFM(), &libmv_camera_intrinsics_options, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// base class BaseSFM declares a common API that would be used in a typical scene reconstruction scenario
// BaseSFM /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:161
pub struct BaseSFM {
	ptr: *mut c_void,
}

opencv_type_boxed! { BaseSFM }

impl Drop for BaseSFM {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_sfm_BaseSFM_delete(self.as_raw_mut_BaseSFM()) };
	}
}

unsafe impl Send for BaseSFM {}

impl crate::sfm::BaseSFMTraitConst for BaseSFM {
	#[inline] fn as_raw_BaseSFM(&self) -> *const c_void { self.as_raw() }
}

impl crate::sfm::BaseSFMTrait for BaseSFM {
	#[inline] fn as_raw_mut_BaseSFM(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BaseSFM, crate::sfm::BaseSFMTraitConst, as_raw_BaseSFM, crate::sfm::BaseSFMTrait, as_raw_mut_BaseSFM }

impl BaseSFM {
}

boxed_cast_descendant! { BaseSFM, crate::sfm::SFMLibmvEuclideanReconstruction, cv_sfm_BaseSFM_to_SFMLibmvEuclideanReconstruction }

impl std::fmt::Debug for BaseSFM {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("BaseSFM")
			.finish()
	}
}

/// Constant methods for [crate::sfm::SFMLibmvEuclideanReconstruction]
// SFMLibmvEuclideanReconstruction /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:193
pub trait SFMLibmvEuclideanReconstructionTraitConst: crate::sfm::BaseSFMTraitConst {
	fn as_raw_SFMLibmvEuclideanReconstruction(&self) -> *const c_void;

	/// Returns the computed reprojection error.
	// getError()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:245
	// ("cv::sfm::SFMLibmvEuclideanReconstruction::getError", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_error(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_getError_const(self.as_raw_SFMLibmvEuclideanReconstruction(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the refined camera calibration matrix.
	// getIntrinsics()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:256
	// ("cv::sfm::SFMLibmvEuclideanReconstruction::getIntrinsics", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_intrinsics(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_getIntrinsics_const(self.as_raw_SFMLibmvEuclideanReconstruction(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::sfm::SFMLibmvEuclideanReconstruction]
pub trait SFMLibmvEuclideanReconstructionTrait: crate::sfm::BaseSFMTrait + crate::sfm::SFMLibmvEuclideanReconstructionTraitConst {
	fn as_raw_mut_SFMLibmvEuclideanReconstruction(&mut self) -> *mut c_void;

	/// Calls the pipeline in order to perform Eclidean reconstruction.
	/// ## Parameters
	/// * points2d: Input vector of vectors of 2d points (the inner vector is per image).
	///
	///
	/// Note:
	///   - Tracks must be as precise as possible. It does not handle outliers and is very sensible to them.
	// run(InputArrayOfArrays)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:203
	// ("cv::sfm::SFMLibmvEuclideanReconstruction::run", vec![(pred!(mut, ["points2d"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn run(&mut self, points2d: &impl ToInputArray) -> Result<()> {
		input_array_arg!(points2d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_run_const__InputArrayR(self.as_raw_mut_SFMLibmvEuclideanReconstruction(), points2d.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calls the pipeline in order to perform Eclidean reconstruction.
	/// ## Parameters
	/// * points2d: Input vector of vectors of 2d points (the inner vector is per image).
	/// * K: Input/Output camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D). Input parameters used as initial guess.
	/// * Rs: Output vector of 3x3 rotations of the camera.
	/// * Ts: Output vector of 3x1 translations of the camera.
	/// * points3d: Output array with estimated 3d points.
	///
	///
	/// Note:
	///   - Tracks must be as precise as possible. It does not handle outliers and is very sensible to them.
	// run(InputArrayOfArrays, InputOutputArray, OutputArray, OutputArray, OutputArray)(InputArray, InputOutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:216
	// ("cv::sfm::SFMLibmvEuclideanReconstruction::run", vec![(pred!(mut, ["points2d", "K", "Rs", "Ts", "points3d"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn run_1(&mut self, points2d: &impl ToInputArray, k: &mut impl ToInputOutputArray, rs: &mut impl ToOutputArray, ts: &mut impl ToOutputArray, points3d: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(points2d);
		input_output_array_arg!(k);
		output_array_arg!(rs);
		output_array_arg!(ts);
		output_array_arg!(points3d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_run_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_SFMLibmvEuclideanReconstruction(), points2d.as_raw__InputArray(), k.as_raw__InputOutputArray(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray(), points3d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calls the pipeline in order to perform Eclidean reconstruction.
	/// ## Parameters
	/// * images: a vector of string with the images paths.
	///
	///
	/// Note:
	///   - The images must be ordered as they were an image sequence. Additionally, each frame should be as close as posible to the previous and posterior.
	///   - For now DAISY features are used in order to compute the 2d points tracks and it only works for 3-4 images.
	// run(const std::vector<String> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:226
	// ("cv::sfm::SFMLibmvEuclideanReconstruction::run", vec![(pred!(mut, ["images"], ["const std::vector<cv::String>*"]), _)]),
	#[inline]
	fn run_2(&mut self, images: &core::Vector<String>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_run_const_vectorLStringGR(self.as_raw_mut_SFMLibmvEuclideanReconstruction(), images.as_raw_VectorOfString(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calls the pipeline in order to perform Eclidean reconstruction.
	/// ## Parameters
	/// * images: a vector of string with the images paths.
	/// * K: Input/Output camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D). Input parameters used as initial guess.
	/// * Rs: Output vector of 3x3 rotations of the camera.
	/// * Ts: Output vector of 3x1 translations of the camera.
	/// * points3d: Output array with estimated 3d points.
	///
	///
	/// Note:
	///   - The images must be ordered as they were an image sequence. Additionally, each frame should be as close as posible to the previous and posterior.
	///   - For now DAISY features are used in order to compute the 2d points tracks and it only works for 3-4 images.
	// run(const std::vector<String> &, InputOutputArray, OutputArray, OutputArray, OutputArray)(CppPassByVoidPtr, InputOutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:239
	// ("cv::sfm::SFMLibmvEuclideanReconstruction::run", vec![(pred!(mut, ["images", "K", "Rs", "Ts", "points3d"], ["const std::vector<cv::String>*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn run_3(&mut self, images: &core::Vector<String>, k: &mut impl ToInputOutputArray, rs: &mut impl ToOutputArray, ts: &mut impl ToOutputArray, points3d: &mut impl ToOutputArray) -> Result<()> {
		input_output_array_arg!(k);
		output_array_arg!(rs);
		output_array_arg!(ts);
		output_array_arg!(points3d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_run_const_vectorLStringGR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_SFMLibmvEuclideanReconstruction(), images.as_raw_VectorOfString(), k.as_raw__InputOutputArray(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray(), points3d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the estimated 3d points.
	/// ## Parameters
	/// * points3d: Output array with estimated 3d points.
	// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:251
	// ("cv::sfm::SFMLibmvEuclideanReconstruction::getPoints", vec![(pred!(mut, ["points3d"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_points(&mut self, points3d: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(points3d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_getPoints_const__OutputArrayR(self.as_raw_mut_SFMLibmvEuclideanReconstruction(), points3d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the estimated camera extrinsic parameters.
	/// ## Parameters
	/// * Rs: Output vector of 3x3 rotations of the camera.
	/// * Ts: Output vector of 3x1 translations of the camera.
	// getCameras(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:263
	// ("cv::sfm::SFMLibmvEuclideanReconstruction::getCameras", vec![(pred!(mut, ["Rs", "Ts"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_cameras(&mut self, rs: &mut impl ToOutputArray, ts: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(rs);
		output_array_arg!(ts);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_getCameras_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_SFMLibmvEuclideanReconstruction(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Setter method for reconstruction options.
	/// ## Parameters
	/// * libmv_reconstruction_options: struct with reconstruction options such as initial keyframes,
	///   automatic keyframe selection, parameters to refine and the verbosity level.
	// setReconstructionOptions(const libmv_ReconstructionOptions &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:271
	// ("cv::sfm::SFMLibmvEuclideanReconstruction::setReconstructionOptions", vec![(pred!(mut, ["libmv_reconstruction_options"], ["const cv::sfm::libmv_ReconstructionOptions*"]), _)]),
	#[inline]
	fn set_reconstruction_options(&mut self, libmv_reconstruction_options: crate::sfm::libmv_ReconstructionOptions) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_setReconstructionOptions_const_libmv_ReconstructionOptionsR(self.as_raw_mut_SFMLibmvEuclideanReconstruction(), &libmv_reconstruction_options, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Setter method for camera intrinsic options.
	/// ## Parameters
	/// * libmv_camera_intrinsics_options: struct with camera intrinsic options such as camera model and
	///   the internal camera parameters.
	// setCameraIntrinsicOptions(const libmv_CameraIntrinsicsOptions &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:279
	// ("cv::sfm::SFMLibmvEuclideanReconstruction::setCameraIntrinsicOptions", vec![(pred!(mut, ["libmv_camera_intrinsics_options"], ["const cv::sfm::libmv_CameraIntrinsicsOptions*"]), _)]),
	#[inline]
	fn set_camera_intrinsic_options(&mut self, libmv_camera_intrinsics_options: crate::sfm::libmv_CameraIntrinsicsOptions) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_setCameraIntrinsicOptions_const_libmv_CameraIntrinsicsOptionsR(self.as_raw_mut_SFMLibmvEuclideanReconstruction(), &libmv_camera_intrinsics_options, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// SFMLibmvEuclideanReconstruction class provides an interface with the Libmv Structure From Motion pipeline.
// SFMLibmvEuclideanReconstruction /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:193
pub struct SFMLibmvEuclideanReconstruction {
	ptr: *mut c_void,
}

opencv_type_boxed! { SFMLibmvEuclideanReconstruction }

impl Drop for SFMLibmvEuclideanReconstruction {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_delete(self.as_raw_mut_SFMLibmvEuclideanReconstruction()) };
	}
}

unsafe impl Send for SFMLibmvEuclideanReconstruction {}

impl crate::sfm::BaseSFMTraitConst for SFMLibmvEuclideanReconstruction {
	#[inline] fn as_raw_BaseSFM(&self) -> *const c_void { self.as_raw() }
}

impl crate::sfm::BaseSFMTrait for SFMLibmvEuclideanReconstruction {
	#[inline] fn as_raw_mut_BaseSFM(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SFMLibmvEuclideanReconstruction, crate::sfm::BaseSFMTraitConst, as_raw_BaseSFM, crate::sfm::BaseSFMTrait, as_raw_mut_BaseSFM }

impl crate::sfm::SFMLibmvEuclideanReconstructionTraitConst for SFMLibmvEuclideanReconstruction {
	#[inline] fn as_raw_SFMLibmvEuclideanReconstruction(&self) -> *const c_void { self.as_raw() }
}

impl crate::sfm::SFMLibmvEuclideanReconstructionTrait for SFMLibmvEuclideanReconstruction {
	#[inline] fn as_raw_mut_SFMLibmvEuclideanReconstruction(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SFMLibmvEuclideanReconstruction, crate::sfm::SFMLibmvEuclideanReconstructionTraitConst, as_raw_SFMLibmvEuclideanReconstruction, crate::sfm::SFMLibmvEuclideanReconstructionTrait, as_raw_mut_SFMLibmvEuclideanReconstruction }

impl SFMLibmvEuclideanReconstruction {
	/// Creates an instance of the SFMLibmvEuclideanReconstruction class. Initializes Libmv.
	///
	/// ## C++ default parameters
	/// * camera_instrinsic_options: libmv_CameraIntrinsicsOptions()
	/// * reconstruction_options: libmv_ReconstructionOptions()
	// create(const libmv_CameraIntrinsicsOptions &, const libmv_ReconstructionOptions &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:283
	// ("cv::sfm::SFMLibmvEuclideanReconstruction::create", vec![(pred!(mut, ["camera_instrinsic_options", "reconstruction_options"], ["const cv::sfm::libmv_CameraIntrinsicsOptions*", "const cv::sfm::libmv_ReconstructionOptions*"]), _)]),
	#[inline]
	pub fn create(camera_instrinsic_options: crate::sfm::libmv_CameraIntrinsicsOptions, reconstruction_options: crate::sfm::libmv_ReconstructionOptions) -> Result<core::Ptr<crate::sfm::SFMLibmvEuclideanReconstruction>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_create_const_libmv_CameraIntrinsicsOptionsR_const_libmv_ReconstructionOptionsR(&camera_instrinsic_options, &reconstruction_options, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::sfm::SFMLibmvEuclideanReconstruction>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates an instance of the SFMLibmvEuclideanReconstruction class. Initializes Libmv.
	///
	/// ## Note
	/// This alternative version of [SFMLibmvEuclideanReconstruction::create] function uses the following default values for its arguments:
	/// * camera_instrinsic_options: libmv_CameraIntrinsicsOptions()
	/// * reconstruction_options: libmv_ReconstructionOptions()
	// cv::sfm::SFMLibmvEuclideanReconstruction::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:283
	// ("cv::sfm::SFMLibmvEuclideanReconstruction::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::sfm::SFMLibmvEuclideanReconstruction>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::sfm::SFMLibmvEuclideanReconstruction>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { SFMLibmvEuclideanReconstruction, crate::sfm::BaseSFM, cv_sfm_SFMLibmvEuclideanReconstruction_to_BaseSFM }

impl std::fmt::Debug for SFMLibmvEuclideanReconstruction {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SFMLibmvEuclideanReconstruction")
			.finish()
	}
}

/// Data structure describing the camera model and its parameters.
/// ## Parameters
/// * _distortion_model: Type of camera model.
/// * _focal_length_x: focal length of the camera (in pixels).
/// * _focal_length_y: focal length of the camera (in pixels).
/// * _principal_point_x: principal point of the camera in the x direction (in pixels).
/// * _principal_point_y: principal point of the camera in the y direction (in pixels).
/// * _polynomial_k1: radial distortion parameter.
/// * _polynomial_k2: radial distortion parameter.
/// * _polynomial_k3: radial distortion parameter.
/// * _polynomial_p1: radial distortion parameter.
/// * _polynomial_p2: radial distortion parameter.
///
/// Is assumed that modern cameras have their principal point in the image center.
///
/// In case that the camera model was SFM_DISTORTION_MODEL_DIVISION, it's only needed to provide
/// _polynomial_k1 and _polynomial_k2 which will be assigned as division distortion parameters.
// libmv_CameraIntrinsicsOptions /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:72
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct libmv_CameraIntrinsicsOptions {
	pub distortion_model: i32,
	pub image_width: i32,
	pub image_height: i32,
	pub focal_length_x: f64,
	pub focal_length_y: f64,
	pub principal_point_x: f64,
	pub principal_point_y: f64,
	pub polynomial_k1: f64,
	pub polynomial_k2: f64,
	pub polynomial_k3: f64,
	pub polynomial_p1: f64,
	pub polynomial_p2: f64,
	pub division_k1: f64,
	pub division_k2: f64,
}

opencv_type_simple! { crate::sfm::libmv_CameraIntrinsicsOptions }

impl libmv_CameraIntrinsicsOptions {
	/// ## C++ default parameters
	/// * _distortion_model: 0
	/// * _focal_length_x: 0
	/// * _focal_length_y: 0
	/// * _principal_point_x: 0
	/// * _principal_point_y: 0
	/// * _polynomial_k1: 0
	/// * _polynomial_k2: 0
	/// * _polynomial_k3: 0
	/// * _polynomial_p1: 0
	/// * _polynomial_p2: 0
	// libmv_CameraIntrinsicsOptions(const int, const double, const double, const double, const double, const double, const double, const double, const double, const double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:76
	// ("cv::sfm::libmv_CameraIntrinsicsOptions::libmv_CameraIntrinsicsOptions", vec![(pred!(mut, ["_distortion_model", "_focal_length_x", "_focal_length_y", "_principal_point_x", "_principal_point_y", "_polynomial_k1", "_polynomial_k2", "_polynomial_k3", "_polynomial_p1", "_polynomial_p2"], ["const int", "const double", "const double", "const double", "const double", "const double", "const double", "const double", "const double", "const double"]), _)]),
	#[inline]
	pub fn new(_distortion_model: i32, _focal_length_x: f64, _focal_length_y: f64, _principal_point_x: f64, _principal_point_y: f64, _polynomial_k1: f64, _polynomial_k2: f64, _polynomial_k3: f64, _polynomial_p1: f64, _polynomial_p2: f64) -> Result<crate::sfm::libmv_CameraIntrinsicsOptions> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_libmv_CameraIntrinsicsOptions_libmv_CameraIntrinsicsOptions_const_int_const_double_const_double_const_double_const_double_const_double_const_double_const_double_const_double_const_double(_distortion_model, _focal_length_x, _focal_length_y, _principal_point_x, _principal_point_y, _polynomial_k1, _polynomial_k2, _polynomial_k3, _polynomial_p1, _polynomial_p2, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * _distortion_model: 0
	/// * _focal_length_x: 0
	/// * _focal_length_y: 0
	/// * _principal_point_x: 0
	/// * _principal_point_y: 0
	/// * _polynomial_k1: 0
	/// * _polynomial_k2: 0
	/// * _polynomial_k3: 0
	/// * _polynomial_p1: 0
	/// * _polynomial_p2: 0
	// cv::sfm::libmv_CameraIntrinsicsOptions::libmv_CameraIntrinsicsOptions() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:76
	// ("cv::sfm::libmv_CameraIntrinsicsOptions::libmv_CameraIntrinsicsOptions", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::sfm::libmv_CameraIntrinsicsOptions> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_libmv_CameraIntrinsicsOptions_libmv_CameraIntrinsicsOptions(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Data structure describing the reconstruction options.
/// ## Parameters
/// * _keyframe1: first keyframe used in order to initialize the reconstruction.
/// * _keyframe2: second keyframe used in order to initialize the reconstruction.
/// * _refine_intrinsics: camera parameter or combination of parameters to refine.
/// * _select_keyframes: allows to select automatically the initial keyframes. If 1 then autoselection is enabled. If 0 then is disabled.
/// * _verbosity_level: verbosity logs level for Glog. If -1 then logs are disabled, otherwise the log level will be the input integer.
// libmv_ReconstructionOptions /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:138
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct libmv_ReconstructionOptions {
	pub keyframe1: i32,
	pub keyframe2: i32,
	pub refine_intrinsics: i32,
	pub select_keyframes: i32,
	pub verbosity_level: i32,
}

opencv_type_simple! { crate::sfm::libmv_ReconstructionOptions }

impl libmv_ReconstructionOptions {
	/// ## C++ default parameters
	/// * _keyframe1: 1
	/// * _keyframe2: 2
	/// * _refine_intrinsics: 1
	/// * _select_keyframes: 1
	/// * _verbosity_level: -1
	// libmv_ReconstructionOptions(const int, const int, const int, const int, const int)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:142
	// ("cv::sfm::libmv_ReconstructionOptions::libmv_ReconstructionOptions", vec![(pred!(mut, ["_keyframe1", "_keyframe2", "_refine_intrinsics", "_select_keyframes", "_verbosity_level"], ["const int", "const int", "const int", "const int", "const int"]), _)]),
	#[inline]
	pub fn new(_keyframe1: i32, _keyframe2: i32, _refine_intrinsics: i32, _select_keyframes: i32, _verbosity_level: i32) -> Result<crate::sfm::libmv_ReconstructionOptions> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_libmv_ReconstructionOptions_libmv_ReconstructionOptions_const_int_const_int_const_int_const_int_const_int(_keyframe1, _keyframe2, _refine_intrinsics, _select_keyframes, _verbosity_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * _keyframe1: 1
	/// * _keyframe2: 2
	/// * _refine_intrinsics: 1
	/// * _select_keyframes: 1
	/// * _verbosity_level: -1
	// cv::sfm::libmv_ReconstructionOptions::libmv_ReconstructionOptions() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/sfm/simple_pipeline.hpp:142
	// ("cv::sfm::libmv_ReconstructionOptions::libmv_ReconstructionOptions", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::sfm::libmv_ReconstructionOptions> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_libmv_ReconstructionOptions_libmv_ReconstructionOptions(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}
