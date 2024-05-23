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
//! The implementation is based on the ArUco Library by R. Muñoz-Salinas and S. Garrido-Jurado [Aruco2014](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Aruco2014).
//!
//! Markers can also be detected based on the AprilTag 2 [wang2016iros](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_wang2016iros) fiducial detection method.
//! ## See also
//! S. Garrido-Jurado, R. Muñoz-Salinas, F. J. Madrid-Cuevas, and M. J. Marín-Jiménez. 2014.
//! "Automatic generation and detection of highly reliable fiducial markers under occlusion".
//! Pattern Recogn. 47, 6 (June 2014), 2280-2292. DOI=10.1016/j.patcog.2014.01.005
//!
//! <http://www.uco.es/investiga/grupos/ava/node/26>
//!
//! This module has been originally developed by Sergio Garrido-Jurado as a project
//! for Google Summer of Code 2015 (GSoC 15).
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{BoardTrait, BoardTraitConst, CharucoBoardTrait, CharucoBoardTraitConst, DetectorParametersTrait, DetectorParametersTraitConst, DictionaryTrait, DictionaryTraitConst, EstimateParametersTrait, EstimateParametersTraitConst, GridBoardTrait, GridBoardTraitConst};
}

/// The marker coordinate system is centered on the middle of the marker.
/// The coordinates of the four corners (CCW order) of the marker in its own coordinate system are:
/// (-markerLength/2, markerLength/2, 0), (markerLength/2, markerLength/2, 0),
/// (markerLength/2, -markerLength/2, 0), (-markerLength/2, -markerLength/2, 0).
///
/// These pattern points define this coordinate system:
/// ![Image with axes drawn](https://docs.opencv.org/3.4.20/singlemarkersaxes.jpg)
// CCW_center /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:239
pub const CCW_center: i32 = 0;
/// Tag and corners detection based on the AprilTag 2 approach [wang2016iros](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_wang2016iros)
// CORNER_REFINE_APRILTAG /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:86
pub const CORNER_REFINE_APRILTAG: i32 = 3;
/// ArUco approach and refine the corners locations using the contour-points line fitting
// CORNER_REFINE_CONTOUR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:85
pub const CORNER_REFINE_CONTOUR: i32 = 2;
/// Tag and corners detection based on the ArUco approach
// CORNER_REFINE_NONE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:83
pub const CORNER_REFINE_NONE: i32 = 0;
/// ArUco approach and refine the corners locations using corner subpixel accuracy
// CORNER_REFINE_SUBPIX /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:84
pub const CORNER_REFINE_SUBPIX: i32 = 1;
/// The marker coordinate system is centered on the top-left corner of the marker.
/// The coordinates of the four corners (CW order) of the marker in its own coordinate system are:
/// (0, 0, 0), (markerLength, 0, 0),
/// (markerLength, markerLength, 0), (0, markerLength, 0).
///
/// These pattern points define this coordinate system:
/// ![Image with axes drawn](https://docs.opencv.org/3.4.20/singlemarkersaxes2.jpg)
// CW_top_left_corner /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:248
pub const CW_top_left_corner: i32 = 1;
/// 4x4 bits, minimum hamming distance between any two codes = 3, 100 codes
// DICT_4X4_100 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:158
pub const DICT_4X4_100: i32 = 1;
/// 4x4 bits, minimum hamming distance between any two codes = 2, 1000 codes
// DICT_4X4_1000 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:160
pub const DICT_4X4_1000: i32 = 3;
/// 4x4 bits, minimum hamming distance between any two codes = 3, 250 codes
// DICT_4X4_250 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:159
pub const DICT_4X4_250: i32 = 2;
/// 4x4 bits, minimum hamming distance between any two codes = 4, 50 codes
// DICT_4X4_50 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:157
pub const DICT_4X4_50: i32 = 0;
/// 5x5 bits, minimum hamming distance between any two codes = 7, 100 codes
// DICT_5X5_100 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:162
pub const DICT_5X5_100: i32 = 5;
/// 5x5 bits, minimum hamming distance between any two codes = 5, 1000 codes
// DICT_5X5_1000 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:164
pub const DICT_5X5_1000: i32 = 7;
/// 5x5 bits, minimum hamming distance between any two codes = 6, 250 codes
// DICT_5X5_250 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:163
pub const DICT_5X5_250: i32 = 6;
/// 5x5 bits, minimum hamming distance between any two codes = 8, 50 codes
// DICT_5X5_50 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:161
pub const DICT_5X5_50: i32 = 4;
/// 6x6 bits, minimum hamming distance between any two codes = 12, 100 codes
// DICT_6X6_100 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:166
pub const DICT_6X6_100: i32 = 9;
/// 6x6 bits, minimum hamming distance between any two codes = 9, 1000 codes
// DICT_6X6_1000 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:168
pub const DICT_6X6_1000: i32 = 11;
/// 6x6 bits, minimum hamming distance between any two codes = 11, 250 codes
// DICT_6X6_250 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:167
pub const DICT_6X6_250: i32 = 10;
/// 6x6 bits, minimum hamming distance between any two codes = 13, 50 codes
// DICT_6X6_50 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:165
pub const DICT_6X6_50: i32 = 8;
/// 7x7 bits, minimum hamming distance between any two codes = 18, 100 codes
// DICT_7X7_100 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:170
pub const DICT_7X7_100: i32 = 13;
/// 7x7 bits, minimum hamming distance between any two codes = 14, 1000 codes
// DICT_7X7_1000 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:172
pub const DICT_7X7_1000: i32 = 15;
/// 7x7 bits, minimum hamming distance between any two codes = 17, 250 codes
// DICT_7X7_250 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:171
pub const DICT_7X7_250: i32 = 14;
/// 7x7 bits, minimum hamming distance between any two codes = 19, 50 codes
// DICT_7X7_50 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:169
pub const DICT_7X7_50: i32 = 12;
/// 4x4 bits, minimum hamming distance between any two codes = 5, 30 codes
// DICT_APRILTAG_16h5 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:174
pub const DICT_APRILTAG_16h5: i32 = 17;
/// 5x5 bits, minimum hamming distance between any two codes = 9, 35 codes
// DICT_APRILTAG_25h9 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:175
pub const DICT_APRILTAG_25h9: i32 = 18;
/// 6x6 bits, minimum hamming distance between any two codes = 10, 2320 codes
// DICT_APRILTAG_36h10 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:176
pub const DICT_APRILTAG_36h10: i32 = 19;
/// 6x6 bits, minimum hamming distance between any two codes = 11, 587 codes
// DICT_APRILTAG_36h11 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:177
pub const DICT_APRILTAG_36h11: i32 = 20;
/// 6x6 bits, minimum hamming distance between any two codes = 3, 1024 codes
// DICT_ARUCO_ORIGINAL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:173
pub const DICT_ARUCO_ORIGINAL: i32 = 16;
// CornerRefineMethod /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:82
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CornerRefineMethod {
	/// Tag and corners detection based on the ArUco approach
	CORNER_REFINE_NONE = 0,
	/// ArUco approach and refine the corners locations using corner subpixel accuracy
	CORNER_REFINE_SUBPIX = 1,
	/// ArUco approach and refine the corners locations using the contour-points line fitting
	CORNER_REFINE_CONTOUR = 2,
	/// Tag and corners detection based on the AprilTag 2 approach [wang2016iros](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_wang2016iros)
	CORNER_REFINE_APRILTAG = 3,
}

impl TryFrom<i32> for CornerRefineMethod {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::CORNER_REFINE_NONE),
			1 => Ok(Self::CORNER_REFINE_SUBPIX),
			2 => Ok(Self::CORNER_REFINE_CONTOUR),
			3 => Ok(Self::CORNER_REFINE_APRILTAG),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::aruco::CornerRefineMethod"))),
		}
	}
}

opencv_type_enum! { crate::aruco::CornerRefineMethod }

/// Predefined markers dictionaries/sets
/// Each dictionary indicates the number of bits and the number of markers contained
/// - DICT_ARUCO_ORIGINAL: standard ArUco Library Markers. 1024 markers, 5x5 bits, 0 minimum
///                        distance
// PREDEFINED_DICTIONARY_NAME /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:156
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PREDEFINED_DICTIONARY_NAME {
	/// 4x4 bits, minimum hamming distance between any two codes = 4, 50 codes
	DICT_4X4_50 = 0,
	/// 4x4 bits, minimum hamming distance between any two codes = 3, 100 codes
	DICT_4X4_100 = 1,
	/// 4x4 bits, minimum hamming distance between any two codes = 3, 250 codes
	DICT_4X4_250 = 2,
	/// 4x4 bits, minimum hamming distance between any two codes = 2, 1000 codes
	DICT_4X4_1000 = 3,
	/// 5x5 bits, minimum hamming distance between any two codes = 8, 50 codes
	DICT_5X5_50 = 4,
	/// 5x5 bits, minimum hamming distance between any two codes = 7, 100 codes
	DICT_5X5_100 = 5,
	/// 5x5 bits, minimum hamming distance between any two codes = 6, 250 codes
	DICT_5X5_250 = 6,
	/// 5x5 bits, minimum hamming distance between any two codes = 5, 1000 codes
	DICT_5X5_1000 = 7,
	/// 6x6 bits, minimum hamming distance between any two codes = 13, 50 codes
	DICT_6X6_50 = 8,
	/// 6x6 bits, minimum hamming distance between any two codes = 12, 100 codes
	DICT_6X6_100 = 9,
	/// 6x6 bits, minimum hamming distance between any two codes = 11, 250 codes
	DICT_6X6_250 = 10,
	/// 6x6 bits, minimum hamming distance between any two codes = 9, 1000 codes
	DICT_6X6_1000 = 11,
	/// 7x7 bits, minimum hamming distance between any two codes = 19, 50 codes
	DICT_7X7_50 = 12,
	/// 7x7 bits, minimum hamming distance between any two codes = 18, 100 codes
	DICT_7X7_100 = 13,
	/// 7x7 bits, minimum hamming distance between any two codes = 17, 250 codes
	DICT_7X7_250 = 14,
	/// 7x7 bits, minimum hamming distance between any two codes = 14, 1000 codes
	DICT_7X7_1000 = 15,
	/// 6x6 bits, minimum hamming distance between any two codes = 3, 1024 codes
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

impl TryFrom<i32> for PREDEFINED_DICTIONARY_NAME {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::DICT_4X4_50),
			1 => Ok(Self::DICT_4X4_100),
			2 => Ok(Self::DICT_4X4_250),
			3 => Ok(Self::DICT_4X4_1000),
			4 => Ok(Self::DICT_5X5_50),
			5 => Ok(Self::DICT_5X5_100),
			6 => Ok(Self::DICT_5X5_250),
			7 => Ok(Self::DICT_5X5_1000),
			8 => Ok(Self::DICT_6X6_50),
			9 => Ok(Self::DICT_6X6_100),
			10 => Ok(Self::DICT_6X6_250),
			11 => Ok(Self::DICT_6X6_1000),
			12 => Ok(Self::DICT_7X7_50),
			13 => Ok(Self::DICT_7X7_100),
			14 => Ok(Self::DICT_7X7_250),
			15 => Ok(Self::DICT_7X7_1000),
			16 => Ok(Self::DICT_ARUCO_ORIGINAL),
			17 => Ok(Self::DICT_APRILTAG_16h5),
			18 => Ok(Self::DICT_APRILTAG_25h9),
			19 => Ok(Self::DICT_APRILTAG_36h10),
			20 => Ok(Self::DICT_APRILTAG_36h11),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::aruco::PREDEFINED_DICTIONARY_NAME"))),
		}
	}
}

opencv_type_enum! { crate::aruco::PREDEFINED_DICTIONARY_NAME }

///
/// rvec/tvec define the right handed coordinate system of the marker.
/// PatternPos defines center this system and axes direction.
/// Axis X (red color) - first coordinate, axis Y (green color) - second coordinate,
/// axis Z (blue color) - third coordinate.
/// ## See also
/// estimatePoseSingleMarkers(), [tutorial_aruco_detection]
// PatternPos /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:230
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PatternPos {
	/// The marker coordinate system is centered on the middle of the marker.
	/// The coordinates of the four corners (CCW order) of the marker in its own coordinate system are:
	/// (-markerLength/2, markerLength/2, 0), (markerLength/2, markerLength/2, 0),
	/// (markerLength/2, -markerLength/2, 0), (-markerLength/2, -markerLength/2, 0).
	///
	/// These pattern points define this coordinate system:
	/// ![Image with axes drawn](https://docs.opencv.org/3.4.20/singlemarkersaxes.jpg)
	CCW_center = 0,
	/// The marker coordinate system is centered on the top-left corner of the marker.
	/// The coordinates of the four corners (CW order) of the marker in its own coordinate system are:
	/// (0, 0, 0), (markerLength, 0, 0),
	/// (markerLength, markerLength, 0), (0, markerLength, 0).
	///
	/// These pattern points define this coordinate system:
	/// ![Image with axes drawn](https://docs.opencv.org/3.4.20/singlemarkersaxes2.jpg)
	CW_top_left_corner = 1,
}

impl TryFrom<i32> for PatternPos {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::CCW_center),
			1 => Ok(Self::CW_top_left_corner),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::aruco::PatternPos"))),
		}
	}
}

opencv_type_enum! { crate::aruco::PatternPos }

/// It's the same function as [calibrate_camera_aruco] but without calibration error estimation.
///
/// ## Note
/// This alternative version of [calibrate_camera_aruco] function uses the following default values for its arguments:
/// * rvecs: noArray()
/// * tvecs: noArray()
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
// cv::aruco::calibrateCameraAruco(InputArray, InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:635
// ("cv::aruco::calibrateCameraAruco", vec![(pred!(mut, ["corners", "ids", "counter", "board", "imageSize", "cameraMatrix", "distCoeffs"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
#[inline]
pub fn calibrate_camera_aruco_def(corners: &impl ToInputArray, ids: &impl ToInputArray, counter: &impl ToInputArray, board: &core::Ptr<crate::aruco::Board>, image_size: core::Size, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &mut impl ToInputOutputArray) -> Result<f64> {
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
/// ## Note
/// This alternative version of [calibrate_camera_aruco_extended] function uses the following default values for its arguments:
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
// cv::aruco::calibrateCameraAruco(InputArray, InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:624
// ("cv::aruco::calibrateCameraAruco", vec![(pred!(mut, ["corners", "ids", "counter", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "perViewErrors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn calibrate_camera_aruco_extended_def(corners: &impl ToInputArray, ids: &impl ToInputArray, counter: &impl ToInputArray, board: &core::Ptr<crate::aruco::Board>, image_size: core::Size, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &mut impl ToInputOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, std_deviations_intrinsics: &mut impl ToOutputArray, std_deviations_extrinsics: &mut impl ToOutputArray, per_view_errors: &mut impl ToOutputArray) -> Result<f64> {
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
/// ## C++ default parameters
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
// calibrateCameraAruco(InputArrayOfArrays, InputArray, InputArray, const Ptr<Board> &, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, OutputArray, OutputArray, OutputArray, int, TermCriteria)(InputArray, InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:624
// ("cv::aruco::calibrateCameraAruco", vec![(pred!(mut, ["corners", "ids", "counter", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "perViewErrors", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
#[inline]
pub fn calibrate_camera_aruco_extended(corners: &impl ToInputArray, ids: &impl ToInputArray, counter: &impl ToInputArray, board: &core::Ptr<crate::aruco::Board>, image_size: core::Size, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &mut impl ToInputOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, std_deviations_intrinsics: &mut impl ToOutputArray, std_deviations_extrinsics: &mut impl ToOutputArray, per_view_errors: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
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
	unsafe { sys::cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(corners.as_raw__InputArray(), ids.as_raw__InputArray(), counter.as_raw__InputArray(), board.as_raw_PtrOfBoard(), &image_size, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), std_deviations_intrinsics.as_raw__OutputArray(), std_deviations_extrinsics.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// It's the same function as [calibrate_camera_aruco] but without calibration error estimation.
///
/// ## C++ default parameters
/// * rvecs: noArray()
/// * tvecs: noArray()
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
// calibrateCameraAruco(InputArrayOfArrays, InputArray, InputArray, const Ptr<Board> &, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, int, TermCriteria)(InputArray, InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:635
// ("cv::aruco::calibrateCameraAruco", vec![(pred!(mut, ["corners", "ids", "counter", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
#[inline]
pub fn calibrate_camera_aruco(corners: &impl ToInputArray, ids: &impl ToInputArray, counter: &impl ToInputArray, board: &core::Ptr<crate::aruco::Board>, image_size: core::Size, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &mut impl ToInputOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
	input_array_arg!(corners);
	input_array_arg!(ids);
	input_array_arg!(counter);
	input_output_array_arg!(camera_matrix);
	input_output_array_arg!(dist_coeffs);
	output_array_arg!(rvecs);
	output_array_arg!(tvecs);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_calibrateCameraAruco_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(corners.as_raw__InputArray(), ids.as_raw__InputArray(), counter.as_raw__InputArray(), board.as_raw_PtrOfBoard(), &image_size, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
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
// cv::aruco::calibrateCameraCharuco(InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:255
// ("cv::aruco::calibrateCameraCharuco", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "imageSize", "cameraMatrix", "distCoeffs"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
#[inline]
pub fn calibrate_camera_charuco_def(charuco_corners: &impl ToInputArray, charuco_ids: &impl ToInputArray, board: &core::Ptr<crate::aruco::CharucoBoard>, image_size: core::Size, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &mut impl ToInputOutputArray) -> Result<f64> {
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
/// ## Note
/// This alternative version of [calibrate_camera_charuco_extended] function uses the following default values for its arguments:
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
// cv::aruco::calibrateCameraCharuco(InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:245
// ("cv::aruco::calibrateCameraCharuco", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "perViewErrors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn calibrate_camera_charuco_extended_def(charuco_corners: &impl ToInputArray, charuco_ids: &impl ToInputArray, board: &core::Ptr<crate::aruco::CharucoBoard>, image_size: core::Size, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &mut impl ToInputOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, std_deviations_intrinsics: &mut impl ToOutputArray, std_deviations_extrinsics: &mut impl ToOutputArray, per_view_errors: &mut impl ToOutputArray) -> Result<f64> {
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
/// ## C++ default parameters
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,30,DBL_EPSILON)
// calibrateCameraCharuco(InputArrayOfArrays, InputArrayOfArrays, const Ptr<CharucoBoard> &, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, OutputArray, OutputArray, OutputArray, int, TermCriteria)(InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, OutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:245
// ("cv::aruco::calibrateCameraCharuco", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "stdDeviationsIntrinsics", "stdDeviationsExtrinsics", "perViewErrors", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
#[inline]
pub fn calibrate_camera_charuco_extended(charuco_corners: &impl ToInputArray, charuco_ids: &impl ToInputArray, board: &core::Ptr<crate::aruco::CharucoBoard>, image_size: core::Size, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &mut impl ToInputOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, std_deviations_intrinsics: &mut impl ToOutputArray, std_deviations_extrinsics: &mut impl ToOutputArray, per_view_errors: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
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
	unsafe { sys::cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), &image_size, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), std_deviations_intrinsics.as_raw__OutputArray(), std_deviations_extrinsics.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
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
// calibrateCameraCharuco(InputArrayOfArrays, InputArrayOfArrays, const Ptr<CharucoBoard> &, Size, InputOutputArray, InputOutputArray, OutputArrayOfArrays, OutputArrayOfArrays, int, TermCriteria)(InputArray, InputArray, CppPassByVoidPtr, SimpleClass, InputOutputArray, InputOutputArray, OutputArray, OutputArray, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:255
// ("cv::aruco::calibrateCameraCharuco", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "imageSize", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "flags", "criteria"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "cv::Size", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "cv::TermCriteria"]), _)]),
#[inline]
pub fn calibrate_camera_charuco(charuco_corners: &impl ToInputArray, charuco_ids: &impl ToInputArray, board: &core::Ptr<crate::aruco::CharucoBoard>, image_size: core::Size, camera_matrix: &mut impl ToInputOutputArray, dist_coeffs: &mut impl ToInputOutputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, flags: i32, criteria: core::TermCriteria) -> Result<f64> {
	input_array_arg!(charuco_corners);
	input_array_arg!(charuco_ids);
	input_output_array_arg!(camera_matrix);
	input_output_array_arg!(dist_coeffs);
	output_array_arg!(rvecs);
	output_array_arg!(tvecs);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_calibrateCameraCharuco_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_Size_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int_TermCriteria(charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), &image_size, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, &criteria, ocvrs_return.as_mut_ptr()) };
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
/// homography. Homography is faster than reprojection but can slightly reduce the detection rate.
///
/// ## Note
/// This alternative version of [detect_charuco_diamond] function uses the following default values for its arguments:
/// * camera_matrix: noArray()
/// * dist_coeffs: noArray()
/// * dictionary: getPredefinedDictionary(PREDEFINED_DICTIONARY_NAME(0))
// cv::aruco::detectCharucoDiamond(InputArray, InputArray, InputArray, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:286
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
/// homography. Homography is faster than reprojection but can slightly reduce the detection rate.
///
/// ## C++ default parameters
/// * camera_matrix: noArray()
/// * dist_coeffs: noArray()
/// * dictionary: getPredefinedDictionary(PREDEFINED_DICTIONARY_NAME(0))
// detectCharucoDiamond(InputArray, InputArrayOfArrays, InputArray, float, OutputArrayOfArrays, OutputArray, InputArray, InputArray, Ptr<Dictionary>)(InputArray, InputArray, InputArray, Primitive, OutputArray, OutputArray, InputArray, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:286
// ("cv::aruco::detectCharucoDiamond", vec![(pred!(mut, ["image", "markerCorners", "markerIds", "squareMarkerLengthRate", "diamondCorners", "diamondIds", "cameraMatrix", "distCoeffs", "dictionary"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "float", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Ptr<cv::aruco::Dictionary>"]), _)]),
#[inline]
pub fn detect_charuco_diamond(image: &impl ToInputArray, marker_corners: &impl ToInputArray, marker_ids: &impl ToInputArray, square_marker_length_rate: f32, diamond_corners: &mut impl ToOutputArray, diamond_ids: &mut impl ToOutputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, mut dictionary: core::Ptr<crate::aruco::Dictionary>) -> Result<()> {
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
///
/// Performs marker detection in the input image. Only markers included in the specific dictionary
/// are searched. For each detected marker, it returns the 2D position of its corner in the image
/// and its corresponding identifier.
/// Note that this function does not perform pose estimation.
///
/// Note: The function does not correct lens distortion or takes it into account. It's recommended to undistort
/// input image with corresponging camera model, if camera parameters are known
/// ## See also
/// undistort, estimatePoseSingleMarkers,  estimatePoseBoard
///
/// ## Note
/// This alternative version of [detect_markers] function uses the following default values for its arguments:
/// * parameters: DetectorParameters::create()
/// * rejected_img_points: noArray()
// cv::aruco::detectMarkers(InputArray, CppPassByVoidPtr, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:220
// ("cv::aruco::detectMarkers", vec![(pred!(mut, ["image", "dictionary", "corners", "ids"], ["const cv::_InputArray*", "const cv::Ptr<cv::aruco::Dictionary>*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn detect_markers_def(image: &impl ToInputArray, dictionary: &core::Ptr<crate::aruco::Dictionary>, corners: &mut impl ToOutputArray, ids: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(image);
	output_array_arg!(corners);
	output_array_arg!(ids);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_detectMarkers_const__InputArrayR_const_PtrLDictionaryGR_const__OutputArrayR_const__OutputArrayR(image.as_raw__InputArray(), dictionary.as_raw_PtrOfDictionary(), corners.as_raw__OutputArray(), ids.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
///
/// Performs marker detection in the input image. Only markers included in the specific dictionary
/// are searched. For each detected marker, it returns the 2D position of its corner in the image
/// and its corresponding identifier.
/// Note that this function does not perform pose estimation.
///
/// Note: The function does not correct lens distortion or takes it into account. It's recommended to undistort
/// input image with corresponging camera model, if camera parameters are known
/// ## See also
/// undistort, estimatePoseSingleMarkers,  estimatePoseBoard
///
/// ## C++ default parameters
/// * parameters: DetectorParameters::create()
/// * rejected_img_points: noArray()
// detectMarkers(InputArray, const Ptr<Dictionary> &, OutputArrayOfArrays, OutputArray, const Ptr<DetectorParameters> &, OutputArrayOfArrays)(InputArray, CppPassByVoidPtr, OutputArray, OutputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:220
// ("cv::aruco::detectMarkers", vec![(pred!(mut, ["image", "dictionary", "corners", "ids", "parameters", "rejectedImgPoints"], ["const cv::_InputArray*", "const cv::Ptr<cv::aruco::Dictionary>*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::Ptr<cv::aruco::DetectorParameters>*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn detect_markers(image: &impl ToInputArray, dictionary: &core::Ptr<crate::aruco::Dictionary>, corners: &mut impl ToOutputArray, ids: &mut impl ToOutputArray, parameters: &core::Ptr<crate::aruco::DetectorParameters>, rejected_img_points: &mut impl ToOutputArray) -> Result<()> {
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

/// ## Note
/// This alternative version of [draw_charuco_diamond] function uses the following default values for its arguments:
/// * margin_size: 0
/// * border_bits: 1
// cv::aruco::drawCharucoDiamond(CppPassByVoidPtr, SimpleClass, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:335
// ("cv::aruco::drawCharucoDiamond", vec![(pred!(mut, ["dictionary", "ids", "squareLength", "markerLength", "img"], ["const cv::Ptr<cv::aruco::Dictionary>*", "cv::Vec4i", "int", "int", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn draw_charuco_diamond_def(dictionary: &core::Ptr<crate::aruco::Dictionary>, ids: core::Vec4i, square_length: i32, marker_length: i32, img: &mut impl ToOutputArray) -> Result<()> {
	output_array_arg!(img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_drawCharucoDiamond_const_PtrLDictionaryGR_Vec4i_int_int_const__OutputArrayR(dictionary.as_raw_PtrOfDictionary(), &ids, square_length, marker_length, img.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * margin_size: 0
/// * border_bits: 1
// drawCharucoDiamond(const Ptr<Dictionary> &, Vec4i, int, int, OutputArray, int, int)(CppPassByVoidPtr, SimpleClass, Primitive, Primitive, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:335
// ("cv::aruco::drawCharucoDiamond", vec![(pred!(mut, ["dictionary", "ids", "squareLength", "markerLength", "img", "marginSize", "borderBits"], ["const cv::Ptr<cv::aruco::Dictionary>*", "cv::Vec4i", "int", "int", "const cv::_OutputArray*", "int", "int"]), _)]),
#[inline]
pub fn draw_charuco_diamond(dictionary: &core::Ptr<crate::aruco::Dictionary>, ids: core::Vec4i, square_length: i32, marker_length: i32, img: &mut impl ToOutputArray, margin_size: i32, border_bits: i32) -> Result<()> {
	output_array_arg!(img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_drawCharucoDiamond_const_PtrLDictionaryGR_Vec4i_int_int_const__OutputArrayR_int_int(dictionary.as_raw_PtrOfDictionary(), &ids, square_length, marker_length, img.as_raw__OutputArray(), margin_size, border_bits, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
/// ## Note
/// This alternative version of [draw_detected_corners_charuco] function uses the following default values for its arguments:
/// * charuco_ids: noArray()
/// * corner_color: Scalar(255,0,0)
// cv::aruco::drawDetectedCornersCharuco(InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:205
// ("cv::aruco::drawDetectedCornersCharuco", vec![(pred!(mut, ["image", "charucoCorners"], ["const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn draw_detected_corners_charuco_def(image: &mut impl ToInputOutputArray, charuco_corners: &impl ToInputArray) -> Result<()> {
	input_output_array_arg!(image);
	input_array_arg!(charuco_corners);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_drawDetectedCornersCharuco_const__InputOutputArrayR_const__InputArrayR(image.as_raw__InputOutputArray(), charuco_corners.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
// drawDetectedCornersCharuco(InputOutputArray, InputArray, InputArray, Scalar)(InputOutputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:205
// ("cv::aruco::drawDetectedCornersCharuco", vec![(pred!(mut, ["image", "charucoCorners", "charucoIds", "cornerColor"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Scalar"]), _)]),
#[inline]
pub fn draw_detected_corners_charuco(image: &mut impl ToInputOutputArray, charuco_corners: &impl ToInputArray, charuco_ids: &impl ToInputArray, corner_color: core::Scalar) -> Result<()> {
	input_output_array_arg!(image);
	input_array_arg!(charuco_corners);
	input_array_arg!(charuco_ids);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_drawDetectedCornersCharuco_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(image.as_raw__InputOutputArray(), charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), &corner_color, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
/// ## Note
/// This alternative version of [draw_detected_diamonds] function uses the following default values for its arguments:
/// * diamond_ids: noArray()
/// * border_color: Scalar(0,0,255)
// cv::aruco::drawDetectedDiamonds(InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:313
// ("cv::aruco::drawDetectedDiamonds", vec![(pred!(mut, ["image", "diamondCorners"], ["const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn draw_detected_diamonds_def(image: &mut impl ToInputOutputArray, diamond_corners: &impl ToInputArray) -> Result<()> {
	input_output_array_arg!(image);
	input_array_arg!(diamond_corners);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_drawDetectedDiamonds_const__InputOutputArrayR_const__InputArrayR(image.as_raw__InputOutputArray(), diamond_corners.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
// drawDetectedDiamonds(InputOutputArray, InputArrayOfArrays, InputArray, Scalar)(InputOutputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:313
// ("cv::aruco::drawDetectedDiamonds", vec![(pred!(mut, ["image", "diamondCorners", "diamondIds", "borderColor"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Scalar"]), _)]),
#[inline]
pub fn draw_detected_diamonds(image: &mut impl ToInputOutputArray, diamond_corners: &impl ToInputArray, diamond_ids: &impl ToInputArray, border_color: core::Scalar) -> Result<()> {
	input_output_array_arg!(image);
	input_array_arg!(diamond_corners);
	input_array_arg!(diamond_ids);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_drawDetectedDiamonds_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(image.as_raw__InputOutputArray(), diamond_corners.as_raw__InputArray(), diamond_ids.as_raw__InputArray(), &border_color, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
/// ## Note
/// This alternative version of [draw_detected_markers] function uses the following default values for its arguments:
/// * ids: noArray()
/// * border_color: Scalar(0,255,0)
// cv::aruco::drawDetectedMarkers(InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:537
// ("cv::aruco::drawDetectedMarkers", vec![(pred!(mut, ["image", "corners"], ["const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn draw_detected_markers_def(image: &mut impl ToInputOutputArray, corners: &impl ToInputArray) -> Result<()> {
	input_output_array_arg!(image);
	input_array_arg!(corners);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_drawDetectedMarkers_const__InputOutputArrayR_const__InputArrayR(image.as_raw__InputOutputArray(), corners.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
// drawDetectedMarkers(InputOutputArray, InputArrayOfArrays, InputArray, Scalar)(InputOutputArray, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:537
// ("cv::aruco::drawDetectedMarkers", vec![(pred!(mut, ["image", "corners", "ids", "borderColor"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::Scalar"]), _)]),
#[inline]
pub fn draw_detected_markers(image: &mut impl ToInputOutputArray, corners: &impl ToInputArray, ids: &impl ToInputArray, border_color: core::Scalar) -> Result<()> {
	input_output_array_arg!(image);
	input_array_arg!(corners);
	input_array_arg!(ids);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_drawDetectedMarkers_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(image.as_raw__InputOutputArray(), corners.as_raw__InputArray(), ids.as_raw__InputArray(), &border_color, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
/// ## Note
/// This alternative version of [draw_marker] function uses the following default values for its arguments:
/// * border_bits: 1
// cv::aruco::drawMarker(CppPassByVoidPtr, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:555
// ("cv::aruco::drawMarker", vec![(pred!(mut, ["dictionary", "id", "sidePixels", "img"], ["const cv::Ptr<cv::aruco::Dictionary>*", "int", "int", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn draw_marker_def(dictionary: &core::Ptr<crate::aruco::Dictionary>, id: i32, side_pixels: i32, img: &mut impl ToOutputArray) -> Result<()> {
	output_array_arg!(img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_drawMarker_const_PtrLDictionaryGR_int_int_const__OutputArrayR(dictionary.as_raw_PtrOfDictionary(), id, side_pixels, img.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
// drawMarker(const Ptr<Dictionary> &, int, int, OutputArray, int)(CppPassByVoidPtr, Primitive, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:555
// ("cv::aruco::drawMarker", vec![(pred!(mut, ["dictionary", "id", "sidePixels", "img", "borderBits"], ["const cv::Ptr<cv::aruco::Dictionary>*", "int", "int", "const cv::_OutputArray*", "int"]), _)]),
#[inline]
pub fn draw_marker(dictionary: &core::Ptr<crate::aruco::Dictionary>, id: i32, side_pixels: i32, img: &mut impl ToOutputArray, border_bits: i32) -> Result<()> {
	output_array_arg!(img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_drawMarker_const_PtrLDictionaryGR_int_int_const__OutputArrayR_int(dictionary.as_raw_PtrOfDictionary(), id, side_pixels, img.as_raw__OutputArray(), border_bits, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
/// ## Note
/// This alternative version of [draw_planar_board] function uses the following default values for its arguments:
/// * margin_size: 0
/// * border_bits: 1
// cv::aruco::drawPlanarBoard(CppPassByVoidPtr, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:575
// ("cv::aruco::drawPlanarBoard", vec![(pred!(mut, ["board", "outSize", "img"], ["const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn draw_planar_board_def(board: &core::Ptr<crate::aruco::Board>, out_size: core::Size, img: &mut impl ToOutputArray) -> Result<()> {
	output_array_arg!(img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_drawPlanarBoard_const_PtrLBoardGR_Size_const__OutputArrayR(board.as_raw_PtrOfBoard(), &out_size, img.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
// drawPlanarBoard(const Ptr<Board> &, Size, OutputArray, int, int)(CppPassByVoidPtr, SimpleClass, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:575
// ("cv::aruco::drawPlanarBoard", vec![(pred!(mut, ["board", "outSize", "img", "marginSize", "borderBits"], ["const cv::Ptr<cv::aruco::Board>*", "cv::Size", "const cv::_OutputArray*", "int", "int"]), _)]),
#[inline]
pub fn draw_planar_board(board: &core::Ptr<crate::aruco::Board>, out_size: core::Size, img: &mut impl ToOutputArray, margin_size: i32, border_bits: i32) -> Result<()> {
	output_array_arg!(img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_drawPlanarBoard_const_PtrLBoardGR_Size_const__OutputArrayR_int_int(board.as_raw_PtrOfBoard(), &out_size, img.as_raw__OutputArray(), margin_size, border_bits, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
/// ## See also
/// use cv::drawFrameAxes to get world coordinate system axis for object points
///
/// ## Note
/// This alternative version of [estimate_pose_board] function uses the following default values for its arguments:
/// * use_extrinsic_guess: false
// cv::aruco::estimatePoseBoard(InputArray, InputArray, CppPassByVoidPtr, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:472
// ("cv::aruco::estimatePoseBoard", vec![(pred!(mut, ["corners", "ids", "board", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn estimate_pose_board_def(corners: &impl ToInputArray, ids: &impl ToInputArray, board: &core::Ptr<crate::aruco::Board>, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToOutputArray, tvec: &mut impl ToOutputArray) -> Result<i32> {
	input_array_arg!(corners);
	input_array_arg!(ids);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(rvec);
	output_array_arg!(tvec);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_estimatePoseBoard_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(corners.as_raw__InputArray(), ids.as_raw__InputArray(), board.as_raw_PtrOfBoard(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
/// ## See also
/// use cv::drawFrameAxes to get world coordinate system axis for object points
///
/// ## C++ default parameters
/// * use_extrinsic_guess: false
// estimatePoseBoard(InputArrayOfArrays, InputArray, const Ptr<Board> &, InputArray, InputArray, OutputArray, OutputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:472
// ("cv::aruco::estimatePoseBoard", vec![(pred!(mut, ["corners", "ids", "board", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool"]), _)]),
#[inline]
pub fn estimate_pose_board(corners: &impl ToInputArray, ids: &impl ToInputArray, board: &core::Ptr<crate::aruco::Board>, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToOutputArray, tvec: &mut impl ToOutputArray, use_extrinsic_guess: bool) -> Result<i32> {
	input_array_arg!(corners);
	input_array_arg!(ids);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(rvec);
	output_array_arg!(tvec);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_estimatePoseBoard_const__InputArrayR_const__InputArrayR_const_PtrLBoardGR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool(corners.as_raw__InputArray(), ids.as_raw__InputArray(), board.as_raw_PtrOfBoard(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), use_extrinsic_guess, ocvrs_return.as_mut_ptr()) };
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
// cv::aruco::estimatePoseCharucoBoard(InputArray, InputArray, CppPassByVoidPtr, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:186
// ("cv::aruco::estimatePoseCharucoBoard", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "cameraMatrix", "distCoeffs", "rvec", "tvec"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn estimate_pose_charuco_board_def(charuco_corners: &impl ToInputArray, charuco_ids: &impl ToInputArray, board: &core::Ptr<crate::aruco::CharucoBoard>, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToOutputArray, tvec: &mut impl ToOutputArray) -> Result<bool> {
	input_array_arg!(charuco_corners);
	input_array_arg!(charuco_ids);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(rvec);
	output_array_arg!(tvec);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_estimatePoseCharucoBoard_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
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
// estimatePoseCharucoBoard(InputArray, InputArray, const Ptr<CharucoBoard> &, InputArray, InputArray, OutputArray, OutputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, InputArray, InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:186
// ("cv::aruco::estimatePoseCharucoBoard", vec![(pred!(mut, ["charucoCorners", "charucoIds", "board", "cameraMatrix", "distCoeffs", "rvec", "tvec", "useExtrinsicGuess"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool"]), _)]),
#[inline]
pub fn estimate_pose_charuco_board(charuco_corners: &impl ToInputArray, charuco_ids: &impl ToInputArray, board: &core::Ptr<crate::aruco::CharucoBoard>, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvec: &mut impl ToOutputArray, tvec: &mut impl ToOutputArray, use_extrinsic_guess: bool) -> Result<bool> {
	input_array_arg!(charuco_corners);
	input_array_arg!(charuco_ids);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(rvec);
	output_array_arg!(tvec);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_estimatePoseCharucoBoard_const__InputArrayR_const__InputArrayR_const_PtrLCharucoBoardGR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool(charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), use_extrinsic_guess, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
/// * estimateParameters: set the origin of coordinate system and the coordinates of the four corners of the marker
/// (default estimateParameters.pattern = PatternPos::CCW_center, estimateParameters.useExtrinsicGuess = false,
/// estimateParameters.solvePnPMethod = SOLVEPNP_ITERATIVE).
///
/// This function receives the detected markers and returns their pose estimation respect to
/// the camera individually. So for each marker, one rotation and translation vector is returned.
/// The returned transformation is the one that transforms points from each marker coordinate system
/// to the camera coordinate system.
/// The marker coordinate system is centered on the middle (by default) or on the top-left corner of the marker,
/// with the Z axis perpendicular to the marker plane.
/// estimateParameters defines the coordinates of the four corners of the marker in its own coordinate system (by default) are:
/// (-markerLength/2, markerLength/2, 0), (markerLength/2, markerLength/2, 0),
/// (markerLength/2, -markerLength/2, 0), (-markerLength/2, -markerLength/2, 0)
/// use cv::drawFrameAxes to get world coordinate system axis for object points
/// [tutorial_aruco_detection]
/// EstimateParameters
/// PatternPos
///
/// ## Note
/// This alternative version of [estimate_pose_single_markers] function uses the following default values for its arguments:
/// * _obj_points: noArray()
/// * estimate_parameters: EstimateParameters::create()
// cv::aruco::estimatePoseSingleMarkers(InputArray, Primitive, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:310
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
/// * estimateParameters: set the origin of coordinate system and the coordinates of the four corners of the marker
/// (default estimateParameters.pattern = PatternPos::CCW_center, estimateParameters.useExtrinsicGuess = false,
/// estimateParameters.solvePnPMethod = SOLVEPNP_ITERATIVE).
///
/// This function receives the detected markers and returns their pose estimation respect to
/// the camera individually. So for each marker, one rotation and translation vector is returned.
/// The returned transformation is the one that transforms points from each marker coordinate system
/// to the camera coordinate system.
/// The marker coordinate system is centered on the middle (by default) or on the top-left corner of the marker,
/// with the Z axis perpendicular to the marker plane.
/// estimateParameters defines the coordinates of the four corners of the marker in its own coordinate system (by default) are:
/// (-markerLength/2, markerLength/2, 0), (markerLength/2, markerLength/2, 0),
/// (markerLength/2, -markerLength/2, 0), (-markerLength/2, -markerLength/2, 0)
/// use cv::drawFrameAxes to get world coordinate system axis for object points
/// [tutorial_aruco_detection]
/// EstimateParameters
/// PatternPos
///
/// ## C++ default parameters
/// * _obj_points: noArray()
/// * estimate_parameters: EstimateParameters::create()
// estimatePoseSingleMarkers(InputArrayOfArrays, float, InputArray, InputArray, OutputArray, OutputArray, OutputArray, Ptr<EstimateParameters>)(InputArray, Primitive, InputArray, InputArray, OutputArray, OutputArray, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:310
// ("cv::aruco::estimatePoseSingleMarkers", vec![(pred!(mut, ["corners", "markerLength", "cameraMatrix", "distCoeffs", "rvecs", "tvecs", "_objPoints", "estimateParameters"], ["const cv::_InputArray*", "float", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::Ptr<cv::aruco::EstimateParameters>"]), _)]),
#[inline]
pub fn estimate_pose_single_markers(corners: &impl ToInputArray, marker_length: f32, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, rvecs: &mut impl ToOutputArray, tvecs: &mut impl ToOutputArray, _obj_points: &mut impl ToOutputArray, mut estimate_parameters: core::Ptr<crate::aruco::EstimateParameters>) -> Result<()> {
	input_array_arg!(corners);
	input_array_arg!(camera_matrix);
	input_array_arg!(dist_coeffs);
	output_array_arg!(rvecs);
	output_array_arg!(tvecs);
	output_array_arg!(_obj_points);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_estimatePoseSingleMarkers_const__InputArrayR_float_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_PtrLEstimateParametersG(corners.as_raw__InputArray(), marker_length, camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), _obj_points.as_raw__OutputArray(), estimate_parameters.as_raw_mut_PtrOfEstimateParameters(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## See also
/// generateCustomDictionary
///
/// ## Note
/// This alternative version of [custom_dictionary] function uses the following default values for its arguments:
/// * random_seed: 0
// cv::aruco::generateCustomDictionary(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:196
// ("cv::aruco::generateCustomDictionary", vec![(pred!(mut, ["nMarkers", "markerSize"], ["int", "int"]), _)]),
#[inline]
pub fn custom_dictionary_def(n_markers: i32, marker_size: i32) -> Result<core::Ptr<crate::aruco::Dictionary>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_generateCustomDictionary_int_int(n_markers, marker_size, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(ret) };
	Ok(ret)
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
/// ## Note
/// This alternative version of [custom_dictionary_from] function uses the following default values for its arguments:
/// * random_seed: 0
// cv::aruco::generateCustomDictionary(Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:215
// ("cv::aruco::generateCustomDictionary", vec![(pred!(mut, ["nMarkers", "markerSize", "baseDictionary"], ["int", "int", "const cv::Ptr<cv::aruco::Dictionary>*"]), _)]),
#[inline]
pub fn custom_dictionary_from_def(n_markers: i32, marker_size: i32, base_dictionary: &core::Ptr<crate::aruco::Dictionary>) -> Result<core::Ptr<crate::aruco::Dictionary>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_generateCustomDictionary_int_int_const_PtrLDictionaryGR(n_markers, marker_size, base_dictionary.as_raw_PtrOfDictionary(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(ret) };
	Ok(ret)
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
// generateCustomDictionary(int, int, const Ptr<Dictionary> &, int)(Primitive, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:215
// ("cv::aruco::generateCustomDictionary", vec![(pred!(mut, ["nMarkers", "markerSize", "baseDictionary", "randomSeed"], ["int", "int", "const cv::Ptr<cv::aruco::Dictionary>*", "int"]), _)]),
#[inline]
pub fn custom_dictionary_from(n_markers: i32, marker_size: i32, base_dictionary: &core::Ptr<crate::aruco::Dictionary>, random_seed: i32) -> Result<core::Ptr<crate::aruco::Dictionary>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_generateCustomDictionary_int_int_const_PtrLDictionaryGR_int(n_markers, marker_size, base_dictionary.as_raw_PtrOfDictionary(), random_seed, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## See also
/// generateCustomDictionary
///
/// ## C++ default parameters
/// * random_seed: 0
// generateCustomDictionary(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:196
// ("cv::aruco::generateCustomDictionary", vec![(pred!(mut, ["nMarkers", "markerSize", "randomSeed"], ["int", "int", "int"]), _)]),
#[inline]
pub fn custom_dictionary(n_markers: i32, marker_size: i32, random_seed: i32) -> Result<core::Ptr<crate::aruco::Dictionary>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_generateCustomDictionary_int_int_int(n_markers, marker_size, random_seed, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(ret) };
	Ok(ret)
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
// getBoardObjectAndImagePoints(const Ptr<Board> &, InputArrayOfArrays, InputArray, OutputArray, OutputArray)(CppPassByVoidPtr, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:652
// ("cv::aruco::getBoardObjectAndImagePoints", vec![(pred!(mut, ["board", "detectedCorners", "detectedIds", "objPoints", "imgPoints"], ["const cv::Ptr<cv::aruco::Board>*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn get_board_object_and_image_points(board: &core::Ptr<crate::aruco::Board>, detected_corners: &impl ToInputArray, detected_ids: &impl ToInputArray, obj_points: &mut impl ToOutputArray, img_points: &mut impl ToOutputArray) -> Result<()> {
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

/// Returns one of the predefined dictionaries defined in PREDEFINED_DICTIONARY_NAME
// getPredefinedDictionary(PREDEFINED_DICTIONARY_NAME)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:184
// ("cv::aruco::getPredefinedDictionary", vec![(pred!(mut, ["name"], ["cv::aruco::PREDEFINED_DICTIONARY_NAME"]), _)]),
#[inline]
pub fn get_predefined_dictionary(name: crate::aruco::PREDEFINED_DICTIONARY_NAME) -> Result<core::Ptr<crate::aruco::Dictionary>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_getPredefinedDictionary_PREDEFINED_DICTIONARY_NAME(name, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Returns one of the predefined dictionaries referenced by DICT_*.
// getPredefinedDictionary(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:190
// ("cv::aruco::getPredefinedDictionary", vec![(pred!(mut, ["dict"], ["int"]), _)]),
#[inline]
pub fn get_predefined_dictionary_i32(dict: i32) -> Result<core::Ptr<crate::aruco::Dictionary>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_getPredefinedDictionary_int(dict, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(ret) };
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
/// ## Note
/// This alternative version of [interpolate_corners_charuco] function uses the following default values for its arguments:
/// * camera_matrix: noArray()
/// * dist_coeffs: noArray()
/// * min_markers: 2
// cv::aruco::interpolateCornersCharuco(InputArray, InputArray, InputArray, CppPassByVoidPtr, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:158
// ("cv::aruco::interpolateCornersCharuco", vec![(pred!(mut, ["markerCorners", "markerIds", "image", "board", "charucoCorners", "charucoIds"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn interpolate_corners_charuco_def(marker_corners: &impl ToInputArray, marker_ids: &impl ToInputArray, image: &impl ToInputArray, board: &core::Ptr<crate::aruco::CharucoBoard>, charuco_corners: &mut impl ToOutputArray, charuco_ids: &mut impl ToOutputArray) -> Result<i32> {
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
/// ## C++ default parameters
/// * camera_matrix: noArray()
/// * dist_coeffs: noArray()
/// * min_markers: 2
// interpolateCornersCharuco(InputArrayOfArrays, InputArray, InputArray, const Ptr<CharucoBoard> &, OutputArray, OutputArray, InputArray, InputArray, int)(InputArray, InputArray, InputArray, CppPassByVoidPtr, OutputArray, OutputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:158
// ("cv::aruco::interpolateCornersCharuco", vec![(pred!(mut, ["markerCorners", "markerIds", "image", "board", "charucoCorners", "charucoIds", "cameraMatrix", "distCoeffs", "minMarkers"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Ptr<cv::aruco::CharucoBoard>*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
#[inline]
pub fn interpolate_corners_charuco(marker_corners: &impl ToInputArray, marker_ids: &impl ToInputArray, image: &impl ToInputArray, board: &core::Ptr<crate::aruco::CharucoBoard>, charuco_corners: &mut impl ToOutputArray, charuco_ids: &mut impl ToOutputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, min_markers: i32) -> Result<i32> {
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
/// ## Note
/// This alternative version of [refine_detected_markers] function uses the following default values for its arguments:
/// * camera_matrix: noArray()
/// * dist_coeffs: noArray()
/// * min_rep_distance: 10.f
/// * error_correction_rate: 3.f
/// * check_all_orders: true
/// * recovered_idxs: noArray()
/// * parameters: DetectorParameters::create()
// cv::aruco::refineDetectedMarkers(InputArray, CppPassByVoidPtr, InputOutputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:510
// ("cv::aruco::refineDetectedMarkers", vec![(pred!(mut, ["image", "board", "detectedCorners", "detectedIds", "rejectedCorners"], ["const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
#[inline]
pub fn refine_detected_markers_def(image: &impl ToInputArray, board: &core::Ptr<crate::aruco::Board>, detected_corners: &mut impl ToInputOutputArray, detected_ids: &mut impl ToInputOutputArray, rejected_corners: &mut impl ToInputOutputArray) -> Result<()> {
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
// refineDetectedMarkers(InputArray, const Ptr<Board> &, InputOutputArrayOfArrays, InputOutputArray, InputOutputArrayOfArrays, InputArray, InputArray, float, float, bool, OutputArray, const Ptr<DetectorParameters> &)(InputArray, CppPassByVoidPtr, InputOutputArray, InputOutputArray, InputOutputArray, InputArray, InputArray, Primitive, Primitive, Primitive, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:510
// ("cv::aruco::refineDetectedMarkers", vec![(pred!(mut, ["image", "board", "detectedCorners", "detectedIds", "rejectedCorners", "cameraMatrix", "distCoeffs", "minRepDistance", "errorCorrectionRate", "checkAllOrders", "recoveredIdxs", "parameters"], ["const cv::_InputArray*", "const cv::Ptr<cv::aruco::Board>*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "float", "float", "bool", "const cv::_OutputArray*", "const cv::Ptr<cv::aruco::DetectorParameters>*"]), _)]),
#[inline]
pub fn refine_detected_markers(image: &impl ToInputArray, board: &core::Ptr<crate::aruco::Board>, detected_corners: &mut impl ToInputOutputArray, detected_ids: &mut impl ToInputOutputArray, rejected_corners: &mut impl ToInputOutputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, min_rep_distance: f32, error_correction_rate: f32, check_all_orders: bool, recovered_idxs: &mut impl ToOutputArray, parameters: &core::Ptr<crate::aruco::DetectorParameters>) -> Result<()> {
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

/// test whether the ChArUco markers are collinear
///
/// ## Parameters
/// * _board: layout of ChArUco board.
/// * _charucoIds: list of identifiers for each corner in charucoCorners per frame.
/// ## Returns
/// bool value, 1 (true) if detected corners form a line, 0 (false) if they do not.
///    solvePnP, calibration functions will fail if the corners are collinear (true).
///
/// The number of ids in charucoIDs should be <= the number of chessboard corners in the board.  This functions checks whether the charuco corners are on a straight line (returns true, if so), or not (false).  Axis parallel, as well as diagonal and other straight lines detected.  Degenerate cases: for number of charucoIDs <= 2, the function returns true.
// testCharucoCornersCollinear(const Ptr<CharucoBoard> &, InputArray)(CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:350
// ("cv::aruco::testCharucoCornersCollinear", vec![(pred!(mut, ["_board", "_charucoIds"], ["const cv::Ptr<cv::aruco::CharucoBoard>*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn test_charuco_corners_collinear(_board: &core::Ptr<crate::aruco::CharucoBoard>, _charuco_ids: &impl ToInputArray) -> Result<bool> {
	input_array_arg!(_charuco_ids);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_aruco_testCharucoCornersCollinear_const_PtrLCharucoBoardGR_const__InputArrayR(_board.as_raw_PtrOfCharucoBoard(), _charuco_ids.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Constant methods for [crate::aruco::Board]
// Board /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:327
pub trait BoardTraitConst {
	fn as_raw_Board(&self) -> *const c_void;

	/// array of object points of all the marker corners in the board
	///  each marker include its 4 corners in this order:
	/// *   objPoints[i][0] - left-top point of i-th marker
	/// *   objPoints[i][1] - right-top point of i-th marker
	/// *   objPoints[i][2] - right-bottom point of i-th marker
	/// *   objPoints[i][3] - left-bottom point of i-th marker
	///
	///  Markers are placed in a certain order - row by row, left to right in every row.
	///  For M markers, the size is Mx4.
	// cv::aruco::Board::objPoints() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:360
	// ("cv::aruco::Board::objPoints", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn obj_points(&self) -> core::Vector<core::Vector<core::Point3f>> {
		let ret = unsafe { sys::cv_aruco_Board_propObjPoints_const(self.as_raw_Board()) };
		let ret = unsafe { core::Vector::<core::Vector<core::Point3f>>::opencv_from_extern(ret) };
		ret
	}

	/// vector of the identifiers of the markers in the board (same size than objPoints)
	/// The identifiers refers to the board dictionary
	// cv::aruco::Board::ids() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:367
	// ("cv::aruco::Board::ids", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn ids(&self) -> core::Vector<i32> {
		let ret = unsafe { sys::cv_aruco_Board_propIds_const(self.as_raw_Board()) };
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		ret
	}

	/// coordinate of the bottom right corner of the board, is set when calling the function create()
	// cv::aruco::Board::rightBottomBorder() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:370
	// ("cv::aruco::Board::rightBottomBorder", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn right_bottom_border(&self) -> core::Point3f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Board_propRightBottomBorder_const(self.as_raw_Board(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

}

/// Mutable methods for [crate::aruco::Board]
pub trait BoardTrait: crate::aruco::BoardTraitConst {
	fn as_raw_mut_Board(&mut self) -> *mut c_void;

	/// array of object points of all the marker corners in the board
	///  each marker include its 4 corners in this order:
	/// *   objPoints[i][0] - left-top point of i-th marker
	/// *   objPoints[i][1] - right-top point of i-th marker
	/// *   objPoints[i][2] - right-bottom point of i-th marker
	/// *   objPoints[i][3] - left-bottom point of i-th marker
	///
	///  Markers are placed in a certain order - row by row, left to right in every row.
	///  For M markers, the size is Mx4.
	// cv::aruco::Board::setObjPoints(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:360
	// ("cv::aruco::Board::setObjPoints", vec![(pred!(mut, ["val"], ["const std::vector<std::vector<cv::Point3f>>"]), _)]),
	#[inline]
	fn set_obj_points(&mut self, val: core::Vector<core::Vector<core::Point3f>>) {
		let ret = unsafe { sys::cv_aruco_Board_propObjPoints_const_vectorLvectorLPoint3fGG(self.as_raw_mut_Board(), val.as_raw_VectorOfVectorOfPoint3f()) };
		ret
	}

	/// the dictionary of markers employed for this board
	// cv::aruco::Board::dictionary() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:363
	// ("cv::aruco::Board::dictionary", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn dictionary(&mut self) -> core::Ptr<crate::aruco::Dictionary> {
		let ret = unsafe { sys::cv_aruco_Board_propDictionary(self.as_raw_mut_Board()) };
		let ret = unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(ret) };
		ret
	}

	/// the dictionary of markers employed for this board
	// cv::aruco::Board::setDictionary(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:363
	// ("cv::aruco::Board::setDictionary", vec![(pred!(mut, ["val"], ["const cv::Ptr<cv::aruco::Dictionary>"]), _)]),
	#[inline]
	fn set_dictionary(&mut self, val: core::Ptr<crate::aruco::Dictionary>) {
		let ret = unsafe { sys::cv_aruco_Board_propDictionary_const_PtrLDictionaryG(self.as_raw_mut_Board(), val.as_raw_PtrOfDictionary()) };
		ret
	}

	/// vector of the identifiers of the markers in the board (same size than objPoints)
	/// The identifiers refers to the board dictionary
	// cv::aruco::Board::setIds(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:367
	// ("cv::aruco::Board::setIds", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
	#[inline]
	fn set_ids(&mut self, val: core::Vector<i32>) {
		let ret = unsafe { sys::cv_aruco_Board_propIds_const_vectorLintG(self.as_raw_mut_Board(), val.as_raw_VectorOfi32()) };
		ret
	}

	/// coordinate of the bottom right corner of the board, is set when calling the function create()
	// cv::aruco::Board::setRightBottomBorder(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:370
	// ("cv::aruco::Board::setRightBottomBorder", vec![(pred!(mut, ["val"], ["const cv::Point3f"]), _)]),
	#[inline]
	fn set_right_bottom_border(&mut self, val: core::Point3f) {
		let ret = unsafe { sys::cv_aruco_Board_propRightBottomBorder_const_Point3f(self.as_raw_mut_Board(), &val) };
		ret
	}

	/// Set ids vector
	///
	/// ## Parameters
	/// * ids: vector of the identifiers of the markers in the board (should be the same size
	/// as objPoints)
	///
	/// Recommended way to set ids vector, which will fail if the size of ids does not match size
	/// of objPoints.
	// setIds(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:349
	// ("cv::aruco::Board::setIds", vec![(pred!(mut, ["ids"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn set_ids_1(&mut self, ids: &impl ToInputArray) -> Result<()> {
		input_array_arg!(ids);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Board_setIds_const__InputArrayR(self.as_raw_mut_Board(), ids.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
// Board /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:327
pub struct Board {
	ptr: *mut c_void,
}

opencv_type_boxed! { Board }

impl Drop for Board {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_aruco_Board_delete(self.as_raw_mut_Board()) };
	}
}

unsafe impl Send for Board {}

impl crate::aruco::BoardTraitConst for Board {
	#[inline] fn as_raw_Board(&self) -> *const c_void { self.as_raw() }
}

impl crate::aruco::BoardTrait for Board {
	#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Board, crate::aruco::BoardTraitConst, as_raw_Board, crate::aruco::BoardTrait, as_raw_mut_Board }

impl Board {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_aruco_Board_defaultNew_const()) }
	}

	/// Provide way to create Board by passing necessary data. Specially needed in Python.
	///
	/// ## Parameters
	/// * objPoints: array of object points of all the marker corners in the board
	/// * dictionary: the dictionary of markers employed for this board
	/// * ids: vector of the identifiers of the markers in the board
	// create(InputArrayOfArrays, const Ptr<Dictionary> &, InputArray)(InputArray, CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:338
	// ("cv::aruco::Board::create", vec![(pred!(mut, ["objPoints", "dictionary", "ids"], ["const cv::_InputArray*", "const cv::Ptr<cv::aruco::Dictionary>*", "const cv::_InputArray*"]), _)]),
	#[inline]
	pub fn create(obj_points: &impl ToInputArray, dictionary: &core::Ptr<crate::aruco::Dictionary>, ids: &impl ToInputArray) -> Result<core::Ptr<crate::aruco::Board>> {
		input_array_arg!(obj_points);
		input_array_arg!(ids);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Board_create_const__InputArrayR_const_PtrLDictionaryGR_const__InputArrayR(obj_points.as_raw__InputArray(), dictionary.as_raw_PtrOfDictionary(), ids.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::aruco::Board>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for Board {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Board")
			.field("obj_points", &crate::aruco::BoardTraitConst::obj_points(self))
			.field("ids", &crate::aruco::BoardTraitConst::ids(self))
			.field("right_bottom_border", &crate::aruco::BoardTraitConst::right_bottom_border(self))
			.finish()
	}
}

impl Default for Board {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::aruco::CharucoBoard]
// CharucoBoard /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:62
pub trait CharucoBoardTraitConst: crate::aruco::BoardTraitConst {
	fn as_raw_CharucoBoard(&self) -> *const c_void;

	// cv::aruco::CharucoBoard::chessboardCorners() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:66
	// ("cv::aruco::CharucoBoard::chessboardCorners", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn chessboard_corners(&self) -> core::Vector<core::Point3f> {
		let ret = unsafe { sys::cv_aruco_CharucoBoard_propChessboardCorners_const(self.as_raw_CharucoBoard()) };
		let ret = unsafe { core::Vector::<core::Point3f>::opencv_from_extern(ret) };
		ret
	}

	// cv::aruco::CharucoBoard::nearestMarkerIdx() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:69
	// ("cv::aruco::CharucoBoard::nearestMarkerIdx", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn nearest_marker_idx(&self) -> core::Vector<core::Vector<i32>> {
		let ret = unsafe { sys::cv_aruco_CharucoBoard_propNearestMarkerIdx_const(self.as_raw_CharucoBoard()) };
		let ret = unsafe { core::Vector::<core::Vector<i32>>::opencv_from_extern(ret) };
		ret
	}

	// cv::aruco::CharucoBoard::nearestMarkerCorners() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:70
	// ("cv::aruco::CharucoBoard::nearestMarkerCorners", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn nearest_marker_corners(&self) -> core::Vector<core::Vector<i32>> {
		let ret = unsafe { sys::cv_aruco_CharucoBoard_propNearestMarkerCorners_const(self.as_raw_CharucoBoard()) };
		let ret = unsafe { core::Vector::<core::Vector<i32>>::opencv_from_extern(ret) };
		ret
	}

	// getChessboardSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:106
	// ("cv::aruco::CharucoBoard::getChessboardSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_chessboard_size(&self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_CharucoBoard_getChessboardSize_const(self.as_raw_CharucoBoard(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getSquareLength()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:111
	// ("cv::aruco::CharucoBoard::getSquareLength", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_square_length(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_CharucoBoard_getSquareLength_const(self.as_raw_CharucoBoard(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMarkerLength()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:116
	// ("cv::aruco::CharucoBoard::getMarkerLength", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_marker_length(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_CharucoBoard_getMarkerLength_const(self.as_raw_CharucoBoard(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::aruco::CharucoBoard]
pub trait CharucoBoardTrait: crate::aruco::BoardTrait + crate::aruco::CharucoBoardTraitConst {
	fn as_raw_mut_CharucoBoard(&mut self) -> *mut c_void;

	// cv::aruco::CharucoBoard::setChessboardCorners(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:66
	// ("cv::aruco::CharucoBoard::setChessboardCorners", vec![(pred!(mut, ["val"], ["const std::vector<cv::Point3f>"]), _)]),
	#[inline]
	fn set_chessboard_corners(&mut self, val: core::Vector<core::Point3f>) {
		let ret = unsafe { sys::cv_aruco_CharucoBoard_propChessboardCorners_const_vectorLPoint3fG(self.as_raw_mut_CharucoBoard(), val.as_raw_VectorOfPoint3f()) };
		ret
	}

	// cv::aruco::CharucoBoard::setNearestMarkerIdx(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:69
	// ("cv::aruco::CharucoBoard::setNearestMarkerIdx", vec![(pred!(mut, ["val"], ["const std::vector<std::vector<int>>"]), _)]),
	#[inline]
	fn set_nearest_marker_idx(&mut self, val: core::Vector<core::Vector<i32>>) {
		let ret = unsafe { sys::cv_aruco_CharucoBoard_propNearestMarkerIdx_const_vectorLvectorLintGG(self.as_raw_mut_CharucoBoard(), val.as_raw_VectorOfVectorOfi32()) };
		ret
	}

	// cv::aruco::CharucoBoard::setNearestMarkerCorners(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:70
	// ("cv::aruco::CharucoBoard::setNearestMarkerCorners", vec![(pred!(mut, ["val"], ["const std::vector<std::vector<int>>"]), _)]),
	#[inline]
	fn set_nearest_marker_corners(&mut self, val: core::Vector<core::Vector<i32>>) {
		let ret = unsafe { sys::cv_aruco_CharucoBoard_propNearestMarkerCorners_const_vectorLvectorLintGG(self.as_raw_mut_CharucoBoard(), val.as_raw_VectorOfVectorOfi32()) };
		ret
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
	// draw(Size, OutputArray, int, int)(SimpleClass, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:83
	// ("cv::aruco::CharucoBoard::draw", vec![(pred!(mut, ["outSize", "img", "marginSize", "borderBits"], ["cv::Size", "const cv::_OutputArray*", "int", "int"]), _)]),
	#[inline]
	fn draw(&mut self, out_size: core::Size, img: &mut impl ToOutputArray, margin_size: i32, border_bits: i32) -> Result<()> {
		output_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_CharucoBoard_draw_Size_const__OutputArrayR_int_int(self.as_raw_mut_CharucoBoard(), &out_size, img.as_raw__OutputArray(), margin_size, border_bits, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [CharucoBoardTrait::draw] function uses the following default values for its arguments:
	/// * margin_size: 0
	/// * border_bits: 1
	// cv::aruco::CharucoBoard::draw(SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:83
	// ("cv::aruco::CharucoBoard::draw", vec![(pred!(mut, ["outSize", "img"], ["cv::Size", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn draw_def(&mut self, out_size: core::Size, img: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_CharucoBoard_draw_Size_const__OutputArrayR(self.as_raw_mut_CharucoBoard(), &out_size, img.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// ChArUco board
/// Specific class for ChArUco boards. A ChArUco board is a planar board where the markers are placed
/// inside the white squares of a chessboard. The benefits of ChArUco boards is that they provide
/// both, ArUco markers versatility and chessboard corner precision, which is important for
/// calibration and pose estimation.
/// This class also allows the easy creation and drawing of ChArUco boards.
// CharucoBoard /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:62
pub struct CharucoBoard {
	ptr: *mut c_void,
}

opencv_type_boxed! { CharucoBoard }

impl Drop for CharucoBoard {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_aruco_CharucoBoard_delete(self.as_raw_mut_CharucoBoard()) };
	}
}

unsafe impl Send for CharucoBoard {}

impl crate::aruco::BoardTraitConst for CharucoBoard {
	#[inline] fn as_raw_Board(&self) -> *const c_void { self.as_raw() }
}

impl crate::aruco::BoardTrait for CharucoBoard {
	#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CharucoBoard, crate::aruco::BoardTraitConst, as_raw_Board, crate::aruco::BoardTrait, as_raw_mut_Board }

impl crate::aruco::CharucoBoardTraitConst for CharucoBoard {
	#[inline] fn as_raw_CharucoBoard(&self) -> *const c_void { self.as_raw() }
}

impl crate::aruco::CharucoBoardTrait for CharucoBoard {
	#[inline] fn as_raw_mut_CharucoBoard(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CharucoBoard, crate::aruco::CharucoBoardTraitConst, as_raw_CharucoBoard, crate::aruco::CharucoBoardTrait, as_raw_mut_CharucoBoard }

impl CharucoBoard {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_aruco_CharucoBoard_defaultNew_const()) }
	}

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
	// create(int, int, float, float, const Ptr<Dictionary> &)(Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/charuco.hpp:100
	// ("cv::aruco::CharucoBoard::create", vec![(pred!(mut, ["squaresX", "squaresY", "squareLength", "markerLength", "dictionary"], ["int", "int", "float", "float", "const cv::Ptr<cv::aruco::Dictionary>*"]), _)]),
	#[inline]
	pub fn create(squares_x: i32, squares_y: i32, square_length: f32, marker_length: f32, dictionary: &core::Ptr<crate::aruco::Dictionary>) -> Result<core::Ptr<crate::aruco::CharucoBoard>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_CharucoBoard_create_int_int_float_float_const_PtrLDictionaryGR(squares_x, squares_y, square_length, marker_length, dictionary.as_raw_PtrOfDictionary(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::aruco::CharucoBoard>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { CharucoBoard, crate::aruco::Board, cv_aruco_CharucoBoard_to_Board }

impl std::fmt::Debug for CharucoBoard {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("CharucoBoard")
			.field("chessboard_corners", &crate::aruco::CharucoBoardTraitConst::chessboard_corners(self))
			.field("nearest_marker_idx", &crate::aruco::CharucoBoardTraitConst::nearest_marker_idx(self))
			.field("nearest_marker_corners", &crate::aruco::CharucoBoardTraitConst::nearest_marker_corners(self))
			.field("obj_points", &crate::aruco::BoardTraitConst::obj_points(self))
			.field("ids", &crate::aruco::BoardTraitConst::ids(self))
			.field("right_bottom_border", &crate::aruco::BoardTraitConst::right_bottom_border(self))
			.finish()
	}
}

impl Default for CharucoBoard {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::aruco::DetectorParameters]
// DetectorParameters /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:151
pub trait DetectorParametersTraitConst {
	fn as_raw_DetectorParameters(&self) -> *const c_void;

	// cv::aruco::DetectorParameters::adaptiveThreshWinSizeMin() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:157
	// ("cv::aruco::DetectorParameters::adaptiveThreshWinSizeMin", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn adaptive_thresh_win_size_min(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMin_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::adaptiveThreshWinSizeMax() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:158
	// ("cv::aruco::DetectorParameters::adaptiveThreshWinSizeMax", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn adaptive_thresh_win_size_max(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMax_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::adaptiveThreshWinSizeStep() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:159
	// ("cv::aruco::DetectorParameters::adaptiveThreshWinSizeStep", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn adaptive_thresh_win_size_step(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeStep_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::adaptiveThreshConstant() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:160
	// ("cv::aruco::DetectorParameters::adaptiveThreshConstant", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn adaptive_thresh_constant(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propAdaptiveThreshConstant_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::minMarkerPerimeterRate() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:161
	// ("cv::aruco::DetectorParameters::minMarkerPerimeterRate", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn min_marker_perimeter_rate(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinMarkerPerimeterRate_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::maxMarkerPerimeterRate() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:162
	// ("cv::aruco::DetectorParameters::maxMarkerPerimeterRate", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn max_marker_perimeter_rate(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propMaxMarkerPerimeterRate_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::polygonalApproxAccuracyRate() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:163
	// ("cv::aruco::DetectorParameters::polygonalApproxAccuracyRate", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn polygonal_approx_accuracy_rate(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propPolygonalApproxAccuracyRate_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::minCornerDistanceRate() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:164
	// ("cv::aruco::DetectorParameters::minCornerDistanceRate", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn min_corner_distance_rate(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinCornerDistanceRate_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::minDistanceToBorder() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:165
	// ("cv::aruco::DetectorParameters::minDistanceToBorder", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn min_distance_to_border(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinDistanceToBorder_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::minMarkerDistanceRate() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:166
	// ("cv::aruco::DetectorParameters::minMarkerDistanceRate", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn min_marker_distance_rate(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinMarkerDistanceRate_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::cornerRefinementMethod() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:167
	// ("cv::aruco::DetectorParameters::cornerRefinementMethod", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn corner_refinement_method(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propCornerRefinementMethod_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::cornerRefinementWinSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:168
	// ("cv::aruco::DetectorParameters::cornerRefinementWinSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn corner_refinement_win_size(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propCornerRefinementWinSize_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::cornerRefinementMaxIterations() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:169
	// ("cv::aruco::DetectorParameters::cornerRefinementMaxIterations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn corner_refinement_max_iterations(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propCornerRefinementMaxIterations_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::cornerRefinementMinAccuracy() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:170
	// ("cv::aruco::DetectorParameters::cornerRefinementMinAccuracy", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn corner_refinement_min_accuracy(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propCornerRefinementMinAccuracy_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::markerBorderBits() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:171
	// ("cv::aruco::DetectorParameters::markerBorderBits", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn marker_border_bits(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propMarkerBorderBits_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::perspectiveRemovePixelPerCell() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:172
	// ("cv::aruco::DetectorParameters::perspectiveRemovePixelPerCell", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn perspective_remove_pixel_per_cell(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propPerspectiveRemovePixelPerCell_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::perspectiveRemoveIgnoredMarginPerCell() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:173
	// ("cv::aruco::DetectorParameters::perspectiveRemoveIgnoredMarginPerCell", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn perspective_remove_ignored_margin_per_cell(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propPerspectiveRemoveIgnoredMarginPerCell_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::maxErroneousBitsInBorderRate() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:174
	// ("cv::aruco::DetectorParameters::maxErroneousBitsInBorderRate", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn max_erroneous_bits_in_border_rate(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propMaxErroneousBitsInBorderRate_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::minOtsuStdDev() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:175
	// ("cv::aruco::DetectorParameters::minOtsuStdDev", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn min_otsu_std_dev(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinOtsuStdDev_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::errorCorrectionRate() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:176
	// ("cv::aruco::DetectorParameters::errorCorrectionRate", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn error_correction_rate(&self) -> f64 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propErrorCorrectionRate_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::aprilTagQuadDecimate() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:179
	// ("cv::aruco::DetectorParameters::aprilTagQuadDecimate", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn april_tag_quad_decimate(&self) -> f32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagQuadDecimate_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::aprilTagQuadSigma() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:180
	// ("cv::aruco::DetectorParameters::aprilTagQuadSigma", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn april_tag_quad_sigma(&self) -> f32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagQuadSigma_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::aprilTagMinClusterPixels() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:183
	// ("cv::aruco::DetectorParameters::aprilTagMinClusterPixels", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn april_tag_min_cluster_pixels(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagMinClusterPixels_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::aprilTagMaxNmaxima() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:184
	// ("cv::aruco::DetectorParameters::aprilTagMaxNmaxima", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn april_tag_max_nmaxima(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagMaxNmaxima_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::aprilTagCriticalRad() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:185
	// ("cv::aruco::DetectorParameters::aprilTagCriticalRad", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn april_tag_critical_rad(&self) -> f32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagCriticalRad_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::aprilTagMaxLineFitMse() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:186
	// ("cv::aruco::DetectorParameters::aprilTagMaxLineFitMse", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn april_tag_max_line_fit_mse(&self) -> f32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagMaxLineFitMse_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::aprilTagMinWhiteBlackDiff() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:187
	// ("cv::aruco::DetectorParameters::aprilTagMinWhiteBlackDiff", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn april_tag_min_white_black_diff(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagMinWhiteBlackDiff_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::aprilTagDeglitch() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:188
	// ("cv::aruco::DetectorParameters::aprilTagDeglitch", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn april_tag_deglitch(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagDeglitch_const(self.as_raw_DetectorParameters()) };
		ret
	}

	// cv::aruco::DetectorParameters::detectInvertedMarker() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:191
	// ("cv::aruco::DetectorParameters::detectInvertedMarker", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn detect_inverted_marker(&self) -> bool {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propDetectInvertedMarker_const(self.as_raw_DetectorParameters()) };
		ret
	}

}

/// Mutable methods for [crate::aruco::DetectorParameters]
pub trait DetectorParametersTrait: crate::aruco::DetectorParametersTraitConst {
	fn as_raw_mut_DetectorParameters(&mut self) -> *mut c_void;

	// cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeMin(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:157
	// ("cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeMin", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_adaptive_thresh_win_size_min(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMin_const_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeMax(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:158
	// ("cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeMax", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_adaptive_thresh_win_size_max(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMax_const_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeStep(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:159
	// ("cv::aruco::DetectorParameters::setAdaptiveThreshWinSizeStep", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_adaptive_thresh_win_size_step(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeStep_const_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setAdaptiveThreshConstant(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:160
	// ("cv::aruco::DetectorParameters::setAdaptiveThreshConstant", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_adaptive_thresh_constant(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propAdaptiveThreshConstant_const_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setMinMarkerPerimeterRate(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:161
	// ("cv::aruco::DetectorParameters::setMinMarkerPerimeterRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_min_marker_perimeter_rate(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinMarkerPerimeterRate_const_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setMaxMarkerPerimeterRate(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:162
	// ("cv::aruco::DetectorParameters::setMaxMarkerPerimeterRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_max_marker_perimeter_rate(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propMaxMarkerPerimeterRate_const_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setPolygonalApproxAccuracyRate(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:163
	// ("cv::aruco::DetectorParameters::setPolygonalApproxAccuracyRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_polygonal_approx_accuracy_rate(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propPolygonalApproxAccuracyRate_const_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setMinCornerDistanceRate(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:164
	// ("cv::aruco::DetectorParameters::setMinCornerDistanceRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_min_corner_distance_rate(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinCornerDistanceRate_const_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setMinDistanceToBorder(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:165
	// ("cv::aruco::DetectorParameters::setMinDistanceToBorder", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_min_distance_to_border(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinDistanceToBorder_const_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setMinMarkerDistanceRate(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:166
	// ("cv::aruco::DetectorParameters::setMinMarkerDistanceRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_min_marker_distance_rate(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinMarkerDistanceRate_const_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setCornerRefinementMethod(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:167
	// ("cv::aruco::DetectorParameters::setCornerRefinementMethod", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_corner_refinement_method(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propCornerRefinementMethod_const_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setCornerRefinementWinSize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:168
	// ("cv::aruco::DetectorParameters::setCornerRefinementWinSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_corner_refinement_win_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propCornerRefinementWinSize_const_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setCornerRefinementMaxIterations(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:169
	// ("cv::aruco::DetectorParameters::setCornerRefinementMaxIterations", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_corner_refinement_max_iterations(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propCornerRefinementMaxIterations_const_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setCornerRefinementMinAccuracy(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:170
	// ("cv::aruco::DetectorParameters::setCornerRefinementMinAccuracy", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_corner_refinement_min_accuracy(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propCornerRefinementMinAccuracy_const_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setMarkerBorderBits(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:171
	// ("cv::aruco::DetectorParameters::setMarkerBorderBits", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_marker_border_bits(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propMarkerBorderBits_const_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setPerspectiveRemovePixelPerCell(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:172
	// ("cv::aruco::DetectorParameters::setPerspectiveRemovePixelPerCell", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_perspective_remove_pixel_per_cell(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propPerspectiveRemovePixelPerCell_const_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setPerspectiveRemoveIgnoredMarginPerCell(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:173
	// ("cv::aruco::DetectorParameters::setPerspectiveRemoveIgnoredMarginPerCell", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_perspective_remove_ignored_margin_per_cell(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propPerspectiveRemoveIgnoredMarginPerCell_const_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setMaxErroneousBitsInBorderRate(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:174
	// ("cv::aruco::DetectorParameters::setMaxErroneousBitsInBorderRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_max_erroneous_bits_in_border_rate(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propMaxErroneousBitsInBorderRate_const_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setMinOtsuStdDev(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:175
	// ("cv::aruco::DetectorParameters::setMinOtsuStdDev", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_min_otsu_std_dev(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinOtsuStdDev_const_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setErrorCorrectionRate(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:176
	// ("cv::aruco::DetectorParameters::setErrorCorrectionRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_error_correction_rate(&mut self, val: f64) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propErrorCorrectionRate_const_double(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setAprilTagQuadDecimate(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:179
	// ("cv::aruco::DetectorParameters::setAprilTagQuadDecimate", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_april_tag_quad_decimate(&mut self, val: f32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagQuadDecimate_const_float(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setAprilTagQuadSigma(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:180
	// ("cv::aruco::DetectorParameters::setAprilTagQuadSigma", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_april_tag_quad_sigma(&mut self, val: f32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagQuadSigma_const_float(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setAprilTagMinClusterPixels(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:183
	// ("cv::aruco::DetectorParameters::setAprilTagMinClusterPixels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_april_tag_min_cluster_pixels(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagMinClusterPixels_const_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setAprilTagMaxNmaxima(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:184
	// ("cv::aruco::DetectorParameters::setAprilTagMaxNmaxima", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_april_tag_max_nmaxima(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagMaxNmaxima_const_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setAprilTagCriticalRad(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:185
	// ("cv::aruco::DetectorParameters::setAprilTagCriticalRad", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_april_tag_critical_rad(&mut self, val: f32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagCriticalRad_const_float(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setAprilTagMaxLineFitMse(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:186
	// ("cv::aruco::DetectorParameters::setAprilTagMaxLineFitMse", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_april_tag_max_line_fit_mse(&mut self, val: f32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagMaxLineFitMse_const_float(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setAprilTagMinWhiteBlackDiff(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:187
	// ("cv::aruco::DetectorParameters::setAprilTagMinWhiteBlackDiff", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_april_tag_min_white_black_diff(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagMinWhiteBlackDiff_const_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setAprilTagDeglitch(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:188
	// ("cv::aruco::DetectorParameters::setAprilTagDeglitch", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_april_tag_deglitch(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagDeglitch_const_int(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// cv::aruco::DetectorParameters::setDetectInvertedMarker(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:191
	// ("cv::aruco::DetectorParameters::setDetectInvertedMarker", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_detect_inverted_marker(&mut self, val: bool) {
		let ret = unsafe { sys::cv_aruco_DetectorParameters_propDetectInvertedMarker_const_bool(self.as_raw_mut_DetectorParameters(), val) };
		ret
	}

	// readDetectorParameters(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:155
	// ("cv::aruco::DetectorParameters::readDetectorParameters", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	#[inline]
	fn read_detector_parameters(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_DetectorParameters_readDetectorParameters_const_FileNodeR(self.as_raw_mut_DetectorParameters(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
///   determine which contours are squares. (default 0.03)
/// - minCornerDistanceRate: minimum distance between corners for detected markers relative to its
///   perimeter (default 0.05)
/// - minDistanceToBorder: minimum distance of any corner to the image border for detected markers
///   (in pixels) (default 3)
/// - minMarkerDistanceRate: minimum mean distance beetween two marker corners to be considered
///   similar, so that the smaller one is removed. The rate is relative to the smaller perimeter
///   of the two markers (default 0.05).
/// - cornerRefinementMethod: corner refinement method. (CORNER_REFINE_NONE, no refinement.
///   CORNER_REFINE_SUBPIX, do subpixel refinement. CORNER_REFINE_CONTOUR use contour-Points,
///   CORNER_REFINE_APRILTAG  use the AprilTag2 approach). (default CORNER_REFINE_NONE)
/// - cornerRefinementWinSize: window size for the corner refinement process (in pixels) (default 5).
/// - cornerRefinementMaxIterations: maximum number of iterations for stop criteria of the corner
///   refinement process (default 30).
/// - cornerRefinementMinAccuracy: minimum error for the stop cristeria of the corner refinement
///   process (default: 0.1)
/// - markerBorderBits: number of bits of the marker border, i.e. marker border width (default 1).
/// - perspectiveRemovePixelPerCell: number of bits (per dimension) for each cell of the marker
///   when removing the perspective (default 4).
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
/// - aprilTagMinClusterPixels: reject quads containing too few pixels. (default 5)
/// - aprilTagMaxNmaxima: how many corner candidates to consider when segmenting a group of pixels into a quad. (default 10)
/// - aprilTagCriticalRad: Reject quads where pairs of edges have angles that are close to straight or close to
///   180 degrees. Zero means that no quads are rejected. (In radians) (default 10*PI/180)
/// - aprilTagMaxLineFitMse:  When fitting lines to the contours, what is the maximum mean squared error
///   allowed?  This is useful in rejecting contours that are far from being quad shaped; rejecting
///   these quads "early" saves expensive decoding processing. (default 10.0)
/// - aprilTagMinWhiteBlackDiff: When we build our model of black & white pixels, we add an extra check that
///   the white model must be (overall) brighter than the black model.  How much brighter? (in pixel values, [0,255]). (default 5)
/// - aprilTagDeglitch:  should the thresholded image be deglitched? Only useful for very noisy images. (default 0)
/// - aprilTagQuadDecimate: Detection of quads can be done on a lower-resolution image, improving speed at a
///   cost of pose accuracy and a slight decrease in detection rate. Decoding the binary payload is still
///   done at full resolution. (default 0.0)
/// - aprilTagQuadSigma: What Gaussian blur should be applied to the segmented image (used for quad detection?)
///   Parameter is the standard deviation in pixels.  Very noisy images benefit from non-zero values (e.g. 0.8). (default 0.0)
/// - detectInvertedMarker: to check if there is a white marker. In order to generate a "white" marker just
///   invert a normal marker by using a tilde, ~markerImage. (default false)
// DetectorParameters /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:151
pub struct DetectorParameters {
	ptr: *mut c_void,
}

opencv_type_boxed! { DetectorParameters }

impl Drop for DetectorParameters {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_aruco_DetectorParameters_delete(self.as_raw_mut_DetectorParameters()) };
	}
}

unsafe impl Send for DetectorParameters {}

impl crate::aruco::DetectorParametersTraitConst for DetectorParameters {
	#[inline] fn as_raw_DetectorParameters(&self) -> *const c_void { self.as_raw() }
}

impl crate::aruco::DetectorParametersTrait for DetectorParameters {
	#[inline] fn as_raw_mut_DetectorParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DetectorParameters, crate::aruco::DetectorParametersTraitConst, as_raw_DetectorParameters, crate::aruco::DetectorParametersTrait, as_raw_mut_DetectorParameters }

impl DetectorParameters {
	// DetectorParameters()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:153
	// ("cv::aruco::DetectorParameters::DetectorParameters", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::aruco::DetectorParameters> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_DetectorParameters_DetectorParameters(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::aruco::DetectorParameters::opencv_from_extern(ret) };
		Ok(ret)
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:154
	// ("cv::aruco::DetectorParameters::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::aruco::DetectorParameters>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_DetectorParameters_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::aruco::DetectorParameters>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl Clone for DetectorParameters {
	#[inline]
	fn clone(&self) -> Self {
		unsafe { Self::from_raw(sys::cv_aruco_DetectorParameters_implicitClone_const(self.as_raw_DetectorParameters())) }
	}
}

impl std::fmt::Debug for DetectorParameters {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("DetectorParameters")
			.field("adaptive_thresh_win_size_min", &crate::aruco::DetectorParametersTraitConst::adaptive_thresh_win_size_min(self))
			.field("adaptive_thresh_win_size_max", &crate::aruco::DetectorParametersTraitConst::adaptive_thresh_win_size_max(self))
			.field("adaptive_thresh_win_size_step", &crate::aruco::DetectorParametersTraitConst::adaptive_thresh_win_size_step(self))
			.field("adaptive_thresh_constant", &crate::aruco::DetectorParametersTraitConst::adaptive_thresh_constant(self))
			.field("min_marker_perimeter_rate", &crate::aruco::DetectorParametersTraitConst::min_marker_perimeter_rate(self))
			.field("max_marker_perimeter_rate", &crate::aruco::DetectorParametersTraitConst::max_marker_perimeter_rate(self))
			.field("polygonal_approx_accuracy_rate", &crate::aruco::DetectorParametersTraitConst::polygonal_approx_accuracy_rate(self))
			.field("min_corner_distance_rate", &crate::aruco::DetectorParametersTraitConst::min_corner_distance_rate(self))
			.field("min_distance_to_border", &crate::aruco::DetectorParametersTraitConst::min_distance_to_border(self))
			.field("min_marker_distance_rate", &crate::aruco::DetectorParametersTraitConst::min_marker_distance_rate(self))
			.field("corner_refinement_method", &crate::aruco::DetectorParametersTraitConst::corner_refinement_method(self))
			.field("corner_refinement_win_size", &crate::aruco::DetectorParametersTraitConst::corner_refinement_win_size(self))
			.field("corner_refinement_max_iterations", &crate::aruco::DetectorParametersTraitConst::corner_refinement_max_iterations(self))
			.field("corner_refinement_min_accuracy", &crate::aruco::DetectorParametersTraitConst::corner_refinement_min_accuracy(self))
			.field("marker_border_bits", &crate::aruco::DetectorParametersTraitConst::marker_border_bits(self))
			.field("perspective_remove_pixel_per_cell", &crate::aruco::DetectorParametersTraitConst::perspective_remove_pixel_per_cell(self))
			.field("perspective_remove_ignored_margin_per_cell", &crate::aruco::DetectorParametersTraitConst::perspective_remove_ignored_margin_per_cell(self))
			.field("max_erroneous_bits_in_border_rate", &crate::aruco::DetectorParametersTraitConst::max_erroneous_bits_in_border_rate(self))
			.field("min_otsu_std_dev", &crate::aruco::DetectorParametersTraitConst::min_otsu_std_dev(self))
			.field("error_correction_rate", &crate::aruco::DetectorParametersTraitConst::error_correction_rate(self))
			.field("april_tag_quad_decimate", &crate::aruco::DetectorParametersTraitConst::april_tag_quad_decimate(self))
			.field("april_tag_quad_sigma", &crate::aruco::DetectorParametersTraitConst::april_tag_quad_sigma(self))
			.field("april_tag_min_cluster_pixels", &crate::aruco::DetectorParametersTraitConst::april_tag_min_cluster_pixels(self))
			.field("april_tag_max_nmaxima", &crate::aruco::DetectorParametersTraitConst::april_tag_max_nmaxima(self))
			.field("april_tag_critical_rad", &crate::aruco::DetectorParametersTraitConst::april_tag_critical_rad(self))
			.field("april_tag_max_line_fit_mse", &crate::aruco::DetectorParametersTraitConst::april_tag_max_line_fit_mse(self))
			.field("april_tag_min_white_black_diff", &crate::aruco::DetectorParametersTraitConst::april_tag_min_white_black_diff(self))
			.field("april_tag_deglitch", &crate::aruco::DetectorParametersTraitConst::april_tag_deglitch(self))
			.field("detect_inverted_marker", &crate::aruco::DetectorParametersTraitConst::detect_inverted_marker(self))
			.finish()
	}
}

/// Constant methods for [crate::aruco::Dictionary]
// Dictionary /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:61
pub trait DictionaryTraitConst {
	fn as_raw_Dictionary(&self) -> *const c_void;

	// cv::aruco::Dictionary::bytesList() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:64
	// ("cv::aruco::Dictionary::bytesList", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn bytes_list(&self) -> core::Mat {
		let ret = unsafe { sys::cv_aruco_Dictionary_propBytesList_const(self.as_raw_Dictionary()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

	// cv::aruco::Dictionary::markerSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:65
	// ("cv::aruco::Dictionary::markerSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn marker_size(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_Dictionary_propMarkerSize_const(self.as_raw_Dictionary()) };
		ret
	}

	// cv::aruco::Dictionary::maxCorrectionBits() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:66
	// ("cv::aruco::Dictionary::maxCorrectionBits", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn max_correction_bits(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_Dictionary_propMaxCorrectionBits_const(self.as_raw_Dictionary()) };
		ret
	}

	/// Given a matrix of bits. Returns whether if marker is identified or not.
	/// It returns by reference the correct id (if any) and the correct rotation
	// identify(const Mat &, int &, int &, double)(TraitClass, Indirect, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:120
	// ("cv::aruco::Dictionary::identify", vec![(pred!(const, ["onlyBits", "idx", "rotation", "maxCorrectionRate"], ["const cv::Mat*", "int*", "int*", "double"]), _)]),
	#[inline]
	fn identify(&self, only_bits: &impl core::MatTraitConst, idx: &mut i32, rotation: &mut i32, max_correction_rate: f64) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_identify_const_const_MatR_intR_intR_double(self.as_raw_Dictionary(), only_bits.as_raw_Mat(), idx, rotation, max_correction_rate, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the distance of the input bits to the specific id. If allRotations is true,
	/// the four posible bits rotation are considered
	///
	/// ## C++ default parameters
	/// * all_rotations: true
	// getDistanceToId(InputArray, int, bool)(InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:126
	// ("cv::aruco::Dictionary::getDistanceToId", vec![(pred!(const, ["bits", "id", "allRotations"], ["const cv::_InputArray*", "int", "bool"]), _)]),
	#[inline]
	fn get_distance_to_id(&self, bits: &impl ToInputArray, id: i32, all_rotations: bool) -> Result<i32> {
		input_array_arg!(bits);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_getDistanceToId_const_const__InputArrayR_int_bool(self.as_raw_Dictionary(), bits.as_raw__InputArray(), id, all_rotations, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns the distance of the input bits to the specific id. If allRotations is true,
	/// the four posible bits rotation are considered
	///
	/// ## Note
	/// This alternative version of [DictionaryTraitConst::get_distance_to_id] function uses the following default values for its arguments:
	/// * all_rotations: true
	// cv::aruco::Dictionary::getDistanceToId(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:126
	// ("cv::aruco::Dictionary::getDistanceToId", vec![(pred!(const, ["bits", "id"], ["const cv::_InputArray*", "int"]), _)]),
	#[inline]
	fn get_distance_to_id_def(&self, bits: &impl ToInputArray, id: i32) -> Result<i32> {
		input_array_arg!(bits);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_getDistanceToId_const_const__InputArrayR_int(self.as_raw_Dictionary(), bits.as_raw__InputArray(), id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Draw a canonical marker image
	///
	/// ## C++ default parameters
	/// * border_bits: 1
	// drawMarker(int, int, OutputArray, int)(Primitive, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:132
	// ("cv::aruco::Dictionary::drawMarker", vec![(pred!(const, ["id", "sidePixels", "_img", "borderBits"], ["int", "int", "const cv::_OutputArray*", "int"]), _)]),
	#[inline]
	fn draw_marker(&self, id: i32, side_pixels: i32, _img: &mut impl ToOutputArray, border_bits: i32) -> Result<()> {
		output_array_arg!(_img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_drawMarker_const_int_int_const__OutputArrayR_int(self.as_raw_Dictionary(), id, side_pixels, _img.as_raw__OutputArray(), border_bits, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Draw a canonical marker image
	///
	/// ## Note
	/// This alternative version of [DictionaryTraitConst::draw_marker] function uses the following default values for its arguments:
	/// * border_bits: 1
	// cv::aruco::Dictionary::drawMarker(Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:132
	// ("cv::aruco::Dictionary::drawMarker", vec![(pred!(const, ["id", "sidePixels", "_img"], ["int", "int", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn draw_marker_def(&self, id: i32, side_pixels: i32, _img: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(_img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_drawMarker_const_int_int_const__OutputArrayR(self.as_raw_Dictionary(), id, side_pixels, _img.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::aruco::Dictionary]
pub trait DictionaryTrait: crate::aruco::DictionaryTraitConst {
	fn as_raw_mut_Dictionary(&mut self) -> *mut c_void;

	// cv::aruco::Dictionary::setBytesList(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:64
	// ("cv::aruco::Dictionary::setBytesList", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_bytes_list(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_aruco_Dictionary_propBytesList_const_Mat(self.as_raw_mut_Dictionary(), val.as_raw_Mat()) };
		ret
	}

	// cv::aruco::Dictionary::setMarkerSize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:65
	// ("cv::aruco::Dictionary::setMarkerSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_marker_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_Dictionary_propMarkerSize_const_int(self.as_raw_mut_Dictionary(), val) };
		ret
	}

	// cv::aruco::Dictionary::setMaxCorrectionBits(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:66
	// ("cv::aruco::Dictionary::setMaxCorrectionBits", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_max_correction_bits(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_Dictionary_propMaxCorrectionBits_const_int(self.as_raw_mut_Dictionary(), val) };
		ret
	}

	/// Read a new dictionary from FileNode. Format:
	///
	/// nmarkers: 35
	///
	/// markersize: 6
	///
	/// maxCorrectionBits: 5
	///
	/// marker_0: "101011111011111001001001101100000000"
	///
	/// ...
	///
	/// marker_34: "011111010000111011111110110101100101"
	// readDictionary(const cv::FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:105
	// ("cv::aruco::Dictionary::readDictionary", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	#[inline]
	fn read_dictionary(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_readDictionary_const_FileNodeR(self.as_raw_mut_Dictionary(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Write a dictionary to FileStorage. Format is the same as in readDictionary().
	// writeDictionary(Ptr<FileStorage> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:110
	// ("cv::aruco::Dictionary::writeDictionary", vec![(pred!(mut, ["fs"], ["cv::Ptr<cv::FileStorage>*"]), _)]),
	#[inline]
	fn write_dictionary(&mut self, fs: &mut core::Ptr<core::FileStorage>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_writeDictionary_PtrLFileStorageGR(self.as_raw_mut_Dictionary(), fs.as_raw_mut_PtrOfFileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
// Dictionary /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:61
pub struct Dictionary {
	ptr: *mut c_void,
}

opencv_type_boxed! { Dictionary }

impl Drop for Dictionary {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_aruco_Dictionary_delete(self.as_raw_mut_Dictionary()) };
	}
}

unsafe impl Send for Dictionary {}

impl crate::aruco::DictionaryTraitConst for Dictionary {
	#[inline] fn as_raw_Dictionary(&self) -> *const c_void { self.as_raw() }
}

impl crate::aruco::DictionaryTrait for Dictionary {
	#[inline] fn as_raw_mut_Dictionary(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Dictionary, crate::aruco::DictionaryTraitConst, as_raw_Dictionary, crate::aruco::DictionaryTrait, as_raw_mut_Dictionary }

impl Dictionary {
	/// ## C++ default parameters
	/// * _bytes_list: Mat()
	/// * _marker_size: 0
	/// * _maxcorr: 0
	// Dictionary(const Mat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:71
	// ("cv::aruco::Dictionary::Dictionary", vec![(pred!(mut, ["_bytesList", "_markerSize", "_maxcorr"], ["const cv::Mat*", "int", "int"]), _)]),
	#[inline]
	pub fn new(_bytes_list: &impl core::MatTraitConst, _marker_size: i32, _maxcorr: i32) -> Result<crate::aruco::Dictionary> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_Dictionary_const_MatR_int_int(_bytes_list.as_raw_Mat(), _marker_size, _maxcorr, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::aruco::Dictionary::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * _bytes_list: Mat()
	/// * _marker_size: 0
	/// * _maxcorr: 0
	// cv::aruco::Dictionary::Dictionary() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:71
	// ("cv::aruco::Dictionary::Dictionary", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::aruco::Dictionary> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_Dictionary(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::aruco::Dictionary::opencv_from_extern(ret) };
		Ok(ret)
	}

	// Dictionary(const Ptr<Dictionary> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:81
	// ("cv::aruco::Dictionary::Dictionary", vec![(pred!(mut, ["_dictionary"], ["const cv::Ptr<cv::aruco::Dictionary>*"]), _)]),
	#[inline]
	pub fn copy(_dictionary: &core::Ptr<crate::aruco::Dictionary>) -> Result<crate::aruco::Dictionary> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_Dictionary_const_PtrLDictionaryGR(_dictionary.as_raw_PtrOfDictionary(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::aruco::Dictionary::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## See also
	/// generateCustomDictionary
	///
	/// ## C++ default parameters
	/// * random_seed: 0
	// create(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:87
	// ("cv::aruco::Dictionary::create", vec![(pred!(mut, ["nMarkers", "markerSize", "randomSeed"], ["int", "int", "int"]), _)]),
	#[inline]
	pub fn create(n_markers: i32, marker_size: i32, random_seed: i32) -> Result<core::Ptr<crate::aruco::Dictionary>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_create_int_int_int(n_markers, marker_size, random_seed, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## See also
	/// generateCustomDictionary
	///
	/// ## Note
	/// This alternative version of [Dictionary::create] function uses the following default values for its arguments:
	/// * random_seed: 0
	// cv::aruco::Dictionary::create(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:87
	// ("cv::aruco::Dictionary::create", vec![(pred!(mut, ["nMarkers", "markerSize"], ["int", "int"]), _)]),
	#[inline]
	pub fn create_def(n_markers: i32, marker_size: i32) -> Result<core::Ptr<crate::aruco::Dictionary>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_create_int_int(n_markers, marker_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## See also
	/// generateCustomDictionary
	///
	/// ## C++ default parameters
	/// * random_seed: 0
	// create(int, int, const Ptr<Dictionary> &, int)(Primitive, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:93
	// ("cv::aruco::Dictionary::create", vec![(pred!(mut, ["nMarkers", "markerSize", "baseDictionary", "randomSeed"], ["int", "int", "const cv::Ptr<cv::aruco::Dictionary>*", "int"]), _)]),
	#[inline]
	pub fn create_from(n_markers: i32, marker_size: i32, base_dictionary: &core::Ptr<crate::aruco::Dictionary>, random_seed: i32) -> Result<core::Ptr<crate::aruco::Dictionary>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_create_int_int_const_PtrLDictionaryGR_int(n_markers, marker_size, base_dictionary.as_raw_PtrOfDictionary(), random_seed, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## See also
	/// generateCustomDictionary
	///
	/// ## Note
	/// This alternative version of [Dictionary::create_from] function uses the following default values for its arguments:
	/// * random_seed: 0
	// cv::aruco::Dictionary::create(Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:93
	// ("cv::aruco::Dictionary::create", vec![(pred!(mut, ["nMarkers", "markerSize", "baseDictionary"], ["int", "int", "const cv::Ptr<cv::aruco::Dictionary>*"]), _)]),
	#[inline]
	pub fn create_from_def(n_markers: i32, marker_size: i32, base_dictionary: &core::Ptr<crate::aruco::Dictionary>) -> Result<core::Ptr<crate::aruco::Dictionary>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_create_int_int_const_PtrLDictionaryGR(n_markers, marker_size, base_dictionary.as_raw_PtrOfDictionary(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## See also
	/// getPredefinedDictionary
	// get(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:114
	// ("cv::aruco::Dictionary::get", vec![(pred!(mut, ["dict"], ["int"]), _)]),
	#[inline]
	pub fn get(dict: i32) -> Result<core::Ptr<crate::aruco::Dictionary>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_get_int(dict, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::aruco::Dictionary>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Transform matrix of bits to list of bytes in the 4 rotations
	// getByteListFromBits(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:138
	// ("cv::aruco::Dictionary::getByteListFromBits", vec![(pred!(mut, ["bits"], ["const cv::Mat*"]), _)]),
	#[inline]
	pub fn get_byte_list_from_bits(bits: &impl core::MatTraitConst) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_getByteListFromBits_const_MatR(bits.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Transform list of bytes to matrix of bits
	// getBitsFromByteList(const Mat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco/dictionary.hpp:144
	// ("cv::aruco::Dictionary::getBitsFromByteList", vec![(pred!(mut, ["byteList", "markerSize"], ["const cv::Mat*", "int"]), _)]),
	#[inline]
	pub fn get_bits_from_byte_list(byte_list: &impl core::MatTraitConst, marker_size: i32) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_Dictionary_getBitsFromByteList_const_MatR_int(byte_list.as_raw_Mat(), marker_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for Dictionary {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Dictionary")
			.field("bytes_list", &crate::aruco::DictionaryTraitConst::bytes_list(self))
			.field("marker_size", &crate::aruco::DictionaryTraitConst::marker_size(self))
			.field("max_correction_bits", &crate::aruco::DictionaryTraitConst::max_correction_bits(self))
			.finish()
	}
}

/// Constant methods for [crate::aruco::EstimateParameters]
// EstimateParameters /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:260
pub trait EstimateParametersTraitConst {
	fn as_raw_EstimateParameters(&self) -> *const c_void;

	// cv::aruco::EstimateParameters::pattern() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:261
	// ("cv::aruco::EstimateParameters::pattern", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pattern(&self) -> crate::aruco::PatternPos {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_EstimateParameters_propPattern_const(self.as_raw_EstimateParameters(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	// cv::aruco::EstimateParameters::useExtrinsicGuess() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:262
	// ("cv::aruco::EstimateParameters::useExtrinsicGuess", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn use_extrinsic_guess(&self) -> bool {
		let ret = unsafe { sys::cv_aruco_EstimateParameters_propUseExtrinsicGuess_const(self.as_raw_EstimateParameters()) };
		ret
	}

	// cv::aruco::EstimateParameters::solvePnPMethod() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:263
	// ("cv::aruco::EstimateParameters::solvePnPMethod", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn solve_pnp_method(&self) -> crate::calib3d::SolvePnPMethod {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_EstimateParameters_propSolvePnPMethod_const(self.as_raw_EstimateParameters(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

}

/// Mutable methods for [crate::aruco::EstimateParameters]
pub trait EstimateParametersTrait: crate::aruco::EstimateParametersTraitConst {
	fn as_raw_mut_EstimateParameters(&mut self) -> *mut c_void;

	// cv::aruco::EstimateParameters::setPattern(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:261
	// ("cv::aruco::EstimateParameters::setPattern", vec![(pred!(mut, ["val"], ["const cv::aruco::PatternPos"]), _)]),
	#[inline]
	fn set_pattern(&mut self, val: crate::aruco::PatternPos) {
		let ret = unsafe { sys::cv_aruco_EstimateParameters_propPattern_const_PatternPos(self.as_raw_mut_EstimateParameters(), val) };
		ret
	}

	// cv::aruco::EstimateParameters::setUseExtrinsicGuess(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:262
	// ("cv::aruco::EstimateParameters::setUseExtrinsicGuess", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_use_extrinsic_guess(&mut self, val: bool) {
		let ret = unsafe { sys::cv_aruco_EstimateParameters_propUseExtrinsicGuess_const_bool(self.as_raw_mut_EstimateParameters(), val) };
		ret
	}

	// cv::aruco::EstimateParameters::setSolvePnPMethod(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:263
	// ("cv::aruco::EstimateParameters::setSolvePnPMethod", vec![(pred!(mut, ["val"], ["const cv::SolvePnPMethod"]), _)]),
	#[inline]
	fn set_solve_pnp_method(&mut self, val: crate::calib3d::SolvePnPMethod) {
		let ret = unsafe { sys::cv_aruco_EstimateParameters_propSolvePnPMethod_const_SolvePnPMethod(self.as_raw_mut_EstimateParameters(), val) };
		ret
	}

}

///
/// Pose estimation parameters
/// ## Parameters
/// * pattern: Defines center this system and axes direction (default PatternPos::CCW_center).
/// * useExtrinsicGuess: Parameter used for SOLVEPNP_ITERATIVE. If true (1), the function uses the provided
/// rvec and tvec values as initial approximations of the rotation and translation vectors, respectively, and further
/// optimizes them (default false).
/// * solvePnPMethod: Method for solving a PnP problem: see [calib3d_solvePnP_flags] (default SOLVEPNP_ITERATIVE).
/// ## See also
/// PatternPos, solvePnP(), [tutorial_aruco_detection]
// EstimateParameters /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:260
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
	// EstimateParameters()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:265
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

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:268
	// ("cv::aruco::EstimateParameters::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::aruco::EstimateParameters>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_EstimateParameters_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::aruco::EstimateParameters>::opencv_from_extern(ret) };
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

/// Constant methods for [crate::aruco::GridBoard]
// GridBoard /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:380
pub trait GridBoardTraitConst: crate::aruco::BoardTraitConst {
	fn as_raw_GridBoard(&self) -> *const c_void;

	// getGridSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:417
	// ("cv::aruco::GridBoard::getGridSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_grid_size(&self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_GridBoard_getGridSize_const(self.as_raw_GridBoard(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMarkerLength()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:422
	// ("cv::aruco::GridBoard::getMarkerLength", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_marker_length(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_GridBoard_getMarkerLength_const(self.as_raw_GridBoard(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMarkerSeparation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:427
	// ("cv::aruco::GridBoard::getMarkerSeparation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_marker_separation(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_GridBoard_getMarkerSeparation_const(self.as_raw_GridBoard(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::aruco::GridBoard]
pub trait GridBoardTrait: crate::aruco::BoardTrait + crate::aruco::GridBoardTraitConst {
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
	// draw(Size, OutputArray, int, int)(SimpleClass, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:394
	// ("cv::aruco::GridBoard::draw", vec![(pred!(mut, ["outSize", "img", "marginSize", "borderBits"], ["cv::Size", "const cv::_OutputArray*", "int", "int"]), _)]),
	#[inline]
	fn draw(&mut self, out_size: core::Size, img: &mut impl ToOutputArray, margin_size: i32, border_bits: i32) -> Result<()> {
		output_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_GridBoard_draw_Size_const__OutputArrayR_int_int(self.as_raw_mut_GridBoard(), &out_size, img.as_raw__OutputArray(), margin_size, border_bits, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

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
	/// ## Note
	/// This alternative version of [GridBoardTrait::draw] function uses the following default values for its arguments:
	/// * margin_size: 0
	/// * border_bits: 1
	// cv::aruco::GridBoard::draw(SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:394
	// ("cv::aruco::GridBoard::draw", vec![(pred!(mut, ["outSize", "img"], ["cv::Size", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn draw_def(&mut self, out_size: core::Size, img: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_GridBoard_draw_Size_const__OutputArrayR(self.as_raw_mut_GridBoard(), &out_size, img.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Planar board with grid arrangement of markers
/// More common type of board. All markers are placed in the same plane in a grid arrangement.
/// The board can be drawn using drawPlanarBoard() function (see also: drawPlanarBoard)
// GridBoard /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:380
pub struct GridBoard {
	ptr: *mut c_void,
}

opencv_type_boxed! { GridBoard }

impl Drop for GridBoard {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_aruco_GridBoard_delete(self.as_raw_mut_GridBoard()) };
	}
}

unsafe impl Send for GridBoard {}

impl crate::aruco::BoardTraitConst for GridBoard {
	#[inline] fn as_raw_Board(&self) -> *const c_void { self.as_raw() }
}

impl crate::aruco::BoardTrait for GridBoard {
	#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { GridBoard, crate::aruco::BoardTraitConst, as_raw_Board, crate::aruco::BoardTrait, as_raw_mut_Board }

impl crate::aruco::GridBoardTraitConst for GridBoard {
	#[inline] fn as_raw_GridBoard(&self) -> *const c_void { self.as_raw() }
}

impl crate::aruco::GridBoardTrait for GridBoard {
	#[inline] fn as_raw_mut_GridBoard(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { GridBoard, crate::aruco::GridBoardTraitConst, as_raw_GridBoard, crate::aruco::GridBoardTrait, as_raw_mut_GridBoard }

impl GridBoard {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_aruco_GridBoard_defaultNew_const()) }
	}

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
	// create(int, int, float, float, const Ptr<Dictionary> &, int)(Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:411
	// ("cv::aruco::GridBoard::create", vec![(pred!(mut, ["markersX", "markersY", "markerLength", "markerSeparation", "dictionary", "firstMarker"], ["int", "int", "float", "float", "const cv::Ptr<cv::aruco::Dictionary>*", "int"]), _)]),
	#[inline]
	pub fn create(markers_x: i32, markers_y: i32, marker_length: f32, marker_separation: f32, dictionary: &core::Ptr<crate::aruco::Dictionary>, first_marker: i32) -> Result<core::Ptr<crate::aruco::GridBoard>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_GridBoard_create_int_int_float_float_const_PtrLDictionaryGR_int(markers_x, markers_y, marker_length, marker_separation, dictionary.as_raw_PtrOfDictionary(), first_marker, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::aruco::GridBoard>::opencv_from_extern(ret) };
		Ok(ret)
	}

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
	/// ## Note
	/// This alternative version of [GridBoard::create] function uses the following default values for its arguments:
	/// * first_marker: 0
	// cv::aruco::GridBoard::create(Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/aruco.hpp:411
	// ("cv::aruco::GridBoard::create", vec![(pred!(mut, ["markersX", "markersY", "markerLength", "markerSeparation", "dictionary"], ["int", "int", "float", "float", "const cv::Ptr<cv::aruco::Dictionary>*"]), _)]),
	#[inline]
	pub fn create_def(markers_x: i32, markers_y: i32, marker_length: f32, marker_separation: f32, dictionary: &core::Ptr<crate::aruco::Dictionary>) -> Result<core::Ptr<crate::aruco::GridBoard>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_GridBoard_create_int_int_float_float_const_PtrLDictionaryGR(markers_x, markers_y, marker_length, marker_separation, dictionary.as_raw_PtrOfDictionary(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::aruco::GridBoard>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { GridBoard, crate::aruco::Board, cv_aruco_GridBoard_to_Board }

impl std::fmt::Debug for GridBoard {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("GridBoard")
			.field("obj_points", &crate::aruco::BoardTraitConst::obj_points(self))
			.field("ids", &crate::aruco::BoardTraitConst::ids(self))
			.field("right_bottom_border", &crate::aruco::BoardTraitConst::right_bottom_border(self))
			.finish()
	}
}

impl Default for GridBoard {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}
