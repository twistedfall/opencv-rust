//! # Camera Calibration and 3D Reconstruction
//! 
//! The functions in this section use a so-called pinhole camera model. In this model, a scene view is
//! formed by projecting 3D points into the image plane using a perspective transformation.
//! 
//! <div lang='latex'>s  \; m' = A [R|t] M'</div>
//! 
//! or
//! 
//! <div lang='latex'>s  \vecthree{u}{v}{1} = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{1}
//! \begin{bmatrix}
//! r_{11} & r_{12} & r_{13} & t_1  \\
//! r_{21} & r_{22} & r_{23} & t_2  \\
//! r_{31} & r_{32} & r_{33} & t_3
//! \end{bmatrix}
//! \begin{bmatrix}
//! X \\
//! Y \\
//! Z \\
//! 1
//! \end{bmatrix}</div>
//! 
//! where:
//! 
//! *   <span lang='latex'>(X, Y, Z)</span> are the coordinates of a 3D point in the world coordinate space
//! *   <span lang='latex'>(u, v)</span> are the coordinates of the projection point in pixels
//! *   <span lang='latex'>A</span> is a camera matrix, or a matrix of intrinsic parameters
//! *   <span lang='latex'>(cx, cy)</span> is a principal point that is usually at the image center
//! *   <span lang='latex'>fx, fy</span> are the focal lengths expressed in pixel units.
//! 
//! Thus, if an image from the camera is scaled by a factor, all of these parameters should be scaled
//! (multiplied/divided, respectively) by the same factor. The matrix of intrinsic parameters does not
//! depend on the scene viewed. So, once estimated, it can be re-used as long as the focal length is
//! fixed (in case of zoom lens). The joint rotation-translation matrix <span lang='latex'>[R|t]</span> is called a matrix of
//! extrinsic parameters. It is used to describe the camera motion around a static scene, or vice versa,
//! rigid motion of an object in front of a still camera. That is, <span lang='latex'>[R|t]</span> translates coordinates of a
//! point <span lang='latex'>(X, Y, Z)</span> to a coordinate system, fixed with respect to the camera. The transformation above
//! is equivalent to the following (when <span lang='latex'>z \ne 0</span> ):
//! 
//! <div lang='latex'>\begin{array}{l}
//! \vecthree{x}{y}{z} = R  \vecthree{X}{Y}{Z} + t \\
//! x' = x/z \\
//! y' = y/z \\
//! u = f_x*x' + c_x \\
//! v = f_y*y' + c_y
//! \end{array}</div>
//! 
//! The following figure illustrates the pinhole camera model.
//! 
//! ![Pinhole camera model](https://docs.opencv.org/3.4.6/pinhole_camera_model.png)
//! 
//! Real lenses usually have some distortion, mostly radial distortion and slight tangential distortion.
//! So, the above model is extended as:
//! 
//! <div lang='latex'>\begin{array}{l}
//! \vecthree{x}{y}{z} = R  \vecthree{X}{Y}{Z} + t \\
//! x' = x/z \\
//! y' = y/z \\
//! x'' = x'  \frac{1 + k_1 r^2 + k_2 r^4 + k_3 r^6}{1 + k_4 r^2 + k_5 r^4 + k_6 r^6} + 2 p_1 x' y' + p_2(r^2 + 2 x'^2) + s_1 r^2 + s_2 r^4 \\
//! y'' = y'  \frac{1 + k_1 r^2 + k_2 r^4 + k_3 r^6}{1 + k_4 r^2 + k_5 r^4 + k_6 r^6} + p_1 (r^2 + 2 y'^2) + 2 p_2 x' y' + s_3 r^2 + s_4 r^4 \\
//! \text{where} \quad r^2 = x'^2 + y'^2  \\
//! u = f_x*x'' + c_x \\
//! v = f_y*y'' + c_y
//! \end{array}</div>
//! 
//! <span lang='latex'>k_1</span>, <span lang='latex'>k_2</span>, <span lang='latex'>k_3</span>, <span lang='latex'>k_4</span>, <span lang='latex'>k_5</span>, and <span lang='latex'>k_6</span> are radial distortion coefficients. <span lang='latex'>p_1</span> and <span lang='latex'>p_2</span> are
//! tangential distortion coefficients. <span lang='latex'>s_1</span>, <span lang='latex'>s_2</span>, <span lang='latex'>s_3</span>, and <span lang='latex'>s_4</span>, are the thin prism distortion
//! coefficients. Higher-order coefficients are not considered in OpenCV.
//! 
//! The next figures show two common types of radial distortion: barrel distortion (typically <span lang='latex'> k_1 < 0 </span>) and pincushion distortion (typically <span lang='latex'> k_1 > 0 </span>).
//! 
//! ![](https://docs.opencv.org/3.4.6/distortion_examples.png)
//! ![](https://docs.opencv.org/3.4.6/distortion_examples2.png)
//! 
//! In some cases the image sensor may be tilted in order to focus an oblique plane in front of the
//! camera (Scheimpfug condition). This can be useful for particle image velocimetry (PIV) or
//! triangulation with a laser fan. The tilt causes a perspective distortion of <span lang='latex'>x''</span> and
//! <span lang='latex'>y''</span>. This distortion can be modelled in the following way, see e.g. [Louhichi07](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Louhichi07).
//! 
//! <div lang='latex'>\begin{array}{l}
//! s\vecthree{x'''}{y'''}{1} =
//! \vecthreethree{R_{33}(\tau_x, \tau_y)}{0}{-R_{13}(\tau_x, \tau_y)}
//! {0}{R_{33}(\tau_x, \tau_y)}{-R_{23}(\tau_x, \tau_y)}
//! {0}{0}{1} R(\tau_x, \tau_y) \vecthree{x''}{y''}{1}\\
//! u = f_x*x''' + c_x \\
//! v = f_y*y''' + c_y
//! \end{array}</div>
//! 
//! where the matrix <span lang='latex'>R(\tau_x, \tau_y)</span> is defined by two rotations with angular parameter <span lang='latex'>\tau_x</span>
//! and <span lang='latex'>\tau_y</span>, respectively,
//! 
//! <div lang='latex'>
//! R(\tau_x, \tau_y) =
//! \vecthreethree{\cos(\tau_y)}{0}{-\sin(\tau_y)}{0}{1}{0}{\sin(\tau_y)}{0}{\cos(\tau_y)}
//! \vecthreethree{1}{0}{0}{0}{\cos(\tau_x)}{\sin(\tau_x)}{0}{-\sin(\tau_x)}{\cos(\tau_x)} =
//! \vecthreethree{\cos(\tau_y)}{\sin(\tau_y)\sin(\tau_x)}{-\sin(\tau_y)\cos(\tau_x)}
//! {0}{\cos(\tau_x)}{\sin(\tau_x)}
//! {\sin(\tau_y)}{-\cos(\tau_y)\sin(\tau_x)}{\cos(\tau_y)\cos(\tau_x)}.
//! </div>
//! 
//! In the functions below the coefficients are passed or returned as
//! 
//! <div lang='latex'>(k_1, k_2, p_1, p_2[, k_3[, k_4, k_5, k_6 [, s_1, s_2, s_3, s_4[, \tau_x, \tau_y]]]])</div>
//! 
//! vector. That is, if the vector contains four elements, it means that <span lang='latex'>k_3=0</span> . The distortion
//! coefficients do not depend on the scene viewed. Thus, they also belong to the intrinsic camera
//! parameters. And they remain the same regardless of the captured image resolution. If, for example, a
//! camera has been calibrated on images of 320 x 240 resolution, absolutely the same distortion
//! coefficients can be used for 640 x 480 images from the same camera while <span lang='latex'>f_x</span>, <span lang='latex'>f_y</span>, <span lang='latex'>c_x</span>, and
//! <span lang='latex'>c_y</span> need to be scaled appropriately.
//! 
//! The functions below use the above model to do the following:
//! 
//! *   Project 3D points to the image plane given intrinsic and extrinsic parameters.
//! *   Compute extrinsic parameters given intrinsic parameters, a few 3D points, and their
//! projections.
//! *   Estimate intrinsic and extrinsic camera parameters from several views of a known calibration
//! pattern (every view is described by several 3D-2D point correspondences).
//! *   Estimate the relative position and orientation of the stereo camera "heads" and compute the
//! *rectification* transformation that makes the camera optical axes parallel.
//! 
//! 
//! Note:
//! *   A calibration sample for 3 cameras in horizontal position can be found at
//! opencv_source_code/samples/cpp/3calibration.cpp
//! *   A calibration sample based on a sequence of images can be found at
//! opencv_source_code/samples/cpp/calibration.cpp
//! *   A calibration sample in order to do 3D reconstruction can be found at
//! opencv_source_code/samples/cpp/build3dmodel.cpp
//! *   A calibration example on stereo calibration can be found at
//! opencv_source_code/samples/cpp/stereo_calib.cpp
//! *   A calibration example on stereo matching can be found at
//! opencv_source_code/samples/cpp/stereo_match.cpp
//! *   (Python) A camera calibration sample can be found at
//! opencv_source_code/samples/python/calibrate.py
//! # Fisheye camera model
//! 
//! Definitions: Let P be a point in 3D of coordinates X in the world reference frame (stored in the
//! matrix X) The coordinate vector of P in the camera reference frame is:
//! 
//! <div lang='latex'>Xc = R X + T</div>
//! 
//! where R is the rotation matrix corresponding to the rotation vector om: R = rodrigues(om); call x, y
//! and z the 3 coordinates of Xc:
//! 
//! <div lang='latex'>x = Xc_1 \\ y = Xc_2 \\ z = Xc_3</div>
//! 
//! The pinhole projection coordinates of P is [a; b] where
//! 
//! <div lang='latex'>a = x / z \ and \ b = y / z \\ r^2 = a^2 + b^2 \\ \theta = atan(r)</div>
//! 
//! Fisheye distortion:
//! 
//! <div lang='latex'>\theta_d = \theta (1 + k_1 \theta^2 + k_2 \theta^4 + k_3 \theta^6 + k_4 \theta^8)</div>
//! 
//! The distorted point coordinates are [x'; y'] where
//! 
//! <div lang='latex'>x' = (\theta_d / r) a \\ y' = (\theta_d / r) b </div>
//! 
//! Finally, conversion into pixel coordinates: The final pixel coordinates vector [u; v] where:
//! 
//! <div lang='latex'>u = f_x (x' + \alpha y') + c_x \\
//! v = f_y y' + c_y</div>
//! 
//! # C API
use std::os::raw::{c_char, c_void};
use libc::size_t;
use crate::{Error, Result, core, sys, types};

pub const CALIB_CB_ADAPTIVE_THRESH: i32 = 1;
pub const CALIB_CB_ASYMMETRIC_GRID: i32 = 2;
pub const CALIB_CB_CLUSTERING: i32 = 4;
pub const CALIB_CB_FAST_CHECK: i32 = 8;
pub const CALIB_CB_FILTER_QUADS: i32 = 4;
pub const CALIB_CB_NORMALIZE_IMAGE: i32 = 2;
pub const CALIB_CB_SYMMETRIC_GRID: i32 = 1;
pub const CALIB_CHECK_COND: i32 = 1 << 2;
pub const CALIB_FIX_ASPECT_RATIO: i32 = 0x00002;
pub const CALIB_FIX_FOCAL_LENGTH: i32 = 0x00010;
pub const CALIB_FIX_INTRINSIC: i32 = 0x00100;
pub const CALIB_FIX_K1: i32 = 0x00020;
pub const CALIB_FIX_K2: i32 = 0x00040;
pub const CALIB_FIX_K3: i32 = 0x00080;
pub const CALIB_FIX_K4: i32 = 0x00800;
pub const CALIB_FIX_K5: i32 = 0x01000;
pub const CALIB_FIX_K6: i32 = 0x02000;
pub const CALIB_FIX_PRINCIPAL_POINT: i32 = 0x00004;
pub const CALIB_FIX_S1_S2_S3_S4: i32 = 0x10000;
pub const CALIB_FIX_SKEW: i32 = 1 << 3;
pub const CALIB_FIX_TANGENT_DIST: i32 = 0x200000;
pub const CALIB_FIX_TAUX_TAUY: i32 = 0x80000;
pub const CALIB_HAND_EYE_ANDREFF: i32 = 3;
pub const CALIB_HAND_EYE_DANIILIDIS: i32 = 4;
pub const CALIB_HAND_EYE_HORAUD: i32 = 2;
pub const CALIB_HAND_EYE_PARK: i32 = 1;
pub const CALIB_HAND_EYE_TSAI: i32 = 0;
pub const CALIB_RATIONAL_MODEL: i32 = 0x04000;
pub const CALIB_RECOMPUTE_EXTRINSIC: i32 = 1 << 1;
pub const CALIB_SAME_FOCAL_LENGTH: i32 = 0x00200;
pub const CALIB_THIN_PRISM_MODEL: i32 = 0x08000;
pub const CALIB_TILTED_MODEL: i32 = 0x40000;
pub const CALIB_USE_EXTRINSIC_GUESS: i32 = (1 << 22);
pub const CALIB_USE_INTRINSIC_GUESS: i32 = 0x00001;
pub const CALIB_USE_LU: i32 = (1 << 17);
pub const CALIB_USE_QR: i32 = 0x100000;
pub const CALIB_ZERO_DISPARITY: i32 = 0x00400;
pub const CALIB_ZERO_TANGENT_DIST: i32 = 0x00008;
pub const CirclesGridFinderParameters_ASYMMETRIC_GRID: i32 = 1;
pub const CirclesGridFinderParameters_SYMMETRIC_GRID: i32 = 0;
pub const FM_7POINT: i32 = 1;
pub const FM_8POINT: i32 = 2;
pub const FM_LMEDS: i32 = 4;
pub const FM_RANSAC: i32 = 8;
pub const LMEDS: i32 = 4;
pub const RANSAC: i32 = 8;
pub const RHO: i32 = 16;
pub const SOLVEPNP_AP3P: i32 = 5;
pub const SOLVEPNP_DLS: i32 = 3;
pub const SOLVEPNP_EPNP: i32 = 1;
pub const SOLVEPNP_ITERATIVE: i32 = 0;
pub const SOLVEPNP_MAX_COUNT: i32 = 5+1;
pub const SOLVEPNP_P3P: i32 = 2;
pub const SOLVEPNP_UPNP: i32 = 4;
pub const StereoBM_PREFILTER_NORMALIZED_RESPONSE: i32 = 0;
pub const StereoBM_PREFILTER_XSOBEL: i32 = 1;
pub const StereoMatcher_DISP_SHIFT: i32 = 4;
pub const StereoSGBM_MODE_HH: i32 = 1;
pub const StereoSGBM_MODE_HH4: i32 = 3;
pub const StereoSGBM_MODE_SGBM: i32 = 0;
pub const StereoSGBM_MODE_SGBM_3WAY: i32 = 2;


#[repr(C)]
#[derive(Copy,Clone,Debug,PartialEq)]
pub struct CirclesGridFinderParameters {
    pub density_neighborhood_size: core::Size2f,
    pub min_density: f32,
    pub kmeans_attempts: i32,
    pub min_distance_to_add_keypoint: i32,
    pub keypoint_scale: i32,
    pub min_graph_confidence: f32,
    pub vertex_gain: f32,
    pub vertex_penalty: f32,
    pub existing_vertex_gain: f32,
    pub edge_gain: f32,
    pub edge_penalty: f32,
    pub convex_hull_factor: f32,
    pub min_rng_edge_switch_dist: f32,
}


#[repr(C)]
#[derive(Copy,Clone,Debug,PartialEq)]
pub struct CirclesGridFinderParameters2 {
    pub square_size: f32,
    pub max_rectified_distance: f32,
}

/// Computes an RQ decomposition of 3x3 matrices.
/// 
/// ## Parameters
/// * src: 3x3 input matrix.
/// * mtxR: Output 3x3 upper-triangular matrix.
/// * mtxQ: Output 3x3 orthogonal matrix.
/// * Qx: Optional output 3x3 rotation matrix around x-axis.
/// * Qy: Optional output 3x3 rotation matrix around y-axis.
/// * Qz: Optional output 3x3 rotation matrix around z-axis.
/// 
/// The function computes a RQ decomposition using the given rotations. This function is used in
/// decomposeProjectionMatrix to decompose the left 3x3 submatrix of a projection matrix into a camera
/// and a rotation matrix.
/// 
/// It optionally returns three rotation matrices, one for each axis, and the three Euler angles in
/// degrees (as the return value) that could be used in OpenGL. Note, there is always more than one
/// sequence of rotations about the three principal axes that results in the same orientation of an
/// object, e.g. see [Slabaugh](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Slabaugh) . Returned tree rotation matrices and corresponding three Euler angles
/// are only one of the possible solutions.
///
/// ## C++ default parameters
/// * qx: noArray()
/// * qy: noArray()
/// * qz: noArray()
pub fn rq_decomp3x3(src: &core::Mat, mtx_r: &mut core::Mat, mtx_q: &mut core::Mat, qx: &mut core::Mat, qy: &mut core::Mat, qz: &mut core::Mat) -> Result<core::Vec3d> {
    unsafe { sys::cv_RQDecomp3x3_Mat_Mat_Mat_Mat_Mat_Mat(src.as_raw_Mat(), mtx_r.as_raw_Mat(), mtx_q.as_raw_Mat(), qx.as_raw_Mat(), qy.as_raw_Mat(), qz.as_raw_Mat()) }.into_result()
}

/// Converts a rotation matrix to a rotation vector or vice versa.
/// 
/// ## Parameters
/// * src: Input rotation vector (3x1 or 1x3) or rotation matrix (3x3).
/// * dst: Output rotation matrix (3x3) or rotation vector (3x1 or 1x3), respectively.
/// * jacobian: Optional output Jacobian matrix, 3x9 or 9x3, which is a matrix of partial
/// derivatives of the output array components with respect to the input array components.
/// 
/// <div lang='latex'>\begin{array}{l} \theta \leftarrow norm(r) \\ r  \leftarrow r/ \theta \\ R =  \cos{\theta} I + (1- \cos{\theta} ) r r^T +  \sin{\theta} \vecthreethree{0}{-r_z}{r_y}{r_z}{0}{-r_x}{-r_y}{r_x}{0} \end{array}</div>
/// 
/// Inverse transformation can be also done easily, since
/// 
/// <div lang='latex'>\sin ( \theta ) \vecthreethree{0}{-r_z}{r_y}{r_z}{0}{-r_x}{-r_y}{r_x}{0} = \frac{R - R^T}{2}</div>
/// 
/// A rotation vector is a convenient and most compact representation of a rotation matrix (since any
/// rotation matrix has just 3 degrees of freedom). The representation is used in the global 3D geometry
/// optimization procedures like calibrateCamera, stereoCalibrate, or solvePnP .
///
/// ## C++ default parameters
/// * jacobian: noArray()
pub fn rodrigues(src: &core::Mat, dst: &mut core::Mat, jacobian: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_Rodrigues_Mat_Mat_Mat(src.as_raw_Mat(), dst.as_raw_Mat(), jacobian.as_raw_Mat()) }.into_result()
}

/// Finds the camera intrinsic and extrinsic parameters from several views of a calibration pattern.
/// 
/// ## Parameters
/// * objectPoints: In the new interface it is a vector of vectors of calibration pattern points in
/// the calibration pattern coordinate space (e.g. std::vector<std::vector<cv::Vec3f>>). The outer
/// vector contains as many elements as the number of the pattern views. If the same calibration pattern
/// is shown in each view and it is fully visible, all the vectors will be the same. Although, it is
/// possible to use partially occluded patterns, or even different patterns in different views. Then,
/// the vectors will be different. The points are 3D, but since they are in a pattern coordinate system,
/// then, if the rig is planar, it may make sense to put the model to a XY coordinate plane so that
/// Z-coordinate of each input object point is 0.
/// In the old interface all the vectors of object points from different views are concatenated
/// together.
/// * imagePoints: In the new interface it is a vector of vectors of the projections of calibration
/// pattern points (e.g. std::vector<std::vector<cv::Vec2f>>). imagePoints.size() and
/// objectPoints.size() and imagePoints[i].size() must be equal to objectPoints[i].size() for each i.
/// In the old interface all the vectors of object points from different views are concatenated
/// together.
/// * imageSize: Size of the image used only to initialize the intrinsic camera matrix.
/// * cameraMatrix: Output 3x3 floating-point camera matrix
/// <span lang='latex'>A = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{1}</span> . If CV\_CALIB\_USE\_INTRINSIC\_GUESS
/// and/or CALIB_FIX_ASPECT_RATIO are specified, some or all of fx, fy, cx, cy must be
/// initialized before calling the function.
/// * distCoeffs: Output vector of distortion coefficients
/// <span lang='latex'>(k_1, k_2, p_1, p_2[, k_3[, k_4, k_5, k_6 [, s_1, s_2, s_3, s_4[, \tau_x, \tau_y]]]])</span> of
/// 4, 5, 8, 12 or 14 elements.
/// * rvecs: Output vector of rotation vectors (see Rodrigues ) estimated for each pattern view
/// (e.g. std::vector<cv::Mat>>). That is, each k-th rotation vector together with the corresponding
/// k-th translation vector (see the next output parameter description) brings the calibration pattern
/// from the model coordinate space (in which object points are specified) to the world coordinate
/// space, that is, a real position of the calibration pattern in the k-th pattern view (k=0.. *M* -1).
/// * tvecs: Output vector of translation vectors estimated for each pattern view.
/// * stdDeviationsIntrinsics: Output vector of standard deviations estimated for intrinsic parameters.
/// Order of deviations values:
/// \f$(f_x, f_y, c_x, c_y, k_1, k_2, p_1, p_2, k_3, k_4, k_5, k_6 , s_1, s_2, s_3,
/// s_4, \tau_x, \tau_y)\f$ If one of parameters is not estimated, it's deviation is equals to zero.
/// * stdDeviationsExtrinsics: Output vector of standard deviations estimated for extrinsic parameters.
/// Order of deviations values: <span lang='latex'>(R_1, T_1, \dotsc , R_M, T_M)</span> where M is number of pattern views,
/// <span lang='latex'>R_i, T_i</span> are concatenated 1x3 vectors.
/// * perViewErrors: Output vector of the RMS re-projection error estimated for each pattern view.
/// * flags: Different flags that may be zero or a combination of the following values:
/// *   **CALIB_USE_INTRINSIC_GUESS** cameraMatrix contains valid initial values of
/// fx, fy, cx, cy that are optimized further. Otherwise, (cx, cy) is initially set to the image
/// center ( imageSize is used), and focal distances are computed in a least-squares fashion.
/// Note, that if intrinsic parameters are known, there is no need to use this function just to
/// estimate extrinsic parameters. Use solvePnP instead.
/// *   **CALIB_FIX_PRINCIPAL_POINT** The principal point is not changed during the global
/// optimization. It stays at the center or at a different location specified when
/// CALIB_USE_INTRINSIC_GUESS is set too.
/// *   **CALIB_FIX_ASPECT_RATIO** The functions considers only fy as a free parameter. The
/// ratio fx/fy stays the same as in the input cameraMatrix . When
/// CALIB_USE_INTRINSIC_GUESS is not set, the actual input values of fx and fy are
/// ignored, only their ratio is computed and used further.
/// *   **CALIB_ZERO_TANGENT_DIST** Tangential distortion coefficients <span lang='latex'>(p_1, p_2)</span> are set
/// to zeros and stay zero.
/// *   **CALIB_FIX_K1,...,CALIB_FIX_K6** The corresponding radial distortion
/// coefficient is not changed during the optimization. If CALIB_USE_INTRINSIC_GUESS is
/// set, the coefficient from the supplied distCoeffs matrix is used. Otherwise, it is set to 0.
/// *   **CALIB_RATIONAL_MODEL** Coefficients k4, k5, and k6 are enabled. To provide the
/// backward compatibility, this extra flag should be explicitly specified to make the
/// calibration function use the rational model and return 8 coefficients. If the flag is not
/// set, the function computes and returns only 5 distortion coefficients.
/// *   **CALIB_THIN_PRISM_MODEL** Coefficients s1, s2, s3 and s4 are enabled. To provide the
/// backward compatibility, this extra flag should be explicitly specified to make the
/// calibration function use the thin prism model and return 12 coefficients. If the flag is not
/// set, the function computes and returns only 5 distortion coefficients.
/// *   **CALIB_FIX_S1_S2_S3_S4** The thin prism distortion coefficients are not changed during
/// the optimization. If CALIB_USE_INTRINSIC_GUESS is set, the coefficient from the
/// supplied distCoeffs matrix is used. Otherwise, it is set to 0.
/// *   **CALIB_TILTED_MODEL** Coefficients tauX and tauY are enabled. To provide the
/// backward compatibility, this extra flag should be explicitly specified to make the
/// calibration function use the tilted sensor model and return 14 coefficients. If the flag is not
/// set, the function computes and returns only 5 distortion coefficients.
/// *   **CALIB_FIX_TAUX_TAUY** The coefficients of the tilted sensor model are not changed during
/// the optimization. If CALIB_USE_INTRINSIC_GUESS is set, the coefficient from the
/// supplied distCoeffs matrix is used. Otherwise, it is set to 0.
/// * criteria: Termination criteria for the iterative optimization algorithm.
/// 
/// ## Returns
/// the overall RMS re-projection error.
/// 
/// The function estimates the intrinsic camera parameters and extrinsic parameters for each of the
/// views. The algorithm is based on [Zhang2000](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Zhang2000) and [BouguetMCT](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_BouguetMCT) . The coordinates of 3D object
/// points and their corresponding 2D projections in each view must be specified. That may be achieved
/// by using an object with a known geometry and easily detectable feature points. Such an object is
/// called a calibration rig or calibration pattern, and OpenCV has built-in support for a chessboard as
/// a calibration rig (see findChessboardCorners ). Currently, initialization of intrinsic parameters
/// (when CALIB_USE_INTRINSIC_GUESS is not set) is only implemented for planar calibration
/// patterns (where Z-coordinates of the object points must be all zeros). 3D calibration rigs can also
/// be used as long as initial cameraMatrix is provided.
/// 
/// The algorithm performs the following steps:
/// 
/// *   Compute the initial intrinsic parameters (the option only available for planar calibration
/// patterns) or read them from the input parameters. The distortion coefficients are all set to
/// zeros initially unless some of CALIB_FIX_K? are specified.
/// 
/// *   Estimate the initial camera pose as if the intrinsic parameters have been already known. This is
/// done using solvePnP .
/// 
/// *   Run the global Levenberg-Marquardt optimization algorithm to minimize the reprojection error,
/// that is, the total sum of squared distances between the observed feature points imagePoints and
/// the projected (using the current estimates for camera parameters and the poses) object points
/// objectPoints. See projectPoints for details.
/// 
/// 
/// Note:
/// If you use a non-square (=non-NxN) grid and findChessboardCorners for calibration, and
/// calibrateCamera returns bad values (zero distortion coefficients, an image center very far from
/// (w/2-0.5,h/2-0.5), and/or large differences between <span lang='latex'>f_x</span> and <span lang='latex'>f_y</span> (ratios of 10:1 or more)),
/// then you have probably used patternSize=cvSize(rows,cols) instead of using
/// patternSize=cvSize(cols,rows) in findChessboardCorners .
/// 
/// ## See also
/// findChessboardCorners, solvePnP, initCameraMatrix2D, stereoCalibrate, undistort
///
/// ## C++ default parameters
/// * flags: 0
/// * criteria: TermCriteria( TermCriteria::COUNT + TermCriteria::EPS, 30, DBL_EPSILON)
pub fn calibrate_camera_with_stddev(object_points: &types::VectorOfMat, image_points: &types::VectorOfMat, image_size: core::Size, camera_matrix: &mut core::Mat, dist_coeffs: &mut core::Mat, rvecs: &mut types::VectorOfMat, tvecs: &mut types::VectorOfMat, std_deviations_intrinsics: &mut core::Mat, std_deviations_extrinsics: &mut core::Mat, per_view_errors: &mut core::Mat, flags: i32, criteria: &core::TermCriteria) -> Result<f64> {
    unsafe { sys::cv_calibrateCamera_VectorOfMat_VectorOfMat_Size_Mat_Mat_VectorOfMat_VectorOfMat_Mat_Mat_Mat_int_TermCriteria(object_points.as_raw_VectorOfMat(), image_points.as_raw_VectorOfMat(), image_size, camera_matrix.as_raw_Mat(), dist_coeffs.as_raw_Mat(), rvecs.as_raw_VectorOfMat(), tvecs.as_raw_VectorOfMat(), std_deviations_intrinsics.as_raw_Mat(), std_deviations_extrinsics.as_raw_Mat(), per_view_errors.as_raw_Mat(), flags, criteria.as_raw_TermCriteria()) }.into_result()
}

/// Finds the camera intrinsic and extrinsic parameters from several views of a calibration pattern.
/// 
/// ## Parameters
/// * objectPoints: In the new interface it is a vector of vectors of calibration pattern points in
/// the calibration pattern coordinate space (e.g. std::vector<std::vector<cv::Vec3f>>). The outer
/// vector contains as many elements as the number of the pattern views. If the same calibration pattern
/// is shown in each view and it is fully visible, all the vectors will be the same. Although, it is
/// possible to use partially occluded patterns, or even different patterns in different views. Then,
/// the vectors will be different. The points are 3D, but since they are in a pattern coordinate system,
/// then, if the rig is planar, it may make sense to put the model to a XY coordinate plane so that
/// Z-coordinate of each input object point is 0.
/// In the old interface all the vectors of object points from different views are concatenated
/// together.
/// * imagePoints: In the new interface it is a vector of vectors of the projections of calibration
/// pattern points (e.g. std::vector<std::vector<cv::Vec2f>>). imagePoints.size() and
/// objectPoints.size() and imagePoints[i].size() must be equal to objectPoints[i].size() for each i.
/// In the old interface all the vectors of object points from different views are concatenated
/// together.
/// * imageSize: Size of the image used only to initialize the intrinsic camera matrix.
/// * cameraMatrix: Output 3x3 floating-point camera matrix
/// <span lang='latex'>A = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{1}</span> . If CV\_CALIB\_USE\_INTRINSIC\_GUESS
/// and/or CALIB_FIX_ASPECT_RATIO are specified, some or all of fx, fy, cx, cy must be
/// initialized before calling the function.
/// * distCoeffs: Output vector of distortion coefficients
/// <span lang='latex'>(k_1, k_2, p_1, p_2[, k_3[, k_4, k_5, k_6 [, s_1, s_2, s_3, s_4[, \tau_x, \tau_y]]]])</span> of
/// 4, 5, 8, 12 or 14 elements.
/// * rvecs: Output vector of rotation vectors (see Rodrigues ) estimated for each pattern view
/// (e.g. std::vector<cv::Mat>>). That is, each k-th rotation vector together with the corresponding
/// k-th translation vector (see the next output parameter description) brings the calibration pattern
/// from the model coordinate space (in which object points are specified) to the world coordinate
/// space, that is, a real position of the calibration pattern in the k-th pattern view (k=0.. *M* -1).
/// * tvecs: Output vector of translation vectors estimated for each pattern view.
/// * stdDeviationsIntrinsics: Output vector of standard deviations estimated for intrinsic parameters.
/// Order of deviations values:
/// \f$(f_x, f_y, c_x, c_y, k_1, k_2, p_1, p_2, k_3, k_4, k_5, k_6 , s_1, s_2, s_3,
/// s_4, \tau_x, \tau_y)\f$ If one of parameters is not estimated, it's deviation is equals to zero.
/// * stdDeviationsExtrinsics: Output vector of standard deviations estimated for extrinsic parameters.
/// Order of deviations values: <span lang='latex'>(R_1, T_1, \dotsc , R_M, T_M)</span> where M is number of pattern views,
/// <span lang='latex'>R_i, T_i</span> are concatenated 1x3 vectors.
/// * perViewErrors: Output vector of the RMS re-projection error estimated for each pattern view.
/// * flags: Different flags that may be zero or a combination of the following values:
/// *   **CALIB_USE_INTRINSIC_GUESS** cameraMatrix contains valid initial values of
/// fx, fy, cx, cy that are optimized further. Otherwise, (cx, cy) is initially set to the image
/// center ( imageSize is used), and focal distances are computed in a least-squares fashion.
/// Note, that if intrinsic parameters are known, there is no need to use this function just to
/// estimate extrinsic parameters. Use solvePnP instead.
/// *   **CALIB_FIX_PRINCIPAL_POINT** The principal point is not changed during the global
/// optimization. It stays at the center or at a different location specified when
/// CALIB_USE_INTRINSIC_GUESS is set too.
/// *   **CALIB_FIX_ASPECT_RATIO** The functions considers only fy as a free parameter. The
/// ratio fx/fy stays the same as in the input cameraMatrix . When
/// CALIB_USE_INTRINSIC_GUESS is not set, the actual input values of fx and fy are
/// ignored, only their ratio is computed and used further.
/// *   **CALIB_ZERO_TANGENT_DIST** Tangential distortion coefficients <span lang='latex'>(p_1, p_2)</span> are set
/// to zeros and stay zero.
/// *   **CALIB_FIX_K1,...,CALIB_FIX_K6** The corresponding radial distortion
/// coefficient is not changed during the optimization. If CALIB_USE_INTRINSIC_GUESS is
/// set, the coefficient from the supplied distCoeffs matrix is used. Otherwise, it is set to 0.
/// *   **CALIB_RATIONAL_MODEL** Coefficients k4, k5, and k6 are enabled. To provide the
/// backward compatibility, this extra flag should be explicitly specified to make the
/// calibration function use the rational model and return 8 coefficients. If the flag is not
/// set, the function computes and returns only 5 distortion coefficients.
/// *   **CALIB_THIN_PRISM_MODEL** Coefficients s1, s2, s3 and s4 are enabled. To provide the
/// backward compatibility, this extra flag should be explicitly specified to make the
/// calibration function use the thin prism model and return 12 coefficients. If the flag is not
/// set, the function computes and returns only 5 distortion coefficients.
/// *   **CALIB_FIX_S1_S2_S3_S4** The thin prism distortion coefficients are not changed during
/// the optimization. If CALIB_USE_INTRINSIC_GUESS is set, the coefficient from the
/// supplied distCoeffs matrix is used. Otherwise, it is set to 0.
/// *   **CALIB_TILTED_MODEL** Coefficients tauX and tauY are enabled. To provide the
/// backward compatibility, this extra flag should be explicitly specified to make the
/// calibration function use the tilted sensor model and return 14 coefficients. If the flag is not
/// set, the function computes and returns only 5 distortion coefficients.
/// *   **CALIB_FIX_TAUX_TAUY** The coefficients of the tilted sensor model are not changed during
/// the optimization. If CALIB_USE_INTRINSIC_GUESS is set, the coefficient from the
/// supplied distCoeffs matrix is used. Otherwise, it is set to 0.
/// * criteria: Termination criteria for the iterative optimization algorithm.
/// 
/// ## Returns
/// the overall RMS re-projection error.
/// 
/// The function estimates the intrinsic camera parameters and extrinsic parameters for each of the
/// views. The algorithm is based on [Zhang2000](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Zhang2000) and [BouguetMCT](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_BouguetMCT) . The coordinates of 3D object
/// points and their corresponding 2D projections in each view must be specified. That may be achieved
/// by using an object with a known geometry and easily detectable feature points. Such an object is
/// called a calibration rig or calibration pattern, and OpenCV has built-in support for a chessboard as
/// a calibration rig (see findChessboardCorners ). Currently, initialization of intrinsic parameters
/// (when CALIB_USE_INTRINSIC_GUESS is not set) is only implemented for planar calibration
/// patterns (where Z-coordinates of the object points must be all zeros). 3D calibration rigs can also
/// be used as long as initial cameraMatrix is provided.
/// 
/// The algorithm performs the following steps:
/// 
/// *   Compute the initial intrinsic parameters (the option only available for planar calibration
/// patterns) or read them from the input parameters. The distortion coefficients are all set to
/// zeros initially unless some of CALIB_FIX_K? are specified.
/// 
/// *   Estimate the initial camera pose as if the intrinsic parameters have been already known. This is
/// done using solvePnP .
/// 
/// *   Run the global Levenberg-Marquardt optimization algorithm to minimize the reprojection error,
/// that is, the total sum of squared distances between the observed feature points imagePoints and
/// the projected (using the current estimates for camera parameters and the poses) object points
/// objectPoints. See projectPoints for details.
/// 
/// 
/// Note:
/// If you use a non-square (=non-NxN) grid and findChessboardCorners for calibration, and
/// calibrateCamera returns bad values (zero distortion coefficients, an image center very far from
/// (w/2-0.5,h/2-0.5), and/or large differences between <span lang='latex'>f_x</span> and <span lang='latex'>f_y</span> (ratios of 10:1 or more)),
/// then you have probably used patternSize=cvSize(rows,cols) instead of using
/// patternSize=cvSize(cols,rows) in findChessboardCorners .
/// 
/// ## See also
/// findChessboardCorners, solvePnP, initCameraMatrix2D, stereoCalibrate, undistort
/// 
/// ## Overloaded parameters
///  double calibrateCamera( InputArrayOfArrays objectPoints,
/// InputArrayOfArrays imagePoints, Size imageSize,
/// InputOutputArray cameraMatrix, InputOutputArray distCoeffs,
/// OutputArrayOfArrays rvecs, OutputArrayOfArrays tvecs,
/// OutputArray stdDeviations, OutputArray perViewErrors,
/// int flags = 0, TermCriteria criteria = TermCriteria(
/// TermCriteria::COUNT + TermCriteria::EPS, 30, DBL_EPSILON) )
///
/// ## C++ default parameters
/// * flags: 0
/// * criteria: TermCriteria( TermCriteria::COUNT + TermCriteria::EPS, 30, DBL_EPSILON)
pub fn calibrate_camera(object_points: &types::VectorOfMat, image_points: &types::VectorOfMat, image_size: core::Size, camera_matrix: &mut core::Mat, dist_coeffs: &mut core::Mat, rvecs: &mut types::VectorOfMat, tvecs: &mut types::VectorOfMat, flags: i32, criteria: &core::TermCriteria) -> Result<f64> {
    unsafe { sys::cv_calibrateCamera_VectorOfMat_VectorOfMat_Size_Mat_Mat_VectorOfMat_VectorOfMat_int_TermCriteria(object_points.as_raw_VectorOfMat(), image_points.as_raw_VectorOfMat(), image_size, camera_matrix.as_raw_Mat(), dist_coeffs.as_raw_Mat(), rvecs.as_raw_VectorOfMat(), tvecs.as_raw_VectorOfMat(), flags, criteria.as_raw_TermCriteria()) }.into_result()
}

/// Computes useful camera characteristics from the camera matrix.
/// 
/// ## Parameters
/// * cameraMatrix: Input camera matrix that can be estimated by calibrateCamera or
/// stereoCalibrate .
/// * imageSize: Input image size in pixels.
/// * apertureWidth: Physical width in mm of the sensor.
/// * apertureHeight: Physical height in mm of the sensor.
/// * fovx: Output field of view in degrees along the horizontal sensor axis.
/// * fovy: Output field of view in degrees along the vertical sensor axis.
/// * focalLength: Focal length of the lens in mm.
/// * principalPoint: Principal point in mm.
/// * aspectRatio: <span lang='latex'>f_y/f_x</span>
/// 
/// The function computes various useful camera characteristics from the previously estimated camera
/// matrix.
/// 
/// 
/// Note:
/// Do keep in mind that the unity measure 'mm' stands for whatever unit of measure one chooses for
/// the chessboard pitch (it can thus be any value).
pub fn calibration_matrix_values(camera_matrix: &core::Mat, image_size: core::Size, aperture_width: f64, aperture_height: f64, fovx: &mut f64, fovy: &mut f64, focal_length: &mut f64, principal_point: &mut core::Point2d, aspect_ratio: &mut f64) -> Result<()> {
    unsafe { sys::cv_calibrationMatrixValues_Mat_Size_double_double_double_double_double_Point2d_double(camera_matrix.as_raw_Mat(), image_size, aperture_width, aperture_height, fovx, fovy, focal_length, principal_point, aspect_ratio) }.into_result()
}

/// Combines two rotation-and-shift transformations.
/// 
/// ## Parameters
/// * rvec1: First rotation vector.
/// * tvec1: First translation vector.
/// * rvec2: Second rotation vector.
/// * tvec2: Second translation vector.
/// * rvec3: Output rotation vector of the superposition.
/// * tvec3: Output translation vector of the superposition.
/// * dr3dr1: 
/// * dr3dt1: 
/// * dr3dr2: 
/// * dr3dt2: 
/// * dt3dr1: 
/// * dt3dt1: 
/// * dt3dr2: 
/// * dt3dt2: Optional output derivatives of rvec3 or tvec3 with regard to rvec1, rvec2, tvec1 and
/// tvec2, respectively.
/// 
/// The functions compute:
/// 
/// <div lang='latex'>\begin{array}{l} \texttt{rvec3} =  \mathrm{rodrigues} ^{-1} \left ( \mathrm{rodrigues} ( \texttt{rvec2} )  \cdot \mathrm{rodrigues} ( \texttt{rvec1} ) \right )  \\ \texttt{tvec3} =  \mathrm{rodrigues} ( \texttt{rvec2} )  \cdot \texttt{tvec1} +  \texttt{tvec2} \end{array} ,</div>
/// 
/// where <span lang='latex'>\mathrm{rodrigues}</span> denotes a rotation vector to a rotation matrix transformation, and
/// <span lang='latex'>\mathrm{rodrigues}^{-1}</span> denotes the inverse transformation. See Rodrigues for details.
/// 
/// Also, the functions can compute the derivatives of the output vectors with regards to the input
/// vectors (see matMulDeriv ). The functions are used inside stereoCalibrate but can also be used in
/// your own code where Levenberg-Marquardt or another gradient-based solver is used to optimize a
/// function that contains a matrix multiplication.
///
/// ## C++ default parameters
/// * dr3dr1: noArray()
/// * dr3dt1: noArray()
/// * dr3dr2: noArray()
/// * dr3dt2: noArray()
/// * dt3dr1: noArray()
/// * dt3dt1: noArray()
/// * dt3dr2: noArray()
/// * dt3dt2: noArray()
pub fn compose_rt(rvec1: &core::Mat, tvec1: &core::Mat, rvec2: &core::Mat, tvec2: &core::Mat, rvec3: &mut core::Mat, tvec3: &mut core::Mat, dr3dr1: &mut core::Mat, dr3dt1: &mut core::Mat, dr3dr2: &mut core::Mat, dr3dt2: &mut core::Mat, dt3dr1: &mut core::Mat, dt3dt1: &mut core::Mat, dt3dr2: &mut core::Mat, dt3dt2: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_composeRT_Mat_Mat_Mat_Mat_Mat_Mat_Mat_Mat_Mat_Mat_Mat_Mat_Mat_Mat(rvec1.as_raw_Mat(), tvec1.as_raw_Mat(), rvec2.as_raw_Mat(), tvec2.as_raw_Mat(), rvec3.as_raw_Mat(), tvec3.as_raw_Mat(), dr3dr1.as_raw_Mat(), dr3dt1.as_raw_Mat(), dr3dr2.as_raw_Mat(), dr3dt2.as_raw_Mat(), dt3dr1.as_raw_Mat(), dt3dt1.as_raw_Mat(), dt3dr2.as_raw_Mat(), dt3dt2.as_raw_Mat()) }.into_result()
}

/// For points in an image of a stereo pair, computes the corresponding epilines in the other image.
/// 
/// ## Parameters
/// * points: Input points. <span lang='latex'>N \times 1</span> or <span lang='latex'>1 \times N</span> matrix of type CV_32FC2 or
/// vector\<Point2f\> .
/// * whichImage: Index of the image (1 or 2) that contains the points .
/// * F: Fundamental matrix that can be estimated using findFundamentalMat or stereoRectify .
/// * lines: Output vector of the epipolar lines corresponding to the points in the other image.
/// Each line <span lang='latex'>ax + by + c=0</span> is encoded by 3 numbers <span lang='latex'>(a, b, c)</span> .
/// 
/// For every point in one of the two images of a stereo pair, the function finds the equation of the
/// corresponding epipolar line in the other image.
/// 
/// From the fundamental matrix definition (see findFundamentalMat ), line <span lang='latex'>l^{(2)}_i</span> in the second
/// image for the point <span lang='latex'>p^{(1)}_i</span> in the first image (when whichImage=1 ) is computed as:
/// 
/// <div lang='latex'>l^{(2)}_i = F p^{(1)}_i</div>
/// 
/// And vice versa, when whichImage=2, <span lang='latex'>l^{(1)}_i</span> is computed from <span lang='latex'>p^{(2)}_i</span> as:
/// 
/// <div lang='latex'>l^{(1)}_i = F^T p^{(2)}_i</div>
/// 
/// Line coefficients are defined up to a scale. They are normalized so that <span lang='latex'>a_i^2+b_i^2=1</span> .
pub fn compute_correspond_epilines(points: &core::Mat, which_image: i32, f: &core::Mat, lines: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_computeCorrespondEpilines_Mat_int_Mat_Mat(points.as_raw_Mat(), which_image, f.as_raw_Mat(), lines.as_raw_Mat()) }.into_result()
}

/// Converts points from homogeneous to Euclidean space.
/// 
/// ## Parameters
/// * src: Input vector of N-dimensional points.
/// * dst: Output vector of N-1-dimensional points.
/// 
/// The function converts points homogeneous to Euclidean space using perspective projection. That is,
/// each point (x1, x2, ... x(n-1), xn) is converted to (x1/xn, x2/xn, ..., x(n-1)/xn). When xn=0, the
/// output point coordinates will be (0,0,0,...).
pub fn convert_points_from_homogeneous(src: &core::Mat, dst: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_convertPointsFromHomogeneous_Mat_Mat(src.as_raw_Mat(), dst.as_raw_Mat()) }.into_result()
}

/// Converts points to/from homogeneous coordinates.
/// 
/// ## Parameters
/// * src: Input array or vector of 2D, 3D, or 4D points.
/// * dst: Output vector of 2D, 3D, or 4D points.
/// 
/// The function converts 2D or 3D points from/to homogeneous coordinates by calling either
/// convertPointsToHomogeneous or convertPointsFromHomogeneous.
/// 
/// 
/// Note: The function is obsolete. Use one of the previous two functions instead.
pub fn convert_points_homogeneous(src: &core::Mat, dst: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_convertPointsHomogeneous_Mat_Mat(src.as_raw_Mat(), dst.as_raw_Mat()) }.into_result()
}

/// Converts points from Euclidean to homogeneous space.
/// 
/// ## Parameters
/// * src: Input vector of N-dimensional points.
/// * dst: Output vector of N+1-dimensional points.
/// 
/// The function converts points from Euclidean to homogeneous space by appending 1's to the tuple of
/// point coordinates. That is, each point (x1, x2, ..., xn) is converted to (x1, x2, ..., xn, 1).
pub fn convert_points_to_homogeneous(src: &core::Mat, dst: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_convertPointsToHomogeneous_Mat_Mat(src.as_raw_Mat(), dst.as_raw_Mat()) }.into_result()
}

/// Refines coordinates of corresponding points.
/// 
/// ## Parameters
/// * F: 3x3 fundamental matrix.
/// * points1: 1xN array containing the first set of points.
/// * points2: 1xN array containing the second set of points.
/// * newPoints1: The optimized points1.
/// * newPoints2: The optimized points2.
/// 
/// The function implements the Optimal Triangulation Method (see Multiple View Geometry for details).
/// For each given point correspondence points1[i] \<-\> points2[i], and a fundamental matrix F, it
/// computes the corrected correspondences newPoints1[i] \<-\> newPoints2[i] that minimize the geometric
/// error <span lang='latex'>d(points1[i], newPoints1[i])^2 + d(points2[i],newPoints2[i])^2</span> (where <span lang='latex'>d(a,b)</span> is the
/// geometric distance between points <span lang='latex'>a</span> and <span lang='latex'>b</span> ) subject to the epipolar constraint
/// <span lang='latex'>newPoints2^T * F * newPoints1 = 0</span> .
pub fn correct_matches(f: &core::Mat, points1: &core::Mat, points2: &core::Mat, new_points1: &mut core::Mat, new_points2: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_correctMatches_Mat_Mat_Mat_Mat_Mat(f.as_raw_Mat(), points1.as_raw_Mat(), points2.as_raw_Mat(), new_points1.as_raw_Mat(), new_points2.as_raw_Mat()) }.into_result()
}

/// Decompose an essential matrix to possible rotations and translation.
/// 
/// ## Parameters
/// * E: The input essential matrix.
/// * R1: One possible rotation matrix.
/// * R2: Another possible rotation matrix.
/// * t: One possible translation.
/// 
/// This function decompose an essential matrix E using svd decomposition [HartleyZ00](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_HartleyZ00) . Generally 4
/// possible poses exists for a given E. They are <span lang='latex'>[R_1, t]</span>, <span lang='latex'>[R_1, -t]</span>, <span lang='latex'>[R_2, t]</span>, <span lang='latex'>[R_2, -t]</span>. By
/// decomposing E, you can only get the direction of the translation, so the function returns unit t.
pub fn decompose_essential_mat(e: &core::Mat, r1: &mut core::Mat, r2: &mut core::Mat, t: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_decomposeEssentialMat_Mat_Mat_Mat_Mat(e.as_raw_Mat(), r1.as_raw_Mat(), r2.as_raw_Mat(), t.as_raw_Mat()) }.into_result()
}

/// Decompose a homography matrix to rotation(s), translation(s) and plane normal(s).
/// 
/// ## Parameters
/// * H: The input homography matrix between two images.
/// * K: The input intrinsic camera calibration matrix.
/// * rotations: Array of rotation matrices.
/// * translations: Array of translation matrices.
/// * normals: Array of plane normal matrices.
/// 
/// This function extracts relative camera motion between two views observing a planar object from the
/// homography H induced by the plane. The intrinsic camera matrix K must also be provided. The function
/// may return up to four mathematical solution sets. At least two of the solutions may further be
/// invalidated if point correspondences are available by applying positive depth constraint (all points
/// must be in front of the camera). The decomposition method is described in detail in [Malis](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Malis) .
pub fn decompose_homography_mat(h: &core::Mat, k: &core::Mat, rotations: &mut types::VectorOfMat, translations: &mut types::VectorOfMat, normals: &mut types::VectorOfMat) -> Result<i32> {
    unsafe { sys::cv_decomposeHomographyMat_Mat_Mat_VectorOfMat_VectorOfMat_VectorOfMat(h.as_raw_Mat(), k.as_raw_Mat(), rotations.as_raw_VectorOfMat(), translations.as_raw_VectorOfMat(), normals.as_raw_VectorOfMat()) }.into_result()
}

/// Decomposes a projection matrix into a rotation matrix and a camera matrix.
/// 
/// ## Parameters
/// * projMatrix: 3x4 input projection matrix P.
/// * cameraMatrix: Output 3x3 camera matrix K.
/// * rotMatrix: Output 3x3 external rotation matrix R.
/// * transVect: Output 4x1 translation vector T.
/// * rotMatrixX: Optional 3x3 rotation matrix around x-axis.
/// * rotMatrixY: Optional 3x3 rotation matrix around y-axis.
/// * rotMatrixZ: Optional 3x3 rotation matrix around z-axis.
/// * eulerAngles: Optional three-element vector containing three Euler angles of rotation in
/// degrees.
/// 
/// The function computes a decomposition of a projection matrix into a calibration and a rotation
/// matrix and the position of a camera.
/// 
/// It optionally returns three rotation matrices, one for each axis, and three Euler angles that could
/// be used in OpenGL. Note, there is always more than one sequence of rotations about the three
/// principal axes that results in the same orientation of an object, e.g. see [Slabaugh](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Slabaugh) . Returned
/// tree rotation matrices and corresponding three Euler angles are only one of the possible solutions.
/// 
/// The function is based on RQDecomp3x3 .
///
/// ## C++ default parameters
/// * rot_matrix_x: noArray()
/// * rot_matrix_y: noArray()
/// * rot_matrix_z: noArray()
/// * euler_angles: noArray()
pub fn decompose_projection_matrix(proj_matrix: &core::Mat, camera_matrix: &mut core::Mat, rot_matrix: &mut core::Mat, trans_vect: &mut core::Mat, rot_matrix_x: &mut core::Mat, rot_matrix_y: &mut core::Mat, rot_matrix_z: &mut core::Mat, euler_angles: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_decomposeProjectionMatrix_Mat_Mat_Mat_Mat_Mat_Mat_Mat_Mat(proj_matrix.as_raw_Mat(), camera_matrix.as_raw_Mat(), rot_matrix.as_raw_Mat(), trans_vect.as_raw_Mat(), rot_matrix_x.as_raw_Mat(), rot_matrix_y.as_raw_Mat(), rot_matrix_z.as_raw_Mat(), euler_angles.as_raw_Mat()) }.into_result()
}

/// Renders the detected chessboard corners.
/// 
/// ## Parameters
/// * image: Destination image. It must be an 8-bit color image.
/// * patternSize: Number of inner corners per a chessboard row and column
/// (patternSize = cv::Size(points_per_row,points_per_column)).
/// * corners: Array of detected corners, the output of findChessboardCorners.
/// * patternWasFound: Parameter indicating whether the complete board was found or not. The
/// return value of findChessboardCorners should be passed here.
/// 
/// The function draws individual chessboard corners detected either as red circles if the board was not
/// found, or as colored corners connected with lines if the board was found.
pub fn draw_chessboard_corners(image: &mut core::Mat, pattern_size: core::Size, corners: &core::Mat, pattern_was_found: bool) -> Result<()> {
    unsafe { sys::cv_drawChessboardCorners_Mat_Size_Mat_bool(image.as_raw_Mat(), pattern_size, corners.as_raw_Mat(), pattern_was_found) }.into_result()
}

/// Draw axes of the world/object coordinate system from pose estimation. ## See also
/// solvePnP
/// 
/// ## Parameters
/// * image: Input/output image. It must have 1 or 3 channels. The number of channels is not altered.
/// * cameraMatrix: Input 3x3 floating-point matrix of camera intrinsic parameters.
/// <span lang='latex'>A = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{1}</span>
/// * distCoeffs: Input vector of distortion coefficients
/// <span lang='latex'>(k_1, k_2, p_1, p_2[, k_3[, k_4, k_5, k_6 [, s_1, s_2, s_3, s_4[, \tau_x, \tau_y]]]])</span> of
/// 4, 5, 8, 12 or 14 elements. If the vector is empty, the zero distortion coefficients are assumed.
/// * rvec: Rotation vector (see @ref Rodrigues ) that, together with tvec , brings points from
/// the model coordinate system to the camera coordinate system.
/// * tvec: Translation vector.
/// * length: Length of the painted axes in the same unit than tvec (usually in meters).
/// * thickness: Line thickness of the painted axes.
/// 
/// This function draws the axes of the world/object coordinate system w.r.t. to the camera frame.
/// OX is drawn in red, OY in green and OZ in blue.
///
/// ## C++ default parameters
/// * thickness: 3
pub fn draw_frame_axes(image: &mut core::Mat, camera_matrix: &core::Mat, dist_coeffs: &core::Mat, rvec: &core::Mat, tvec: &core::Mat, length: f32, thickness: i32) -> Result<()> {
    unsafe { sys::cv_drawFrameAxes_Mat_Mat_Mat_Mat_Mat_float_int(image.as_raw_Mat(), camera_matrix.as_raw_Mat(), dist_coeffs.as_raw_Mat(), rvec.as_raw_Mat(), tvec.as_raw_Mat(), length, thickness) }.into_result()
}

/// Computes an optimal affine transformation between two 2D point sets.
/// 
/// It computes
/// <div lang='latex'>
/// \begin{bmatrix}
/// x\\
/// y\\
/// \end{bmatrix}
/// =
/// \begin{bmatrix}
/// a_{11} & a_{12}\\
/// a_{21} & a_{22}\\
/// \end{bmatrix}
/// \begin{bmatrix}
/// X\\
/// Y\\
/// \end{bmatrix}
/// +
/// \begin{bmatrix}
/// b_1\\
/// b_2\\
/// \end{bmatrix}
/// </div>
/// 
/// ## Parameters
/// * from: First input 2D point set containing <span lang='latex'>(X,Y)</span>.
/// * to: Second input 2D point set containing <span lang='latex'>(x,y)</span>.
/// * inliers: Output vector indicating which points are inliers (1-inlier, 0-outlier).
/// * method: Robust method used to compute transformation. The following methods are possible:
/// *   cv::RANSAC - RANSAC-based robust method
/// *   cv::LMEDS - Least-Median robust method
/// RANSAC is the default method.
/// * ransacReprojThreshold: Maximum reprojection error in the RANSAC algorithm to consider
/// a point as an inlier. Applies only to RANSAC.
/// * maxIters: The maximum number of robust method iterations.
/// * confidence: Confidence level, between 0 and 1, for the estimated transformation. Anything
/// between 0.95 and 0.99 is usually good enough. Values too close to 1 can slow down the estimation
/// significantly. Values lower than 0.8-0.9 can result in an incorrectly estimated transformation.
/// * refineIters: Maximum number of iterations of refining algorithm (Levenberg-Marquardt).
/// Passing 0 will disable refining, so the output matrix will be output of robust method.
/// 
/// ## Returns
/// Output 2D affine transformation matrix <span lang='latex'>2 \times 3</span> or empty matrix if transformation
/// could not be estimated. The returned matrix has the following form:
/// <div lang='latex'>
/// \begin{bmatrix}
/// a_{11} & a_{12} & b_1\\
/// a_{21} & a_{22} & b_2\\
/// \end{bmatrix}
/// </div>
/// 
/// The function estimates an optimal 2D affine transformation between two 2D point sets using the
/// selected robust algorithm.
/// 
/// The computed transformation is then refined further (using only inliers) with the
/// Levenberg-Marquardt method to reduce the re-projection error even more.
/// 
/// 
/// Note:
/// The RANSAC method can handle practically any ratio of outliers but needs a threshold to
/// distinguish inliers from outliers. The method LMeDS does not need any threshold but it works
/// correctly only when there are more than 50% of inliers.
/// 
/// ## See also
/// estimateAffinePartial2D, getAffineTransform
///
/// ## C++ default parameters
/// * inliers: noArray()
/// * method: RANSAC
/// * ransac_reproj_threshold: 3
/// * max_iters: 2000
/// * confidence: 0.99
/// * refine_iters: 10
pub fn estimate_affine_2d(from: &core::Mat, to: &core::Mat, inliers: &mut core::Mat, method: i32, ransac_reproj_threshold: f64, max_iters: size_t, confidence: f64, refine_iters: size_t) -> Result<core::Mat> {
    unsafe { sys::cv_estimateAffine2D_Mat_Mat_Mat_int_double_size_t_double_size_t(from.as_raw_Mat(), to.as_raw_Mat(), inliers.as_raw_Mat(), method, ransac_reproj_threshold, max_iters, confidence, refine_iters) }.into_result().map(|ptr| core::Mat { ptr })
}

/// Computes an optimal affine transformation between two 3D point sets.
/// 
/// It computes
/// <div lang='latex'>
/// \begin{bmatrix}
/// x\\
/// y\\
/// z\\
/// \end{bmatrix}
/// =
/// \begin{bmatrix}
/// a_{11} & a_{12} & a_{13}\\
/// a_{21} & a_{22} & a_{23}\\
/// a_{31} & a_{32} & a_{33}\\
/// \end{bmatrix}
/// \begin{bmatrix}
/// X\\
/// Y\\
/// Z\\
/// \end{bmatrix}
/// +
/// \begin{bmatrix}
/// b_1\\
/// b_2\\
/// b_3\\
/// \end{bmatrix}
/// </div>
/// 
/// ## Parameters
/// * src: First input 3D point set containing <span lang='latex'>(X,Y,Z)</span>.
/// * dst: Second input 3D point set containing <span lang='latex'>(x,y,z)</span>.
/// * out: Output 3D affine transformation matrix <span lang='latex'>3 \times 4</span> of the form
/// <div lang='latex'>
/// \begin{bmatrix}
/// a_{11} & a_{12} & a_{13} & b_1\\
/// a_{21} & a_{22} & a_{23} & b_2\\
/// a_{31} & a_{32} & a_{33} & b_3\\
/// \end{bmatrix}
/// </div>
/// * inliers: Output vector indicating which points are inliers (1-inlier, 0-outlier).
/// * ransacThreshold: Maximum reprojection error in the RANSAC algorithm to consider a point as
/// an inlier.
/// * confidence: Confidence level, between 0 and 1, for the estimated transformation. Anything
/// between 0.95 and 0.99 is usually good enough. Values too close to 1 can slow down the estimation
/// significantly. Values lower than 0.8-0.9 can result in an incorrectly estimated transformation.
/// 
/// The function estimates an optimal 3D affine transformation between two 3D point sets using the
/// RANSAC algorithm.
///
/// ## C++ default parameters
/// * ransac_threshold: 3
/// * confidence: 0.99
pub fn estimate_affine_3d(src: &core::Mat, dst: &core::Mat, out: &mut core::Mat, inliers: &mut core::Mat, ransac_threshold: f64, confidence: f64) -> Result<i32> {
    unsafe { sys::cv_estimateAffine3D_Mat_Mat_Mat_Mat_double_double(src.as_raw_Mat(), dst.as_raw_Mat(), out.as_raw_Mat(), inliers.as_raw_Mat(), ransac_threshold, confidence) }.into_result()
}

/// Computes an optimal limited affine transformation with 4 degrees of freedom between
/// two 2D point sets.
/// 
/// ## Parameters
/// * from: First input 2D point set.
/// * to: Second input 2D point set.
/// * inliers: Output vector indicating which points are inliers.
/// * method: Robust method used to compute transformation. The following methods are possible:
/// *   cv::RANSAC - RANSAC-based robust method
/// *   cv::LMEDS - Least-Median robust method
/// RANSAC is the default method.
/// * ransacReprojThreshold: Maximum reprojection error in the RANSAC algorithm to consider
/// a point as an inlier. Applies only to RANSAC.
/// * maxIters: The maximum number of robust method iterations.
/// * confidence: Confidence level, between 0 and 1, for the estimated transformation. Anything
/// between 0.95 and 0.99 is usually good enough. Values too close to 1 can slow down the estimation
/// significantly. Values lower than 0.8-0.9 can result in an incorrectly estimated transformation.
/// * refineIters: Maximum number of iterations of refining algorithm (Levenberg-Marquardt).
/// Passing 0 will disable refining, so the output matrix will be output of robust method.
/// 
/// ## Returns
/// Output 2D affine transformation (4 degrees of freedom) matrix <span lang='latex'>2 \times 3</span> or
/// empty matrix if transformation could not be estimated.
/// 
/// The function estimates an optimal 2D affine transformation with 4 degrees of freedom limited to
/// combinations of translation, rotation, and uniform scaling. Uses the selected algorithm for robust
/// estimation.
/// 
/// The computed transformation is then refined further (using only inliers) with the
/// Levenberg-Marquardt method to reduce the re-projection error even more.
/// 
/// Estimated transformation matrix is:
/// <div lang='latex'> \begin{bmatrix} \cos(\theta) \cdot s & -\sin(\theta) \cdot s & t_x \\
/// \sin(\theta) \cdot s & \cos(\theta) \cdot s & t_y
/// \end{bmatrix} </div>
/// Where <span lang='latex'> \theta </span> is the rotation angle, <span lang='latex'> s </span> the scaling factor and <span lang='latex'> t_x, t_y </span> are
/// translations in <span lang='latex'> x, y </span> axes respectively.
/// 
/// 
/// Note:
/// The RANSAC method can handle practically any ratio of outliers but need a threshold to
/// distinguish inliers from outliers. The method LMeDS does not need any threshold but it works
/// correctly only when there are more than 50% of inliers.
/// 
/// ## See also
/// estimateAffine2D, getAffineTransform
///
/// ## C++ default parameters
/// * inliers: noArray()
/// * method: RANSAC
/// * ransac_reproj_threshold: 3
/// * max_iters: 2000
/// * confidence: 0.99
/// * refine_iters: 10
pub fn estimate_affine_partial_2d(from: &core::Mat, to: &core::Mat, inliers: &mut core::Mat, method: i32, ransac_reproj_threshold: f64, max_iters: size_t, confidence: f64, refine_iters: size_t) -> Result<core::Mat> {
    unsafe { sys::cv_estimateAffinePartial2D_Mat_Mat_Mat_int_double_size_t_double_size_t(from.as_raw_Mat(), to.as_raw_Mat(), inliers.as_raw_Mat(), method, ransac_reproj_threshold, max_iters, confidence, refine_iters) }.into_result().map(|ptr| core::Mat { ptr })
}

/// Filters homography decompositions based on additional information.
/// 
/// ## Parameters
/// * rotations: Vector of rotation matrices.
/// * normals: Vector of plane normal matrices.
/// * beforePoints: Vector of (rectified) visible reference points before the homography is applied
/// * afterPoints: Vector of (rectified) visible reference points after the homography is applied
/// * possibleSolutions: Vector of int indices representing the viable solution set after filtering
/// * pointsMask: optional Mat/Vector of 8u type representing the mask for the inliers as given by the findHomography function
/// 
/// This function is intended to filter the output of the decomposeHomographyMat based on additional
/// information as described in [Malis](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Malis) . The summary of the method: the decomposeHomographyMat function
/// returns 2 unique solutions and their "opposites" for a total of 4 solutions. If we have access to the
/// sets of points visible in the camera frame before and after the homography transformation is applied,
/// we can determine which are the true potential solutions and which are the opposites by verifying which
/// homographies are consistent with all visible reference points being in front of the camera. The inputs
/// are left unchanged; the filtered solution set is returned as indices into the existing one.
///
/// ## C++ default parameters
/// * points_mask: noArray()
pub fn filter_homography_decomp_by_visible_refpoints(rotations: &types::VectorOfMat, normals: &types::VectorOfMat, before_points: &core::Mat, after_points: &core::Mat, possible_solutions: &mut core::Mat, points_mask: &core::Mat) -> Result<()> {
    unsafe { sys::cv_filterHomographyDecompByVisibleRefpoints_VectorOfMat_VectorOfMat_Mat_Mat_Mat_Mat(rotations.as_raw_VectorOfMat(), normals.as_raw_VectorOfMat(), before_points.as_raw_Mat(), after_points.as_raw_Mat(), possible_solutions.as_raw_Mat(), points_mask.as_raw_Mat()) }.into_result()
}

/// Filters off small noise blobs (speckles) in the disparity map
/// 
/// ## Parameters
/// * img: The input 16-bit signed disparity image
/// * newVal: The disparity value used to paint-off the speckles
/// * maxSpeckleSize: The maximum speckle size to consider it a speckle. Larger blobs are not
/// affected by the algorithm
/// * maxDiff: Maximum difference between neighbor disparity pixels to put them into the same
/// blob. Note that since StereoBM, StereoSGBM and may be other algorithms return a fixed-point
/// disparity map, where disparity values are multiplied by 16, this scale factor should be taken into
/// account when specifying this parameter value.
/// * buf: The optional temporary buffer to avoid memory allocation within the function.
///
/// ## C++ default parameters
/// * buf: noArray()
pub fn filter_speckles(img: &mut core::Mat, new_val: f64, max_speckle_size: i32, max_diff: f64, buf: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_filterSpeckles_Mat_double_int_double_Mat(img.as_raw_Mat(), new_val, max_speckle_size, max_diff, buf.as_raw_Mat()) }.into_result()
}

/// finds subpixel-accurate positions of the chessboard corners
pub fn find4_quad_corner_subpix(img: &core::Mat, corners: &mut core::Mat, region_size: core::Size) -> Result<bool> {
    unsafe { sys::cv_find4QuadCornerSubpix_Mat_Mat_Size(img.as_raw_Mat(), corners.as_raw_Mat(), region_size) }.into_result()
}

/// Finds the positions of internal corners of the chessboard.
/// 
/// ## Parameters
/// * image: Source chessboard view. It must be an 8-bit grayscale or color image.
/// * patternSize: Number of inner corners per a chessboard row and column
/// ( patternSize = cvSize(points_per_row,points_per_colum) = cvSize(columns,rows) ).
/// * corners: Output array of detected corners.
/// * flags: Various operation flags that can be zero or a combination of the following values:
/// *   **CALIB_CB_ADAPTIVE_THRESH** Use adaptive thresholding to convert the image to black
/// and white, rather than a fixed threshold level (computed from the average image brightness).
/// *   **CALIB_CB_NORMALIZE_IMAGE** Normalize the image gamma with equalizeHist before
/// applying fixed or adaptive thresholding.
/// *   **CALIB_CB_FILTER_QUADS** Use additional criteria (like contour area, perimeter,
/// square-like shape) to filter out false quads extracted at the contour retrieval stage.
/// *   **CALIB_CB_FAST_CHECK** Run a fast check on the image that looks for chessboard corners,
/// and shortcut the call if none is found. This can drastically speed up the call in the
/// degenerate condition when no chessboard is observed.
/// 
/// The function attempts to determine whether the input image is a view of the chessboard pattern and
/// locate the internal chessboard corners. The function returns a non-zero value if all of the corners
/// are found and they are placed in a certain order (row by row, left to right in every row).
/// Otherwise, if the function fails to find all the corners or reorder them, it returns 0. For example,
/// a regular chessboard has 8 x 8 squares and 7 x 7 internal corners, that is, points where the black
/// squares touch each other. The detected coordinates are approximate, and to determine their positions
/// more accurately, the function calls cornerSubPix. You also may use the function cornerSubPix with
/// different parameters if returned coordinates are not accurate enough.
/// 
/// Sample usage of detecting and drawing chessboard corners: :
/// ```ignore
/// Size patternsize(8,6); //interior number of corners
/// Mat gray = ....; //source image
/// vector<Point2f> corners; //this will be filled by the detected corners
/// 
/// //CALIB_CB_FAST_CHECK saves a lot of time on images
/// //that do not contain any chessboard corners
/// bool patternfound = findChessboardCorners(gray, patternsize, corners,
/// CALIB_CB_ADAPTIVE_THRESH + CALIB_CB_NORMALIZE_IMAGE
/// + CALIB_CB_FAST_CHECK);
/// 
/// if(patternfound)
/// cornerSubPix(gray, corners, Size(11, 11), Size(-1, -1),
/// TermCriteria(CV_TERMCRIT_EPS + CV_TERMCRIT_ITER, 30, 0.1));
/// 
/// drawChessboardCorners(img, patternsize, Mat(corners), patternfound);
/// ```
/// 
/// 
/// Note: The function requires white space (like a square-thick border, the wider the better) around
/// the board to make the detection more robust in various environments. Otherwise, if there is no
/// border and the background is dark, the outer black squares cannot be segmented properly and so the
/// square grouping and ordering algorithm fails.
///
/// ## C++ default parameters
/// * flags: CALIB_CB_ADAPTIVE_THRESH + CALIB_CB_NORMALIZE_IMAGE
pub fn find_chessboard_corners(image: &core::Mat, pattern_size: core::Size, corners: &mut core::Mat, flags: i32) -> Result<bool> {
    unsafe { sys::cv_findChessboardCorners_Mat_Size_Mat_int(image.as_raw_Mat(), pattern_size, corners.as_raw_Mat(), flags) }.into_result()
}

/// Calculates an essential matrix from the corresponding points in two images.
/// 
/// ## Parameters
/// * points1: Array of N (N \>= 5) 2D points from the first image. The point coordinates should
/// be floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1 .
/// * cameraMatrix: Camera matrix <span lang='latex'>K = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{1}</span> .
/// Note that this function assumes that points1 and points2 are feature points from cameras with the
/// same camera matrix.
/// * method: Method for computing an essential matrix.
/// *   **RANSAC** for the RANSAC algorithm.
/// *   **LMEDS** for the LMedS algorithm.
/// * prob: Parameter used for the RANSAC or LMedS methods only. It specifies a desirable level of
/// confidence (probability) that the estimated matrix is correct.
/// * threshold: Parameter used for RANSAC. It is the maximum distance from a point to an epipolar
/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
/// point localization, image resolution, and the image noise.
/// * mask: Output array of N elements, every element of which is set to 0 for outliers and to 1
/// for the other points. The array is computed only in the RANSAC and LMedS methods.
/// 
/// This function estimates essential matrix based on the five-point algorithm solver in [Nister03](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Nister03) .
/// [SteweniusCFS](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_SteweniusCFS) is also a related. The epipolar geometry is described by the following equation:
/// 
/// <div lang='latex'>[p_2; 1]^T K^{-T} E K^{-1} [p_1; 1] = 0</div>
/// 
/// where <span lang='latex'>E</span> is an essential matrix, <span lang='latex'>p_1</span> and <span lang='latex'>p_2</span> are corresponding points in the first and the
/// second images, respectively. The result of this function may be passed further to
/// decomposeEssentialMat or recoverPose to recover the relative pose between cameras.
///
/// ## C++ default parameters
/// * method: RANSAC
/// * prob: 0.999
/// * threshold: 1.0
/// * mask: noArray()
pub fn find_essential_mat_matrix(points1: &core::Mat, points2: &core::Mat, camera_matrix: &core::Mat, method: i32, prob: f64, threshold: f64, mask: &mut core::Mat) -> Result<core::Mat> {
    unsafe { sys::cv_findEssentialMat_Mat_Mat_Mat_int_double_double_Mat(points1.as_raw_Mat(), points2.as_raw_Mat(), camera_matrix.as_raw_Mat(), method, prob, threshold, mask.as_raw_Mat()) }.into_result().map(|ptr| core::Mat { ptr })
}

/// Calculates an essential matrix from the corresponding points in two images.
/// 
/// ## Parameters
/// * points1: Array of N (N \>= 5) 2D points from the first image. The point coordinates should
/// be floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1 .
/// * cameraMatrix: Camera matrix <span lang='latex'>K = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{1}</span> .
/// Note that this function assumes that points1 and points2 are feature points from cameras with the
/// same camera matrix.
/// * method: Method for computing an essential matrix.
/// *   **RANSAC** for the RANSAC algorithm.
/// *   **LMEDS** for the LMedS algorithm.
/// * prob: Parameter used for the RANSAC or LMedS methods only. It specifies a desirable level of
/// confidence (probability) that the estimated matrix is correct.
/// * threshold: Parameter used for RANSAC. It is the maximum distance from a point to an epipolar
/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
/// point localization, image resolution, and the image noise.
/// * mask: Output array of N elements, every element of which is set to 0 for outliers and to 1
/// for the other points. The array is computed only in the RANSAC and LMedS methods.
/// 
/// This function estimates essential matrix based on the five-point algorithm solver in [Nister03](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Nister03) .
/// [SteweniusCFS](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_SteweniusCFS) is also a related. The epipolar geometry is described by the following equation:
/// 
/// <div lang='latex'>[p_2; 1]^T K^{-T} E K^{-1} [p_1; 1] = 0</div>
/// 
/// where <span lang='latex'>E</span> is an essential matrix, <span lang='latex'>p_1</span> and <span lang='latex'>p_2</span> are corresponding points in the first and the
/// second images, respectively. The result of this function may be passed further to
/// decomposeEssentialMat or recoverPose to recover the relative pose between cameras.
/// 
/// ## Overloaded parameters
/// 
/// * points1: Array of N (N \>= 5) 2D points from the first image. The point coordinates should
/// be floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1 .
/// * focal: focal length of the camera. Note that this function assumes that points1 and points2
/// are feature points from cameras with same focal length and principal point.
/// * pp: principal point of the camera.
/// * method: Method for computing a fundamental matrix.
/// *   **RANSAC** for the RANSAC algorithm.
/// *   **LMEDS** for the LMedS algorithm.
/// * threshold: Parameter used for RANSAC. It is the maximum distance from a point to an epipolar
/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
/// point localization, image resolution, and the image noise.
/// * prob: Parameter used for the RANSAC or LMedS methods only. It specifies a desirable level of
/// confidence (probability) that the estimated matrix is correct.
/// * mask: Output array of N elements, every element of which is set to 0 for outliers and to 1
/// for the other points. The array is computed only in the RANSAC and LMedS methods.
/// 
/// This function differs from the one above that it computes camera matrix from focal length and
/// principal point:
/// 
/// <div lang='latex'>K =
/// \begin{bmatrix}
/// f & 0 & x_{pp}  \\
/// 0 & f & y_{pp}  \\
/// 0 & 0 & 1
/// \end{bmatrix}</div>
///
/// ## C++ default parameters
/// * focal: 1.0
/// * pp: Point2d(0, 0)
/// * method: RANSAC
/// * prob: 0.999
/// * threshold: 1.0
/// * mask: noArray()
pub fn find_essential_mat(points1: &core::Mat, points2: &core::Mat, focal: f64, pp: core::Point2d, method: i32, prob: f64, threshold: f64, mask: &mut core::Mat) -> Result<core::Mat> {
    unsafe { sys::cv_findEssentialMat_Mat_Mat_double_Point2d_int_double_double_Mat(points1.as_raw_Mat(), points2.as_raw_Mat(), focal, pp, method, prob, threshold, mask.as_raw_Mat()) }.into_result().map(|ptr| core::Mat { ptr })
}

/// Calculates a fundamental matrix from the corresponding points in two images.
/// 
/// ## Parameters
/// * points1: Array of N points from the first image. The point coordinates should be
/// floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1 .
/// * method: Method for computing a fundamental matrix.
/// *   **CV_FM_7POINT** for a 7-point algorithm. <span lang='latex'>N = 7</span>
/// *   **CV_FM_8POINT** for an 8-point algorithm. <span lang='latex'>N \ge 8</span>
/// *   **CV_FM_RANSAC** for the RANSAC algorithm. <span lang='latex'>N \ge 8</span>
/// *   **CV_FM_LMEDS** for the LMedS algorithm. <span lang='latex'>N \ge 8</span>
/// * ransacReprojThreshold: Parameter used only for RANSAC. It is the maximum distance from a point to an epipolar
/// line in pixels, beyond which the point is considered an outlier and is not used for computing the
/// final fundamental matrix. It can be set to something like 1-3, depending on the accuracy of the
/// point localization, image resolution, and the image noise.
/// * confidence: Parameter used for the RANSAC and LMedS methods only. It specifies a desirable level
/// of confidence (probability) that the estimated matrix is correct.
/// * mask: 
/// 
/// The epipolar geometry is described by the following equation:
/// 
/// <div lang='latex'>[p_2; 1]^T F [p_1; 1] = 0</div>
/// 
/// where <span lang='latex'>F</span> is a fundamental matrix, <span lang='latex'>p_1</span> and <span lang='latex'>p_2</span> are corresponding points in the first and the
/// second images, respectively.
/// 
/// The function calculates the fundamental matrix using one of four methods listed above and returns
/// the found fundamental matrix. Normally just one matrix is found. But in case of the 7-point
/// algorithm, the function may return up to 3 solutions ( <span lang='latex'>9 \times 3</span> matrix that stores all 3
/// matrices sequentially).
/// 
/// The calculated fundamental matrix may be passed further to computeCorrespondEpilines that finds the
/// epipolar lines corresponding to the specified points. It can also be passed to
/// stereoRectifyUncalibrated to compute the rectification transformation. :
/// ```ignore
/// // Example. Estimation of fundamental matrix using the RANSAC algorithm
/// int point_count = 100;
/// vector<Point2f> points1(point_count);
/// vector<Point2f> points2(point_count);
/// 
/// // initialize the points here ...
/// for( int i = 0; i < point_count; i++ )
/// {
/// points1[i] = ...;
/// points2[i] = ...;
/// }
/// 
/// Mat fundamental_matrix =
/// findFundamentalMat(points1, points2, FM_RANSAC, 3, 0.99);
/// ```
/// 
/// 
/// ## Overloaded parameters
///
/// ## C++ default parameters
/// * method: FM_RANSAC
/// * ransac_reproj_threshold: 3.
/// * confidence: 0.99
pub fn find_fundamental_mat(points1: &core::Mat, points2: &core::Mat, mask: &mut core::Mat, method: i32, ransac_reproj_threshold: f64, confidence: f64) -> Result<core::Mat> {
    unsafe { sys::cv_findFundamentalMat_Mat_Mat_Mat_int_double_double(points1.as_raw_Mat(), points2.as_raw_Mat(), mask.as_raw_Mat(), method, ransac_reproj_threshold, confidence) }.into_result().map(|ptr| core::Mat { ptr })
}

/// Finds a perspective transformation between two planes.
/// 
/// ## Parameters
/// * srcPoints: Coordinates of the points in the original plane, a matrix of the type CV_32FC2
/// or vector\<Point2f\> .
/// * dstPoints: Coordinates of the points in the target plane, a matrix of the type CV_32FC2 or
/// a vector\<Point2f\> .
/// * method: Method used to compute a homography matrix. The following methods are possible:
/// *   **0** - a regular method using all the points, i.e., the least squares method
/// *   **RANSAC** - RANSAC-based robust method
/// *   **LMEDS** - Least-Median robust method
/// *   **RHO** - PROSAC-based robust method
/// * ransacReprojThreshold: Maximum allowed reprojection error to treat a point pair as an inlier
/// (used in the RANSAC and RHO methods only). That is, if
/// <div lang='latex'>\| \texttt{dstPoints} _i -  \texttt{convertPointsHomogeneous} ( \texttt{H} * \texttt{srcPoints} _i) \|_2  >  \texttt{ransacReprojThreshold}</div>
/// then the point <span lang='latex'>i</span> is considered as an outlier. If srcPoints and dstPoints are measured in pixels,
/// it usually makes sense to set this parameter somewhere in the range of 1 to 10.
/// * mask: Optional output mask set by a robust method ( RANSAC or LMEDS ). Note that the input
/// mask values are ignored.
/// * maxIters: The maximum number of RANSAC iterations.
/// * confidence: Confidence level, between 0 and 1.
/// 
/// The function finds and returns the perspective transformation <span lang='latex'>H</span> between the source and the
/// destination planes:
/// 
/// <div lang='latex'>s_i  \vecthree{x'_i}{y'_i}{1} \sim H  \vecthree{x_i}{y_i}{1}</div>
/// 
/// so that the back-projection error
/// 
/// <div lang='latex'>\sum _i \left ( x'_i- \frac{h_{11} x_i + h_{12} y_i + h_{13}}{h_{31} x_i + h_{32} y_i + h_{33}} \right )^2+ \left ( y'_i- \frac{h_{21} x_i + h_{22} y_i + h_{23}}{h_{31} x_i + h_{32} y_i + h_{33}} \right )^2</div>
/// 
/// is minimized. If the parameter method is set to the default value 0, the function uses all the point
/// pairs to compute an initial homography estimate with a simple least-squares scheme.
/// 
/// However, if not all of the point pairs ( <span lang='latex'>srcPoints_i</span>, <span lang='latex'>dstPoints_i</span> ) fit the rigid perspective
/// transformation (that is, there are some outliers), this initial estimate will be poor. In this case,
/// you can use one of the three robust methods. The methods RANSAC, LMeDS and RHO try many different
/// random subsets of the corresponding point pairs (of four pairs each, collinear pairs are discarded), estimate the homography matrix
/// using this subset and a simple least-squares algorithm, and then compute the quality/goodness of the
/// computed homography (which is the number of inliers for RANSAC or the least median re-projection error for
/// LMeDS). The best subset is then used to produce the initial estimate of the homography matrix and
/// the mask of inliers/outliers.
/// 
/// Regardless of the method, robust or not, the computed homography matrix is refined further (using
/// inliers only in case of a robust method) with the Levenberg-Marquardt method to reduce the
/// re-projection error even more.
/// 
/// The methods RANSAC and RHO can handle practically any ratio of outliers but need a threshold to
/// distinguish inliers from outliers. The method LMeDS does not need any threshold but it works
/// correctly only when there are more than 50% of inliers. Finally, if there are no outliers and the
/// noise is rather small, use the default method (method=0).
/// 
/// The function is used to find initial intrinsic and extrinsic matrices. Homography matrix is
/// determined up to a scale. Thus, it is normalized so that <span lang='latex'>h_{33}=1</span>. Note that whenever an <span lang='latex'>H</span> matrix
/// cannot be estimated, an empty one will be returned.
/// 
/// ## See also
/// getAffineTransform, estimateAffine2D, estimateAffinePartial2D, getPerspectiveTransform, warpPerspective,
/// perspectiveTransform
/// 
/// ## Overloaded parameters
///
/// ## C++ default parameters
/// * method: 0
/// * ransac_reproj_threshold: 3
pub fn find_homography(src_points: &core::Mat, dst_points: &core::Mat, mask: &mut core::Mat, method: i32, ransac_reproj_threshold: f64) -> Result<core::Mat> {
    unsafe { sys::cv_findHomography_Mat_Mat_Mat_int_double(src_points.as_raw_Mat(), dst_points.as_raw_Mat(), mask.as_raw_Mat(), method, ransac_reproj_threshold) }.into_result().map(|ptr| core::Mat { ptr })
}

/// Finds a perspective transformation between two planes.
/// 
/// ## Parameters
/// * srcPoints: Coordinates of the points in the original plane, a matrix of the type CV_32FC2
/// or vector\<Point2f\> .
/// * dstPoints: Coordinates of the points in the target plane, a matrix of the type CV_32FC2 or
/// a vector\<Point2f\> .
/// * method: Method used to compute a homography matrix. The following methods are possible:
/// *   **0** - a regular method using all the points, i.e., the least squares method
/// *   **RANSAC** - RANSAC-based robust method
/// *   **LMEDS** - Least-Median robust method
/// *   **RHO** - PROSAC-based robust method
/// * ransacReprojThreshold: Maximum allowed reprojection error to treat a point pair as an inlier
/// (used in the RANSAC and RHO methods only). That is, if
/// <div lang='latex'>\| \texttt{dstPoints} _i -  \texttt{convertPointsHomogeneous} ( \texttt{H} * \texttt{srcPoints} _i) \|_2  >  \texttt{ransacReprojThreshold}</div>
/// then the point <span lang='latex'>i</span> is considered as an outlier. If srcPoints and dstPoints are measured in pixels,
/// it usually makes sense to set this parameter somewhere in the range of 1 to 10.
/// * mask: Optional output mask set by a robust method ( RANSAC or LMEDS ). Note that the input
/// mask values are ignored.
/// * maxIters: The maximum number of RANSAC iterations.
/// * confidence: Confidence level, between 0 and 1.
/// 
/// The function finds and returns the perspective transformation <span lang='latex'>H</span> between the source and the
/// destination planes:
/// 
/// <div lang='latex'>s_i  \vecthree{x'_i}{y'_i}{1} \sim H  \vecthree{x_i}{y_i}{1}</div>
/// 
/// so that the back-projection error
/// 
/// <div lang='latex'>\sum _i \left ( x'_i- \frac{h_{11} x_i + h_{12} y_i + h_{13}}{h_{31} x_i + h_{32} y_i + h_{33}} \right )^2+ \left ( y'_i- \frac{h_{21} x_i + h_{22} y_i + h_{23}}{h_{31} x_i + h_{32} y_i + h_{33}} \right )^2</div>
/// 
/// is minimized. If the parameter method is set to the default value 0, the function uses all the point
/// pairs to compute an initial homography estimate with a simple least-squares scheme.
/// 
/// However, if not all of the point pairs ( <span lang='latex'>srcPoints_i</span>, <span lang='latex'>dstPoints_i</span> ) fit the rigid perspective
/// transformation (that is, there are some outliers), this initial estimate will be poor. In this case,
/// you can use one of the three robust methods. The methods RANSAC, LMeDS and RHO try many different
/// random subsets of the corresponding point pairs (of four pairs each, collinear pairs are discarded), estimate the homography matrix
/// using this subset and a simple least-squares algorithm, and then compute the quality/goodness of the
/// computed homography (which is the number of inliers for RANSAC or the least median re-projection error for
/// LMeDS). The best subset is then used to produce the initial estimate of the homography matrix and
/// the mask of inliers/outliers.
/// 
/// Regardless of the method, robust or not, the computed homography matrix is refined further (using
/// inliers only in case of a robust method) with the Levenberg-Marquardt method to reduce the
/// re-projection error even more.
/// 
/// The methods RANSAC and RHO can handle practically any ratio of outliers but need a threshold to
/// distinguish inliers from outliers. The method LMeDS does not need any threshold but it works
/// correctly only when there are more than 50% of inliers. Finally, if there are no outliers and the
/// noise is rather small, use the default method (method=0).
/// 
/// The function is used to find initial intrinsic and extrinsic matrices. Homography matrix is
/// determined up to a scale. Thus, it is normalized so that <span lang='latex'>h_{33}=1</span>. Note that whenever an <span lang='latex'>H</span> matrix
/// cannot be estimated, an empty one will be returned.
/// 
/// ## See also
/// getAffineTransform, estimateAffine2D, estimateAffinePartial2D, getPerspectiveTransform, warpPerspective,
/// perspectiveTransform
///
/// ## C++ default parameters
/// * method: 0
/// * ransac_reproj_threshold: 3
/// * mask: noArray()
/// * max_iters: 2000
/// * confidence: 0.995
pub fn find_homography_full(src_points: &core::Mat, dst_points: &core::Mat, method: i32, ransac_reproj_threshold: f64, mask: &mut core::Mat, max_iters: i32, confidence: f64) -> Result<core::Mat> {
    unsafe { sys::cv_findHomography_Mat_Mat_int_double_Mat_int_double(src_points.as_raw_Mat(), dst_points.as_raw_Mat(), method, ransac_reproj_threshold, mask.as_raw_Mat(), max_iters, confidence) }.into_result().map(|ptr| core::Mat { ptr })
}

/// Performs camera calibaration
/// 
/// ## Parameters
/// * objectPoints: vector of vectors of calibration pattern points in the calibration pattern
/// coordinate space.
/// * imagePoints: vector of vectors of the projections of calibration pattern points.
/// imagePoints.size() and objectPoints.size() and imagePoints[i].size() must be equal to
/// objectPoints[i].size() for each i.
/// * image_size: Size of the image used only to initialize the intrinsic camera matrix.
/// * K: Output 3x3 floating-point camera matrix
/// <span lang='latex'>A = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{1}</span> . If
/// fisheye::CALIB_USE_INTRINSIC_GUESS/ is specified, some or all of fx, fy, cx, cy must be
/// initialized before calling the function.
/// * D: Output vector of distortion coefficients <span lang='latex'>(k_1, k_2, k_3, k_4)</span>.
/// * rvecs: Output vector of rotation vectors (see Rodrigues ) estimated for each pattern view.
/// That is, each k-th rotation vector together with the corresponding k-th translation vector (see
/// the next output parameter description) brings the calibration pattern from the model coordinate
/// space (in which object points are specified) to the world coordinate space, that is, a real
/// position of the calibration pattern in the k-th pattern view (k=0.. *M* -1).
/// * tvecs: Output vector of translation vectors estimated for each pattern view.
/// * flags: Different flags that may be zero or a combination of the following values:
/// *   **fisheye::CALIB_USE_INTRINSIC_GUESS** cameraMatrix contains valid initial values of
/// fx, fy, cx, cy that are optimized further. Otherwise, (cx, cy) is initially set to the image
/// center ( imageSize is used), and focal distances are computed in a least-squares fashion.
/// *   **fisheye::CALIB_RECOMPUTE_EXTRINSIC** Extrinsic will be recomputed after each iteration
/// of intrinsic optimization.
/// *   **fisheye::CALIB_CHECK_COND** The functions will check validity of condition number.
/// *   **fisheye::CALIB_FIX_SKEW** Skew coefficient (alpha) is set to zero and stay zero.
/// *   **fisheye::CALIB_FIX_K1..fisheye::CALIB_FIX_K4** Selected distortion coefficients
/// are set to zeros and stay zero.
/// *   **fisheye::CALIB_FIX_PRINCIPAL_POINT** The principal point is not changed during the global
/// optimization. It stays at the center or at a different location specified when CALIB_USE_INTRINSIC_GUESS is set too.
/// * criteria: Termination criteria for the iterative optimization algorithm.
///
/// ## C++ default parameters
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT + TermCriteria::EPS, 100, DBL_EPSILON)
pub fn calibrate(object_points: &types::VectorOfMat, image_points: &types::VectorOfMat, image_size: core::Size, k: &mut core::Mat, d: &mut core::Mat, rvecs: &mut types::VectorOfMat, tvecs: &mut types::VectorOfMat, flags: i32, criteria: &core::TermCriteria) -> Result<f64> {
    unsafe { sys::cv_fisheye_calibrate_VectorOfMat_VectorOfMat_Size_Mat_Mat_VectorOfMat_VectorOfMat_int_TermCriteria(object_points.as_raw_VectorOfMat(), image_points.as_raw_VectorOfMat(), image_size, k.as_raw_Mat(), d.as_raw_Mat(), rvecs.as_raw_VectorOfMat(), tvecs.as_raw_VectorOfMat(), flags, criteria.as_raw_TermCriteria()) }.into_result()
}

/// Distorts 2D points using fisheye model.
/// 
/// ## Parameters
/// * undistorted: Array of object points, 1xN/Nx1 2-channel (or vector\<Point2f\> ), where N is
/// the number of points in the view.
/// * K: Camera matrix <span lang='latex'>K = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{_1}</span>.
/// * D: Input vector of distortion coefficients <span lang='latex'>(k_1, k_2, k_3, k_4)</span>.
/// * alpha: The skew coefficient.
/// * distorted: Output array of image points, 1xN/Nx1 2-channel, or vector\<Point2f\> .
/// 
/// Note that the function assumes the camera matrix of the undistorted points to be identity.
/// This means if you want to transform back points undistorted with undistortPoints() you have to
/// multiply them with <span lang='latex'>P^{-1}</span>.
///
/// ## C++ default parameters
/// * alpha: 0
pub fn distort_points(undistorted: &core::Mat, distorted: &mut core::Mat, k: &core::Mat, d: &core::Mat, alpha: f64) -> Result<()> {
    unsafe { sys::cv_fisheye_distortPoints_Mat_Mat_Mat_Mat_double(undistorted.as_raw_Mat(), distorted.as_raw_Mat(), k.as_raw_Mat(), d.as_raw_Mat(), alpha) }.into_result()
}

/// Estimates new camera matrix for undistortion or rectification.
/// 
/// ## Parameters
/// * K: Camera matrix <span lang='latex'>K = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{_1}</span>.
/// * image_size: 
/// * D: Input vector of distortion coefficients <span lang='latex'>(k_1, k_2, k_3, k_4)</span>.
/// * R: Rectification transformation in the object space: 3x3 1-channel, or vector: 3x1/1x3
/// 1-channel or 1x1 3-channel
/// * P: New camera matrix (3x3) or new projection matrix (3x4)
/// * balance: Sets the new focal length in range between the min focal length and the max focal
/// length. Balance is in range of [0, 1].
/// * new_size: 
/// * fov_scale: Divisor for new focal length.
///
/// ## C++ default parameters
/// * balance: 0.0
/// * new_size: Size()
/// * fov_scale: 1.0
pub fn estimate_new_camera_matrix_for_undistort_rectify(k: &core::Mat, d: &core::Mat, image_size: core::Size, r: &core::Mat, p: &mut core::Mat, balance: f64, new_size: core::Size, fov_scale: f64) -> Result<()> {
    unsafe { sys::cv_fisheye_estimateNewCameraMatrixForUndistortRectify_Mat_Mat_Size_Mat_Mat_double_Size_double(k.as_raw_Mat(), d.as_raw_Mat(), image_size, r.as_raw_Mat(), p.as_raw_Mat(), balance, new_size, fov_scale) }.into_result()
}

/// Computes undistortion and rectification maps for image transform by cv::remap(). If D is empty zero
/// distortion is used, if R or P is empty identity matrixes are used.
/// 
/// ## Parameters
/// * K: Camera matrix <span lang='latex'>K = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{_1}</span>.
/// * D: Input vector of distortion coefficients <span lang='latex'>(k_1, k_2, k_3, k_4)</span>.
/// * R: Rectification transformation in the object space: 3x3 1-channel, or vector: 3x1/1x3
/// 1-channel or 1x1 3-channel
/// * P: New camera matrix (3x3) or new projection matrix (3x4)
/// * size: Undistorted image size.
/// * m1type: Type of the first output map that can be CV_32FC1 or CV_16SC2 . See convertMaps()
/// for details.
/// * map1: The first output map.
/// * map2: The second output map.
pub fn init_undistort_rectify_map(k: &core::Mat, d: &core::Mat, r: &core::Mat, p: &core::Mat, size: core::Size, m1type: i32, map1: &mut core::Mat, map2: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_fisheye_initUndistortRectifyMap_Mat_Mat_Mat_Mat_Size_int_Mat_Mat(k.as_raw_Mat(), d.as_raw_Mat(), r.as_raw_Mat(), p.as_raw_Mat(), size, m1type, map1.as_raw_Mat(), map2.as_raw_Mat()) }.into_result()
}

/// Projects points using fisheye model
/// 
/// ## Parameters
/// * objectPoints: Array of object points, 1xN/Nx1 3-channel (or vector\<Point3f\> ), where N is
/// the number of points in the view.
/// * imagePoints: Output array of image points, 2xN/Nx2 1-channel or 1xN/Nx1 2-channel, or
/// vector\<Point2f\>.
/// * affine: 
/// * K: Camera matrix <span lang='latex'>K = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{_1}</span>.
/// * D: Input vector of distortion coefficients <span lang='latex'>(k_1, k_2, k_3, k_4)</span>.
/// * alpha: The skew coefficient.
/// * jacobian: Optional output 2Nx15 jacobian matrix of derivatives of image points with respect
/// to components of the focal lengths, coordinates of the principal point, distortion coefficients,
/// rotation vector, translation vector, and the skew. In the old interface different components of
/// the jacobian are returned via different output parameters.
/// 
/// The function computes projections of 3D points to the image plane given intrinsic and extrinsic
/// camera parameters. Optionally, the function computes Jacobians - matrices of partial derivatives of
/// image points coordinates (as functions of all the input parameters) with respect to the particular
/// parameters, intrinsic and/or extrinsic.
/// 
/// ## Overloaded parameters
///
/// ## C++ default parameters
/// * alpha: 0
/// * jacobian: noArray()
pub fn fisheye_project_points(object_points: &core::Mat, image_points: &mut core::Mat, rvec: &core::Mat, tvec: &core::Mat, k: &core::Mat, d: &core::Mat, alpha: f64, jacobian: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_fisheye_projectPoints_Mat_Mat_Mat_Mat_Mat_Mat_double_Mat(object_points.as_raw_Mat(), image_points.as_raw_Mat(), rvec.as_raw_Mat(), tvec.as_raw_Mat(), k.as_raw_Mat(), d.as_raw_Mat(), alpha, jacobian.as_raw_Mat()) }.into_result()
}

/// Performs stereo calibration
/// 
/// ## Parameters
/// * objectPoints: Vector of vectors of the calibration pattern points.
/// * imagePoints1: Vector of vectors of the projections of the calibration pattern points,
/// observed by the first camera.
/// * imagePoints2: Vector of vectors of the projections of the calibration pattern points,
/// observed by the second camera.
/// * K1: Input/output first camera matrix:
/// <span lang='latex'>\vecthreethree{f_x^{(j)}}{0}{c_x^{(j)}}{0}{f_y^{(j)}}{c_y^{(j)}}{0}{0}{1}</span> , <span lang='latex'>j = 0,\, 1</span> . If
/// any of fisheye::CALIB_USE_INTRINSIC_GUESS , fisheye::CALIB_FIX_INTRINSIC are specified,
/// some or all of the matrix components must be initialized.
/// * D1: Input/output vector of distortion coefficients <span lang='latex'>(k_1, k_2, k_3, k_4)</span> of 4 elements.
/// * K2: Input/output second camera matrix. The parameter is similar to K1 .
/// * D2: Input/output lens distortion coefficients for the second camera. The parameter is
/// similar to D1 .
/// * imageSize: Size of the image used only to initialize intrinsic camera matrix.
/// * R: Output rotation matrix between the 1st and the 2nd camera coordinate systems.
/// * T: Output translation vector between the coordinate systems of the cameras.
/// * flags: Different flags that may be zero or a combination of the following values:
/// *   **fisheye::CALIB_FIX_INTRINSIC** Fix K1, K2? and D1, D2? so that only R, T matrices
/// are estimated.
/// *   **fisheye::CALIB_USE_INTRINSIC_GUESS** K1, K2 contains valid initial values of
/// fx, fy, cx, cy that are optimized further. Otherwise, (cx, cy) is initially set to the image
/// center (imageSize is used), and focal distances are computed in a least-squares fashion.
/// *   **fisheye::CALIB_RECOMPUTE_EXTRINSIC** Extrinsic will be recomputed after each iteration
/// of intrinsic optimization.
/// *   **fisheye::CALIB_CHECK_COND** The functions will check validity of condition number.
/// *   **fisheye::CALIB_FIX_SKEW** Skew coefficient (alpha) is set to zero and stay zero.
/// *   **fisheye::CALIB_FIX_K1..4** Selected distortion coefficients are set to zeros and stay
/// zero.
/// * criteria: Termination criteria for the iterative optimization algorithm.
///
/// ## C++ default parameters
/// * flags: fisheye::CALIB_FIX_INTRINSIC
/// * criteria: TermCriteria(TermCriteria::COUNT + TermCriteria::EPS, 100, DBL_EPSILON)
pub fn stereo_calibrate(object_points: &types::VectorOfMat, image_points1: &types::VectorOfMat, image_points2: &types::VectorOfMat, k1: &mut core::Mat, d1: &mut core::Mat, k2: &mut core::Mat, d2: &mut core::Mat, image_size: core::Size, r: &mut core::Mat, t: &mut core::Mat, flags: i32, criteria: &core::TermCriteria) -> Result<f64> {
    unsafe { sys::cv_fisheye_stereoCalibrate_VectorOfMat_VectorOfMat_VectorOfMat_Mat_Mat_Mat_Mat_Size_Mat_Mat_int_TermCriteria(object_points.as_raw_VectorOfMat(), image_points1.as_raw_VectorOfMat(), image_points2.as_raw_VectorOfMat(), k1.as_raw_Mat(), d1.as_raw_Mat(), k2.as_raw_Mat(), d2.as_raw_Mat(), image_size, r.as_raw_Mat(), t.as_raw_Mat(), flags, criteria.as_raw_TermCriteria()) }.into_result()
}

/// Stereo rectification for fisheye camera model
/// 
/// ## Parameters
/// * K1: First camera matrix.
/// * D1: First camera distortion parameters.
/// * K2: Second camera matrix.
/// * D2: Second camera distortion parameters.
/// * imageSize: Size of the image used for stereo calibration.
/// * R: Rotation matrix between the coordinate systems of the first and the second
/// cameras.
/// * tvec: Translation vector between coordinate systems of the cameras.
/// * R1: Output 3x3 rectification transform (rotation matrix) for the first camera.
/// * R2: Output 3x3 rectification transform (rotation matrix) for the second camera.
/// * P1: Output 3x4 projection matrix in the new (rectified) coordinate systems for the first
/// camera.
/// * P2: Output 3x4 projection matrix in the new (rectified) coordinate systems for the second
/// camera.
/// * Q: Output <span lang='latex'>4 \times 4</span> disparity-to-depth mapping matrix (see reprojectImageTo3D ).
/// * flags: Operation flags that may be zero or CALIB_ZERO_DISPARITY . If the flag is set,
/// the function makes the principal points of each camera have the same pixel coordinates in the
/// rectified views. And if the flag is not set, the function may still shift the images in the
/// horizontal or vertical direction (depending on the orientation of epipolar lines) to maximize the
/// useful image area.
/// * newImageSize: New image resolution after rectification. The same size should be passed to
/// initUndistortRectifyMap (see the stereo_calib.cpp sample in OpenCV samples directory). When (0,0)
/// is passed (default), it is set to the original imageSize . Setting it to larger value can help you
/// preserve details in the original image, especially when there is a big radial distortion.
/// * balance: Sets the new focal length in range between the min focal length and the max focal
/// length. Balance is in range of [0, 1].
/// * fov_scale: Divisor for new focal length.
///
/// ## C++ default parameters
/// * new_image_size: Size()
/// * balance: 0.0
/// * fov_scale: 1.0
pub fn stereo_rectify(k1: &core::Mat, d1: &core::Mat, k2: &core::Mat, d2: &core::Mat, image_size: core::Size, r: &core::Mat, tvec: &core::Mat, r1: &mut core::Mat, r2: &mut core::Mat, p1: &mut core::Mat, p2: &mut core::Mat, q: &mut core::Mat, flags: i32, new_image_size: core::Size, balance: f64, fov_scale: f64) -> Result<()> {
    unsafe { sys::cv_fisheye_stereoRectify_Mat_Mat_Mat_Mat_Size_Mat_Mat_Mat_Mat_Mat_Mat_Mat_int_Size_double_double(k1.as_raw_Mat(), d1.as_raw_Mat(), k2.as_raw_Mat(), d2.as_raw_Mat(), image_size, r.as_raw_Mat(), tvec.as_raw_Mat(), r1.as_raw_Mat(), r2.as_raw_Mat(), p1.as_raw_Mat(), p2.as_raw_Mat(), q.as_raw_Mat(), flags, new_image_size, balance, fov_scale) }.into_result()
}

/// Transforms an image to compensate for fisheye lens distortion.
/// 
/// ## Parameters
/// * distorted: image with fisheye lens distortion.
/// * undistorted: Output image with compensated fisheye lens distortion.
/// * K: Camera matrix <span lang='latex'>K = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{_1}</span>.
/// * D: Input vector of distortion coefficients <span lang='latex'>(k_1, k_2, k_3, k_4)</span>.
/// * Knew: Camera matrix of the distorted image. By default, it is the identity matrix but you
/// may additionally scale and shift the result by using a different matrix.
/// * new_size: 
/// 
/// The function transforms an image to compensate radial and tangential lens distortion.
/// 
/// The function is simply a combination of fisheye::initUndistortRectifyMap (with unity R ) and remap
/// (with bilinear interpolation). See the former function for details of the transformation being
/// performed.
/// 
/// See below the results of undistortImage.
/// *   a\) result of undistort of perspective camera model (all possible coefficients (k_1, k_2, k_3,
/// k_4, k_5, k_6) of distortion were optimized under calibration)
/// *   b\) result of fisheye::undistortImage of fisheye camera model (all possible coefficients (k_1, k_2,
/// k_3, k_4) of fisheye distortion were optimized under calibration)
/// *   c\) original image was captured with fisheye lens
/// 
/// Pictures a) and b) almost the same. But if we consider points of image located far from the center
/// of image, we can notice that on image a) these points are distorted.
/// 
/// ![image](https://docs.opencv.org/3.4.6/fisheye_undistorted.jpg)
///
/// ## C++ default parameters
/// * knew: cv::noArray()
/// * new_size: Size()
pub fn fisheye_undistort_image(distorted: &core::Mat, undistorted: &mut core::Mat, k: &core::Mat, d: &core::Mat, knew: &core::Mat, new_size: core::Size) -> Result<()> {
    unsafe { sys::cv_fisheye_undistortImage_Mat_Mat_Mat_Mat_Mat_Size(distorted.as_raw_Mat(), undistorted.as_raw_Mat(), k.as_raw_Mat(), d.as_raw_Mat(), knew.as_raw_Mat(), new_size) }.into_result()
}

/// Undistorts 2D points using fisheye model
/// 
/// ## Parameters
/// * distorted: Array of object points, 1xN/Nx1 2-channel (or vector\<Point2f\> ), where N is the
/// number of points in the view.
/// * K: Camera matrix <span lang='latex'>K = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{_1}</span>.
/// * D: Input vector of distortion coefficients <span lang='latex'>(k_1, k_2, k_3, k_4)</span>.
/// * R: Rectification transformation in the object space: 3x3 1-channel, or vector: 3x1/1x3
/// 1-channel or 1x1 3-channel
/// * P: New camera matrix (3x3) or new projection matrix (3x4)
/// * undistorted: Output array of image points, 1xN/Nx1 2-channel, or vector\<Point2f\> .
///
/// ## C++ default parameters
/// * r: noArray()
/// * p: noArray()
pub fn fisheye_undistort_points(distorted: &core::Mat, undistorted: &mut core::Mat, k: &core::Mat, d: &core::Mat, r: &core::Mat, p: &core::Mat) -> Result<()> {
    unsafe { sys::cv_fisheye_undistortPoints_Mat_Mat_Mat_Mat_Mat_Mat(distorted.as_raw_Mat(), undistorted.as_raw_Mat(), k.as_raw_Mat(), d.as_raw_Mat(), r.as_raw_Mat(), p.as_raw_Mat()) }.into_result()
}

/// Returns the new camera matrix based on the free scaling parameter.
/// 
/// ## Parameters
/// * cameraMatrix: Input camera matrix.
/// * distCoeffs: Input vector of distortion coefficients
/// <span lang='latex'>(k_1, k_2, p_1, p_2[, k_3[, k_4, k_5, k_6 [, s_1, s_2, s_3, s_4[, \tau_x, \tau_y]]]])</span> of
/// 4, 5, 8, 12 or 14 elements. If the vector is NULL/empty, the zero distortion coefficients are
/// assumed.
/// * imageSize: Original image size.
/// * alpha: Free scaling parameter between 0 (when all the pixels in the undistorted image are
/// valid) and 1 (when all the source image pixels are retained in the undistorted image). See
/// stereoRectify for details.
/// * newImgSize: Image size after rectification. By default, it is set to imageSize .
/// * validPixROI: Optional output rectangle that outlines all-good-pixels region in the
/// undistorted image. See roi1, roi2 description in stereoRectify .
/// * centerPrincipalPoint: Optional flag that indicates whether in the new camera matrix the
/// principal point should be at the image center or not. By default, the principal point is chosen to
/// best fit a subset of the source image (determined by alpha) to the corrected image.
/// ## Returns
/// new_camera_matrix Output new camera matrix.
/// 
/// The function computes and returns the optimal new camera matrix based on the free scaling parameter.
/// By varying this parameter, you may retrieve only sensible pixels alpha=0 , keep all the original
/// image pixels if there is valuable information in the corners alpha=1 , or get something in between.
/// When alpha\>0 , the undistorted result is likely to have some black pixels corresponding to
/// "virtual" pixels outside of the captured distorted image. The original camera matrix, distortion
/// coefficients, the computed new camera matrix, and newImageSize should be passed to
/// initUndistortRectifyMap to produce the maps for remap .
///
/// ## C++ default parameters
/// * new_img_size: Size()
/// * valid_pix_roi: 0
/// * center_principal_point: false
pub fn get_optimal_new_camera_matrix(camera_matrix: &core::Mat, dist_coeffs: &core::Mat, image_size: core::Size, alpha: f64, new_img_size: core::Size, valid_pix_roi: &mut core::Rect, center_principal_point: bool) -> Result<core::Mat> {
    unsafe { sys::cv_getOptimalNewCameraMatrix_Mat_Mat_Size_double_Size_Rect_X_bool(camera_matrix.as_raw_Mat(), dist_coeffs.as_raw_Mat(), image_size, alpha, new_img_size, valid_pix_roi, center_principal_point) }.into_result().map(|ptr| core::Mat { ptr })
}

/// computes valid disparity ROI from the valid ROIs of the rectified images (that are returned by cv::stereoRectify())
pub fn get_valid_disparity_roi(roi1: core::Rect, roi2: core::Rect, min_disparity: i32, number_of_disparities: i32, sad_window_size: i32) -> Result<core::Rect> {
    unsafe { sys::cv_getValidDisparityROI_Rect_Rect_int_int_int(roi1, roi2, min_disparity, number_of_disparities, sad_window_size) }.into_result()
}

/// Finds an initial camera matrix from 3D-2D point correspondences.
/// 
/// ## Parameters
/// * objectPoints: Vector of vectors of the calibration pattern points in the calibration pattern
/// coordinate space. In the old interface all the per-view vectors are concatenated. See
/// calibrateCamera for details.
/// * imagePoints: Vector of vectors of the projections of the calibration pattern points. In the
/// old interface all the per-view vectors are concatenated.
/// * imageSize: Image size in pixels used to initialize the principal point.
/// * aspectRatio: If it is zero or negative, both <span lang='latex'>f_x</span> and <span lang='latex'>f_y</span> are estimated independently.
/// Otherwise, <span lang='latex'>f_x = f_y * \texttt{aspectRatio}</span> .
/// 
/// The function estimates and returns an initial camera matrix for the camera calibration process.
/// Currently, the function only supports planar calibration patterns, which are patterns where each
/// object point has z-coordinate =0.
///
/// ## C++ default parameters
/// * aspect_ratio: 1.0
pub fn init_camera_matrix_2d(object_points: &types::VectorOfMat, image_points: &types::VectorOfMat, image_size: core::Size, aspect_ratio: f64) -> Result<core::Mat> {
    unsafe { sys::cv_initCameraMatrix2D_VectorOfMat_VectorOfMat_Size_double(object_points.as_raw_VectorOfMat(), image_points.as_raw_VectorOfMat(), image_size, aspect_ratio) }.into_result().map(|ptr| core::Mat { ptr })
}

/// Computes partial derivatives of the matrix product for each multiplied matrix.
/// 
/// ## Parameters
/// * A: First multiplied matrix.
/// * B: Second multiplied matrix.
/// * dABdA: First output derivative matrix d(A\*B)/dA of size
/// <span lang='latex'>\texttt{A.rows*B.cols} \times {A.rows*A.cols}</span> .
/// * dABdB: Second output derivative matrix d(A\*B)/dB of size
/// <span lang='latex'>\texttt{A.rows*B.cols} \times {B.rows*B.cols}</span> .
/// 
/// The function computes partial derivatives of the elements of the matrix product <span lang='latex'>A*B</span> with regard to
/// the elements of each of the two input matrices. The function is used to compute the Jacobian
/// matrices in stereoCalibrate but can also be used in any other similar optimization function.
pub fn mat_mul_deriv(a: &core::Mat, b: &core::Mat, d_a_bd_a: &mut core::Mat, d_a_bd_b: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_matMulDeriv_Mat_Mat_Mat_Mat(a.as_raw_Mat(), b.as_raw_Mat(), d_a_bd_a.as_raw_Mat(), d_a_bd_b.as_raw_Mat()) }.into_result()
}

/// Projects 3D points to an image plane.
/// 
/// ## Parameters
/// * objectPoints: Array of object points, 3xN/Nx3 1-channel or 1xN/Nx1 3-channel (or
/// vector\<Point3f\> ), where N is the number of points in the view.
/// * rvec: Rotation vector. See Rodrigues for details.
/// * tvec: Translation vector.
/// * cameraMatrix: Camera matrix <span lang='latex'>A = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{_1}</span> .
/// * distCoeffs: Input vector of distortion coefficients
/// <span lang='latex'>(k_1, k_2, p_1, p_2[, k_3[, k_4, k_5, k_6 [, s_1, s_2, s_3, s_4[, \tau_x, \tau_y]]]])</span> of
/// 4, 5, 8, 12 or 14 elements. If the vector is empty, the zero distortion coefficients are assumed.
/// * imagePoints: Output array of image points, 2xN/Nx2 1-channel or 1xN/Nx1 2-channel, or
/// vector\<Point2f\> .
/// * jacobian: Optional output 2Nx(10+\<numDistCoeffs\>) jacobian matrix of derivatives of image
/// points with respect to components of the rotation vector, translation vector, focal lengths,
/// coordinates of the principal point and the distortion coefficients. In the old interface different
/// components of the jacobian are returned via different output parameters.
/// * aspectRatio: Optional "fixed aspect ratio" parameter. If the parameter is not 0, the
/// function assumes that the aspect ratio (*fx/fy*) is fixed and correspondingly adjusts the jacobian
/// matrix.
/// 
/// The function computes projections of 3D points to the image plane given intrinsic and extrinsic
/// camera parameters. Optionally, the function computes Jacobians - matrices of partial derivatives of
/// image points coordinates (as functions of all the input parameters) with respect to the particular
/// parameters, intrinsic and/or extrinsic. The Jacobians are used during the global optimization in
/// calibrateCamera, solvePnP, and stereoCalibrate . The function itself can also be used to compute a
/// re-projection error given the current intrinsic and extrinsic parameters.
/// 
/// 
/// Note: By setting rvec=tvec=(0,0,0) or by setting cameraMatrix to a 3x3 identity matrix, or by
/// passing zero distortion coefficients, you can get various useful partial cases of the function. This
/// means that you can compute the distorted coordinates for a sparse set of points or apply a
/// perspective transformation (and also compute the derivatives) in the ideal zero-distortion setup.
///
/// ## C++ default parameters
/// * jacobian: noArray()
/// * aspect_ratio: 0
pub fn project_points(object_points: &core::Mat, rvec: &core::Mat, tvec: &core::Mat, camera_matrix: &core::Mat, dist_coeffs: &core::Mat, image_points: &mut core::Mat, jacobian: &mut core::Mat, aspect_ratio: f64) -> Result<()> {
    unsafe { sys::cv_projectPoints_Mat_Mat_Mat_Mat_Mat_Mat_Mat_double(object_points.as_raw_Mat(), rvec.as_raw_Mat(), tvec.as_raw_Mat(), camera_matrix.as_raw_Mat(), dist_coeffs.as_raw_Mat(), image_points.as_raw_Mat(), jacobian.as_raw_Mat(), aspect_ratio) }.into_result()
}

/// Recover relative camera rotation and translation from an estimated essential matrix and the
/// corresponding points in two images, using cheirality check. Returns the number of inliers which pass
/// the check.
/// 
/// ## Parameters
/// * E: The input essential matrix.
/// * points1: Array of N 2D points from the first image. The point coordinates should be
/// floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1 .
/// * cameraMatrix: Camera matrix <span lang='latex'>K = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{1}</span> .
/// Note that this function assumes that points1 and points2 are feature points from cameras with the
/// same camera matrix.
/// * R: Recovered relative rotation.
/// * t: Recovered relative translation.
/// * mask: Input/output mask for inliers in points1 and points2.
/// :   If it is not empty, then it marks inliers in points1 and points2 for then given essential
/// matrix E. Only these inliers will be used to recover pose. In the output mask only inliers
/// which pass the cheirality check.
/// This function decomposes an essential matrix using decomposeEssentialMat and then verifies possible
/// pose hypotheses by doing cheirality check. The cheirality check basically means that the
/// triangulated 3D points should have positive depth. Some details can be found in [Nister03](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Nister03) .
/// 
/// This function can be used to process output E and mask from findEssentialMat. In this scenario,
/// points1 and points2 are the same input for findEssentialMat. :
/// ```ignore
/// // Example. Estimation of fundamental matrix using the RANSAC algorithm
/// int point_count = 100;
/// vector<Point2f> points1(point_count);
/// vector<Point2f> points2(point_count);
/// 
/// // initialize the points here ...
/// for( int i = 0; i < point_count; i++ )
/// {
/// points1[i] = ...;
/// points2[i] = ...;
/// }
/// 
/// // cametra matrix with both focal lengths = 1, and principal point = (0, 0)
/// Mat cameraMatrix = Mat::eye(3, 3, CV_64F);
/// 
/// Mat E, R, t, mask;
/// 
/// E = findEssentialMat(points1, points2, cameraMatrix, RANSAC, 0.999, 1.0, mask);
/// recoverPose(E, points1, points2, cameraMatrix, R, t, mask);
/// ```
///
/// ## C++ default parameters
/// * mask: noArray()
pub fn recover_pose_camera_with_points(e: &core::Mat, points1: &core::Mat, points2: &core::Mat, camera_matrix: &core::Mat, r: &mut core::Mat, t: &mut core::Mat, mask: &mut core::Mat) -> Result<i32> {
    unsafe { sys::cv_recoverPose_Mat_Mat_Mat_Mat_Mat_Mat_Mat(e.as_raw_Mat(), points1.as_raw_Mat(), points2.as_raw_Mat(), camera_matrix.as_raw_Mat(), r.as_raw_Mat(), t.as_raw_Mat(), mask.as_raw_Mat()) }.into_result()
}

/// Recover relative camera rotation and translation from an estimated essential matrix and the
/// corresponding points in two images, using cheirality check. Returns the number of inliers which pass
/// the check.
/// 
/// ## Parameters
/// * E: The input essential matrix.
/// * points1: Array of N 2D points from the first image. The point coordinates should be
/// floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1 .
/// * cameraMatrix: Camera matrix <span lang='latex'>K = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{1}</span> .
/// Note that this function assumes that points1 and points2 are feature points from cameras with the
/// same camera matrix.
/// * R: Recovered relative rotation.
/// * t: Recovered relative translation.
/// * mask: Input/output mask for inliers in points1 and points2.
/// :   If it is not empty, then it marks inliers in points1 and points2 for then given essential
/// matrix E. Only these inliers will be used to recover pose. In the output mask only inliers
/// which pass the cheirality check.
/// This function decomposes an essential matrix using decomposeEssentialMat and then verifies possible
/// pose hypotheses by doing cheirality check. The cheirality check basically means that the
/// triangulated 3D points should have positive depth. Some details can be found in [Nister03](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Nister03) .
/// 
/// This function can be used to process output E and mask from findEssentialMat. In this scenario,
/// points1 and points2 are the same input for findEssentialMat. :
/// ```ignore
/// // Example. Estimation of fundamental matrix using the RANSAC algorithm
/// int point_count = 100;
/// vector<Point2f> points1(point_count);
/// vector<Point2f> points2(point_count);
/// 
/// // initialize the points here ...
/// for( int i = 0; i < point_count; i++ )
/// {
/// points1[i] = ...;
/// points2[i] = ...;
/// }
/// 
/// // cametra matrix with both focal lengths = 1, and principal point = (0, 0)
/// Mat cameraMatrix = Mat::eye(3, 3, CV_64F);
/// 
/// Mat E, R, t, mask;
/// 
/// E = findEssentialMat(points1, points2, cameraMatrix, RANSAC, 0.999, 1.0, mask);
/// recoverPose(E, points1, points2, cameraMatrix, R, t, mask);
/// ```
/// 
/// 
/// ## Overloaded parameters
/// 
/// * E: The input essential matrix.
/// * points1: Array of N 2D points from the first image. The point coordinates should be
/// floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1.
/// * cameraMatrix: Camera matrix <span lang='latex'>K = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{1}</span> .
/// Note that this function assumes that points1 and points2 are feature points from cameras with the
/// same camera matrix.
/// * R: Recovered relative rotation.
/// * t: Recovered relative translation.
/// * distanceThresh: threshold distance which is used to filter out far away points (i.e. infinite points).
/// * mask: Input/output mask for inliers in points1 and points2.
/// :   If it is not empty, then it marks inliers in points1 and points2 for then given essential
/// matrix E. Only these inliers will be used to recover pose. In the output mask only inliers
/// which pass the cheirality check.
/// * triangulatedPoints: 3d points which were reconstructed by triangulation.
///
/// ## C++ default parameters
/// * mask: noArray()
/// * triangulated_points: noArray()
pub fn recover_pose_camera(e: &core::Mat, points1: &core::Mat, points2: &core::Mat, camera_matrix: &core::Mat, r: &mut core::Mat, t: &mut core::Mat, distance_thresh: f64, mask: &mut core::Mat, triangulated_points: &mut core::Mat) -> Result<i32> {
    unsafe { sys::cv_recoverPose_Mat_Mat_Mat_Mat_Mat_Mat_double_Mat_Mat(e.as_raw_Mat(), points1.as_raw_Mat(), points2.as_raw_Mat(), camera_matrix.as_raw_Mat(), r.as_raw_Mat(), t.as_raw_Mat(), distance_thresh, mask.as_raw_Mat(), triangulated_points.as_raw_Mat()) }.into_result()
}

/// Recover relative camera rotation and translation from an estimated essential matrix and the
/// corresponding points in two images, using cheirality check. Returns the number of inliers which pass
/// the check.
/// 
/// ## Parameters
/// * E: The input essential matrix.
/// * points1: Array of N 2D points from the first image. The point coordinates should be
/// floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1 .
/// * cameraMatrix: Camera matrix <span lang='latex'>K = \vecthreethree{f_x}{0}{c_x}{0}{f_y}{c_y}{0}{0}{1}</span> .
/// Note that this function assumes that points1 and points2 are feature points from cameras with the
/// same camera matrix.
/// * R: Recovered relative rotation.
/// * t: Recovered relative translation.
/// * mask: Input/output mask for inliers in points1 and points2.
/// :   If it is not empty, then it marks inliers in points1 and points2 for then given essential
/// matrix E. Only these inliers will be used to recover pose. In the output mask only inliers
/// which pass the cheirality check.
/// This function decomposes an essential matrix using decomposeEssentialMat and then verifies possible
/// pose hypotheses by doing cheirality check. The cheirality check basically means that the
/// triangulated 3D points should have positive depth. Some details can be found in [Nister03](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Nister03) .
/// 
/// This function can be used to process output E and mask from findEssentialMat. In this scenario,
/// points1 and points2 are the same input for findEssentialMat. :
/// ```ignore
/// // Example. Estimation of fundamental matrix using the RANSAC algorithm
/// int point_count = 100;
/// vector<Point2f> points1(point_count);
/// vector<Point2f> points2(point_count);
/// 
/// // initialize the points here ...
/// for( int i = 0; i < point_count; i++ )
/// {
/// points1[i] = ...;
/// points2[i] = ...;
/// }
/// 
/// // cametra matrix with both focal lengths = 1, and principal point = (0, 0)
/// Mat cameraMatrix = Mat::eye(3, 3, CV_64F);
/// 
/// Mat E, R, t, mask;
/// 
/// E = findEssentialMat(points1, points2, cameraMatrix, RANSAC, 0.999, 1.0, mask);
/// recoverPose(E, points1, points2, cameraMatrix, R, t, mask);
/// ```
/// 
/// 
/// ## Overloaded parameters
/// 
/// * E: The input essential matrix.
/// * points1: Array of N 2D points from the first image. The point coordinates should be
/// floating-point (single or double precision).
/// * points2: Array of the second image points of the same size and format as points1 .
/// * R: Recovered relative rotation.
/// * t: Recovered relative translation.
/// * focal: Focal length of the camera. Note that this function assumes that points1 and points2
/// are feature points from cameras with same focal length and principal point.
/// * pp: principal point of the camera.
/// * mask: Input/output mask for inliers in points1 and points2.
/// :   If it is not empty, then it marks inliers in points1 and points2 for then given essential
/// matrix E. Only these inliers will be used to recover pose. In the output mask only inliers
/// which pass the cheirality check.
/// 
/// This function differs from the one above that it computes camera matrix from focal length and
/// principal point:
/// 
/// <div lang='latex'>K =
/// \begin{bmatrix}
/// f & 0 & x_{pp}  \\
/// 0 & f & y_{pp}  \\
/// 0 & 0 & 1
/// \end{bmatrix}</div>
///
/// ## C++ default parameters
/// * focal: 1.0
/// * pp: Point2d(0, 0)
/// * mask: noArray()
pub fn recover_pose(e: &core::Mat, points1: &core::Mat, points2: &core::Mat, r: &mut core::Mat, t: &mut core::Mat, focal: f64, pp: core::Point2d, mask: &mut core::Mat) -> Result<i32> {
    unsafe { sys::cv_recoverPose_Mat_Mat_Mat_Mat_Mat_double_Point2d_Mat(e.as_raw_Mat(), points1.as_raw_Mat(), points2.as_raw_Mat(), r.as_raw_Mat(), t.as_raw_Mat(), focal, pp, mask.as_raw_Mat()) }.into_result()
}

/// computes the rectification transformations for 3-head camera, where all the heads are on the same line.
pub fn rectify3_collinear(camera_matrix1: &core::Mat, dist_coeffs1: &core::Mat, camera_matrix2: &core::Mat, dist_coeffs2: &core::Mat, camera_matrix3: &core::Mat, dist_coeffs3: &core::Mat, imgpt1: &types::VectorOfMat, imgpt3: &types::VectorOfMat, image_size: core::Size, r12: &core::Mat, t12: &core::Mat, r13: &core::Mat, t13: &core::Mat, r1: &mut core::Mat, r2: &mut core::Mat, r3: &mut core::Mat, p1: &mut core::Mat, p2: &mut core::Mat, p3: &mut core::Mat, q: &mut core::Mat, alpha: f64, new_img_size: core::Size, roi1: &mut core::Rect, roi2: &mut core::Rect, flags: i32) -> Result<f32> {
    unsafe { sys::cv_rectify3Collinear_Mat_Mat_Mat_Mat_Mat_Mat_VectorOfMat_VectorOfMat_Size_Mat_Mat_Mat_Mat_Mat_Mat_Mat_Mat_Mat_Mat_Mat_double_Size_Rect_X_Rect_X_int(camera_matrix1.as_raw_Mat(), dist_coeffs1.as_raw_Mat(), camera_matrix2.as_raw_Mat(), dist_coeffs2.as_raw_Mat(), camera_matrix3.as_raw_Mat(), dist_coeffs3.as_raw_Mat(), imgpt1.as_raw_VectorOfMat(), imgpt3.as_raw_VectorOfMat(), image_size, r12.as_raw_Mat(), t12.as_raw_Mat(), r13.as_raw_Mat(), t13.as_raw_Mat(), r1.as_raw_Mat(), r2.as_raw_Mat(), r3.as_raw_Mat(), p1.as_raw_Mat(), p2.as_raw_Mat(), p3.as_raw_Mat(), q.as_raw_Mat(), alpha, new_img_size, roi1, roi2, flags) }.into_result()
}

/// Reprojects a disparity image to 3D space.
/// 
/// ## Parameters
/// * disparity: Input single-channel 8-bit unsigned, 16-bit signed, 32-bit signed or 32-bit
/// floating-point disparity image. If 16-bit signed format is used, the values are assumed to have no
/// fractional bits.
/// * _3dImage: Output 3-channel floating-point image of the same size as disparity . Each
/// element of _3dImage(x,y) contains 3D coordinates of the point (x,y) computed from the disparity
/// map.
/// * Q: <span lang='latex'>4 \times 4</span> perspective transformation matrix that can be obtained with stereoRectify.
/// * handleMissingValues: Indicates, whether the function should handle missing values (i.e.
/// points where the disparity was not computed). If handleMissingValues=true, then pixels with the
/// minimal disparity that corresponds to the outliers (see StereoMatcher::compute ) are transformed
/// to 3D points with a very large Z value (currently set to 10000).
/// * ddepth: The optional output array depth. If it is -1, the output image will have CV_32F
/// depth. ddepth can also be set to CV_16S, CV_32S or CV_32F.
/// 
/// The function transforms a single-channel disparity map to a 3-channel image representing a 3D
/// surface. That is, for each pixel (x,y) and the corresponding disparity d=disparity(x,y) , it
/// computes:
/// 
/// <div lang='latex'>\begin{array}{l} [X \; Y \; Z \; W]^T =  \texttt{Q} *[x \; y \; \texttt{disparity} (x,y) \; 1]^T  \\ \texttt{\_3dImage} (x,y) = (X/W, \; Y/W, \; Z/W) \end{array}</div>
/// 
/// The matrix Q can be an arbitrary <span lang='latex'>4 \times 4</span> matrix (for example, the one computed by
/// stereoRectify). To reproject a sparse set of points {(x,y,d),...} to 3D space, use
/// perspectiveTransform .
///
/// ## C++ default parameters
/// * handle_missing_values: false
/// * ddepth: -1
pub fn reproject_image_to_3d(disparity: &core::Mat, _3d_image: &mut core::Mat, q: &core::Mat, handle_missing_values: bool, ddepth: i32) -> Result<()> {
    unsafe { sys::cv_reprojectImageTo3D_Mat_Mat_Mat_bool_int(disparity.as_raw_Mat(), _3d_image.as_raw_Mat(), q.as_raw_Mat(), handle_missing_values, ddepth) }.into_result()
}

/// Calculates the Sampson Distance between two points.
/// 
/// The function cv::sampsonDistance calculates and returns the first order approximation of the geometric error as:
/// <div lang='latex'>
/// sd( \texttt{pt1} , \texttt{pt2} )=
/// \frac{(\texttt{pt2}^t \cdot \texttt{F} \cdot \texttt{pt1})^2}
/// {((\texttt{F} \cdot \texttt{pt1})(0))^2 +
/// ((\texttt{F} \cdot \texttt{pt1})(1))^2 +
/// ((\texttt{F}^t \cdot \texttt{pt2})(0))^2 +
/// ((\texttt{F}^t \cdot \texttt{pt2})(1))^2}
/// </div>
/// The fundamental matrix may be calculated using the cv::findFundamentalMat function. See [HartleyZ00](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_HartleyZ00) 11.4.3 for details.
/// ## Parameters
/// * pt1: first homogeneous 2d point
/// * pt2: second homogeneous 2d point
/// * F: fundamental matrix
/// ## Returns
/// The computed Sampson distance.
pub fn sampson_distance(pt1: &core::Mat, pt2: &core::Mat, f: &core::Mat) -> Result<f64> {
    unsafe { sys::cv_sampsonDistance_Mat_Mat_Mat(pt1.as_raw_Mat(), pt2.as_raw_Mat(), f.as_raw_Mat()) }.into_result()
}

/// Finds an object pose from 3 3D-2D point correspondences.
/// 
/// ## Parameters
/// * objectPoints: Array of object points in the object coordinate space, 3x3 1-channel or
/// 1x3/3x1 3-channel. vector\<Point3f\> can be also passed here.
/// * imagePoints: Array of corresponding image points, 3x2 1-channel or 1x3/3x1 2-channel.
/// vector\<Point2f\> can be also passed here.
/// * cameraMatrix: Input camera matrix <span lang='latex'>A = \vecthreethree{fx}{0}{cx}{0}{fy}{cy}{0}{0}{1}</span> .
/// * distCoeffs: Input vector of distortion coefficients
/// <span lang='latex'>(k_1, k_2, p_1, p_2[, k_3[, k_4, k_5, k_6 [, s_1, s_2, s_3, s_4[, \tau_x, \tau_y]]]])</span> of
/// 4, 5, 8, 12 or 14 elements. If the vector is NULL/empty, the zero distortion coefficients are
/// assumed.
/// * rvecs: Output rotation vectors (see Rodrigues ) that, together with tvecs , brings points from
/// the model coordinate system to the camera coordinate system. A P3P problem has up to 4 solutions.
/// * tvecs: Output translation vectors.
/// * flags: Method for solving a P3P problem:
/// *   **SOLVEPNP_P3P** Method is based on the paper of X.S. Gao, X.-R. Hou, J. Tang, H.-F. Chang
/// "Complete Solution Classification for the Perspective-Three-Point Problem" ([gao2003complete](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_gao2003complete)).
/// *   **SOLVEPNP_AP3P** Method is based on the paper of Tong Ke and Stergios I. Roumeliotis.
/// "An Efficient Algebraic Solution to the Perspective-Three-Point Problem" ([Ke17](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Ke17)).
/// 
/// The function estimates the object pose given 3 object points, their corresponding image
/// projections, as well as the camera matrix and the distortion coefficients.
pub fn solve_p3p(object_points: &core::Mat, image_points: &core::Mat, camera_matrix: &core::Mat, dist_coeffs: &core::Mat, rvecs: &mut types::VectorOfMat, tvecs: &mut types::VectorOfMat, flags: i32) -> Result<i32> {
    unsafe { sys::cv_solveP3P_Mat_Mat_Mat_Mat_VectorOfMat_VectorOfMat_int(object_points.as_raw_Mat(), image_points.as_raw_Mat(), camera_matrix.as_raw_Mat(), dist_coeffs.as_raw_Mat(), rvecs.as_raw_VectorOfMat(), tvecs.as_raw_VectorOfMat(), flags) }.into_result()
}

/// Finds an object pose from 3D-2D point correspondences using the RANSAC scheme.
/// 
/// ## Parameters
/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or
/// 1xN/Nx1 3-channel, where N is the number of points. vector\<Point3f\> can be also passed here.
/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
/// where N is the number of points. vector\<Point2f\> can be also passed here.
/// * cameraMatrix: Input camera matrix <span lang='latex'>A = \vecthreethree{fx}{0}{cx}{0}{fy}{cy}{0}{0}{1}</span> .
/// * distCoeffs: Input vector of distortion coefficients
/// <span lang='latex'>(k_1, k_2, p_1, p_2[, k_3[, k_4, k_5, k_6 [, s_1, s_2, s_3, s_4[, \tau_x, \tau_y]]]])</span> of
/// 4, 5, 8, 12 or 14 elements. If the vector is NULL/empty, the zero distortion coefficients are
/// assumed.
/// * rvec: Output rotation vector (see Rodrigues ) that, together with tvec , brings points from
/// the model coordinate system to the camera coordinate system.
/// * tvec: Output translation vector.
/// * useExtrinsicGuess: Parameter used for SOLVEPNP_ITERATIVE. If true (1), the function uses
/// the provided rvec and tvec values as initial approximations of the rotation and translation
/// vectors, respectively, and further optimizes them.
/// * iterationsCount: Number of iterations.
/// * reprojectionError: Inlier threshold value used by the RANSAC procedure. The parameter value
/// is the maximum allowed distance between the observed and computed point projections to consider it
/// an inlier.
/// * confidence: The probability that the algorithm produces a useful result.
/// * inliers: Output vector that contains indices of inliers in objectPoints and imagePoints .
/// * flags: Method for solving a PnP problem (see solvePnP ).
/// 
/// The function estimates an object pose given a set of object points, their corresponding image
/// projections, as well as the camera matrix and the distortion coefficients. This function finds such
/// a pose that minimizes reprojection error, that is, the sum of squared distances between the observed
/// projections imagePoints and the projected (using projectPoints ) objectPoints. The use of RANSAC
/// makes the function resistant to outliers.
/// 
/// 
/// Note:
/// *   An example of how to use solvePNPRansac for object detection can be found at
/// opencv_source_code/samples/cpp/tutorial_code/calib3d/real_time_pose_estimation/
/// *   The default method used to estimate the camera pose for the Minimal Sample Sets step
/// is #SOLVEPNP_EPNP. Exceptions are:
/// - if you choose #SOLVEPNP_P3P or #SOLVEPNP_AP3P, these methods will be used.
/// - if the number of input points is equal to 4, #SOLVEPNP_P3P is used.
/// *   The method used to estimate the camera pose using all the inliers is defined by the
/// flags parameters unless it is equal to #SOLVEPNP_P3P or #SOLVEPNP_AP3P. In this case,
/// the method #SOLVEPNP_EPNP will be used instead.
///
/// ## C++ default parameters
/// * use_extrinsic_guess: false
/// * iterations_count: 100
/// * reprojection_error: 8.0
/// * confidence: 0.99
/// * inliers: noArray()
/// * flags: SOLVEPNP_ITERATIVE
pub fn solve_pnp_ransac(object_points: &core::Mat, image_points: &core::Mat, camera_matrix: &core::Mat, dist_coeffs: &core::Mat, rvec: &mut core::Mat, tvec: &mut core::Mat, use_extrinsic_guess: bool, iterations_count: i32, reprojection_error: f32, confidence: f64, inliers: &mut core::Mat, flags: i32) -> Result<bool> {
    unsafe { sys::cv_solvePnPRansac_Mat_Mat_Mat_Mat_Mat_Mat_bool_int_float_double_Mat_int(object_points.as_raw_Mat(), image_points.as_raw_Mat(), camera_matrix.as_raw_Mat(), dist_coeffs.as_raw_Mat(), rvec.as_raw_Mat(), tvec.as_raw_Mat(), use_extrinsic_guess, iterations_count, reprojection_error, confidence, inliers.as_raw_Mat(), flags) }.into_result()
}

/// Finds an object pose from 3D-2D point correspondences.
/// 
/// ## Parameters
/// * objectPoints: Array of object points in the object coordinate space, Nx3 1-channel or
/// 1xN/Nx1 3-channel, where N is the number of points. vector\<Point3f\> can be also passed here.
/// * imagePoints: Array of corresponding image points, Nx2 1-channel or 1xN/Nx1 2-channel,
/// where N is the number of points. vector\<Point2f\> can be also passed here.
/// * cameraMatrix: Input camera matrix <span lang='latex'>A = \vecthreethree{fx}{0}{cx}{0}{fy}{cy}{0}{0}{1}</span> .
/// * distCoeffs: Input vector of distortion coefficients
/// <span lang='latex'>(k_1, k_2, p_1, p_2[, k_3[, k_4, k_5, k_6 [, s_1, s_2, s_3, s_4[, \tau_x, \tau_y]]]])</span> of
/// 4, 5, 8, 12 or 14 elements. If the vector is NULL/empty, the zero distortion coefficients are
/// assumed.
/// * rvec: Output rotation vector (see @ref Rodrigues ) that, together with tvec , brings points from
/// the model coordinate system to the camera coordinate system.
/// * tvec: Output translation vector.
/// * useExtrinsicGuess: Parameter used for #SOLVEPNP_ITERATIVE. If true (1), the function uses
/// the provided rvec and tvec values as initial approximations of the rotation and translation
/// vectors, respectively, and further optimizes them.
/// * flags: Method for solving a PnP problem:
/// *   **SOLVEPNP_ITERATIVE** Iterative method is based on Levenberg-Marquardt optimization. In
/// this case the function finds such a pose that minimizes reprojection error, that is the sum
/// of squared distances between the observed projections imagePoints and the projected (using
/// projectPoints ) objectPoints .
/// *   **SOLVEPNP_P3P** Method is based on the paper of X.S. Gao, X.-R. Hou, J. Tang, H.-F. Chang
/// "Complete Solution Classification for the Perspective-Three-Point Problem" ([gao2003complete](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_gao2003complete)).
/// In this case the function requires exactly four object and image points.
/// *   **SOLVEPNP_AP3P** Method is based on the paper of T. Ke, S. Roumeliotis
/// "An Efficient Algebraic Solution to the Perspective-Three-Point Problem" ([Ke17](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Ke17)).
/// In this case the function requires exactly four object and image points.
/// *   **SOLVEPNP_EPNP** Method has been introduced by F.Moreno-Noguer, V.Lepetit and P.Fua in the
/// paper "EPnP: Efficient Perspective-n-Point Camera Pose Estimation" ([lepetit2009epnp](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_lepetit2009epnp)).
/// *   **SOLVEPNP_DLS** Method is based on the paper of Joel A. Hesch and Stergios I. Roumeliotis.
/// "A Direct Least-Squares (DLS) Method for PnP" ([hesch2011direct](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_hesch2011direct)).
/// *   **SOLVEPNP_UPNP** Method is based on the paper of A.Penate-Sanchez, J.Andrade-Cetto,
/// F.Moreno-Noguer. "Exhaustive Linearization for Robust Camera Pose and Focal Length
/// Estimation" ([penate2013exhaustive](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_penate2013exhaustive)). In this case the function also estimates the parameters <span lang='latex'>f_x</span> and <span lang='latex'>f_y</span>
/// assuming that both have the same value. Then the cameraMatrix is updated with the estimated
/// focal length.
/// *   **SOLVEPNP_AP3P** Method is based on the paper of Tong Ke and Stergios I. Roumeliotis.
/// "An Efficient Algebraic Solution to the Perspective-Three-Point Problem" ([Ke17](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Ke17)). In this case the
/// function requires exactly four object and image points.
/// 
/// The function estimates the object pose given a set of object points, their corresponding image
/// projections, as well as the camera matrix and the distortion coefficients, see the figure below
/// (more precisely, the X-axis of the camera frame is pointing to the right, the Y-axis downward
/// and the Z-axis forward).
/// 
/// ![](https://docs.opencv.org/3.4.6/pnp.jpg)
/// 
/// Points expressed in the world frame <span lang='latex'> \bf{X}_w </span> are projected into the image plane <span lang='latex'> \left[ u, v \right] </span>
/// using the perspective projection model <span lang='latex'> \Pi </span> and the camera intrinsic parameters matrix <span lang='latex'> \bf{A} </span>:
/// 
/// <div lang='latex'>
/// \begin{align*}
/// \begin{bmatrix}
/// u \\
/// v \\
/// 1
/// \end{bmatrix} &=
/// \bf{A} \hspace{0.1em} \Pi \hspace{0.2em} ^{c}\bf{M}_w
/// \begin{bmatrix}
/// X_{w} \\
/// Y_{w} \\
/// Z_{w} \\
/// 1
/// \end{bmatrix} \\
/// \begin{bmatrix}
/// u \\
/// v \\
/// 1
/// \end{bmatrix} &=
/// \begin{bmatrix}
/// f_x & 0 & c_x \\
/// 0 & f_y & c_y \\
/// 0 & 0 & 1
/// \end{bmatrix}
/// \begin{bmatrix}
/// 1 & 0 & 0 & 0 \\
/// 0 & 1 & 0 & 0 \\
/// 0 & 0 & 1 & 0
/// \end{bmatrix}
/// \begin{bmatrix}
/// r_{11} & r_{12} & r_{13} & t_x \\
/// r_{21} & r_{22} & r_{23} & t_y \\
/// r_{31} & r_{32} & r_{33} & t_z \\
/// 0 & 0 & 0 & 1
/// \end{bmatrix}
/// \begin{bmatrix}
/// X_{w} \\
/// Y_{w} \\
/// Z_{w} \\
/// 1
/// \end{bmatrix}
/// \end{align*}
/// </div>
/// 
/// The estimated pose is thus the rotation (`rvec`) and the translation (`tvec`) vectors that allow to transform
/// a 3D point expressed in the world frame into the camera frame:
/// 
/// <div lang='latex'>
/// \begin{align*}
/// \begin{bmatrix}
/// X_c \\
/// Y_c \\
/// Z_c \\
/// 1
/// \end{bmatrix} &=
/// \hspace{0.2em} ^{c}\bf{M}_w
/// \begin{bmatrix}
/// X_{w} \\
/// Y_{w} \\
/// Z_{w} \\
/// 1
/// \end{bmatrix} \\
/// \begin{bmatrix}
/// X_c \\
/// Y_c \\
/// Z_c \\
/// 1
/// \end{bmatrix} &=
/// \begin{bmatrix}
/// r_{11} & r_{12} & r_{13} & t_x \\
/// r_{21} & r_{22} & r_{23} & t_y \\
/// r_{31} & r_{32} & r_{33} & t_z \\
/// 0 & 0 & 0 & 1
/// \end{bmatrix}
/// \begin{bmatrix}
/// X_{w} \\
/// Y_{w} \\
/// Z_{w} \\
/// 1
/// \end{bmatrix}
/// \end{align*}
/// </div>
/// 
/// 
/// Note:
/// *   An example of how to use solvePnP for planar augmented reality can be found at
/// opencv_source_code/samples/python/plane_ar.py
/// *   If you are using Python:
/// - Numpy array slices won't work as input because solvePnP requires contiguous
/// arrays (enforced by the assertion using cv::Mat::checkVector() around line 55 of
/// modules/calib3d/src/solvepnp.cpp version 2.4.9)
/// - The P3P algorithm requires image points to be in an array of shape (N,1,2) due
/// to its calling of cv::undistortPoints (around line 75 of modules/calib3d/src/solvepnp.cpp version 2.4.9)
/// which requires 2-channel information.
/// - Thus, given some data D = np.array(...) where D.shape = (N,M), in order to use a subset of
/// it as, e.g., imagePoints, one must effectively copy it into a new array: imagePoints =
/// np.ascontiguousarray(D[:,:2]).reshape((N,1,2))
/// *   The methods **SOLVEPNP_DLS** and **SOLVEPNP_UPNP** cannot be used as the current implementations are
/// unstable and sometimes give completely wrong results. If you pass one of these two
/// flags, **SOLVEPNP_EPNP** method will be used instead.
/// *   The minimum number of points is 4 in the general case. In the case of **SOLVEPNP_P3P** and **SOLVEPNP_AP3P**
/// methods, it is required to use exactly 4 points (the first 3 points are used to estimate all the solutions
/// of the P3P problem, the last one is used to retain the best solution that minimizes the reprojection error).
/// *   With **SOLVEPNP_ITERATIVE** method and `useExtrinsicGuess=true`, the minimum number of points is 3 (3 points
/// are sufficient to compute a pose but there are up to 4 solutions). The initial solution should be close to the
/// global solution to converge.
///
/// ## C++ default parameters
/// * use_extrinsic_guess: false
/// * flags: SOLVEPNP_ITERATIVE
pub fn solve_pnp(object_points: &core::Mat, image_points: &core::Mat, camera_matrix: &core::Mat, dist_coeffs: &core::Mat, rvec: &mut core::Mat, tvec: &mut core::Mat, use_extrinsic_guess: bool, flags: i32) -> Result<bool> {
    unsafe { sys::cv_solvePnP_Mat_Mat_Mat_Mat_Mat_Mat_bool_int(object_points.as_raw_Mat(), image_points.as_raw_Mat(), camera_matrix.as_raw_Mat(), dist_coeffs.as_raw_Mat(), rvec.as_raw_Mat(), tvec.as_raw_Mat(), use_extrinsic_guess, flags) }.into_result()
}

/// Calibrates the stereo camera.
/// 
/// ## Parameters
/// * objectPoints: Vector of vectors of the calibration pattern points.
/// * imagePoints1: Vector of vectors of the projections of the calibration pattern points,
/// observed by the first camera.
/// * imagePoints2: Vector of vectors of the projections of the calibration pattern points,
/// observed by the second camera.
/// * cameraMatrix1: Input/output first camera matrix:
/// <span lang='latex'>\vecthreethree{f_x^{(j)}}{0}{c_x^{(j)}}{0}{f_y^{(j)}}{c_y^{(j)}}{0}{0}{1}</span> , <span lang='latex'>j = 0,\, 1</span> . If
/// any of CALIB_USE_INTRINSIC_GUESS , CALIB_FIX_ASPECT_RATIO ,
/// CALIB_FIX_INTRINSIC , or CALIB_FIX_FOCAL_LENGTH are specified, some or all of the
/// matrix components must be initialized. See the flags description for details.
/// * distCoeffs1: Input/output vector of distortion coefficients
/// <span lang='latex'>(k_1, k_2, p_1, p_2[, k_3[, k_4, k_5, k_6 [, s_1, s_2, s_3, s_4[, \tau_x, \tau_y]]]])</span> of
/// 4, 5, 8, 12 or 14 elements. The output vector length depends on the flags.
/// * cameraMatrix2: Input/output second camera matrix. The parameter is similar to cameraMatrix1
/// * distCoeffs2: Input/output lens distortion coefficients for the second camera. The parameter
/// is similar to distCoeffs1 .
/// * imageSize: Size of the image used only to initialize intrinsic camera matrix.
/// * R: Output rotation matrix between the 1st and the 2nd camera coordinate systems.
/// * T: Output translation vector between the coordinate systems of the cameras.
/// * E: Output essential matrix.
/// * F: Output fundamental matrix.
/// * perViewErrors: Output vector of the RMS re-projection error estimated for each pattern view.
/// * flags: Different flags that may be zero or a combination of the following values:
/// *   **CALIB_FIX_INTRINSIC** Fix cameraMatrix? and distCoeffs? so that only R, T, E , and F
/// matrices are estimated.
/// *   **CALIB_USE_INTRINSIC_GUESS** Optimize some or all of the intrinsic parameters
/// according to the specified flags. Initial values are provided by the user.
/// *   **CALIB_USE_EXTRINSIC_GUESS** R, T contain valid initial values that are optimized further.
/// Otherwise R, T are initialized to the median value of the pattern views (each dimension separately).
/// *   **CALIB_FIX_PRINCIPAL_POINT** Fix the principal points during the optimization.
/// *   **CALIB_FIX_FOCAL_LENGTH** Fix <span lang='latex'>f^{(j)}_x</span> and <span lang='latex'>f^{(j)}_y</span> .
/// *   **CALIB_FIX_ASPECT_RATIO** Optimize <span lang='latex'>f^{(j)}_y</span> . Fix the ratio <span lang='latex'>f^{(j)}_x/f^{(j)}_y</span>
/// .
/// *   **CALIB_SAME_FOCAL_LENGTH** Enforce <span lang='latex'>f^{(0)}_x=f^{(1)}_x</span> and <span lang='latex'>f^{(0)}_y=f^{(1)}_y</span> .
/// *   **CALIB_ZERO_TANGENT_DIST** Set tangential distortion coefficients for each camera to
/// zeros and fix there.
/// *   **CALIB_FIX_K1,...,CALIB_FIX_K6** Do not change the corresponding radial
/// distortion coefficient during the optimization. If CALIB_USE_INTRINSIC_GUESS is set,
/// the coefficient from the supplied distCoeffs matrix is used. Otherwise, it is set to 0.
/// *   **CALIB_RATIONAL_MODEL** Enable coefficients k4, k5, and k6. To provide the backward
/// compatibility, this extra flag should be explicitly specified to make the calibration
/// function use the rational model and return 8 coefficients. If the flag is not set, the
/// function computes and returns only 5 distortion coefficients.
/// *   **CALIB_THIN_PRISM_MODEL** Coefficients s1, s2, s3 and s4 are enabled. To provide the
/// backward compatibility, this extra flag should be explicitly specified to make the
/// calibration function use the thin prism model and return 12 coefficients. If the flag is not
/// set, the function computes and returns only 5 distortion coefficients.
/// *   **CALIB_FIX_S1_S2_S3_S4** The thin prism distortion coefficients are not changed during
/// the optimization. If CALIB_USE_INTRINSIC_GUESS is set, the coefficient from the
/// supplied distCoeffs matrix is used. Otherwise, it is set to 0.
/// *   **CALIB_TILTED_MODEL** Coefficients tauX and tauY are enabled. To provide the
/// backward compatibility, this extra flag should be explicitly specified to make the
/// calibration function use the tilted sensor model and return 14 coefficients. If the flag is not
/// set, the function computes and returns only 5 distortion coefficients.
/// *   **CALIB_FIX_TAUX_TAUY** The coefficients of the tilted sensor model are not changed during
/// the optimization. If CALIB_USE_INTRINSIC_GUESS is set, the coefficient from the
/// supplied distCoeffs matrix is used. Otherwise, it is set to 0.
/// * criteria: Termination criteria for the iterative optimization algorithm.
/// 
/// The function estimates transformation between two cameras making a stereo pair. If you have a stereo
/// camera where the relative position and orientation of two cameras is fixed, and if you computed
/// poses of an object relative to the first camera and to the second camera, (R1, T1) and (R2, T2),
/// respectively (this can be done with solvePnP ), then those poses definitely relate to each other.
/// This means that, given ( <span lang='latex'>R_1</span>,<span lang='latex'>T_1</span> ), it should be possible to compute ( <span lang='latex'>R_2</span>,<span lang='latex'>T_2</span> ). You only
/// need to know the position and orientation of the second camera relative to the first camera. This is
/// what the described function does. It computes ( <span lang='latex'>R</span>,<span lang='latex'>T</span> ) so that:
/// 
/// <div lang='latex'>R_2=R*R_1</div>
/// <div lang='latex'>T_2=R*T_1 + T,</div>
/// 
/// Optionally, it computes the essential matrix E:
/// 
/// <div lang='latex'>E= \vecthreethree{0}{-T_2}{T_1}{T_2}{0}{-T_0}{-T_1}{T_0}{0} *R</div>
/// 
/// where <span lang='latex'>T_i</span> are components of the translation vector <span lang='latex'>T</span> : <span lang='latex'>T=[T_0, T_1, T_2]^T</span> . And the function
/// can also compute the fundamental matrix F:
/// 
/// <div lang='latex'>F = cameraMatrix2^{-T} E cameraMatrix1^{-1}</div>
/// 
/// Besides the stereo-related information, the function can also perform a full calibration of each of
/// two cameras. However, due to the high dimensionality of the parameter space and noise in the input
/// data, the function can diverge from the correct solution. If the intrinsic parameters can be
/// estimated with high accuracy for each of the cameras individually (for example, using
/// calibrateCamera ), you are recommended to do so and then pass CALIB_FIX_INTRINSIC flag to the
/// function along with the computed intrinsic parameters. Otherwise, if all the parameters are
/// estimated at once, it makes sense to restrict some parameters, for example, pass
/// CALIB_SAME_FOCAL_LENGTH and CALIB_ZERO_TANGENT_DIST flags, which is usually a
/// reasonable assumption.
/// 
/// Similarly to calibrateCamera , the function minimizes the total re-projection error for all the
/// points in all the available views from both cameras. The function returns the final value of the
/// re-projection error.
///
/// ## C++ default parameters
/// * flags: CALIB_FIX_INTRINSIC
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS, 30, 1e-6)
pub fn stereo_calibrate_camera_with_errors(object_points: &types::VectorOfMat, image_points1: &types::VectorOfMat, image_points2: &types::VectorOfMat, camera_matrix1: &mut core::Mat, dist_coeffs1: &mut core::Mat, camera_matrix2: &mut core::Mat, dist_coeffs2: &mut core::Mat, image_size: core::Size, r: &mut core::Mat, t: &mut core::Mat, e: &mut core::Mat, f: &mut core::Mat, per_view_errors: &mut core::Mat, flags: i32, criteria: &core::TermCriteria) -> Result<f64> {
    unsafe { sys::cv_stereoCalibrate_VectorOfMat_VectorOfMat_VectorOfMat_Mat_Mat_Mat_Mat_Size_Mat_Mat_Mat_Mat_Mat_int_TermCriteria(object_points.as_raw_VectorOfMat(), image_points1.as_raw_VectorOfMat(), image_points2.as_raw_VectorOfMat(), camera_matrix1.as_raw_Mat(), dist_coeffs1.as_raw_Mat(), camera_matrix2.as_raw_Mat(), dist_coeffs2.as_raw_Mat(), image_size, r.as_raw_Mat(), t.as_raw_Mat(), e.as_raw_Mat(), f.as_raw_Mat(), per_view_errors.as_raw_Mat(), flags, criteria.as_raw_TermCriteria()) }.into_result()
}

///
/// ## C++ default parameters
/// * flags: CALIB_FIX_INTRINSIC
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS, 30, 1e-6)
pub fn stereo_calibrate_camera(object_points: &types::VectorOfMat, image_points1: &types::VectorOfMat, image_points2: &types::VectorOfMat, camera_matrix1: &mut core::Mat, dist_coeffs1: &mut core::Mat, camera_matrix2: &mut core::Mat, dist_coeffs2: &mut core::Mat, image_size: core::Size, r: &mut core::Mat, t: &mut core::Mat, e: &mut core::Mat, f: &mut core::Mat, flags: i32, criteria: &core::TermCriteria) -> Result<f64> {
    unsafe { sys::cv_stereoCalibrate_VectorOfMat_VectorOfMat_VectorOfMat_Mat_Mat_Mat_Mat_Size_Mat_Mat_Mat_Mat_int_TermCriteria(object_points.as_raw_VectorOfMat(), image_points1.as_raw_VectorOfMat(), image_points2.as_raw_VectorOfMat(), camera_matrix1.as_raw_Mat(), dist_coeffs1.as_raw_Mat(), camera_matrix2.as_raw_Mat(), dist_coeffs2.as_raw_Mat(), image_size, r.as_raw_Mat(), t.as_raw_Mat(), e.as_raw_Mat(), f.as_raw_Mat(), flags, criteria.as_raw_TermCriteria()) }.into_result()
}

/// Computes a rectification transform for an uncalibrated stereo camera.
/// 
/// ## Parameters
/// * points1: Array of feature points in the first image.
/// * points2: The corresponding points in the second image. The same formats as in
/// findFundamentalMat are supported.
/// * F: Input fundamental matrix. It can be computed from the same set of point pairs using
/// findFundamentalMat .
/// * imgSize: Size of the image.
/// * H1: Output rectification homography matrix for the first image.
/// * H2: Output rectification homography matrix for the second image.
/// * threshold: Optional threshold used to filter out the outliers. If the parameter is greater
/// than zero, all the point pairs that do not comply with the epipolar geometry (that is, the points
/// for which <span lang='latex'>|\texttt{points2[i]}^T*\texttt{F}*\texttt{points1[i]}|>\texttt{threshold}</span> ) are
/// rejected prior to computing the homographies. Otherwise, all the points are considered inliers.
/// 
/// The function computes the rectification transformations without knowing intrinsic parameters of the
/// cameras and their relative position in the space, which explains the suffix "uncalibrated". Another
/// related difference from stereoRectify is that the function outputs not the rectification
/// transformations in the object (3D) space, but the planar perspective transformations encoded by the
/// homography matrices H1 and H2 . The function implements the algorithm [Hartley99](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Hartley99) .
/// 
/// 
/// Note:
/// While the algorithm does not need to know the intrinsic parameters of the cameras, it heavily
/// depends on the epipolar geometry. Therefore, if the camera lenses have a significant distortion,
/// it would be better to correct it before computing the fundamental matrix and calling this
/// function. For example, distortion coefficients can be estimated for each head of stereo camera
/// separately by using calibrateCamera . Then, the images can be corrected using undistort , or
/// just the point coordinates can be corrected with undistortPoints .
///
/// ## C++ default parameters
/// * threshold: 5
pub fn stereo_rectify_uncalibrated(points1: &core::Mat, points2: &core::Mat, f: &core::Mat, img_size: core::Size, h1: &mut core::Mat, h2: &mut core::Mat, threshold: f64) -> Result<bool> {
    unsafe { sys::cv_stereoRectifyUncalibrated_Mat_Mat_Mat_Size_Mat_Mat_double(points1.as_raw_Mat(), points2.as_raw_Mat(), f.as_raw_Mat(), img_size, h1.as_raw_Mat(), h2.as_raw_Mat(), threshold) }.into_result()
}

/// Computes rectification transforms for each head of a calibrated stereo camera.
/// 
/// ## Parameters
/// * cameraMatrix1: First camera matrix.
/// * distCoeffs1: First camera distortion parameters.
/// * cameraMatrix2: Second camera matrix.
/// * distCoeffs2: Second camera distortion parameters.
/// * imageSize: Size of the image used for stereo calibration.
/// * R: Rotation matrix between the coordinate systems of the first and the second cameras.
/// * T: Translation vector between coordinate systems of the cameras.
/// * R1: Output 3x3 rectification transform (rotation matrix) for the first camera.
/// * R2: Output 3x3 rectification transform (rotation matrix) for the second camera.
/// * P1: Output 3x4 projection matrix in the new (rectified) coordinate systems for the first
/// camera.
/// * P2: Output 3x4 projection matrix in the new (rectified) coordinate systems for the second
/// camera.
/// * Q: Output <span lang='latex'>4 \times 4</span> disparity-to-depth mapping matrix (see reprojectImageTo3D ).
/// * flags: Operation flags that may be zero or CALIB_ZERO_DISPARITY . If the flag is set,
/// the function makes the principal points of each camera have the same pixel coordinates in the
/// rectified views. And if the flag is not set, the function may still shift the images in the
/// horizontal or vertical direction (depending on the orientation of epipolar lines) to maximize the
/// useful image area.
/// * alpha: Free scaling parameter. If it is -1 or absent, the function performs the default
/// scaling. Otherwise, the parameter should be between 0 and 1. alpha=0 means that the rectified
/// images are zoomed and shifted so that only valid pixels are visible (no black areas after
/// rectification). alpha=1 means that the rectified image is decimated and shifted so that all the
/// pixels from the original images from the cameras are retained in the rectified images (no source
/// image pixels are lost). Obviously, any intermediate value yields an intermediate result between
/// those two extreme cases.
/// * newImageSize: New image resolution after rectification. The same size should be passed to
/// initUndistortRectifyMap (see the stereo_calib.cpp sample in OpenCV samples directory). When (0,0)
/// is passed (default), it is set to the original imageSize . Setting it to larger value can help you
/// preserve details in the original image, especially when there is a big radial distortion.
/// * validPixROI1: Optional output rectangles inside the rectified images where all the pixels
/// are valid. If alpha=0 , the ROIs cover the whole images. Otherwise, they are likely to be smaller
/// (see the picture below).
/// * validPixROI2: Optional output rectangles inside the rectified images where all the pixels
/// are valid. If alpha=0 , the ROIs cover the whole images. Otherwise, they are likely to be smaller
/// (see the picture below).
/// 
/// The function computes the rotation matrices for each camera that (virtually) make both camera image
/// planes the same plane. Consequently, this makes all the epipolar lines parallel and thus simplifies
/// the dense stereo correspondence problem. The function takes the matrices computed by stereoCalibrate
/// as input. As output, it provides two rotation matrices and also two projection matrices in the new
/// coordinates. The function distinguishes the following two cases:
/// 
/// *   **Horizontal stereo**: the first and the second camera views are shifted relative to each other
/// mainly along the x axis (with possible small vertical shift). In the rectified images, the
/// corresponding epipolar lines in the left and right cameras are horizontal and have the same
/// y-coordinate. P1 and P2 look like:
/// 
/// <div lang='latex'>\texttt{P1} = \begin{bmatrix} f & 0 & cx_1 & 0 \\ 0 & f & cy & 0 \\ 0 & 0 & 1 & 0 \end{bmatrix}</div>
/// 
/// <div lang='latex'>\texttt{P2} = \begin{bmatrix} f & 0 & cx_2 & T_x*f \\ 0 & f & cy & 0 \\ 0 & 0 & 1 & 0 \end{bmatrix} ,</div>
/// 
/// where <span lang='latex'>T_x</span> is a horizontal shift between the cameras and <span lang='latex'>cx_1=cx_2</span> if
/// CALIB_ZERO_DISPARITY is set.
/// 
/// *   **Vertical stereo**: the first and the second camera views are shifted relative to each other
/// mainly in vertical direction (and probably a bit in the horizontal direction too). The epipolar
/// lines in the rectified images are vertical and have the same x-coordinate. P1 and P2 look like:
/// 
/// <div lang='latex'>\texttt{P1} = \begin{bmatrix} f & 0 & cx & 0 \\ 0 & f & cy_1 & 0 \\ 0 & 0 & 1 & 0 \end{bmatrix}</div>
/// 
/// <div lang='latex'>\texttt{P2} = \begin{bmatrix} f & 0 & cx & 0 \\ 0 & f & cy_2 & T_y*f \\ 0 & 0 & 1 & 0 \end{bmatrix} ,</div>
/// 
/// where <span lang='latex'>T_y</span> is a vertical shift between the cameras and <span lang='latex'>cy_1=cy_2</span> if CALIB_ZERO_DISPARITY is
/// set.
/// 
/// As you can see, the first three columns of P1 and P2 will effectively be the new "rectified" camera
/// matrices. The matrices, together with R1 and R2 , can then be passed to initUndistortRectifyMap to
/// initialize the rectification map for each camera.
/// 
/// See below the screenshot from the stereo_calib.cpp sample. Some red horizontal lines pass through
/// the corresponding image regions. This means that the images are well rectified, which is what most
/// stereo correspondence algorithms rely on. The green rectangles are roi1 and roi2 . You see that
/// their interiors are all valid pixels.
/// 
/// ![image](https://docs.opencv.org/3.4.6/stereo_undistort.jpg)
///
/// ## C++ default parameters
/// * flags: CALIB_ZERO_DISPARITY
/// * alpha: -1
/// * new_image_size: Size()
/// * valid_pix_roi1: 0
/// * valid_pix_roi2: 0
pub fn stereo_rectify_camera(camera_matrix1: &core::Mat, dist_coeffs1: &core::Mat, camera_matrix2: &core::Mat, dist_coeffs2: &core::Mat, image_size: core::Size, r: &core::Mat, t: &core::Mat, r1: &mut core::Mat, r2: &mut core::Mat, p1: &mut core::Mat, p2: &mut core::Mat, q: &mut core::Mat, flags: i32, alpha: f64, new_image_size: core::Size, valid_pix_roi1: &mut core::Rect, valid_pix_roi2: &mut core::Rect) -> Result<()> {
    unsafe { sys::cv_stereoRectify_Mat_Mat_Mat_Mat_Size_Mat_Mat_Mat_Mat_Mat_Mat_Mat_int_double_Size_Rect_X_Rect_X(camera_matrix1.as_raw_Mat(), dist_coeffs1.as_raw_Mat(), camera_matrix2.as_raw_Mat(), dist_coeffs2.as_raw_Mat(), image_size, r.as_raw_Mat(), t.as_raw_Mat(), r1.as_raw_Mat(), r2.as_raw_Mat(), p1.as_raw_Mat(), p2.as_raw_Mat(), q.as_raw_Mat(), flags, alpha, new_image_size, valid_pix_roi1, valid_pix_roi2) }.into_result()
}

/// Reconstructs points by triangulation.
/// 
/// ## Parameters
/// * projMatr1: 3x4 projection matrix of the first camera.
/// * projMatr2: 3x4 projection matrix of the second camera.
/// * projPoints1: 2xN array of feature points in the first image. In case of c++ version it can
/// be also a vector of feature points or two-channel matrix of size 1xN or Nx1.
/// * projPoints2: 2xN array of corresponding points in the second image. In case of c++ version
/// it can be also a vector of feature points or two-channel matrix of size 1xN or Nx1.
/// * points4D: 4xN array of reconstructed points in homogeneous coordinates.
/// 
/// The function reconstructs 3-dimensional points (in homogeneous coordinates) by using their
/// observations with a stereo camera. Projections matrices can be obtained from stereoRectify.
/// 
/// 
/// Note:
/// Keep in mind that all input data should be of float type in order for this function to work.
/// 
/// ## See also
/// reprojectImageTo3D
pub fn triangulate_points(proj_matr1: &core::Mat, proj_matr2: &core::Mat, proj_points1: &core::Mat, proj_points2: &core::Mat, points4_d: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_triangulatePoints_Mat_Mat_Mat_Mat_Mat(proj_matr1.as_raw_Mat(), proj_matr2.as_raw_Mat(), proj_points1.as_raw_Mat(), proj_points2.as_raw_Mat(), points4_d.as_raw_Mat()) }.into_result()
}

/// validates disparity using the left-right check. The matrix "cost" should be computed by the stereo correspondence algorithm
///
/// ## C++ default parameters
/// * disp12_max_disp: 1
pub fn validate_disparity(disparity: &mut core::Mat, cost: &core::Mat, min_disparity: i32, number_of_disparities: i32, disp12_max_disp: i32) -> Result<()> {
    unsafe { sys::cv_validateDisparity_Mat_Mat_int_int_int(disparity.as_raw_Mat(), cost.as_raw_Mat(), min_disparity, number_of_disparities, disp12_max_disp) }.into_result()
}

impl CirclesGridFinderParameters {

}

impl CirclesGridFinderParameters2 {

    pub fn new() -> Result<crate::calib3d::CirclesGridFinderParameters2> {
        unsafe { sys::cv_CirclesGridFinderParameters2_CirclesGridFinderParameters2() }.into_result()
    }
    
}

// Generating impl for trait cv::StereoBM (trait)
/// Class for computing stereo correspondence using the block matching algorithm, introduced and
/// contributed to OpenCV by K. Konolige.
pub trait StereoBM: crate::calib3d::StereoMatcher {
    #[inline(always)] fn as_raw_StereoBM(&self) -> *mut c_void;
    fn get_pre_filter_type(&self) -> Result<i32> {
        unsafe { sys::cv_StereoBM_getPreFilterType_const(self.as_raw_StereoBM()) }.into_result()
    }
    
    fn set_pre_filter_type(&mut self, pre_filter_type: i32) -> Result<()> {
        unsafe { sys::cv_StereoBM_setPreFilterType_int(self.as_raw_StereoBM(), pre_filter_type) }.into_result()
    }
    
    fn get_pre_filter_size(&self) -> Result<i32> {
        unsafe { sys::cv_StereoBM_getPreFilterSize_const(self.as_raw_StereoBM()) }.into_result()
    }
    
    fn set_pre_filter_size(&mut self, pre_filter_size: i32) -> Result<()> {
        unsafe { sys::cv_StereoBM_setPreFilterSize_int(self.as_raw_StereoBM(), pre_filter_size) }.into_result()
    }
    
    fn get_pre_filter_cap(&self) -> Result<i32> {
        unsafe { sys::cv_StereoBM_getPreFilterCap_const(self.as_raw_StereoBM()) }.into_result()
    }
    
    fn set_pre_filter_cap(&mut self, pre_filter_cap: i32) -> Result<()> {
        unsafe { sys::cv_StereoBM_setPreFilterCap_int(self.as_raw_StereoBM(), pre_filter_cap) }.into_result()
    }
    
    fn get_texture_threshold(&self) -> Result<i32> {
        unsafe { sys::cv_StereoBM_getTextureThreshold_const(self.as_raw_StereoBM()) }.into_result()
    }
    
    fn set_texture_threshold(&mut self, texture_threshold: i32) -> Result<()> {
        unsafe { sys::cv_StereoBM_setTextureThreshold_int(self.as_raw_StereoBM(), texture_threshold) }.into_result()
    }
    
    fn get_uniqueness_ratio(&self) -> Result<i32> {
        unsafe { sys::cv_StereoBM_getUniquenessRatio_const(self.as_raw_StereoBM()) }.into_result()
    }
    
    fn set_uniqueness_ratio(&mut self, uniqueness_ratio: i32) -> Result<()> {
        unsafe { sys::cv_StereoBM_setUniquenessRatio_int(self.as_raw_StereoBM(), uniqueness_ratio) }.into_result()
    }
    
    fn get_smaller_block_size(&self) -> Result<i32> {
        unsafe { sys::cv_StereoBM_getSmallerBlockSize_const(self.as_raw_StereoBM()) }.into_result()
    }
    
    fn set_smaller_block_size(&mut self, block_size: i32) -> Result<()> {
        unsafe { sys::cv_StereoBM_setSmallerBlockSize_int(self.as_raw_StereoBM(), block_size) }.into_result()
    }
    
    fn get_roi1(&self) -> Result<core::Rect> {
        unsafe { sys::cv_StereoBM_getROI1_const(self.as_raw_StereoBM()) }.into_result()
    }
    
    fn set_roi1(&mut self, roi1: core::Rect) -> Result<()> {
        unsafe { sys::cv_StereoBM_setROI1_Rect(self.as_raw_StereoBM(), roi1) }.into_result()
    }
    
    fn get_roi2(&self) -> Result<core::Rect> {
        unsafe { sys::cv_StereoBM_getROI2_const(self.as_raw_StereoBM()) }.into_result()
    }
    
    fn set_roi2(&mut self, roi2: core::Rect) -> Result<()> {
        unsafe { sys::cv_StereoBM_setROI2_Rect(self.as_raw_StereoBM(), roi2) }.into_result()
    }
    
}

impl dyn StereoBM + '_ {

    /// Creates StereoBM object
    /// 
    /// ## Parameters
    /// * numDisparities: the disparity search range. For each pixel algorithm will find the best
    /// disparity from 0 (default minimum disparity) to numDisparities. The search range can then be
    /// shifted by changing the minimum disparity.
    /// * blockSize: the linear size of the blocks compared by the algorithm. The size should be odd
    /// (as the block is centered at the current pixel). Larger block size implies smoother, though less
    /// accurate disparity map. Smaller block size gives more detailed disparity map, but there is higher
    /// chance for algorithm to find a wrong correspondence.
    /// 
    /// The function create StereoBM object. You can then call StereoBM::compute() to compute disparity for
    /// a specific stereo pair.
    ///
    /// ## C++ default parameters
    /// * num_disparities: 0
    /// * block_size: 21
    pub fn create(num_disparities: i32, block_size: i32) -> Result<types::PtrOfStereoBM> {
        unsafe { sys::cv_StereoBM_create_int_int(num_disparities, block_size) }.into_result().map(|ptr| types::PtrOfStereoBM { ptr })
    }
    
}

// Generating impl for trait cv::StereoMatcher (trait)
/// The base class for stereo correspondence algorithms.
pub trait StereoMatcher: core::Algorithm {
    #[inline(always)] fn as_raw_StereoMatcher(&self) -> *mut c_void;
    /// Computes disparity map for the specified stereo pair
    /// 
    /// ## Parameters
    /// * left: Left 8-bit single-channel image.
    /// * right: Right image of the same size and the same type as the left one.
    /// * disparity: Output disparity map. It has the same size as the input images. Some algorithms,
    /// like StereoBM or StereoSGBM compute 16-bit fixed-point disparity map (where each disparity value
    /// has 4 fractional bits), whereas other algorithms output 32-bit floating-point disparity map.
    fn compute(&mut self, left: &core::Mat, right: &core::Mat, disparity: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_StereoMatcher_compute_Mat_Mat_Mat(self.as_raw_StereoMatcher(), left.as_raw_Mat(), right.as_raw_Mat(), disparity.as_raw_Mat()) }.into_result()
    }
    
    fn get_min_disparity(&self) -> Result<i32> {
        unsafe { sys::cv_StereoMatcher_getMinDisparity_const(self.as_raw_StereoMatcher()) }.into_result()
    }
    
    fn set_min_disparity(&mut self, min_disparity: i32) -> Result<()> {
        unsafe { sys::cv_StereoMatcher_setMinDisparity_int(self.as_raw_StereoMatcher(), min_disparity) }.into_result()
    }
    
    fn get_num_disparities(&self) -> Result<i32> {
        unsafe { sys::cv_StereoMatcher_getNumDisparities_const(self.as_raw_StereoMatcher()) }.into_result()
    }
    
    fn set_num_disparities(&mut self, num_disparities: i32) -> Result<()> {
        unsafe { sys::cv_StereoMatcher_setNumDisparities_int(self.as_raw_StereoMatcher(), num_disparities) }.into_result()
    }
    
    fn get_block_size(&self) -> Result<i32> {
        unsafe { sys::cv_StereoMatcher_getBlockSize_const(self.as_raw_StereoMatcher()) }.into_result()
    }
    
    fn set_block_size(&mut self, block_size: i32) -> Result<()> {
        unsafe { sys::cv_StereoMatcher_setBlockSize_int(self.as_raw_StereoMatcher(), block_size) }.into_result()
    }
    
    fn get_speckle_window_size(&self) -> Result<i32> {
        unsafe { sys::cv_StereoMatcher_getSpeckleWindowSize_const(self.as_raw_StereoMatcher()) }.into_result()
    }
    
    fn set_speckle_window_size(&mut self, speckle_window_size: i32) -> Result<()> {
        unsafe { sys::cv_StereoMatcher_setSpeckleWindowSize_int(self.as_raw_StereoMatcher(), speckle_window_size) }.into_result()
    }
    
    fn get_speckle_range(&self) -> Result<i32> {
        unsafe { sys::cv_StereoMatcher_getSpeckleRange_const(self.as_raw_StereoMatcher()) }.into_result()
    }
    
    fn set_speckle_range(&mut self, speckle_range: i32) -> Result<()> {
        unsafe { sys::cv_StereoMatcher_setSpeckleRange_int(self.as_raw_StereoMatcher(), speckle_range) }.into_result()
    }
    
    fn get_disp12_max_diff(&self) -> Result<i32> {
        unsafe { sys::cv_StereoMatcher_getDisp12MaxDiff_const(self.as_raw_StereoMatcher()) }.into_result()
    }
    
    fn set_disp12_max_diff(&mut self, disp12_max_diff: i32) -> Result<()> {
        unsafe { sys::cv_StereoMatcher_setDisp12MaxDiff_int(self.as_raw_StereoMatcher(), disp12_max_diff) }.into_result()
    }
    
}

// Generating impl for trait cv::StereoSGBM (trait)
/// The class implements the modified H. Hirschmuller algorithm [HH08](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_HH08) that differs from the original
/// one as follows:
/// 
/// *   By default, the algorithm is single-pass, which means that you consider only 5 directions
/// instead of 8. Set mode=StereoSGBM::MODE_HH in createStereoSGBM to run the full variant of the
/// algorithm but beware that it may consume a lot of memory.
/// *   The algorithm matches blocks, not individual pixels. Though, setting blockSize=1 reduces the
/// blocks to single pixels.
/// *   Mutual information cost function is not implemented. Instead, a simpler Birchfield-Tomasi
/// sub-pixel metric from [BT98](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_BT98) is used. Though, the color images are supported as well.
/// *   Some pre- and post- processing steps from K. Konolige algorithm StereoBM are included, for
/// example: pre-filtering (StereoBM::PREFILTER_XSOBEL type) and post-filtering (uniqueness
/// check, quadratic interpolation and speckle filtering).
/// 
/// 
/// Note:
/// *   (Python) An example illustrating the use of the StereoSGBM matching algorithm can be found
/// at opencv_source_code/samples/python/stereo_match.py
pub trait StereoSGBM: crate::calib3d::StereoMatcher {
    #[inline(always)] fn as_raw_StereoSGBM(&self) -> *mut c_void;
    fn get_pre_filter_cap(&self) -> Result<i32> {
        unsafe { sys::cv_StereoSGBM_getPreFilterCap_const(self.as_raw_StereoSGBM()) }.into_result()
    }
    
    fn set_pre_filter_cap(&mut self, pre_filter_cap: i32) -> Result<()> {
        unsafe { sys::cv_StereoSGBM_setPreFilterCap_int(self.as_raw_StereoSGBM(), pre_filter_cap) }.into_result()
    }
    
    fn get_uniqueness_ratio(&self) -> Result<i32> {
        unsafe { sys::cv_StereoSGBM_getUniquenessRatio_const(self.as_raw_StereoSGBM()) }.into_result()
    }
    
    fn set_uniqueness_ratio(&mut self, uniqueness_ratio: i32) -> Result<()> {
        unsafe { sys::cv_StereoSGBM_setUniquenessRatio_int(self.as_raw_StereoSGBM(), uniqueness_ratio) }.into_result()
    }
    
    fn get_p1(&self) -> Result<i32> {
        unsafe { sys::cv_StereoSGBM_getP1_const(self.as_raw_StereoSGBM()) }.into_result()
    }
    
    fn set_p1(&mut self, p1: i32) -> Result<()> {
        unsafe { sys::cv_StereoSGBM_setP1_int(self.as_raw_StereoSGBM(), p1) }.into_result()
    }
    
    fn get_p2(&self) -> Result<i32> {
        unsafe { sys::cv_StereoSGBM_getP2_const(self.as_raw_StereoSGBM()) }.into_result()
    }
    
    fn set_p2(&mut self, p2: i32) -> Result<()> {
        unsafe { sys::cv_StereoSGBM_setP2_int(self.as_raw_StereoSGBM(), p2) }.into_result()
    }
    
    fn get_mode(&self) -> Result<i32> {
        unsafe { sys::cv_StereoSGBM_getMode_const(self.as_raw_StereoSGBM()) }.into_result()
    }
    
    fn set_mode(&mut self, mode: i32) -> Result<()> {
        unsafe { sys::cv_StereoSGBM_setMode_int(self.as_raw_StereoSGBM(), mode) }.into_result()
    }
    
}

impl dyn StereoSGBM + '_ {

    /// Creates StereoSGBM object
    /// 
    /// ## Parameters
    /// * minDisparity: Minimum possible disparity value. Normally, it is zero but sometimes
    /// rectification algorithms can shift images, so this parameter needs to be adjusted accordingly.
    /// * numDisparities: Maximum disparity minus minimum disparity. The value is always greater than
    /// zero. In the current implementation, this parameter must be divisible by 16.
    /// * blockSize: Matched block size. It must be an odd number \>=1 . Normally, it should be
    /// somewhere in the 3..11 range.
    /// * P1: The first parameter controlling the disparity smoothness. See below.
    /// * P2: The second parameter controlling the disparity smoothness. The larger the values are,
    /// the smoother the disparity is. P1 is the penalty on the disparity change by plus or minus 1
    /// between neighbor pixels. P2 is the penalty on the disparity change by more than 1 between neighbor
    /// pixels. The algorithm requires P2 \> P1 . See stereo_match.cpp sample where some reasonably good
    /// P1 and P2 values are shown (like 8\*number_of_image_channels\*SADWindowSize\*SADWindowSize and
    /// 32\*number_of_image_channels\*SADWindowSize\*SADWindowSize , respectively).
    /// * disp12MaxDiff: Maximum allowed difference (in integer pixel units) in the left-right
    /// disparity check. Set it to a non-positive value to disable the check.
    /// * preFilterCap: Truncation value for the prefiltered image pixels. The algorithm first
    /// computes x-derivative at each pixel and clips its value by [-preFilterCap, preFilterCap] interval.
    /// The result values are passed to the Birchfield-Tomasi pixel cost function.
    /// * uniquenessRatio: Margin in percentage by which the best (minimum) computed cost function
    /// value should "win" the second best value to consider the found match correct. Normally, a value
    /// within the 5-15 range is good enough.
    /// * speckleWindowSize: Maximum size of smooth disparity regions to consider their noise speckles
    /// and invalidate. Set it to 0 to disable speckle filtering. Otherwise, set it somewhere in the
    /// 50-200 range.
    /// * speckleRange: Maximum disparity variation within each connected component. If you do speckle
    /// filtering, set the parameter to a positive value, it will be implicitly multiplied by 16.
    /// Normally, 1 or 2 is good enough.
    /// * mode: Set it to StereoSGBM::MODE_HH to run the full-scale two-pass dynamic programming
    /// algorithm. It will consume O(W\*H\*numDisparities) bytes, which is large for 640x480 stereo and
    /// huge for HD-size pictures. By default, it is set to false .
    /// 
    /// The first constructor initializes StereoSGBM with all the default parameters. So, you only have to
    /// set StereoSGBM::numDisparities at minimum. The second constructor enables you to set each parameter
    /// to a custom value.
    ///
    /// ## C++ default parameters
    /// * min_disparity: 0
    /// * num_disparities: 16
    /// * block_size: 3
    /// * p1: 0
    /// * p2: 0
    /// * disp12_max_diff: 0
    /// * pre_filter_cap: 0
    /// * uniqueness_ratio: 0
    /// * speckle_window_size: 0
    /// * speckle_range: 0
    /// * mode: StereoSGBM::MODE_SGBM
    pub fn create(min_disparity: i32, num_disparities: i32, block_size: i32, p1: i32, p2: i32, disp12_max_diff: i32, pre_filter_cap: i32, uniqueness_ratio: i32, speckle_window_size: i32, speckle_range: i32, mode: i32) -> Result<types::PtrOfStereoSGBM> {
        unsafe { sys::cv_StereoSGBM_create_int_int_int_int_int_int_int_int_int_int_int(min_disparity, num_disparities, block_size, p1, p2, disp12_max_diff, pre_filter_cap, uniqueness_ratio, speckle_window_size, speckle_range, mode) }.into_result().map(|ptr| types::PtrOfStereoSGBM { ptr })
    }
    
}

pub const StereoMatcher_DISP_SCALE: i32 = 0x10; // 16
