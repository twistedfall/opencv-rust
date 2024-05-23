//! # Custom Calibration Pattern for 3D reconstruction
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{CustomPatternTrait, CustomPatternTraitConst, MultiCameraCalibrationTrait, MultiCameraCalibrationTraitConst, MultiCameraCalibration_edgeTrait, MultiCameraCalibration_edgeTraitConst, MultiCameraCalibration_vertexTrait, MultiCameraCalibration_vertexTraitConst, RandomPatternCornerFinderTrait, RandomPatternCornerFinderTraitConst, RandomPatternGeneratorTrait, RandomPatternGeneratorTraitConst};
}

// CALIB_FIX_CENTER /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:65
pub const CALIB_FIX_CENTER: i32 = 256;
// CALIB_FIX_GAMMA /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:64
pub const CALIB_FIX_GAMMA: i32 = 128;
// CALIB_FIX_K1 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:59
pub const CALIB_FIX_K1: i32 = 4;
// CALIB_FIX_K2 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:60
pub const CALIB_FIX_K2: i32 = 8;
// CALIB_FIX_P1 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:61
pub const CALIB_FIX_P1: i32 = 16;
// CALIB_FIX_P2 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:62
pub const CALIB_FIX_P2: i32 = 32;
// CALIB_FIX_SKEW /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:58
pub const CALIB_FIX_SKEW: i32 = 2;
// CALIB_FIX_XI /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:63
pub const CALIB_FIX_XI: i32 = 64;
// CALIB_USE_GUESS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:57
pub const CALIB_USE_GUESS: i32 = 1;
// HEAD /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:55
pub const HEAD: i32 = -1;
// INVALID /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:56
pub const INVALID: i32 = -2;
// OMNIDIRECTIONAL /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:77
pub const MultiCameraCalibration_OMNIDIRECTIONAL: i32 = 1;
// PINHOLE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:76
pub const MultiCameraCalibration_PINHOLE: i32 = 0;
// RECTIFY_CYLINDRICAL /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:70
pub const RECTIFY_CYLINDRICAL: i32 = 2;
// RECTIFY_LONGLATI /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:71
pub const RECTIFY_LONGLATI: i32 = 3;
// RECTIFY_PERSPECTIVE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:69
pub const RECTIFY_PERSPECTIVE: i32 = 1;
// RECTIFY_STEREOGRAPHIC /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:72
pub const RECTIFY_STEREOGRAPHIC: i32 = 4;
// XYZ /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:77
pub const XYZ: i32 = 2;
// XYZRGB /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:76
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
/// * D: Output distortion parameters ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%29)
/// * rvecs: Output rotations for each calibration images
/// * tvecs: Output translation for each calibration images
/// * flags: The flags that control calibrate
/// * criteria: Termination criteria for optimization
/// * idx: Indices of images that pass initialization, which are really used in calibration. So the size of rvecs is the
/// same as idx.total().
///
/// ## Note
/// This alternative version of [calibrate] function uses the following default values for its arguments:
/// * idx: noArray()
// cv::omnidir::calibrate(InputArray, InputArray, SimpleClass, InputOutputArray, InputOutputArray, InputOutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:176
// ("cv::omnidir::calibrate", vec![(pred!(mut, ["objectPoints", "imagePoints", "size", "K", "xi", "D", "rvecs", "tvecs", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
#[inline]
pub fn calibrate_def(object_points: &impl ToInputArray, image_points: &impl ToInputArray, size: core::Size, k: &mut impl ToInputOutputArray, xi: &mut impl ToInputOutputArray, d: &mut impl ToInputOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
	input_array_arg!(object_points);
	input_array_arg!(image_points);
	input_output_array_arg!(k);
	input_output_array_arg!(xi);
	input_output_array_arg!(d);
	output_array_arg!(rvecs);
	output_array_arg!(tvecs);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_omnidir_calibrate_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), &size, k.as_raw__InputOutputArray(), xi.as_raw__InputOutputArray(), d.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

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
/// * D: Output distortion parameters ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%29)
/// * rvecs: Output rotations for each calibration images
/// * tvecs: Output translation for each calibration images
/// * flags: The flags that control calibrate
/// * criteria: Termination criteria for optimization
/// * idx: Indices of images that pass initialization, which are really used in calibration. So the size of rvecs is the
/// same as idx.total().
///
/// ## C++ default parameters
/// * idx: noArray()
// calibrate(InputArrayOfArrays, InputArrayOfArrays, Size, InputOutputArray, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, int, TermCriteria, OutputArray)(InputArray, InputArray, SimpleClass, InputOutputArray, InputOutputArray, InputOutputArray, OutputArray, OutputArray, Primitive, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:176
// ("cv::omnidir::calibrate", vec![(pred!(mut, ["objectPoints", "imagePoints", "size", "K", "xi", "D", "rvecs", "tvecs", "flags", "criteria", "idx"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn calibrate(object_points: &impl ToInputArray, image_points: &impl ToInputArray, size: core::Size, k: &mut impl ToInputOutputArray, xi: &mut impl ToInputOutputArray, d: &mut impl ToInputOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria, idx: &mut impl ToOutputArray) -> Result<f64> {
	input_array_arg!(object_points);
	input_array_arg!(image_points);
	input_output_array_arg!(k);
	input_output_array_arg!(xi);
	input_output_array_arg!(d);
	output_array_arg!(rvecs);
	output_array_arg!(tvecs);
	output_array_arg!(idx);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_omnidir_calibrate_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), &size, k.as_raw__InputOutputArray(), xi.as_raw__InputOutputArray(), d.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, &criteria, idx.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes undistortion and rectification maps for omnidirectional camera image transform by a rotation R.
/// It output two maps that are used for cv::remap(). If D is empty then zero distortion is used,
/// if R or P is empty then identity matrices are used.
///
/// ## Parameters
/// * K: Camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%20s%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%20%5F1%20%5Cend%7Bbmatrix%7D), with depth CV_32F or CV_64F
/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%29), with depth CV_32F or CV_64F
/// * xi: The parameter xi for CMei's model
/// * R: Rotation transform between the original and object space : 3x3 1-channel, or vector: 3x1/1x3, with depth CV_32F or CV_64F
/// * P: New camera matrix (3x3) or new projection matrix (3x4)
/// * size: Undistorted image size.
/// * m1type: Type of the first output map that can be CV_32FC1 or CV_16SC2 . See convertMaps()
/// for details.
/// * map1: The first output map.
/// * map2: The second output map.
/// * flags: Flags indicates the rectification type,  RECTIFY_PERSPECTIVE, RECTIFY_CYLINDRICAL, RECTIFY_LONGLATI and RECTIFY_STEREOGRAPHIC
/// are supported.
// initUndistortRectifyMap(InputArray, InputArray, InputArray, InputArray, InputArray, const cv::Size &, int, OutputArray, OutputArray, int)(InputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass, Primitive, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:141
// ("cv::omnidir::initUndistortRectifyMap", vec![(pred!(mut, ["K", "D", "xi", "R", "P", "size", "m1type", "map1", "map2", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "int", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
#[inline]
pub fn init_undistort_rectify_map(k: &impl ToInputArray, d: &impl ToInputArray, xi: &impl ToInputArray, r: &impl ToInputArray, p: &impl ToInputArray, size: core::Size, m1type: i32, map1: &mut impl ToOutputArray, map2: &mut impl ToOutputArray, flags: i32) -> Result<()> {
	input_array_arg!(k);
	input_array_arg!(d);
	input_array_arg!(xi);
	input_array_arg!(r);
	input_array_arg!(p);
	output_array_arg!(map1);
	output_array_arg!(map2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_omnidir_initUndistortRectifyMap_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_int_const__OutputArrayR_const__OutputArrayR_int(k.as_raw__InputArray(), d.as_raw__InputArray(), xi.as_raw__InputArray(), r.as_raw__InputArray(), p.as_raw__InputArray(), &size, m1type, map1.as_raw__OutputArray(), map2.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// @overload
///
/// ## Note
/// This alternative version of [project_points_1] function uses the following default values for its arguments:
/// * jacobian: noArray()
// cv::omnidir::projectPoints(InputArray, OutputArray, SimpleClass, InputArray, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:107
// ("cv::omnidir::projectPoints", vec![(pred!(mut, ["objectPoints", "imagePoints", "affine", "K", "xi", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::Affine3d*", "const cv::_InputArray*", "double", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn project_points_1_def(object_points: &impl ToInputArray, image_points: &mut impl ToOutputArray, affine: core::Affine3d, k: &impl ToInputArray, xi: f64, d: &impl ToInputArray) -> Result<()> {
	input_array_arg!(object_points);
	output_array_arg!(image_points);
	input_array_arg!(k);
	input_array_arg!(d);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_omnidir_projectPoints_const__InputArrayR_const__OutputArrayR_const_Affine3dR_const__InputArrayR_double_const__InputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__OutputArray(), &affine, k.as_raw__InputArray(), xi, d.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
/// * K: Camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%20s%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%20%5F1%20%5Cend%7Bbmatrix%7D).
/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%29).
/// * xi: The parameter xi for CMei's model
/// * jacobian: Optional output 2Nx16 of type CV_64F jacobian matrix, contains the derivatives of
/// image pixel points wrt parameters including ![inline formula](https://latex.codecogs.com/png.latex?om%2C%20T%2C%20f%5Fx%2C%20f%5Fy%2C%20s%2C%20c%5Fx%2C%20c%5Fy%2C%20xi%2C%20k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2).
/// This matrix will be used in calibration by optimization.
///
/// The function projects object 3D points of world coordinate to image pixels, parameter by intrinsic
/// and extrinsic parameters. Also, it optionally compute a by-product: the jacobian matrix containing
/// contains the derivatives of image pixel points wrt intrinsic and extrinsic parameters.
///
/// ## Overloaded parameters
///
/// ## C++ default parameters
/// * jacobian: noArray()
// projectPoints(InputArray, OutputArray, const Affine3d &, InputArray, double, InputArray, OutputArray)(InputArray, OutputArray, SimpleClass, InputArray, Primitive, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:107
// ("cv::omnidir::projectPoints", vec![(pred!(mut, ["objectPoints", "imagePoints", "affine", "K", "xi", "D", "jacobian"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::Affine3d*", "const cv::_InputArray*", "double", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn project_points_1(object_points: &impl ToInputArray, image_points: &mut impl ToOutputArray, affine: core::Affine3d, k: &impl ToInputArray, xi: f64, d: &impl ToInputArray, jacobian: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(object_points);
	output_array_arg!(image_points);
	input_array_arg!(k);
	input_array_arg!(d);
	output_array_arg!(jacobian);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_omnidir_projectPoints_const__InputArrayR_const__OutputArrayR_const_Affine3dR_const__InputArrayR_double_const__InputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__OutputArray(), &affine, k.as_raw__InputArray(), xi, d.as_raw__InputArray(), jacobian.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
/// * K: Camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%20s%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%20%5F1%20%5Cend%7Bbmatrix%7D).
/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%29).
/// * xi: The parameter xi for CMei's model
/// * jacobian: Optional output 2Nx16 of type CV_64F jacobian matrix, contains the derivatives of
/// image pixel points wrt parameters including ![inline formula](https://latex.codecogs.com/png.latex?om%2C%20T%2C%20f%5Fx%2C%20f%5Fy%2C%20s%2C%20c%5Fx%2C%20c%5Fy%2C%20xi%2C%20k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2).
/// This matrix will be used in calibration by optimization.
///
/// The function projects object 3D points of world coordinate to image pixels, parameter by intrinsic
/// and extrinsic parameters. Also, it optionally compute a by-product: the jacobian matrix containing
/// contains the derivatives of image pixel points wrt intrinsic and extrinsic parameters.
///
/// ## Note
/// This alternative version of [project_points] function uses the following default values for its arguments:
/// * jacobian: noArray()
// cv::omnidir::projectPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:103
// ("cv::omnidir::projectPoints", vec![(pred!(mut, ["objectPoints", "imagePoints", "rvec", "tvec", "K", "xi", "D"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn project_points_def(object_points: &impl ToInputArray, image_points: &mut impl ToOutputArray, rvec: &impl ToInputArray, tvec: &impl ToInputArray, k: &impl ToInputArray, xi: f64, d: &impl ToInputArray) -> Result<()> {
	input_array_arg!(object_points);
	output_array_arg!(image_points);
	input_array_arg!(rvec);
	input_array_arg!(tvec);
	input_array_arg!(k);
	input_array_arg!(d);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_omnidir_projectPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_const__InputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__OutputArray(), rvec.as_raw__InputArray(), tvec.as_raw__InputArray(), k.as_raw__InputArray(), xi, d.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
/// * K: Camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%20s%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%20%5F1%20%5Cend%7Bbmatrix%7D).
/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%29).
/// * xi: The parameter xi for CMei's model
/// * jacobian: Optional output 2Nx16 of type CV_64F jacobian matrix, contains the derivatives of
/// image pixel points wrt parameters including ![inline formula](https://latex.codecogs.com/png.latex?om%2C%20T%2C%20f%5Fx%2C%20f%5Fy%2C%20s%2C%20c%5Fx%2C%20c%5Fy%2C%20xi%2C%20k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2).
/// This matrix will be used in calibration by optimization.
///
/// The function projects object 3D points of world coordinate to image pixels, parameter by intrinsic
/// and extrinsic parameters. Also, it optionally compute a by-product: the jacobian matrix containing
/// contains the derivatives of image pixel points wrt intrinsic and extrinsic parameters.
///
/// ## C++ default parameters
/// * jacobian: noArray()
// projectPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, double, InputArray, OutputArray)(InputArray, OutputArray, InputArray, InputArray, InputArray, Primitive, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:103
// ("cv::omnidir::projectPoints", vec![(pred!(mut, ["objectPoints", "imagePoints", "rvec", "tvec", "K", "xi", "D", "jacobian"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "double", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn project_points(object_points: &impl ToInputArray, image_points: &mut impl ToOutputArray, rvec: &impl ToInputArray, tvec: &impl ToInputArray, k: &impl ToInputArray, xi: f64, d: &impl ToInputArray, jacobian: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(object_points);
	output_array_arg!(image_points);
	input_array_arg!(rvec);
	input_array_arg!(tvec);
	input_array_arg!(k);
	input_array_arg!(d);
	output_array_arg!(jacobian);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_omnidir_projectPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_const__InputArrayR_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__OutputArray(), rvec.as_raw__InputArray(), tvec.as_raw__InputArray(), k.as_raw__InputArray(), xi, d.as_raw__InputArray(), jacobian.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
/// * D1: Output distortion parameters ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%29) for the first camera
/// * K2: Output camera matrix for the first camera.
/// * xi2: Output parameter xi of CMei's model for the second camera
/// * D2: Output distortion parameters ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%29) for the second camera
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
/// ## Note
/// This alternative version of [stereo_calibrate] function uses the following default values for its arguments:
/// * idx: noArray()
// cv::omnidir::stereoCalibrate(InputOutputArray, InputOutputArray, InputOutputArray, SimpleClass, SimpleClass, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:207
// ("cv::omnidir::stereoCalibrate", vec![(pred!(mut, ["objectPoints", "imagePoints1", "imagePoints2", "imageSize1", "imageSize2", "K1", "xi1", "D1", "K2", "xi2", "D2", "rvec", "tvec", "rvecsL", "tvecsL", "flags", "criteria"], ["const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::Size*", "const cv::Size*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
#[inline]
pub fn stereo_calibrate_def(object_points: &mut impl ToInputOutputArray, image_points1: &mut impl ToInputOutputArray, image_points2: &mut impl ToInputOutputArray, image_size1: core::Size, image_size2: core::Size, k1: &mut impl ToInputOutputArray, xi1: &mut impl ToInputOutputArray, d1: &mut impl ToInputOutputArray, k2: &mut impl ToInputOutputArray, xi2: &mut impl ToInputOutputArray, d2: &mut impl ToInputOutputArray, rvec: &mut impl ToOutputArray, tvec: &mut impl ToOutputArray, rvecs_l: &mut impl ToOutputArray, tvecs_l: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
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
	return_send!(via ocvrs_return);
	unsafe { sys::cv_omnidir_stereoCalibrate_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const_SizeR_const_SizeR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(object_points.as_raw__InputOutputArray(), image_points1.as_raw__InputOutputArray(), image_points2.as_raw__InputOutputArray(), &image_size1, &image_size2, k1.as_raw__InputOutputArray(), xi1.as_raw__InputOutputArray(), d1.as_raw__InputOutputArray(), k2.as_raw__InputOutputArray(), xi2.as_raw__InputOutputArray(), d2.as_raw__InputOutputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), rvecs_l.as_raw__OutputArray(), tvecs_l.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
/// * D1: Output distortion parameters ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%29) for the first camera
/// * K2: Output camera matrix for the first camera.
/// * xi2: Output parameter xi of CMei's model for the second camera
/// * D2: Output distortion parameters ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%29) for the second camera
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
// stereoCalibrate(InputOutputArrayOfArrays, InputOutputArrayOfArrays, InputOutputArrayOfArrays, const Size &, const Size &, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArrayOfArrays, OutputArrayOfArrays, int, TermCriteria, OutputArray)(InputOutputArray, InputOutputArray, InputOutputArray, SimpleClass, SimpleClass, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:207
// ("cv::omnidir::stereoCalibrate", vec![(pred!(mut, ["objectPoints", "imagePoints1", "imagePoints2", "imageSize1", "imageSize2", "K1", "xi1", "D1", "K2", "xi2", "D2", "rvec", "tvec", "rvecsL", "tvecsL", "flags", "criteria", "idx"], ["const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::Size*", "const cv::Size*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn stereo_calibrate(object_points: &mut impl ToInputOutputArray, image_points1: &mut impl ToInputOutputArray, image_points2: &mut impl ToInputOutputArray, image_size1: core::Size, image_size2: core::Size, k1: &mut impl ToInputOutputArray, xi1: &mut impl ToInputOutputArray, d1: &mut impl ToInputOutputArray, k2: &mut impl ToInputOutputArray, xi2: &mut impl ToInputOutputArray, d2: &mut impl ToInputOutputArray, rvec: &mut impl ToOutputArray, tvec: &mut impl ToOutputArray, rvecs_l: &mut impl ToOutputArray, tvecs_l: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria, idx: &mut impl ToOutputArray) -> Result<f64> {
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
	return_send!(via ocvrs_return);
	unsafe { sys::cv_omnidir_stereoCalibrate_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const_SizeR_const_SizeR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria_const__OutputArrayR(object_points.as_raw__InputOutputArray(), image_points1.as_raw__InputOutputArray(), image_points2.as_raw__InputOutputArray(), &image_size1, &image_size2, k1.as_raw__InputOutputArray(), xi1.as_raw__InputOutputArray(), d1.as_raw__InputOutputArray(), k2.as_raw__InputOutputArray(), xi2.as_raw__InputOutputArray(), d2.as_raw__InputOutputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), rvecs_l.as_raw__OutputArray(), tvecs_l.as_raw__OutputArray(), flags, &criteria, idx.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Stereo 3D reconstruction from a pair of images
///
/// ## Parameters
/// * image1: The first input image
/// * image2: The second input image
/// * K1: Input camera matrix of the first camera
/// * D1: Input distortion parameters ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%29) for the first camera
/// * xi1: Input parameter xi for the first camera for CMei's model
/// * K2: Input camera matrix of the second camera
/// * D2: Input distortion parameters ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%29) for the second camera
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
/// ## Note
/// This alternative version of [stereo_reconstruct] function uses the following default values for its arguments:
/// * new_size: Size()
/// * knew: cv::noArray()
/// * point_cloud: cv::noArray()
/// * point_type: XYZRGB
// cv::omnidir::stereoReconstruct(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, Primitive, Primitive, Primitive, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:243
// ("cv::omnidir::stereoReconstruct", vec![(pred!(mut, ["image1", "image2", "K1", "D1", "xi1", "K2", "D2", "xi2", "R", "T", "flag", "numDisparities", "SADWindowSize", "disparity", "image1Rec", "image2Rec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "int", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn stereo_reconstruct_def(image1: &impl ToInputArray, image2: &impl ToInputArray, k1: &impl ToInputArray, d1: &impl ToInputArray, xi1: &impl ToInputArray, k2: &impl ToInputArray, d2: &impl ToInputArray, xi2: &impl ToInputArray, r: &impl ToInputArray, t: &impl ToInputArray, flag: i32, num_disparities: i32, sad_window_size: i32, disparity: &mut impl ToOutputArray, image1_rec: &mut impl ToOutputArray, image2_rec: &mut impl ToOutputArray) -> Result<()> {
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
	return_send!(via ocvrs_return);
	unsafe { sys::cv_omnidir_stereoReconstruct_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_int_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(image1.as_raw__InputArray(), image2.as_raw__InputArray(), k1.as_raw__InputArray(), d1.as_raw__InputArray(), xi1.as_raw__InputArray(), k2.as_raw__InputArray(), d2.as_raw__InputArray(), xi2.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), flag, num_disparities, sad_window_size, disparity.as_raw__OutputArray(), image1_rec.as_raw__OutputArray(), image2_rec.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Stereo 3D reconstruction from a pair of images
///
/// ## Parameters
/// * image1: The first input image
/// * image2: The second input image
/// * K1: Input camera matrix of the first camera
/// * D1: Input distortion parameters ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%29) for the first camera
/// * xi1: Input parameter xi for the first camera for CMei's model
/// * K2: Input camera matrix of the second camera
/// * D2: Input distortion parameters ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%29) for the second camera
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
// stereoReconstruct(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, int, int, int, OutputArray, OutputArray, OutputArray, const Size &, InputArray, OutputArray, int)(InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, InputArray, Primitive, Primitive, Primitive, OutputArray, OutputArray, OutputArray, SimpleClass, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:243
// ("cv::omnidir::stereoReconstruct", vec![(pred!(mut, ["image1", "image2", "K1", "D1", "xi1", "K2", "D2", "xi2", "R", "T", "flag", "numDisparities", "SADWindowSize", "disparity", "image1Rec", "image2Rec", "newSize", "Knew", "pointCloud", "pointType"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "int", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::Size*", "const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
#[inline]
pub fn stereo_reconstruct(image1: &impl ToInputArray, image2: &impl ToInputArray, k1: &impl ToInputArray, d1: &impl ToInputArray, xi1: &impl ToInputArray, k2: &impl ToInputArray, d2: &impl ToInputArray, xi2: &impl ToInputArray, r: &impl ToInputArray, t: &impl ToInputArray, flag: i32, num_disparities: i32, sad_window_size: i32, disparity: &mut impl ToOutputArray, image1_rec: &mut impl ToOutputArray, image2_rec: &mut impl ToOutputArray, new_size: core::Size, knew: &impl ToInputArray, point_cloud: &mut impl ToOutputArray, point_type: i32) -> Result<()> {
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
	return_send!(via ocvrs_return);
	unsafe { sys::cv_omnidir_stereoReconstruct_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_int_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const_SizeR_const__InputArrayR_const__OutputArrayR_int(image1.as_raw__InputArray(), image2.as_raw__InputArray(), k1.as_raw__InputArray(), d1.as_raw__InputArray(), xi1.as_raw__InputArray(), k2.as_raw__InputArray(), d2.as_raw__InputArray(), xi2.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), flag, num_disparities, sad_window_size, disparity.as_raw__OutputArray(), image1_rec.as_raw__OutputArray(), image2_rec.as_raw__OutputArray(), &new_size, knew.as_raw__InputArray(), point_cloud.as_raw__OutputArray(), point_type, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Stereo rectification for omnidirectional camera model. It computes the rectification rotations for two cameras
///
/// ## Parameters
/// * R: Rotation between the first and second camera
/// * T: Translation between the first and second camera
/// * R1: Output 3x3 rotation matrix for the first camera
/// * R2: Output 3x3 rotation matrix for the second camera
// stereoRectify(InputArray, InputArray, OutputArray, OutputArray)(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:218
// ("cv::omnidir::stereoRectify", vec![(pred!(mut, ["R", "T", "R1", "R2"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn stereo_rectify(r: &impl ToInputArray, t: &impl ToInputArray, r1: &mut impl ToOutputArray, r2: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(r);
	input_array_arg!(t);
	output_array_arg!(r1);
	output_array_arg!(r2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_omnidir_stereoRectify_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(r.as_raw__InputArray(), t.as_raw__InputArray(), r1.as_raw__OutputArray(), r2.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Undistort omnidirectional images to perspective images
///
/// ## Parameters
/// * distorted: The input omnidirectional image.
/// * undistorted: The output undistorted image.
/// * K: Camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%20s%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%20%5F1%20%5Cend%7Bbmatrix%7D).
/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%29).
/// * xi: The parameter xi for CMei's model.
/// * flags: Flags indicates the rectification type,  RECTIFY_PERSPECTIVE, RECTIFY_CYLINDRICAL, RECTIFY_LONGLATI and RECTIFY_STEREOGRAPHIC
/// * Knew: Camera matrix of the distorted image. If it is not assigned, it is just K.
/// * new_size: The new image size. By default, it is the size of distorted.
/// * R: Rotation matrix between the input and output images. By default, it is identity matrix.
///
/// ## Note
/// This alternative version of [undistort_image] function uses the following default values for its arguments:
/// * knew: cv::noArray()
/// * new_size: Size()
/// * r: Mat::eye(3,3,CV_64F)
// cv::omnidir::undistortImage(InputArray, OutputArray, InputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:156
// ("cv::omnidir::undistortImage", vec![(pred!(mut, ["distorted", "undistorted", "K", "D", "xi", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
#[inline]
pub fn undistort_image_def(distorted: &impl ToInputArray, undistorted: &mut impl ToOutputArray, k: &impl ToInputArray, d: &impl ToInputArray, xi: &impl ToInputArray, flags: i32) -> Result<()> {
	input_array_arg!(distorted);
	output_array_arg!(undistorted);
	input_array_arg!(k);
	input_array_arg!(d);
	input_array_arg!(xi);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_omnidir_undistortImage_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int(distorted.as_raw__InputArray(), undistorted.as_raw__OutputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), xi.as_raw__InputArray(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Undistort omnidirectional images to perspective images
///
/// ## Parameters
/// * distorted: The input omnidirectional image.
/// * undistorted: The output undistorted image.
/// * K: Camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%20s%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%20%5F1%20%5Cend%7Bbmatrix%7D).
/// * D: Input vector of distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%29).
/// * xi: The parameter xi for CMei's model.
/// * flags: Flags indicates the rectification type,  RECTIFY_PERSPECTIVE, RECTIFY_CYLINDRICAL, RECTIFY_LONGLATI and RECTIFY_STEREOGRAPHIC
/// * Knew: Camera matrix of the distorted image. If it is not assigned, it is just K.
/// * new_size: The new image size. By default, it is the size of distorted.
/// * R: Rotation matrix between the input and output images. By default, it is identity matrix.
///
/// ## C++ default parameters
/// * knew: cv::noArray()
/// * new_size: Size()
/// * r: Mat::eye(3,3,CV_64F)
// undistortImage(InputArray, OutputArray, InputArray, InputArray, InputArray, int, InputArray, const Size &, InputArray)(InputArray, OutputArray, InputArray, InputArray, InputArray, Primitive, InputArray, SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:156
// ("cv::omnidir::undistortImage", vec![(pred!(mut, ["distorted", "undistorted", "K", "D", "xi", "flags", "Knew", "new_size", "R"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "const cv::_InputArray*", "const cv::Size*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn undistort_image(distorted: &impl ToInputArray, undistorted: &mut impl ToOutputArray, k: &impl ToInputArray, d: &impl ToInputArray, xi: &impl ToInputArray, flags: i32, knew: &impl ToInputArray, new_size: core::Size, r: &impl ToInputArray) -> Result<()> {
	input_array_arg!(distorted);
	output_array_arg!(undistorted);
	input_array_arg!(k);
	input_array_arg!(d);
	input_array_arg!(xi);
	input_array_arg!(knew);
	input_array_arg!(r);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_omnidir_undistortImage_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_const__InputArrayR_const_SizeR_const__InputArrayR(distorted.as_raw__InputArray(), undistorted.as_raw__OutputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), xi.as_raw__InputArray(), flags, knew.as_raw__InputArray(), &new_size, r.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Undistort 2D image points for omnidirectional camera using CMei's model
///
/// ## Parameters
/// * distorted: Array of distorted image points, vector of Vec2f
/// or 1xN/Nx1 2-channel Mat of type CV_32F, 64F depth is also acceptable
/// * K: Camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%20s%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%20%5F1%20%5Cend%7Bbmatrix%7D).
/// * D: Distortion coefficients ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%29).
/// * xi: The parameter xi for CMei's model
/// * R: Rotation trainsform between the original and object space : 3x3 1-channel, or vector: 3x1/1x3
/// 1-channel or 1x1 3-channel
/// * undistorted: array of normalized object points, vector of Vec2f/Vec2d or 1xN/Nx1 2-channel Mat with the same
/// depth of distorted points.
// undistortPoints(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray)(InputArray, OutputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/omnidir.hpp:122
// ("cv::omnidir::undistortPoints", vec![(pred!(mut, ["distorted", "undistorted", "K", "D", "xi", "R"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn undistort_points(distorted: &impl ToInputArray, undistorted: &mut impl ToOutputArray, k: &impl ToInputArray, d: &impl ToInputArray, xi: &impl ToInputArray, r: &impl ToInputArray) -> Result<()> {
	input_array_arg!(distorted);
	output_array_arg!(undistorted);
	input_array_arg!(k);
	input_array_arg!(d);
	input_array_arg!(xi);
	input_array_arg!(r);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_omnidir_undistortPoints_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(distorted.as_raw__InputArray(), undistorted.as_raw__OutputArray(), k.as_raw__InputArray(), d.as_raw__InputArray(), xi.as_raw__InputArray(), r.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Constant methods for [crate::ccalib::CustomPattern]
// CustomPattern /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:60
pub trait CustomPatternTraitConst: core::AlgorithmTraitConst {
	fn as_raw_CustomPattern(&self) -> *const c_void;

}

/// Mutable methods for [crate::ccalib::CustomPattern]
pub trait CustomPatternTrait: core::AlgorithmTrait + crate::ccalib::CustomPatternTraitConst {
	fn as_raw_mut_CustomPattern(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * output: noArray()
	// create(InputArray, const Size2f, OutputArray)(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:66
	// ("cv::ccalib::CustomPattern::create", vec![(pred!(mut, ["pattern", "boardSize", "output"], ["const cv::_InputArray*", "const cv::Size2f", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn create(&mut self, pattern: &impl ToInputArray, board_size: core::Size2f, output: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(pattern);
		output_array_arg!(output);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_create_const__InputArrayR_const_Size2f_const__OutputArrayR(self.as_raw_mut_CustomPattern(), pattern.as_raw__InputArray(), &board_size, output.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [CustomPatternTrait::create] function uses the following default values for its arguments:
	/// * output: noArray()
	// cv::ccalib::CustomPattern::create(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:66
	// ("cv::ccalib::CustomPattern::create", vec![(pred!(mut, ["pattern", "boardSize"], ["const cv::_InputArray*", "const cv::Size2f"]), _)]),
	#[inline]
	fn create_def(&mut self, pattern: &impl ToInputArray, board_size: core::Size2f) -> Result<bool> {
		input_array_arg!(pattern);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_create_const__InputArrayR_const_Size2f(self.as_raw_mut_CustomPattern(), pattern.as_raw__InputArray(), &board_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * ratio: 0.7
	/// * proj_error: 8.0
	/// * refine_position: false
	/// * out: noArray()
	/// * h: noArray()
	/// * pattern_corners: noArray()
	// findPattern(InputArray, OutputArray, OutputArray, const double, const double, const bool, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, Primitive, Primitive, Primitive, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:68
	// ("cv::ccalib::CustomPattern::findPattern", vec![(pred!(mut, ["image", "matched_features", "pattern_points", "ratio", "proj_error", "refine_position", "out", "H", "pattern_corners"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const double", "const double", "const bool", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn find_pattern(&mut self, image: &impl ToInputArray, matched_features: &mut impl ToOutputArray, pattern_points: &mut impl ToOutputArray, ratio: f64, proj_error: f64, refine_position: bool, out: &mut impl ToOutputArray, h: &mut impl ToOutputArray, pattern_corners: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(matched_features);
		output_array_arg!(pattern_points);
		output_array_arg!(out);
		output_array_arg!(h);
		output_array_arg!(pattern_corners);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_findPattern_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const_double_const_double_const_bool_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_CustomPattern(), image.as_raw__InputArray(), matched_features.as_raw__OutputArray(), pattern_points.as_raw__OutputArray(), ratio, proj_error, refine_position, out.as_raw__OutputArray(), h.as_raw__OutputArray(), pattern_corners.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [CustomPatternTrait::find_pattern] function uses the following default values for its arguments:
	/// * ratio: 0.7
	/// * proj_error: 8.0
	/// * refine_position: false
	/// * out: noArray()
	/// * h: noArray()
	/// * pattern_corners: noArray()
	// cv::ccalib::CustomPattern::findPattern(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:68
	// ("cv::ccalib::CustomPattern::findPattern", vec![(pred!(mut, ["image", "matched_features", "pattern_points"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn find_pattern_def(&mut self, image: &impl ToInputArray, matched_features: &mut impl ToOutputArray, pattern_points: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(matched_features);
		output_array_arg!(pattern_points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_findPattern_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_CustomPattern(), image.as_raw__InputArray(), matched_features.as_raw__OutputArray(), pattern_points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// isInitialized()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:72
	// ("cv::ccalib::CustomPattern::isInitialized", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn is_initialized(&mut self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_isInitialized(self.as_raw_mut_CustomPattern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getPatternPoints(std::vector<KeyPoint> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:74
	// ("cv::ccalib::CustomPattern::getPatternPoints", vec![(pred!(mut, ["original_points"], ["std::vector<cv::KeyPoint>*"]), _)]),
	#[inline]
	fn get_pattern_points(&mut self, original_points: &mut core::Vector<core::KeyPoint>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_getPatternPoints_vectorLKeyPointGR(self.as_raw_mut_CustomPattern(), original_points.as_raw_mut_VectorOfKeyPoint(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getPixelSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:78
	// ("cv::ccalib::CustomPattern::getPixelSize", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_pixel_size(&mut self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_getPixelSize(self.as_raw_mut_CustomPattern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setFeatureDetector(Ptr<FeatureDetector>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:83
	// ("cv::ccalib::CustomPattern::setFeatureDetector", vec![(pred!(mut, ["featureDetector"], ["cv::Ptr<cv::Feature2D>"]), _)]),
	#[inline]
	fn set_feature_detector(&mut self, mut feature_detector: core::Ptr<crate::features::Feature2D>) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_setFeatureDetector_PtrLFeature2DG(self.as_raw_mut_CustomPattern(), feature_detector.as_raw_mut_PtrOfFeature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setDescriptorExtractor(Ptr<DescriptorExtractor>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:84
	// ("cv::ccalib::CustomPattern::setDescriptorExtractor", vec![(pred!(mut, ["extractor"], ["cv::Ptr<cv::Feature2D>"]), _)]),
	#[inline]
	fn set_descriptor_extractor(&mut self, mut extractor: core::Ptr<crate::features::Feature2D>) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_setDescriptorExtractor_PtrLFeature2DG(self.as_raw_mut_CustomPattern(), extractor.as_raw_mut_PtrOfFeature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setDescriptorMatcher(Ptr<DescriptorMatcher>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:85
	// ("cv::ccalib::CustomPattern::setDescriptorMatcher", vec![(pred!(mut, ["matcher"], ["cv::Ptr<cv::DescriptorMatcher>"]), _)]),
	#[inline]
	fn set_descriptor_matcher(&mut self, mut matcher: core::Ptr<crate::features::DescriptorMatcher>) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_setDescriptorMatcher_PtrLDescriptorMatcherG(self.as_raw_mut_CustomPattern(), matcher.as_raw_mut_PtrOfDescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getFeatureDetector()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:87
	// ("cv::ccalib::CustomPattern::getFeatureDetector", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_feature_detector(&mut self) -> Result<core::Ptr<crate::features::Feature2D>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_getFeatureDetector(self.as_raw_mut_CustomPattern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::Feature2D>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getDescriptorExtractor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:88
	// ("cv::ccalib::CustomPattern::getDescriptorExtractor", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_descriptor_extractor(&mut self) -> Result<core::Ptr<crate::features::Feature2D>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_getDescriptorExtractor(self.as_raw_mut_CustomPattern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::Feature2D>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getDescriptorMatcher()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:89
	// ("cv::ccalib::CustomPattern::getDescriptorMatcher", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_descriptor_matcher(&mut self) -> Result<core::Ptr<crate::features::DescriptorMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_getDescriptorMatcher(self.as_raw_mut_CustomPattern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::DescriptorMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
	// calibrate(InputArrayOfArrays, InputArrayOfArrays, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, int, TermCriteria)(InputArray, InputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:91
	// ("cv::ccalib::CustomPattern::calibrate", vec![(pred!(mut, ["objectPoints", "imagePoints", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
	#[inline]
	fn calibrate(&mut self, object_points: &impl ToInputArray, image_points: &impl ToInputArray, image_size: core::Size, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &mut impl ToInputOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_output_array_arg!(camera_matrix);
		input_output_array_arg!(dist_coeffs);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_calibrate_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(self.as_raw_mut_CustomPattern(), object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), &image_size, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [CustomPatternTrait::calibrate] function uses the following default values for its arguments:
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
	// cv::ccalib::CustomPattern::calibrate(InputArray, InputArray, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:91
	// ("cv::ccalib::CustomPattern::calibrate", vec![(pred!(mut, ["objectPoints", "imagePoints", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn calibrate_def(&mut self, object_points: &impl ToInputArray, image_points: &impl ToInputArray, image_size: core::Size, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &mut impl ToInputOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray) -> Result<f64> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_output_array_arg!(camera_matrix);
		input_output_array_arg!(dist_coeffs);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_calibrate_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_CustomPattern(), object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), &image_size, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * use_extrinsic_guess: false
	/// * flags: SOLVEPNP_ITERATIVE
	// findRt(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, bool, int)(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:99
	// ("cv::ccalib::CustomPattern::findRt", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "bool", "int"]), _)]),
	#[inline]
	fn find_rt(&mut self, object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToInputOutputArray, tvec: &mut impl ToInputOutputArray, use_extrinsic_guess: bool, flags: i32) -> Result<bool> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		input_output_array_arg!(rvec);
		input_output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_findRt_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool_int(self.as_raw_mut_CustomPattern(), object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), use_extrinsic_guess, flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [CustomPatternTrait::find_rt] function uses the following default values for its arguments:
	/// * use_extrinsic_guess: false
	/// * flags: SOLVEPNP_ITERATIVE
	// cv::ccalib::CustomPattern::findRt(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:99
	// ("cv::ccalib::CustomPattern::findRt", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	#[inline]
	fn find_rt_def(&mut self, object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToInputOutputArray, tvec: &mut impl ToInputOutputArray) -> Result<bool> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		input_output_array_arg!(rvec);
		input_output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_findRt_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(self.as_raw_mut_CustomPattern(), object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * use_extrinsic_guess: false
	/// * flags: SOLVEPNP_ITERATIVE
	// findRt(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, bool, int)(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:101
	// ("cv::ccalib::CustomPattern::findRt", vec![(pred!(mut, ["image", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "bool", "int"]), _)]),
	#[inline]
	fn find_rt_1(&mut self, image: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToInputOutputArray, tvec: &mut impl ToInputOutputArray, use_extrinsic_guess: bool, flags: i32) -> Result<bool> {
		input_array_arg!(image);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		input_output_array_arg!(rvec);
		input_output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_findRt_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool_int(self.as_raw_mut_CustomPattern(), image.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), use_extrinsic_guess, flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [CustomPatternTrait::find_rt] function uses the following default values for its arguments:
	/// * use_extrinsic_guess: false
	/// * flags: SOLVEPNP_ITERATIVE
	// cv::ccalib::CustomPattern::findRt(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:101
	// ("cv::ccalib::CustomPattern::findRt", vec![(pred!(mut, ["image", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	#[inline]
	fn find_rt_def_1(&mut self, image: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToInputOutputArray, tvec: &mut impl ToInputOutputArray) -> Result<bool> {
		input_array_arg!(image);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		input_output_array_arg!(rvec);
		input_output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_findRt_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(self.as_raw_mut_CustomPattern(), image.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * use_extrinsic_guess: false
	/// * iterations_count: 100
	/// * reprojection_error: 8.0
	/// * min_inliers_count: 100
	/// * inliers: noArray()
	/// * flags: SOLVEPNP_ITERATIVE
	// findRtRANSAC(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, bool, int, float, int, OutputArray, int)(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, Primitive, Primitive, Primitive, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:108
	// ("cv::ccalib::CustomPattern::findRtRANSAC", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess", "iterationsCount", "reprojectionError", "minInliersCount", "inliers", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "bool", "int", "float", "int", "const cv::_OutputArray*", "int"]), _)]),
	#[inline]
	fn find_rt_ransac(&mut self, object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToInputOutputArray, tvec: &mut impl ToInputOutputArray, use_extrinsic_guess: bool, iterations_count: i32, reprojection_error: f32, min_inliers_count: i32, inliers: &mut impl ToOutputArray, flags: i32) -> Result<bool> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		input_output_array_arg!(rvec);
		input_output_array_arg!(tvec);
		output_array_arg!(inliers);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_findRtRANSAC_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool_int_float_int_const__OutputArrayR_int(self.as_raw_mut_CustomPattern(), object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), use_extrinsic_guess, iterations_count, reprojection_error, min_inliers_count, inliers.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [CustomPatternTrait::find_rt_ransac] function uses the following default values for its arguments:
	/// * use_extrinsic_guess: false
	/// * iterations_count: 100
	/// * reprojection_error: 8.0
	/// * min_inliers_count: 100
	/// * inliers: noArray()
	/// * flags: SOLVEPNP_ITERATIVE
	// cv::ccalib::CustomPattern::findRtRANSAC(InputArray, InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:108
	// ("cv::ccalib::CustomPattern::findRtRANSAC", vec![(pred!(mut, ["objectPoints", "imagePoints", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	#[inline]
	fn find_rt_ransac_def(&mut self, object_points: &impl ToInputArray, image_points: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToInputOutputArray, tvec: &mut impl ToInputOutputArray) -> Result<bool> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		input_output_array_arg!(rvec);
		input_output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_findRtRANSAC_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(self.as_raw_mut_CustomPattern(), object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * use_extrinsic_guess: false
	/// * iterations_count: 100
	/// * reprojection_error: 8.0
	/// * min_inliers_count: 100
	/// * inliers: noArray()
	/// * flags: SOLVEPNP_ITERATIVE
	// findRtRANSAC(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, bool, int, float, int, OutputArray, int)(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray, Primitive, Primitive, Primitive, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:111
	// ("cv::ccalib::CustomPattern::findRtRANSAC", vec![(pred!(mut, ["image", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess", "iterationsCount", "reprojectionError", "minInliersCount", "inliers", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "bool", "int", "float", "int", "const cv::_OutputArray*", "int"]), _)]),
	#[inline]
	fn find_rt_ransac_1(&mut self, image: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToInputOutputArray, tvec: &mut impl ToInputOutputArray, use_extrinsic_guess: bool, iterations_count: i32, reprojection_error: f32, min_inliers_count: i32, inliers: &mut impl ToOutputArray, flags: i32) -> Result<bool> {
		input_array_arg!(image);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		input_output_array_arg!(rvec);
		input_output_array_arg!(tvec);
		output_array_arg!(inliers);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_findRtRANSAC_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool_int_float_int_const__OutputArrayR_int(self.as_raw_mut_CustomPattern(), image.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), use_extrinsic_guess, iterations_count, reprojection_error, min_inliers_count, inliers.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [CustomPatternTrait::find_rt_ransac] function uses the following default values for its arguments:
	/// * use_extrinsic_guess: false
	/// * iterations_count: 100
	/// * reprojection_error: 8.0
	/// * min_inliers_count: 100
	/// * inliers: noArray()
	/// * flags: SOLVEPNP_ITERATIVE
	// cv::ccalib::CustomPattern::findRtRANSAC(InputArray, InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:111
	// ("cv::ccalib::CustomPattern::findRtRANSAC", vec![(pred!(mut, ["image", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	#[inline]
	fn find_rt_ransac_def_1(&mut self, image: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToInputOutputArray, tvec: &mut impl ToInputOutputArray) -> Result<bool> {
		input_array_arg!(image);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		input_output_array_arg!(rvec);
		input_output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_findRtRANSAC_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(self.as_raw_mut_CustomPattern(), image.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * axis_length: 3
	/// * axis_width: 2
	// drawOrientation(InputOutputArray, InputArray, InputArray, InputArray, InputArray, double, int)(InputOutputArray, InputArray, InputArray, InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:118
	// ("cv::ccalib::CustomPattern::drawOrientation", vec![(pred!(mut, ["image", "tvec", "rvec", "cameraMatrix", "distCoeffs", "axis_length", "axis_width"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "double", "int"]), _)]),
	#[inline]
	fn draw_orientation(&mut self, image: &mut impl ToInputOutputArray, tvec: &impl ToInputArray, rvec: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, axis_length: f64, axis_width: i32) -> Result<()> {
		input_output_array_arg!(image);
		input_array_arg!(tvec);
		input_array_arg!(rvec);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_drawOrientation_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_int(self.as_raw_mut_CustomPattern(), image.as_raw__InputOutputArray(), tvec.as_raw__InputArray(), rvec.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), axis_length, axis_width, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [CustomPatternTrait::draw_orientation] function uses the following default values for its arguments:
	/// * axis_length: 3
	/// * axis_width: 2
	// cv::ccalib::CustomPattern::drawOrientation(InputOutputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:118
	// ("cv::ccalib::CustomPattern::drawOrientation", vec![(pred!(mut, ["image", "tvec", "rvec", "cameraMatrix", "distCoeffs"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn draw_orientation_def(&mut self, image: &mut impl ToInputOutputArray, tvec: &impl ToInputArray, rvec: &impl ToInputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray) -> Result<()> {
		input_output_array_arg!(image);
		input_array_arg!(tvec);
		input_array_arg!(rvec);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_drawOrientation_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_CustomPattern(), image.as_raw__InputOutputArray(), tvec.as_raw__InputArray(), rvec.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// CustomPattern /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:60
pub struct CustomPattern {
	ptr: *mut c_void,
}

opencv_type_boxed! { CustomPattern }

impl Drop for CustomPattern {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_ccalib_CustomPattern_delete(self.as_raw_mut_CustomPattern()) };
	}
}

unsafe impl Send for CustomPattern {}

impl core::AlgorithmTraitConst for CustomPattern {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for CustomPattern {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CustomPattern, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::ccalib::CustomPatternTraitConst for CustomPattern {
	#[inline] fn as_raw_CustomPattern(&self) -> *const c_void { self.as_raw() }
}

impl crate::ccalib::CustomPatternTrait for CustomPattern {
	#[inline] fn as_raw_mut_CustomPattern(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CustomPattern, crate::ccalib::CustomPatternTraitConst, as_raw_CustomPattern, crate::ccalib::CustomPatternTrait, as_raw_mut_CustomPattern }

impl CustomPattern {
	// CustomPattern()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib.hpp:63
	// ("cv::ccalib::CustomPattern::CustomPattern", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::ccalib::CustomPattern> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ccalib_CustomPattern_CustomPattern(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::ccalib::CustomPattern::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { CustomPattern, core::Algorithm, cv_ccalib_CustomPattern_to_Algorithm }

impl std::fmt::Debug for CustomPattern {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("CustomPattern")
			.finish()
	}
}

/// Constant methods for [crate::ccalib::MultiCameraCalibration]
// MultiCameraCalibration /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:72
pub trait MultiCameraCalibrationTraitConst {
	fn as_raw_MultiCameraCalibration(&self) -> *const c_void;

}

/// Mutable methods for [crate::ccalib::MultiCameraCalibration]
pub trait MultiCameraCalibrationTrait: crate::ccalib::MultiCameraCalibrationTraitConst {
	fn as_raw_mut_MultiCameraCalibration(&mut self) -> *mut c_void;

	// loadImages()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:141
	// ("cv::multicalib::MultiCameraCalibration::loadImages", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn load_images(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_multicalib_MultiCameraCalibration_loadImages(self.as_raw_mut_MultiCameraCalibration(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// initialize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:145
	// ("cv::multicalib::MultiCameraCalibration::initialize", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn initialize(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_multicalib_MultiCameraCalibration_initialize(self.as_raw_mut_MultiCameraCalibration(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// optimizeExtrinsics()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:149
	// ("cv::multicalib::MultiCameraCalibration::optimizeExtrinsics", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn optimize_extrinsics(&mut self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_multicalib_MultiCameraCalibration_optimizeExtrinsics(self.as_raw_mut_MultiCameraCalibration(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// run()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:153
	// ("cv::multicalib::MultiCameraCalibration::run", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn run(&mut self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_multicalib_MultiCameraCalibration_run(self.as_raw_mut_MultiCameraCalibration(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// writeParameters(const std::string &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:157
	// ("cv::multicalib::MultiCameraCalibration::writeParameters", vec![(pred!(mut, ["filename"], ["const std::string*"]), _)]),
	#[inline]
	fn write_parameters(&mut self, filename: &str) -> Result<()> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_multicalib_MultiCameraCalibration_writeParameters_const_stringR(self.as_raw_mut_MultiCameraCalibration(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class for multiple camera calibration that supports pinhole camera and omnidirection camera.
/// For omnidirectional camera model, please refer to omnidir.hpp in ccalib module.
/// It first calibrate each camera individually, then a bundle adjustment like optimization is applied to
/// refine extrinsic parameters. So far, it only support "random" pattern for calibration,
/// see randomPattern.hpp in ccalib module for details.
/// Images that are used should be named by "cameraIdx-timestamp.*", several images with the same timestamp
/// means that they are the same pattern that are photographed. cameraIdx should start from 0.
///
/// For more details, please refer to paper
///    B. Li, L. Heng, K. Kevin  and M. Pollefeys, "A Multiple-Camera System
///    Calibration Toolbox Using A Feature Descriptor-Based Calibration
///    Pattern", in IROS 2013.
// MultiCameraCalibration /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:72
pub struct MultiCameraCalibration {
	ptr: *mut c_void,
}

opencv_type_boxed! { MultiCameraCalibration }

impl Drop for MultiCameraCalibration {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_multicalib_MultiCameraCalibration_delete(self.as_raw_mut_MultiCameraCalibration()) };
	}
}

unsafe impl Send for MultiCameraCalibration {}

impl crate::ccalib::MultiCameraCalibrationTraitConst for MultiCameraCalibration {
	#[inline] fn as_raw_MultiCameraCalibration(&self) -> *const c_void { self.as_raw() }
}

impl crate::ccalib::MultiCameraCalibrationTrait for MultiCameraCalibration {
	#[inline] fn as_raw_mut_MultiCameraCalibration(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MultiCameraCalibration, crate::ccalib::MultiCameraCalibrationTraitConst, as_raw_MultiCameraCalibration, crate::ccalib::MultiCameraCalibrationTrait, as_raw_mut_MultiCameraCalibration }

impl MultiCameraCalibration {
	/// ## C++ default parameters
	/// * verbose: 0
	/// * show_extration: 0
	/// * n_mini_matches: 20
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,200,1e-7)
	/// * detector: xfeatures2d::AKAZE::create(xfeatures2d::AKAZE::DESCRIPTOR_MLDB,0,3,0.006f)
	/// * descriptor: xfeatures2d::AKAZE::create(xfeatures2d::AKAZE::DESCRIPTOR_MLDB,0,3,0.006f)
	/// * matcher: DescriptorMatcher::create("BruteForce-L1")
	// MultiCameraCalibration(int, int, const std::string &, float, float, int, int, int, int, TermCriteria, Ptr<FeatureDetector>, Ptr<DescriptorExtractor>, Ptr<DescriptorMatcher>)(Primitive, Primitive, InString, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, SimpleClass, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:132
	// ("cv::multicalib::MultiCameraCalibration::MultiCameraCalibration", vec![(pred!(mut, ["cameraType", "nCameras", "fileName", "patternWidth", "patternHeight", "verbose", "showExtration", "nMiniMatches", "flags", "criteria", "detector", "descriptor", "matcher"], ["int", "int", "const std::string*", "float", "float", "int", "int", "int", "int", "cv::TermCriteria", "cv::Ptr<cv::Feature2D>", "cv::Ptr<cv::Feature2D>", "cv::Ptr<cv::DescriptorMatcher>"]), _)]),
	#[inline]
	pub fn new(camera_type: i32, n_cameras: i32, file_name: &str, pattern_width: f32, pattern_height: f32, verbose: i32, show_extration: i32, n_mini_matches: i32, flags: i32, criteria: core::TermCriteria, mut detector: core::Ptr<crate::features::Feature2D>, mut descriptor: core::Ptr<crate::features::Feature2D>, mut matcher: core::Ptr<crate::features::DescriptorMatcher>) -> Result<crate::ccalib::MultiCameraCalibration> {
		extern_container_arg!(file_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_multicalib_MultiCameraCalibration_MultiCameraCalibration_int_int_const_stringR_float_float_int_int_int_int_TermCriteria_PtrLFeature2DG_PtrLFeature2DG_PtrLDescriptorMatcherG(camera_type, n_cameras, file_name.opencv_as_extern(), pattern_width, pattern_height, verbose, show_extration, n_mini_matches, flags, &criteria, detector.as_raw_mut_PtrOfFeature2D(), descriptor.as_raw_mut_PtrOfFeature2D(), matcher.as_raw_mut_PtrOfDescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::ccalib::MultiCameraCalibration::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * verbose: 0
	/// * show_extration: 0
	/// * n_mini_matches: 20
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,200,1e-7)
	/// * detector: xfeatures2d::AKAZE::create(xfeatures2d::AKAZE::DESCRIPTOR_MLDB,0,3,0.006f)
	/// * descriptor: xfeatures2d::AKAZE::create(xfeatures2d::AKAZE::DESCRIPTOR_MLDB,0,3,0.006f)
	/// * matcher: DescriptorMatcher::create("BruteForce-L1")
	// cv::multicalib::MultiCameraCalibration::MultiCameraCalibration(Primitive, Primitive, InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:132
	// ("cv::multicalib::MultiCameraCalibration::MultiCameraCalibration", vec![(pred!(mut, ["cameraType", "nCameras", "fileName", "patternWidth", "patternHeight"], ["int", "int", "const std::string*", "float", "float"]), _)]),
	#[inline]
	pub fn new_def(camera_type: i32, n_cameras: i32, file_name: &str, pattern_width: f32, pattern_height: f32) -> Result<crate::ccalib::MultiCameraCalibration> {
		extern_container_arg!(file_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_multicalib_MultiCameraCalibration_MultiCameraCalibration_int_int_const_stringR_float_float(camera_type, n_cameras, file_name.opencv_as_extern(), pattern_width, pattern_height, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::ccalib::MultiCameraCalibration::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for MultiCameraCalibration {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MultiCameraCalibration")
			.finish()
	}
}

/// Constant methods for [crate::ccalib::MultiCameraCalibration_edge]
// edge /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:82
pub trait MultiCameraCalibration_edgeTraitConst {
	fn as_raw_MultiCameraCalibration_edge(&self) -> *const c_void;

	// cv::multicalib::MultiCameraCalibration::edge::cameraVertex() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:84
	// ("cv::multicalib::MultiCameraCalibration::edge::cameraVertex", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn camera_vertex(&self) -> i32 {
		let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_propCameraVertex_const(self.as_raw_MultiCameraCalibration_edge()) };
		ret
	}

	// cv::multicalib::MultiCameraCalibration::edge::photoVertex() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:85
	// ("cv::multicalib::MultiCameraCalibration::edge::photoVertex", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn photo_vertex(&self) -> i32 {
		let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_propPhotoVertex_const(self.as_raw_MultiCameraCalibration_edge()) };
		ret
	}

	// cv::multicalib::MultiCameraCalibration::edge::photoIndex() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:86
	// ("cv::multicalib::MultiCameraCalibration::edge::photoIndex", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn photo_index(&self) -> i32 {
		let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_propPhotoIndex_const(self.as_raw_MultiCameraCalibration_edge()) };
		ret
	}

	// cv::multicalib::MultiCameraCalibration::edge::transform() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:87
	// ("cv::multicalib::MultiCameraCalibration::edge::transform", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn transform(&self) -> core::Mat {
		let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_propTransform_const(self.as_raw_MultiCameraCalibration_edge()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

}

/// Mutable methods for [crate::ccalib::MultiCameraCalibration_edge]
pub trait MultiCameraCalibration_edgeTrait: crate::ccalib::MultiCameraCalibration_edgeTraitConst {
	fn as_raw_mut_MultiCameraCalibration_edge(&mut self) -> *mut c_void;

	// cv::multicalib::MultiCameraCalibration::edge::setCameraVertex(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:84
	// ("cv::multicalib::MultiCameraCalibration::edge::setCameraVertex", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_camera_vertex(&mut self, val: i32) {
		let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_propCameraVertex_const_int(self.as_raw_mut_MultiCameraCalibration_edge(), val) };
		ret
	}

	// cv::multicalib::MultiCameraCalibration::edge::setPhotoVertex(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:85
	// ("cv::multicalib::MultiCameraCalibration::edge::setPhotoVertex", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_photo_vertex(&mut self, val: i32) {
		let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_propPhotoVertex_const_int(self.as_raw_mut_MultiCameraCalibration_edge(), val) };
		ret
	}

	// cv::multicalib::MultiCameraCalibration::edge::setPhotoIndex(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:86
	// ("cv::multicalib::MultiCameraCalibration::edge::setPhotoIndex", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_photo_index(&mut self, val: i32) {
		let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_propPhotoIndex_const_int(self.as_raw_mut_MultiCameraCalibration_edge(), val) };
		ret
	}

	// cv::multicalib::MultiCameraCalibration::edge::setTransform(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:87
	// ("cv::multicalib::MultiCameraCalibration::edge::setTransform", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_transform(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_propTransform_const_Mat(self.as_raw_mut_MultiCameraCalibration_edge(), val.as_raw_Mat()) };
		ret
	}

}

// edge /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:82
pub struct MultiCameraCalibration_edge {
	ptr: *mut c_void,
}

opencv_type_boxed! { MultiCameraCalibration_edge }

impl Drop for MultiCameraCalibration_edge {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_delete(self.as_raw_mut_MultiCameraCalibration_edge()) };
	}
}

unsafe impl Send for MultiCameraCalibration_edge {}

impl crate::ccalib::MultiCameraCalibration_edgeTraitConst for MultiCameraCalibration_edge {
	#[inline] fn as_raw_MultiCameraCalibration_edge(&self) -> *const c_void { self.as_raw() }
}

impl crate::ccalib::MultiCameraCalibration_edgeTrait for MultiCameraCalibration_edge {
	#[inline] fn as_raw_mut_MultiCameraCalibration_edge(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MultiCameraCalibration_edge, crate::ccalib::MultiCameraCalibration_edgeTraitConst, as_raw_MultiCameraCalibration_edge, crate::ccalib::MultiCameraCalibration_edgeTrait, as_raw_mut_MultiCameraCalibration_edge }

impl MultiCameraCalibration_edge {
	// edge(int, int, int, Mat)(Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:89
	// ("cv::multicalib::MultiCameraCalibration::edge::edge", vec![(pred!(mut, ["cv", "pv", "pi", "trans"], ["int", "int", "int", "cv::Mat"]), _)]),
	#[inline]
	pub fn new(cv: i32, pv: i32, pi: i32, mut trans: impl core::MatTrait) -> Result<crate::ccalib::MultiCameraCalibration_edge> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_edge_int_int_int_Mat(cv, pv, pi, trans.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::ccalib::MultiCameraCalibration_edge::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for MultiCameraCalibration_edge {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MultiCameraCalibration_edge")
			.field("camera_vertex", &crate::ccalib::MultiCameraCalibration_edgeTraitConst::camera_vertex(self))
			.field("photo_vertex", &crate::ccalib::MultiCameraCalibration_edgeTraitConst::photo_vertex(self))
			.field("photo_index", &crate::ccalib::MultiCameraCalibration_edgeTraitConst::photo_index(self))
			.field("transform", &crate::ccalib::MultiCameraCalibration_edgeTraitConst::transform(self))
			.finish()
	}
}

/// Constant methods for [crate::ccalib::MultiCameraCalibration_vertex]
// vertex /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:98
pub trait MultiCameraCalibration_vertexTraitConst {
	fn as_raw_MultiCameraCalibration_vertex(&self) -> *const c_void;

	// cv::multicalib::MultiCameraCalibration::vertex::pose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:100
	// ("cv::multicalib::MultiCameraCalibration::vertex::pose", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pose(&self) -> core::Mat {
		let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_vertex_propPose_const(self.as_raw_MultiCameraCalibration_vertex()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

	// cv::multicalib::MultiCameraCalibration::vertex::timestamp() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:103
	// ("cv::multicalib::MultiCameraCalibration::vertex::timestamp", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn timestamp(&self) -> i32 {
		let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_vertex_propTimestamp_const(self.as_raw_MultiCameraCalibration_vertex()) };
		ret
	}

}

/// Mutable methods for [crate::ccalib::MultiCameraCalibration_vertex]
pub trait MultiCameraCalibration_vertexTrait: crate::ccalib::MultiCameraCalibration_vertexTraitConst {
	fn as_raw_mut_MultiCameraCalibration_vertex(&mut self) -> *mut c_void;

	// cv::multicalib::MultiCameraCalibration::vertex::setPose(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:100
	// ("cv::multicalib::MultiCameraCalibration::vertex::setPose", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_pose(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_vertex_propPose_const_Mat(self.as_raw_mut_MultiCameraCalibration_vertex(), val.as_raw_Mat()) };
		ret
	}

	// cv::multicalib::MultiCameraCalibration::vertex::setTimestamp(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:103
	// ("cv::multicalib::MultiCameraCalibration::vertex::setTimestamp", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_timestamp(&mut self, val: i32) {
		let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_vertex_propTimestamp_const_int(self.as_raw_mut_MultiCameraCalibration_vertex(), val) };
		ret
	}

}

// vertex /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:98
pub struct MultiCameraCalibration_vertex {
	ptr: *mut c_void,
}

opencv_type_boxed! { MultiCameraCalibration_vertex }

impl Drop for MultiCameraCalibration_vertex {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_multicalib_MultiCameraCalibration_vertex_delete(self.as_raw_mut_MultiCameraCalibration_vertex()) };
	}
}

unsafe impl Send for MultiCameraCalibration_vertex {}

impl crate::ccalib::MultiCameraCalibration_vertexTraitConst for MultiCameraCalibration_vertex {
	#[inline] fn as_raw_MultiCameraCalibration_vertex(&self) -> *const c_void { self.as_raw() }
}

impl crate::ccalib::MultiCameraCalibration_vertexTrait for MultiCameraCalibration_vertex {
	#[inline] fn as_raw_mut_MultiCameraCalibration_vertex(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MultiCameraCalibration_vertex, crate::ccalib::MultiCameraCalibration_vertexTraitConst, as_raw_MultiCameraCalibration_vertex, crate::ccalib::MultiCameraCalibration_vertexTrait, as_raw_mut_MultiCameraCalibration_vertex }

impl MultiCameraCalibration_vertex {
	// vertex(Mat, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:105
	// ("cv::multicalib::MultiCameraCalibration::vertex::vertex", vec![(pred!(mut, ["po", "ts"], ["cv::Mat", "int"]), _)]),
	#[inline]
	pub fn new(mut po: impl core::MatTrait, ts: i32) -> Result<crate::ccalib::MultiCameraCalibration_vertex> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_multicalib_MultiCameraCalibration_vertex_vertex_Mat_int(po.as_raw_mut_Mat(), ts, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::ccalib::MultiCameraCalibration_vertex::opencv_from_extern(ret) };
		Ok(ret)
	}

	// vertex()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/multicalib.hpp:111
	// ("cv::multicalib::MultiCameraCalibration::vertex::vertex", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::ccalib::MultiCameraCalibration_vertex> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_multicalib_MultiCameraCalibration_vertex_vertex(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::ccalib::MultiCameraCalibration_vertex::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for MultiCameraCalibration_vertex {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MultiCameraCalibration_vertex")
			.field("pose", &crate::ccalib::MultiCameraCalibration_vertexTraitConst::pose(self))
			.field("timestamp", &crate::ccalib::MultiCameraCalibration_vertexTraitConst::timestamp(self))
			.finish()
	}
}

/// Constant methods for [crate::ccalib::RandomPatternCornerFinder]
// RandomPatternCornerFinder /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/randpattern.hpp:66
pub trait RandomPatternCornerFinderTraitConst {
	fn as_raw_RandomPatternCornerFinder(&self) -> *const c_void;

}

/// Mutable methods for [crate::ccalib::RandomPatternCornerFinder]
pub trait RandomPatternCornerFinderTrait: crate::ccalib::RandomPatternCornerFinderTraitConst {
	fn as_raw_mut_RandomPatternCornerFinder(&mut self) -> *mut c_void;

	// loadPattern(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/randpattern.hpp:90
	// ("cv::randpattern::RandomPatternCornerFinder::loadPattern", vec![(pred!(mut, ["patternImage"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn load_pattern(&mut self, pattern_image: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_randpattern_RandomPatternCornerFinder_loadPattern_const_MatR(self.as_raw_mut_RandomPatternCornerFinder(), pattern_image.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// loadPattern(const cv::Mat &, const std::vector<cv::KeyPoint> &, const cv::Mat &)(TraitClass, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/randpattern.hpp:97
	// ("cv::randpattern::RandomPatternCornerFinder::loadPattern", vec![(pred!(mut, ["patternImage", "patternKeyPoints", "patternDescriptors"], ["const cv::Mat*", "const std::vector<cv::KeyPoint>*", "const cv::Mat*"]), _)]),
	#[inline]
	fn load_pattern_1(&mut self, pattern_image: &impl core::MatTraitConst, pattern_key_points: &core::Vector<core::KeyPoint>, pattern_descriptors: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_randpattern_RandomPatternCornerFinder_loadPattern_const_MatR_const_vectorLKeyPointGR_const_MatR(self.as_raw_mut_RandomPatternCornerFinder(), pattern_image.as_raw_Mat(), pattern_key_points.as_raw_VectorOfKeyPoint(), pattern_descriptors.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// computeObjectImagePoints(std::vector<cv::Mat>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/randpattern.hpp:106
	// ("cv::randpattern::RandomPatternCornerFinder::computeObjectImagePoints", vec![(pred!(mut, ["inputImages"], ["std::vector<cv::Mat>"]), _)]),
	#[inline]
	fn compute_object_image_points(&mut self, mut input_images: core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_randpattern_RandomPatternCornerFinder_computeObjectImagePoints_vectorLMatG(self.as_raw_mut_RandomPatternCornerFinder(), input_images.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// computeObjectImagePointsForSingle(cv::Mat)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/randpattern.hpp:115
	// ("cv::randpattern::RandomPatternCornerFinder::computeObjectImagePointsForSingle", vec![(pred!(mut, ["inputImage"], ["cv::Mat"]), _)]),
	#[inline]
	fn compute_object_image_points_for_single(&mut self, mut input_image: impl core::MatTrait) -> Result<core::Vector<core::Mat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_randpattern_RandomPatternCornerFinder_computeObjectImagePointsForSingle_Mat(self.as_raw_mut_RandomPatternCornerFinder(), input_image.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getObjectPoints()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/randpattern.hpp:119
	// ("cv::randpattern::RandomPatternCornerFinder::getObjectPoints", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_object_points(&mut self) -> Result<core::Vector<core::Mat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_randpattern_RandomPatternCornerFinder_getObjectPoints(self.as_raw_mut_RandomPatternCornerFinder(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getImagePoints()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/randpattern.hpp:123
	// ("cv::randpattern::RandomPatternCornerFinder::getImagePoints", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_image_points(&mut self) -> Result<core::Vector<core::Mat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_randpattern_RandomPatternCornerFinder_getImagePoints(self.as_raw_mut_RandomPatternCornerFinder(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Class for finding features points and corresponding 3D in world coordinate of
/// a "random" pattern, which can be to be used in calibration. It is useful when pattern is
/// partly occluded or only a part of pattern can be observed in multiple cameras calibration.
/// The pattern can be generated by RandomPatternGenerator class described in this file.
///
/// Please refer to paper
///    B. Li, L. Heng, K. Kevin  and M. Pollefeys, "A Multiple-Camera System
///    Calibration Toolbox Using A Feature Descriptor-Based Calibration
///    Pattern", in IROS 2013.
// RandomPatternCornerFinder /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/randpattern.hpp:66
pub struct RandomPatternCornerFinder {
	ptr: *mut c_void,
}

opencv_type_boxed! { RandomPatternCornerFinder }

impl Drop for RandomPatternCornerFinder {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_randpattern_RandomPatternCornerFinder_delete(self.as_raw_mut_RandomPatternCornerFinder()) };
	}
}

unsafe impl Send for RandomPatternCornerFinder {}

impl crate::ccalib::RandomPatternCornerFinderTraitConst for RandomPatternCornerFinder {
	#[inline] fn as_raw_RandomPatternCornerFinder(&self) -> *const c_void { self.as_raw() }
}

impl crate::ccalib::RandomPatternCornerFinderTrait for RandomPatternCornerFinder {
	#[inline] fn as_raw_mut_RandomPatternCornerFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { RandomPatternCornerFinder, crate::ccalib::RandomPatternCornerFinderTraitConst, as_raw_RandomPatternCornerFinder, crate::ccalib::RandomPatternCornerFinderTrait, as_raw_mut_RandomPatternCornerFinder }

impl RandomPatternCornerFinder {
	/// ## C++ default parameters
	/// * nmini_match: 20
	/// * depth: CV_32F
	/// * verbose: 0
	/// * show_extraction: 0
	/// * detector: xfeatures2d::AKAZE::create(xfeatures2d::AKAZE::DESCRIPTOR_MLDB,0,3,0.005f)
	/// * descriptor: xfeatures2d::AKAZE::create(xfeatures2d::AKAZE::DESCRIPTOR_MLDB,0,3,0.005f)
	/// * matcher: DescriptorMatcher::create("BruteForce-L1")
	// RandomPatternCornerFinder(float, float, int, int, int, int, Ptr<FeatureDetector>, Ptr<DescriptorExtractor>, Ptr<DescriptorMatcher>)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/randpattern.hpp:81
	// ("cv::randpattern::RandomPatternCornerFinder::RandomPatternCornerFinder", vec![(pred!(mut, ["patternWidth", "patternHeight", "nminiMatch", "depth", "verbose", "showExtraction", "detector", "descriptor", "matcher"], ["float", "float", "int", "int", "int", "int", "cv::Ptr<cv::Feature2D>", "cv::Ptr<cv::Feature2D>", "cv::Ptr<cv::DescriptorMatcher>"]), _)]),
	#[inline]
	pub fn new(pattern_width: f32, pattern_height: f32, nmini_match: i32, depth: i32, verbose: i32, show_extraction: i32, mut detector: core::Ptr<crate::features::Feature2D>, mut descriptor: core::Ptr<crate::features::Feature2D>, mut matcher: core::Ptr<crate::features::DescriptorMatcher>) -> Result<crate::ccalib::RandomPatternCornerFinder> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_randpattern_RandomPatternCornerFinder_RandomPatternCornerFinder_float_float_int_int_int_int_PtrLFeature2DG_PtrLFeature2DG_PtrLDescriptorMatcherG(pattern_width, pattern_height, nmini_match, depth, verbose, show_extraction, detector.as_raw_mut_PtrOfFeature2D(), descriptor.as_raw_mut_PtrOfFeature2D(), matcher.as_raw_mut_PtrOfDescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::ccalib::RandomPatternCornerFinder::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * nmini_match: 20
	/// * depth: CV_32F
	/// * verbose: 0
	/// * show_extraction: 0
	/// * detector: xfeatures2d::AKAZE::create(xfeatures2d::AKAZE::DESCRIPTOR_MLDB,0,3,0.005f)
	/// * descriptor: xfeatures2d::AKAZE::create(xfeatures2d::AKAZE::DESCRIPTOR_MLDB,0,3,0.005f)
	/// * matcher: DescriptorMatcher::create("BruteForce-L1")
	// cv::randpattern::RandomPatternCornerFinder::RandomPatternCornerFinder(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/randpattern.hpp:81
	// ("cv::randpattern::RandomPatternCornerFinder::RandomPatternCornerFinder", vec![(pred!(mut, ["patternWidth", "patternHeight"], ["float", "float"]), _)]),
	#[inline]
	pub fn new_def(pattern_width: f32, pattern_height: f32) -> Result<crate::ccalib::RandomPatternCornerFinder> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_randpattern_RandomPatternCornerFinder_RandomPatternCornerFinder_float_float(pattern_width, pattern_height, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::ccalib::RandomPatternCornerFinder::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for RandomPatternCornerFinder {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("RandomPatternCornerFinder")
			.finish()
	}
}

/// Constant methods for [crate::ccalib::RandomPatternGenerator]
// RandomPatternGenerator /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/randpattern.hpp:161
pub trait RandomPatternGeneratorTraitConst {
	fn as_raw_RandomPatternGenerator(&self) -> *const c_void;

}

/// Mutable methods for [crate::ccalib::RandomPatternGenerator]
pub trait RandomPatternGeneratorTrait: crate::ccalib::RandomPatternGeneratorTraitConst {
	fn as_raw_mut_RandomPatternGenerator(&mut self) -> *mut c_void;

	// generatePattern()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/randpattern.hpp:173
	// ("cv::randpattern::RandomPatternGenerator::generatePattern", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn generate_pattern(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_randpattern_RandomPatternGenerator_generatePattern(self.as_raw_mut_RandomPatternGenerator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getPattern()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/randpattern.hpp:176
	// ("cv::randpattern::RandomPatternGenerator::getPattern", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_pattern(&mut self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_randpattern_RandomPatternGenerator_getPattern(self.as_raw_mut_RandomPatternGenerator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

// RandomPatternGenerator /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/randpattern.hpp:161
pub struct RandomPatternGenerator {
	ptr: *mut c_void,
}

opencv_type_boxed! { RandomPatternGenerator }

impl Drop for RandomPatternGenerator {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_randpattern_RandomPatternGenerator_delete(self.as_raw_mut_RandomPatternGenerator()) };
	}
}

unsafe impl Send for RandomPatternGenerator {}

impl crate::ccalib::RandomPatternGeneratorTraitConst for RandomPatternGenerator {
	#[inline] fn as_raw_RandomPatternGenerator(&self) -> *const c_void { self.as_raw() }
}

impl crate::ccalib::RandomPatternGeneratorTrait for RandomPatternGenerator {
	#[inline] fn as_raw_mut_RandomPatternGenerator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { RandomPatternGenerator, crate::ccalib::RandomPatternGeneratorTraitConst, as_raw_RandomPatternGenerator, crate::ccalib::RandomPatternGeneratorTrait, as_raw_mut_RandomPatternGenerator }

impl RandomPatternGenerator {
	// RandomPatternGenerator(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/ccalib/randpattern.hpp:169
	// ("cv::randpattern::RandomPatternGenerator::RandomPatternGenerator", vec![(pred!(mut, ["imageWidth", "imageHeight"], ["int", "int"]), _)]),
	#[inline]
	pub fn new(image_width: i32, image_height: i32) -> Result<crate::ccalib::RandomPatternGenerator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_randpattern_RandomPatternGenerator_RandomPatternGenerator_int_int(image_width, image_height, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::ccalib::RandomPatternGenerator::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for RandomPatternGenerator {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("RandomPatternGenerator")
			.finish()
	}
}
