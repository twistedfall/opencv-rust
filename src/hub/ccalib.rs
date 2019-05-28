//! # Custom Calibration Pattern for 3D reconstruction
use std::os::raw::{c_char, c_void};
use libc::size_t;
use crate::{Error, Result, core, sys, types};

pub const CALIB_FIX_CENTER: i32 = 256;
pub const CALIB_FIX_GAMMA: i32 = 128;
pub const CALIB_FIX_K1: i32 = 4;
pub const CALIB_FIX_K2: i32 = 8;
pub const CALIB_FIX_P1: i32 = 16;
pub const CALIB_FIX_P2: i32 = 32;
pub const CALIB_FIX_SKEW: i32 = 2;
pub const CALIB_FIX_XI: i32 = 64;
pub const CALIB_USE_GUESS: i32 = 1;
pub const HEAD: i32 = -1;
pub const INVALID: i32 = -2;
pub const MultiCameraCalibration_OMNIDIRECTIONAL: i32 = 1;
pub const MultiCameraCalibration_PINHOLE: i32 = 0;
pub const RECTIFY_CYLINDRICAL: i32 = 2;
pub const RECTIFY_LONGLATI: i32 = 3;
pub const RECTIFY_PERSPECTIVE: i32 = 1;
pub const RECTIFY_STEREOGRAPHIC: i32 = 4;
pub const XYZ: i32 = 2;
pub const XYZRGB: i32 = 1;

/// Perform omnidirectional camera calibration, the default depth of outputs is CV_64F.
/// 
/// ## Parameters
/// * objectPoints: Vector of vector of Vec3f object points in world (pattern) coordinate.
/// It also can be vector of Mat with size 1xN/Nx1 and type CV_32FC3. Data with depth of 64_F is also acceptable.
/// * imagePoints: Vector of vector of Vec2f corresponding image points of objectPoints. It must be the same
/// size and the same type with objectPoints.
/// * size: Image size of calibration images.
/// * K: Output calibrated camera matrix.
/// * xi: Output parameter xi for CMei's model
/// * D: Output distortion parameters <span lang='latex'>(k_1, k_2, p_1, p_2)</span>
/// * rvecs: Output rotations for each calibration images
/// * tvecs: Output translation for each calibration images
/// * flags: The flags that control calibrate
/// * criteria: Termination criteria for optimization
/// * idx: Indices of images that pass initialization, which are really used in calibration. So the size of rvecs is the
/// same as idx.total().
///
/// ## C++ default parameters
/// * idx: noArray()
pub fn calibrate(object_points: &types::VectorOfMat, image_points: &types::VectorOfMat, size: core::Size, k: &mut core::Mat, xi: &mut core::Mat, d: &mut core::Mat, rvecs: &mut types::VectorOfMat, tvecs: &mut types::VectorOfMat, flags: i32, criteria: &core::TermCriteria, idx: &mut core::Mat) -> Result<f64> {
    unsafe { sys::cv_omnidir_calibrate_VectorOfMat_VectorOfMat_Size_Mat_Mat_Mat_VectorOfMat_VectorOfMat_int_TermCriteria_Mat(object_points.as_raw_VectorOfMat(), image_points.as_raw_VectorOfMat(), size, k.as_raw_Mat(), xi.as_raw_Mat(), d.as_raw_Mat(), rvecs.as_raw_VectorOfMat(), tvecs.as_raw_VectorOfMat(), flags, criteria.as_raw_TermCriteria(), idx.as_raw_Mat()) }.into_result()
}

/// Computes undistortion and rectification maps for omnidirectional camera image transform by a rotation R.
/// It output two maps that are used for cv::remap(). If D is empty then zero distortion is used,
/// if R or P is empty then identity matrices are used.
/// 
/// ## Parameters
/// * K: Camera matrix <span lang='latex'>K = \vecthreethree{f_x}{s}{c_x}{0}{f_y}{c_y}{0}{0}{_1}</span>, with depth CV_32F or CV_64F
/// * D: Input vector of distortion coefficients <span lang='latex'>(k_1, k_2, p_1, p_2)</span>, with depth CV_32F or CV_64F
/// * xi: The parameter xi for CMei's model
/// * R: Rotation transform between the original and object space : 3x3 1-channel, or vector: 3x1/1x3, with depth CV_32F or CV_64F
/// * P: New camera matrix (3x3) or new projection matrix (3x4)
/// * size: Undistorted image size.
/// * mltype: Type of the first output map that can be CV_32FC1 or CV_16SC2 . See convertMaps()
/// for details.
/// * map1: The first output map.
/// * map2: The second output map.
/// * flags: Flags indicates the rectification type,  RECTIFY_PERSPECTIVE, RECTIFY_CYLINDRICAL, RECTIFY_LONGLATI and RECTIFY_STEREOGRAPHIC
/// are supported.
pub fn init_undistort_rectify_map(k: &core::Mat, d: &core::Mat, xi: &core::Mat, r: &core::Mat, p: &core::Mat, size: core::Size, mltype: i32, map1: &mut core::Mat, map2: &mut core::Mat, flags: i32) -> Result<()> {
    unsafe { sys::cv_omnidir_initUndistortRectifyMap_Mat_Mat_Mat_Mat_Mat_Size_int_Mat_Mat_int(k.as_raw_Mat(), d.as_raw_Mat(), xi.as_raw_Mat(), r.as_raw_Mat(), p.as_raw_Mat(), size, mltype, map1.as_raw_Mat(), map2.as_raw_Mat(), flags) }.into_result()
}

pub fn compose_motion(_om1: &core::Mat, _t1: &core::Mat, _om2: &core::Mat, _t2: &core::Mat, om3: &mut core::Mat, t3: &mut core::Mat, dom3dom1: &mut core::Mat, dom3d_t1: &mut core::Mat, dom3dom2: &mut core::Mat, dom3d_t2: &mut core::Mat, d_t3dom1: &mut core::Mat, d_t3d_t1: &mut core::Mat, d_t3dom2: &mut core::Mat, d_t3d_t2: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_omnidir_internal_compose_motion_Mat_Mat_Mat_Mat_Mat_Mat_Mat_Mat_Mat_Mat_Mat_Mat_Mat_Mat(_om1.as_raw_Mat(), _t1.as_raw_Mat(), _om2.as_raw_Mat(), _t2.as_raw_Mat(), om3.as_raw_Mat(), t3.as_raw_Mat(), dom3dom1.as_raw_Mat(), dom3d_t1.as_raw_Mat(), dom3dom2.as_raw_Mat(), dom3d_t2.as_raw_Mat(), d_t3dom1.as_raw_Mat(), d_t3d_t1.as_raw_Mat(), d_t3dom2.as_raw_Mat(), d_t3d_t2.as_raw_Mat()) }.into_result()
}

pub fn compute_jacobian_stereo(object_points: &types::VectorOfMat, image_points1: &types::VectorOfMat, image_points2: &types::VectorOfMat, parameters: &core::Mat, jtj_inv: &mut core::Mat, jte: &mut core::Mat, flags: i32, epsilon: f64) -> Result<()> {
    unsafe { sys::cv_omnidir_internal_computeJacobianStereo_VectorOfMat_VectorOfMat_VectorOfMat_Mat_Mat_Mat_int_double(object_points.as_raw_VectorOfMat(), image_points1.as_raw_VectorOfMat(), image_points2.as_raw_VectorOfMat(), parameters.as_raw_Mat(), jtj_inv.as_raw_Mat(), jte.as_raw_Mat(), flags, epsilon) }.into_result()
}

pub fn compute_jacobian(object_points: &types::VectorOfMat, image_points: &types::VectorOfMat, parameters: &core::Mat, jtj_inv: &mut core::Mat, jte: &mut core::Mat, flags: i32, epsilon: f64) -> Result<()> {
    unsafe { sys::cv_omnidir_internal_computeJacobian_VectorOfMat_VectorOfMat_Mat_Mat_Mat_int_double(object_points.as_raw_VectorOfMat(), image_points.as_raw_VectorOfMat(), parameters.as_raw_Mat(), jtj_inv.as_raw_Mat(), jte.as_raw_Mat(), flags, epsilon) }.into_result()
}

pub fn compute_mean_repro_err_stereo(object_points: &types::VectorOfMat, image_points1: &types::VectorOfMat, image_points2: &types::VectorOfMat, k1: &core::Mat, k2: &core::Mat, d1: &core::Mat, d2: &core::Mat, xi1: f64, xi2: f64, om: &core::Mat, t: &core::Mat, om_l: &types::VectorOfMat, tl: &types::VectorOfMat) -> Result<f64> {
    unsafe { sys::cv_omnidir_internal_computeMeanReproErrStereo_VectorOfMat_VectorOfMat_VectorOfMat_Mat_Mat_Mat_Mat_double_double_Mat_Mat_VectorOfMat_VectorOfMat(object_points.as_raw_VectorOfMat(), image_points1.as_raw_VectorOfMat(), image_points2.as_raw_VectorOfMat(), k1.as_raw_Mat(), k2.as_raw_Mat(), d1.as_raw_Mat(), d2.as_raw_Mat(), xi1, xi2, om.as_raw_Mat(), t.as_raw_Mat(), om_l.as_raw_VectorOfMat(), tl.as_raw_VectorOfMat()) }.into_result()
}

pub fn compute_mean_repro_err(image_points: &types::VectorOfMat, pro_image_points: &types::VectorOfMat) -> Result<f64> {
    unsafe { sys::cv_omnidir_internal_computeMeanReproErr_VectorOfMat_VectorOfMat(image_points.as_raw_VectorOfMat(), pro_image_points.as_raw_VectorOfMat()) }.into_result()
}

pub fn compute_mean_repro_err_1(object_points: &types::VectorOfMat, image_points: &types::VectorOfMat, k: &core::Mat, d: &core::Mat, xi: f64, om_all: &types::VectorOfMat, t_all: &types::VectorOfMat) -> Result<f64> {
    unsafe { sys::cv_omnidir_internal_computeMeanReproErr_VectorOfMat_VectorOfMat_Mat_Mat_double_VectorOfMat_VectorOfMat(object_points.as_raw_VectorOfMat(), image_points.as_raw_VectorOfMat(), k.as_raw_Mat(), d.as_raw_Mat(), xi, om_all.as_raw_VectorOfMat(), t_all.as_raw_VectorOfMat()) }.into_result()
}

pub fn decode_parameters_stereo(parameters: &core::Mat, k1: &mut core::Mat, k2: &mut core::Mat, om: &mut core::Mat, t: &mut core::Mat, om_l: &mut types::VectorOfMat, t_l: &mut types::VectorOfMat, d1: &mut core::Mat, d2: &mut core::Mat, xi1: &mut f64, xi2: &mut f64) -> Result<()> {
    unsafe { sys::cv_omnidir_internal_decodeParametersStereo_Mat_Mat_Mat_Mat_Mat_VectorOfMat_VectorOfMat_Mat_Mat_double_double(parameters.as_raw_Mat(), k1.as_raw_Mat(), k2.as_raw_Mat(), om.as_raw_Mat(), t.as_raw_Mat(), om_l.as_raw_VectorOfMat(), t_l.as_raw_VectorOfMat(), d1.as_raw_Mat(), d2.as_raw_Mat(), xi1, xi2) }.into_result()
}

pub fn decode_parameters(paramsters: &core::Mat, k: &mut core::Mat, om_all: &mut types::VectorOfMat, t_all: &mut types::VectorOfMat, distoration: &mut core::Mat, xi: &mut f64) -> Result<()> {
    unsafe { sys::cv_omnidir_internal_decodeParameters_Mat_Mat_VectorOfMat_VectorOfMat_Mat_double(paramsters.as_raw_Mat(), k.as_raw_Mat(), om_all.as_raw_VectorOfMat(), t_all.as_raw_VectorOfMat(), distoration.as_raw_Mat(), xi) }.into_result()
}

pub fn encode_parameters_stereo(k1: &core::Mat, k2: &core::Mat, om: &core::Mat, t: &core::Mat, om_l: &types::VectorOfMat, t_l: &types::VectorOfMat, d1: &core::Mat, d2: &core::Mat, xi1: f64, xi2: f64, parameters: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_omnidir_internal_encodeParametersStereo_Mat_Mat_Mat_Mat_VectorOfMat_VectorOfMat_Mat_Mat_double_double_Mat(k1.as_raw_Mat(), k2.as_raw_Mat(), om.as_raw_Mat(), t.as_raw_Mat(), om_l.as_raw_VectorOfMat(), t_l.as_raw_VectorOfMat(), d1.as_raw_Mat(), d2.as_raw_Mat(), xi1, xi2, parameters.as_raw_Mat()) }.into_result()
}

pub fn encode_parameters(k: &core::Mat, om_all: &types::VectorOfMat, t_all: &types::VectorOfMat, distoaration: &core::Mat, xi: f64, parameters: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_omnidir_internal_encodeParameters_Mat_VectorOfMat_VectorOfMat_Mat_double_Mat(k.as_raw_Mat(), om_all.as_raw_VectorOfMat(), t_all.as_raw_VectorOfMat(), distoaration.as_raw_Mat(), xi, parameters.as_raw_Mat()) }.into_result()
}

pub fn estimate_uncertainties_stereo(object_points: &types::VectorOfMat, image_points1: &types::VectorOfMat, image_points2: &types::VectorOfMat, parameters: &core::Mat, errors: &mut core::Mat, std_error: &mut core::Vec2d, rms: &mut f64, flags: i32) -> Result<()> {
    unsafe { sys::cv_omnidir_internal_estimateUncertaintiesStereo_VectorOfMat_VectorOfMat_VectorOfMat_Mat_Mat_Vec2d_double_int(object_points.as_raw_VectorOfMat(), image_points1.as_raw_VectorOfMat(), image_points2.as_raw_VectorOfMat(), parameters.as_raw_Mat(), errors.as_raw_Mat(), std_error, rms, flags) }.into_result()
}

pub fn estimate_uncertainties(object_points: &types::VectorOfMat, image_points: &types::VectorOfMat, parameters: &core::Mat, errors: &mut core::Mat, std_error: &mut core::Vec2d, rms: &mut f64, flags: i32) -> Result<()> {
    unsafe { sys::cv_omnidir_internal_estimateUncertainties_VectorOfMat_VectorOfMat_Mat_Mat_Vec2d_double_int(object_points.as_raw_VectorOfMat(), image_points.as_raw_VectorOfMat(), parameters.as_raw_Mat(), errors.as_raw_Mat(), std_error, rms, flags) }.into_result()
}

pub fn fill_fixed_stereo(g: &mut core::Mat, flags: i32, n: i32) -> Result<()> {
    unsafe { sys::cv_omnidir_internal_fillFixedStereo_Mat_int_int(g.as_raw_Mat(), flags, n) }.into_result()
}

pub fn fill_fixed(g: &mut core::Mat, flags: i32, n: i32) -> Result<()> {
    unsafe { sys::cv_omnidir_internal_fillFixed_Mat_int_int(g.as_raw_Mat(), flags, n) }.into_result()
}

pub fn find_median3(mat: &core::Mat) -> Result<core::Vec3d> {
    unsafe { sys::cv_omnidir_internal_findMedian3_Mat(mat.as_raw_Mat()) }.into_result()
}

pub fn find_median(row: &core::Mat) -> Result<f64> {
    unsafe { sys::cv_omnidir_internal_findMedian_Mat(row.as_raw_Mat()) }.into_result()
}

pub fn flags2idx_stereo(flags: i32, idx: &mut types::VectorOfint, n: i32) -> Result<()> {
    unsafe { sys::cv_omnidir_internal_flags2idxStereo_int_VectorOfint_int(flags, idx.as_raw_VectorOfint(), n) }.into_result()
}

pub fn flags2idx(flags: i32, idx: &mut types::VectorOfint, n: i32) -> Result<()> {
    unsafe { sys::cv_omnidir_internal_flags2idx_int_VectorOfint_int(flags, idx.as_raw_VectorOfint(), n) }.into_result()
}

pub fn get_interset(idx1: &core::Mat, idx2: &core::Mat, inter1: &mut core::Mat, inter2: &mut core::Mat, inter_ori: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_omnidir_internal_getInterset_Mat_Mat_Mat_Mat_Mat(idx1.as_raw_Mat(), idx2.as_raw_Mat(), inter1.as_raw_Mat(), inter2.as_raw_Mat(), inter_ori.as_raw_Mat()) }.into_result()
}

///
/// ## C++ default parameters
/// * idx: noArray()
pub fn initialize_calibration(object_points: &types::VectorOfMat, image_points: &types::VectorOfMat, size: core::Size, om_all: &mut types::VectorOfMat, t_all: &mut types::VectorOfMat, k: &mut core::Mat, xi: &mut f64, idx: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_omnidir_internal_initializeCalibration_VectorOfMat_VectorOfMat_Size_VectorOfMat_VectorOfMat_Mat_double_Mat(object_points.as_raw_VectorOfMat(), image_points.as_raw_VectorOfMat(), size, om_all.as_raw_VectorOfMat(), t_all.as_raw_VectorOfMat(), k.as_raw_Mat(), xi, idx.as_raw_Mat()) }.into_result()
}

pub fn initialize_stereo_calibration(object_points: &types::VectorOfMat, image_points1: &types::VectorOfMat, image_points2: &types::VectorOfMat, size1: core::Size, size2: core::Size, om: &mut core::Mat, t: &mut core::Mat, om_l: &mut types::VectorOfMat, t_l: &mut types::VectorOfMat, k1: &mut core::Mat, d1: &mut core::Mat, k2: &mut core::Mat, d2: &mut core::Mat, xi1: &mut f64, xi2: &mut f64, flags: i32, idx: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_omnidir_internal_initializeStereoCalibration_VectorOfMat_VectorOfMat_VectorOfMat_Size_Size_Mat_Mat_VectorOfMat_VectorOfMat_Mat_Mat_Mat_Mat_double_double_int_Mat(object_points.as_raw_VectorOfMat(), image_points1.as_raw_VectorOfMat(), image_points2.as_raw_VectorOfMat(), size1, size2, om.as_raw_Mat(), t.as_raw_Mat(), om_l.as_raw_VectorOfMat(), t_l.as_raw_VectorOfMat(), k1.as_raw_Mat(), d1.as_raw_Mat(), k2.as_raw_Mat(), d2.as_raw_Mat(), xi1, xi2, flags, idx.as_raw_Mat()) }.into_result()
}

pub fn sub_matrix(src: &core::Mat, dst: &mut core::Mat, cols: &types::VectorOfint, rows: &types::VectorOfint) -> Result<()> {
    unsafe { sys::cv_omnidir_internal_subMatrix_Mat_Mat_VectorOfint_VectorOfint(src.as_raw_Mat(), dst.as_raw_Mat(), cols.as_raw_VectorOfint(), rows.as_raw_VectorOfint()) }.into_result()
}

/// Projects points for omnidirectional camera using CMei's model
/// 
/// ## Parameters
/// * objectPoints: Object points in world coordinate, vector of vector of Vec3f or Mat of
/// 1xN/Nx1 3-channel of type CV_32F and N is the number of points. 64F is also acceptable.
/// * imagePoints: Output array of image points, vector of vector of Vec2f or
/// 1xN/Nx1 2-channel of type CV_32F. 64F is also acceptable.
/// * rvec: vector of rotation between world coordinate and camera coordinate, i.e., om
/// * tvec: vector of translation between pattern coordinate and camera coordinate
/// * K: Camera matrix <span lang='latex'>K = \vecthreethree{f_x}{s}{c_x}{0}{f_y}{c_y}{0}{0}{_1}</span>.
/// * D: Input vector of distortion coefficients <span lang='latex'>(k_1, k_2, p_1, p_2)</span>.
/// * xi: The parameter xi for CMei's model
/// * jacobian: Optional output 2Nx16 of type CV_64F jacobian matrix, contains the derivatives of
/// image pixel points wrt parameters including <span lang='latex'>om, T, f_x, f_y, s, c_x, c_y, xi, k_1, k_2, p_1, p_2</span>.
/// This matrix will be used in calibration by optimization.
/// 
/// The function projects object 3D points of world coordinate to image pixels, parameter by intrinsic
/// and extrinsic parameters. Also, it optionally compute a by-product: the jacobian matrix containing
/// contains the derivatives of image pixel points wrt intrinsic and extrinsic parameters.
///
/// ## C++ default parameters
/// * jacobian: noArray()
pub fn project_points(object_points: &core::Mat, image_points: &mut core::Mat, rvec: &core::Mat, tvec: &core::Mat, k: &core::Mat, xi: f64, d: &core::Mat, jacobian: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_omnidir_projectPoints_Mat_Mat_Mat_Mat_Mat_double_Mat_Mat(object_points.as_raw_Mat(), image_points.as_raw_Mat(), rvec.as_raw_Mat(), tvec.as_raw_Mat(), k.as_raw_Mat(), xi, d.as_raw_Mat(), jacobian.as_raw_Mat()) }.into_result()
}

/// Stereo calibration for omnidirectional camera model. It computes the intrinsic parameters for two
/// cameras and the extrinsic parameters between two cameras. The default depth of outputs is CV_64F.
/// 
/// ## Parameters
/// * objectPoints: Object points in world (pattern) coordinate. Its type is vector<vector<Vec3f> >.
/// It also can be vector of Mat with size 1xN/Nx1 and type CV_32FC3. Data with depth of 64_F is also acceptable.
/// * imagePoints1: The corresponding image points of the first camera, with type vector<vector<Vec2f> >.
/// It must be the same size and the same type as objectPoints.
/// * imagePoints2: The corresponding image points of the second camera, with type vector<vector<Vec2f> >.
/// It must be the same size and the same type as objectPoints.
/// * imageSize1: Image size of calibration images of the first camera.
/// * imageSize2: Image size of calibration images of the second camera.
/// * K1: Output camera matrix for the first camera.
/// * xi1: Output parameter xi of Mei's model for the first camera
/// * D1: Output distortion parameters <span lang='latex'>(k_1, k_2, p_1, p_2)</span> for the first camera
/// * K2: Output camera matrix for the first camera.
/// * xi2: Output parameter xi of CMei's model for the second camera
/// * D2: Output distortion parameters <span lang='latex'>(k_1, k_2, p_1, p_2)</span> for the second camera
/// * rvec: Output rotation between the first and second camera
/// * tvec: Output translation between the first and second camera
/// * rvecsL: Output rotation for each image of the first camera
/// * tvecsL: Output translation for each image of the first camera
/// * flags: The flags that control stereoCalibrate
/// * criteria: Termination criteria for optimization
/// * idx: Indices of image pairs that pass initialization, which are really used in calibration. So the size of rvecs is the
/// same as idx.total().
/// @
///
/// ## C++ default parameters
/// * idx: noArray()
pub fn stereo_calibrate(object_points: &mut types::VectorOfMat, image_points1: &mut types::VectorOfMat, image_points2: &mut types::VectorOfMat, image_size1: core::Size, image_size2: core::Size, k1: &mut core::Mat, xi1: &mut core::Mat, d1: &mut core::Mat, k2: &mut core::Mat, xi2: &mut core::Mat, d2: &mut core::Mat, rvec: &mut core::Mat, tvec: &mut core::Mat, rvecs_l: &mut types::VectorOfMat, tvecs_l: &mut types::VectorOfMat, flags: i32, criteria: &core::TermCriteria, idx: &mut core::Mat) -> Result<f64> {
    unsafe { sys::cv_omnidir_stereoCalibrate_VectorOfMat_VectorOfMat_VectorOfMat_Size_Size_Mat_Mat_Mat_Mat_Mat_Mat_Mat_Mat_VectorOfMat_VectorOfMat_int_TermCriteria_Mat(object_points.as_raw_VectorOfMat(), image_points1.as_raw_VectorOfMat(), image_points2.as_raw_VectorOfMat(), image_size1, image_size2, k1.as_raw_Mat(), xi1.as_raw_Mat(), d1.as_raw_Mat(), k2.as_raw_Mat(), xi2.as_raw_Mat(), d2.as_raw_Mat(), rvec.as_raw_Mat(), tvec.as_raw_Mat(), rvecs_l.as_raw_VectorOfMat(), tvecs_l.as_raw_VectorOfMat(), flags, criteria.as_raw_TermCriteria(), idx.as_raw_Mat()) }.into_result()
}

/// Stereo 3D reconstruction from a pair of images
/// 
/// ## Parameters
/// * image1: The first input image
/// * image2: The second input image
/// * K1: Input camera matrix of the first camera
/// * D1: Input distortion parameters <span lang='latex'>(k_1, k_2, p_1, p_2)</span> for the first camera
/// * xi1: Input parameter xi for the first camera for CMei's model
/// * K2: Input camera matrix of the second camera
/// * D2: Input distortion parameters <span lang='latex'>(k_1, k_2, p_1, p_2)</span> for the second camera
/// * xi2: Input parameter xi for the second camera for CMei's model
/// * R: Rotation between the first and second camera
/// * T: Translation between the first and second camera
/// * flag: Flag of rectification type, RECTIFY_PERSPECTIVE or RECTIFY_LONGLATI
/// * numDisparities: The parameter 'numDisparities' in StereoSGBM, see StereoSGBM for details.
/// * SADWindowSize: The parameter 'SADWindowSize' in StereoSGBM, see StereoSGBM for details.
/// * disparity: Disparity map generated by stereo matching
/// * image1Rec: Rectified image of the first image
/// * image2Rec: rectified image of the second image
/// * newSize: Image size of rectified image, see omnidir::undistortImage
/// * Knew: New camera matrix of rectified image, see omnidir::undistortImage
/// * pointCloud: Point cloud of 3D reconstruction, with type CV_64FC3
/// * pointType: Point cloud type, it can be XYZRGB or XYZ
///
/// ## C++ default parameters
/// * new_size: Size()
/// * knew: cv::noArray()
/// * point_cloud: cv::noArray()
/// * point_type: XYZRGB
pub fn stereo_reconstruct(image1: &core::Mat, image2: &core::Mat, k1: &core::Mat, d1: &core::Mat, xi1: &core::Mat, k2: &core::Mat, d2: &core::Mat, xi2: &core::Mat, r: &core::Mat, t: &core::Mat, flag: i32, num_disparities: i32, sad_window_size: i32, disparity: &mut core::Mat, image1_rec: &mut core::Mat, image2_rec: &mut core::Mat, new_size: core::Size, knew: &core::Mat, point_cloud: &mut core::Mat, point_type: i32) -> Result<()> {
    unsafe { sys::cv_omnidir_stereoReconstruct_Mat_Mat_Mat_Mat_Mat_Mat_Mat_Mat_Mat_Mat_int_int_int_Mat_Mat_Mat_Size_Mat_Mat_int(image1.as_raw_Mat(), image2.as_raw_Mat(), k1.as_raw_Mat(), d1.as_raw_Mat(), xi1.as_raw_Mat(), k2.as_raw_Mat(), d2.as_raw_Mat(), xi2.as_raw_Mat(), r.as_raw_Mat(), t.as_raw_Mat(), flag, num_disparities, sad_window_size, disparity.as_raw_Mat(), image1_rec.as_raw_Mat(), image2_rec.as_raw_Mat(), new_size, knew.as_raw_Mat(), point_cloud.as_raw_Mat(), point_type) }.into_result()
}

/// Stereo rectification for omnidirectional camera model. It computes the rectification rotations for two cameras
/// 
/// ## Parameters
/// * R: Rotation between the first and second camera
/// * T: Translation between the first and second camera
/// * R1: Output 3x3 rotation matrix for the first camera
/// * R2: Output 3x3 rotation matrix for the second camera
pub fn stereo_rectify(r: &core::Mat, t: &core::Mat, r1: &mut core::Mat, r2: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_omnidir_stereoRectify_Mat_Mat_Mat_Mat(r.as_raw_Mat(), t.as_raw_Mat(), r1.as_raw_Mat(), r2.as_raw_Mat()) }.into_result()
}

/// Undistort omnidirectional images to perspective images
/// 
/// ## Parameters
/// * distorted: The input omnidirectional image.
/// * undistorted: The output undistorted image.
/// * K: Camera matrix <span lang='latex'>K = \vecthreethree{f_x}{s}{c_x}{0}{f_y}{c_y}{0}{0}{_1}</span>.
/// * D: Input vector of distortion coefficients <span lang='latex'>(k_1, k_2, p_1, p_2)</span>.
/// * xi: The parameter xi for CMei's model.
/// * flags: Flags indicates the rectification type,  RECTIFY_PERSPECTIVE, RECTIFY_CYLINDRICAL, RECTIFY_LONGLATI and RECTIFY_STEREOGRAPHIC
/// * Knew: Camera matrix of the distorted image. If it is not assigned, it is just K.
/// * new_size: The new image size. By default, it is the size of distorted.
/// * R: Rotation matrix between the input and output images. By default, it is identity matrix.
///
/// ## C++ default parameters
/// * knew: cv::noArray()
/// * new_size: Size()
/// * r: Mat::eye(3, 3, CV_64F)
pub fn undistort_image(distorted: &core::Mat, undistorted: &mut core::Mat, k: &core::Mat, d: &core::Mat, xi: &core::Mat, flags: i32, knew: &core::Mat, new_size: core::Size, r: &core::Mat) -> Result<()> {
    unsafe { sys::cv_omnidir_undistortImage_Mat_Mat_Mat_Mat_Mat_int_Mat_Size_Mat(distorted.as_raw_Mat(), undistorted.as_raw_Mat(), k.as_raw_Mat(), d.as_raw_Mat(), xi.as_raw_Mat(), flags, knew.as_raw_Mat(), new_size, r.as_raw_Mat()) }.into_result()
}

/// Undistort 2D image points for omnidirectional camera using CMei's model
/// 
/// ## Parameters
/// * distorted: Array of distorted image points, vector of Vec2f
/// or 1xN/Nx1 2-channel Mat of type CV_32F, 64F depth is also acceptable
/// * K: Camera matrix <span lang='latex'>K = \vecthreethree{f_x}{s}{c_x}{0}{f_y}{c_y}{0}{0}{_1}</span>.
/// * D: Distortion coefficients <span lang='latex'>(k_1, k_2, p_1, p_2)</span>.
/// * xi: The parameter xi for CMei's model
/// * R: Rotation trainsform between the original and object space : 3x3 1-channel, or vector: 3x1/1x3
/// 1-channel or 1x1 3-channel
/// * undistorted: array of normalized object points, vector of Vec2f/Vec2d or 1xN/Nx1 2-channel Mat with the same
/// depth of distorted points.
pub fn undistort_points(distorted: &core::Mat, undistorted: &mut core::Mat, k: &core::Mat, d: &core::Mat, xi: &core::Mat, r: &core::Mat) -> Result<()> {
    unsafe { sys::cv_omnidir_undistortPoints_Mat_Mat_Mat_Mat_Mat_Mat(distorted.as_raw_Mat(), undistorted.as_raw_Mat(), k.as_raw_Mat(), d.as_raw_Mat(), xi.as_raw_Mat(), r.as_raw_Mat()) }.into_result()
}

// boxed class cv::ccalib::CustomPattern
#[allow(dead_code)]
pub struct CustomPattern {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::ccalib::CustomPattern {
    fn drop(&mut self) {
        unsafe { sys::cv_CustomPattern_delete(self.ptr) };
    }
}
impl crate::ccalib::CustomPattern {
    pub fn as_raw_CustomPattern(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        CustomPattern {
            ptr
        }
    }
}

impl core::Algorithm for CustomPattern {
    fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl CustomPattern {

    pub fn new() -> Result<crate::ccalib::CustomPattern> {
        unsafe { sys::cv_ccalib_CustomPattern_CustomPattern() }.into_result().map(|x| crate::ccalib::CustomPattern { ptr: x })
    }
    
    ///
    /// ## C++ default parameters
    /// * output: noArray()
    pub fn create(&mut self, pattern: &core::Mat, board_size: core::Size2f, output: &mut core::Mat) -> Result<bool> {
        unsafe { sys::cv_ccalib_CustomPattern_create_Mat_Size2f_Mat(self.as_raw_CustomPattern(), pattern.as_raw_Mat(), board_size, output.as_raw_Mat()) }.into_result()
    }
    
    ///
    /// ## C++ default parameters
    /// * ratio: 0.7
    /// * proj_error: 8.0
    /// * refine_position: false
    /// * out: noArray()
    /// * h: noArray()
    /// * pattern_corners: noArray()
    pub fn find_pattern(&mut self, image: &core::Mat, matched_features: &mut core::Mat, pattern_points: &mut core::Mat, ratio: f64, proj_error: f64, refine_position: bool, out: &mut core::Mat, h: &mut core::Mat, pattern_corners: &mut core::Mat) -> Result<bool> {
        unsafe { sys::cv_ccalib_CustomPattern_findPattern_Mat_Mat_Mat_double_double_bool_Mat_Mat_Mat(self.as_raw_CustomPattern(), image.as_raw_Mat(), matched_features.as_raw_Mat(), pattern_points.as_raw_Mat(), ratio, proj_error, refine_position, out.as_raw_Mat(), h.as_raw_Mat(), pattern_corners.as_raw_Mat()) }.into_result()
    }
    
    pub fn is_initialized(&mut self) -> Result<bool> {
        unsafe { sys::cv_ccalib_CustomPattern_isInitialized(self.as_raw_CustomPattern()) }.into_result()
    }
    
    pub fn get_pattern_points(&mut self, original_points: &mut types::VectorOfKeyPoint) -> Result<()> {
        unsafe { sys::cv_ccalib_CustomPattern_getPatternPoints_VectorOfKeyPoint(self.as_raw_CustomPattern(), original_points.as_raw_VectorOfKeyPoint()) }.into_result()
    }
    
    /// <
    /// Returns a vector<Point> of the original points.
    pub fn get_pixel_size(&mut self) -> Result<f64> {
        unsafe { sys::cv_ccalib_CustomPattern_getPixelSize(self.as_raw_CustomPattern()) }.into_result()
    }
    
    ///
    /// ## C++ default parameters
    /// * flags: 0
    /// * criteria: TermCriteria(TermCriteria::COUNT + TermCriteria::EPS, 30, DBL_EPSILON)
    pub fn calibrate(&mut self, object_points: &types::VectorOfMat, image_points: &types::VectorOfMat, image_size: core::Size, camera_matrix: &mut core::Mat, dist_coeffs: &mut core::Mat, rvecs: &mut types::VectorOfMat, tvecs: &mut types::VectorOfMat, flags: i32, criteria: &core::TermCriteria) -> Result<f64> {
        unsafe { sys::cv_ccalib_CustomPattern_calibrate_VectorOfMat_VectorOfMat_Size_Mat_Mat_VectorOfMat_VectorOfMat_int_TermCriteria(self.as_raw_CustomPattern(), object_points.as_raw_VectorOfMat(), image_points.as_raw_VectorOfMat(), image_size, camera_matrix.as_raw_Mat(), dist_coeffs.as_raw_Mat(), rvecs.as_raw_VectorOfMat(), tvecs.as_raw_VectorOfMat(), flags, criteria.as_raw_TermCriteria()) }.into_result()
    }
    
    /// <
    /// Calls the calirateCamera function with the same inputs.
    ///
    /// ## C++ default parameters
    /// * use_extrinsic_guess: false
    /// * flags: SOLVEPNP_ITERATIVE
    pub fn find_rt(&mut self, object_points: &core::Mat, image_points: &core::Mat, camera_matrix: &core::Mat, dist_coeffs: &core::Mat, rvec: &mut core::Mat, tvec: &mut core::Mat, use_extrinsic_guess: bool, flags: i32) -> Result<bool> {
        unsafe { sys::cv_ccalib_CustomPattern_findRt_Mat_Mat_Mat_Mat_Mat_Mat_bool_int(self.as_raw_CustomPattern(), object_points.as_raw_Mat(), image_points.as_raw_Mat(), camera_matrix.as_raw_Mat(), dist_coeffs.as_raw_Mat(), rvec.as_raw_Mat(), tvec.as_raw_Mat(), use_extrinsic_guess, flags) }.into_result()
    }
    
    ///
    /// ## C++ default parameters
    /// * use_extrinsic_guess: false
    /// * flags: SOLVEPNP_ITERATIVE
    pub fn find_rt_1(&mut self, image: &core::Mat, camera_matrix: &core::Mat, dist_coeffs: &core::Mat, rvec: &mut core::Mat, tvec: &mut core::Mat, use_extrinsic_guess: bool, flags: i32) -> Result<bool> {
        unsafe { sys::cv_ccalib_CustomPattern_findRt_Mat_Mat_Mat_Mat_Mat_bool_int(self.as_raw_CustomPattern(), image.as_raw_Mat(), camera_matrix.as_raw_Mat(), dist_coeffs.as_raw_Mat(), rvec.as_raw_Mat(), tvec.as_raw_Mat(), use_extrinsic_guess, flags) }.into_result()
    }
    
    /// <
    /// Uses solvePnP to find the rotation and translation of the pattern
    /// with respect to the camera frame.
    ///
    /// ## C++ default parameters
    /// * use_extrinsic_guess: false
    /// * iterations_count: 100
    /// * reprojection_error: 8.0
    /// * min_inliers_count: 100
    /// * inliers: noArray()
    /// * flags: SOLVEPNP_ITERATIVE
    pub fn find_rt_ransac(&mut self, object_points: &core::Mat, image_points: &core::Mat, camera_matrix: &core::Mat, dist_coeffs: &core::Mat, rvec: &mut core::Mat, tvec: &mut core::Mat, use_extrinsic_guess: bool, iterations_count: i32, reprojection_error: f32, min_inliers_count: i32, inliers: &mut core::Mat, flags: i32) -> Result<bool> {
        unsafe { sys::cv_ccalib_CustomPattern_findRtRANSAC_Mat_Mat_Mat_Mat_Mat_Mat_bool_int_float_int_Mat_int(self.as_raw_CustomPattern(), object_points.as_raw_Mat(), image_points.as_raw_Mat(), camera_matrix.as_raw_Mat(), dist_coeffs.as_raw_Mat(), rvec.as_raw_Mat(), tvec.as_raw_Mat(), use_extrinsic_guess, iterations_count, reprojection_error, min_inliers_count, inliers.as_raw_Mat(), flags) }.into_result()
    }
    
    ///
    /// ## C++ default parameters
    /// * use_extrinsic_guess: false
    /// * iterations_count: 100
    /// * reprojection_error: 8.0
    /// * min_inliers_count: 100
    /// * inliers: noArray()
    /// * flags: SOLVEPNP_ITERATIVE
    pub fn find_rt_ransac_1(&mut self, image: &core::Mat, camera_matrix: &core::Mat, dist_coeffs: &core::Mat, rvec: &mut core::Mat, tvec: &mut core::Mat, use_extrinsic_guess: bool, iterations_count: i32, reprojection_error: f32, min_inliers_count: i32, inliers: &mut core::Mat, flags: i32) -> Result<bool> {
        unsafe { sys::cv_ccalib_CustomPattern_findRtRANSAC_Mat_Mat_Mat_Mat_Mat_bool_int_float_int_Mat_int(self.as_raw_CustomPattern(), image.as_raw_Mat(), camera_matrix.as_raw_Mat(), dist_coeffs.as_raw_Mat(), rvec.as_raw_Mat(), tvec.as_raw_Mat(), use_extrinsic_guess, iterations_count, reprojection_error, min_inliers_count, inliers.as_raw_Mat(), flags) }.into_result()
    }
    
    /// <
    /// Uses solvePnPRansac()
    ///
    /// ## C++ default parameters
    /// * axis_length: 3
    /// * axis_width: 2
    pub fn draw_orientation(&mut self, image: &mut core::Mat, tvec: &core::Mat, rvec: &core::Mat, camera_matrix: &core::Mat, dist_coeffs: &core::Mat, axis_length: f64, axis_width: i32) -> Result<()> {
        unsafe { sys::cv_ccalib_CustomPattern_drawOrientation_Mat_Mat_Mat_Mat_Mat_double_int(self.as_raw_CustomPattern(), image.as_raw_Mat(), tvec.as_raw_Mat(), rvec.as_raw_Mat(), camera_matrix.as_raw_Mat(), dist_coeffs.as_raw_Mat(), axis_length, axis_width) }.into_result()
    }
    
}

// boxed class cv::multicalib::MultiCameraCalibration
/// Class for multiple camera calibration that supports pinhole camera and omnidirection camera.
/// For omnidirectional camera model, please refer to omnidir.hpp in ccalib module.
/// It first calibrate each camera individually, then a bundle adjustment like optimization is applied to
/// refine extrinsic parameters. So far, it only support "random" pattern for calibration,
/// see randomPattern.hpp in ccalib module for details.
/// Images that are used should be named by "cameraIdx-timestamp.*", several images with the same timestamp
/// means that they are the same pattern that are photographed. cameraIdx should start from 0.
/// 
/// For more details, please refer to paper
/// B. Li, L. Heng, K. Kevin  and M. Pollefeys, "A Multiple-Camera System
/// Calibration Toolbox Using A Feature Descriptor-Based Calibration
/// Pattern", in IROS 2013.
#[allow(dead_code)]
pub struct MultiCameraCalibration {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::ccalib::MultiCameraCalibration {
    fn drop(&mut self) {
        unsafe { sys::cv_MultiCameraCalibration_delete(self.ptr) };
    }
}
impl crate::ccalib::MultiCameraCalibration {
    pub fn as_raw_MultiCameraCalibration(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        MultiCameraCalibration {
            ptr
        }
    }
}

impl MultiCameraCalibration {

    pub fn load_images(&mut self) -> Result<()> {
        unsafe { sys::cv_multicalib_MultiCameraCalibration_loadImages(self.as_raw_MultiCameraCalibration()) }.into_result()
    }
    
    pub fn initialize(&mut self) -> Result<()> {
        unsafe { sys::cv_multicalib_MultiCameraCalibration_initialize(self.as_raw_MultiCameraCalibration()) }.into_result()
    }
    
    pub fn optimize_extrinsics(&mut self) -> Result<f64> {
        unsafe { sys::cv_multicalib_MultiCameraCalibration_optimizeExtrinsics(self.as_raw_MultiCameraCalibration()) }.into_result()
    }
    
    pub fn run(&mut self) -> Result<f64> {
        unsafe { sys::cv_multicalib_MultiCameraCalibration_run(self.as_raw_MultiCameraCalibration()) }.into_result()
    }
    
    pub fn write_parameters(&mut self, filename: &str) -> Result<()> {
        string_arg!(filename);
        unsafe { sys::cv_multicalib_MultiCameraCalibration_writeParameters_std_string(self.as_raw_MultiCameraCalibration(), filename.as_ptr()) }.into_result()
    }
    
}

// boxed class cv::multicalib::MultiCameraCalibration::edge
#[allow(dead_code)]
pub struct MultiCameraCalibration_edge {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::ccalib::MultiCameraCalibration_edge {
    fn drop(&mut self) {
        unsafe { sys::cv_MultiCameraCalibration_edge_delete(self.ptr) };
    }
}
impl crate::ccalib::MultiCameraCalibration_edge {
    pub fn as_raw_MultiCameraCalibration_edge(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        MultiCameraCalibration_edge {
            ptr
        }
    }
}

impl MultiCameraCalibration_edge {

    pub fn new(cv: i32, pv: i32, pi: i32, trans: &core::Mat) -> Result<crate::ccalib::MultiCameraCalibration_edge> {
        unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_edge_int_int_int_Mat(cv, pv, pi, trans.as_raw_Mat()) }.into_result().map(|x| crate::ccalib::MultiCameraCalibration_edge { ptr: x })
    }
    
}

// boxed class cv::multicalib::MultiCameraCalibration::vertex
#[allow(dead_code)]
pub struct MultiCameraCalibration_vertex {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::ccalib::MultiCameraCalibration_vertex {
    fn drop(&mut self) {
        unsafe { sys::cv_MultiCameraCalibration_vertex_delete(self.ptr) };
    }
}
impl crate::ccalib::MultiCameraCalibration_vertex {
    pub fn as_raw_MultiCameraCalibration_vertex(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        MultiCameraCalibration_vertex {
            ptr
        }
    }
}

impl MultiCameraCalibration_vertex {

    pub fn new(po: &core::Mat, ts: i32) -> Result<crate::ccalib::MultiCameraCalibration_vertex> {
        unsafe { sys::cv_multicalib_MultiCameraCalibration_vertex_vertex_Mat_int(po.as_raw_Mat(), ts) }.into_result().map(|x| crate::ccalib::MultiCameraCalibration_vertex { ptr: x })
    }
    
    pub fn new_1() -> Result<crate::ccalib::MultiCameraCalibration_vertex> {
        unsafe { sys::cv_multicalib_MultiCameraCalibration_vertex_vertex() }.into_result().map(|x| crate::ccalib::MultiCameraCalibration_vertex { ptr: x })
    }
    
}

// boxed class cv::randpattern::RandomPatternCornerFinder
/// Class for finding features points and corresponding 3D in world coordinate of
/// a "random" pattern, which can be to be used in calibration. It is useful when pattern is
/// partly occluded or only a part of pattern can be observed in multiple cameras calibration.
/// The pattern can be generated by RandomPatternGenerator class described in this file.
/// 
/// Please refer to paper
/// B. Li, L. Heng, K. Kevin  and M. Pollefeys, "A Multiple-Camera System
/// Calibration Toolbox Using A Feature Descriptor-Based Calibration
/// Pattern", in IROS 2013.
#[allow(dead_code)]
pub struct RandomPatternCornerFinder {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::ccalib::RandomPatternCornerFinder {
    fn drop(&mut self) {
        unsafe { sys::cv_RandomPatternCornerFinder_delete(self.ptr) };
    }
}
impl crate::ccalib::RandomPatternCornerFinder {
    pub fn as_raw_RandomPatternCornerFinder(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        RandomPatternCornerFinder {
            ptr
        }
    }
}

impl RandomPatternCornerFinder {

    pub fn load_pattern(&mut self, pattern_image: &core::Mat) -> Result<()> {
        unsafe { sys::cv_randpattern_RandomPatternCornerFinder_loadPattern_Mat(self.as_raw_RandomPatternCornerFinder(), pattern_image.as_raw_Mat()) }.into_result()
    }
    
    pub fn load_pattern_1(&mut self, pattern_image: &core::Mat, pattern_key_points: &types::VectorOfKeyPoint, pattern_descriptors: &core::Mat) -> Result<()> {
        unsafe { sys::cv_randpattern_RandomPatternCornerFinder_loadPattern_Mat_VectorOfKeyPoint_Mat(self.as_raw_RandomPatternCornerFinder(), pattern_image.as_raw_Mat(), pattern_key_points.as_raw_VectorOfKeyPoint(), pattern_descriptors.as_raw_Mat()) }.into_result()
    }
    
    pub fn compute_object_image_points(&mut self, input_images: &types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_randpattern_RandomPatternCornerFinder_computeObjectImagePoints_VectorOfMat(self.as_raw_RandomPatternCornerFinder(), input_images.as_raw_VectorOfMat()) }.into_result()
    }
    
    pub fn compute_object_image_points_for_single(&mut self, input_image: &core::Mat) -> Result<types::VectorOfMat> {
        unsafe { sys::cv_randpattern_RandomPatternCornerFinder_computeObjectImagePointsForSingle_Mat(self.as_raw_RandomPatternCornerFinder(), input_image.as_raw_Mat()) }.into_result().map(|x| types::VectorOfMat { ptr: x })
    }
    
    pub fn get_object_points(&mut self) -> Result<types::VectorOfMat> {
        unsafe { sys::cv_randpattern_RandomPatternCornerFinder_getObjectPoints(self.as_raw_RandomPatternCornerFinder()) }.into_result().map(|x| types::VectorOfMat { ptr: x })
    }
    
    pub fn get_image_points(&mut self) -> Result<types::VectorOfMat> {
        unsafe { sys::cv_randpattern_RandomPatternCornerFinder_getImagePoints(self.as_raw_RandomPatternCornerFinder()) }.into_result().map(|x| types::VectorOfMat { ptr: x })
    }
    
}

// boxed class cv::randpattern::RandomPatternGenerator
#[allow(dead_code)]
pub struct RandomPatternGenerator {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::ccalib::RandomPatternGenerator {
    fn drop(&mut self) {
        unsafe { sys::cv_RandomPatternGenerator_delete(self.ptr) };
    }
}
impl crate::ccalib::RandomPatternGenerator {
    pub fn as_raw_RandomPatternGenerator(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        RandomPatternGenerator {
            ptr
        }
    }
}

impl RandomPatternGenerator {

    pub fn new(image_width: i32, image_height: i32) -> Result<crate::ccalib::RandomPatternGenerator> {
        unsafe { sys::cv_randpattern_RandomPatternGenerator_RandomPatternGenerator_int_int(image_width, image_height) }.into_result().map(|x| crate::ccalib::RandomPatternGenerator { ptr: x })
    }
    
    pub fn generate_pattern(&mut self) -> Result<()> {
        unsafe { sys::cv_randpattern_RandomPatternGenerator_generatePattern(self.as_raw_RandomPatternGenerator()) }.into_result()
    }
    
    pub fn get_pattern(&mut self) -> Result<core::Mat> {
        unsafe { sys::cv_randpattern_RandomPatternGenerator_getPattern(self.as_raw_RandomPatternGenerator()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
}

