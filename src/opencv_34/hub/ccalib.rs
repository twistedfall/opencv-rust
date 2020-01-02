//! # Custom Calibration Pattern for 3D reconstruction
use crate::{mod_prelude::*, core, sys, types};
use crate::core::{_InputArrayTrait, _OutputArrayTrait};

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
/// * D: Output distortion parameters ![inline formula](https://latex.codecogs.com/png.latex?%28k_1%2C%20k_2%2C%20p_1%2C%20p_2%29)
/// * rvecs: Output rotations for each calibration images
/// * tvecs: Output translation for each calibration images
/// * flags: The flags that control calibrate
/// * criteria: Termination criteria for optimization
/// * idx: Indices of images that pass initialization, which are really used in calibration. So the size of rvecs is the
/// same as idx.total().
///
/// ## C++ default parameters
/// * idx: noArray()
pub fn calibrate(object_points: &dyn core::ToInputArray, image_points: &dyn core::ToInputArray, size: core::Size, k: &mut dyn core::ToInputOutputArray, xi: &mut dyn core::ToInputOutputArray, d: &mut dyn core::ToInputOutputArray, rvecs: &mut dyn core::ToOutputArray, tvecs: &mut dyn core::ToOutputArray, flags: i32, criteria: &core::TermCriteria, idx: &mut dyn core::ToOutputArray) -> Result<f64> {
    input_array_arg!(object_points);
    input_array_arg!(image_points);
    input_output_array_arg!(k);
    input_output_array_arg!(xi);
    input_output_array_arg!(d);
    output_array_arg!(rvecs);
    output_array_arg!(tvecs);
    output_array_arg!(idx);
    unsafe { sys::cv_omnidir_calibrate__InputArray__InputArray_Size__InputOutputArray__InputOutputArray__InputOutputArray__OutputArray__OutputArray_int_TermCriteria__OutputArray(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), size, k.as_raw__InputOutputArray(), xi.as_raw__InputOutputArray(), d.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, criteria.as_raw_TermCriteria(), idx.as_raw__OutputArray()) }.into_result()
}

/// Computes undistortion and rectification maps for omnidirectional camera image transform by a rotation R.
/// It output two maps that are used for cv::remap(). If D is empty then zero distortion is used,
/// if R or P is empty then identity matrices are used.
///
/// ## Parameters
/// * K: Camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f_x%20%26%20s%20%26%20c_x%5C%5C%200%20%26%20f_y%20%26%20c_y%5C%5C%200%20%26%200%20%26%20_1%20%5Cend%7Bbmatrix%7D), with depth CV_32F or CV_64F
/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%28k_1%2C%20k_2%2C%20p_1%2C%20p_2%29), with depth CV_32F or CV_64F
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
pub fn init_undistort_rectify_map(k: &dyn core::ToInputArray, d: &dyn core::ToInputArray, xi: &dyn core::ToInputArray, r: &dyn core::ToInputArray, p: &dyn core::ToInputArray, size: core::Size, mltype: i32, map1: &mut dyn core::ToOutputArray, map2: &mut dyn core::ToOutputArray, flags: i32) -> Result<()> {
    input_array_arg!(k);
    input_array_arg!(d);
    input_array_arg!(xi);
    input_array_arg!(r);
    input_array_arg!(p);
    output_array_arg!(map1);
    output_array_arg!(map2);
    unsafe { sys::cv_omnidir_initUndistortRectifyMap__InputArray__InputArray__InputArray__InputArray__InputArray_Size_int__OutputArray__OutputArray_int(k.as_raw__InputArray(), d.as_raw__InputArray(), xi.as_raw__InputArray(), r.as_raw__InputArray(), p.as_raw__InputArray(), size, mltype, map1.as_raw__OutputArray(), map2.as_raw__OutputArray(), flags) }.into_result()
}

pub fn compose_motion(_om1: &dyn core::ToInputArray, _t1: &dyn core::ToInputArray, _om2: &dyn core::ToInputArray, _t2: &dyn core::ToInputArray, om3: &mut core::Mat, t3: &mut core::Mat, dom3dom1: &mut core::Mat, dom3d_t1: &mut core::Mat, dom3dom2: &mut core::Mat, dom3d_t2: &mut core::Mat, d_t3dom1: &mut core::Mat, d_t3d_t1: &mut core::Mat, d_t3dom2: &mut core::Mat, d_t3d_t2: &mut core::Mat) -> Result<()> {
    input_array_arg!(_om1);
    input_array_arg!(_t1);
    input_array_arg!(_om2);
    input_array_arg!(_t2);
    unsafe { sys::cv_omnidir_internal_compose_motion__InputArray__InputArray__InputArray__InputArray_Mat_Mat_Mat_Mat_Mat_Mat_Mat_Mat_Mat_Mat(_om1.as_raw__InputArray(), _t1.as_raw__InputArray(), _om2.as_raw__InputArray(), _t2.as_raw__InputArray(), om3.as_raw_Mat(), t3.as_raw_Mat(), dom3dom1.as_raw_Mat(), dom3d_t1.as_raw_Mat(), dom3dom2.as_raw_Mat(), dom3d_t2.as_raw_Mat(), d_t3dom1.as_raw_Mat(), d_t3d_t1.as_raw_Mat(), d_t3dom2.as_raw_Mat(), d_t3d_t2.as_raw_Mat()) }.into_result()
}

pub fn compute_jacobian_stereo(object_points: &dyn core::ToInputArray, image_points1: &dyn core::ToInputArray, image_points2: &dyn core::ToInputArray, parameters: &dyn core::ToInputArray, jtj_inv: &mut core::Mat, jte: &mut core::Mat, flags: i32, epsilon: f64) -> Result<()> {
    input_array_arg!(object_points);
    input_array_arg!(image_points1);
    input_array_arg!(image_points2);
    input_array_arg!(parameters);
    unsafe { sys::cv_omnidir_internal_computeJacobianStereo__InputArray__InputArray__InputArray__InputArray_Mat_Mat_int_double(object_points.as_raw__InputArray(), image_points1.as_raw__InputArray(), image_points2.as_raw__InputArray(), parameters.as_raw__InputArray(), jtj_inv.as_raw_Mat(), jte.as_raw_Mat(), flags, epsilon) }.into_result()
}

pub fn compute_jacobian(object_points: &dyn core::ToInputArray, image_points: &dyn core::ToInputArray, parameters: &dyn core::ToInputArray, jtj_inv: &mut core::Mat, jte: &mut core::Mat, flags: i32, epsilon: f64) -> Result<()> {
    input_array_arg!(object_points);
    input_array_arg!(image_points);
    input_array_arg!(parameters);
    unsafe { sys::cv_omnidir_internal_computeJacobian__InputArray__InputArray__InputArray_Mat_Mat_int_double(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), parameters.as_raw__InputArray(), jtj_inv.as_raw_Mat(), jte.as_raw_Mat(), flags, epsilon) }.into_result()
}

pub fn compute_mean_repro_err_stereo(object_points: &dyn core::ToInputArray, image_points1: &dyn core::ToInputArray, image_points2: &dyn core::ToInputArray, k1: &dyn core::ToInputArray, k2: &dyn core::ToInputArray, d1: &dyn core::ToInputArray, d2: &dyn core::ToInputArray, xi1: f64, xi2: f64, om: &dyn core::ToInputArray, t: &dyn core::ToInputArray, om_l: &dyn core::ToInputArray, tl: &dyn core::ToInputArray) -> Result<f64> {
    input_array_arg!(object_points);
    input_array_arg!(image_points1);
    input_array_arg!(image_points2);
    input_array_arg!(k1);
    input_array_arg!(k2);
    input_array_arg!(d1);
    input_array_arg!(d2);
    input_array_arg!(om);
    input_array_arg!(t);
    input_array_arg!(om_l);
    input_array_arg!(tl);
    unsafe { sys::cv_omnidir_internal_computeMeanReproErrStereo__InputArray__InputArray__InputArray__InputArray__InputArray__InputArray__InputArray_double_double__InputArray__InputArray__InputArray__InputArray(object_points.as_raw__InputArray(), image_points1.as_raw__InputArray(), image_points2.as_raw__InputArray(), k1.as_raw__InputArray(), k2.as_raw__InputArray(), d1.as_raw__InputArray(), d2.as_raw__InputArray(), xi1, xi2, om.as_raw__InputArray(), t.as_raw__InputArray(), om_l.as_raw__InputArray(), tl.as_raw__InputArray()) }.into_result()
}

pub fn compute_mean_repro_err(image_points: &dyn core::ToInputArray, pro_image_points: &dyn core::ToInputArray) -> Result<f64> {
    input_array_arg!(image_points);
    input_array_arg!(pro_image_points);
    unsafe { sys::cv_omnidir_internal_computeMeanReproErr__InputArray__InputArray(image_points.as_raw__InputArray(), pro_image_points.as_raw__InputArray()) }.into_result()
}

pub fn compute_mean_repro_err_1(object_points: &dyn core::ToInputArray, image_points: &dyn core::ToInputArray, k: &dyn core::ToInputArray, d: &dyn core::ToInputArray, xi: f64, om_all: &dyn core::ToInputArray, t_all: &dyn core::ToInputArray) -> Result<f64> {
    input_array_arg!(object_points);
    input_array_arg!(image_points);
    input_array_arg!(k);
    input_array_arg!(d);
    input_array_arg!(om_all);
    input_array_arg!(t_all);
    unsafe { sys::cv_omnidir_internal_computeMeanReproErr__InputArray__InputArray__InputArray__InputArray_double__InputArray__InputArray(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), xi, om_all.as_raw__InputArray(), t_all.as_raw__InputArray()) }.into_result()
}

pub fn decode_parameters_stereo(parameters: &dyn core::ToInputArray, k1: &mut dyn core::ToOutputArray, k2: &mut dyn core::ToOutputArray, om: &mut dyn core::ToOutputArray, t: &mut dyn core::ToOutputArray, om_l: &mut dyn core::ToOutputArray, t_l: &mut dyn core::ToOutputArray, d1: &mut dyn core::ToOutputArray, d2: &mut dyn core::ToOutputArray, xi1: &mut f64, xi2: &mut f64) -> Result<()> {
    input_array_arg!(parameters);
    output_array_arg!(k1);
    output_array_arg!(k2);
    output_array_arg!(om);
    output_array_arg!(t);
    output_array_arg!(om_l);
    output_array_arg!(t_l);
    output_array_arg!(d1);
    output_array_arg!(d2);
    unsafe { sys::cv_omnidir_internal_decodeParametersStereo__InputArray__OutputArray__OutputArray__OutputArray__OutputArray__OutputArray__OutputArray__OutputArray__OutputArray_double_double(parameters.as_raw__InputArray(), k1.as_raw__OutputArray(), k2.as_raw__OutputArray(), om.as_raw__OutputArray(), t.as_raw__OutputArray(), om_l.as_raw__OutputArray(), t_l.as_raw__OutputArray(), d1.as_raw__OutputArray(), d2.as_raw__OutputArray(), xi1, xi2) }.into_result()
}

pub fn decode_parameters(paramsters: &dyn core::ToInputArray, k: &mut dyn core::ToOutputArray, om_all: &mut dyn core::ToOutputArray, t_all: &mut dyn core::ToOutputArray, distoration: &mut dyn core::ToOutputArray, xi: &mut f64) -> Result<()> {
    input_array_arg!(paramsters);
    output_array_arg!(k);
    output_array_arg!(om_all);
    output_array_arg!(t_all);
    output_array_arg!(distoration);
    unsafe { sys::cv_omnidir_internal_decodeParameters__InputArray__OutputArray__OutputArray__OutputArray__OutputArray_double(paramsters.as_raw__InputArray(), k.as_raw__OutputArray(), om_all.as_raw__OutputArray(), t_all.as_raw__OutputArray(), distoration.as_raw__OutputArray(), xi) }.into_result()
}

pub fn encode_parameters_stereo(k1: &dyn core::ToInputArray, k2: &dyn core::ToInputArray, om: &dyn core::ToInputArray, t: &dyn core::ToInputArray, om_l: &dyn core::ToInputArray, t_l: &dyn core::ToInputArray, d1: &dyn core::ToInputArray, d2: &dyn core::ToInputArray, xi1: f64, xi2: f64, parameters: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(k1);
    input_array_arg!(k2);
    input_array_arg!(om);
    input_array_arg!(t);
    input_array_arg!(om_l);
    input_array_arg!(t_l);
    input_array_arg!(d1);
    input_array_arg!(d2);
    output_array_arg!(parameters);
    unsafe { sys::cv_omnidir_internal_encodeParametersStereo__InputArray__InputArray__InputArray__InputArray__InputArray__InputArray__InputArray__InputArray_double_double__OutputArray(k1.as_raw__InputArray(), k2.as_raw__InputArray(), om.as_raw__InputArray(), t.as_raw__InputArray(), om_l.as_raw__InputArray(), t_l.as_raw__InputArray(), d1.as_raw__InputArray(), d2.as_raw__InputArray(), xi1, xi2, parameters.as_raw__OutputArray()) }.into_result()
}

pub fn encode_parameters(k: &dyn core::ToInputArray, om_all: &dyn core::ToInputArray, t_all: &dyn core::ToInputArray, distoaration: &dyn core::ToInputArray, xi: f64, parameters: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(k);
    input_array_arg!(om_all);
    input_array_arg!(t_all);
    input_array_arg!(distoaration);
    output_array_arg!(parameters);
    unsafe { sys::cv_omnidir_internal_encodeParameters__InputArray__InputArray__InputArray__InputArray_double__OutputArray(k.as_raw__InputArray(), om_all.as_raw__InputArray(), t_all.as_raw__InputArray(), distoaration.as_raw__InputArray(), xi, parameters.as_raw__OutputArray()) }.into_result()
}

pub fn estimate_uncertainties_stereo(object_points: &dyn core::ToInputArray, image_points1: &dyn core::ToInputArray, image_points2: &dyn core::ToInputArray, parameters: &dyn core::ToInputArray, errors: &mut core::Mat, std_error: &mut core::Vec2d, rms: &mut f64, flags: i32) -> Result<()> {
    input_array_arg!(object_points);
    input_array_arg!(image_points1);
    input_array_arg!(image_points2);
    input_array_arg!(parameters);
    unsafe { sys::cv_omnidir_internal_estimateUncertaintiesStereo__InputArray__InputArray__InputArray__InputArray_Mat_Vec2d_double_int(object_points.as_raw__InputArray(), image_points1.as_raw__InputArray(), image_points2.as_raw__InputArray(), parameters.as_raw__InputArray(), errors.as_raw_Mat(), std_error, rms, flags) }.into_result()
}

pub fn estimate_uncertainties(object_points: &dyn core::ToInputArray, image_points: &dyn core::ToInputArray, parameters: &dyn core::ToInputArray, errors: &mut core::Mat, std_error: &mut core::Vec2d, rms: &mut f64, flags: i32) -> Result<()> {
    input_array_arg!(object_points);
    input_array_arg!(image_points);
    input_array_arg!(parameters);
    unsafe { sys::cv_omnidir_internal_estimateUncertainties__InputArray__InputArray__InputArray_Mat_Vec2d_double_int(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), parameters.as_raw__InputArray(), errors.as_raw_Mat(), std_error, rms, flags) }.into_result()
}

pub fn fill_fixed_stereo(g: &mut core::Mat, flags: i32, n: i32) -> Result<()> {
    unsafe { sys::cv_omnidir_internal_fillFixedStereo_Mat_int_int(g.as_raw_Mat(), flags, n) }.into_result()
}

pub fn fill_fixed(g: &mut core::Mat, flags: i32, n: i32) -> Result<()> {
    unsafe { sys::cv_omnidir_internal_fillFixed_Mat_int_int(g.as_raw_Mat(), flags, n) }.into_result()
}

pub fn find_median3(mat: &dyn core::ToInputArray) -> Result<core::Vec3d> {
    input_array_arg!(mat);
    unsafe { sys::cv_omnidir_internal_findMedian3__InputArray(mat.as_raw__InputArray()) }.into_result()
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

pub fn get_interset(idx1: &dyn core::ToInputArray, idx2: &dyn core::ToInputArray, inter1: &mut dyn core::ToOutputArray, inter2: &mut dyn core::ToOutputArray, inter_ori: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(idx1);
    input_array_arg!(idx2);
    output_array_arg!(inter1);
    output_array_arg!(inter2);
    output_array_arg!(inter_ori);
    unsafe { sys::cv_omnidir_internal_getInterset__InputArray__InputArray__OutputArray__OutputArray__OutputArray(idx1.as_raw__InputArray(), idx2.as_raw__InputArray(), inter1.as_raw__OutputArray(), inter2.as_raw__OutputArray(), inter_ori.as_raw__OutputArray()) }.into_result()
}

///
/// ## C++ default parameters
/// * idx: noArray()
pub fn initialize_calibration(object_points: &dyn core::ToInputArray, image_points: &dyn core::ToInputArray, size: core::Size, om_all: &mut dyn core::ToOutputArray, t_all: &mut dyn core::ToOutputArray, k: &mut dyn core::ToOutputArray, xi: &mut f64, idx: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(object_points);
    input_array_arg!(image_points);
    output_array_arg!(om_all);
    output_array_arg!(t_all);
    output_array_arg!(k);
    output_array_arg!(idx);
    unsafe { sys::cv_omnidir_internal_initializeCalibration__InputArray__InputArray_Size__OutputArray__OutputArray__OutputArray_double__OutputArray(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), size, om_all.as_raw__OutputArray(), t_all.as_raw__OutputArray(), k.as_raw__OutputArray(), xi, idx.as_raw__OutputArray()) }.into_result()
}

pub fn initialize_stereo_calibration(object_points: &dyn core::ToInputArray, image_points1: &dyn core::ToInputArray, image_points2: &dyn core::ToInputArray, size1: core::Size, size2: core::Size, om: &mut dyn core::ToOutputArray, t: &mut dyn core::ToOutputArray, om_l: &mut dyn core::ToOutputArray, t_l: &mut dyn core::ToOutputArray, k1: &mut dyn core::ToOutputArray, d1: &mut dyn core::ToOutputArray, k2: &mut dyn core::ToOutputArray, d2: &mut dyn core::ToOutputArray, xi1: &mut f64, xi2: &mut f64, flags: i32, idx: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(object_points);
    input_array_arg!(image_points1);
    input_array_arg!(image_points2);
    output_array_arg!(om);
    output_array_arg!(t);
    output_array_arg!(om_l);
    output_array_arg!(t_l);
    output_array_arg!(k1);
    output_array_arg!(d1);
    output_array_arg!(k2);
    output_array_arg!(d2);
    output_array_arg!(idx);
    unsafe { sys::cv_omnidir_internal_initializeStereoCalibration__InputArray__InputArray__InputArray_Size_Size__OutputArray__OutputArray__OutputArray__OutputArray__OutputArray__OutputArray__OutputArray__OutputArray_double_double_int__OutputArray(object_points.as_raw__InputArray(), image_points1.as_raw__InputArray(), image_points2.as_raw__InputArray(), size1, size2, om.as_raw__OutputArray(), t.as_raw__OutputArray(), om_l.as_raw__OutputArray(), t_l.as_raw__OutputArray(), k1.as_raw__OutputArray(), d1.as_raw__OutputArray(), k2.as_raw__OutputArray(), d2.as_raw__OutputArray(), xi1, xi2, flags, idx.as_raw__OutputArray()) }.into_result()
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
/// * K: Camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f_x%20%26%20s%20%26%20c_x%5C%5C%200%20%26%20f_y%20%26%20c_y%5C%5C%200%20%26%200%20%26%20_1%20%5Cend%7Bbmatrix%7D).
/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%28k_1%2C%20k_2%2C%20p_1%2C%20p_2%29).
/// * xi: The parameter xi for CMei's model
/// * jacobian: Optional output 2Nx16 of type CV_64F jacobian matrix, contains the derivatives of
/// image pixel points wrt parameters including ![inline formula](https://latex.codecogs.com/png.latex?om%2C%20T%2C%20f_x%2C%20f_y%2C%20s%2C%20c_x%2C%20c_y%2C%20xi%2C%20k_1%2C%20k_2%2C%20p_1%2C%20p_2).
/// This matrix will be used in calibration by optimization.
///
/// The function projects object 3D points of world coordinate to image pixels, parameter by intrinsic
/// and extrinsic parameters. Also, it optionally compute a by-product: the jacobian matrix containing
/// contains the derivatives of image pixel points wrt intrinsic and extrinsic parameters.
///
/// ## C++ default parameters
/// * jacobian: noArray()
pub fn project_points(object_points: &dyn core::ToInputArray, image_points: &mut dyn core::ToOutputArray, rvec: &dyn core::ToInputArray, tvec: &dyn core::ToInputArray, k: &dyn core::ToInputArray, xi: f64, d: &dyn core::ToInputArray, jacobian: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(object_points);
    output_array_arg!(image_points);
    input_array_arg!(rvec);
    input_array_arg!(tvec);
    input_array_arg!(k);
    input_array_arg!(d);
    output_array_arg!(jacobian);
    unsafe { sys::cv_omnidir_projectPoints__InputArray__OutputArray__InputArray__InputArray__InputArray_double__InputArray__OutputArray(object_points.as_raw__InputArray(), image_points.as_raw__OutputArray(), rvec.as_raw__InputArray(), tvec.as_raw__InputArray(), k.as_raw__InputArray(), xi, d.as_raw__InputArray(), jacobian.as_raw__OutputArray()) }.into_result()
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
/// * D1: Output distortion parameters ![inline formula](https://latex.codecogs.com/png.latex?%28k_1%2C%20k_2%2C%20p_1%2C%20p_2%29) for the first camera
/// * K2: Output camera matrix for the first camera.
/// * xi2: Output parameter xi of CMei's model for the second camera
/// * D2: Output distortion parameters ![inline formula](https://latex.codecogs.com/png.latex?%28k_1%2C%20k_2%2C%20p_1%2C%20p_2%29) for the second camera
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
pub fn stereo_calibrate(object_points: &mut dyn core::ToInputOutputArray, image_points1: &mut dyn core::ToInputOutputArray, image_points2: &mut dyn core::ToInputOutputArray, image_size1: core::Size, image_size2: core::Size, k1: &mut dyn core::ToInputOutputArray, xi1: &mut dyn core::ToInputOutputArray, d1: &mut dyn core::ToInputOutputArray, k2: &mut dyn core::ToInputOutputArray, xi2: &mut dyn core::ToInputOutputArray, d2: &mut dyn core::ToInputOutputArray, rvec: &mut dyn core::ToOutputArray, tvec: &mut dyn core::ToOutputArray, rvecs_l: &mut dyn core::ToOutputArray, tvecs_l: &mut dyn core::ToOutputArray, flags: i32, criteria: &core::TermCriteria, idx: &mut dyn core::ToOutputArray) -> Result<f64> {
    input_output_array_arg!(object_points);
    input_output_array_arg!(image_points1);
    input_output_array_arg!(image_points2);
    input_output_array_arg!(k1);
    input_output_array_arg!(xi1);
    input_output_array_arg!(d1);
    input_output_array_arg!(k2);
    input_output_array_arg!(xi2);
    input_output_array_arg!(d2);
    output_array_arg!(rvec);
    output_array_arg!(tvec);
    output_array_arg!(rvecs_l);
    output_array_arg!(tvecs_l);
    output_array_arg!(idx);
    unsafe { sys::cv_omnidir_stereoCalibrate__InputOutputArray__InputOutputArray__InputOutputArray_Size_Size__InputOutputArray__InputOutputArray__InputOutputArray__InputOutputArray__InputOutputArray__InputOutputArray__OutputArray__OutputArray__OutputArray__OutputArray_int_TermCriteria__OutputArray(object_points.as_raw__InputOutputArray(), image_points1.as_raw__InputOutputArray(), image_points2.as_raw__InputOutputArray(), image_size1, image_size2, k1.as_raw__InputOutputArray(), xi1.as_raw__InputOutputArray(), d1.as_raw__InputOutputArray(), k2.as_raw__InputOutputArray(), xi2.as_raw__InputOutputArray(), d2.as_raw__InputOutputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), rvecs_l.as_raw__OutputArray(), tvecs_l.as_raw__OutputArray(), flags, criteria.as_raw_TermCriteria(), idx.as_raw__OutputArray()) }.into_result()
}

/// Stereo 3D reconstruction from a pair of images
///
/// ## Parameters
/// * image1: The first input image
/// * image2: The second input image
/// * K1: Input camera matrix of the first camera
/// * D1: Input distortion parameters ![inline formula](https://latex.codecogs.com/png.latex?%28k_1%2C%20k_2%2C%20p_1%2C%20p_2%29) for the first camera
/// * xi1: Input parameter xi for the first camera for CMei's model
/// * K2: Input camera matrix of the second camera
/// * D2: Input distortion parameters ![inline formula](https://latex.codecogs.com/png.latex?%28k_1%2C%20k_2%2C%20p_1%2C%20p_2%29) for the second camera
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
pub fn stereo_reconstruct(image1: &dyn core::ToInputArray, image2: &dyn core::ToInputArray, k1: &dyn core::ToInputArray, d1: &dyn core::ToInputArray, xi1: &dyn core::ToInputArray, k2: &dyn core::ToInputArray, d2: &dyn core::ToInputArray, xi2: &dyn core::ToInputArray, r: &dyn core::ToInputArray, t: &dyn core::ToInputArray, flag: i32, num_disparities: i32, sad_window_size: i32, disparity: &mut dyn core::ToOutputArray, image1_rec: &mut dyn core::ToOutputArray, image2_rec: &mut dyn core::ToOutputArray, new_size: core::Size, knew: &dyn core::ToInputArray, point_cloud: &mut dyn core::ToOutputArray, point_type: i32) -> Result<()> {
    input_array_arg!(image1);
    input_array_arg!(image2);
    input_array_arg!(k1);
    input_array_arg!(d1);
    input_array_arg!(xi1);
    input_array_arg!(k2);
    input_array_arg!(d2);
    input_array_arg!(xi2);
    input_array_arg!(r);
    input_array_arg!(t);
    output_array_arg!(disparity);
    output_array_arg!(image1_rec);
    output_array_arg!(image2_rec);
    input_array_arg!(knew);
    output_array_arg!(point_cloud);
    unsafe { sys::cv_omnidir_stereoReconstruct__InputArray__InputArray__InputArray__InputArray__InputArray__InputArray__InputArray__InputArray__InputArray__InputArray_int_int_int__OutputArray__OutputArray__OutputArray_Size__InputArray__OutputArray_int(image1.as_raw__InputArray(), image2.as_raw__InputArray(), k1.as_raw__InputArray(), d1.as_raw__InputArray(), xi1.as_raw__InputArray(), k2.as_raw__InputArray(), d2.as_raw__InputArray(), xi2.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), flag, num_disparities, sad_window_size, disparity.as_raw__OutputArray(), image1_rec.as_raw__OutputArray(), image2_rec.as_raw__OutputArray(), new_size, knew.as_raw__InputArray(), point_cloud.as_raw__OutputArray(), point_type) }.into_result()
}

/// Stereo rectification for omnidirectional camera model. It computes the rectification rotations for two cameras
///
/// ## Parameters
/// * R: Rotation between the first and second camera
/// * T: Translation between the first and second camera
/// * R1: Output 3x3 rotation matrix for the first camera
/// * R2: Output 3x3 rotation matrix for the second camera
pub fn stereo_rectify(r: &dyn core::ToInputArray, t: &dyn core::ToInputArray, r1: &mut dyn core::ToOutputArray, r2: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(r);
    input_array_arg!(t);
    output_array_arg!(r1);
    output_array_arg!(r2);
    unsafe { sys::cv_omnidir_stereoRectify__InputArray__InputArray__OutputArray__OutputArray(r.as_raw__InputArray(), t.as_raw__InputArray(), r1.as_raw__OutputArray(), r2.as_raw__OutputArray()) }.into_result()
}

/// Undistort omnidirectional images to perspective images
///
/// ## Parameters
/// * distorted: The input omnidirectional image.
/// * undistorted: The output undistorted image.
/// * K: Camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f_x%20%26%20s%20%26%20c_x%5C%5C%200%20%26%20f_y%20%26%20c_y%5C%5C%200%20%26%200%20%26%20_1%20%5Cend%7Bbmatrix%7D).
/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%28k_1%2C%20k_2%2C%20p_1%2C%20p_2%29).
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
pub fn undistort_image(distorted: &dyn core::ToInputArray, undistorted: &mut dyn core::ToOutputArray, k: &dyn core::ToInputArray, d: &dyn core::ToInputArray, xi: &dyn core::ToInputArray, flags: i32, knew: &dyn core::ToInputArray, new_size: core::Size, r: &dyn core::ToInputArray) -> Result<()> {
    input_array_arg!(distorted);
    output_array_arg!(undistorted);
    input_array_arg!(k);
    input_array_arg!(d);
    input_array_arg!(xi);
    input_array_arg!(knew);
    input_array_arg!(r);
    unsafe { sys::cv_omnidir_undistortImage__InputArray__OutputArray__InputArray__InputArray__InputArray_int__InputArray_Size__InputArray(distorted.as_raw__InputArray(), undistorted.as_raw__OutputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), xi.as_raw__InputArray(), flags, knew.as_raw__InputArray(), new_size, r.as_raw__InputArray()) }.into_result()
}

/// Undistort 2D image points for omnidirectional camera using CMei's model
///
/// ## Parameters
/// * distorted: Array of distorted image points, vector of Vec2f
/// or 1xN/Nx1 2-channel Mat of type CV_32F, 64F depth is also acceptable
/// * K: Camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f_x%20%26%20s%20%26%20c_x%5C%5C%200%20%26%20f_y%20%26%20c_y%5C%5C%200%20%26%200%20%26%20_1%20%5Cend%7Bbmatrix%7D).
/// * D: Distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%28k_1%2C%20k_2%2C%20p_1%2C%20p_2%29).
/// * xi: The parameter xi for CMei's model
/// * R: Rotation trainsform between the original and object space : 3x3 1-channel, or vector: 3x1/1x3
/// 1-channel or 1x1 3-channel
/// * undistorted: array of normalized object points, vector of Vec2f/Vec2d or 1xN/Nx1 2-channel Mat with the same
/// depth of distorted points.
pub fn undistort_points(distorted: &dyn core::ToInputArray, undistorted: &mut dyn core::ToOutputArray, k: &dyn core::ToInputArray, d: &dyn core::ToInputArray, xi: &dyn core::ToInputArray, r: &dyn core::ToInputArray) -> Result<()> {
    input_array_arg!(distorted);
    output_array_arg!(undistorted);
    input_array_arg!(k);
    input_array_arg!(d);
    input_array_arg!(xi);
    input_array_arg!(r);
    unsafe { sys::cv_omnidir_undistortPoints__InputArray__OutputArray__InputArray__InputArray__InputArray__InputArray(distorted.as_raw__InputArray(), undistorted.as_raw__OutputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), xi.as_raw__InputArray(), r.as_raw__InputArray()) }.into_result()
}

// boxed class cv::ccalib::CustomPattern
pub struct CustomPattern {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for CustomPattern {
    fn drop(&mut self) {
        unsafe { sys::cv_CustomPattern_delete(self.ptr) };
    }
}

impl CustomPattern {
    #[inline(always)] pub fn as_raw_CustomPattern(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for CustomPattern {}

impl core::AlgorithmTrait for CustomPattern {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl CustomPattern {
    pub fn default() -> Result<crate::ccalib::CustomPattern> {
        unsafe { sys::cv_ccalib_CustomPattern_CustomPattern() }.into_result().map(|ptr| crate::ccalib::CustomPattern { ptr })
    }
    
    ///
    /// ## C++ default parameters
    /// * output: noArray()
    pub fn create(&mut self, pattern: &dyn core::ToInputArray, board_size: core::Size2f, output: &mut dyn core::ToOutputArray) -> Result<bool> {
        input_array_arg!(pattern);
        output_array_arg!(output);
        unsafe { sys::cv_ccalib_CustomPattern_create__InputArray_Size2f__OutputArray(self.as_raw_CustomPattern(), pattern.as_raw__InputArray(), board_size, output.as_raw__OutputArray()) }.into_result()
    }
    
    ///
    /// ## C++ default parameters
    /// * ratio: 0.7
    /// * proj_error: 8.0
    /// * refine_position: false
    /// * out: noArray()
    /// * h: noArray()
    /// * pattern_corners: noArray()
    pub fn find_pattern(&mut self, image: &dyn core::ToInputArray, matched_features: &mut dyn core::ToOutputArray, pattern_points: &mut dyn core::ToOutputArray, ratio: f64, proj_error: f64, refine_position: bool, out: &mut dyn core::ToOutputArray, h: &mut dyn core::ToOutputArray, pattern_corners: &mut dyn core::ToOutputArray) -> Result<bool> {
        input_array_arg!(image);
        output_array_arg!(matched_features);
        output_array_arg!(pattern_points);
        output_array_arg!(out);
        output_array_arg!(h);
        output_array_arg!(pattern_corners);
        unsafe { sys::cv_ccalib_CustomPattern_findPattern__InputArray__OutputArray__OutputArray_double_double_bool__OutputArray__OutputArray__OutputArray(self.as_raw_CustomPattern(), image.as_raw__InputArray(), matched_features.as_raw__OutputArray(), pattern_points.as_raw__OutputArray(), ratio, proj_error, refine_position, out.as_raw__OutputArray(), h.as_raw__OutputArray(), pattern_corners.as_raw__OutputArray()) }.into_result()
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
    
    /// <
    /// Get the pixel size of the pattern
    pub fn set_feature_detector(&mut self, feature_detector: &types::PtrOfFeature2D) -> Result<bool> {
        unsafe { sys::cv_ccalib_CustomPattern_setFeatureDetector_PtrOfFeature2D(self.as_raw_CustomPattern(), feature_detector.as_raw_PtrOfFeature2D()) }.into_result()
    }
    
    pub fn set_descriptor_extractor(&mut self, extractor: &types::PtrOfFeature2D) -> Result<bool> {
        unsafe { sys::cv_ccalib_CustomPattern_setDescriptorExtractor_PtrOfFeature2D(self.as_raw_CustomPattern(), extractor.as_raw_PtrOfFeature2D()) }.into_result()
    }
    
    pub fn set_descriptor_matcher(&mut self, matcher: &types::PtrOfDescriptorMatcher) -> Result<bool> {
        unsafe { sys::cv_ccalib_CustomPattern_setDescriptorMatcher_PtrOfDescriptorMatcher(self.as_raw_CustomPattern(), matcher.as_raw_PtrOfDescriptorMatcher()) }.into_result()
    }
    
    pub fn get_feature_detector(&mut self) -> Result<types::PtrOfFeature2D> {
        unsafe { sys::cv_ccalib_CustomPattern_getFeatureDetector(self.as_raw_CustomPattern()) }.into_result().map(|ptr| types::PtrOfFeature2D { ptr })
    }
    
    pub fn get_descriptor_extractor(&mut self) -> Result<types::PtrOfFeature2D> {
        unsafe { sys::cv_ccalib_CustomPattern_getDescriptorExtractor(self.as_raw_CustomPattern()) }.into_result().map(|ptr| types::PtrOfFeature2D { ptr })
    }
    
    pub fn get_descriptor_matcher(&mut self) -> Result<types::PtrOfDescriptorMatcher> {
        unsafe { sys::cv_ccalib_CustomPattern_getDescriptorMatcher(self.as_raw_CustomPattern()) }.into_result().map(|ptr| types::PtrOfDescriptorMatcher { ptr })
    }
    
    ///
    /// ## C++ default parameters
    /// * flags: 0
    /// * criteria: TermCriteria(TermCriteria::COUNT + TermCriteria::EPS, 30, DBL_EPSILON)
    pub fn calibrate(&mut self, object_points: &dyn core::ToInputArray, image_points: &dyn core::ToInputArray, image_size: core::Size, camera_matrix: &mut dyn core::ToInputOutputArray, dist_coeffs: &mut dyn core::ToInputOutputArray, rvecs: &mut dyn core::ToOutputArray, tvecs: &mut dyn core::ToOutputArray, flags: i32, criteria: &core::TermCriteria) -> Result<f64> {
        input_array_arg!(object_points);
        input_array_arg!(image_points);
        input_output_array_arg!(camera_matrix);
        input_output_array_arg!(dist_coeffs);
        output_array_arg!(rvecs);
        output_array_arg!(tvecs);
        unsafe { sys::cv_ccalib_CustomPattern_calibrate__InputArray__InputArray_Size__InputOutputArray__InputOutputArray__OutputArray__OutputArray_int_TermCriteria(self.as_raw_CustomPattern(), object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), image_size, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, criteria.as_raw_TermCriteria()) }.into_result()
    }
    
    /// <
    /// Calls the calirateCamera function with the same inputs.
    ///
    /// ## C++ default parameters
    /// * use_extrinsic_guess: false
    /// * flags: SOLVEPNP_ITERATIVE
    pub fn find_rt(&mut self, object_points: &dyn core::ToInputArray, image_points: &dyn core::ToInputArray, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, rvec: &mut dyn core::ToOutputArray, tvec: &mut dyn core::ToOutputArray, use_extrinsic_guess: bool, flags: i32) -> Result<bool> {
        input_array_arg!(object_points);
        input_array_arg!(image_points);
        input_array_arg!(camera_matrix);
        input_array_arg!(dist_coeffs);
        output_array_arg!(rvec);
        output_array_arg!(tvec);
        unsafe { sys::cv_ccalib_CustomPattern_findRt__InputArray__InputArray__InputArray__InputArray__OutputArray__OutputArray_bool_int(self.as_raw_CustomPattern(), object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), use_extrinsic_guess, flags) }.into_result()
    }
    
    ///
    /// ## C++ default parameters
    /// * use_extrinsic_guess: false
    /// * flags: SOLVEPNP_ITERATIVE
    pub fn find_rt_1(&mut self, image: &dyn core::ToInputArray, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, rvec: &mut dyn core::ToOutputArray, tvec: &mut dyn core::ToOutputArray, use_extrinsic_guess: bool, flags: i32) -> Result<bool> {
        input_array_arg!(image);
        input_array_arg!(camera_matrix);
        input_array_arg!(dist_coeffs);
        output_array_arg!(rvec);
        output_array_arg!(tvec);
        unsafe { sys::cv_ccalib_CustomPattern_findRt__InputArray__InputArray__InputArray__OutputArray__OutputArray_bool_int(self.as_raw_CustomPattern(), image.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), use_extrinsic_guess, flags) }.into_result()
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
    pub fn find_rt_ransac(&mut self, object_points: &dyn core::ToInputArray, image_points: &dyn core::ToInputArray, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, rvec: &mut dyn core::ToOutputArray, tvec: &mut dyn core::ToOutputArray, use_extrinsic_guess: bool, iterations_count: i32, reprojection_error: f32, min_inliers_count: i32, inliers: &mut dyn core::ToOutputArray, flags: i32) -> Result<bool> {
        input_array_arg!(object_points);
        input_array_arg!(image_points);
        input_array_arg!(camera_matrix);
        input_array_arg!(dist_coeffs);
        output_array_arg!(rvec);
        output_array_arg!(tvec);
        output_array_arg!(inliers);
        unsafe { sys::cv_ccalib_CustomPattern_findRtRANSAC__InputArray__InputArray__InputArray__InputArray__OutputArray__OutputArray_bool_int_float_int__OutputArray_int(self.as_raw_CustomPattern(), object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), use_extrinsic_guess, iterations_count, reprojection_error, min_inliers_count, inliers.as_raw__OutputArray(), flags) }.into_result()
    }
    
    ///
    /// ## C++ default parameters
    /// * use_extrinsic_guess: false
    /// * iterations_count: 100
    /// * reprojection_error: 8.0
    /// * min_inliers_count: 100
    /// * inliers: noArray()
    /// * flags: SOLVEPNP_ITERATIVE
    pub fn find_rt_ransac_1(&mut self, image: &dyn core::ToInputArray, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, rvec: &mut dyn core::ToOutputArray, tvec: &mut dyn core::ToOutputArray, use_extrinsic_guess: bool, iterations_count: i32, reprojection_error: f32, min_inliers_count: i32, inliers: &mut dyn core::ToOutputArray, flags: i32) -> Result<bool> {
        input_array_arg!(image);
        input_array_arg!(camera_matrix);
        input_array_arg!(dist_coeffs);
        output_array_arg!(rvec);
        output_array_arg!(tvec);
        output_array_arg!(inliers);
        unsafe { sys::cv_ccalib_CustomPattern_findRtRANSAC__InputArray__InputArray__InputArray__OutputArray__OutputArray_bool_int_float_int__OutputArray_int(self.as_raw_CustomPattern(), image.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), use_extrinsic_guess, iterations_count, reprojection_error, min_inliers_count, inliers.as_raw__OutputArray(), flags) }.into_result()
    }
    
    /// <
    /// Uses solvePnPRansac()
    ///
    /// ## C++ default parameters
    /// * axis_length: 3
    /// * axis_width: 2
    pub fn draw_orientation(&mut self, image: &mut dyn core::ToInputOutputArray, tvec: &dyn core::ToInputArray, rvec: &dyn core::ToInputArray, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, axis_length: f64, axis_width: i32) -> Result<()> {
        input_output_array_arg!(image);
        input_array_arg!(tvec);
        input_array_arg!(rvec);
        input_array_arg!(camera_matrix);
        input_array_arg!(dist_coeffs);
        unsafe { sys::cv_ccalib_CustomPattern_drawOrientation__InputOutputArray__InputArray__InputArray__InputArray__InputArray_double_int(self.as_raw_CustomPattern(), image.as_raw__InputOutputArray(), tvec.as_raw__InputArray(), rvec.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), axis_length, axis_width) }.into_result()
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
pub struct MultiCameraCalibration {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for MultiCameraCalibration {
    fn drop(&mut self) {
        unsafe { sys::cv_MultiCameraCalibration_delete(self.ptr) };
    }
}

impl MultiCameraCalibration {
    #[inline(always)] pub fn as_raw_MultiCameraCalibration(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for MultiCameraCalibration {}

impl MultiCameraCalibration {
    ///
    /// ## C++ default parameters
    /// * verbose: 0
    /// * show_extration: 0
    /// * n_mini_matches: 20
    /// * flags: 0
    /// * criteria: TermCriteria(TermCriteria::COUNT + TermCriteria::EPS, 200, 1e-7)
    /// * detector: AKAZE::create(AKAZE::DESCRIPTOR_MLDB, 0, 3, 0.006f)
    /// * descriptor: AKAZE::create(AKAZE::DESCRIPTOR_MLDB,0, 3, 0.006f)
    /// * matcher: DescriptorMatcher::create("BruteForce-L1" )
    pub fn new(camera_type: i32, n_cameras: i32, file_name: &str, pattern_width: f32, pattern_height: f32, verbose: i32, show_extration: i32, n_mini_matches: i32, flags: i32, criteria: &core::TermCriteria, detector: &types::PtrOfFeature2D, descriptor: &types::PtrOfFeature2D, matcher: &types::PtrOfDescriptorMatcher) -> Result<crate::ccalib::MultiCameraCalibration> {
        string_arg!(file_name);
        unsafe { sys::cv_multicalib_MultiCameraCalibration_MultiCameraCalibration_int_int_std_string_float_float_int_int_int_int_TermCriteria_PtrOfFeature2D_PtrOfFeature2D_PtrOfDescriptorMatcher(camera_type, n_cameras, file_name.as_ptr(), pattern_width, pattern_height, verbose, show_extration, n_mini_matches, flags, criteria.as_raw_TermCriteria(), detector.as_raw_PtrOfFeature2D(), descriptor.as_raw_PtrOfFeature2D(), matcher.as_raw_PtrOfDescriptorMatcher()) }.into_result().map(|ptr| crate::ccalib::MultiCameraCalibration { ptr })
    }
    
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
pub struct MultiCameraCalibration_edge {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for MultiCameraCalibration_edge {
    fn drop(&mut self) {
        unsafe { sys::cv_MultiCameraCalibration_edge_delete(self.ptr) };
    }
}

impl MultiCameraCalibration_edge {
    #[inline(always)] pub fn as_raw_MultiCameraCalibration_edge(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for MultiCameraCalibration_edge {}

impl MultiCameraCalibration_edge {
    pub fn new(cv: i32, pv: i32, pi: i32, trans: &core::Mat) -> Result<crate::ccalib::MultiCameraCalibration_edge> {
        unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_edge_int_int_int_Mat(cv, pv, pi, trans.as_raw_Mat()) }.into_result().map(|ptr| crate::ccalib::MultiCameraCalibration_edge { ptr })
    }
    
}

// boxed class cv::multicalib::MultiCameraCalibration::vertex
pub struct MultiCameraCalibration_vertex {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for MultiCameraCalibration_vertex {
    fn drop(&mut self) {
        unsafe { sys::cv_MultiCameraCalibration_vertex_delete(self.ptr) };
    }
}

impl MultiCameraCalibration_vertex {
    #[inline(always)] pub fn as_raw_MultiCameraCalibration_vertex(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for MultiCameraCalibration_vertex {}

impl MultiCameraCalibration_vertex {
    pub fn new(po: &core::Mat, ts: i32) -> Result<crate::ccalib::MultiCameraCalibration_vertex> {
        unsafe { sys::cv_multicalib_MultiCameraCalibration_vertex_vertex_Mat_int(po.as_raw_Mat(), ts) }.into_result().map(|ptr| crate::ccalib::MultiCameraCalibration_vertex { ptr })
    }
    
    pub fn default() -> Result<crate::ccalib::MultiCameraCalibration_vertex> {
        unsafe { sys::cv_multicalib_MultiCameraCalibration_vertex_vertex() }.into_result().map(|ptr| crate::ccalib::MultiCameraCalibration_vertex { ptr })
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
pub struct RandomPatternCornerFinder {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for RandomPatternCornerFinder {
    fn drop(&mut self) {
        unsafe { sys::cv_RandomPatternCornerFinder_delete(self.ptr) };
    }
}

impl RandomPatternCornerFinder {
    #[inline(always)] pub fn as_raw_RandomPatternCornerFinder(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for RandomPatternCornerFinder {}

impl RandomPatternCornerFinder {
    ///
    /// ## C++ default parameters
    /// * nmini_match: 20
    /// * depth: CV_32F
    /// * verbose: 0
    /// * show_extraction: 0
    /// * detector: AKAZE::create(AKAZE::DESCRIPTOR_MLDB, 0, 3, 0.005f)
    /// * descriptor: AKAZE::create(AKAZE::DESCRIPTOR_MLDB,0, 3, 0.005f)
    /// * matcher: DescriptorMatcher::create("BruteForce-L1" )
    pub fn new(pattern_width: f32, pattern_height: f32, nmini_match: i32, depth: i32, verbose: i32, show_extraction: i32, detector: &types::PtrOfFeature2D, descriptor: &types::PtrOfFeature2D, matcher: &types::PtrOfDescriptorMatcher) -> Result<crate::ccalib::RandomPatternCornerFinder> {
        unsafe { sys::cv_randpattern_RandomPatternCornerFinder_RandomPatternCornerFinder_float_float_int_int_int_int_PtrOfFeature2D_PtrOfFeature2D_PtrOfDescriptorMatcher(pattern_width, pattern_height, nmini_match, depth, verbose, show_extraction, detector.as_raw_PtrOfFeature2D(), descriptor.as_raw_PtrOfFeature2D(), matcher.as_raw_PtrOfDescriptorMatcher()) }.into_result().map(|ptr| crate::ccalib::RandomPatternCornerFinder { ptr })
    }
    
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
        unsafe { sys::cv_randpattern_RandomPatternCornerFinder_computeObjectImagePointsForSingle_Mat(self.as_raw_RandomPatternCornerFinder(), input_image.as_raw_Mat()) }.into_result().map(|ptr| types::VectorOfMat { ptr })
    }
    
    pub fn get_object_points(&mut self) -> Result<types::VectorOfMat> {
        unsafe { sys::cv_randpattern_RandomPatternCornerFinder_getObjectPoints(self.as_raw_RandomPatternCornerFinder()) }.into_result().map(|ptr| types::VectorOfMat { ptr })
    }
    
    pub fn get_image_points(&mut self) -> Result<types::VectorOfMat> {
        unsafe { sys::cv_randpattern_RandomPatternCornerFinder_getImagePoints(self.as_raw_RandomPatternCornerFinder()) }.into_result().map(|ptr| types::VectorOfMat { ptr })
    }
    
}

// boxed class cv::randpattern::RandomPatternGenerator
pub struct RandomPatternGenerator {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for RandomPatternGenerator {
    fn drop(&mut self) {
        unsafe { sys::cv_RandomPatternGenerator_delete(self.ptr) };
    }
}

impl RandomPatternGenerator {
    #[inline(always)] pub fn as_raw_RandomPatternGenerator(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for RandomPatternGenerator {}

impl RandomPatternGenerator {
    pub fn new(image_width: i32, image_height: i32) -> Result<crate::ccalib::RandomPatternGenerator> {
        unsafe { sys::cv_randpattern_RandomPatternGenerator_RandomPatternGenerator_int_int(image_width, image_height) }.into_result().map(|ptr| crate::ccalib::RandomPatternGenerator { ptr })
    }
    
    pub fn generate_pattern(&mut self) -> Result<()> {
        unsafe { sys::cv_randpattern_RandomPatternGenerator_generatePattern(self.as_raw_RandomPatternGenerator()) }.into_result()
    }
    
    pub fn get_pattern(&mut self) -> Result<core::Mat> {
        unsafe { sys::cv_randpattern_RandomPatternGenerator_getPattern(self.as_raw_RandomPatternGenerator()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
}

