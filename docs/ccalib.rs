pub mod ccalib {
	//! # Custom Calibration Pattern for 3D reconstruction
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::CustomPatternTraitConst, super::CustomPatternTrait, super::RandomPatternCornerFinderTraitConst, super::RandomPatternCornerFinderTrait, super::RandomPatternGeneratorTraitConst, super::RandomPatternGeneratorTrait, super::MultiCameraCalibration_edgeTraitConst, super::MultiCameraCalibration_edgeTrait, super::MultiCameraCalibration_vertexTraitConst, super::MultiCameraCalibration_vertexTrait, super::MultiCameraCalibrationTraitConst, super::MultiCameraCalibrationTrait };
	}
	
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
	#[inline]
	pub fn calibrate_def(object_points: &impl core::ToInputArray, image_points: &impl core::ToInputArray, size: core::Size, k: &mut impl core::ToInputOutputArray, xi: &mut impl core::ToInputOutputArray, d: &mut impl core::ToInputOutputArray, rvecs: &mut impl core::ToOutputArray, tvecs: &mut impl core::ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_output_array_arg!(k);
		input_output_array_arg!(xi);
		input_output_array_arg!(d);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_omnidir_calibrate_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), size.opencv_as_extern(), k.as_raw__InputOutputArray(), xi.as_raw__InputOutputArray(), d.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, criteria.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
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
	#[inline]
	pub fn calibrate(object_points: &impl core::ToInputArray, image_points: &impl core::ToInputArray, size: core::Size, k: &mut impl core::ToInputOutputArray, xi: &mut impl core::ToInputOutputArray, d: &mut impl core::ToInputOutputArray, rvecs: &mut impl core::ToOutputArray, tvecs: &mut impl core::ToOutputArray, flags: i32, criteria: core::TermCriteria, idx: &mut impl core::ToOutputArray) -> Result<f64> {
		input_array_arg!(object_points);
		input_array_arg!(image_points);
		input_output_array_arg!(k);
		input_output_array_arg!(xi);
		input_output_array_arg!(d);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		output_array_arg!(idx);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_omnidir_calibrate_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria_const__OutputArrayR(object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), size.opencv_as_extern(), k.as_raw__InputOutputArray(), xi.as_raw__InputOutputArray(), d.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, criteria.opencv_as_extern(), idx.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
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
	#[inline]
	pub fn init_undistort_rectify_map(k: &impl core::ToInputArray, d: &impl core::ToInputArray, xi: &impl core::ToInputArray, r: &impl core::ToInputArray, p: &impl core::ToInputArray, size: core::Size, m1type: i32, map1: &mut impl core::ToOutputArray, map2: &mut impl core::ToOutputArray, flags: i32) -> Result<()> {
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
	#[inline]
	pub fn project_points_1_def(object_points: &impl core::ToInputArray, image_points: &mut impl core::ToOutputArray, affine: core::Affine3d, k: &impl core::ToInputArray, xi: f64, d: &impl core::ToInputArray) -> Result<()> {
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
	#[inline]
	pub fn project_points_1(object_points: &impl core::ToInputArray, image_points: &mut impl core::ToOutputArray, affine: core::Affine3d, k: &impl core::ToInputArray, xi: f64, d: &impl core::ToInputArray, jacobian: &mut impl core::ToOutputArray) -> Result<()> {
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
	#[inline]
	pub fn project_points_def(object_points: &impl core::ToInputArray, image_points: &mut impl core::ToOutputArray, rvec: &impl core::ToInputArray, tvec: &impl core::ToInputArray, k: &impl core::ToInputArray, xi: f64, d: &impl core::ToInputArray) -> Result<()> {
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
	#[inline]
	pub fn project_points(object_points: &impl core::ToInputArray, image_points: &mut impl core::ToOutputArray, rvec: &impl core::ToInputArray, tvec: &impl core::ToInputArray, k: &impl core::ToInputArray, xi: f64, d: &impl core::ToInputArray, jacobian: &mut impl core::ToOutputArray) -> Result<()> {
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
	#[inline]
	pub fn stereo_calibrate_def(object_points: &mut impl core::ToInputOutputArray, image_points1: &mut impl core::ToInputOutputArray, image_points2: &mut impl core::ToInputOutputArray, image_size1: core::Size, image_size2: core::Size, k1: &mut impl core::ToInputOutputArray, xi1: &mut impl core::ToInputOutputArray, d1: &mut impl core::ToInputOutputArray, k2: &mut impl core::ToInputOutputArray, xi2: &mut impl core::ToInputOutputArray, d2: &mut impl core::ToInputOutputArray, rvec: &mut impl core::ToOutputArray, tvec: &mut impl core::ToOutputArray, rvecs_l: &mut impl core::ToOutputArray, tvecs_l: &mut impl core::ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
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
		unsafe { sys::cv_omnidir_stereoCalibrate_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const_SizeR_const_SizeR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(object_points.as_raw__InputOutputArray(), image_points1.as_raw__InputOutputArray(), image_points2.as_raw__InputOutputArray(), &image_size1, &image_size2, k1.as_raw__InputOutputArray(), xi1.as_raw__InputOutputArray(), d1.as_raw__InputOutputArray(), k2.as_raw__InputOutputArray(), xi2.as_raw__InputOutputArray(), d2.as_raw__InputOutputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), rvecs_l.as_raw__OutputArray(), tvecs_l.as_raw__OutputArray(), flags, criteria.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
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
	#[inline]
	pub fn stereo_calibrate(object_points: &mut impl core::ToInputOutputArray, image_points1: &mut impl core::ToInputOutputArray, image_points2: &mut impl core::ToInputOutputArray, image_size1: core::Size, image_size2: core::Size, k1: &mut impl core::ToInputOutputArray, xi1: &mut impl core::ToInputOutputArray, d1: &mut impl core::ToInputOutputArray, k2: &mut impl core::ToInputOutputArray, xi2: &mut impl core::ToInputOutputArray, d2: &mut impl core::ToInputOutputArray, rvec: &mut impl core::ToOutputArray, tvec: &mut impl core::ToOutputArray, rvecs_l: &mut impl core::ToOutputArray, tvecs_l: &mut impl core::ToOutputArray, flags: i32, criteria: core::TermCriteria, idx: &mut impl core::ToOutputArray) -> Result<f64> {
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
		unsafe { sys::cv_omnidir_stereoCalibrate_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const_SizeR_const_SizeR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria_const__OutputArrayR(object_points.as_raw__InputOutputArray(), image_points1.as_raw__InputOutputArray(), image_points2.as_raw__InputOutputArray(), &image_size1, &image_size2, k1.as_raw__InputOutputArray(), xi1.as_raw__InputOutputArray(), d1.as_raw__InputOutputArray(), k2.as_raw__InputOutputArray(), xi2.as_raw__InputOutputArray(), d2.as_raw__InputOutputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), rvecs_l.as_raw__OutputArray(), tvecs_l.as_raw__OutputArray(), flags, criteria.opencv_as_extern(), idx.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
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
	#[inline]
	pub fn stereo_reconstruct_def(image1: &impl core::ToInputArray, image2: &impl core::ToInputArray, k1: &impl core::ToInputArray, d1: &impl core::ToInputArray, xi1: &impl core::ToInputArray, k2: &impl core::ToInputArray, d2: &impl core::ToInputArray, xi2: &impl core::ToInputArray, r: &impl core::ToInputArray, t: &impl core::ToInputArray, flag: i32, num_disparities: i32, sad_window_size: i32, disparity: &mut impl core::ToOutputArray, image1_rec: &mut impl core::ToOutputArray, image2_rec: &mut impl core::ToOutputArray) -> Result<()> {
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
	#[inline]
	pub fn stereo_reconstruct(image1: &impl core::ToInputArray, image2: &impl core::ToInputArray, k1: &impl core::ToInputArray, d1: &impl core::ToInputArray, xi1: &impl core::ToInputArray, k2: &impl core::ToInputArray, d2: &impl core::ToInputArray, xi2: &impl core::ToInputArray, r: &impl core::ToInputArray, t: &impl core::ToInputArray, flag: i32, num_disparities: i32, sad_window_size: i32, disparity: &mut impl core::ToOutputArray, image1_rec: &mut impl core::ToOutputArray, image2_rec: &mut impl core::ToOutputArray, new_size: core::Size, knew: &impl core::ToInputArray, point_cloud: &mut impl core::ToOutputArray, point_type: i32) -> Result<()> {
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
	#[inline]
	pub fn stereo_rectify(r: &impl core::ToInputArray, t: &impl core::ToInputArray, r1: &mut impl core::ToOutputArray, r2: &mut impl core::ToOutputArray) -> Result<()> {
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
	#[inline]
	pub fn undistort_image_def(distorted: &impl core::ToInputArray, undistorted: &mut impl core::ToOutputArray, k: &impl core::ToInputArray, d: &impl core::ToInputArray, xi: &impl core::ToInputArray, flags: i32) -> Result<()> {
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
	#[inline]
	pub fn undistort_image(distorted: &impl core::ToInputArray, undistorted: &mut impl core::ToOutputArray, k: &impl core::ToInputArray, d: &impl core::ToInputArray, xi: &impl core::ToInputArray, flags: i32, knew: &impl core::ToInputArray, new_size: core::Size, r: &impl core::ToInputArray) -> Result<()> {
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
	#[inline]
	pub fn undistort_points(distorted: &impl core::ToInputArray, undistorted: &mut impl core::ToOutputArray, k: &impl core::ToInputArray, d: &impl core::ToInputArray, xi: &impl core::ToInputArray, r: &impl core::ToInputArray) -> Result<()> {
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
	pub trait CustomPatternTraitConst: core::AlgorithmTraitConst {
		fn as_raw_CustomPattern(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::ccalib::CustomPattern]
	pub trait CustomPatternTrait: core::AlgorithmTrait + crate::ccalib::CustomPatternTraitConst {
		fn as_raw_mut_CustomPattern(&mut self) -> *mut c_void;
	
		/// ## C++ default parameters
		/// * output: noArray()
		#[inline]
		fn create(&mut self, pattern: &impl core::ToInputArray, board_size: core::Size2f, output: &mut impl core::ToOutputArray) -> Result<bool> {
			input_array_arg!(pattern);
			output_array_arg!(output);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ccalib_CustomPattern_create_const__InputArrayR_const_Size2f_const__OutputArrayR(self.as_raw_mut_CustomPattern(), pattern.as_raw__InputArray(), board_size.opencv_as_extern(), output.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * output: noArray()
		#[inline]
		fn create_def(&mut self, pattern: &impl core::ToInputArray, board_size: core::Size2f) -> Result<bool> {
			input_array_arg!(pattern);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ccalib_CustomPattern_create_const__InputArrayR_const_Size2f(self.as_raw_mut_CustomPattern(), pattern.as_raw__InputArray(), board_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
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
		#[inline]
		fn find_pattern(&mut self, image: &impl core::ToInputArray, matched_features: &mut impl core::ToOutputArray, pattern_points: &mut impl core::ToOutputArray, ratio: f64, proj_error: f64, refine_position: bool, out: &mut impl core::ToOutputArray, h: &mut impl core::ToOutputArray, pattern_corners: &mut impl core::ToOutputArray) -> Result<bool> {
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
		/// This alternative version of [find_pattern] function uses the following default values for its arguments:
		/// * ratio: 0.7
		/// * proj_error: 8.0
		/// * refine_position: false
		/// * out: noArray()
		/// * h: noArray()
		/// * pattern_corners: noArray()
		#[inline]
		fn find_pattern_def(&mut self, image: &impl core::ToInputArray, matched_features: &mut impl core::ToOutputArray, pattern_points: &mut impl core::ToOutputArray) -> Result<bool> {
			input_array_arg!(image);
			output_array_arg!(matched_features);
			output_array_arg!(pattern_points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ccalib_CustomPattern_findPattern_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_CustomPattern(), image.as_raw__InputArray(), matched_features.as_raw__OutputArray(), pattern_points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn is_initialized(&mut self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ccalib_CustomPattern_isInitialized(self.as_raw_mut_CustomPattern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_pattern_points(&mut self, original_points: &mut core::Vector<core::KeyPoint>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ccalib_CustomPattern_getPatternPoints_vectorLKeyPointGR(self.as_raw_mut_CustomPattern(), original_points.as_raw_mut_VectorOfKeyPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_pixel_size(&mut self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ccalib_CustomPattern_getPixelSize(self.as_raw_mut_CustomPattern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_feature_detector(&mut self, mut feature_detector: core::Ptr<crate::features2d::Feature2D>) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ccalib_CustomPattern_setFeatureDetector_PtrLFeature2DG(self.as_raw_mut_CustomPattern(), feature_detector.as_raw_mut_PtrOfFeature2D(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_descriptor_extractor(&mut self, mut extractor: core::Ptr<crate::features2d::Feature2D>) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ccalib_CustomPattern_setDescriptorExtractor_PtrLFeature2DG(self.as_raw_mut_CustomPattern(), extractor.as_raw_mut_PtrOfFeature2D(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_descriptor_matcher(&mut self, mut matcher: core::Ptr<crate::features2d::DescriptorMatcher>) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ccalib_CustomPattern_setDescriptorMatcher_PtrLDescriptorMatcherG(self.as_raw_mut_CustomPattern(), matcher.as_raw_mut_PtrOfDescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_feature_detector(&mut self) -> Result<core::Ptr<crate::features2d::Feature2D>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ccalib_CustomPattern_getFeatureDetector(self.as_raw_mut_CustomPattern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features2d::Feature2D>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_descriptor_extractor(&mut self) -> Result<core::Ptr<crate::features2d::Feature2D>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ccalib_CustomPattern_getDescriptorExtractor(self.as_raw_mut_CustomPattern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features2d::Feature2D>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_descriptor_matcher(&mut self) -> Result<core::Ptr<crate::features2d::DescriptorMatcher>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ccalib_CustomPattern_getDescriptorMatcher(self.as_raw_mut_CustomPattern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features2d::DescriptorMatcher>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * flags: 0
		/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
		#[inline]
		fn calibrate(&mut self, object_points: &impl core::ToInputArray, image_points: &impl core::ToInputArray, image_size: core::Size, camera_matrix: &mut impl core::ToInputOutputArray, dist_coeffs: &mut impl core::ToInputOutputArray, rvecs: &mut impl core::ToOutputArray, tvecs: &mut impl core::ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
			input_array_arg!(object_points);
			input_array_arg!(image_points);
			input_output_array_arg!(camera_matrix);
			input_output_array_arg!(dist_coeffs);
			output_array_arg!(rvecs);
			output_array_arg!(tvecs);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ccalib_CustomPattern_calibrate_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(self.as_raw_mut_CustomPattern(), object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), image_size.opencv_as_extern(), camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, criteria.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [calibrate] function uses the following default values for its arguments:
		/// * flags: 0
		/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
		#[inline]
		fn calibrate_def(&mut self, object_points: &impl core::ToInputArray, image_points: &impl core::ToInputArray, image_size: core::Size, camera_matrix: &mut impl core::ToInputOutputArray, dist_coeffs: &mut impl core::ToInputOutputArray, rvecs: &mut impl core::ToOutputArray, tvecs: &mut impl core::ToOutputArray) -> Result<f64> {
			input_array_arg!(object_points);
			input_array_arg!(image_points);
			input_output_array_arg!(camera_matrix);
			input_output_array_arg!(dist_coeffs);
			output_array_arg!(rvecs);
			output_array_arg!(tvecs);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ccalib_CustomPattern_calibrate_const__InputArrayR_const__InputArrayR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_CustomPattern(), object_points.as_raw__InputArray(), image_points.as_raw__InputArray(), image_size.opencv_as_extern(), camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * use_extrinsic_guess: false
		/// * flags: SOLVEPNP_ITERATIVE
		#[inline]
		fn find_rt(&mut self, object_points: &impl core::ToInputArray, image_points: &impl core::ToInputArray, camera_matrix: &impl core::ToInputArray, dist_coeffs: &impl core::ToInputArray, rvec: &mut impl core::ToInputOutputArray, tvec: &mut impl core::ToInputOutputArray, use_extrinsic_guess: bool, flags: i32) -> Result<bool> {
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
		/// This alternative version of [find_rt] function uses the following default values for its arguments:
		/// * use_extrinsic_guess: false
		/// * flags: SOLVEPNP_ITERATIVE
		#[inline]
		fn find_rt_def(&mut self, object_points: &impl core::ToInputArray, image_points: &impl core::ToInputArray, camera_matrix: &impl core::ToInputArray, dist_coeffs: &impl core::ToInputArray, rvec: &mut impl core::ToInputOutputArray, tvec: &mut impl core::ToInputOutputArray) -> Result<bool> {
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
		#[inline]
		fn find_rt_1(&mut self, image: &impl core::ToInputArray, camera_matrix: &impl core::ToInputArray, dist_coeffs: &impl core::ToInputArray, rvec: &mut impl core::ToInputOutputArray, tvec: &mut impl core::ToInputOutputArray, use_extrinsic_guess: bool, flags: i32) -> Result<bool> {
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
		/// This alternative version of [find_rt] function uses the following default values for its arguments:
		/// * use_extrinsic_guess: false
		/// * flags: SOLVEPNP_ITERATIVE
		#[inline]
		fn find_rt_def_1(&mut self, image: &impl core::ToInputArray, camera_matrix: &impl core::ToInputArray, dist_coeffs: &impl core::ToInputArray, rvec: &mut impl core::ToInputOutputArray, tvec: &mut impl core::ToInputOutputArray) -> Result<bool> {
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
		#[inline]
		fn find_rt_ransac(&mut self, object_points: &impl core::ToInputArray, image_points: &impl core::ToInputArray, camera_matrix: &impl core::ToInputArray, dist_coeffs: &impl core::ToInputArray, rvec: &mut impl core::ToInputOutputArray, tvec: &mut impl core::ToInputOutputArray, use_extrinsic_guess: bool, iterations_count: i32, reprojection_error: f32, min_inliers_count: i32, inliers: &mut impl core::ToOutputArray, flags: i32) -> Result<bool> {
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
		/// This alternative version of [find_rt_ransac] function uses the following default values for its arguments:
		/// * use_extrinsic_guess: false
		/// * iterations_count: 100
		/// * reprojection_error: 8.0
		/// * min_inliers_count: 100
		/// * inliers: noArray()
		/// * flags: SOLVEPNP_ITERATIVE
		#[inline]
		fn find_rt_ransac_def(&mut self, object_points: &impl core::ToInputArray, image_points: &impl core::ToInputArray, camera_matrix: &impl core::ToInputArray, dist_coeffs: &impl core::ToInputArray, rvec: &mut impl core::ToInputOutputArray, tvec: &mut impl core::ToInputOutputArray) -> Result<bool> {
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
		#[inline]
		fn find_rt_ransac_1(&mut self, image: &impl core::ToInputArray, camera_matrix: &impl core::ToInputArray, dist_coeffs: &impl core::ToInputArray, rvec: &mut impl core::ToInputOutputArray, tvec: &mut impl core::ToInputOutputArray, use_extrinsic_guess: bool, iterations_count: i32, reprojection_error: f32, min_inliers_count: i32, inliers: &mut impl core::ToOutputArray, flags: i32) -> Result<bool> {
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
		/// This alternative version of [find_rt_ransac] function uses the following default values for its arguments:
		/// * use_extrinsic_guess: false
		/// * iterations_count: 100
		/// * reprojection_error: 8.0
		/// * min_inliers_count: 100
		/// * inliers: noArray()
		/// * flags: SOLVEPNP_ITERATIVE
		#[inline]
		fn find_rt_ransac_def_1(&mut self, image: &impl core::ToInputArray, camera_matrix: &impl core::ToInputArray, dist_coeffs: &impl core::ToInputArray, rvec: &mut impl core::ToInputOutputArray, tvec: &mut impl core::ToInputOutputArray) -> Result<bool> {
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
		#[inline]
		fn draw_orientation(&mut self, image: &mut impl core::ToInputOutputArray, tvec: &impl core::ToInputArray, rvec: &impl core::ToInputArray, camera_matrix: &impl core::ToInputArray, dist_coeffs: &impl core::ToInputArray, axis_length: f64, axis_width: i32) -> Result<()> {
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
		/// This alternative version of [draw_orientation] function uses the following default values for its arguments:
		/// * axis_length: 3
		/// * axis_width: 2
		#[inline]
		fn draw_orientation_def(&mut self, image: &mut impl core::ToInputOutputArray, tvec: &impl core::ToInputArray, rvec: &impl core::ToInputArray, camera_matrix: &impl core::ToInputArray, dist_coeffs: &impl core::ToInputArray) -> Result<()> {
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
	
	pub struct CustomPattern {
		ptr: *mut c_void
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
	
	impl crate::ccalib::CustomPatternTraitConst for CustomPattern {
		#[inline] fn as_raw_CustomPattern(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::ccalib::CustomPatternTrait for CustomPattern {
		#[inline] fn as_raw_mut_CustomPattern(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CustomPattern {
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
	pub trait MultiCameraCalibrationTraitConst {
		fn as_raw_MultiCameraCalibration(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::ccalib::MultiCameraCalibration]
	pub trait MultiCameraCalibrationTrait: crate::ccalib::MultiCameraCalibrationTraitConst {
		fn as_raw_mut_MultiCameraCalibration(&mut self) -> *mut c_void;
	
		#[inline]
		fn load_images(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_multicalib_MultiCameraCalibration_loadImages(self.as_raw_mut_MultiCameraCalibration(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn initialize(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_multicalib_MultiCameraCalibration_initialize(self.as_raw_mut_MultiCameraCalibration(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn optimize_extrinsics(&mut self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_multicalib_MultiCameraCalibration_optimizeExtrinsics(self.as_raw_mut_MultiCameraCalibration(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn run(&mut self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_multicalib_MultiCameraCalibration_run(self.as_raw_mut_MultiCameraCalibration(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	pub struct MultiCameraCalibration {
		ptr: *mut c_void
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
	
	impl MultiCameraCalibration {
		/// ## C++ default parameters
		/// * verbose: 0
		/// * show_extration: 0
		/// * n_mini_matches: 20
		/// * flags: 0
		/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,200,1e-7)
		/// * detector: AKAZE::create(AKAZE::DESCRIPTOR_MLDB,0,3,0.006f)
		/// * descriptor: AKAZE::create(AKAZE::DESCRIPTOR_MLDB,0,3,0.006f)
		/// * matcher: DescriptorMatcher::create("BruteForce-L1")
		#[inline]
		pub fn new(camera_type: i32, n_cameras: i32, file_name: &str, pattern_width: f32, pattern_height: f32, verbose: i32, show_extration: i32, n_mini_matches: i32, flags: i32, criteria: core::TermCriteria, mut detector: core::Ptr<crate::features2d::Feature2D>, mut descriptor: core::Ptr<crate::features2d::Feature2D>, mut matcher: core::Ptr<crate::features2d::DescriptorMatcher>) -> Result<crate::ccalib::MultiCameraCalibration> {
			extern_container_arg!(file_name);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_multicalib_MultiCameraCalibration_MultiCameraCalibration_int_int_const_stringR_float_float_int_int_int_int_TermCriteria_PtrLFeature2DG_PtrLFeature2DG_PtrLDescriptorMatcherG(camera_type, n_cameras, file_name.opencv_as_extern(), pattern_width, pattern_height, verbose, show_extration, n_mini_matches, flags, criteria.opencv_as_extern(), detector.as_raw_mut_PtrOfFeature2D(), descriptor.as_raw_mut_PtrOfFeature2D(), matcher.as_raw_mut_PtrOfDescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
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
		/// * detector: AKAZE::create(AKAZE::DESCRIPTOR_MLDB,0,3,0.006f)
		/// * descriptor: AKAZE::create(AKAZE::DESCRIPTOR_MLDB,0,3,0.006f)
		/// * matcher: DescriptorMatcher::create("BruteForce-L1")
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
	pub trait MultiCameraCalibration_edgeTraitConst {
		fn as_raw_MultiCameraCalibration_edge(&self) -> *const c_void;
	
		#[inline]
		fn camera_vertex(&self) -> i32 {
			let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_propCameraVertex_const(self.as_raw_MultiCameraCalibration_edge()) };
			ret
		}
		
		#[inline]
		fn photo_vertex(&self) -> i32 {
			let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_propPhotoVertex_const(self.as_raw_MultiCameraCalibration_edge()) };
			ret
		}
		
		#[inline]
		fn photo_index(&self) -> i32 {
			let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_propPhotoIndex_const(self.as_raw_MultiCameraCalibration_edge()) };
			ret
		}
		
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
	
		#[inline]
		fn set_camera_vertex(&mut self, val: i32) {
			let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_propCameraVertex_int(self.as_raw_mut_MultiCameraCalibration_edge(), val) };
			ret
		}
		
		#[inline]
		fn set_photo_vertex(&mut self, val: i32) {
			let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_propPhotoVertex_int(self.as_raw_mut_MultiCameraCalibration_edge(), val) };
			ret
		}
		
		#[inline]
		fn set_photo_index(&mut self, val: i32) {
			let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_propPhotoIndex_int(self.as_raw_mut_MultiCameraCalibration_edge(), val) };
			ret
		}
		
		#[inline]
		fn set_transform(&mut self, mut val: core::Mat) {
			let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_edge_propTransform_Mat(self.as_raw_mut_MultiCameraCalibration_edge(), val.as_raw_mut_Mat()) };
			ret
		}
		
	}
	
	pub struct MultiCameraCalibration_edge {
		ptr: *mut c_void
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
	
	impl MultiCameraCalibration_edge {
		#[inline]
		pub fn new(cv: i32, pv: i32, pi: i32, mut trans: core::Mat) -> Result<crate::ccalib::MultiCameraCalibration_edge> {
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
	pub trait MultiCameraCalibration_vertexTraitConst {
		fn as_raw_MultiCameraCalibration_vertex(&self) -> *const c_void;
	
		#[inline]
		fn pose(&self) -> core::Mat {
			let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_vertex_propPose_const(self.as_raw_MultiCameraCalibration_vertex()) };
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn timestamp(&self) -> i32 {
			let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_vertex_propTimestamp_const(self.as_raw_MultiCameraCalibration_vertex()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::ccalib::MultiCameraCalibration_vertex]
	pub trait MultiCameraCalibration_vertexTrait: crate::ccalib::MultiCameraCalibration_vertexTraitConst {
		fn as_raw_mut_MultiCameraCalibration_vertex(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_pose(&mut self, mut val: core::Mat) {
			let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_vertex_propPose_Mat(self.as_raw_mut_MultiCameraCalibration_vertex(), val.as_raw_mut_Mat()) };
			ret
		}
		
		#[inline]
		fn set_timestamp(&mut self, val: i32) {
			let ret = unsafe { sys::cv_multicalib_MultiCameraCalibration_vertex_propTimestamp_int(self.as_raw_mut_MultiCameraCalibration_vertex(), val) };
			ret
		}
		
	}
	
	pub struct MultiCameraCalibration_vertex {
		ptr: *mut c_void
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
	
	impl MultiCameraCalibration_vertex {
		#[inline]
		pub fn new(mut po: core::Mat, ts: i32) -> Result<crate::ccalib::MultiCameraCalibration_vertex> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_multicalib_MultiCameraCalibration_vertex_vertex_Mat_int(po.as_raw_mut_Mat(), ts, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::ccalib::MultiCameraCalibration_vertex::opencv_from_extern(ret) };
			Ok(ret)
		}
		
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
	pub trait RandomPatternCornerFinderTraitConst {
		fn as_raw_RandomPatternCornerFinder(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::ccalib::RandomPatternCornerFinder]
	pub trait RandomPatternCornerFinderTrait: crate::ccalib::RandomPatternCornerFinderTraitConst {
		fn as_raw_mut_RandomPatternCornerFinder(&mut self) -> *mut c_void;
	
		#[inline]
		fn load_pattern(&mut self, pattern_image: &core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_randpattern_RandomPatternCornerFinder_loadPattern_const_MatR(self.as_raw_mut_RandomPatternCornerFinder(), pattern_image.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn load_pattern_1(&mut self, pattern_image: &core::Mat, pattern_key_points: &core::Vector<core::KeyPoint>, pattern_descriptors: &core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_randpattern_RandomPatternCornerFinder_loadPattern_const_MatR_const_vectorLKeyPointGR_const_MatR(self.as_raw_mut_RandomPatternCornerFinder(), pattern_image.as_raw_Mat(), pattern_key_points.as_raw_VectorOfKeyPoint(), pattern_descriptors.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn compute_object_image_points(&mut self, mut input_images: core::Vector<core::Mat>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_randpattern_RandomPatternCornerFinder_computeObjectImagePoints_vectorLMatG(self.as_raw_mut_RandomPatternCornerFinder(), input_images.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn compute_object_image_points_for_single(&mut self, mut input_image: core::Mat) -> Result<core::Vector<core::Mat>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_randpattern_RandomPatternCornerFinder_computeObjectImagePointsForSingle_Mat(self.as_raw_mut_RandomPatternCornerFinder(), input_image.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_object_points(&mut self) -> Result<core::Vector<core::Mat>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_randpattern_RandomPatternCornerFinder_getObjectPoints(self.as_raw_mut_RandomPatternCornerFinder(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
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
	pub struct RandomPatternCornerFinder {
		ptr: *mut c_void
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
	
	impl RandomPatternCornerFinder {
		/// ## C++ default parameters
		/// * nmini_match: 20
		/// * depth: CV_32F
		/// * verbose: 0
		/// * show_extraction: 0
		/// * detector: AKAZE::create(AKAZE::DESCRIPTOR_MLDB,0,3,0.005f)
		/// * descriptor: AKAZE::create(AKAZE::DESCRIPTOR_MLDB,0,3,0.005f)
		/// * matcher: DescriptorMatcher::create("BruteForce-L1")
		#[inline]
		pub fn new(pattern_width: f32, pattern_height: f32, nmini_match: i32, depth: i32, verbose: i32, show_extraction: i32, mut detector: core::Ptr<crate::features2d::Feature2D>, mut descriptor: core::Ptr<crate::features2d::Feature2D>, mut matcher: core::Ptr<crate::features2d::DescriptorMatcher>) -> Result<crate::ccalib::RandomPatternCornerFinder> {
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
		/// * detector: AKAZE::create(AKAZE::DESCRIPTOR_MLDB,0,3,0.005f)
		/// * descriptor: AKAZE::create(AKAZE::DESCRIPTOR_MLDB,0,3,0.005f)
		/// * matcher: DescriptorMatcher::create("BruteForce-L1")
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
	pub trait RandomPatternGeneratorTraitConst {
		fn as_raw_RandomPatternGenerator(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::ccalib::RandomPatternGenerator]
	pub trait RandomPatternGeneratorTrait: crate::ccalib::RandomPatternGeneratorTraitConst {
		fn as_raw_mut_RandomPatternGenerator(&mut self) -> *mut c_void;
	
		#[inline]
		fn generate_pattern(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_randpattern_RandomPatternGenerator_generatePattern(self.as_raw_mut_RandomPatternGenerator(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	
	pub struct RandomPatternGenerator {
		ptr: *mut c_void
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
	
	impl RandomPatternGenerator {
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
}
