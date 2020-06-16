#![allow(unused_parens)]
//! # ArUco Marker Detection
//! This module is dedicated to square fiducial markers (also known as Augmented Reality Markers)
//! These markers are useful for easy, fast and robust camera pose estimation.ç
//! 
//! The main functionalities are:
//! - Detection of markers in an image
//! - Pose estimation from a single marker or from a board/set of markers
//! - Detection of ChArUco board for high subpixel accuracy
//! - Camera calibration from both, ArUco boards and ChArUco boards.
//! - Detection of ChArUco diamond markers
//! The samples directory includes easy examples of how to use the module.
//! 
//! The implementation is based on the ArUco Library by R. Muñoz-Salinas and S. Garrido-Jurado [Aruco2014](https://docs.opencv.org/3.4.10/d0/de3/citelist.html#CITEREF_Aruco2014).
//! 
//! Markers can also be detected based on the AprilTag 2 [wang2016iros](https://docs.opencv.org/3.4.10/d0/de3/citelist.html#CITEREF_wang2016iros) fiducial detection method.
//! ## See also
//! S. Garrido-Jurado, R. Muñoz-Salinas, F. J. Madrid-Cuevas, and M. J. Marín-Jiménez. 2014.
//! "Automatic generation and detection of highly reliable fiducial markers under occlusion".
//! Pattern Recogn. 47, 6 (June 2014), 2280-2292. DOI=10.1016/j.patcog.2014.01.005
//! 
//! http://www.uco.es/investiga/grupos/ava/node/26
//! 
//! This module has been originally developed by Sergio Garrido-Jurado as a project
//! for Google Summer of Code 2015 (GSoC 15).
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::DictionaryTrait, super::DetectorParametersTrait, super::BoardTrait, super::GridBoardTrait, super::CharucoBoardTrait };
}

/// Tag and corners detection based on the AprilTag 2 approach [wang2016iros](https://docs.opencv.org/3.4.10/d0/de3/citelist.html#CITEREF_wang2016iros)
pub const CORNER_REFINE_APRILTAG: i32 = 3;
/// ArUco approach and refine the corners locations using the contour-points line fitting
pub const CORNER_REFINE_CONTOUR: i32 = 2;
/// Tag and corners detection based on the ArUco approach
pub const CORNER_REFINE_NONE: i32 = 0;
/// ArUco approach and refine the corners locations using corner subpixel accuracy
pub const CORNER_REFINE_SUBPIX: i32 = 1;
pub const DICT_4X4_100: i32 = 1;
pub const DICT_4X4_1000: i32 = 3;
pub const DICT_4X4_250: i32 = 2;
pub const DICT_4X4_50: i32 = 0;
pub const DICT_5X5_100: i32 = 5;
pub const DICT_5X5_1000: i32 = 7;
pub const DICT_5X5_250: i32 = 6;
pub const DICT_5X5_50: i32 = 4;
pub const DICT_6X6_100: i32 = 9;
pub const DICT_6X6_1000: i32 = 11;
pub const DICT_6X6_250: i32 = 10;
pub const DICT_6X6_50: i32 = 8;
pub const DICT_7X7_100: i32 = 13;
pub const DICT_7X7_1000: i32 = 15;
pub const DICT_7X7_250: i32 = 14;
pub const DICT_7X7_50: i32 = 12;
/// 4x4 bits, minimum hamming distance between any two codes = 5, 30 codes
pub const DICT_APRILTAG_16h5: i32 = 17;
/// 5x5 bits, minimum hamming distance between any two codes = 9, 35 codes
pub const DICT_APRILTAG_25h9: i32 = 18;
/// 6x6 bits, minimum hamming distance between any two codes = 10, 2320 codes
pub const DICT_APRILTAG_36h10: i32 = 19;
/// 6x6 bits, minimum hamming distance between any two codes = 11, 587 codes
pub const DICT_APRILTAG_36h11: i32 = 20;
pub const DICT_ARUCO_ORIGINAL: i32 = 16;
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CornerRefineMethod {
	/// Tag and corners detection based on the ArUco approach
	CORNER_REFINE_NONE = 0,
	/// ArUco approach and refine the corners locations using corner subpixel accuracy
	CORNER_REFINE_SUBPIX = 1,
	/// ArUco approach and refine the corners locations using the contour-points line fitting
	CORNER_REFINE_CONTOUR = 2,
	/// Tag and corners detection based on the AprilTag 2 approach [wang2016iros](https://docs.opencv.org/3.4.10/d0/de3/citelist.html#CITEREF_wang2016iros)
	CORNER_REFINE_APRILTAG = 3,
}

opencv_type_enum! { crate::aruco::CornerRefineMethod }

/// Predefined markers dictionaries/sets
/// Each dictionary indicates the number of bits and the number of markers contained
/// - DICT_ARUCO_ORIGINAL: standard ArUco Library Markers. 1024 markers, 5x5 bits, 0 minimum
///                        distance
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PREDEFINED_DICTIONARY_NAME {
	DICT_4X4_50 = 0,
	DICT_4X4_100 = 1,
	DICT_4X4_250 = 2,
	DICT_4X4_1000 = 3,
	DICT_5X5_50 = 4,
	DICT_5X5_100 = 5,
	DICT_5X5_250 = 6,
	DICT_5X5_1000 = 7,
	DICT_6X6_50 = 8,
	DICT_6X6_100 = 9,
	DICT_6X6_250 = 10,
	DICT_6X6_1000 = 11,
	DICT_7X7_50 = 12,
	DICT_7X7_100 = 13,
	DICT_7X7_250 = 14,
	DICT_7X7_1000 = 15,
	DICT_ARUCO_ORIGINAL = 16,
	/// 4x4 bits, minimum hamming distance between any two codes = 5, 30 codes
	DICT_APRILTAG_16h5 = 17,
	/// 5x5 bits, minimum hamming distance between any two codes = 9, 35 codes
	DICT_APRILTAG_25h9 = 18,
	/// 6x6 bits, minimum hamming distance between any two codes = 10, 2320 codes
	DICT_APRILTAG_36h10 = 19,
	/// 6x6 bits, minimum hamming distance between any two codes = 11, 587 codes
	DICT_APRILTAG_36h11 = 20,
}

opencv_type_enum! { crate::aruco::PREDEFINED_DICTIONARY_NAME }

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
/// * flags: flags Different flags  for the calibration process (see #calibrateCamera for details).
/// * criteria: Termination criteria for the iterative optimization algorithm.
/// 
/// This function calibrates a camera using an Aruco Board. The function receives a list of
/// detected markers from several views of the Board. The process is similar to the chessboard
/// calibration in calibrateCamera(). The function returns the final re-projection error.
/// 
/// ## C++ default parameters
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
pub fn calibrate_camera_aruco_extended(corners: &dyn core::ToInputArray, ids: &dyn core::ToInputArray, counter: &dyn core::ToInputArray, board: &core::Ptr::<crate::aruco::Board>, image_size: core::Size, camera_matrix: &mut dyn core::ToInputOutputArray, dist_coeffs: &mut dyn core::ToInputOutputArray, rvecs: &mut dyn core::ToOutputArray, tvecs: &mut dyn core::ToOutputArray, std_deviations_intrinsics: &mut dyn core::ToOutputArray, std_deviations_extrinsics: &mut dyn core::ToOutputArray, per_view_errors: &mut dyn core::ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
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
	unsafe { sys::cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_Ptr_Board_R_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(corners.as_raw__InputArray(), ids.as_raw__InputArray(), counter.as_raw__InputArray(), board.as_raw_PtrOfBoard(), image_size.opencv_to_extern(), camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), std_deviations_intrinsics.as_raw__OutputArray(), std_deviations_extrinsics.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), flags, criteria.opencv_to_extern()) }.into_result()
}

/// It's the same function as #calibrateCameraAruco but without calibration error estimation.
/// 
/// ## C++ default parameters
/// * rvecs: noArray()
/// * tvecs: noArray()
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
pub fn calibrate_camera_aruco(corners: &dyn core::ToInputArray, ids: &dyn core::ToInputArray, counter: &dyn core::ToInputArray, board: &core::Ptr::<crate::aruco::Board>, image_size: core::Size, camera_matrix: &mut dyn core::ToInputOutputArray, dist_coeffs: &mut dyn core::ToInputOutputArray, rvecs: &mut dyn core::ToOutputArray, tvecs: &mut dyn core::ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
	input_array_arg!(corners);
	input_array_arg!(ids);
	input_array_arg!(counter);
	input_output_array_arg!(camera_matrix);
	input_output_array_arg!(dist_coeffs);
	output_array_arg!(rvecs);
	output_array_arg!(tvecs);
	unsafe { sys::cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_Ptr_Board_R_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(corners.as_raw__InputArray(), ids.as_raw__InputArray(), counter.as_raw__InputArray(), board.as_raw_PtrOfBoard(), image_size.opencv_to_extern(), camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, criteria.opencv_to_extern()) }.into_result()
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
/// * flags: flags Different flags  for the calibration process (see #calibrateCamera for details).
/// * criteria: Termination criteria for the iterative optimization algorithm.
/// 
/// This function calibrates a camera using a set of corners of a  Charuco Board. The function
/// receives a list of detected corners and its identifiers from several views of the Board.
/// The function returns the final re-projection error.
/// 
/// ## C++ default parameters
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
pub fn calibrate_camera_charuco_extended(charuco_corners: &dyn core::ToInputArray, charuco_ids: &dyn core::ToInputArray, board: &core::Ptr::<crate::aruco::CharucoBoard>, image_size: core::Size, camera_matrix: &mut dyn core::ToInputOutputArray, dist_coeffs: &mut dyn core::ToInputOutputArray, rvecs: &mut dyn core::ToOutputArray, tvecs: &mut dyn core::ToOutputArray, std_deviations_intrinsics: &mut dyn core::ToOutputArray, std_deviations_extrinsics: &mut dyn core::ToOutputArray, per_view_errors: &mut dyn core::ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
	input_array_arg!(charuco_corners);
	input_array_arg!(charuco_ids);
	input_output_array_arg!(camera_matrix);
	input_output_array_arg!(dist_coeffs);
	output_array_arg!(rvecs);
	output_array_arg!(tvecs);
	output_array_arg!(std_deviations_intrinsics);
	output_array_arg!(std_deviations_extrinsics);
	output_array_arg!(per_view_errors);
	unsafe { sys::cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_Ptr_CharucoBoard_R_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), image_size.opencv_to_extern(), camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), std_deviations_intrinsics.as_raw__OutputArray(), std_deviations_extrinsics.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), flags, criteria.opencv_to_extern()) }.into_result()
}

/// It's the same function as #calibrateCameraCharuco but without calibration error estimation.
/// 
/// ## C++ default parameters
/// * rvecs: noArray()
/// * tvecs: noArray()
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
pub fn calibrate_camera_charuco(charuco_corners: &dyn core::ToInputArray, charuco_ids: &dyn core::ToInputArray, board: &core::Ptr::<crate::aruco::CharucoBoard>, image_size: core::Size, camera_matrix: &mut dyn core::ToInputOutputArray, dist_coeffs: &mut dyn core::ToInputOutputArray, rvecs: &mut dyn core::ToOutputArray, tvecs: &mut dyn core::ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
	input_array_arg!(charuco_corners);
	input_array_arg!(charuco_ids);
	input_output_array_arg!(camera_matrix);
	input_output_array_arg!(dist_coeffs);
	output_array_arg!(rvecs);
	output_array_arg!(tvecs);
	unsafe { sys::cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_Ptr_CharucoBoard_R_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), image_size.opencv_to_extern(), camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, criteria.opencv_to_extern()) }.into_result()
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
/// 
/// This function detects Diamond markers from the previous detected ArUco markers. The diamonds
/// are returned in the diamondCorners and diamondIds parameters. If camera calibration parameters
/// are provided, the diamond search is based on reprojection. If not, diamond search is based on
/// homography. Homography is faster than reprojection but can slightly reduce the detection rate.
/// 
/// ## C++ default parameters
/// * camera_matrix: noArray()
/// * dist_coeffs: noArray()
pub fn detect_charuco_diamond(image: &dyn core::ToInputArray, marker_corners: &dyn core::ToInputArray, marker_ids: &dyn core::ToInputArray, square_marker_length_rate: f32, diamond_corners: &mut dyn core::ToOutputArray, diamond_ids: &mut dyn core::ToOutputArray, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(image);
	input_array_arg!(marker_corners);
	input_array_arg!(marker_ids);
	output_array_arg!(diamond_corners);
	output_array_arg!(diamond_ids);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	unsafe { sys::cv_aruco_detectCharucoDiamond_const__InputArrayR_const__InputArrayR_const__InputArrayR_float_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR(image.as_raw__InputArray(), marker_corners.as_raw__InputArray(), marker_ids.as_raw__InputArray(), square_marker_length_rate, diamond_corners.as_raw__OutputArray(), diamond_ids.as_raw__OutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray()) }.into_result()
}

/// Basic marker detection
/// 
/// ## Parameters
/// * image: input image
/// * dictionary: indicates the type of markers that will be searched
/// * corners: vector of detected marker corners. For each marker, its four corners
/// are provided, (e.g std::vector<std::vector<cv::Point2f> > ). For N detected markers,
/// the dimensions of this array is Nx4. The order of the corners is clockwise.
/// * ids: vector of identifiers of the detected markers. The identifier is of type int
/// (e.g. std::vector<int>). For N detected markers, the size of ids is also N.
/// The identifiers have the same order than the markers in the imgPoints array.
/// * parameters: marker detection parameters
/// * rejectedImgPoints: contains the imgPoints of those squares whose inner code has not a
/// correct codification. Useful for debugging purposes.
/// * cameraMatrix: optional input 3x3 floating-point camera matrix
/// ![inline formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D)
/// * distCoeff: optional vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5D%2C%5Bs%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5D%5D%29) of 4, 5, 8 or 12 elements
/// 
/// Performs marker detection in the input image. Only markers included in the specific dictionary
/// are searched. For each detected marker, it returns the 2D position of its corner in the image
/// and its corresponding identifier.
/// Note that this function does not perform pose estimation.
/// ## See also
/// estimatePoseSingleMarkers,  estimatePoseBoard
/// 
/// ## C++ default parameters
/// * parameters: DetectorParameters::create()
/// * rejected_img_points: noArray()
/// * camera_matrix: noArray()
/// * dist_coeff: noArray()
pub fn detect_markers(image: &dyn core::ToInputArray, dictionary: &core::Ptr::<crate::aruco::Dictionary>, corners: &mut dyn core::ToOutputArray, ids: &mut dyn core::ToOutputArray, parameters: &core::Ptr::<crate::aruco::DetectorParameters>, rejected_img_points: &mut dyn core::ToOutputArray, camera_matrix: &dyn core::ToInputArray, dist_coeff: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(image);
	output_array_arg!(corners);
	output_array_arg!(ids);
	output_array_arg!(rejected_img_points);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeff);
	unsafe { sys::cv_aruco_detectMarkers_const__InputArrayR_const_Ptr_Dictionary_R_const__OutputArrayR_const__OutputArrayR_const_Ptr_DetectorParameters_R_const__OutputArrayR_const__InputArrayR_const__InputArrayR(image.as_raw__InputArray(), dictionary.as_raw_PtrOfDictionary(), corners.as_raw__OutputArray(), ids.as_raw__OutputArray(), parameters.as_raw_PtrOfDetectorParameters(), rejected_img_points.as_raw__OutputArray(), camera_matrix.as_raw__InputArray(), dist_coeff.as_raw__InputArray()) }.into_result()
}

/// Draw coordinate system axis from pose estimation
/// 
/// ## Parameters
/// * image: input/output image. It must have 1 or 3 channels. The number of channels is not
/// altered.
/// * cameraMatrix: input 3x3 floating-point camera matrix
/// ![inline formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D)
/// * distCoeffs: vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5D%2C%5Bs%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5D%5D%29) of 4, 5, 8 or 12 elements
/// * rvec: rotation vector of the coordinate system that will be drawn. (see also: Rodrigues).
/// * tvec: translation vector of the coordinate system that will be drawn.
/// * length: length of the painted axis in the same unit than tvec (usually in meters)
/// 
/// Given the pose estimation of a marker or board, this function draws the axis of the world
/// coordinate system, i.e. the system centered on the marker/board. Useful for debugging purposes.
/// 
/// 
/// **Deprecated**: use cv::drawFrameAxes
#[deprecated = "use cv::drawFrameAxes"]
pub fn draw_axis(image: &mut dyn core::ToInputOutputArray, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, rvec: &dyn core::ToInputArray, tvec: &dyn core::ToInputArray, length: f32) -> Result<()> {
	input_output_array_arg!(image);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	input_array_arg!(rvec);
	input_array_arg!(tvec);
	unsafe { sys::cv_aruco_drawAxis_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_float(image.as_raw__InputOutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputArray(), tvec.as_raw__InputArray(), length) }.into_result()
}

/// ## C++ default parameters
/// * margin_size: 0
/// * border_bits: 1
pub fn draw_charuco_diamond(dictionary: &core::Ptr::<crate::aruco::Dictionary>, ids: core::Vec4i, square_length: i32, marker_length: i32, img: &mut dyn core::ToOutputArray, margin_size: i32, border_bits: i32) -> Result<()> {
	output_array_arg!(img);
	unsafe { sys::cv_aruco_drawCharucoDiamond_const_Ptr_Dictionary_R_Vec4i_int_int_const__OutputArrayR_int_int(dictionary.as_raw_PtrOfDictionary(), ids.opencv_to_extern(), square_length, marker_length, img.as_raw__OutputArray(), margin_size, border_bits) }.into_result()
}

/// Draws a set of Charuco corners
/// ## Parameters
/// * image: input/output image. It must have 1 or 3 channels. The number of channels is not
/// altered.
/// * charucoCorners: vector of detected charuco corners
/// * charucoIds: list of identifiers for each corner in charucoCorners
/// * cornerColor: color of the square surrounding each corner
/// 
/// This function draws a set of detected Charuco corners. If identifiers vector is provided, it also
/// draws the id of each corner.
/// 
/// ## C++ default parameters
/// * charuco_ids: noArray()
/// * corner_color: Scalar(255,0,0)
pub fn draw_detected_corners_charuco(image: &mut dyn core::ToInputOutputArray, charuco_corners: &dyn core::ToInputArray, charuco_ids: &dyn core::ToInputArray, corner_color: core::Scalar) -> Result<()> {
	input_output_array_arg!(image);
	input_array_arg!(charuco_corners);
	input_array_arg!(charuco_ids);
	unsafe { sys::cv_aruco_drawDetectedCornersCharuco_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(image.as_raw__InputOutputArray(), charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), corner_color.opencv_to_extern()) }.into_result()
}

/// Draw a set of detected ChArUco Diamond markers
/// 
/// ## Parameters
/// * image: input/output image. It must have 1 or 3 channels. The number of channels is not
/// altered.
/// * diamondCorners: positions of diamond corners in the same format returned by
/// detectCharucoDiamond(). (e.g std::vector<std::vector<cv::Point2f> > ). For N detected markers,
/// the dimensions of this array should be Nx4. The order of the corners should be clockwise.
/// * diamondIds: vector of identifiers for diamonds in diamondCorners, in the same format
/// returned by detectCharucoDiamond() (e.g. std::vector<Vec4i>).
/// Optional, if not provided, ids are not painted.
/// * borderColor: color of marker borders. Rest of colors (text color and first corner color)
/// are calculated based on this one.
/// 
/// Given an array of detected diamonds, this functions draws them in the image. The marker borders
/// are painted and the markers identifiers if provided.
/// Useful for debugging purposes.
/// 
/// ## C++ default parameters
/// * diamond_ids: noArray()
/// * border_color: Scalar(0,0,255)
pub fn draw_detected_diamonds(image: &mut dyn core::ToInputOutputArray, diamond_corners: &dyn core::ToInputArray, diamond_ids: &dyn core::ToInputArray, border_color: core::Scalar) -> Result<()> {
	input_output_array_arg!(image);
	input_array_arg!(diamond_corners);
	input_array_arg!(diamond_ids);
	unsafe { sys::cv_aruco_drawDetectedDiamonds_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(image.as_raw__InputOutputArray(), diamond_corners.as_raw__InputArray(), diamond_ids.as_raw__InputArray(), border_color.opencv_to_extern()) }.into_result()
}

/// Draw detected markers in image
/// 
/// ## Parameters
/// * image: input/output image. It must have 1 or 3 channels. The number of channels is not
/// altered.
/// * corners: positions of marker corners on input image.
/// (e.g std::vector<std::vector<cv::Point2f> > ). For N detected markers, the dimensions of
/// this array should be Nx4. The order of the corners should be clockwise.
/// * ids: vector of identifiers for markers in markersCorners .
/// Optional, if not provided, ids are not painted.
/// * borderColor: color of marker borders. Rest of colors (text color and first corner color)
/// are calculated based on this one to improve visualization.
/// 
/// Given an array of detected marker corners and its corresponding ids, this functions draws
/// the markers in the image. The marker borders are painted and the markers identifiers if provided.
/// Useful for debugging purposes.
/// 
/// ## C++ default parameters
/// * ids: noArray()
/// * border_color: Scalar(0,255,0)
pub fn draw_detected_markers(image: &mut dyn core::ToInputOutputArray, corners: &dyn core::ToInputArray, ids: &dyn core::ToInputArray, border_color: core::Scalar) -> Result<()> {
	input_output_array_arg!(image);
	input_array_arg!(corners);
	input_array_arg!(ids);
	unsafe { sys::cv_aruco_drawDetectedMarkers_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(image.as_raw__InputOutputArray(), corners.as_raw__InputArray(), ids.as_raw__InputArray(), border_color.opencv_to_extern()) }.into_result()
}

/// Draw a canonical marker image
/// 
/// ## Parameters
/// * dictionary: dictionary of markers indicating the type of markers
/// * id: identifier of the marker that will be returned. It has to be a valid id
/// in the specified dictionary.
/// * sidePixels: size of the image in pixels
/// * img: output image with the marker
/// * borderBits: width of the marker border.
/// 
/// This function returns a marker image in its canonical form (i.e. ready to be printed)
/// 
/// ## C++ default parameters
/// * border_bits: 1
pub fn draw_marker(dictionary: &core::Ptr::<crate::aruco::Dictionary>, id: i32, side_pixels: i32, img: &mut dyn core::ToOutputArray, border_bits: i32) -> Result<()> {
	output_array_arg!(img);
	unsafe { sys::cv_aruco_drawMarker_const_Ptr_Dictionary_R_int_int_const__OutputArrayR_int(dictionary.as_raw_PtrOfDictionary(), id, side_pixels, img.as_raw__OutputArray(), border_bits) }.into_result()
}

/// Draw a planar board
/// ## See also
/// _drawPlanarBoardImpl
/// 
/// ## Parameters
/// * board: layout of the board that will be drawn. The board should be planar,
/// z coordinate is ignored
/// * outSize: size of the output image in pixels.
/// * img: output image with the board. The size of this image will be outSize
/// and the board will be on the center, keeping the board proportions.
/// * marginSize: minimum margins (in pixels) of the board in the output image
/// * borderBits: width of the marker borders.
/// 
/// This function return the image of a planar board, ready to be printed. It assumes
/// the Board layout specified is planar by ignoring the z coordinates of the object points.
/// 
/// ## C++ default parameters
/// * margin_size: 0
/// * border_bits: 1
pub fn draw_planar_board(board: &core::Ptr::<crate::aruco::Board>, out_size: core::Size, img: &mut dyn core::ToOutputArray, margin_size: i32, border_bits: i32) -> Result<()> {
	output_array_arg!(img);
	unsafe { sys::cv_aruco_drawPlanarBoard_const_Ptr_Board_R_Size_const__OutputArrayR_int_int(board.as_raw_PtrOfBoard(), out_size.opencv_to_extern(), img.as_raw__OutputArray(), margin_size, border_bits) }.into_result()
}

/// Pose estimation for a board of markers
/// 
/// ## Parameters
/// * corners: vector of already detected markers corners. For each marker, its four corners
/// are provided, (e.g std::vector<std::vector<cv::Point2f> > ). For N detected markers, the
/// dimensions of this array should be Nx4. The order of the corners should be clockwise.
/// * ids: list of identifiers for each marker in corners
/// * board: layout of markers in the board. The layout is composed by the marker identifiers
/// and the positions of each marker corner in the board reference system.
/// * cameraMatrix: input 3x3 floating-point camera matrix
/// ![inline formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D)
/// * distCoeffs: vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5D%2C%5Bs%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5D%5D%29) of 4, 5, 8 or 12 elements
/// * rvec: Output vector (e.g. cv::Mat) corresponding to the rotation vector of the board
/// (see cv::Rodrigues). Used as initial guess if not empty.
/// * tvec: Output vector (e.g. cv::Mat) corresponding to the translation vector of the board.
/// * useExtrinsicGuess: defines whether initial guess for \b rvec and \b tvec will be used or not.
/// Used as initial guess if not empty.
/// 
/// This function receives the detected markers and returns the pose of a marker board composed
/// by those markers.
/// A Board of marker has a single world coordinate system which is defined by the board layout.
/// The returned transformation is the one that transforms points from the board coordinate system
/// to the camera coordinate system.
/// Input markers that are not included in the board layout are ignored.
/// The function returns the number of markers from the input employed for the board pose estimation.
/// Note that returning a 0 means the pose has not been estimated.
/// 
/// ## C++ default parameters
/// * use_extrinsic_guess: false
pub fn estimate_pose_board(corners: &dyn core::ToInputArray, ids: &dyn core::ToInputArray, board: &core::Ptr::<crate::aruco::Board>, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, rvec: &mut dyn core::ToOutputArray, tvec: &mut dyn core::ToOutputArray, use_extrinsic_guess: bool) -> Result<i32> {
	input_array_arg!(corners);
	input_array_arg!(ids);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(rvec);
	output_array_arg!(tvec);
	unsafe { sys::cv_aruco_estimatePoseBoard_const__InputArrayR_const__InputArrayR_const_Ptr_Board_R_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool(corners.as_raw__InputArray(), ids.as_raw__InputArray(), board.as_raw_PtrOfBoard(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), use_extrinsic_guess) }.into_result()
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
/// ## C++ default parameters
/// * use_extrinsic_guess: false
pub fn estimate_pose_charuco_board(charuco_corners: &dyn core::ToInputArray, charuco_ids: &dyn core::ToInputArray, board: &core::Ptr::<crate::aruco::CharucoBoard>, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, rvec: &mut dyn core::ToOutputArray, tvec: &mut dyn core::ToOutputArray, use_extrinsic_guess: bool) -> Result<bool> {
	input_array_arg!(charuco_corners);
	input_array_arg!(charuco_ids);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(rvec);
	output_array_arg!(tvec);
	unsafe { sys::cv_aruco_estimatePoseCharucoBoard_const__InputArrayR_const__InputArrayR_const_Ptr_CharucoBoard_R_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool(charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), use_extrinsic_guess) }.into_result()
}

/// Pose estimation for single markers
/// 
/// ## Parameters
/// * corners: vector of already detected markers corners. For each marker, its four corners
/// are provided, (e.g std::vector<std::vector<cv::Point2f> > ). For N detected markers,
/// the dimensions of this array should be Nx4. The order of the corners should be clockwise.
/// ## See also
/// detectMarkers
/// * markerLength: the length of the markers' side. The returning translation vectors will
/// be in the same unit. Normally, unit is meters.
/// * cameraMatrix: input 3x3 floating-point camera matrix
/// ![inline formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D)
/// * distCoeffs: vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5D%2C%5Bs%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5D%5D%29) of 4, 5, 8 or 12 elements
/// * rvecs: array of output rotation vectors (Rodrigues) (e.g. std::vector<cv::Vec3d>).
/// Each element in rvecs corresponds to the specific marker in imgPoints.
/// * tvecs: array of output translation vectors (e.g. std::vector<cv::Vec3d>).
/// Each element in tvecs corresponds to the specific marker in imgPoints.
/// * _objPoints: array of object points of all the marker corners
/// 
/// This function receives the detected markers and returns their pose estimation respect to
/// the camera individually. So for each marker, one rotation and translation vector is returned.
/// The returned transformation is the one that transforms points from each marker coordinate system
/// to the camera coordinate system.
/// The marker corrdinate system is centered on the middle of the marker, with the Z axis
/// perpendicular to the marker plane.
/// The coordinates of the four corners of the marker in its own coordinate system are:
/// (-markerLength/2, markerLength/2, 0), (markerLength/2, markerLength/2, 0),
/// (markerLength/2, -markerLength/2, 0), (-markerLength/2, -markerLength/2, 0)
/// 
/// ## C++ default parameters
/// * _obj_points: noArray()
pub fn estimate_pose_single_markers(corners: &dyn core::ToInputArray, marker_length: f32, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, rvecs: &mut dyn core::ToOutputArray, tvecs: &mut dyn core::ToOutputArray, _obj_points: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(corners);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(rvecs);
	output_array_arg!(tvecs);
	output_array_arg!(_obj_points);
	unsafe { sys::cv_aruco_estimatePoseSingleMarkers_const__InputArrayR_float_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(corners.as_raw__InputArray(), marker_length, camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), _obj_points.as_raw__OutputArray()) }.into_result()
}

/// Generates a new customizable marker dictionary
/// 
/// ## Parameters
/// * nMarkers: number of markers in the dictionary
/// * markerSize: number of bits per dimension of each markers
/// * baseDictionary: Include the markers in this dictionary at the beginning (optional)
/// * randomSeed: a user supplied seed for theRNG()
/// 
/// This function creates a new dictionary composed by nMarkers markers and each markers composed
/// by markerSize x markerSize bits. If baseDictionary is provided, its markers are directly
/// included and the rest are generated based on them. If the size of baseDictionary is higher
/// than nMarkers, only the first nMarkers in baseDictionary are taken and no new marker is added.
/// 
/// ## C++ default parameters
/// * random_seed: 0
pub fn custom_dictionary_from(n_markers: i32, marker_size: i32, base_dictionary: &core::Ptr::<crate::aruco::Dictionary>, random_seed: i32) -> Result<core::Ptr::<crate::aruco::Dictionary>> {
	unsafe { sys::cv_aruco_generateCustomDictionary_int_int_const_Ptr_Dictionary_R_int(n_markers, marker_size, base_dictionary.as_raw_PtrOfDictionary(), random_seed) }.into_result().map(|r| unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(r) } )
}

/// ## See also
/// generateCustomDictionary
/// 
/// ## C++ default parameters
/// * random_seed: 0
pub fn custom_dictionary(n_markers: i32, marker_size: i32, random_seed: i32) -> Result<core::Ptr::<crate::aruco::Dictionary>> {
	unsafe { sys::cv_aruco_generateCustomDictionary_int_int_int(n_markers, marker_size, random_seed) }.into_result().map(|r| unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(r) } )
}

/// Given a board configuration and a set of detected markers, returns the corresponding
/// image points and object points to call solvePnP
/// 
/// ## Parameters
/// * board: Marker board layout.
/// * detectedCorners: List of detected marker corners of the board.
/// * detectedIds: List of identifiers for each marker.
/// * objPoints: Vector of vectors of board marker points in the board coordinate space.
/// * imgPoints: Vector of vectors of the projections of board marker corner points.
pub fn get_board_object_and_image_points(board: &core::Ptr::<crate::aruco::Board>, detected_corners: &dyn core::ToInputArray, detected_ids: &dyn core::ToInputArray, obj_points: &mut dyn core::ToOutputArray, img_points: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(detected_corners);
	input_array_arg!(detected_ids);
	output_array_arg!(obj_points);
	output_array_arg!(img_points);
	unsafe { sys::cv_aruco_getBoardObjectAndImagePoints_const_Ptr_Board_R_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(board.as_raw_PtrOfBoard(), detected_corners.as_raw__InputArray(), detected_ids.as_raw__InputArray(), obj_points.as_raw__OutputArray(), img_points.as_raw__OutputArray()) }.into_result()
}

/// Returns one of the predefined dictionaries defined in PREDEFINED_DICTIONARY_NAME
pub fn get_predefined_dictionary(name: crate::aruco::PREDEFINED_DICTIONARY_NAME) -> Result<core::Ptr::<crate::aruco::Dictionary>> {
	unsafe { sys::cv_aruco_getPredefinedDictionary_PREDEFINED_DICTIONARY_NAME(name) }.into_result().map(|r| unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(r) } )
}

/// Returns one of the predefined dictionaries referenced by DICT_*.
pub fn get_predefined_dictionary_i32(dict: i32) -> Result<core::Ptr::<crate::aruco::Dictionary>> {
	unsafe { sys::cv_aruco_getPredefinedDictionary_int(dict) }.into_result().map(|r| unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(r) } )
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
/// ## C++ default parameters
/// * camera_matrix: noArray()
/// * dist_coeffs: noArray()
/// * min_markers: 2
pub fn interpolate_corners_charuco(marker_corners: &dyn core::ToInputArray, marker_ids: &dyn core::ToInputArray, image: &dyn core::ToInputArray, board: &core::Ptr::<crate::aruco::CharucoBoard>, charuco_corners: &mut dyn core::ToOutputArray, charuco_ids: &mut dyn core::ToOutputArray, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, min_markers: i32) -> Result<i32> {
	input_array_arg!(marker_corners);
	input_array_arg!(marker_ids);
	input_array_arg!(image);
	output_array_arg!(charuco_corners);
	output_array_arg!(charuco_ids);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	unsafe { sys::cv_aruco_interpolateCornersCharuco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_Ptr_CharucoBoard_R_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int(marker_corners.as_raw__InputArray(), marker_ids.as_raw__InputArray(), image.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), charuco_corners.as_raw__OutputArray(), charuco_ids.as_raw__OutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), min_markers) }.into_result()
}

/// Refind not detected markers based on the already detected and the board layout
/// 
/// ## Parameters
/// * image: input image
/// * board: layout of markers in the board.
/// * detectedCorners: vector of already detected marker corners.
/// * detectedIds: vector of already detected marker identifiers.
/// * rejectedCorners: vector of rejected candidates during the marker detection process.
/// * cameraMatrix: optional input 3x3 floating-point camera matrix
/// ![inline formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D)
/// * distCoeffs: optional vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%28k%5F1%2C%20k%5F2%2C%20p%5F1%2C%20p%5F2%5B%2C%20k%5F3%5B%2C%20k%5F4%2C%20k%5F5%2C%20k%5F6%5D%2C%5Bs%5F1%2C%20s%5F2%2C%20s%5F3%2C%20s%5F4%5D%5D%29) of 4, 5, 8 or 12 elements
/// * minRepDistance: minimum distance between the corners of the rejected candidate and the
/// reprojected marker in order to consider it as a correspondence.
/// * errorCorrectionRate: rate of allowed erroneous bits respect to the error correction
/// capability of the used dictionary. -1 ignores the error correction step.
/// * checkAllOrders: Consider the four posible corner orders in the rejectedCorners array.
/// If it set to false, only the provided corner order is considered (default true).
/// * recoveredIdxs: Optional array to returns the indexes of the recovered candidates in the
/// original rejectedCorners array.
/// * parameters: marker detection parameters
/// 
/// This function tries to find markers that were not detected in the basic detecMarkers function.
/// First, based on the current detected marker and the board layout, the function interpolates
/// the position of the missing markers. Then it tries to find correspondence between the reprojected
/// markers and the rejected candidates based on the minRepDistance and errorCorrectionRate
/// parameters.
/// If camera parameters and distortion coefficients are provided, missing markers are reprojected
/// using projectPoint function. If not, missing marker projections are interpolated using global
/// homography, and all the marker corners in the board must have the same Z coordinate.
/// 
/// ## C++ default parameters
/// * camera_matrix: noArray()
/// * dist_coeffs: noArray()
/// * min_rep_distance: 10.f
/// * error_correction_rate: 3.f
/// * check_all_orders: true
/// * recovered_idxs: noArray()
/// * parameters: DetectorParameters::create()
pub fn refine_detected_markers(image: &dyn core::ToInputArray, board: &core::Ptr::<crate::aruco::Board>, detected_corners: &mut dyn core::ToInputOutputArray, detected_ids: &mut dyn core::ToInputOutputArray, rejected_corners: &mut dyn core::ToInputOutputArray, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, min_rep_distance: f32, error_correction_rate: f32, check_all_orders: bool, recovered_idxs: &mut dyn core::ToOutputArray, parameters: &core::Ptr::<crate::aruco::DetectorParameters>) -> Result<()> {
	input_array_arg!(image);
	input_output_array_arg!(detected_corners);
	input_output_array_arg!(detected_ids);
	input_output_array_arg!(rejected_corners);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(recovered_idxs);
	unsafe { sys::cv_aruco_refineDetectedMarkers_const__InputArrayR_const_Ptr_Board_R_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_float_float_bool_const__OutputArrayR_const_Ptr_DetectorParameters_R(image.as_raw__InputArray(), board.as_raw_PtrOfBoard(), detected_corners.as_raw__InputOutputArray(), detected_ids.as_raw__InputOutputArray(), rejected_corners.as_raw__InputOutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), min_rep_distance, error_correction_rate, check_all_orders, recovered_idxs.as_raw__OutputArray(), parameters.as_raw_PtrOfDetectorParameters()) }.into_result()
}

/// Board of markers
/// 
/// A board is a set of markers in the 3D space with a common coordinate system.
/// The common form of a board of marker is a planar (2D) board, however any 3D layout can be used.
/// A Board object is composed by:
/// - The object points of the marker corners, i.e. their coordinates respect to the board system.
/// - The dictionary which indicates the type of markers of the board
/// - The identifier of all the markers in the board.
pub trait BoardTrait {
	fn as_raw_Board(&self) -> *const c_void;
	fn as_raw_mut_Board(&mut self) -> *mut c_void;

	/// array of object points of all the marker corners in the board
	/// each marker include its 4 corners in CCW order. For M markers, the size is Mx4.
	fn obj_points(&mut self) -> core::Vector::<core::Vector::<core::Point3f>> {
		unsafe { sys::cv_aruco_Board_getPropObjPoints(self.as_raw_mut_Board()) }.into_result().map(|r| unsafe { core::Vector::<core::Vector::<core::Point3f>>::opencv_from_extern(r) } ).expect("Infallible function failed: obj_points")
	}
	
	/// array of object points of all the marker corners in the board
	/// each marker include its 4 corners in CCW order. For M markers, the size is Mx4.
	fn set_obj_points(&mut self, mut val: core::Vector::<core::Vector::<core::Point3f>>) -> () {
		unsafe { sys::cv_aruco_Board_setPropObjPoints_vector_vector_Point3f__(self.as_raw_mut_Board(), val.as_raw_mut_VectorOfVectorOfPoint3f()) }.into_result().expect("Infallible function failed: set_obj_points")
	}
	
	/// the dictionary of markers employed for this board
	fn dictionary(&mut self) -> core::Ptr::<crate::aruco::Dictionary> {
		unsafe { sys::cv_aruco_Board_getPropDictionary(self.as_raw_mut_Board()) }.into_result().map(|r| unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(r) } ).expect("Infallible function failed: dictionary")
	}
	
	/// the dictionary of markers employed for this board
	fn set_dictionary(&mut self, mut val: core::Ptr::<crate::aruco::Dictionary>) -> () {
		unsafe { sys::cv_aruco_Board_setPropDictionary_Ptr_Dictionary_(self.as_raw_mut_Board(), val.as_raw_mut_PtrOfDictionary()) }.into_result().expect("Infallible function failed: set_dictionary")
	}
	
	/// vector of the identifiers of the markers in the board (same size than objPoints)
	/// The identifiers refers to the board dictionary
	fn ids(&mut self) -> core::Vector::<i32> {
		unsafe { sys::cv_aruco_Board_getPropIds(self.as_raw_mut_Board()) }.into_result().map(|r| unsafe { core::Vector::<i32>::opencv_from_extern(r) } ).expect("Infallible function failed: ids")
	}
	
	/// vector of the identifiers of the markers in the board (same size than objPoints)
	/// The identifiers refers to the board dictionary
	fn set_ids(&mut self, mut val: core::Vector::<i32>) -> () {
		unsafe { sys::cv_aruco_Board_setPropIds_vector_int_(self.as_raw_mut_Board(), val.as_raw_mut_VectorOfi32()) }.into_result().expect("Infallible function failed: set_ids")
	}
	
}

/// Board of markers
/// 
/// A board is a set of markers in the 3D space with a common coordinate system.
/// The common form of a board of marker is a planar (2D) board, however any 3D layout can be used.
/// A Board object is composed by:
/// - The object points of the marker corners, i.e. their coordinates respect to the board system.
/// - The dictionary which indicates the type of markers of the board
/// - The identifier of all the markers in the board.
pub struct Board {
	ptr: *mut c_void
}

opencv_type_boxed! { Board }

impl Drop for Board {
	fn drop(&mut self) {
		extern "C" { fn cv_Board_delete(instance: *mut c_void); }
		unsafe { cv_Board_delete(self.as_raw_mut_Board()) };
	}
}

impl Board {
	#[inline] pub fn as_raw_Board(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Board(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Board {}

impl crate::aruco::BoardTrait for Board {
	#[inline] fn as_raw_Board(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Board {
	/// Provide way to create Board by passing necessary data. Specially needed in Python.
	/// 
	/// ## Parameters
	/// * objPoints: array of object points of all the marker corners in the board
	/// * dictionary: the dictionary of markers employed for this board
	/// * ids: vector of the identifiers of the markers in the board
	pub fn create(obj_points: &dyn core::ToInputArray, dictionary: &core::Ptr::<crate::aruco::Dictionary>, ids: &dyn core::ToInputArray) -> Result<core::Ptr::<crate::aruco::Board>> {
		input_array_arg!(obj_points);
		input_array_arg!(ids);
		unsafe { sys::cv_aruco_Board_create_const__InputArrayR_const_Ptr_Dictionary_R_const__InputArrayR(obj_points.as_raw__InputArray(), dictionary.as_raw_PtrOfDictionary(), ids.as_raw__InputArray()) }.into_result().map(|r| unsafe { core::Ptr::<crate::aruco::Board>::opencv_from_extern(r) } )
	}
	
}

/// ChArUco board
/// Specific class for ChArUco boards. A ChArUco board is a planar board where the markers are placed
/// inside the white squares of a chessboard. The benefits of ChArUco boards is that they provide
/// both, ArUco markers versatility and chessboard corner precision, which is important for
/// calibration and pose estimation.
/// This class also allows the easy creation and drawing of ChArUco boards.
pub trait CharucoBoardTrait: crate::aruco::BoardTrait {
	fn as_raw_CharucoBoard(&self) -> *const c_void;
	fn as_raw_mut_CharucoBoard(&mut self) -> *mut c_void;

	fn chessboard_corners(&mut self) -> core::Vector::<core::Point3f> {
		unsafe { sys::cv_aruco_CharucoBoard_getPropChessboardCorners(self.as_raw_mut_CharucoBoard()) }.into_result().map(|r| unsafe { core::Vector::<core::Point3f>::opencv_from_extern(r) } ).expect("Infallible function failed: chessboard_corners")
	}
	
	fn set_chessboard_corners(&mut self, mut val: core::Vector::<core::Point3f>) -> () {
		unsafe { sys::cv_aruco_CharucoBoard_setPropChessboardCorners_vector_Point3f_(self.as_raw_mut_CharucoBoard(), val.as_raw_mut_VectorOfPoint3f()) }.into_result().expect("Infallible function failed: set_chessboard_corners")
	}
	
	fn nearest_marker_idx(&mut self) -> core::Vector::<core::Vector::<i32>> {
		unsafe { sys::cv_aruco_CharucoBoard_getPropNearestMarkerIdx(self.as_raw_mut_CharucoBoard()) }.into_result().map(|r| unsafe { core::Vector::<core::Vector::<i32>>::opencv_from_extern(r) } ).expect("Infallible function failed: nearest_marker_idx")
	}
	
	fn set_nearest_marker_idx(&mut self, mut val: core::Vector::<core::Vector::<i32>>) -> () {
		unsafe { sys::cv_aruco_CharucoBoard_setPropNearestMarkerIdx_vector_vector_int__(self.as_raw_mut_CharucoBoard(), val.as_raw_mut_VectorOfVectorOfi32()) }.into_result().expect("Infallible function failed: set_nearest_marker_idx")
	}
	
	fn nearest_marker_corners(&mut self) -> core::Vector::<core::Vector::<i32>> {
		unsafe { sys::cv_aruco_CharucoBoard_getPropNearestMarkerCorners(self.as_raw_mut_CharucoBoard()) }.into_result().map(|r| unsafe { core::Vector::<core::Vector::<i32>>::opencv_from_extern(r) } ).expect("Infallible function failed: nearest_marker_corners")
	}
	
	fn set_nearest_marker_corners(&mut self, mut val: core::Vector::<core::Vector::<i32>>) -> () {
		unsafe { sys::cv_aruco_CharucoBoard_setPropNearestMarkerCorners_vector_vector_int__(self.as_raw_mut_CharucoBoard(), val.as_raw_mut_VectorOfVectorOfi32()) }.into_result().expect("Infallible function failed: set_nearest_marker_corners")
	}
	
	/// Draw a ChArUco board
	/// 
	/// ## Parameters
	/// * outSize: size of the output image in pixels.
	/// * img: output image with the board. The size of this image will be outSize
	/// and the board will be on the center, keeping the board proportions.
	/// * marginSize: minimum margins (in pixels) of the board in the output image
	/// * borderBits: width of the marker borders.
	/// 
	/// This function return the image of the ChArUco board, ready to be printed.
	/// 
	/// ## C++ default parameters
	/// * margin_size: 0
	/// * border_bits: 1
	fn draw(&mut self, out_size: core::Size, img: &mut dyn core::ToOutputArray, margin_size: i32, border_bits: i32) -> Result<()> {
		output_array_arg!(img);
		unsafe { sys::cv_aruco_CharucoBoard_draw_Size_const__OutputArrayR_int_int(self.as_raw_mut_CharucoBoard(), out_size.opencv_to_extern(), img.as_raw__OutputArray(), margin_size, border_bits) }.into_result()
	}
	
	fn get_chessboard_size(&self) -> Result<core::Size> {
		unsafe { sys::cv_aruco_CharucoBoard_getChessboardSize_const(self.as_raw_CharucoBoard()) }.into_result()
	}
	
	fn get_square_length(&self) -> Result<f32> {
		unsafe { sys::cv_aruco_CharucoBoard_getSquareLength_const(self.as_raw_CharucoBoard()) }.into_result()
	}
	
	fn get_marker_length(&self) -> Result<f32> {
		unsafe { sys::cv_aruco_CharucoBoard_getMarkerLength_const(self.as_raw_CharucoBoard()) }.into_result()
	}
	
}

/// ChArUco board
/// Specific class for ChArUco boards. A ChArUco board is a planar board where the markers are placed
/// inside the white squares of a chessboard. The benefits of ChArUco boards is that they provide
/// both, ArUco markers versatility and chessboard corner precision, which is important for
/// calibration and pose estimation.
/// This class also allows the easy creation and drawing of ChArUco boards.
pub struct CharucoBoard {
	ptr: *mut c_void
}

opencv_type_boxed! { CharucoBoard }

impl Drop for CharucoBoard {
	fn drop(&mut self) {
		extern "C" { fn cv_CharucoBoard_delete(instance: *mut c_void); }
		unsafe { cv_CharucoBoard_delete(self.as_raw_mut_CharucoBoard()) };
	}
}

impl CharucoBoard {
	#[inline] pub fn as_raw_CharucoBoard(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_CharucoBoard(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for CharucoBoard {}

impl crate::aruco::BoardTrait for CharucoBoard {
	#[inline] fn as_raw_Board(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::aruco::CharucoBoardTrait for CharucoBoard {
	#[inline] fn as_raw_CharucoBoard(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_CharucoBoard(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CharucoBoard {
	/// Create a CharucoBoard object
	/// 
	/// ## Parameters
	/// * squaresX: number of chessboard squares in X direction
	/// * squaresY: number of chessboard squares in Y direction
	/// * squareLength: chessboard square side length (normally in meters)
	/// * markerLength: marker side length (same unit than squareLength)
	/// * dictionary: dictionary of markers indicating the type of markers.
	/// The first markers in the dictionary are used to fill the white chessboard squares.
	/// ## Returns
	/// the output CharucoBoard object
	/// 
	/// This functions creates a CharucoBoard object given the number of squares in each direction
	/// and the size of the markers and chessboard squares.
	pub fn create(squares_x: i32, squares_y: i32, square_length: f32, marker_length: f32, dictionary: &core::Ptr::<crate::aruco::Dictionary>) -> Result<core::Ptr::<crate::aruco::CharucoBoard>> {
		unsafe { sys::cv_aruco_CharucoBoard_create_int_int_float_float_const_Ptr_Dictionary_R(squares_x, squares_y, square_length, marker_length, dictionary.as_raw_PtrOfDictionary()) }.into_result().map(|r| unsafe { core::Ptr::<crate::aruco::CharucoBoard>::opencv_from_extern(r) } )
	}
	
}

/// Parameters for the detectMarker process:
/// - adaptiveThreshWinSizeMin: minimum window size for adaptive thresholding before finding
///   contours (default 3).
/// - adaptiveThreshWinSizeMax: maximum window size for adaptive thresholding before finding
///   contours (default 23).
/// - adaptiveThreshWinSizeStep: increments from adaptiveThreshWinSizeMin to adaptiveThreshWinSizeMax
///   during the thresholding (default 10).
/// - adaptiveThreshConstant: constant for adaptive thresholding before finding contours (default 7)
/// - minMarkerPerimeterRate: determine minimum perimeter for marker contour to be detected. This
///   is defined as a rate respect to the maximum dimension of the input image (default 0.03).
/// - maxMarkerPerimeterRate:  determine maximum perimeter for marker contour to be detected. This
///   is defined as a rate respect to the maximum dimension of the input image (default 4.0).
/// - polygonalApproxAccuracyRate: minimum accuracy during the polygonal approximation process to
///   determine which contours are squares.
/// - minCornerDistanceRate: minimum distance between corners for detected markers relative to its
///   perimeter (default 0.05)
/// - minDistanceToBorder: minimum distance of any corner to the image border for detected markers
///   (in pixels) (default 3)
/// - minMarkerDistanceRate: minimum mean distance beetween two marker corners to be considered
///   similar, so that the smaller one is removed. The rate is relative to the smaller perimeter
///   of the two markers (default 0.05).
/// - cornerRefinementMethod: corner refinement method. (CORNER_REFINE_NONE, no refinement.
///   CORNER_REFINE_SUBPIX, do subpixel refinement. CORNER_REFINE_CONTOUR use contour-Points,
///   CORNER_REFINE_APRILTAG  use the AprilTag2 approach)
/// - cornerRefinementWinSize: window size for the corner refinement process (in pixels) (default 5).
/// - cornerRefinementMaxIterations: maximum number of iterations for stop criteria of the corner
///   refinement process (default 30).
/// - cornerRefinementMinAccuracy: minimum error for the stop cristeria of the corner refinement
///   process (default: 0.1)
/// - markerBorderBits: number of bits of the marker border, i.e. marker border width (default 1).
/// - perspectiveRemovePixelPerCell: number of bits (per dimension) for each cell of the marker
///   when removing the perspective (default 8).
/// - perspectiveRemoveIgnoredMarginPerCell: width of the margin of pixels on each cell not
///   considered for the determination of the cell bit. Represents the rate respect to the total
///   size of the cell, i.e. perspectiveRemovePixelPerCell (default 0.13)
/// - maxErroneousBitsInBorderRate: maximum number of accepted erroneous bits in the border (i.e.
///   number of allowed white bits in the border). Represented as a rate respect to the total
///   number of bits per marker (default 0.35).
/// - minOtsuStdDev: minimun standard deviation in pixels values during the decodification step to
///   apply Otsu thresholding (otherwise, all the bits are set to 0 or 1 depending on mean higher
///   than 128 or not) (default 5.0)
/// - errorCorrectionRate error correction rate respect to the maximun error correction capability
///   for each dictionary. (default 0.6).
/// - aprilTagMinClusterPixels: reject quads containing too few pixels.
/// - aprilTagMaxNmaxima: how many corner candidates to consider when segmenting a group of pixels into a quad.
/// - aprilTagCriticalRad: Reject quads where pairs of edges have angles that are close to straight or close to
///   180 degrees. Zero means that no quads are rejected. (In radians).
/// - aprilTagMaxLineFitMse:  When fitting lines to the contours, what is the maximum mean squared error
///   allowed?  This is useful in rejecting contours that are far from being quad shaped; rejecting
///   these quads "early" saves expensive decoding processing.
/// - aprilTagMinWhiteBlackDiff: When we build our model of black & white pixels, we add an extra check that
///   the white model must be (overall) brighter than the black model.  How much brighter? (in pixel values, [0,255]).
/// - aprilTagDeglitch:  should the thresholded image be deglitched? Only useful for very noisy images
/// - aprilTagQuadDecimate: Detection of quads can be done on a lower-resolution image, improving speed at a
///   cost of pose accuracy and a slight decrease in detection rate. Decoding the binary payload is still
///   done at full resolution.
/// - aprilTagQuadSigma: What Gaussian blur should be applied to the segmented image (used for quad detection?)
///   Parameter is the standard deviation in pixels.  Very noisy images benefit from non-zero values (e.g. 0.8).
/// - detectInvertedMarker: to check if there is a white marker. In order to generate a "white" marker just
///   invert a normal marker by using a tilde, ~markerImage. (default false)
pub trait DetectorParametersTrait {
	fn as_raw_DetectorParameters(&self) -> *const c_void;
	fn as_raw_mut_DetectorParameters(&mut self) -> *mut c_void;

	fn adaptive_thresh_win_size_min(&self) -> i32 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropAdaptiveThreshWinSizeMin_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: adaptive_thresh_win_size_min")
	}
	
	fn set_adaptive_thresh_win_size_min(&mut self, val: i32) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropAdaptiveThreshWinSizeMin_int(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_adaptive_thresh_win_size_min")
	}
	
	fn adaptive_thresh_win_size_max(&self) -> i32 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropAdaptiveThreshWinSizeMax_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: adaptive_thresh_win_size_max")
	}
	
	fn set_adaptive_thresh_win_size_max(&mut self, val: i32) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropAdaptiveThreshWinSizeMax_int(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_adaptive_thresh_win_size_max")
	}
	
	fn adaptive_thresh_win_size_step(&self) -> i32 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropAdaptiveThreshWinSizeStep_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: adaptive_thresh_win_size_step")
	}
	
	fn set_adaptive_thresh_win_size_step(&mut self, val: i32) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropAdaptiveThreshWinSizeStep_int(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_adaptive_thresh_win_size_step")
	}
	
	fn adaptive_thresh_constant(&self) -> f64 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropAdaptiveThreshConstant_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: adaptive_thresh_constant")
	}
	
	fn set_adaptive_thresh_constant(&mut self, val: f64) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropAdaptiveThreshConstant_double(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_adaptive_thresh_constant")
	}
	
	fn min_marker_perimeter_rate(&self) -> f64 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropMinMarkerPerimeterRate_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: min_marker_perimeter_rate")
	}
	
	fn set_min_marker_perimeter_rate(&mut self, val: f64) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropMinMarkerPerimeterRate_double(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_min_marker_perimeter_rate")
	}
	
	fn max_marker_perimeter_rate(&self) -> f64 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropMaxMarkerPerimeterRate_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: max_marker_perimeter_rate")
	}
	
	fn set_max_marker_perimeter_rate(&mut self, val: f64) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropMaxMarkerPerimeterRate_double(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_max_marker_perimeter_rate")
	}
	
	fn polygonal_approx_accuracy_rate(&self) -> f64 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropPolygonalApproxAccuracyRate_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: polygonal_approx_accuracy_rate")
	}
	
	fn set_polygonal_approx_accuracy_rate(&mut self, val: f64) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropPolygonalApproxAccuracyRate_double(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_polygonal_approx_accuracy_rate")
	}
	
	fn min_corner_distance_rate(&self) -> f64 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropMinCornerDistanceRate_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: min_corner_distance_rate")
	}
	
	fn set_min_corner_distance_rate(&mut self, val: f64) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropMinCornerDistanceRate_double(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_min_corner_distance_rate")
	}
	
	fn min_distance_to_border(&self) -> i32 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropMinDistanceToBorder_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: min_distance_to_border")
	}
	
	fn set_min_distance_to_border(&mut self, val: i32) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropMinDistanceToBorder_int(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_min_distance_to_border")
	}
	
	fn min_marker_distance_rate(&self) -> f64 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropMinMarkerDistanceRate_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: min_marker_distance_rate")
	}
	
	fn set_min_marker_distance_rate(&mut self, val: f64) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropMinMarkerDistanceRate_double(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_min_marker_distance_rate")
	}
	
	fn corner_refinement_method(&self) -> i32 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropCornerRefinementMethod_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: corner_refinement_method")
	}
	
	fn set_corner_refinement_method(&mut self, val: i32) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropCornerRefinementMethod_int(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_corner_refinement_method")
	}
	
	fn corner_refinement_win_size(&self) -> i32 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropCornerRefinementWinSize_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: corner_refinement_win_size")
	}
	
	fn set_corner_refinement_win_size(&mut self, val: i32) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropCornerRefinementWinSize_int(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_corner_refinement_win_size")
	}
	
	fn corner_refinement_max_iterations(&self) -> i32 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropCornerRefinementMaxIterations_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: corner_refinement_max_iterations")
	}
	
	fn set_corner_refinement_max_iterations(&mut self, val: i32) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropCornerRefinementMaxIterations_int(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_corner_refinement_max_iterations")
	}
	
	fn corner_refinement_min_accuracy(&self) -> f64 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropCornerRefinementMinAccuracy_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: corner_refinement_min_accuracy")
	}
	
	fn set_corner_refinement_min_accuracy(&mut self, val: f64) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropCornerRefinementMinAccuracy_double(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_corner_refinement_min_accuracy")
	}
	
	fn marker_border_bits(&self) -> i32 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropMarkerBorderBits_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: marker_border_bits")
	}
	
	fn set_marker_border_bits(&mut self, val: i32) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropMarkerBorderBits_int(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_marker_border_bits")
	}
	
	fn perspective_remove_pixel_per_cell(&self) -> i32 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropPerspectiveRemovePixelPerCell_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: perspective_remove_pixel_per_cell")
	}
	
	fn set_perspective_remove_pixel_per_cell(&mut self, val: i32) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropPerspectiveRemovePixelPerCell_int(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_perspective_remove_pixel_per_cell")
	}
	
	fn perspective_remove_ignored_margin_per_cell(&self) -> f64 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropPerspectiveRemoveIgnoredMarginPerCell_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: perspective_remove_ignored_margin_per_cell")
	}
	
	fn set_perspective_remove_ignored_margin_per_cell(&mut self, val: f64) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropPerspectiveRemoveIgnoredMarginPerCell_double(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_perspective_remove_ignored_margin_per_cell")
	}
	
	fn max_erroneous_bits_in_border_rate(&self) -> f64 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropMaxErroneousBitsInBorderRate_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: max_erroneous_bits_in_border_rate")
	}
	
	fn set_max_erroneous_bits_in_border_rate(&mut self, val: f64) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropMaxErroneousBitsInBorderRate_double(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_max_erroneous_bits_in_border_rate")
	}
	
	fn min_otsu_std_dev(&self) -> f64 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropMinOtsuStdDev_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: min_otsu_std_dev")
	}
	
	fn set_min_otsu_std_dev(&mut self, val: f64) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropMinOtsuStdDev_double(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_min_otsu_std_dev")
	}
	
	fn error_correction_rate(&self) -> f64 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropErrorCorrectionRate_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: error_correction_rate")
	}
	
	fn set_error_correction_rate(&mut self, val: f64) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropErrorCorrectionRate_double(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_error_correction_rate")
	}
	
	fn april_tag_quad_decimate(&self) -> f32 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropAprilTagQuadDecimate_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: april_tag_quad_decimate")
	}
	
	fn set_april_tag_quad_decimate(&mut self, val: f32) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropAprilTagQuadDecimate_float(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_april_tag_quad_decimate")
	}
	
	fn april_tag_quad_sigma(&self) -> f32 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropAprilTagQuadSigma_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: april_tag_quad_sigma")
	}
	
	fn set_april_tag_quad_sigma(&mut self, val: f32) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropAprilTagQuadSigma_float(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_april_tag_quad_sigma")
	}
	
	fn april_tag_min_cluster_pixels(&self) -> i32 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropAprilTagMinClusterPixels_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: april_tag_min_cluster_pixels")
	}
	
	fn set_april_tag_min_cluster_pixels(&mut self, val: i32) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropAprilTagMinClusterPixels_int(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_april_tag_min_cluster_pixels")
	}
	
	fn april_tag_max_nmaxima(&self) -> i32 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropAprilTagMaxNmaxima_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: april_tag_max_nmaxima")
	}
	
	fn set_april_tag_max_nmaxima(&mut self, val: i32) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropAprilTagMaxNmaxima_int(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_april_tag_max_nmaxima")
	}
	
	fn april_tag_critical_rad(&self) -> f32 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropAprilTagCriticalRad_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: april_tag_critical_rad")
	}
	
	fn set_april_tag_critical_rad(&mut self, val: f32) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropAprilTagCriticalRad_float(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_april_tag_critical_rad")
	}
	
	fn april_tag_max_line_fit_mse(&self) -> f32 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropAprilTagMaxLineFitMse_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: april_tag_max_line_fit_mse")
	}
	
	fn set_april_tag_max_line_fit_mse(&mut self, val: f32) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropAprilTagMaxLineFitMse_float(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_april_tag_max_line_fit_mse")
	}
	
	fn april_tag_min_white_black_diff(&self) -> i32 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropAprilTagMinWhiteBlackDiff_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: april_tag_min_white_black_diff")
	}
	
	fn set_april_tag_min_white_black_diff(&mut self, val: i32) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropAprilTagMinWhiteBlackDiff_int(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_april_tag_min_white_black_diff")
	}
	
	fn april_tag_deglitch(&self) -> i32 {
		unsafe { sys::cv_aruco_DetectorParameters_getPropAprilTagDeglitch_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: april_tag_deglitch")
	}
	
	fn set_april_tag_deglitch(&mut self, val: i32) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropAprilTagDeglitch_int(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_april_tag_deglitch")
	}
	
	fn detect_inverted_marker(&self) -> bool {
		unsafe { sys::cv_aruco_DetectorParameters_getPropDetectInvertedMarker_const(self.as_raw_DetectorParameters()) }.into_result().expect("Infallible function failed: detect_inverted_marker")
	}
	
	fn set_detect_inverted_marker(&mut self, val: bool) -> () {
		unsafe { sys::cv_aruco_DetectorParameters_setPropDetectInvertedMarker_bool(self.as_raw_mut_DetectorParameters(), val) }.into_result().expect("Infallible function failed: set_detect_inverted_marker")
	}
	
}

/// Parameters for the detectMarker process:
/// - adaptiveThreshWinSizeMin: minimum window size for adaptive thresholding before finding
///   contours (default 3).
/// - adaptiveThreshWinSizeMax: maximum window size for adaptive thresholding before finding
///   contours (default 23).
/// - adaptiveThreshWinSizeStep: increments from adaptiveThreshWinSizeMin to adaptiveThreshWinSizeMax
///   during the thresholding (default 10).
/// - adaptiveThreshConstant: constant for adaptive thresholding before finding contours (default 7)
/// - minMarkerPerimeterRate: determine minimum perimeter for marker contour to be detected. This
///   is defined as a rate respect to the maximum dimension of the input image (default 0.03).
/// - maxMarkerPerimeterRate:  determine maximum perimeter for marker contour to be detected. This
///   is defined as a rate respect to the maximum dimension of the input image (default 4.0).
/// - polygonalApproxAccuracyRate: minimum accuracy during the polygonal approximation process to
///   determine which contours are squares.
/// - minCornerDistanceRate: minimum distance between corners for detected markers relative to its
///   perimeter (default 0.05)
/// - minDistanceToBorder: minimum distance of any corner to the image border for detected markers
///   (in pixels) (default 3)
/// - minMarkerDistanceRate: minimum mean distance beetween two marker corners to be considered
///   similar, so that the smaller one is removed. The rate is relative to the smaller perimeter
///   of the two markers (default 0.05).
/// - cornerRefinementMethod: corner refinement method. (CORNER_REFINE_NONE, no refinement.
///   CORNER_REFINE_SUBPIX, do subpixel refinement. CORNER_REFINE_CONTOUR use contour-Points,
///   CORNER_REFINE_APRILTAG  use the AprilTag2 approach)
/// - cornerRefinementWinSize: window size for the corner refinement process (in pixels) (default 5).
/// - cornerRefinementMaxIterations: maximum number of iterations for stop criteria of the corner
///   refinement process (default 30).
/// - cornerRefinementMinAccuracy: minimum error for the stop cristeria of the corner refinement
///   process (default: 0.1)
/// - markerBorderBits: number of bits of the marker border, i.e. marker border width (default 1).
/// - perspectiveRemovePixelPerCell: number of bits (per dimension) for each cell of the marker
///   when removing the perspective (default 8).
/// - perspectiveRemoveIgnoredMarginPerCell: width of the margin of pixels on each cell not
///   considered for the determination of the cell bit. Represents the rate respect to the total
///   size of the cell, i.e. perspectiveRemovePixelPerCell (default 0.13)
/// - maxErroneousBitsInBorderRate: maximum number of accepted erroneous bits in the border (i.e.
///   number of allowed white bits in the border). Represented as a rate respect to the total
///   number of bits per marker (default 0.35).
/// - minOtsuStdDev: minimun standard deviation in pixels values during the decodification step to
///   apply Otsu thresholding (otherwise, all the bits are set to 0 or 1 depending on mean higher
///   than 128 or not) (default 5.0)
/// - errorCorrectionRate error correction rate respect to the maximun error correction capability
///   for each dictionary. (default 0.6).
/// - aprilTagMinClusterPixels: reject quads containing too few pixels.
/// - aprilTagMaxNmaxima: how many corner candidates to consider when segmenting a group of pixels into a quad.
/// - aprilTagCriticalRad: Reject quads where pairs of edges have angles that are close to straight or close to
///   180 degrees. Zero means that no quads are rejected. (In radians).
/// - aprilTagMaxLineFitMse:  When fitting lines to the contours, what is the maximum mean squared error
///   allowed?  This is useful in rejecting contours that are far from being quad shaped; rejecting
///   these quads "early" saves expensive decoding processing.
/// - aprilTagMinWhiteBlackDiff: When we build our model of black & white pixels, we add an extra check that
///   the white model must be (overall) brighter than the black model.  How much brighter? (in pixel values, [0,255]).
/// - aprilTagDeglitch:  should the thresholded image be deglitched? Only useful for very noisy images
/// - aprilTagQuadDecimate: Detection of quads can be done on a lower-resolution image, improving speed at a
///   cost of pose accuracy and a slight decrease in detection rate. Decoding the binary payload is still
///   done at full resolution.
/// - aprilTagQuadSigma: What Gaussian blur should be applied to the segmented image (used for quad detection?)
///   Parameter is the standard deviation in pixels.  Very noisy images benefit from non-zero values (e.g. 0.8).
/// - detectInvertedMarker: to check if there is a white marker. In order to generate a "white" marker just
///   invert a normal marker by using a tilde, ~markerImage. (default false)
pub struct DetectorParameters {
	ptr: *mut c_void
}

opencv_type_boxed! { DetectorParameters }

impl Drop for DetectorParameters {
	fn drop(&mut self) {
		extern "C" { fn cv_DetectorParameters_delete(instance: *mut c_void); }
		unsafe { cv_DetectorParameters_delete(self.as_raw_mut_DetectorParameters()) };
	}
}

impl DetectorParameters {
	#[inline] pub fn as_raw_DetectorParameters(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_DetectorParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for DetectorParameters {}

impl crate::aruco::DetectorParametersTrait for DetectorParameters {
	#[inline] fn as_raw_DetectorParameters(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_DetectorParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl DetectorParameters {
	pub fn default() -> Result<crate::aruco::DetectorParameters> {
		unsafe { sys::cv_aruco_DetectorParameters_DetectorParameters() }.into_result().map(|r| unsafe { crate::aruco::DetectorParameters::opencv_from_extern(r) } )
	}
	
	pub fn create() -> Result<core::Ptr::<crate::aruco::DetectorParameters>> {
		unsafe { sys::cv_aruco_DetectorParameters_create() }.into_result().map(|r| unsafe { core::Ptr::<crate::aruco::DetectorParameters>::opencv_from_extern(r) } )
	}
	
}

/// Dictionary/Set of markers. It contains the inner codification
/// 
/// bytesList contains the marker codewords where
/// - bytesList.rows is the dictionary size
/// - each marker is encoded using `nbytes = ceil(markerSize*markerSize/8.)`
/// - each row contains all 4 rotations of the marker, so its length is `4*nbytes`
/// 
/// `bytesList.ptr(i)[k*nbytes + j]` is then the j-th byte of i-th marker, in its k-th rotation.
pub trait DictionaryTrait {
	fn as_raw_Dictionary(&self) -> *const c_void;
	fn as_raw_mut_Dictionary(&mut self) -> *mut c_void;

	fn bytes_list(&mut self) -> core::Mat {
		unsafe { sys::cv_aruco_Dictionary_getPropBytesList(self.as_raw_mut_Dictionary()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: bytes_list")
	}
	
	fn set_bytes_list(&mut self, mut val: core::Mat) -> () {
		unsafe { sys::cv_aruco_Dictionary_setPropBytesList_Mat(self.as_raw_mut_Dictionary(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_bytes_list")
	}
	
	fn marker_size(&self) -> i32 {
		unsafe { sys::cv_aruco_Dictionary_getPropMarkerSize_const(self.as_raw_Dictionary()) }.into_result().expect("Infallible function failed: marker_size")
	}
	
	fn set_marker_size(&mut self, val: i32) -> () {
		unsafe { sys::cv_aruco_Dictionary_setPropMarkerSize_int(self.as_raw_mut_Dictionary(), val) }.into_result().expect("Infallible function failed: set_marker_size")
	}
	
	fn max_correction_bits(&self) -> i32 {
		unsafe { sys::cv_aruco_Dictionary_getPropMaxCorrectionBits_const(self.as_raw_Dictionary()) }.into_result().expect("Infallible function failed: max_correction_bits")
	}
	
	fn set_max_correction_bits(&mut self, val: i32) -> () {
		unsafe { sys::cv_aruco_Dictionary_setPropMaxCorrectionBits_int(self.as_raw_mut_Dictionary(), val) }.into_result().expect("Infallible function failed: set_max_correction_bits")
	}
	
	/// Given a matrix of bits. Returns whether if marker is identified or not.
	/// It returns by reference the correct id (if any) and the correct rotation
	fn identify(&self, only_bits: &core::Mat, idx: &mut i32, rotation: &mut i32, max_correction_rate: f64) -> Result<bool> {
		unsafe { sys::cv_aruco_Dictionary_identify_const_const_MatR_intR_intR_double(self.as_raw_Dictionary(), only_bits.as_raw_Mat(), idx, rotation, max_correction_rate) }.into_result()
	}
	
	/// Returns the distance of the input bits to the specific id. If allRotations is true,
	/// the four posible bits rotation are considered
	/// 
	/// ## C++ default parameters
	/// * all_rotations: true
	fn get_distance_to_id(&self, bits: &dyn core::ToInputArray, id: i32, all_rotations: bool) -> Result<i32> {
		input_array_arg!(bits);
		unsafe { sys::cv_aruco_Dictionary_getDistanceToId_const_const__InputArrayR_int_bool(self.as_raw_Dictionary(), bits.as_raw__InputArray(), id, all_rotations) }.into_result()
	}
	
	/// Draw a canonical marker image
	/// 
	/// ## C++ default parameters
	/// * border_bits: 1
	fn draw_marker(&self, id: i32, side_pixels: i32, _img: &mut dyn core::ToOutputArray, border_bits: i32) -> Result<()> {
		output_array_arg!(_img);
		unsafe { sys::cv_aruco_Dictionary_drawMarker_const_int_int_const__OutputArrayR_int(self.as_raw_Dictionary(), id, side_pixels, _img.as_raw__OutputArray(), border_bits) }.into_result()
	}
	
}

/// Dictionary/Set of markers. It contains the inner codification
/// 
/// bytesList contains the marker codewords where
/// - bytesList.rows is the dictionary size
/// - each marker is encoded using `nbytes = ceil(markerSize*markerSize/8.)`
/// - each row contains all 4 rotations of the marker, so its length is `4*nbytes`
/// 
/// `bytesList.ptr(i)[k*nbytes + j]` is then the j-th byte of i-th marker, in its k-th rotation.
pub struct Dictionary {
	ptr: *mut c_void
}

opencv_type_boxed! { Dictionary }

impl Drop for Dictionary {
	fn drop(&mut self) {
		extern "C" { fn cv_Dictionary_delete(instance: *mut c_void); }
		unsafe { cv_Dictionary_delete(self.as_raw_mut_Dictionary()) };
	}
}

impl Dictionary {
	#[inline] pub fn as_raw_Dictionary(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Dictionary(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Dictionary {}

impl crate::aruco::DictionaryTrait for Dictionary {
	#[inline] fn as_raw_Dictionary(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Dictionary(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Dictionary {
	/// ## C++ default parameters
	/// * _bytes_list: Mat()
	/// * _marker_size: 0
	/// * _maxcorr: 0
	pub fn new(_bytes_list: &core::Mat, _marker_size: i32, _maxcorr: i32) -> Result<crate::aruco::Dictionary> {
		unsafe { sys::cv_aruco_Dictionary_Dictionary_const_MatR_int_int(_bytes_list.as_raw_Mat(), _marker_size, _maxcorr) }.into_result().map(|r| unsafe { crate::aruco::Dictionary::opencv_from_extern(r) } )
	}
	
	pub fn copy(_dictionary: &core::Ptr::<crate::aruco::Dictionary>) -> Result<crate::aruco::Dictionary> {
		unsafe { sys::cv_aruco_Dictionary_Dictionary_const_Ptr_Dictionary_R(_dictionary.as_raw_PtrOfDictionary()) }.into_result().map(|r| unsafe { crate::aruco::Dictionary::opencv_from_extern(r) } )
	}
	
	/// ## See also
	/// generateCustomDictionary
	/// 
	/// ## C++ default parameters
	/// * random_seed: 0
	pub fn create(n_markers: i32, marker_size: i32, random_seed: i32) -> Result<core::Ptr::<crate::aruco::Dictionary>> {
		unsafe { sys::cv_aruco_Dictionary_create_int_int_int(n_markers, marker_size, random_seed) }.into_result().map(|r| unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(r) } )
	}
	
	/// ## See also
	/// generateCustomDictionary
	/// 
	/// ## C++ default parameters
	/// * random_seed: 0
	pub fn create_from(n_markers: i32, marker_size: i32, base_dictionary: &core::Ptr::<crate::aruco::Dictionary>, random_seed: i32) -> Result<core::Ptr::<crate::aruco::Dictionary>> {
		unsafe { sys::cv_aruco_Dictionary_create_int_int_const_Ptr_Dictionary_R_int(n_markers, marker_size, base_dictionary.as_raw_PtrOfDictionary(), random_seed) }.into_result().map(|r| unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(r) } )
	}
	
	/// ## See also
	/// getPredefinedDictionary
	pub fn get(dict: i32) -> Result<core::Ptr::<crate::aruco::Dictionary>> {
		unsafe { sys::cv_aruco_Dictionary_get_int(dict) }.into_result().map(|r| unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(r) } )
	}
	
	/// Transform matrix of bits to list of bytes in the 4 rotations
	pub fn get_byte_list_from_bits(bits: &core::Mat) -> Result<core::Mat> {
		unsafe { sys::cv_aruco_Dictionary_getByteListFromBits_const_MatR(bits.as_raw_Mat()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Transform list of bytes to matrix of bits
	pub fn get_bits_from_byte_list(byte_list: &core::Mat, marker_size: i32) -> Result<core::Mat> {
		unsafe { sys::cv_aruco_Dictionary_getBitsFromByteList_const_MatR_int(byte_list.as_raw_Mat(), marker_size) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

/// Planar board with grid arrangement of markers
/// More common type of board. All markers are placed in the same plane in a grid arrangement.
/// The board can be drawn using drawPlanarBoard() function (see also: drawPlanarBoard)
pub trait GridBoardTrait: crate::aruco::BoardTrait {
	fn as_raw_GridBoard(&self) -> *const c_void;
	fn as_raw_mut_GridBoard(&mut self) -> *mut c_void;

	/// Draw a GridBoard
	/// 
	/// ## Parameters
	/// * outSize: size of the output image in pixels.
	/// * img: output image with the board. The size of this image will be outSize
	/// and the board will be on the center, keeping the board proportions.
	/// * marginSize: minimum margins (in pixels) of the board in the output image
	/// * borderBits: width of the marker borders.
	/// 
	/// This function return the image of the GridBoard, ready to be printed.
	/// 
	/// ## C++ default parameters
	/// * margin_size: 0
	/// * border_bits: 1
	fn draw(&mut self, out_size: core::Size, img: &mut dyn core::ToOutputArray, margin_size: i32, border_bits: i32) -> Result<()> {
		output_array_arg!(img);
		unsafe { sys::cv_aruco_GridBoard_draw_Size_const__OutputArrayR_int_int(self.as_raw_mut_GridBoard(), out_size.opencv_to_extern(), img.as_raw__OutputArray(), margin_size, border_bits) }.into_result()
	}
	
	fn get_grid_size(&self) -> Result<core::Size> {
		unsafe { sys::cv_aruco_GridBoard_getGridSize_const(self.as_raw_GridBoard()) }.into_result()
	}
	
	fn get_marker_length(&self) -> Result<f32> {
		unsafe { sys::cv_aruco_GridBoard_getMarkerLength_const(self.as_raw_GridBoard()) }.into_result()
	}
	
	fn get_marker_separation(&self) -> Result<f32> {
		unsafe { sys::cv_aruco_GridBoard_getMarkerSeparation_const(self.as_raw_GridBoard()) }.into_result()
	}
	
}

/// Planar board with grid arrangement of markers
/// More common type of board. All markers are placed in the same plane in a grid arrangement.
/// The board can be drawn using drawPlanarBoard() function (see also: drawPlanarBoard)
pub struct GridBoard {
	ptr: *mut c_void
}

opencv_type_boxed! { GridBoard }

impl Drop for GridBoard {
	fn drop(&mut self) {
		extern "C" { fn cv_GridBoard_delete(instance: *mut c_void); }
		unsafe { cv_GridBoard_delete(self.as_raw_mut_GridBoard()) };
	}
}

impl GridBoard {
	#[inline] pub fn as_raw_GridBoard(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_GridBoard(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for GridBoard {}

impl crate::aruco::BoardTrait for GridBoard {
	#[inline] fn as_raw_Board(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::aruco::GridBoardTrait for GridBoard {
	#[inline] fn as_raw_GridBoard(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_GridBoard(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GridBoard {
	/// Create a GridBoard object
	/// 
	/// ## Parameters
	/// * markersX: number of markers in X direction
	/// * markersY: number of markers in Y direction
	/// * markerLength: marker side length (normally in meters)
	/// * markerSeparation: separation between two markers (same unit as markerLength)
	/// * dictionary: dictionary of markers indicating the type of markers
	/// * firstMarker: id of first marker in dictionary to use on board.
	/// ## Returns
	/// the output GridBoard object
	/// 
	/// This functions creates a GridBoard object given the number of markers in each direction and
	/// the marker size and marker separation.
	/// 
	/// ## C++ default parameters
	/// * first_marker: 0
	pub fn create(markers_x: i32, markers_y: i32, marker_length: f32, marker_separation: f32, dictionary: &core::Ptr::<crate::aruco::Dictionary>, first_marker: i32) -> Result<core::Ptr::<crate::aruco::GridBoard>> {
		unsafe { sys::cv_aruco_GridBoard_create_int_int_float_float_const_Ptr_Dictionary_R_int(markers_x, markers_y, marker_length, marker_separation, dictionary.as_raw_PtrOfDictionary(), first_marker) }.into_result().map(|r| unsafe { core::Ptr::<crate::aruco::GridBoard>::opencv_from_extern(r) } )
	}
	
}
