//! # Aruco markers, module functionality was moved to objdetect module
//! ArUco Marker Detection, module functionality was moved to objdetect module
//! ## See also
//! ArucoDetector, CharucoDetector, Board, GridBoard, CharucoBoard
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{EstimateParametersTrait, EstimateParametersTraitConst};
}

/// The marker coordinate system is centered on the middle of the marker.
///
/// The coordinates of the four corners (CCW order) of the marker in its own coordinate system are:
/// (-markerLength/2, markerLength/2, 0), (markerLength/2, markerLength/2, 0),
/// (markerLength/2, -markerLength/2, 0), (-markerLength/2, -markerLength/2, 0).
// ARUCO_CCW_CENTER /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:31
pub const ARUCO_CCW_CENTER: i32 = 0;
/// The marker coordinate system is centered on the top-left corner of the marker.
///
/// The coordinates of the four corners (CW order) of the marker in its own coordinate system are:
/// (0, 0, 0), (markerLength, 0, 0),
/// (markerLength, markerLength, 0), (0, markerLength, 0).
///
/// These pattern dots are convenient to use with a chessboard/ChArUco board.
// ARUCO_CW_TOP_LEFT_CORNER /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:40
pub const ARUCO_CW_TOP_LEFT_CORNER: i32 = 1;
/// rvec/tvec define the right handed coordinate system of the marker.
///
/// PatternPositionType defines center this system and axes direction.
/// Axis X (red color) - first coordinate, axis Y (green color) - second coordinate,
/// axis Z (blue color) - third coordinate.
///
///
/// **Deprecated**: Use Board::matchImagePoints and cv::solvePnP
/// ## See also
/// estimatePoseSingleMarkers()
#[deprecated = "Use Board::matchImagePoints and cv::solvePnP"]
// PatternPositionType /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:24
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PatternPositionType {
	/// The marker coordinate system is centered on the middle of the marker.
	///
	/// The coordinates of the four corners (CCW order) of the marker in its own coordinate system are:
	/// (-markerLength/2, markerLength/2, 0), (markerLength/2, markerLength/2, 0),
	/// (markerLength/2, -markerLength/2, 0), (-markerLength/2, -markerLength/2, 0).
	ARUCO_CCW_CENTER = 0,
	/// The marker coordinate system is centered on the top-left corner of the marker.
	///
	/// The coordinates of the four corners (CW order) of the marker in its own coordinate system are:
	/// (0, 0, 0), (markerLength, 0, 0),
	/// (markerLength, markerLength, 0), (0, markerLength, 0).
	///
	/// These pattern dots are convenient to use with a chessboard/ChArUco board.
	ARUCO_CW_TOP_LEFT_CORNER = 1,
}

impl TryFrom<i32> for PatternPositionType {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::ARUCO_CCW_CENTER),
			1 => Ok(Self::ARUCO_CW_TOP_LEFT_CORNER),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::aruco::PatternPositionType"))),
		}
	}
}

opencv_type_enum! { crate::aruco::PatternPositionType }

/// @overload
/// It's the same function as [calibrate_camera_aruco] but without calibration error estimation.
///
/// **Deprecated**: Use Board::matchImagePoints and cv::solvePnP
///
/// ## Note
/// This alternative version of [calibrate_camera_aruco] function uses the following default values for its arguments:
/// * rvecs: noArray()
/// * tvecs: noArray()
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
#[deprecated = "Use Board::matchImagePoints and cv::solvePnP"]
// cv::aruco::calibrateCameraAruco(InputArray, InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:112
// ("cv::aruco::calibrateCameraAruco", vec![(pred!(mut, ["corners", "ids", "counter", "board", "imageSize", "cameraMatrix", "distCoeffs"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
#[inline]
pub fn calibrate_camera_aruco_def(corners: &impl ToInputArray, ids: &impl ToInputArray, counter: &impl ToInputArray, board: &core::Ptr<crate::objdetect::Board>, image_size: core::Size, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &mut impl ToInputOutputArray) -> Result<f64> {
	input_array_arg!(corners);
	input_array_arg!(ids);
	input_array_arg!(counter);
	input_output_array_arg!(camera_matrix);
	input_output_array_arg!(dist_coeffs);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR(corners.as_raw__InputArray(), ids.as_raw__InputArray(), counter.as_raw__InputArray(), board.as_raw_PtrOfBoard(), &image_size, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
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
///
/// **Deprecated**: Use Board::matchImagePoints and cv::solvePnP
///
/// ## Note
/// This alternative version of [calibrate_camera_aruco_extended] function uses the following default values for its arguments:
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
#[deprecated = "Use Board::matchImagePoints and cv::solvePnP"]
// cv::aruco::calibrateCameraAruco(InputArray, InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:102
// ("cv::aruco::calibrateCameraAruco", vec![(pred!(mut, ["corners", "ids", "counter", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "perViewErrors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn calibrate_camera_aruco_extended_def(corners: &impl ToInputArray, ids: &impl ToInputArray, counter: &impl ToInputArray, board: &core::Ptr<crate::objdetect::Board>, image_size: core::Size, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &mut impl ToInputOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, std_deviations_intrinsics: &mut impl ToOutputArray, std_deviations_extrinsics: &mut impl ToOutputArray, per_view_errors: &mut impl ToOutputArray) -> Result<f64> {
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
	unsafe { sys::cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(corners.as_raw__InputArray(), ids.as_raw__InputArray(), counter.as_raw__InputArray(), board.as_raw_PtrOfBoard(), &image_size, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), std_deviations_intrinsics.as_raw__OutputArray(), std_deviations_extrinsics.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
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
///
/// **Deprecated**: Use Board::matchImagePoints and cv::solvePnP
///
/// ## C++ default parameters
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
#[deprecated = "Use Board::matchImagePoints and cv::solvePnP"]
// calibrateCameraAruco(InputArrayOfArrays, InputArray, InputArray, const Ptr<Board> &, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, OutputArray, OutputArray, OutputArray, int, const TermCriteria &)(InputArray, InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:102
// ("cv::aruco::calibrateCameraAruco", vec![(pred!(mut, ["corners", "ids", "counter", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "perViewErrors", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "const cv::TermCriteria*"]), _)]),
#[inline]
pub fn calibrate_camera_aruco_extended(corners: &impl ToInputArray, ids: &impl ToInputArray, counter: &impl ToInputArray, board: &core::Ptr<crate::objdetect::Board>, image_size: core::Size, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &mut impl ToInputOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, std_deviations_intrinsics: &mut impl ToOutputArray, std_deviations_extrinsics: &mut impl ToOutputArray, per_view_errors: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
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
	unsafe { sys::cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_TermCriteriaR(corners.as_raw__InputArray(), ids.as_raw__InputArray(), counter.as_raw__InputArray(), board.as_raw_PtrOfBoard(), &image_size, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), std_deviations_intrinsics.as_raw__OutputArray(), std_deviations_extrinsics.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
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
///
/// **Deprecated**: Use Board::matchImagePoints and cv::solvePnP
///
/// ## Overloaded parameters
///
/// It's the same function as [calibrate_camera_aruco] but without calibration error estimation.
///
/// **Deprecated**: Use Board::matchImagePoints and cv::solvePnP
///
/// ## C++ default parameters
/// * rvecs: noArray()
/// * tvecs: noArray()
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
#[deprecated = "Use Board::matchImagePoints and cv::solvePnP"]
// calibrateCameraAruco(InputArrayOfArrays, InputArray, InputArray, const Ptr<Board> &, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, int, const TermCriteria &)(InputArray, InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:112
// ("cv::aruco::calibrateCameraAruco", vec![(pred!(mut, ["corners", "ids", "counter", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "const cv::TermCriteria*"]), _)]),
#[inline]
pub fn calibrate_camera_aruco(corners: &impl ToInputArray, ids: &impl ToInputArray, counter: &impl ToInputArray, board: &core::Ptr<crate::objdetect::Board>, image_size: core::Size, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &mut impl ToInputOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
	input_array_arg!(corners);
	input_array_arg!(ids);
	input_array_arg!(counter);
	input_output_array_arg!(camera_matrix);
	input_output_array_arg!(dist_coeffs);
	output_array_arg!(rvecs);
	output_array_arg!(tvecs);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_TermCriteriaR(corners.as_raw__InputArray(), ids.as_raw__InputArray(), counter.as_raw__InputArray(), board.as_raw_PtrOfBoard(), &image_size, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// It's the same function as [calibrate_camera_charuco] but without calibration error estimation.
///
///
/// **Deprecated**: Use CharucoBoard::matchImagePoints and cv::solvePnP
///
/// ## Note
/// This alternative version of [calibrate_camera_charuco] function uses the following default values for its arguments:
/// * rvecs: noArray()
/// * tvecs: noArray()
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
#[deprecated = "Use CharucoBoard::matchImagePoints and cv::solvePnP"]
// cv::aruco::calibrateCameraCharuco(InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:169
// ("cv::aruco::calibrateCameraCharuco", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "imageSize", "cameraMatrix", "distCoeffs"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
#[inline]
pub fn calibrate_camera_charuco_def(charuco_corners: &impl ToInputArray, charuco_ids: &impl ToInputArray, board: &core::Ptr<crate::objdetect::CharucoBoard>, image_size: core::Size, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &mut impl ToInputOutputArray) -> Result<f64> {
	input_array_arg!(charuco_corners);
	input_array_arg!(charuco_ids);
	input_output_array_arg!(camera_matrix);
	input_output_array_arg!(dist_coeffs);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR(charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), &image_size, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
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
///
/// **Deprecated**: Use CharucoBoard::matchImagePoints and cv::solvePnP
///
/// ## Note
/// This alternative version of [calibrate_camera_charuco_extended] function uses the following default values for its arguments:
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
#[deprecated = "Use CharucoBoard::matchImagePoints and cv::solvePnP"]
// cv::aruco::calibrateCameraCharuco(InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:157
// ("cv::aruco::calibrateCameraCharuco", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "perViewErrors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn calibrate_camera_charuco_extended_def(charuco_corners: &impl ToInputArray, charuco_ids: &impl ToInputArray, board: &core::Ptr<crate::objdetect::CharucoBoard>, image_size: core::Size, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &mut impl ToInputOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, std_deviations_intrinsics: &mut impl ToOutputArray, std_deviations_extrinsics: &mut impl ToOutputArray, per_view_errors: &mut impl ToOutputArray) -> Result<f64> {
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
	unsafe { sys::cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), &image_size, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), std_deviations_intrinsics.as_raw__OutputArray(), std_deviations_extrinsics.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
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
///
/// **Deprecated**: Use CharucoBoard::matchImagePoints and cv::solvePnP
///
/// ## C++ default parameters
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
#[deprecated = "Use CharucoBoard::matchImagePoints and cv::solvePnP"]
// calibrateCameraCharuco(InputArrayOfArrays, InputArrayOfArrays, const Ptr<CharucoBoard> &, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, OutputArray, OutputArray, OutputArray, int, const TermCriteria &)(InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:157
// ("cv::aruco::calibrateCameraCharuco", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "perViewErrors", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "const cv::TermCriteria*"]), _)]),
#[inline]
pub fn calibrate_camera_charuco_extended(charuco_corners: &impl ToInputArray, charuco_ids: &impl ToInputArray, board: &core::Ptr<crate::objdetect::CharucoBoard>, image_size: core::Size, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &mut impl ToInputOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, std_deviations_intrinsics: &mut impl ToOutputArray, std_deviations_extrinsics: &mut impl ToOutputArray, per_view_errors: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
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
	unsafe { sys::cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_TermCriteriaR(charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), &image_size, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), std_deviations_intrinsics.as_raw__OutputArray(), std_deviations_extrinsics.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// It's the same function as [calibrate_camera_charuco] but without calibration error estimation.
///
///
/// **Deprecated**: Use CharucoBoard::matchImagePoints and cv::solvePnP
///
/// ## C++ default parameters
/// * rvecs: noArray()
/// * tvecs: noArray()
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
#[deprecated = "Use CharucoBoard::matchImagePoints and cv::solvePnP"]
// calibrateCameraCharuco(InputArrayOfArrays, InputArrayOfArrays, const Ptr<CharucoBoard> &, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, int, const TermCriteria &)(InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:169
// ("cv::aruco::calibrateCameraCharuco", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "const cv::TermCriteria*"]), _)]),
#[inline]
pub fn calibrate_camera_charuco(charuco_corners: &impl ToInputArray, charuco_ids: &impl ToInputArray, board: &core::Ptr<crate::objdetect::CharucoBoard>, image_size: core::Size, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &mut impl ToInputOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
	input_array_arg!(charuco_corners);
	input_array_arg!(charuco_ids);
	input_output_array_arg!(camera_matrix);
	input_output_array_arg!(dist_coeffs);
	output_array_arg!(rvecs);
	output_array_arg!(tvecs);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_TermCriteriaR(charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), &image_size, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
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
// cv::aruco::detectCharucoDiamond(InputArray, InputArray, InputArray, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/charuco.hpp:77
// ("cv::aruco::detectCharucoDiamond", vec![(pred!(mut, ["image", "markerCorners", "markerIds", "squareMarkerLengthRate", "diamondCorners", "diamondIds"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "float", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn detect_charuco_diamond_def(image: &impl ToInputArray, marker_corners: &impl ToInputArray, marker_ids: &impl ToInputArray, square_marker_length_rate: f32, diamond_corners: &mut impl ToOutputArray, diamond_ids: &mut impl ToOutputArray) -> Result<()> {
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
// detectCharucoDiamond(InputArray, InputArrayOfArrays, InputArray, float, OutputArrayOfArrays, OutputArray, InputArray, InputArray, Ptr<Dictionary>)(InputArray, InputArray, InputArray, Primitive, OutputArray, OutputArray, InputArray, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/charuco.hpp:77
// ("cv::aruco::detectCharucoDiamond", vec![(pred!(mut, ["image", "markerCorners", "markerIds", "squareMarkerLengthRate", "diamondCorners", "diamondIds", "cameraMatrix", "distCoeffs", "dictionary"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "float", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Ptr<cv::aruco::Dictionary>"]), _)]),
#[inline]
pub fn detect_charuco_diamond(image: &impl ToInputArray, marker_corners: &impl ToInputArray, marker_ids: &impl ToInputArray, square_marker_length_rate: f32, diamond_corners: &mut impl ToOutputArray, diamond_ids: &mut impl ToOutputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, mut dictionary: core::Ptr<crate::objdetect::Dictionary>) -> Result<()> {
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
// cv::aruco::detectMarkers(InputArray, CppPassByVoidPtr, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:27
// ("cv::aruco::detectMarkers", vec![(pred!(mut, ["image", "dictionary", "corners", "ids"], ["const cv::_InputArray*", "const cv::Ptr<cv::aruco::Dictionary>*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn detect_markers_def(image: &impl ToInputArray, dictionary: &core::Ptr<crate::objdetect::Dictionary>, corners: &mut impl ToOutputArray, ids: &mut impl ToOutputArray) -> Result<()> {
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
// detectMarkers(InputArray, const Ptr<Dictionary> &, OutputArrayOfArrays, OutputArray, const Ptr<DetectorParameters> &, OutputArrayOfArrays)(InputArray, CppPassByVoidPtr, OutputArray, OutputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:27
// ("cv::aruco::detectMarkers", vec![(pred!(mut, ["image", "dictionary", "corners", "ids", "parameters", "rejectedImgPoints"], ["const cv::_InputArray*", "const cv::Ptr<cv::aruco::Dictionary>*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::Ptr<cv::aruco::DetectorParameters>*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn detect_markers(image: &impl ToInputArray, dictionary: &core::Ptr<crate::objdetect::Dictionary>, corners: &mut impl ToOutputArray, ids: &mut impl ToOutputArray, parameters: &core::Ptr<crate::objdetect::DetectorParameters>, rejected_img_points: &mut impl ToOutputArray) -> Result<()> {
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
///
/// **Deprecated**: Use CharucoBoard::generateImage()
///
/// ## Note
/// This alternative version of [draw_charuco_diamond] function uses the following default values for its arguments:
/// * margin_size: 0
/// * border_bits: 1
#[deprecated = "Use CharucoBoard::generateImage()"]
// cv::aruco::drawCharucoDiamond(CppPassByVoidPtr, SimpleClass, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/charuco.hpp:102
// ("cv::aruco::drawCharucoDiamond", vec![(pred!(mut, ["dictionary", "ids", "squareLength", "markerLength", "img"], ["const cv::Ptr<cv::aruco::Dictionary>*", "cv::Vec4i", "int", "int", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn draw_charuco_diamond_def(dictionary: &core::Ptr<crate::objdetect::Dictionary>, ids: core::Vec4i, square_length: i32, marker_length: i32, img: &mut impl ToOutputArray) -> Result<()> {
	output_array_arg!(img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_drawCharucoDiamond_const_PtrLDictionaryGR_Vec4i_int_int_const__OutputArrayR(dictionary.as_raw_PtrOfDictionary(), &ids, square_length, marker_length, img.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
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
///
/// **Deprecated**: Use CharucoBoard::generateImage()
///
/// ## C++ default parameters
/// * margin_size: 0
/// * border_bits: 1
#[deprecated = "Use CharucoBoard::generateImage()"]
// drawCharucoDiamond(const Ptr<Dictionary> &, Vec4i, int, int, OutputArray, int, int)(CppPassByVoidPtr, SimpleClass, Primitive, Primitive, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/charuco.hpp:102
// ("cv::aruco::drawCharucoDiamond", vec![(pred!(mut, ["dictionary", "ids", "squareLength", "markerLength", "img", "marginSize", "borderBits"], ["const cv::Ptr<cv::aruco::Dictionary>*", "cv::Vec4i", "int", "int", "const cv::_OutputArray*", "int", "int"]), _)]),
#[inline]
pub fn draw_charuco_diamond(dictionary: &core::Ptr<crate::objdetect::Dictionary>, ids: core::Vec4i, square_length: i32, marker_length: i32, img: &mut impl ToOutputArray, margin_size: i32, border_bits: i32) -> Result<()> {
	output_array_arg!(img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_drawCharucoDiamond_const_PtrLDictionaryGR_Vec4i_int_int_const__OutputArrayR_int_int(dictionary.as_raw_PtrOfDictionary(), &ids, square_length, marker_length, img.as_raw__OutputArray(), margin_size, border_bits, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// draw planar board
///
/// **Deprecated**: Use Board::generateImage
#[deprecated = "Use Board::generateImage"]
// drawPlanarBoard(const Ptr<Board> &, Size, OutputArray, int, int)(CppPassByVoidPtr, SimpleClass, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:45
// ("cv::aruco::drawPlanarBoard", vec![(pred!(mut, ["board", "outSize", "img", "marginSize", "borderBits"], ["const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_OutputArray*", "int", "int"]), _)]),
#[inline]
pub fn draw_planar_board(board: &core::Ptr<crate::objdetect::Board>, out_size: core::Size, img: &mut impl ToOutputArray, margin_size: i32, border_bits: i32) -> Result<()> {
	output_array_arg!(img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_drawPlanarBoard_const_PtrLBoardGR_Size_const__OutputArrayR_int_int(board.as_raw_PtrOfBoard(), &out_size, img.as_raw__OutputArray(), margin_size, border_bits, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

///
/// **Deprecated**: Use Board::matchImagePoints and cv::solvePnP
///
/// ## Note
/// This alternative version of [estimate_pose_board] function uses the following default values for its arguments:
/// * use_extrinsic_guess: false
#[deprecated = "Use Board::matchImagePoints and cv::solvePnP"]
// cv::aruco::estimatePoseBoard(InputArray, InputArray, CppPassByVoidPtr, InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:57
// ("cv::aruco::estimatePoseBoard", vec![(pred!(mut, ["corners", "ids", "board", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
#[inline]
pub fn estimate_pose_board_def(corners: &impl ToInputArray, ids: &impl ToInputArray, board: &core::Ptr<crate::objdetect::Board>, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToInputOutputArray, tvec: &mut impl ToInputOutputArray) -> Result<i32> {
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
/// **Deprecated**: Use Board::matchImagePoints and cv::solvePnP
///
/// ## C++ default parameters
/// * use_extrinsic_guess: false
#[deprecated = "Use Board::matchImagePoints and cv::solvePnP"]
// estimatePoseBoard(InputArrayOfArrays, InputArray, const Ptr<Board> &, InputArray, InputArray, InputOutputArray, InputOutputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, InputArray, InputArray, InputOutputArray, InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:57
// ("cv::aruco::estimatePoseBoard", vec![(pred!(mut, ["corners", "ids", "board", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "bool"]), _)]),
#[inline]
pub fn estimate_pose_board(corners: &impl ToInputArray, ids: &impl ToInputArray, board: &core::Ptr<crate::objdetect::Board>, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToInputOutputArray, tvec: &mut impl ToInputOutputArray, use_extrinsic_guess: bool) -> Result<i32> {
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
///
/// **Deprecated**: Use CharucoBoard::matchImagePoints and cv::solvePnP
/// ## See also
/// use cv::drawFrameAxes to get world coordinate system axis for object points
///
/// ## Note
/// This alternative version of [estimate_pose_charuco_board] function uses the following default values for its arguments:
/// * use_extrinsic_guess: false
#[deprecated = "Use CharucoBoard::matchImagePoints and cv::solvePnP"]
// cv::aruco::estimatePoseCharucoBoard(InputArray, InputArray, CppPassByVoidPtr, InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:81
// ("cv::aruco::estimatePoseCharucoBoard", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
#[inline]
pub fn estimate_pose_charuco_board_def(charuco_corners: &impl ToInputArray, charuco_ids: &impl ToInputArray, board: &core::Ptr<crate::objdetect::CharucoBoard>, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToInputOutputArray, tvec: &mut impl ToInputOutputArray) -> Result<bool> {
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
///
/// **Deprecated**: Use CharucoBoard::matchImagePoints and cv::solvePnP
/// ## See also
/// use cv::drawFrameAxes to get world coordinate system axis for object points
///
/// ## C++ default parameters
/// * use_extrinsic_guess: false
#[deprecated = "Use CharucoBoard::matchImagePoints and cv::solvePnP"]
// estimatePoseCharucoBoard(InputArray, InputArray, const Ptr<CharucoBoard> &, InputArray, InputArray, InputOutputArray, InputOutputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, InputArray, InputArray, InputOutputArray, InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:81
// ("cv::aruco::estimatePoseCharucoBoard", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "bool"]), _)]),
#[inline]
pub fn estimate_pose_charuco_board(charuco_corners: &impl ToInputArray, charuco_ids: &impl ToInputArray, board: &core::Ptr<crate::objdetect::CharucoBoard>, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToInputOutputArray, tvec: &mut impl ToInputOutputArray, use_extrinsic_guess: bool) -> Result<bool> {
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
// cv::aruco::estimatePoseSingleMarkers(InputArray, Primitive, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:88
// ("cv::aruco::estimatePoseSingleMarkers", vec![(pred!(mut, ["corners", "markerLength", "cameraMatrix", "distCoeffs", "rvecs", "tvecs"], ["const cv::_InputArray*", "float", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn estimate_pose_single_markers_def(corners: &impl ToInputArray, marker_length: f32, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray) -> Result<()> {
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
// estimatePoseSingleMarkers(InputArrayOfArrays, float, InputArray, InputArray, OutputArray, OutputArray, OutputArray, const Ptr<EstimateParameters> &)(InputArray, Primitive, InputArray, InputArray, OutputArray, OutputArray, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:88
// ("cv::aruco::estimatePoseSingleMarkers", vec![(pred!(mut, ["corners", "markerLength", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "objPoints", "estimateParameters"], ["const cv::_InputArray*", "float", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::Ptr<cv::aruco::EstimateParameters>*"]), _)]),
#[inline]
pub fn estimate_pose_single_markers(corners: &impl ToInputArray, marker_length: f32, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, obj_points: &mut impl ToOutputArray, estimate_parameters: &core::Ptr<crate::aruco::EstimateParameters>) -> Result<()> {
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
// getBoardObjectAndImagePoints(const Ptr<Board> &, InputArrayOfArrays, InputArray, OutputArray, OutputArray)(CppPassByVoidPtr, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:51
// ("cv::aruco::getBoardObjectAndImagePoints", vec![(pred!(mut, ["board", "detectedCorners", "detectedIds", "objPoints", "imgPoints"], ["const cv::Ptr<cv::aruco::Board>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn get_board_object_and_image_points(board: &core::Ptr<crate::objdetect::Board>, detected_corners: &impl ToInputArray, detected_ids: &impl ToInputArray, obj_points: &mut impl ToOutputArray, img_points: &mut impl ToOutputArray) -> Result<()> {
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
// cv::aruco::interpolateCornersCharuco(InputArray, InputArray, InputArray, CppPassByVoidPtr, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/charuco.hpp:46
// ("cv::aruco::interpolateCornersCharuco", vec![(pred!(mut, ["markerCorners", "markerIds", "image", "board", "charucoCorners", "charucoIds"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn interpolate_corners_charuco_def(marker_corners: &impl ToInputArray, marker_ids: &impl ToInputArray, image: &impl ToInputArray, board: &core::Ptr<crate::objdetect::CharucoBoard>, charuco_corners: &mut impl ToOutputArray, charuco_ids: &mut impl ToOutputArray) -> Result<i32> {
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
// interpolateCornersCharuco(InputArrayOfArrays, InputArray, InputArray, const Ptr<CharucoBoard> &, OutputArray, OutputArray, InputArray, InputArray, int)(InputArray, InputArray, InputArray, CppPassByVoidPtr, OutputArray, OutputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/charuco.hpp:46
// ("cv::aruco::interpolateCornersCharuco", vec![(pred!(mut, ["markerCorners", "markerIds", "image", "board", "charucoCorners", "charucoIds", "cameraMatrix", "distCoeffs", "minMarkers"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
#[inline]
pub fn interpolate_corners_charuco(marker_corners: &impl ToInputArray, marker_ids: &impl ToInputArray, image: &impl ToInputArray, board: &core::Ptr<crate::objdetect::CharucoBoard>, charuco_corners: &mut impl ToOutputArray, charuco_ids: &mut impl ToOutputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, min_markers: i32) -> Result<i32> {
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
// cv::aruco::refineDetectedMarkers(InputArray, CppPassByVoidPtr, InputOutputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:34
// ("cv::aruco::refineDetectedMarkers", vec![(pred!(mut, ["image", "board", "detectedCorners", "detectedIds", "rejectedCorners"], ["const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
#[inline]
pub fn refine_detected_markers_def(image: &impl ToInputArray, board: &core::Ptr<crate::objdetect::Board>, detected_corners: &mut impl ToInputOutputArray, detected_ids: &mut impl ToInputOutputArray, rejected_corners: &mut impl ToInputOutputArray) -> Result<()> {
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
// refineDetectedMarkers(InputArray, const Ptr<Board> &, InputOutputArrayOfArrays, InputOutputArray, InputOutputArrayOfArrays, InputArray, InputArray, float, float, bool, OutputArray, const Ptr<DetectorParameters> &)(InputArray, CppPassByVoidPtr, InputOutputArray, InputOutputArray, InputOutputArray, InputArray, InputArray, Primitive, Primitive, Primitive, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:34
// ("cv::aruco::refineDetectedMarkers", vec![(pred!(mut, ["image", "board", "detectedCorners", "detectedIds", "rejectedCorners", "cameraMatrix", "distCoeffs", "minRepDistance", "errorCorrectionRate", "checkAllOrders", "recoveredIdxs", "parameters"], ["const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "float", "float", "bool", "const cv::_OutputArray*", "const cv::Ptr<cv::aruco::DetectorParameters>*"]), _)]),
#[inline]
pub fn refine_detected_markers(image: &impl ToInputArray, board: &core::Ptr<crate::objdetect::Board>, detected_corners: &mut impl ToInputOutputArray, detected_ids: &mut impl ToInputOutputArray, rejected_corners: &mut impl ToInputOutputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, min_rep_distance: f32, error_correction_rate: f32, check_all_orders: bool, recovered_idxs: &mut impl ToOutputArray, parameters: &core::Ptr<crate::objdetect::DetectorParameters>) -> Result<()> {
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
// testCharucoCornersCollinear(const Ptr<CharucoBoard> &, InputArray)(CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco.hpp:96
// ("cv::aruco::testCharucoCornersCollinear", vec![(pred!(mut, ["board", "charucoIds"], ["const cv::Ptr<cv::aruco::CharucoBoard>*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn test_charuco_corners_collinear(board: &core::Ptr<crate::objdetect::CharucoBoard>, charuco_ids: &impl ToInputArray) -> Result<bool> {
	input_array_arg!(charuco_ids);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_testCharucoCornersCollinear_const_PtrLCharucoBoardGR_const__InputArrayR(board.as_raw_PtrOfCharucoBoard(), charuco_ids.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Constant methods for [crate::aruco::EstimateParameters]
// EstimateParameters /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:55
pub trait EstimateParametersTraitConst {
	fn as_raw_EstimateParameters(&self) -> *const c_void;

	// cv::aruco::EstimateParameters::pattern() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:56
	// ("cv::aruco::EstimateParameters::pattern", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pattern(&self) -> crate::aruco::PatternPositionType {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_EstimateParameters_propPattern_const(self.as_raw_EstimateParameters(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	// cv::aruco::EstimateParameters::useExtrinsicGuess() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:57
	// ("cv::aruco::EstimateParameters::useExtrinsicGuess", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn use_extrinsic_guess(&self) -> bool {
		let ret = unsafe { sys::cv_aruco_EstimateParameters_propUseExtrinsicGuess_const(self.as_raw_EstimateParameters()) };
		ret
	}

	// cv::aruco::EstimateParameters::solvePnPMethod() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:58
	// ("cv::aruco::EstimateParameters::solvePnPMethod", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn solve_pnp_method(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_EstimateParameters_propSolvePnPMethod_const(self.as_raw_EstimateParameters()) };
		ret
	}

}

/// Mutable methods for [crate::aruco::EstimateParameters]
pub trait EstimateParametersTrait: crate::aruco::EstimateParametersTraitConst {
	fn as_raw_mut_EstimateParameters(&mut self) -> *mut c_void;

	// cv::aruco::EstimateParameters::setPattern(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:56
	// ("cv::aruco::EstimateParameters::setPattern", vec![(pred!(mut, ["val"], ["const cv::aruco::PatternPositionType"]), _)]),
	#[inline]
	fn set_pattern(&mut self, val: crate::aruco::PatternPositionType) {
		let ret = unsafe { sys::cv_aruco_EstimateParameters_propPattern_const_PatternPositionType(self.as_raw_mut_EstimateParameters(), val) };
		ret
	}

	// cv::aruco::EstimateParameters::setUseExtrinsicGuess(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:57
	// ("cv::aruco::EstimateParameters::setUseExtrinsicGuess", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_use_extrinsic_guess(&mut self, val: bool) {
		let ret = unsafe { sys::cv_aruco_EstimateParameters_propUseExtrinsicGuess_const_bool(self.as_raw_mut_EstimateParameters(), val) };
		ret
	}

	// cv::aruco::EstimateParameters::setSolvePnPMethod(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:58
	// ("cv::aruco::EstimateParameters::setSolvePnPMethod", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_solve_pnp_method(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_EstimateParameters_propSolvePnPMethod_const_int(self.as_raw_mut_EstimateParameters(), val) };
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
///
///
/// **Deprecated**: Use Board::matchImagePoints and cv::solvePnP
/// ## See also
/// PatternPositionType, solvePnP()
#[deprecated = "Use Board::matchImagePoints and cv::solvePnP"]
// EstimateParameters /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:55
pub struct EstimateParameters {
	ptr: *mut c_void,
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

boxed_ref! { EstimateParameters, crate::aruco::EstimateParametersTraitConst, as_raw_EstimateParameters, crate::aruco::EstimateParametersTrait, as_raw_mut_EstimateParameters }

impl EstimateParameters {
	// EstimateParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/aruco/aruco_calib.hpp:60
	// ("cv::aruco::EstimateParameters::EstimateParameters", vec![(pred!(mut, [], []), _)]),
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
