pub mod aruco {
	//! # Aruco markers, module functionality was moved to objdetect module
	//! ArUco Marker Detection, module functionality was moved to objdetect module
	//! ## See also
	//! ArucoDetector, CharucoDetector, Board, GridBoard, CharucoBoard
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::EstimateParametersTraitConst, super::EstimateParametersTrait };
	}
	
	/// The marker coordinate system is centered on the middle of the marker.
	/// 
	/// The coordinates of the four corners (CCW order) of the marker in its own coordinate system are:
	/// (-markerLength/2, markerLength/2, 0), (markerLength/2, markerLength/2, 0),
	/// (markerLength/2, -markerLength/2, 0), (-markerLength/2, -markerLength/2, 0).
	/// 
	/// These pattern points define this coordinate system:
	/// ![Image with axes drawn](https://docs.opencv.org/4.8.1/singlemarkersaxes2.jpg)
	pub const ARUCO_CCW_CENTER: i32 = 0;
	/// The marker coordinate system is centered on the top-left corner of the marker.
	/// 
	/// The coordinates of the four corners (CW order) of the marker in its own coordinate system are:
	/// (0, 0, 0), (markerLength, 0, 0),
	/// (markerLength, markerLength, 0), (0, markerLength, 0).
	/// 
	/// These pattern points define this coordinate system:
	/// ![Image with axes drawn](https://docs.opencv.org/4.8.1/singlemarkersaxes.jpg)
	/// 
	/// These pattern dots are convenient to use with a chessboard/ChArUco board.
	pub const ARUCO_CW_TOP_LEFT_CORNER: i32 = 1;
	/// rvec/tvec define the right handed coordinate system of the marker.
	/// 
	/// PatternPositionType defines center this system and axes direction.
	/// Axis X (red color) - first coordinate, axis Y (green color) - second coordinate,
	/// axis Z (blue color) - third coordinate.
	/// ## See also
	/// estimatePoseSingleMarkers(), check tutorial_aruco_detection in aruco contrib
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum PatternPositionType {
		/// The marker coordinate system is centered on the middle of the marker.
		/// 
		/// The coordinates of the four corners (CCW order) of the marker in its own coordinate system are:
		/// (-markerLength/2, markerLength/2, 0), (markerLength/2, markerLength/2, 0),
		/// (markerLength/2, -markerLength/2, 0), (-markerLength/2, -markerLength/2, 0).
		/// 
		/// These pattern points define this coordinate system:
		/// ![Image with axes drawn](https://docs.opencv.org/4.8.1/singlemarkersaxes2.jpg)
		ARUCO_CCW_CENTER = 0,
		/// The marker coordinate system is centered on the top-left corner of the marker.
		/// 
		/// The coordinates of the four corners (CW order) of the marker in its own coordinate system are:
		/// (0, 0, 0), (markerLength, 0, 0),
		/// (markerLength, markerLength, 0), (0, markerLength, 0).
		/// 
		/// These pattern points define this coordinate system:
		/// ![Image with axes drawn](https://docs.opencv.org/4.8.1/singlemarkersaxes.jpg)
		/// 
		/// These pattern dots are convenient to use with a chessboard/ChArUco board.
		ARUCO_CW_TOP_LEFT_CORNER = 1,
	}
	
	opencv_type_enum! { crate::aruco::PatternPositionType }
	
	/// @overload
	/// It's the same function as [calibrate_camera_aruco] but without calibration error estimation.
	/// 
	/// ## Note
	/// This alternative version of [calibrate_camera_aruco] function uses the following default values for its arguments:
	/// * rvecs: noArray()
	/// * tvecs: noArray()
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
	#[inline]
	pub fn calibrate_camera_aruco_def(corners: &impl core::ToInputArray, ids: &impl core::ToInputArray, counter: &impl core::ToInputArray, board: &core::Ptr<crate::objdetect::Board>, image_size: core::Size, camera_matrix: &mut impl core::ToInputOutputArray, dist_coeffs: &mut impl core::ToInputOutputArray) -> Result<f64> {
		input_array_arg!(corners);
		input_array_arg!(ids);
		input_array_arg!(counter);
		input_output_array_arg!(camera_matrix);
		input_output_array_arg!(dist_coeffs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR(corners.as_raw__InputArray(), ids.as_raw__InputArray(), counter.as_raw__InputArray(), board.as_raw_PtrOfBoard(), image_size.opencv_as_extern(), camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Calibrate a camera using aruco markers
	/// 
	/// ## Parameters
	/// * corners: vector of detected marker corners in all frames.
	/// The corners should have the same format returned by detectMarkers (see #detectMarkers).
	/// * ids: list of identifiers for each marker in corners
	/// * counter: number of markers in each frame so that corners and ids can be split
	/// * board: Marker Board layout
	/// * imageSize: Size of the image used only to initialize the intrinsic camera matrix.
	/// * cameraMatrix: Output 3x3 floating-point camera matrix
	/// ![inline formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) . If CV\_CALIB\_USE\_INTRINSIC\_GUESS
	/// and/or CV_CALIB_FIX_ASPECT_RATIO are specified, some or all of fx, fy, cx, cy must be
	/// initialized before calling the function.
	/// * distCoeffs: Output vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5D%2C%5Bs%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5D%5D%29) of 4, 5, 8 or 12 elements
	/// * rvecs: Output vector of rotation vectors (see Rodrigues ) estimated for each board view
	/// (e.g. std::vector<cv::Mat>>). That is, each k-th rotation vector together with the corresponding
	/// k-th translation vector (see the next output parameter description) brings the board pattern
	/// from the model coordinate space (in which object points are specified) to the world coordinate
	/// space, that is, a real position of the board pattern in the k-th pattern view (k=0.. *M* -1).
	/// * tvecs: Output vector of translation vectors estimated for each pattern view.
	/// * stdDeviationsIntrinsics: Output vector of standard deviations estimated for intrinsic parameters.
	/// Order of deviations values:
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28f%5Fx%2C%20f%5Fy%2C%20c%5Fx%2C%20c%5Fy%2C%20k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%2C%20k%5F3%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%20%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%0As%5F4%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%29) If one of parameters is not estimated, it's deviation is equals to zero.
	/// * stdDeviationsExtrinsics: Output vector of standard deviations estimated for extrinsic parameters.
	/// Order of deviations values: ![inline formula](https://latex.codecogs.com/png.latex?%28R%5F1%2C%20T%5F1%2C%20%5Cdotsc%20%2C%20R%5FM%2C%20T%5FM%29) where M is number of pattern views,
	/// ![inline formula](https://latex.codecogs.com/png.latex?R%5Fi%2C%20T%5Fi) are concatenated 1x3 vectors.
	/// * perViewErrors: Output vector of average re-projection errors estimated for each pattern view.
	/// * flags: flags Different flags  for the calibration process (see [calibrate_camera] for details).
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	/// 
	/// This function calibrates a camera using an Aruco Board. The function receives a list of
	/// detected markers from several views of the Board. The process is similar to the chessboard
	/// calibration in calibrateCamera(). The function returns the final re-projection error.
	/// 
	/// ## Note
	/// This alternative version of [calibrate_camera_aruco_extended] function uses the following default values for its arguments:
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
	#[inline]
	pub fn calibrate_camera_aruco_extended_def(corners: &impl core::ToInputArray, ids: &impl core::ToInputArray, counter: &impl core::ToInputArray, board: &core::Ptr<crate::objdetect::Board>, image_size: core::Size, camera_matrix: &mut impl core::ToInputOutputArray, dist_coeffs: &mut impl core::ToInputOutputArray, rvecs: &mut impl core::ToOutputArray, tvecs: &mut impl core::ToOutputArray, std_deviations_intrinsics: &mut impl core::ToOutputArray, std_deviations_extrinsics: &mut impl core::ToOutputArray, per_view_errors: &mut impl core::ToOutputArray) -> Result<f64> {
		input_array_arg!(corners);
		input_array_arg!(ids);
		input_array_arg!(counter);
		input_output_array_arg!(camera_matrix);
		input_output_array_arg!(dist_coeffs);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		output_array_arg!(std_deviations_intrinsics);
		output_array_arg!(std_deviations_extrinsics);
		output_array_arg!(per_view_errors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(corners.as_raw__InputArray(), ids.as_raw__InputArray(), counter.as_raw__InputArray(), board.as_raw_PtrOfBoard(), image_size.opencv_as_extern(), camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), std_deviations_intrinsics.as_raw__OutputArray(), std_deviations_extrinsics.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Calibrate a camera using aruco markers
	/// 
	/// ## Parameters
	/// * corners: vector of detected marker corners in all frames.
	/// The corners should have the same format returned by detectMarkers (see #detectMarkers).
	/// * ids: list of identifiers for each marker in corners
	/// * counter: number of markers in each frame so that corners and ids can be split
	/// * board: Marker Board layout
	/// * imageSize: Size of the image used only to initialize the intrinsic camera matrix.
	/// * cameraMatrix: Output 3x3 floating-point camera matrix
	/// ![inline formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) . If CV\_CALIB\_USE\_INTRINSIC\_GUESS
	/// and/or CV_CALIB_FIX_ASPECT_RATIO are specified, some or all of fx, fy, cx, cy must be
	/// initialized before calling the function.
	/// * distCoeffs: Output vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5D%2C%5Bs%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5D%5D%29) of 4, 5, 8 or 12 elements
	/// * rvecs: Output vector of rotation vectors (see Rodrigues ) estimated for each board view
	/// (e.g. std::vector<cv::Mat>>). That is, each k-th rotation vector together with the corresponding
	/// k-th translation vector (see the next output parameter description) brings the board pattern
	/// from the model coordinate space (in which object points are specified) to the world coordinate
	/// space, that is, a real position of the board pattern in the k-th pattern view (k=0.. *M* -1).
	/// * tvecs: Output vector of translation vectors estimated for each pattern view.
	/// * stdDeviationsIntrinsics: Output vector of standard deviations estimated for intrinsic parameters.
	/// Order of deviations values:
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28f%5Fx%2C%20f%5Fy%2C%20c%5Fx%2C%20c%5Fy%2C%20k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%2C%20k%5F3%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%20%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%0As%5F4%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%29) If one of parameters is not estimated, it's deviation is equals to zero.
	/// * stdDeviationsExtrinsics: Output vector of standard deviations estimated for extrinsic parameters.
	/// Order of deviations values: ![inline formula](https://latex.codecogs.com/png.latex?%28R%5F1%2C%20T%5F1%2C%20%5Cdotsc%20%2C%20R%5FM%2C%20T%5FM%29) where M is number of pattern views,
	/// ![inline formula](https://latex.codecogs.com/png.latex?R%5Fi%2C%20T%5Fi) are concatenated 1x3 vectors.
	/// * perViewErrors: Output vector of average re-projection errors estimated for each pattern view.
	/// * flags: flags Different flags  for the calibration process (see [calibrate_camera] for details).
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	/// 
	/// This function calibrates a camera using an Aruco Board. The function receives a list of
	/// detected markers from several views of the Board. The process is similar to the chessboard
	/// calibration in calibrateCamera(). The function returns the final re-projection error.
	/// 
	/// ## C++ default parameters
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
	#[inline]
	pub fn calibrate_camera_aruco_extended(corners: &impl core::ToInputArray, ids: &impl core::ToInputArray, counter: &impl core::ToInputArray, board: &core::Ptr<crate::objdetect::Board>, image_size: core::Size, camera_matrix: &mut impl core::ToInputOutputArray, dist_coeffs: &mut impl core::ToInputOutputArray, rvecs: &mut impl core::ToOutputArray, tvecs: &mut impl core::ToOutputArray, std_deviations_intrinsics: &mut impl core::ToOutputArray, std_deviations_extrinsics: &mut impl core::ToOutputArray, per_view_errors: &mut impl core::ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
		input_array_arg!(corners);
		input_array_arg!(ids);
		input_array_arg!(counter);
		input_output_array_arg!(camera_matrix);
		input_output_array_arg!(dist_coeffs);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		output_array_arg!(std_deviations_intrinsics);
		output_array_arg!(std_deviations_extrinsics);
		output_array_arg!(per_view_errors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_TermCriteriaR(corners.as_raw__InputArray(), ids.as_raw__InputArray(), counter.as_raw__InputArray(), board.as_raw_PtrOfBoard(), image_size.opencv_as_extern(), camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), std_deviations_intrinsics.as_raw__OutputArray(), std_deviations_extrinsics.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Calibrate a camera using aruco markers
	/// 
	/// ## Parameters
	/// * corners: vector of detected marker corners in all frames.
	/// The corners should have the same format returned by detectMarkers (see #detectMarkers).
	/// * ids: list of identifiers for each marker in corners
	/// * counter: number of markers in each frame so that corners and ids can be split
	/// * board: Marker Board layout
	/// * imageSize: Size of the image used only to initialize the intrinsic camera matrix.
	/// * cameraMatrix: Output 3x3 floating-point camera matrix
	/// ![inline formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) . If CV\_CALIB\_USE\_INTRINSIC\_GUESS
	/// and/or CV_CALIB_FIX_ASPECT_RATIO are specified, some or all of fx, fy, cx, cy must be
	/// initialized before calling the function.
	/// * distCoeffs: Output vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5D%2C%5Bs%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5D%5D%29) of 4, 5, 8 or 12 elements
	/// * rvecs: Output vector of rotation vectors (see Rodrigues ) estimated for each board view
	/// (e.g. std::vector<cv::Mat>>). That is, each k-th rotation vector together with the corresponding
	/// k-th translation vector (see the next output parameter description) brings the board pattern
	/// from the model coordinate space (in which object points are specified) to the world coordinate
	/// space, that is, a real position of the board pattern in the k-th pattern view (k=0.. *M* -1).
	/// * tvecs: Output vector of translation vectors estimated for each pattern view.
	/// * stdDeviationsIntrinsics: Output vector of standard deviations estimated for intrinsic parameters.
	/// Order of deviations values:
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28f%5Fx%2C%20f%5Fy%2C%20c%5Fx%2C%20c%5Fy%2C%20k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%2C%20k%5F3%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%20%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%0As%5F4%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%29) If one of parameters is not estimated, it's deviation is equals to zero.
	/// * stdDeviationsExtrinsics: Output vector of standard deviations estimated for extrinsic parameters.
	/// Order of deviations values: ![inline formula](https://latex.codecogs.com/png.latex?%28R%5F1%2C%20T%5F1%2C%20%5Cdotsc%20%2C%20R%5FM%2C%20T%5FM%29) where M is number of pattern views,
	/// ![inline formula](https://latex.codecogs.com/png.latex?R%5Fi%2C%20T%5Fi) are concatenated 1x3 vectors.
	/// * perViewErrors: Output vector of average re-projection errors estimated for each pattern view.
	/// * flags: flags Different flags  for the calibration process (see [calibrate_camera] for details).
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	/// 
	/// This function calibrates a camera using an Aruco Board. The function receives a list of
	/// detected markers from several views of the Board. The process is similar to the chessboard
	/// calibration in calibrateCamera(). The function returns the final re-projection error.
	/// 
	/// ## Overloaded parameters
	/// 
	///  * It's the same function as [calibrate_camera_aruco] but without calibration error estimation.
	/// 
	/// ## C++ default parameters
	/// * rvecs: noArray()
	/// * tvecs: noArray()
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
	#[inline]
	pub fn calibrate_camera_aruco(corners: &impl core::ToInputArray, ids: &impl core::ToInputArray, counter: &impl core::ToInputArray, board: &core::Ptr<crate::objdetect::Board>, image_size: core::Size, camera_matrix: &mut impl core::ToInputOutputArray, dist_coeffs: &mut impl core::ToInputOutputArray, rvecs: &mut impl core::ToOutputArray, tvecs: &mut impl core::ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
		input_array_arg!(corners);
		input_array_arg!(ids);
		input_array_arg!(counter);
		input_output_array_arg!(camera_matrix);
		input_output_array_arg!(dist_coeffs);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_TermCriteriaR(corners.as_raw__InputArray(), ids.as_raw__InputArray(), counter.as_raw__InputArray(), board.as_raw_PtrOfBoard(), image_size.opencv_as_extern(), camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// It's the same function as [calibrate_camera_charuco] but without calibration error estimation.
	/// 
	/// ## Note
	/// This alternative version of [calibrate_camera_charuco] function uses the following default values for its arguments:
	/// * rvecs: noArray()
	/// * tvecs: noArray()
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
	#[inline]
	pub fn calibrate_camera_charuco_def(charuco_corners: &impl core::ToInputArray, charuco_ids: &impl core::ToInputArray, board: &core::Ptr<crate::objdetect::CharucoBoard>, image_size: core::Size, camera_matrix: &mut impl core::ToInputOutputArray, dist_coeffs: &mut impl core::ToInputOutputArray) -> Result<f64> {
		input_array_arg!(charuco_corners);
		input_array_arg!(charuco_ids);
		input_output_array_arg!(camera_matrix);
		input_output_array_arg!(dist_coeffs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR(charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), image_size.opencv_as_extern(), camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Calibrate a camera using Charuco corners
	/// 
	/// ## Parameters
	/// * charucoCorners: vector of detected charuco corners per frame
	/// * charucoIds: list of identifiers for each corner in charucoCorners per frame
	/// * board: Marker Board layout
	/// * imageSize: input image size
	/// * cameraMatrix: Output 3x3 floating-point camera matrix
	/// ![inline formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) . If CV\_CALIB\_USE\_INTRINSIC\_GUESS
	/// and/or CV_CALIB_FIX_ASPECT_RATIO are specified, some or all of fx, fy, cx, cy must be
	/// initialized before calling the function.
	/// * distCoeffs: Output vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5D%2C%5Bs%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5D%5D%29) of 4, 5, 8 or 12 elements
	/// * rvecs: Output vector of rotation vectors (see Rodrigues ) estimated for each board view
	/// (e.g. std::vector<cv::Mat>>). That is, each k-th rotation vector together with the corresponding
	/// k-th translation vector (see the next output parameter description) brings the board pattern
	/// from the model coordinate space (in which object points are specified) to the world coordinate
	/// space, that is, a real position of the board pattern in the k-th pattern view (k=0.. *M* -1).
	/// * tvecs: Output vector of translation vectors estimated for each pattern view.
	/// * stdDeviationsIntrinsics: Output vector of standard deviations estimated for intrinsic parameters.
	/// Order of deviations values:
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28f%5Fx%2C%20f%5Fy%2C%20c%5Fx%2C%20c%5Fy%2C%20k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%2C%20k%5F3%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%20%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%0As%5F4%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%29) If one of parameters is not estimated, it's deviation is equals to zero.
	/// * stdDeviationsExtrinsics: Output vector of standard deviations estimated for extrinsic parameters.
	/// Order of deviations values: ![inline formula](https://latex.codecogs.com/png.latex?%28R%5F1%2C%20T%5F1%2C%20%5Cdotsc%20%2C%20R%5FM%2C%20T%5FM%29) where M is number of pattern views,
	/// ![inline formula](https://latex.codecogs.com/png.latex?R%5Fi%2C%20T%5Fi) are concatenated 1x3 vectors.
	/// * perViewErrors: Output vector of average re-projection errors estimated for each pattern view.
	/// * flags: flags Different flags  for the calibration process (see [calibrate_camera] for details).
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	/// 
	/// This function calibrates a camera using a set of corners of a  Charuco Board. The function
	/// receives a list of detected corners and its identifiers from several views of the Board.
	/// The function returns the final re-projection error.
	/// 
	/// ## Note
	/// This alternative version of [calibrate_camera_charuco_extended] function uses the following default values for its arguments:
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
	#[inline]
	pub fn calibrate_camera_charuco_extended_def(charuco_corners: &impl core::ToInputArray, charuco_ids: &impl core::ToInputArray, board: &core::Ptr<crate::objdetect::CharucoBoard>, image_size: core::Size, camera_matrix: &mut impl core::ToInputOutputArray, dist_coeffs: &mut impl core::ToInputOutputArray, rvecs: &mut impl core::ToOutputArray, tvecs: &mut impl core::ToOutputArray, std_deviations_intrinsics: &mut impl core::ToOutputArray, std_deviations_extrinsics: &mut impl core::ToOutputArray, per_view_errors: &mut impl core::ToOutputArray) -> Result<f64> {
		input_array_arg!(charuco_corners);
		input_array_arg!(charuco_ids);
		input_output_array_arg!(camera_matrix);
		input_output_array_arg!(dist_coeffs);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		output_array_arg!(std_deviations_intrinsics);
		output_array_arg!(std_deviations_extrinsics);
		output_array_arg!(per_view_errors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), image_size.opencv_as_extern(), camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), std_deviations_intrinsics.as_raw__OutputArray(), std_deviations_extrinsics.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Calibrate a camera using Charuco corners
	/// 
	/// ## Parameters
	/// * charucoCorners: vector of detected charuco corners per frame
	/// * charucoIds: list of identifiers for each corner in charucoCorners per frame
	/// * board: Marker Board layout
	/// * imageSize: input image size
	/// * cameraMatrix: Output 3x3 floating-point camera matrix
	/// ![inline formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) . If CV\_CALIB\_USE\_INTRINSIC\_GUESS
	/// and/or CV_CALIB_FIX_ASPECT_RATIO are specified, some or all of fx, fy, cx, cy must be
	/// initialized before calling the function.
	/// * distCoeffs: Output vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5D%2C%5Bs%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5D%5D%29) of 4, 5, 8 or 12 elements
	/// * rvecs: Output vector of rotation vectors (see Rodrigues ) estimated for each board view
	/// (e.g. std::vector<cv::Mat>>). That is, each k-th rotation vector together with the corresponding
	/// k-th translation vector (see the next output parameter description) brings the board pattern
	/// from the model coordinate space (in which object points are specified) to the world coordinate
	/// space, that is, a real position of the board pattern in the k-th pattern view (k=0.. *M* -1).
	/// * tvecs: Output vector of translation vectors estimated for each pattern view.
	/// * stdDeviationsIntrinsics: Output vector of standard deviations estimated for intrinsic parameters.
	/// Order of deviations values:
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28f%5Fx%2C%20f%5Fy%2C%20c%5Fx%2C%20c%5Fy%2C%20k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%2C%20k%5F3%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%20%2C%20s%5F1%2C%20s%5F2%2C%20s%5F3%2C%0As%5F4%2C%20%5Ctau%5Fx%2C%20%5Ctau%5Fy%29) If one of parameters is not estimated, it's deviation is equals to zero.
	/// * stdDeviationsExtrinsics: Output vector of standard deviations estimated for extrinsic parameters.
	/// Order of deviations values: ![inline formula](https://latex.codecogs.com/png.latex?%28R%5F1%2C%20T%5F1%2C%20%5Cdotsc%20%2C%20R%5FM%2C%20T%5FM%29) where M is number of pattern views,
	/// ![inline formula](https://latex.codecogs.com/png.latex?R%5Fi%2C%20T%5Fi) are concatenated 1x3 vectors.
	/// * perViewErrors: Output vector of average re-projection errors estimated for each pattern view.
	/// * flags: flags Different flags  for the calibration process (see [calibrate_camera] for details).
	/// * criteria: Termination criteria for the iterative optimization algorithm.
	/// 
	/// This function calibrates a camera using a set of corners of a  Charuco Board. The function
	/// receives a list of detected corners and its identifiers from several views of the Board.
	/// The function returns the final re-projection error.
	/// 
	/// ## C++ default parameters
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
	#[inline]
	pub fn calibrate_camera_charuco_extended(charuco_corners: &impl core::ToInputArray, charuco_ids: &impl core::ToInputArray, board: &core::Ptr<crate::objdetect::CharucoBoard>, image_size: core::Size, camera_matrix: &mut impl core::ToInputOutputArray, dist_coeffs: &mut impl core::ToInputOutputArray, rvecs: &mut impl core::ToOutputArray, tvecs: &mut impl core::ToOutputArray, std_deviations_intrinsics: &mut impl core::ToOutputArray, std_deviations_extrinsics: &mut impl core::ToOutputArray, per_view_errors: &mut impl core::ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
		input_array_arg!(charuco_corners);
		input_array_arg!(charuco_ids);
		input_output_array_arg!(camera_matrix);
		input_output_array_arg!(dist_coeffs);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		output_array_arg!(std_deviations_intrinsics);
		output_array_arg!(std_deviations_extrinsics);
		output_array_arg!(per_view_errors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_TermCriteriaR(charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), image_size.opencv_as_extern(), camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), std_deviations_intrinsics.as_raw__OutputArray(), std_deviations_extrinsics.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// It's the same function as [calibrate_camera_charuco] but without calibration error estimation.
	/// 
	/// ## C++ default parameters
	/// * rvecs: noArray()
	/// * tvecs: noArray()
	/// * flags: 0
	/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
	#[inline]
	pub fn calibrate_camera_charuco(charuco_corners: &impl core::ToInputArray, charuco_ids: &impl core::ToInputArray, board: &core::Ptr<crate::objdetect::CharucoBoard>, image_size: core::Size, camera_matrix: &mut impl core::ToInputOutputArray, dist_coeffs: &mut impl core::ToInputOutputArray, rvecs: &mut impl core::ToOutputArray, tvecs: &mut impl core::ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
		input_array_arg!(charuco_corners);
		input_array_arg!(charuco_ids);
		input_output_array_arg!(camera_matrix);
		input_output_array_arg!(dist_coeffs);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_TermCriteriaR(charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), image_size.opencv_as_extern(), camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Detect ChArUco Diamond markers
	/// 
	/// ## Parameters
	/// * image: input image necessary for corner subpixel.
	/// * markerCorners: list of detected marker corners from detectMarkers function.
	/// * markerIds: list of marker ids in markerCorners.
	/// * squareMarkerLengthRate: rate between square and marker length:
	/// squareMarkerLengthRate = squareLength/markerLength. The real units are not necessary.
	/// * diamondCorners: output list of detected diamond corners (4 corners per diamond). The order
	/// is the same than in marker corners: top left, top right, bottom right and bottom left. Similar
	/// format than the corners returned by detectMarkers (e.g std::vector<std::vector<cv::Point2f> > ).
	/// * diamondIds: ids of the diamonds in diamondCorners. The id of each diamond is in fact of
	/// type Vec4i, so each diamond has 4 ids, which are the ids of the aruco markers composing the
	/// diamond.
	/// * cameraMatrix: Optional camera calibration matrix.
	/// * distCoeffs: Optional camera distortion coefficients.
	/// * dictionary: dictionary of markers indicating the type of markers.
	/// 
	/// This function detects Diamond markers from the previous detected ArUco markers. The diamonds
	/// are returned in the diamondCorners and diamondIds parameters. If camera calibration parameters
	/// are provided, the diamond search is based on reprojection. If not, diamond search is based on
	/// homography. Homography is faster than reprojection, but less accurate.
	/// 
	/// 
	/// **Deprecated**: Use CharucoDetector::detectDiamonds
	/// 
	/// ## Note
	/// This alternative version of [detect_charuco_diamond] function uses the following default values for its arguments:
	/// * camera_matrix: noArray()
	/// * dist_coeffs: noArray()
	/// * dictionary: makePtr<Dictionary>(getPredefinedDictionary(PredefinedDictionaryType::DICT_4X4_50))
	#[deprecated = "Use CharucoDetector::detectDiamonds"]
	#[inline]
	pub fn detect_charuco_diamond_def(image: &impl core::ToInputArray, marker_corners: &impl core::ToInputArray, marker_ids: &impl core::ToInputArray, square_marker_length_rate: f32, diamond_corners: &mut impl core::ToOutputArray, diamond_ids: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		input_array_arg!(marker_corners);
		input_array_arg!(marker_ids);
		output_array_arg!(diamond_corners);
		output_array_arg!(diamond_ids);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_detectCharucoDiamond_const__InputArrayR_const__InputArrayR_const__InputArrayR_float_const__OutputArrayR_const__OutputArrayR(image.as_raw__InputArray(), marker_corners.as_raw__InputArray(), marker_ids.as_raw__InputArray(), square_marker_length_rate, diamond_corners.as_raw__OutputArray(), diamond_ids.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Detect ChArUco Diamond markers
	/// 
	/// ## Parameters
	/// * image: input image necessary for corner subpixel.
	/// * markerCorners: list of detected marker corners from detectMarkers function.
	/// * markerIds: list of marker ids in markerCorners.
	/// * squareMarkerLengthRate: rate between square and marker length:
	/// squareMarkerLengthRate = squareLength/markerLength. The real units are not necessary.
	/// * diamondCorners: output list of detected diamond corners (4 corners per diamond). The order
	/// is the same than in marker corners: top left, top right, bottom right and bottom left. Similar
	/// format than the corners returned by detectMarkers (e.g std::vector<std::vector<cv::Point2f> > ).
	/// * diamondIds: ids of the diamonds in diamondCorners. The id of each diamond is in fact of
	/// type Vec4i, so each diamond has 4 ids, which are the ids of the aruco markers composing the
	/// diamond.
	/// * cameraMatrix: Optional camera calibration matrix.
	/// * distCoeffs: Optional camera distortion coefficients.
	/// * dictionary: dictionary of markers indicating the type of markers.
	/// 
	/// This function detects Diamond markers from the previous detected ArUco markers. The diamonds
	/// are returned in the diamondCorners and diamondIds parameters. If camera calibration parameters
	/// are provided, the diamond search is based on reprojection. If not, diamond search is based on
	/// homography. Homography is faster than reprojection, but less accurate.
	/// 
	/// 
	/// **Deprecated**: Use CharucoDetector::detectDiamonds
	/// 
	/// ## C++ default parameters
	/// * camera_matrix: noArray()
	/// * dist_coeffs: noArray()
	/// * dictionary: makePtr<Dictionary>(getPredefinedDictionary(PredefinedDictionaryType::DICT_4X4_50))
	#[deprecated = "Use CharucoDetector::detectDiamonds"]
	#[inline]
	pub fn detect_charuco_diamond(image: &impl core::ToInputArray, marker_corners: &impl core::ToInputArray, marker_ids: &impl core::ToInputArray, square_marker_length_rate: f32, diamond_corners: &mut impl core::ToOutputArray, diamond_ids: &mut impl core::ToOutputArray, camera_matrix: &impl core::ToInputArray, dist_coeffs: &impl core::ToInputArray, mut dictionary: core::Ptr<crate::objdetect::Dictionary>) -> Result<()> {
		input_array_arg!(image);
		input_array_arg!(marker_corners);
		input_array_arg!(marker_ids);
		output_array_arg!(diamond_corners);
		output_array_arg!(diamond_ids);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_detectCharucoDiamond_const__InputArrayR_const__InputArrayR_const__InputArrayR_float_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_PtrLDictionaryG(image.as_raw__InputArray(), marker_corners.as_raw__InputArray(), marker_ids.as_raw__InputArray(), square_marker_length_rate, diamond_corners.as_raw__OutputArray(), diamond_ids.as_raw__OutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), dictionary.as_raw_mut_PtrOfDictionary(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// detect markers
	/// 
	/// **Deprecated**: Use class ArucoDetector::detectMarkers
	/// 
	/// ## Note
	/// This alternative version of [detect_markers] function uses the following default values for its arguments:
	/// * parameters: makePtr<DetectorParameters>()
	/// * rejected_img_points: noArray()
	#[deprecated = "Use class ArucoDetector::detectMarkers"]
	#[inline]
	pub fn detect_markers_def(image: &impl core::ToInputArray, dictionary: &core::Ptr<crate::objdetect::Dictionary>, corners: &mut impl core::ToOutputArray, ids: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(corners);
		output_array_arg!(ids);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_detectMarkers_const__InputArrayR_const_PtrLDictionaryGR_const__OutputArrayR_const__OutputArrayR(image.as_raw__InputArray(), dictionary.as_raw_PtrOfDictionary(), corners.as_raw__OutputArray(), ids.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// detect markers
	/// 
	/// **Deprecated**: Use class ArucoDetector::detectMarkers
	/// 
	/// ## C++ default parameters
	/// * parameters: makePtr<DetectorParameters>()
	/// * rejected_img_points: noArray()
	#[deprecated = "Use class ArucoDetector::detectMarkers"]
	#[inline]
	pub fn detect_markers(image: &impl core::ToInputArray, dictionary: &core::Ptr<crate::objdetect::Dictionary>, corners: &mut impl core::ToOutputArray, ids: &mut impl core::ToOutputArray, parameters: &core::Ptr<crate::objdetect::DetectorParameters>, rejected_img_points: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(corners);
		output_array_arg!(ids);
		output_array_arg!(rejected_img_points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_detectMarkers_const__InputArrayR_const_PtrLDictionaryGR_const__OutputArrayR_const__OutputArrayR_const_PtrLDetectorParametersGR_const__OutputArrayR(image.as_raw__InputArray(), dictionary.as_raw_PtrOfDictionary(), corners.as_raw__OutputArray(), ids.as_raw__OutputArray(), parameters.as_raw_PtrOfDetectorParameters(), rejected_img_points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Draw a ChArUco Diamond marker
	/// 
	/// ## Parameters
	/// * dictionary: dictionary of markers indicating the type of markers.
	/// * ids: list of 4 ids for each ArUco marker in the ChArUco marker.
	/// * squareLength: size of the chessboard squares in pixels.
	/// * markerLength: size of the markers in pixels.
	/// * img: output image with the marker. The size of this image will be
	/// 3*squareLength + 2*marginSize,.
	/// * marginSize: minimum margins (in pixels) of the marker in the output image
	/// * borderBits: width of the marker borders.
	/// 
	/// This function return the image of a ChArUco marker, ready to be printed.
	/// 
	/// ## Note
	/// This alternative version of [draw_charuco_diamond] function uses the following default values for its arguments:
	/// * margin_size: 0
	/// * border_bits: 1
	#[inline]
	pub fn draw_charuco_diamond_def(dictionary: &core::Ptr<crate::objdetect::Dictionary>, ids: core::Vec4i, square_length: i32, marker_length: i32, img: &mut impl core::ToOutputArray) -> Result<()> {
		output_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_drawCharucoDiamond_const_PtrLDictionaryGR_Vec4i_int_int_const__OutputArrayR(dictionary.as_raw_PtrOfDictionary(), ids.opencv_as_extern(), square_length, marker_length, img.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Draw a ChArUco Diamond marker
	/// 
	/// ## Parameters
	/// * dictionary: dictionary of markers indicating the type of markers.
	/// * ids: list of 4 ids for each ArUco marker in the ChArUco marker.
	/// * squareLength: size of the chessboard squares in pixels.
	/// * markerLength: size of the markers in pixels.
	/// * img: output image with the marker. The size of this image will be
	/// 3*squareLength + 2*marginSize,.
	/// * marginSize: minimum margins (in pixels) of the marker in the output image
	/// * borderBits: width of the marker borders.
	/// 
	/// This function return the image of a ChArUco marker, ready to be printed.
	/// 
	/// ## C++ default parameters
	/// * margin_size: 0
	/// * border_bits: 1
	#[inline]
	pub fn draw_charuco_diamond(dictionary: &core::Ptr<crate::objdetect::Dictionary>, ids: core::Vec4i, square_length: i32, marker_length: i32, img: &mut impl core::ToOutputArray, margin_size: i32, border_bits: i32) -> Result<()> {
		output_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_drawCharucoDiamond_const_PtrLDictionaryGR_Vec4i_int_int_const__OutputArrayR_int_int(dictionary.as_raw_PtrOfDictionary(), ids.opencv_as_extern(), square_length, marker_length, img.as_raw__OutputArray(), margin_size, border_bits, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// draw planar board
	/// 
	/// **Deprecated**: Use Board::generateImage
	#[deprecated = "Use Board::generateImage"]
	#[inline]
	pub fn draw_planar_board(board: &core::Ptr<crate::objdetect::Board>, out_size: core::Size, img: &mut impl core::ToOutputArray, margin_size: i32, border_bits: i32) -> Result<()> {
		output_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_drawPlanarBoard_const_PtrLBoardGR_Size_const__OutputArrayR_int_int(board.as_raw_PtrOfBoard(), out_size.opencv_as_extern(), img.as_raw__OutputArray(), margin_size, border_bits, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// 
	/// **Deprecated**: Use cv::solvePnP
	/// 
	/// ## Note
	/// This alternative version of [estimate_pose_board] function uses the following default values for its arguments:
	/// * use_extrinsic_guess: false
	#[deprecated = "Use cv::solvePnP"]
	#[inline]
	pub fn estimate_pose_board_def(corners: &impl core::ToInputArray, ids: &impl core::ToInputArray, board: &core::Ptr<crate::objdetect::Board>, camera_matrix: &impl core::ToInputArray, dist_coeffs: &impl core::ToInputArray, rvec: &mut impl core::ToInputOutputArray, tvec: &mut impl core::ToInputOutputArray) -> Result<i32> {
		input_array_arg!(corners);
		input_array_arg!(ids);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		input_output_array_arg!(rvec);
		input_output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_estimatePoseBoard_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(corners.as_raw__InputArray(), ids.as_raw__InputArray(), board.as_raw_PtrOfBoard(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// 
	/// **Deprecated**: Use cv::solvePnP
	/// 
	/// ## C++ default parameters
	/// * use_extrinsic_guess: false
	#[deprecated = "Use cv::solvePnP"]
	#[inline]
	pub fn estimate_pose_board(corners: &impl core::ToInputArray, ids: &impl core::ToInputArray, board: &core::Ptr<crate::objdetect::Board>, camera_matrix: &impl core::ToInputArray, dist_coeffs: &impl core::ToInputArray, rvec: &mut impl core::ToInputOutputArray, tvec: &mut impl core::ToInputOutputArray, use_extrinsic_guess: bool) -> Result<i32> {
		input_array_arg!(corners);
		input_array_arg!(ids);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		input_output_array_arg!(rvec);
		input_output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_estimatePoseBoard_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool(corners.as_raw__InputArray(), ids.as_raw__InputArray(), board.as_raw_PtrOfBoard(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), use_extrinsic_guess, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Pose estimation for a ChArUco board given some of their corners
	/// ## Parameters
	/// * charucoCorners: vector of detected charuco corners
	/// * charucoIds: list of identifiers for each corner in charucoCorners
	/// * board: layout of ChArUco board.
	/// * cameraMatrix: input 3x3 floating-point camera matrix
	/// ![inline formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D)
	/// * distCoeffs: vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5D%2C%5Bs%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5D%5D%29) of 4, 5, 8 or 12 elements
	/// * rvec: Output vector (e.g. cv::Mat) corresponding to the rotation vector of the board
	/// (see cv::Rodrigues).
	/// * tvec: Output vector (e.g. cv::Mat) corresponding to the translation vector of the board.
	/// * useExtrinsicGuess: defines whether initial guess for \b rvec and \b tvec will be used or not.
	/// 
	/// This function estimates a Charuco board pose from some detected corners.
	/// The function checks if the input corners are enough and valid to perform pose estimation.
	/// If pose estimation is valid, returns true, else returns false.
	/// ## See also
	/// use cv::drawFrameAxes to get world coordinate system axis for object points
	/// 
	/// ## Note
	/// This alternative version of [estimate_pose_charuco_board] function uses the following default values for its arguments:
	/// * use_extrinsic_guess: false
	#[inline]
	pub fn estimate_pose_charuco_board_def(charuco_corners: &impl core::ToInputArray, charuco_ids: &impl core::ToInputArray, board: &core::Ptr<crate::objdetect::CharucoBoard>, camera_matrix: &impl core::ToInputArray, dist_coeffs: &impl core::ToInputArray, rvec: &mut impl core::ToInputOutputArray, tvec: &mut impl core::ToInputOutputArray) -> Result<bool> {
		input_array_arg!(charuco_corners);
		input_array_arg!(charuco_ids);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		input_output_array_arg!(rvec);
		input_output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_estimatePoseCharucoBoard_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Pose estimation for a ChArUco board given some of their corners
	/// ## Parameters
	/// * charucoCorners: vector of detected charuco corners
	/// * charucoIds: list of identifiers for each corner in charucoCorners
	/// * board: layout of ChArUco board.
	/// * cameraMatrix: input 3x3 floating-point camera matrix
	/// ![inline formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D)
	/// * distCoeffs: vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5D%2C%5Bs%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5D%5D%29) of 4, 5, 8 or 12 elements
	/// * rvec: Output vector (e.g. cv::Mat) corresponding to the rotation vector of the board
	/// (see cv::Rodrigues).
	/// * tvec: Output vector (e.g. cv::Mat) corresponding to the translation vector of the board.
	/// * useExtrinsicGuess: defines whether initial guess for \b rvec and \b tvec will be used or not.
	/// 
	/// This function estimates a Charuco board pose from some detected corners.
	/// The function checks if the input corners are enough and valid to perform pose estimation.
	/// If pose estimation is valid, returns true, else returns false.
	/// ## See also
	/// use cv::drawFrameAxes to get world coordinate system axis for object points
	/// 
	/// ## C++ default parameters
	/// * use_extrinsic_guess: false
	#[inline]
	pub fn estimate_pose_charuco_board(charuco_corners: &impl core::ToInputArray, charuco_ids: &impl core::ToInputArray, board: &core::Ptr<crate::objdetect::CharucoBoard>, camera_matrix: &impl core::ToInputArray, dist_coeffs: &impl core::ToInputArray, rvec: &mut impl core::ToInputOutputArray, tvec: &mut impl core::ToInputOutputArray, use_extrinsic_guess: bool) -> Result<bool> {
		input_array_arg!(charuco_corners);
		input_array_arg!(charuco_ids);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		input_output_array_arg!(rvec);
		input_output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_estimatePoseCharucoBoard_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_bool(charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), use_extrinsic_guess, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// 
	/// **Deprecated**: Use cv::solvePnP
	/// 
	/// ## Note
	/// This alternative version of [estimate_pose_single_markers] function uses the following default values for its arguments:
	/// * obj_points: noArray()
	/// * estimate_parameters: makePtr<EstimateParameters>()
	#[deprecated = "Use cv::solvePnP"]
	#[inline]
	pub fn estimate_pose_single_markers_def(corners: &impl core::ToInputArray, marker_length: f32, camera_matrix: &impl core::ToInputArray, dist_coeffs: &impl core::ToInputArray, rvecs: &mut impl core::ToOutputArray, tvecs: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(corners);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_estimatePoseSingleMarkers_const__InputArrayR_float_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(corners.as_raw__InputArray(), marker_length, camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// 
	/// **Deprecated**: Use cv::solvePnP
	/// 
	/// ## C++ default parameters
	/// * obj_points: noArray()
	/// * estimate_parameters: makePtr<EstimateParameters>()
	#[deprecated = "Use cv::solvePnP"]
	#[inline]
	pub fn estimate_pose_single_markers(corners: &impl core::ToInputArray, marker_length: f32, camera_matrix: &impl core::ToInputArray, dist_coeffs: &impl core::ToInputArray, rvecs: &mut impl core::ToOutputArray, tvecs: &mut impl core::ToOutputArray, obj_points: &mut impl core::ToOutputArray, estimate_parameters: &core::Ptr<crate::aruco::EstimateParameters>) -> Result<()> {
		input_array_arg!(corners);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		output_array_arg!(rvecs);
		output_array_arg!(tvecs);
		output_array_arg!(obj_points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_estimatePoseSingleMarkers_const__InputArrayR_float_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const_PtrLEstimateParametersGR(corners.as_raw__InputArray(), marker_length, camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), obj_points.as_raw__OutputArray(), estimate_parameters.as_raw_PtrOfEstimateParameters(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// get board object and image points
	/// 
	/// **Deprecated**: Use Board::matchImagePoints
	#[deprecated = "Use Board::matchImagePoints"]
	#[inline]
	pub fn get_board_object_and_image_points(board: &core::Ptr<crate::objdetect::Board>, detected_corners: &impl core::ToInputArray, detected_ids: &impl core::ToInputArray, obj_points: &mut impl core::ToOutputArray, img_points: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(detected_corners);
		input_array_arg!(detected_ids);
		output_array_arg!(obj_points);
		output_array_arg!(img_points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_getBoardObjectAndImagePoints_const_PtrLBoardGR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(board.as_raw_PtrOfBoard(), detected_corners.as_raw__InputArray(), detected_ids.as_raw__InputArray(), obj_points.as_raw__OutputArray(), img_points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Interpolate position of ChArUco board corners
	/// ## Parameters
	/// * markerCorners: vector of already detected markers corners. For each marker, its four
	/// corners are provided, (e.g std::vector<std::vector<cv::Point2f> > ). For N detected markers, the
	/// dimensions of this array should be Nx4. The order of the corners should be clockwise.
	/// * markerIds: list of identifiers for each marker in corners
	/// * image: input image necesary for corner refinement. Note that markers are not detected and
	/// should be sent in corners and ids parameters.
	/// * board: layout of ChArUco board.
	/// * charucoCorners: interpolated chessboard corners
	/// * charucoIds: interpolated chessboard corners identifiers
	/// * cameraMatrix: optional 3x3 floating-point camera matrix
	/// ![inline formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D)
	/// * distCoeffs: optional vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5D%2C%5Bs%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5D%5D%29) of 4, 5, 8 or 12 elements
	/// * minMarkers: number of adjacent markers that must be detected to return a charuco corner
	/// 
	/// This function receives the detected markers and returns the 2D position of the chessboard corners
	/// from a ChArUco board using the detected Aruco markers. If camera parameters are provided,
	/// the process is based in an approximated pose estimation, else it is based on local homography.
	/// Only visible corners are returned. For each corner, its corresponding identifier is
	/// also returned in charucoIds.
	/// The function returns the number of interpolated corners.
	/// 
	/// 
	/// **Deprecated**: Use CharucoDetector::detectBoard
	/// 
	/// ## Note
	/// This alternative version of [interpolate_corners_charuco] function uses the following default values for its arguments:
	/// * camera_matrix: noArray()
	/// * dist_coeffs: noArray()
	/// * min_markers: 2
	#[deprecated = "Use CharucoDetector::detectBoard"]
	#[inline]
	pub fn interpolate_corners_charuco_def(marker_corners: &impl core::ToInputArray, marker_ids: &impl core::ToInputArray, image: &impl core::ToInputArray, board: &core::Ptr<crate::objdetect::CharucoBoard>, charuco_corners: &mut impl core::ToOutputArray, charuco_ids: &mut impl core::ToOutputArray) -> Result<i32> {
		input_array_arg!(marker_corners);
		input_array_arg!(marker_ids);
		input_array_arg!(image);
		output_array_arg!(charuco_corners);
		output_array_arg!(charuco_ids);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_interpolateCornersCharuco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_const__OutputArrayR_const__OutputArrayR(marker_corners.as_raw__InputArray(), marker_ids.as_raw__InputArray(), image.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), charuco_corners.as_raw__OutputArray(), charuco_ids.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Interpolate position of ChArUco board corners
	/// ## Parameters
	/// * markerCorners: vector of already detected markers corners. For each marker, its four
	/// corners are provided, (e.g std::vector<std::vector<cv::Point2f> > ). For N detected markers, the
	/// dimensions of this array should be Nx4. The order of the corners should be clockwise.
	/// * markerIds: list of identifiers for each marker in corners
	/// * image: input image necesary for corner refinement. Note that markers are not detected and
	/// should be sent in corners and ids parameters.
	/// * board: layout of ChArUco board.
	/// * charucoCorners: interpolated chessboard corners
	/// * charucoIds: interpolated chessboard corners identifiers
	/// * cameraMatrix: optional 3x3 floating-point camera matrix
	/// ![inline formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D)
	/// * distCoeffs: optional vector of distortion coefficients
	/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5D%2C%5Bs%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5D%5D%29) of 4, 5, 8 or 12 elements
	/// * minMarkers: number of adjacent markers that must be detected to return a charuco corner
	/// 
	/// This function receives the detected markers and returns the 2D position of the chessboard corners
	/// from a ChArUco board using the detected Aruco markers. If camera parameters are provided,
	/// the process is based in an approximated pose estimation, else it is based on local homography.
	/// Only visible corners are returned. For each corner, its corresponding identifier is
	/// also returned in charucoIds.
	/// The function returns the number of interpolated corners.
	/// 
	/// 
	/// **Deprecated**: Use CharucoDetector::detectBoard
	/// 
	/// ## C++ default parameters
	/// * camera_matrix: noArray()
	/// * dist_coeffs: noArray()
	/// * min_markers: 2
	#[deprecated = "Use CharucoDetector::detectBoard"]
	#[inline]
	pub fn interpolate_corners_charuco(marker_corners: &impl core::ToInputArray, marker_ids: &impl core::ToInputArray, image: &impl core::ToInputArray, board: &core::Ptr<crate::objdetect::CharucoBoard>, charuco_corners: &mut impl core::ToOutputArray, charuco_ids: &mut impl core::ToOutputArray, camera_matrix: &impl core::ToInputArray, dist_coeffs: &impl core::ToInputArray, min_markers: i32) -> Result<i32> {
		input_array_arg!(marker_corners);
		input_array_arg!(marker_ids);
		input_array_arg!(image);
		output_array_arg!(charuco_corners);
		output_array_arg!(charuco_ids);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_interpolateCornersCharuco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int(marker_corners.as_raw__InputArray(), marker_ids.as_raw__InputArray(), image.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), charuco_corners.as_raw__OutputArray(), charuco_ids.as_raw__OutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), min_markers, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// refine detected markers
	/// 
	/// **Deprecated**: Use class ArucoDetector::refineDetectedMarkers
	/// 
	/// ## Note
	/// This alternative version of [refine_detected_markers] function uses the following default values for its arguments:
	/// * camera_matrix: noArray()
	/// * dist_coeffs: noArray()
	/// * min_rep_distance: 10.f
	/// * error_correction_rate: 3.f
	/// * check_all_orders: true
	/// * recovered_idxs: noArray()
	/// * parameters: makePtr<DetectorParameters>()
	#[deprecated = "Use class ArucoDetector::refineDetectedMarkers"]
	#[inline]
	pub fn refine_detected_markers_def(image: &impl core::ToInputArray, board: &core::Ptr<crate::objdetect::Board>, detected_corners: &mut impl core::ToInputOutputArray, detected_ids: &mut impl core::ToInputOutputArray, rejected_corners: &mut impl core::ToInputOutputArray) -> Result<()> {
		input_array_arg!(image);
		input_output_array_arg!(detected_corners);
		input_output_array_arg!(detected_ids);
		input_output_array_arg!(rejected_corners);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_refineDetectedMarkers_const__InputArrayR_const_PtrLBoardGR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(image.as_raw__InputArray(), board.as_raw_PtrOfBoard(), detected_corners.as_raw__InputOutputArray(), detected_ids.as_raw__InputOutputArray(), rejected_corners.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// refine detected markers
	/// 
	/// **Deprecated**: Use class ArucoDetector::refineDetectedMarkers
	/// 
	/// ## C++ default parameters
	/// * camera_matrix: noArray()
	/// * dist_coeffs: noArray()
	/// * min_rep_distance: 10.f
	/// * error_correction_rate: 3.f
	/// * check_all_orders: true
	/// * recovered_idxs: noArray()
	/// * parameters: makePtr<DetectorParameters>()
	#[deprecated = "Use class ArucoDetector::refineDetectedMarkers"]
	#[inline]
	pub fn refine_detected_markers(image: &impl core::ToInputArray, board: &core::Ptr<crate::objdetect::Board>, detected_corners: &mut impl core::ToInputOutputArray, detected_ids: &mut impl core::ToInputOutputArray, rejected_corners: &mut impl core::ToInputOutputArray, camera_matrix: &impl core::ToInputArray, dist_coeffs: &impl core::ToInputArray, min_rep_distance: f32, error_correction_rate: f32, check_all_orders: bool, recovered_idxs: &mut impl core::ToOutputArray, parameters: &core::Ptr<crate::objdetect::DetectorParameters>) -> Result<()> {
		input_array_arg!(image);
		input_output_array_arg!(detected_corners);
		input_output_array_arg!(detected_ids);
		input_output_array_arg!(rejected_corners);
		input_array_arg!(camera_matrix);
		input_array_arg!(dist_coeffs);
		output_array_arg!(recovered_idxs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_refineDetectedMarkers_const__InputArrayR_const_PtrLBoardGR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_float_float_bool_const__OutputArrayR_const_PtrLDetectorParametersGR(image.as_raw__InputArray(), board.as_raw_PtrOfBoard(), detected_corners.as_raw__InputOutputArray(), detected_ids.as_raw__InputOutputArray(), rejected_corners.as_raw__InputOutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), min_rep_distance, error_correction_rate, check_all_orders, recovered_idxs.as_raw__OutputArray(), parameters.as_raw_PtrOfDetectorParameters(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// 
	/// **Deprecated**: Use CharucoBoard::checkCharucoCornersCollinear
	#[deprecated = "Use CharucoBoard::checkCharucoCornersCollinear"]
	#[inline]
	pub fn test_charuco_corners_collinear(board: &core::Ptr<crate::objdetect::CharucoBoard>, charuco_ids: &impl core::ToInputArray) -> Result<bool> {
		input_array_arg!(charuco_ids);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_testCharucoCornersCollinear_const_PtrLCharucoBoardGR_const__InputArrayR(board.as_raw_PtrOfCharucoBoard(), charuco_ids.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Constant methods for [crate::aruco::EstimateParameters]
	pub trait EstimateParametersTraitConst {
		fn as_raw_EstimateParameters(&self) -> *const c_void;
	
		#[inline]
		fn pattern(&self) -> crate::aruco::PatternPositionType {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_EstimateParameters_propPattern_const(self.as_raw_EstimateParameters(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		#[inline]
		fn use_extrinsic_guess(&self) -> bool {
			let ret = unsafe { sys::cv_aruco_EstimateParameters_propUseExtrinsicGuess_const(self.as_raw_EstimateParameters()) };
			ret
		}
		
		#[inline]
		fn solve_pnp_method(&self) -> i32 {
			let ret = unsafe { sys::cv_aruco_EstimateParameters_propSolvePnPMethod_const(self.as_raw_EstimateParameters()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::aruco::EstimateParameters]
	pub trait EstimateParametersTrait: crate::aruco::EstimateParametersTraitConst {
		fn as_raw_mut_EstimateParameters(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_pattern(&mut self, val: crate::aruco::PatternPositionType) {
			let ret = unsafe { sys::cv_aruco_EstimateParameters_propPattern_PatternPositionType(self.as_raw_mut_EstimateParameters(), val) };
			ret
		}
		
		#[inline]
		fn set_use_extrinsic_guess(&mut self, val: bool) {
			let ret = unsafe { sys::cv_aruco_EstimateParameters_propUseExtrinsicGuess_bool(self.as_raw_mut_EstimateParameters(), val) };
			ret
		}
		
		#[inline]
		fn set_solve_pnp_method(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_EstimateParameters_propSolvePnPMethod_int(self.as_raw_mut_EstimateParameters(), val) };
			ret
		}
		
	}
	
	/// Pose estimation parameters
	/// 
	/// ## Parameters
	/// * pattern: Defines center this system and axes direction (default PatternPositionType::ARUCO_CCW_CENTER).
	/// * useExtrinsicGuess: Parameter used for SOLVEPNP_ITERATIVE. If true (1), the function uses the provided
	/// rvec and tvec values as initial approximations of the rotation and translation vectors, respectively, and further
	/// optimizes them (default false).
	/// * solvePnPMethod: Method for solving a PnP problem: see [calib3d_solvePnP_flags] (default SOLVEPNP_ITERATIVE).
	/// ## See also
	/// PatternPositionType, solvePnP(), check tutorial_aruco_detection in aruco contrib
	pub struct EstimateParameters {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { EstimateParameters }
	
	impl Drop for EstimateParameters {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_aruco_EstimateParameters_delete(self.as_raw_mut_EstimateParameters()) };
		}
	}
	
	unsafe impl Send for EstimateParameters {}
	
	impl crate::aruco::EstimateParametersTraitConst for EstimateParameters {
		#[inline] fn as_raw_EstimateParameters(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::aruco::EstimateParametersTrait for EstimateParameters {
		#[inline] fn as_raw_mut_EstimateParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl EstimateParameters {
		#[inline]
		pub fn default() -> Result<crate::aruco::EstimateParameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_EstimateParameters_EstimateParameters(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::aruco::EstimateParameters::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl Clone for EstimateParameters {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_aruco_EstimateParameters_implicitClone_const(self.as_raw_EstimateParameters())) }
		}
	}
	
	impl std::fmt::Debug for EstimateParameters {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("EstimateParameters")
				.field("pattern", &crate::aruco::EstimateParametersTraitConst::pattern(self))
				.field("use_extrinsic_guess", &crate::aruco::EstimateParametersTraitConst::use_extrinsic_guess(self))
				.field("solve_pnp_method", &crate::aruco::EstimateParametersTraitConst::solve_pnp_method(self))
				.finish()
		}
	}
}
