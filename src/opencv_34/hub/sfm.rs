//! # Structure From Motion
//!
//! The opencv_sfm module contains algorithms to perform 3d reconstruction
//! from 2d images.\n
//! The core of the module is based on a light version of
//! [Libmv](https://developer.blender.org/project/profile/59) originally
//! developed by Sameer Agarwal and Keir Mierle.
//!
//! __Whats is libmv?__ \n
//! libmv, also known as the Library for Multiview Reconstruction (or LMV),
//! is the computer vision backend for Blender's motion tracking abilities.
//! Unlike other vision libraries with general ambitions, libmv is focused
//! on algorithms for match moving, specifically targeting [Blender](https://developer.blender.org) as the
//! primary customer. Dense reconstruction, reconstruction from unorganized
//! photo collections, image recognition, and other tasks are not a focus
//! of libmv.
//!
//! __Development__ \n
//! libmv is officially under the Blender umbrella, and so is developed
//! on developer.blender.org. The [source repository](https://developer.blender.org/diffusion/LMV) can get checked out
//! independently from Blender.
//!
//! This module has been originally developed as a project for Google Summer of Code 2012-2015.
//!
//!
//! Note:
//! - Notice that it is compiled only when Eigen, GLog and GFlags are correctly installed.\n
//! Check installation instructions in the following tutorial: @ref tutorial_sfm_installation
//! # Conditioning
//! # Fundamental
//! # Input/Output
//! # Numeric
//! # Projection
//! # Robust Estimation
//! # Triangulation
//!
//! # Reconstruction
//!
//! Note:
//! - Notice that it is compiled only when Ceres Solver is correctly installed.\n
//! Check installation instructions in the following tutorial: @ref tutorial_sfm_installation
//!
//!
//! # Simple Pipeline
//!
//! Note:
//! - Notice that it is compiled only when Ceres Solver is correctly installed.\n
//! Check installation instructions in the following tutorial: @ref tutorial_sfm_installation
use std::os::raw::{c_char, c_void};
use libc::{ptrdiff_t, size_t};
use crate::{Error, Result, core, sys, types};

pub const SFM_DISTORTION_MODEL_DIVISION: i32 = 1;
pub const SFM_DISTORTION_MODEL_POLYNOMIAL: i32 = 0;
pub const SFM_IO_BUNDLER: i32 = 0;
pub const SFM_IO_OPENMVG: i32 = 3;
pub const SFM_IO_OPENSFM: i32 = 2;
pub const SFM_IO_THEIASFM: i32 = 4;
pub const SFM_IO_VISUALSFM: i32 = 1;
pub const SFM_REFINE_FOCAL_LENGTH: i32 = (1 << 0);
pub const SFM_REFINE_PRINCIPAL_POINT: i32 = (1 << 1);
pub const SFM_REFINE_RADIAL_DISTORTION_K1: i32 = (1 << 2);
pub const SFM_REFINE_RADIAL_DISTORTION_K2: i32 = (1 << 4);

/// Data structure describing the camera model and its parameters.
/// ## Parameters
/// * _distortion_model: Type of camera model.
/// * _focal_length: focal length of the camera (in pixels).
/// * _principal_point_x: principal point of the camera in the x direction (in pixels).
/// * _principal_point_y: principal point of the camera in the y direction (in pixels).
/// * _polynomial_k1: radial distortion parameter.
/// * _polynomial_k2: radial distortion parameter.
/// * _polynomial_k3: radial distortion parameter.
/// * _polynomial_p1: radial distortion parameter.
/// * _polynomial_p2: radial distortion parameter.
///
/// Is assumed that modern cameras have their principal point in the image center.\n
/// In case that the camera model was SFM_DISTORTION_MODEL_DIVISION, it's only needed to provide
/// _polynomial_k1 and _polynomial_k2 which will be assigned as division distortion parameters.
#[repr(C)]
#[derive(Copy,Clone,Debug,PartialEq)]
pub struct libmv_CameraIntrinsicsOptions {
    pub distortion_model: i32,
    pub image_width: i32,
    pub image_height: i32,
    pub focal_length: f64,
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

/// Data structure describing the reconstruction options.
/// ## Parameters
/// * _keyframe1: first keyframe used in order to initialize the reconstruction.
/// * _keyframe2: second keyframe used in order to initialize the reconstruction.
/// * _refine_intrinsics: camera parameter or combination of parameters to refine.
/// * _select_keyframes: allows to select automatically the initial keyframes. If 1 then autoselection is enabled. If 0 then is disabled.
/// * _verbosity_level: verbosity logs level for Glog. If -1 then logs are disabled, otherwise the log level will be the input integer.
#[repr(C)]
#[derive(Copy,Clone,Debug,PartialEq)]
pub struct libmv_ReconstructionOptions {
    pub keyframe1: i32,
    pub keyframe2: i32,
    pub refine_intrinsics: i32,
    pub select_keyframes: i32,
    pub verbosity_level: i32,
}

/// Get K, R and t from projection matrix P, decompose using the RQ decomposition.
/// ## Parameters
/// * P: Input 3x4 projection matrix.
/// * K: Output 3x3 camera matrix <span lang='latex'>K = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{1}</span>.
/// * R: Output 3x3 rotation matrix.
/// * t: Output 3x1 translation vector.
///
/// Reference: [HartleyZ00](https://docs.opencv.org/3.4.7/d0/de3/citelist.html#CITEREF_HartleyZ00) A4.1.1 pag.579
pub fn k_rt_from_projection(p: &core::Mat, k: &mut core::Mat, r: &mut core::Mat, t: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_sfm_KRtFromProjection_Mat_Mat_Mat_Mat(p.as_raw_Mat(), k.as_raw_Mat(), r.as_raw_Mat(), t.as_raw_Mat()) }.into_result()
}

/// Apply Transformation to points.
/// ## Parameters
/// * points: Input vector of N-dimensional points.
/// * T: Input 3x3 transformation matrix such that <span lang='latex'>x = T*X</span>, where <span lang='latex'>X</span> are the points to transform and <span lang='latex'>x</span> the transformed points.
/// * transformed_points: Output vector of N-dimensional transformed points.
pub fn apply_transformation_to_points(points: &core::Mat, t: &core::Mat, transformed_points: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_sfm_applyTransformationToPoints_Mat_Mat_Mat(points.as_raw_Mat(), t.as_raw_Mat(), transformed_points.as_raw_Mat()) }.into_result()
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
pub fn compute_orientation(x1: &types::VectorOfMat, x2: &types::VectorOfMat, r: &mut core::Mat, t: &mut core::Mat, s: f64) -> Result<()> {
    unsafe { sys::cv_sfm_computeOrientation_VectorOfMat_VectorOfMat_Mat_Mat_double(x1.as_raw_VectorOfMat(), x2.as_raw_VectorOfMat(), r.as_raw_Mat(), t.as_raw_Mat(), s) }.into_result()
}

/// Returns the depth of a point transformed by a rigid transform.
/// ## Parameters
/// * R: Input 3x3 rotation matrix.
/// * t: Input 3x1 translation vector.
/// * X: Input 3x1 or 4x1 vector with the 3d point.
pub fn depth(r: &core::Mat, t: &core::Mat, x: &core::Mat) -> Result<f64> {
    unsafe { sys::cv_sfm_depth_Mat_Mat_Mat(r.as_raw_Mat(), t.as_raw_Mat(), x.as_raw_Mat()) }.into_result()
}

/// Get Essential matrix from Fundamental and Camera matrices.
/// ## Parameters
/// * F: Input 3x3 fundamental matrix.
/// * K1: Input 3x3 first camera matrix <span lang='latex'>K = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{1}</span>.
/// * K2: Input 3x3 second camera matrix. The parameters are similar to K1.
/// * E: Output 3x3 essential matrix.
///
/// Reference: [HartleyZ00](https://docs.opencv.org/3.4.7/d0/de3/citelist.html#CITEREF_HartleyZ00) 9.6 pag 257 (formula 9.12)
pub fn essential_from_fundamental(f: &core::Mat, k1: &core::Mat, k2: &core::Mat, e: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_sfm_essentialFromFundamental_Mat_Mat_Mat_Mat(f.as_raw_Mat(), k1.as_raw_Mat(), k2.as_raw_Mat(), e.as_raw_Mat()) }.into_result()
}

/// Get Essential matrix from Motion (R's and t's ).
/// ## Parameters
/// * R1: Input 3x3 first camera rotation matrix.
/// * t1: Input 3x1 first camera translation vector.
/// * R2: Input 3x3 second camera rotation matrix.
/// * t2: Input 3x1 second camera translation vector.
/// * E: Output 3x3 essential matrix.
///
/// Reference: [HartleyZ00](https://docs.opencv.org/3.4.7/d0/de3/citelist.html#CITEREF_HartleyZ00) 9.6 pag 257 (formula 9.12)
pub fn essential_from_rt(r1: &core::Mat, t1: &core::Mat, r2: &core::Mat, t2: &core::Mat, e: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_sfm_essentialFromRt_Mat_Mat_Mat_Mat_Mat(r1.as_raw_Mat(), t1.as_raw_Mat(), r2.as_raw_Mat(), t2.as_raw_Mat(), e.as_raw_Mat()) }.into_result()
}

/// Converts points from Euclidean to homogeneous space. E.g., ((x,y)->(x,y,1))
/// ## Parameters
/// * src: Input vector of N-dimensional points.
/// * dst: Output vector of N+1-dimensional points.
pub fn euclidean_to_homogeneous(src: &core::Mat, dst: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_sfm_euclideanToHomogeneous_Mat_Mat(src.as_raw_Mat(), dst.as_raw_Mat()) }.into_result()
}

/// Estimate robustly the fundamental matrix between two dataset of 2D point (image coords space).
/// ## Parameters
/// * x1: Input 2xN Array of 2D points in view 1.
/// * x2: Input 2xN Array of 2D points in view 2.
/// * max_error: maximum error (in pixels).
/// * F: Output 3x3 fundamental matrix such that <span lang='latex'>x_2^T F x_1=0</span>.
/// * inliers: Output 1xN vector that contains the indexes of the detected inliers.
/// * outliers_probability: outliers probability (in ]0,1[).
/// The number of iterations is controlled using the following equation:
/// <span lang='latex'>k = \frac{log(1-p)}{log(1.0 - w^n )}</span> where <span lang='latex'>k</span>, <span lang='latex'>w</span> and <span lang='latex'>n</span> are the number of
/// iterations, the inliers ratio and minimun number of selected independent samples.
/// The more this value is high, the less the function selects ramdom samples.
///
/// The fundamental solver relies on the 7 point solution. Returns the best error (in pixels), associated to the solution F.
///
/// ## C++ default parameters
/// * outliers_probability: 1e-2
pub fn fundamental_from_correspondences7_point_robust(x1: &core::Mat, x2: &core::Mat, max_error: f64, f: &mut core::Mat, inliers: &mut core::Mat, outliers_probability: f64) -> Result<f64> {
    unsafe { sys::cv_sfm_fundamentalFromCorrespondences7PointRobust_Mat_Mat_double_Mat_Mat_double(x1.as_raw_Mat(), x2.as_raw_Mat(), max_error, f.as_raw_Mat(), inliers.as_raw_Mat(), outliers_probability) }.into_result()
}

/// Estimate robustly the fundamental matrix between two dataset of 2D point (image coords space).
/// ## Parameters
/// * x1: Input 2xN Array of 2D points in view 1.
/// * x2: Input 2xN Array of 2D points in view 2.
/// * max_error: maximum error (in pixels).
/// * F: Output 3x3 fundamental matrix such that <span lang='latex'>x_2^T F x_1=0</span>.
/// * inliers: Output 1xN vector that contains the indexes of the detected inliers.
/// * outliers_probability: outliers probability (in ]0,1[).
/// The number of iterations is controlled using the following equation:
/// <span lang='latex'>k = \frac{log(1-p)}{log(1.0 - w^n )}</span> where <span lang='latex'>k</span>, <span lang='latex'>w</span> and <span lang='latex'>n</span> are the number of
/// iterations, the inliers ratio and minimun number of selected independent samples.
/// The more this value is high, the less the function selects ramdom samples.
///
/// The fundamental solver relies on the 8 point solution. Returns the best error (in pixels), associated to the solution F.
///
/// ## C++ default parameters
/// * outliers_probability: 1e-2
pub fn fundamental_from_correspondences8_point_robust(x1: &core::Mat, x2: &core::Mat, max_error: f64, f: &mut core::Mat, inliers: &mut core::Mat, outliers_probability: f64) -> Result<f64> {
    unsafe { sys::cv_sfm_fundamentalFromCorrespondences8PointRobust_Mat_Mat_double_Mat_Mat_double(x1.as_raw_Mat(), x2.as_raw_Mat(), max_error, f.as_raw_Mat(), inliers.as_raw_Mat(), outliers_probability) }.into_result()
}

/// Get Essential matrix from Fundamental and Camera matrices.
/// ## Parameters
/// * E: Input 3x3 essential matrix.
/// * K1: Input 3x3 first camera matrix <span lang='latex'>K = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{1}</span>.
/// * K2: Input 3x3 second camera matrix. The parameters are similar to K1.
/// * F: Output 3x3 fundamental matrix.
///
/// Reference: [HartleyZ00](https://docs.opencv.org/3.4.7/d0/de3/citelist.html#CITEREF_HartleyZ00) 9.6 pag 257 (formula 9.12) or http://ai.stanford.edu/~birch/projective/node20.html
pub fn fundamental_from_essential(e: &core::Mat, k1: &core::Mat, k2: &core::Mat, f: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_sfm_fundamentalFromEssential_Mat_Mat_Mat_Mat(e.as_raw_Mat(), k1.as_raw_Mat(), k2.as_raw_Mat(), f.as_raw_Mat()) }.into_result()
}

/// Get Fundamental matrix from Projection matrices.
/// ## Parameters
/// * P1: Input 3x4 first projection matrix.
/// * P2: Input 3x4 second projection matrix.
/// * F: Output 3x3 fundamental matrix.
pub fn fundamental_from_projections(p1: &core::Mat, p2: &core::Mat, f: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_sfm_fundamentalFromProjections_Mat_Mat_Mat(p1.as_raw_Mat(), p2.as_raw_Mat(), f.as_raw_Mat()) }.into_result()
}

/// Converts point coordinates from homogeneous to euclidean pixel coordinates. E.g., ((x,y,z)->(x/z, y/z))
/// ## Parameters
/// * src: Input vector of N-dimensional points.
/// * dst: Output vector of N-1-dimensional points.
pub fn homogeneous_to_euclidean(src: &core::Mat, dst: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_sfm_homogeneousToEuclidean_Mat_Mat(src.as_raw_Mat(), dst.as_raw_Mat()) }.into_result()
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
pub fn import_reconstruction(file: &str, rs: &mut types::VectorOfMat, ts: &mut types::VectorOfMat, ks: &mut types::VectorOfMat, points3d: &mut core::Mat, file_format: i32) -> Result<()> {
    string_arg!(file);
    unsafe { sys::cv_sfm_importReconstruction_String_VectorOfMat_VectorOfMat_VectorOfMat_Mat_int(file.as_ptr(), rs.as_raw_VectorOfMat(), ts.as_raw_VectorOfMat(), ks.as_raw_VectorOfMat(), points3d.as_raw_Mat(), file_format) }.into_result()
}

/// Point conditioning (isotropic).
/// ## Parameters
/// * points: Input vector of N-dimensional points.
/// * T: Output 3x3 transformation matrix.
///
/// Computes the transformation matrix such that each coordinate direction will be scaled equally,
/// bringing the centroid to the origin with an average centroid <span lang='latex'>(1,1,1)^T</span>.\n
/// Reference: [HartleyZ00](https://docs.opencv.org/3.4.7/d0/de3/citelist.html#CITEREF_HartleyZ00) 4.4.4 pag.107.
pub fn isotropic_preconditioner_from_points(points: &core::Mat, t: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_sfm_isotropicPreconditionerFromPoints_Mat_Mat(points.as_raw_Mat(), t.as_raw_Mat()) }.into_result()
}

/// Computes the mean and variance of a given matrix along its rows.
/// ## Parameters
/// * A: Input NxN matrix.
/// * mean: Output Nx1 matrix with computed mean.
/// * variance: Output Nx1 matrix with computed variance.
///
/// It computes in the same way as woud do @ref reduce but with \a Variance function.
pub fn mean_and_variance_along_rows(a: &core::Mat, mean: &mut core::Mat, variance: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_sfm_meanAndVarianceAlongRows_Mat_Mat_Mat(a.as_raw_Mat(), mean.as_raw_Mat(), variance.as_raw_Mat()) }.into_result()
}

/// Choose one of the four possible motion solutions from an essential matrix.
/// ## Parameters
/// * Rs: Input vector of 3x3 rotation matrices.
/// * ts: Input vector of 3x1 translation vectors.
/// * K1: Input 3x3 first camera matrix <span lang='latex'>K = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{1}</span>.
/// * x1: Input 2x1 vector with first 2d point.
/// * K2: Input 3x3 second camera matrix. The parameters are similar to K1.
/// * x2: Input 2x1 vector with second 2d point.
///
/// Decides the right solution by checking that the triangulation of a match
/// x1--x2 lies in front of the cameras. Return index of the right solution or -1 if no solution.
///
/// Reference: See [HartleyZ00](https://docs.opencv.org/3.4.7/d0/de3/citelist.html#CITEREF_HartleyZ00) 9.6 pag 259 (9.6.3 Geometrical interpretation of the 4 solutions).
pub fn motion_from_essential_choose_solution(rs: &types::VectorOfMat, ts: &types::VectorOfMat, k1: &core::Mat, x1: &core::Mat, k2: &core::Mat, x2: &core::Mat) -> Result<i32> {
    unsafe { sys::cv_sfm_motionFromEssentialChooseSolution_VectorOfMat_VectorOfMat_Mat_Mat_Mat_Mat(rs.as_raw_VectorOfMat(), ts.as_raw_VectorOfMat(), k1.as_raw_Mat(), x1.as_raw_Mat(), k2.as_raw_Mat(), x2.as_raw_Mat()) }.into_result()
}

/// Get Motion (R's and t's ) from Essential matrix.
/// ## Parameters
/// * E: Input 3x3 essential matrix.
/// * Rs: Output vector of 3x3 rotation matrices.
/// * ts: Output vector of 3x1 translation vectors.
///
/// Reference: [HartleyZ00](https://docs.opencv.org/3.4.7/d0/de3/citelist.html#CITEREF_HartleyZ00) 9.6 pag 259 (Result 9.19)
pub fn motion_from_essential(e: &core::Mat, rs: &mut types::VectorOfMat, ts: &mut types::VectorOfMat) -> Result<()> {
    unsafe { sys::cv_sfm_motionFromEssential_Mat_VectorOfMat_VectorOfMat(e.as_raw_Mat(), rs.as_raw_VectorOfMat(), ts.as_raw_VectorOfMat()) }.into_result()
}

/// Normalizes the Fundamental matrix.
/// ## Parameters
/// * F: Input 3x3 fundamental matrix.
/// * F_normalized: Output 3x3 normalized fundamental matrix.
///
/// By default divides the fundamental matrix by its L2 norm.
pub fn normalize_fundamental(f: &core::Mat, f_normalized: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_sfm_normalizeFundamental_Mat_Mat(f.as_raw_Mat(), f_normalized.as_raw_Mat()) }.into_result()
}

/// This function normalizes points. (isotropic).
/// ## Parameters
/// * points: Input vector of N-dimensional points.
/// * normalized_points: Output vector of the same N-dimensional points but with mean 0 and average norm <span lang='latex'>\sqrt{2}</span>.
/// * T: Output 3x3 transform matrix such that <span lang='latex'>x = T*X</span>, where <span lang='latex'>X</span> are the points to normalize and <span lang='latex'>x</span> the normalized points.
///
/// Internally calls @ref preconditionerFromPoints in order to get the scaling matrix before applying @ref applyTransformationToPoints.
/// This operation is an essential step before applying the DLT algorithm in order to consider the result as optimal.\n
/// Reference: [HartleyZ00](https://docs.opencv.org/3.4.7/d0/de3/citelist.html#CITEREF_HartleyZ00) 4.4.4 pag.107.
pub fn normalize_isotropic_points(points: &core::Mat, normalized_points: &mut core::Mat, t: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_sfm_normalizeIsotropicPoints_Mat_Mat_Mat(points.as_raw_Mat(), normalized_points.as_raw_Mat(), t.as_raw_Mat()) }.into_result()
}

/// This function normalizes points (non isotropic).
/// ## Parameters
/// * points: Input vector of N-dimensional points.
/// * normalized_points: Output vector of the same N-dimensional points but with mean 0 and average norm <span lang='latex'>\sqrt{2}</span>.
/// * T: Output 3x3 transform matrix such that <span lang='latex'>x = T*X</span>, where <span lang='latex'>X</span> are the points to normalize and <span lang='latex'>x</span> the normalized points.
///
/// Internally calls @ref preconditionerFromPoints in order to get the scaling matrix before applying @ref applyTransformationToPoints.
/// This operation is an essential step before applying the DLT algorithm in order to consider the result as optimal.\n
/// Reference: [HartleyZ00](https://docs.opencv.org/3.4.7/d0/de3/citelist.html#CITEREF_HartleyZ00) 4.4.4 pag.109
pub fn normalize_points(points: &core::Mat, normalized_points: &mut core::Mat, t: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_sfm_normalizePoints_Mat_Mat_Mat(points.as_raw_Mat(), normalized_points.as_raw_Mat(), t.as_raw_Mat()) }.into_result()
}

/// Estimate the fundamental matrix between two dataset of 2D point (image coords space).
/// ## Parameters
/// * x1: Input 2xN Array of 2D points in view 1.
/// * x2: Input 2xN Array of 2D points in view 2.
/// * F: Output 3x3 fundamental matrix.
///
/// Uses the normalized 8-point fundamental matrix solver.
/// Reference: [HartleyZ00](https://docs.opencv.org/3.4.7/d0/de3/citelist.html#CITEREF_HartleyZ00) 11.2 pag.281 (x1 = x, x2 = x')
pub fn normalized_eight_point_solver(x1: &core::Mat, x2: &core::Mat, f: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_sfm_normalizedEightPointSolver_Mat_Mat_Mat(x1.as_raw_Mat(), x2.as_raw_Mat(), f.as_raw_Mat()) }.into_result()
}

/// Point conditioning (non isotropic).
/// ## Parameters
/// * points: Input vector of N-dimensional points.
/// * T: Output 3x3 transformation matrix.
///
/// Computes the transformation matrix such that the two principal moments of the set of points are equal to unity,
/// forming an approximately symmetric circular cloud of points of radius 1 about the origin.\n
/// Reference: [HartleyZ00](https://docs.opencv.org/3.4.7/d0/de3/citelist.html#CITEREF_HartleyZ00) 4.4.4 pag.109
pub fn preconditioner_from_points(points: &core::Mat, t: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_sfm_preconditionerFromPoints_Mat_Mat(points.as_raw_Mat(), t.as_raw_Mat()) }.into_result()
}

/// Get projection matrix P from K, R and t.
/// ## Parameters
/// * K: Input 3x3 camera matrix <span lang='latex'>K = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{1}</span>.
/// * R: Input 3x3 rotation matrix.
/// * t: Input 3x1 translation vector.
/// * P: Output 3x4 projection matrix.
///
/// This function estimate the projection matrix by solving the following equation: <span lang='latex'>P = K * [R|t]</span>
pub fn projection_from_k_rt(k: &core::Mat, r: &core::Mat, t: &core::Mat, p: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_sfm_projectionFromKRt_Mat_Mat_Mat_Mat(k.as_raw_Mat(), r.as_raw_Mat(), t.as_raw_Mat(), p.as_raw_Mat()) }.into_result()
}

/// Get projection matrices from Fundamental matrix
/// ## Parameters
/// * F: Input 3x3 fundamental matrix.
/// * P1: Output 3x4 one possible projection matrix.
/// * P2: Output 3x4 another possible projection matrix.
pub fn projections_from_fundamental(f: &core::Mat, p1: &mut core::Mat, p2: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_sfm_projectionsFromFundamental_Mat_Mat_Mat(f.as_raw_Mat(), p1.as_raw_Mat(), p2.as_raw_Mat()) }.into_result()
}

/// Reconstruct 3d points from 2d correspondences while performing autocalibration.
/// ## Parameters
/// * points2d: Input vector of vectors of 2d points (the inner vector is per image).
/// * Rs: Output vector of 3x3 rotations of the camera.
/// * Ts: Output vector of 3x1 translations of the camera.
/// * points3d: Output array with estimated 3d points.
/// * K: Input/Output camera matrix <span lang='latex'>K = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{1}</span>. Input parameters used as initial guess.
/// * is_projective: if true, the cameras are supposed to be projective.
///
/// Internally calls libmv simple pipeline routine with some default parameters by instatiating SFMLibmvEuclideanReconstruction class.
///
///
/// Note:
/// - Tracks must be as precise as possible. It does not handle outliers and is very sensible to them.
/// - To see a working example for camera motion reconstruction, check the following tutorial: @ref tutorial_sfm_trajectory_estimation.
///
/// ## C++ default parameters
/// * is_projective: false
pub fn reconstruct(points2d: &types::VectorOfMat, rs: &mut core::Mat, ts: &mut core::Mat, k: &mut core::Mat, points3d: &mut core::Mat, is_projective: bool) -> Result<()> {
    unsafe { sys::cv_sfm_reconstruct_VectorOfMat_Mat_Mat_Mat_Mat_bool(points2d.as_raw_VectorOfMat(), rs.as_raw_Mat(), ts.as_raw_Mat(), k.as_raw_Mat(), points3d.as_raw_Mat(), is_projective) }.into_result()
}

/// Reconstruct 3d points from 2d correspondences while performing autocalibration.
/// ## Parameters
/// * points2d: Input vector of vectors of 2d points (the inner vector is per image).
/// * Ps: Output vector with the 3x4 projections matrices of each image.
/// * points3d: Output array with estimated 3d points.
/// * K: Input/Output camera matrix <span lang='latex'>K = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{1}</span>. Input parameters used as initial guess.
/// * is_projective: if true, the cameras are supposed to be projective.
///
/// This method calls below signature and extracts projection matrices from estimated K, R and t.
///
///
/// Note:
/// - Tracks must be as precise as possible. It does not handle outliers and is very sensible to them.
///
/// ## C++ default parameters
/// * is_projective: false
pub fn reconstruct_1(points2d: &types::VectorOfMat, ps: &mut core::Mat, points3d: &mut core::Mat, k: &mut core::Mat, is_projective: bool) -> Result<()> {
    unsafe { sys::cv_sfm_reconstruct_VectorOfMat_Mat_Mat_Mat_bool(points2d.as_raw_VectorOfMat(), ps.as_raw_Mat(), points3d.as_raw_Mat(), k.as_raw_Mat(), is_projective) }.into_result()
}

/// Reconstruct 3d points from 2d images while performing autocalibration.
/// ## Parameters
/// * images: a vector of string with the images paths.
/// * Rs: Output vector of 3x3 rotations of the camera.
/// * Ts: Output vector of 3x1 translations of the camera.
/// * points3d: Output array with estimated 3d points.
/// * K: Input/Output camera matrix <span lang='latex'>K = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{1}</span>. Input parameters used as initial guess.
/// * is_projective: if true, the cameras are supposed to be projective.
///
/// Internally calls libmv simple pipeline routine with some default parameters by instatiating SFMLibmvEuclideanReconstruction class.
///
///
/// Note:
/// - The images must be ordered as they were an image sequence. Additionally, each frame should be as close as posible to the previous and posterior.
/// - For now DAISY features are used in order to compute the 2d points tracks and it only works for 3-4 images.
/// - To see a working example for scene reconstruction, check the following tutorial: @ref tutorial_sfm_scene_reconstruction.
///
/// ## C++ default parameters
/// * is_projective: false
pub fn reconstruct_2(images: &types::VectorOfString, rs: &mut core::Mat, ts: &mut core::Mat, k: &mut core::Mat, points3d: &mut core::Mat, is_projective: bool) -> Result<()> {
    unsafe { sys::cv_sfm_reconstruct_VectorOfString_Mat_Mat_Mat_Mat_bool(images.as_raw_VectorOfString(), rs.as_raw_Mat(), ts.as_raw_Mat(), k.as_raw_Mat(), points3d.as_raw_Mat(), is_projective) }.into_result()
}

/// Reconstruct 3d points from 2d images while performing autocalibration.
/// ## Parameters
/// * images: a vector of string with the images paths.
/// * Ps: Output vector with the 3x4 projections matrices of each image.
/// * points3d: Output array with estimated 3d points.
/// * K: Input/Output camera matrix <span lang='latex'>K = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{1}</span>. Input parameters used as initial guess.
/// * is_projective: if true, the cameras are supposed to be projective.
///
/// This method calls below signature and extracts projection matrices from estimated K, R and t.
///
///
/// Note:
/// - The images must be ordered as they were an image sequence. Additionally, each frame should be as close as posible to the previous and posterior.
/// - For now DAISY features are used in order to compute the 2d points tracks and it only works for 3-4 images.
///
/// ## C++ default parameters
/// * is_projective: false
pub fn reconstruct_3(images: &types::VectorOfString, ps: &mut core::Mat, points3d: &mut core::Mat, k: &mut core::Mat, is_projective: bool) -> Result<()> {
    unsafe { sys::cv_sfm_reconstruct_VectorOfString_Mat_Mat_Mat_bool(images.as_raw_VectorOfString(), ps.as_raw_Mat(), points3d.as_raw_Mat(), k.as_raw_Mat(), is_projective) }.into_result()
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
/// If T1 and T2 are the camera motions, the computed relative motion is <span lang='latex'>T = T_2 T_1^{-1}</span>
pub fn relative_camera_motion(r1: &core::Mat, t1: &core::Mat, r2: &core::Mat, t2: &core::Mat, r: &mut core::Mat, t: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_sfm_relativeCameraMotion_Mat_Mat_Mat_Mat_Mat_Mat(r1.as_raw_Mat(), t1.as_raw_Mat(), r2.as_raw_Mat(), t2.as_raw_Mat(), r.as_raw_Mat(), t.as_raw_Mat()) }.into_result()
}

/// Returns the 3x3 skew symmetric matrix of a vector.
/// ## Parameters
/// * x: Input 3x1 vector.
///
/// Reference: [HartleyZ00](https://docs.opencv.org/3.4.7/d0/de3/citelist.html#CITEREF_HartleyZ00), p581, equation (A4.5).
pub fn skew(x: &core::Mat) -> Result<core::Mat> {
    unsafe { sys::cv_sfm_skew_Mat(x.as_raw_Mat()) }.into_result().map(|ptr| core::Mat { ptr })
}

/// Reconstructs bunch of points by triangulation.
/// ## Parameters
/// * points2d: Input vector of vectors of 2d points (the inner vector is per image). Has to be 2 X N.
/// * projection_matrices: Input vector with 3x4 projections matrices of each image.
/// * points3d: Output array with computed 3d points. Is 3 x N.
///
/// Triangulates the 3d position of 2d correspondences between several images.
/// Reference: Internally it uses DLT method [HartleyZ00](https://docs.opencv.org/3.4.7/d0/de3/citelist.html#CITEREF_HartleyZ00) 12.2 pag.312
pub fn triangulate_points(points2d: &types::VectorOfMat, projection_matrices: &types::VectorOfMat, points3d: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_sfm_triangulatePoints_VectorOfMat_VectorOfMat_Mat(points2d.as_raw_VectorOfMat(), projection_matrices.as_raw_VectorOfMat(), points3d.as_raw_Mat()) }.into_result()
}

// Generating impl for trait cv::sfm::BaseSFM (trait)
/// base class BaseSFM declares a common API that would be used in a typical scene reconstruction scenario
pub trait BaseSFM {
    #[inline(always)] fn as_raw_BaseSFM(&self) -> *mut c_void;
    fn run(&mut self, points2d: &types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_sfm_BaseSFM_run_VectorOfMat(self.as_raw_BaseSFM(), points2d.as_raw_VectorOfMat()) }.into_result()
    }
    
    fn run_1(&mut self, points2d: &types::VectorOfMat, k: &mut core::Mat, rs: &mut core::Mat, ts: &mut core::Mat, points3d: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_sfm_BaseSFM_run_VectorOfMat_Mat_Mat_Mat_Mat(self.as_raw_BaseSFM(), points2d.as_raw_VectorOfMat(), k.as_raw_Mat(), rs.as_raw_Mat(), ts.as_raw_Mat(), points3d.as_raw_Mat()) }.into_result()
    }
    
    fn run_2(&mut self, images: &types::VectorOfString) -> Result<()> {
        unsafe { sys::cv_sfm_BaseSFM_run_VectorOfString(self.as_raw_BaseSFM(), images.as_raw_VectorOfString()) }.into_result()
    }
    
    fn run_3(&mut self, images: &types::VectorOfString, k: &mut core::Mat, rs: &mut core::Mat, ts: &mut core::Mat, points3d: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_sfm_BaseSFM_run_VectorOfString_Mat_Mat_Mat_Mat(self.as_raw_BaseSFM(), images.as_raw_VectorOfString(), k.as_raw_Mat(), rs.as_raw_Mat(), ts.as_raw_Mat(), points3d.as_raw_Mat()) }.into_result()
    }
    
    fn get_error(&self) -> Result<f64> {
        unsafe { sys::cv_sfm_BaseSFM_getError_const(self.as_raw_BaseSFM()) }.into_result()
    }
    
    fn get_points(&mut self, points3d: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_sfm_BaseSFM_getPoints_Mat(self.as_raw_BaseSFM(), points3d.as_raw_Mat()) }.into_result()
    }
    
    fn get_intrinsics(&self) -> Result<core::Mat> {
        unsafe { sys::cv_sfm_BaseSFM_getIntrinsics_const(self.as_raw_BaseSFM()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    fn get_cameras(&mut self, rs: &mut core::Mat, ts: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_sfm_BaseSFM_getCameras_Mat_Mat(self.as_raw_BaseSFM(), rs.as_raw_Mat(), ts.as_raw_Mat()) }.into_result()
    }
    
    fn set_reconstruction_options(&mut self, libmv_reconstruction_options: crate::sfm::libmv_ReconstructionOptions) -> Result<()> {
        unsafe { sys::cv_sfm_BaseSFM_setReconstructionOptions_libmv_ReconstructionOptions(self.as_raw_BaseSFM(), libmv_reconstruction_options) }.into_result()
    }
    
    fn set_camera_intrinsic_options(&mut self, libmv_camera_intrinsics_options: crate::sfm::libmv_CameraIntrinsicsOptions) -> Result<()> {
        unsafe { sys::cv_sfm_BaseSFM_setCameraIntrinsicOptions_libmv_CameraIntrinsicsOptions(self.as_raw_BaseSFM(), libmv_camera_intrinsics_options) }.into_result()
    }
    
}

// boxed class cv::sfm::SFMLibmvEuclideanReconstruction
/// SFMLibmvEuclideanReconstruction class provides an interface with the Libmv Structure From Motion pipeline.
pub struct SFMLibmvEuclideanReconstruction {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::sfm::SFMLibmvEuclideanReconstruction {
    fn drop(&mut self) {
        unsafe { sys::cv_SFMLibmvEuclideanReconstruction_delete(self.ptr) };
    }
}
impl crate::sfm::SFMLibmvEuclideanReconstruction {
    #[inline(always)] pub fn as_raw_SFMLibmvEuclideanReconstruction(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for SFMLibmvEuclideanReconstruction {}

impl crate::sfm::BaseSFM for SFMLibmvEuclideanReconstruction {
    #[inline(always)] fn as_raw_BaseSFM(&self) -> *mut c_void { self.ptr }
}

impl SFMLibmvEuclideanReconstruction {

    /// Calls the pipeline in order to perform Eclidean reconstruction.
    /// ## Parameters
    /// * points2d: Input vector of vectors of 2d points (the inner vector is per image).
    ///
    ///
    /// Note:
    /// - Tracks must be as precise as possible. It does not handle outliers and is very sensible to them.
    pub fn run(&mut self, points2d: &types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_run_VectorOfMat(self.as_raw_SFMLibmvEuclideanReconstruction(), points2d.as_raw_VectorOfMat()) }.into_result()
    }
    
    /// Calls the pipeline in order to perform Eclidean reconstruction.
    /// ## Parameters
    /// * points2d: Input vector of vectors of 2d points (the inner vector is per image).
    /// * K: Input/Output camera matrix <span lang='latex'>K = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{1}</span>. Input parameters used as initial guess.
    /// * Rs: Output vector of 3x3 rotations of the camera.
    /// * Ts: Output vector of 3x1 translations of the camera.
    /// * points3d: Output array with estimated 3d points.
    ///
    ///
    /// Note:
    /// - Tracks must be as precise as possible. It does not handle outliers and is very sensible to them.
    pub fn run_1(&mut self, points2d: &types::VectorOfMat, k: &mut core::Mat, rs: &mut core::Mat, ts: &mut core::Mat, points3d: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_run_VectorOfMat_Mat_Mat_Mat_Mat(self.as_raw_SFMLibmvEuclideanReconstruction(), points2d.as_raw_VectorOfMat(), k.as_raw_Mat(), rs.as_raw_Mat(), ts.as_raw_Mat(), points3d.as_raw_Mat()) }.into_result()
    }
    
    /// Calls the pipeline in order to perform Eclidean reconstruction.
    /// ## Parameters
    /// * images: a vector of string with the images paths.
    ///
    ///
    /// Note:
    /// - The images must be ordered as they were an image sequence. Additionally, each frame should be as close as posible to the previous and posterior.
    /// - For now DAISY features are used in order to compute the 2d points tracks and it only works for 3-4 images.
    pub fn run_2(&mut self, images: &types::VectorOfString) -> Result<()> {
        unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_run_VectorOfString(self.as_raw_SFMLibmvEuclideanReconstruction(), images.as_raw_VectorOfString()) }.into_result()
    }
    
    /// Calls the pipeline in order to perform Eclidean reconstruction.
    /// ## Parameters
    /// * images: a vector of string with the images paths.
    /// * K: Input/Output camera matrix <span lang='latex'>K = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{1}</span>. Input parameters used as initial guess.
    /// * Rs: Output vector of 3x3 rotations of the camera.
    /// * Ts: Output vector of 3x1 translations of the camera.
    /// * points3d: Output array with estimated 3d points.
    ///
    ///
    /// Note:
    /// - The images must be ordered as they were an image sequence. Additionally, each frame should be as close as posible to the previous and posterior.
    /// - For now DAISY features are used in order to compute the 2d points tracks and it only works for 3-4 images.
    pub fn run_3(&mut self, images: &types::VectorOfString, k: &mut core::Mat, rs: &mut core::Mat, ts: &mut core::Mat, points3d: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_run_VectorOfString_Mat_Mat_Mat_Mat(self.as_raw_SFMLibmvEuclideanReconstruction(), images.as_raw_VectorOfString(), k.as_raw_Mat(), rs.as_raw_Mat(), ts.as_raw_Mat(), points3d.as_raw_Mat()) }.into_result()
    }
    
    /// Returns the computed reprojection error.
    pub fn get_error(&self) -> Result<f64> {
        unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_getError_const(self.as_raw_SFMLibmvEuclideanReconstruction()) }.into_result()
    }
    
    /// Returns the estimated 3d points.
    /// ## Parameters
    /// * points3d: Output array with estimated 3d points.
    pub fn get_points(&mut self, points3d: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_getPoints_Mat(self.as_raw_SFMLibmvEuclideanReconstruction(), points3d.as_raw_Mat()) }.into_result()
    }
    
    /// Returns the refined camera calibration matrix.
    pub fn get_intrinsics(&self) -> Result<core::Mat> {
        unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_getIntrinsics_const(self.as_raw_SFMLibmvEuclideanReconstruction()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// Returns the estimated camera extrinsic parameters.
    /// ## Parameters
    /// * Rs: Output vector of 3x3 rotations of the camera.
    /// * Ts: Output vector of 3x1 translations of the camera.
    pub fn get_cameras(&mut self, rs: &mut core::Mat, ts: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_getCameras_Mat_Mat(self.as_raw_SFMLibmvEuclideanReconstruction(), rs.as_raw_Mat(), ts.as_raw_Mat()) }.into_result()
    }
    
    /// Setter method for reconstruction options.
    /// ## Parameters
    /// * libmv_reconstruction_options: struct with reconstruction options such as initial keyframes,
    /// automatic keyframe selection, parameters to refine and the verbosity level.
    pub fn set_reconstruction_options(&mut self, libmv_reconstruction_options: crate::sfm::libmv_ReconstructionOptions) -> Result<()> {
        unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_setReconstructionOptions_libmv_ReconstructionOptions(self.as_raw_SFMLibmvEuclideanReconstruction(), libmv_reconstruction_options) }.into_result()
    }
    
    /// Setter method for camera intrinsic options.
    /// ## Parameters
    /// * libmv_camera_intrinsics_options: struct with camera intrinsic options such as camera model and
    /// the internal camera parameters.
    pub fn set_camera_intrinsic_options(&mut self, libmv_camera_intrinsics_options: crate::sfm::libmv_CameraIntrinsicsOptions) -> Result<()> {
        unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_setCameraIntrinsicOptions_libmv_CameraIntrinsicsOptions(self.as_raw_SFMLibmvEuclideanReconstruction(), libmv_camera_intrinsics_options) }.into_result()
    }
    
    /// Creates an instance of the SFMLibmvEuclideanReconstruction class. Initializes Libmv.
    ///
    /// ## C++ default parameters
    /// * camera_instrinsic_options: libmv_CameraIntrinsicsOptions()
    /// * reconstruction_options: libmv_ReconstructionOptions()
    pub fn create(camera_instrinsic_options: crate::sfm::libmv_CameraIntrinsicsOptions, reconstruction_options: crate::sfm::libmv_ReconstructionOptions) -> Result<types::PtrOfSFMLibmvEuclideanReconstruction> {
        unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_create_libmv_CameraIntrinsicsOptions_libmv_ReconstructionOptions(camera_instrinsic_options, reconstruction_options) }.into_result().map(|ptr| types::PtrOfSFMLibmvEuclideanReconstruction { ptr })
    }
    
}

impl libmv_CameraIntrinsicsOptions {

    ///
    /// ## C++ default parameters
    /// * _distortion_model: 0
    /// * _focal_length: 0
    /// * _principal_point_x: 0
    /// * _principal_point_y: 0
    /// * _polynomial_k1: 0
    /// * _polynomial_k2: 0
    /// * _polynomial_k3: 0
    /// * _polynomial_p1: 0
    /// * _polynomial_p2: 0
    pub fn new(_distortion_model: i32, _focal_length: f64, _principal_point_x: f64, _principal_point_y: f64, _polynomial_k1: f64, _polynomial_k2: f64, _polynomial_k3: f64, _polynomial_p1: f64, _polynomial_p2: f64) -> Result<crate::sfm::libmv_CameraIntrinsicsOptions> {
        unsafe { sys::cv_sfm_libmv_CameraIntrinsicsOptions_libmv_CameraIntrinsicsOptions_int_double_double_double_double_double_double_double_double(_distortion_model, _focal_length, _principal_point_x, _principal_point_y, _polynomial_k1, _polynomial_k2, _polynomial_k3, _polynomial_p1, _polynomial_p2) }.into_result()
    }
    
}

impl libmv_ReconstructionOptions {

    ///
    /// ## C++ default parameters
    /// * _keyframe1: 1
    /// * _keyframe2: 2
    /// * _refine_intrinsics: 1
    /// * _select_keyframes: 1
    /// * _verbosity_level: -1
    pub fn new(_keyframe1: i32, _keyframe2: i32, _refine_intrinsics: i32, _select_keyframes: i32, _verbosity_level: i32) -> Result<crate::sfm::libmv_ReconstructionOptions> {
        unsafe { sys::cv_sfm_libmv_ReconstructionOptions_libmv_ReconstructionOptions_int_int_int_int_int(_keyframe1, _keyframe2, _refine_intrinsics, _select_keyframes, _verbosity_level) }.into_result()
    }
    
}
