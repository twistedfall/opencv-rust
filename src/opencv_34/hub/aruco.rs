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
//! The implementation is based on the ArUco Library by R. Muñoz-Salinas and S. Garrido-Jurado [Aruco2014](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Aruco2014).
//!
//! Markers can also be detected based on the AprilTag 2 [wang2016iros](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_wang2016iros) fiducial detection method.
//!
//! ## See also
//! S. Garrido-Jurado, R. Muñoz-Salinas, F. J. Madrid-Cuevas, and M. J. Marín-Jiménez. 2014.
//! "Automatic generation and detection of highly reliable fiducial markers under occlusion".
//! Pattern Recogn. 47, 6 (June 2014), 2280-2292. DOI=10.1016/j.patcog.2014.01.005
//!
//!  http://www.uco.es/investiga/grupos/ava/node/26
//!
//! This module has been originally developed by Sergio Garrido-Jurado as a project
//! for Google Summer of Code 2015 (GSoC 15).
use crate::{mod_prelude::*, core, sys, types};
use crate::core::{_InputArrayTrait, _OutputArrayTrait};

pub const CORNER_REFINE_APRILTAG: i32 = 3;
pub const CORNER_REFINE_CONTOUR: i32 = 2;
pub const CORNER_REFINE_NONE: i32 = 0;
pub const CORNER_REFINE_SUBPIX: i32 = 1;
pub const DICT_4X4_100: i32 = 0+1;
pub const DICT_4X4_1000: i32 = 0+3;
pub const DICT_4X4_250: i32 = 0+2;
pub const DICT_4X4_50: i32 = 0;
pub const DICT_5X5_100: i32 = 0+5;
pub const DICT_5X5_1000: i32 = 0+7;
pub const DICT_5X5_250: i32 = 0+6;
pub const DICT_5X5_50: i32 = 0+4;
pub const DICT_6X6_100: i32 = 0+9;
pub const DICT_6X6_1000: i32 = 0+11;
pub const DICT_6X6_250: i32 = 0+10;
pub const DICT_6X6_50: i32 = 0+8;
pub const DICT_7X7_100: i32 = 0+13;
pub const DICT_7X7_1000: i32 = 0+15;
pub const DICT_7X7_250: i32 = 0+14;
pub const DICT_7X7_50: i32 = 0+12;
pub const DICT_APRILTAG_16h5: i32 = 0+17;
pub const DICT_APRILTAG_25h9: i32 = 0+18;
pub const DICT_APRILTAG_36h10: i32 = 0+19;
pub const DICT_APRILTAG_36h11: i32 = 0+20;
pub const DICT_ARUCO_ORIGINAL: i32 = 0+16;

/// Predefined markers dictionaries/sets
/// Each dictionary indicates the number of bits and the number of markers contained
/// - DICT_ARUCO_ORIGINAL: standard ArUco Library Markers. 1024 markers, 5x5 bits, 0 minimum
/// distance
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PREDEFINED_DICTIONARY_NAME {
    DICT_4X4_50 = DICT_4X4_50 as isize,
    DICT_4X4_100 = DICT_4X4_100 as isize,
    DICT_4X4_250 = DICT_4X4_250 as isize,
    DICT_4X4_1000 = DICT_4X4_1000 as isize,
    DICT_5X5_50 = DICT_5X5_50 as isize,
    DICT_5X5_100 = DICT_5X5_100 as isize,
    DICT_5X5_250 = DICT_5X5_250 as isize,
    DICT_5X5_1000 = DICT_5X5_1000 as isize,
    DICT_6X6_50 = DICT_6X6_50 as isize,
    DICT_6X6_100 = DICT_6X6_100 as isize,
    DICT_6X6_250 = DICT_6X6_250 as isize,
    DICT_6X6_1000 = DICT_6X6_1000 as isize,
    DICT_7X7_50 = DICT_7X7_50 as isize,
    DICT_7X7_100 = DICT_7X7_100 as isize,
    DICT_7X7_250 = DICT_7X7_250 as isize,
    DICT_7X7_1000 = DICT_7X7_1000 as isize,
    DICT_ARUCO_ORIGINAL = DICT_ARUCO_ORIGINAL as isize,
    DICT_APRILTAG_16h5 = DICT_APRILTAG_16h5 as isize,
    DICT_APRILTAG_25h9 = DICT_APRILTAG_25h9 as isize,
    DICT_APRILTAG_36h10 = DICT_APRILTAG_36h10 as isize,
    DICT_APRILTAG_36h11 = DICT_APRILTAG_36h11 as isize,
}

/// Implementation of drawPlanarBoard that accepts a raw Board pointer.
///
/// ## C++ default parameters
/// * margin_size: 0
/// * border_bits: 1
pub fn _draw_planar_board_impl(board: &mut dyn crate::aruco::BoardTrait, out_size: core::Size, img: &mut dyn core::ToOutputArray, margin_size: i32, border_bits: i32) -> Result<()> {
    output_array_arg!(img);
    unsafe { sys::cv_aruco__drawPlanarBoardImpl_Board_Size__OutputArray_int_int(board.as_raw_Board(), out_size, img.as_raw__OutputArray(), margin_size, border_bits) }.into_result()
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
/// ![inline formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f_x%20%26%200%20%26%20c_x%5C%5C%200%20%26%20f_y%20%26%20c_y%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) . If CV\_CALIB\_USE\_INTRINSIC\_GUESS
/// and/or CV_CALIB_FIX_ASPECT_RATIO are specified, some or all of fx, fy, cx, cy must be
/// initialized before calling the function.
/// * distCoeffs: Output vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%28k_1%2C%20k_2%2C%20p_1%2C%20p_2%5B%2C%20k_3%5B%2C%20k_4%2C%20k_5%2C%20k_6%5D%2C%5Bs_1%2C%20s_2%2C%20s_3%2C%20s_4%5D%5D%29) of 4, 5, 8 or 12 elements
/// * rvecs: Output vector of rotation vectors (see Rodrigues ) estimated for each board view
/// (e.g. std::vector<cv::Mat>>). That is, each k-th rotation vector together with the corresponding
/// k-th translation vector (see the next output parameter description) brings the board pattern
/// from the model coordinate space (in which object points are specified) to the world coordinate
/// space, that is, a real position of the board pattern in the k-th pattern view (k=0.. *M* -1).
/// * tvecs: Output vector of translation vectors estimated for each pattern view.
/// * stdDeviationsIntrinsics: Output vector of standard deviations estimated for intrinsic parameters.
/// Order of deviations values:
/// ![inline formula](https://latex.codecogs.com/png.latex?%28f_x%2C%20f_y%2C%20c_x%2C%20c_y%2C%20k_1%2C%20k_2%2C%20p_1%2C%20p_2%2C%20k_3%2C%20k_4%2C%20k_5%2C%20k_6%20%2C%20s_1%2C%20s_2%2C%20s_3%2C%0As_4%2C%20%5Ctau_x%2C%20%5Ctau_y%29) If one of parameters is not estimated, it's deviation is equals to zero.
/// * stdDeviationsExtrinsics: Output vector of standard deviations estimated for extrinsic parameters.
/// Order of deviations values: ![inline formula](https://latex.codecogs.com/png.latex?%28R_1%2C%20T_1%2C%20%5Cdotsc%20%2C%20R_M%2C%20T_M%29) where M is number of pattern views,
/// ![inline formula](https://latex.codecogs.com/png.latex?R_i%2C%20T_i) are concatenated 1x3 vectors.
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
/// * criteria: TermCriteria(TermCriteria::COUNT + TermCriteria::EPS, 30, DBL_EPSILON)
pub fn calibrate_camera_aruco_with_stddev(corners: &dyn core::ToInputArray, ids: &dyn core::ToInputArray, counter: &dyn core::ToInputArray, board: &types::PtrOfBoard, image_size: core::Size, camera_matrix: &mut dyn core::ToInputOutputArray, dist_coeffs: &mut dyn core::ToInputOutputArray, rvecs: &mut dyn core::ToOutputArray, tvecs: &mut dyn core::ToOutputArray, std_deviations_intrinsics: &mut dyn core::ToOutputArray, std_deviations_extrinsics: &mut dyn core::ToOutputArray, per_view_errors: &mut dyn core::ToOutputArray, flags: i32, criteria: &core::TermCriteria) -> Result<f64> {
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
    unsafe { sys::cv_aruco_calibrateCameraAruco__InputArray__InputArray__InputArray_PtrOfBoard_Size__InputOutputArray__InputOutputArray__OutputArray__OutputArray__OutputArray__OutputArray__OutputArray_int_TermCriteria(corners.as_raw__InputArray(), ids.as_raw__InputArray(), counter.as_raw__InputArray(), board.as_raw_PtrOfBoard(), image_size, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), std_deviations_intrinsics.as_raw__OutputArray(), std_deviations_extrinsics.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), flags, criteria.as_raw_TermCriteria()) }.into_result()
}

/// It's the same function as #calibrateCameraAruco but without calibration error estimation.
///
/// ## C++ default parameters
/// * rvecs: noArray()
/// * tvecs: noArray()
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT + TermCriteria::EPS, 30, DBL_EPSILON)
pub fn calibrate_camera_aruco(corners: &dyn core::ToInputArray, ids: &dyn core::ToInputArray, counter: &dyn core::ToInputArray, board: &types::PtrOfBoard, image_size: core::Size, camera_matrix: &mut dyn core::ToInputOutputArray, dist_coeffs: &mut dyn core::ToInputOutputArray, rvecs: &mut dyn core::ToOutputArray, tvecs: &mut dyn core::ToOutputArray, flags: i32, criteria: &core::TermCriteria) -> Result<f64> {
    input_array_arg!(corners);
    input_array_arg!(ids);
    input_array_arg!(counter);
    input_output_array_arg!(camera_matrix);
    input_output_array_arg!(dist_coeffs);
    output_array_arg!(rvecs);
    output_array_arg!(tvecs);
    unsafe { sys::cv_aruco_calibrateCameraAruco__InputArray__InputArray__InputArray_PtrOfBoard_Size__InputOutputArray__InputOutputArray__OutputArray__OutputArray_int_TermCriteria(corners.as_raw__InputArray(), ids.as_raw__InputArray(), counter.as_raw__InputArray(), board.as_raw_PtrOfBoard(), image_size, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, criteria.as_raw_TermCriteria()) }.into_result()
}

/// Calibrate a camera using Charuco corners
///
/// ## Parameters
/// * charucoCorners: vector of detected charuco corners per frame
/// * charucoIds: list of identifiers for each corner in charucoCorners per frame
/// * board: Marker Board layout
/// * imageSize: input image size
/// * cameraMatrix: Output 3x3 floating-point camera matrix
/// ![inline formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f_x%20%26%200%20%26%20c_x%5C%5C%200%20%26%20f_y%20%26%20c_y%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D) . If CV\_CALIB\_USE\_INTRINSIC\_GUESS
/// and/or CV_CALIB_FIX_ASPECT_RATIO are specified, some or all of fx, fy, cx, cy must be
/// initialized before calling the function.
/// * distCoeffs: Output vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%28k_1%2C%20k_2%2C%20p_1%2C%20p_2%5B%2C%20k_3%5B%2C%20k_4%2C%20k_5%2C%20k_6%5D%2C%5Bs_1%2C%20s_2%2C%20s_3%2C%20s_4%5D%5D%29) of 4, 5, 8 or 12 elements
/// * rvecs: Output vector of rotation vectors (see Rodrigues ) estimated for each board view
/// (e.g. std::vector<cv::Mat>>). That is, each k-th rotation vector together with the corresponding
/// k-th translation vector (see the next output parameter description) brings the board pattern
/// from the model coordinate space (in which object points are specified) to the world coordinate
/// space, that is, a real position of the board pattern in the k-th pattern view (k=0.. *M* -1).
/// * tvecs: Output vector of translation vectors estimated for each pattern view.
/// * stdDeviationsIntrinsics: Output vector of standard deviations estimated for intrinsic parameters.
/// Order of deviations values:
/// ![inline formula](https://latex.codecogs.com/png.latex?%28f_x%2C%20f_y%2C%20c_x%2C%20c_y%2C%20k_1%2C%20k_2%2C%20p_1%2C%20p_2%2C%20k_3%2C%20k_4%2C%20k_5%2C%20k_6%20%2C%20s_1%2C%20s_2%2C%20s_3%2C%0As_4%2C%20%5Ctau_x%2C%20%5Ctau_y%29) If one of parameters is not estimated, it's deviation is equals to zero.
/// * stdDeviationsExtrinsics: Output vector of standard deviations estimated for extrinsic parameters.
/// Order of deviations values: ![inline formula](https://latex.codecogs.com/png.latex?%28R_1%2C%20T_1%2C%20%5Cdotsc%20%2C%20R_M%2C%20T_M%29) where M is number of pattern views,
/// ![inline formula](https://latex.codecogs.com/png.latex?R_i%2C%20T_i) are concatenated 1x3 vectors.
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
/// * criteria: TermCriteria(TermCriteria::COUNT + TermCriteria::EPS, 30, DBL_EPSILON)
pub fn calibrate_camera_charuco_with_stddev(charuco_corners: &dyn core::ToInputArray, charuco_ids: &dyn core::ToInputArray, board: &types::PtrOfCharucoBoard, image_size: core::Size, camera_matrix: &mut dyn core::ToInputOutputArray, dist_coeffs: &mut dyn core::ToInputOutputArray, rvecs: &mut dyn core::ToOutputArray, tvecs: &mut dyn core::ToOutputArray, std_deviations_intrinsics: &mut dyn core::ToOutputArray, std_deviations_extrinsics: &mut dyn core::ToOutputArray, per_view_errors: &mut dyn core::ToOutputArray, flags: i32, criteria: &core::TermCriteria) -> Result<f64> {
    input_array_arg!(charuco_corners);
    input_array_arg!(charuco_ids);
    input_output_array_arg!(camera_matrix);
    input_output_array_arg!(dist_coeffs);
    output_array_arg!(rvecs);
    output_array_arg!(tvecs);
    output_array_arg!(std_deviations_intrinsics);
    output_array_arg!(std_deviations_extrinsics);
    output_array_arg!(per_view_errors);
    unsafe { sys::cv_aruco_calibrateCameraCharuco__InputArray__InputArray_PtrOfCharucoBoard_Size__InputOutputArray__InputOutputArray__OutputArray__OutputArray__OutputArray__OutputArray__OutputArray_int_TermCriteria(charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), image_size, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), std_deviations_intrinsics.as_raw__OutputArray(), std_deviations_extrinsics.as_raw__OutputArray(), per_view_errors.as_raw__OutputArray(), flags, criteria.as_raw_TermCriteria()) }.into_result()
}

/// It's the same function as #calibrateCameraCharuco but without calibration error estimation.
///
/// ## C++ default parameters
/// * rvecs: noArray()
/// * tvecs: noArray()
/// * flags: 0
/// * criteria: TermCriteria(TermCriteria::COUNT + TermCriteria::EPS, 30, DBL_EPSILON)
pub fn calibrate_camera_charuco(charuco_corners: &dyn core::ToInputArray, charuco_ids: &dyn core::ToInputArray, board: &types::PtrOfCharucoBoard, image_size: core::Size, camera_matrix: &mut dyn core::ToInputOutputArray, dist_coeffs: &mut dyn core::ToInputOutputArray, rvecs: &mut dyn core::ToOutputArray, tvecs: &mut dyn core::ToOutputArray, flags: i32, criteria: &core::TermCriteria) -> Result<f64> {
    input_array_arg!(charuco_corners);
    input_array_arg!(charuco_ids);
    input_output_array_arg!(camera_matrix);
    input_output_array_arg!(dist_coeffs);
    output_array_arg!(rvecs);
    output_array_arg!(tvecs);
    unsafe { sys::cv_aruco_calibrateCameraCharuco__InputArray__InputArray_PtrOfCharucoBoard_Size__InputOutputArray__InputOutputArray__OutputArray__OutputArray_int_TermCriteria(charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), image_size, camera_matrix.as_raw__InputOutputArray(), dist_coeffs.as_raw__InputOutputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), flags, criteria.as_raw_TermCriteria()) }.into_result()
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
    unsafe { sys::cv_aruco_detectCharucoDiamond__InputArray__InputArray__InputArray_float__OutputArray__OutputArray__InputArray__InputArray(image.as_raw__InputArray(), marker_corners.as_raw__InputArray(), marker_ids.as_raw__InputArray(), square_marker_length_rate, diamond_corners.as_raw__OutputArray(), diamond_ids.as_raw__OutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray()) }.into_result()
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
/// ![inline formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f_x%20%26%200%20%26%20c_x%5C%5C%200%20%26%20f_y%20%26%20c_y%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D)
/// * distCoeff: optional vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%28k_1%2C%20k_2%2C%20p_1%2C%20p_2%5B%2C%20k_3%5B%2C%20k_4%2C%20k_5%2C%20k_6%5D%2C%5Bs_1%2C%20s_2%2C%20s_3%2C%20s_4%5D%5D%29) of 4, 5, 8 or 12 elements
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
pub fn detect_markers(image: &dyn core::ToInputArray, dictionary: &types::PtrOfDictionary, corners: &mut dyn core::ToOutputArray, ids: &mut dyn core::ToOutputArray, parameters: &types::PtrOfDetectorParameters, rejected_img_points: &mut dyn core::ToOutputArray, camera_matrix: &dyn core::ToInputArray, dist_coeff: &dyn core::ToInputArray) -> Result<()> {
    input_array_arg!(image);
    output_array_arg!(corners);
    output_array_arg!(ids);
    output_array_arg!(rejected_img_points);
    input_array_arg!(camera_matrix);
    input_array_arg!(dist_coeff);
    unsafe { sys::cv_aruco_detectMarkers__InputArray_PtrOfDictionary__OutputArray__OutputArray_PtrOfDetectorParameters__OutputArray__InputArray__InputArray(image.as_raw__InputArray(), dictionary.as_raw_PtrOfDictionary(), corners.as_raw__OutputArray(), ids.as_raw__OutputArray(), parameters.as_raw_PtrOfDetectorParameters(), rejected_img_points.as_raw__OutputArray(), camera_matrix.as_raw__InputArray(), dist_coeff.as_raw__InputArray()) }.into_result()
}

/// Draw coordinate system axis from pose estimation
///
/// ## Parameters
/// * image: input/output image. It must have 1 or 3 channels. The number of channels is not
/// altered.
/// * cameraMatrix: input 3x3 floating-point camera matrix
/// ![inline formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f_x%20%26%200%20%26%20c_x%5C%5C%200%20%26%20f_y%20%26%20c_y%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D)
/// * distCoeffs: vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%28k_1%2C%20k_2%2C%20p_1%2C%20p_2%5B%2C%20k_3%5B%2C%20k_4%2C%20k_5%2C%20k_6%5D%2C%5Bs_1%2C%20s_2%2C%20s_3%2C%20s_4%5D%5D%29) of 4, 5, 8 or 12 elements
/// * rvec: rotation vector of the coordinate system that will be drawn. (## See also
/// Rodrigues).
/// * tvec: translation vector of the coordinate system that will be drawn.
/// * length: length of the painted axis in the same unit than tvec (usually in meters)
///
/// Given the pose estimation of a marker or board, this function draws the axis of the world
/// coordinate system, i.e. the system centered on the marker/board. Useful for debugging purposes.
///
/// **Deprecated**: use cv::drawFrameAxes
#[deprecated = "use cv::drawFrameAxes"]
pub fn draw_axis(image: &mut dyn core::ToInputOutputArray, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, rvec: &dyn core::ToInputArray, tvec: &dyn core::ToInputArray, length: f32) -> Result<()> {
    input_output_array_arg!(image);
    input_array_arg!(camera_matrix);
    input_array_arg!(dist_coeffs);
    input_array_arg!(rvec);
    input_array_arg!(tvec);
    unsafe { sys::cv_aruco_drawAxis__InputOutputArray__InputArray__InputArray__InputArray__InputArray_float(image.as_raw__InputOutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__InputArray(), tvec.as_raw__InputArray(), length) }.into_result()
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
pub fn draw_charuco_diamond(dictionary: &types::PtrOfDictionary, ids: core::Vec4i, square_length: i32, marker_length: i32, img: &mut dyn core::ToOutputArray, margin_size: i32, border_bits: i32) -> Result<()> {
    output_array_arg!(img);
    unsafe { sys::cv_aruco_drawCharucoDiamond_PtrOfDictionary_Vec4i_int_int__OutputArray_int_int(dictionary.as_raw_PtrOfDictionary(), ids, square_length, marker_length, img.as_raw__OutputArray(), margin_size, border_bits) }.into_result()
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
/// * corner_color: Scalar(255, 0, 0)
pub fn draw_detected_corners_charuco(image: &mut dyn core::ToInputOutputArray, charuco_corners: &dyn core::ToInputArray, charuco_ids: &dyn core::ToInputArray, corner_color: core::Scalar) -> Result<()> {
    input_output_array_arg!(image);
    input_array_arg!(charuco_corners);
    input_array_arg!(charuco_ids);
    unsafe { sys::cv_aruco_drawDetectedCornersCharuco__InputOutputArray__InputArray__InputArray_Scalar(image.as_raw__InputOutputArray(), charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), corner_color) }.into_result()
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
/// * border_color: Scalar(0, 0, 255)
pub fn draw_detected_diamonds(image: &mut dyn core::ToInputOutputArray, diamond_corners: &dyn core::ToInputArray, diamond_ids: &dyn core::ToInputArray, border_color: core::Scalar) -> Result<()> {
    input_output_array_arg!(image);
    input_array_arg!(diamond_corners);
    input_array_arg!(diamond_ids);
    unsafe { sys::cv_aruco_drawDetectedDiamonds__InputOutputArray__InputArray__InputArray_Scalar(image.as_raw__InputOutputArray(), diamond_corners.as_raw__InputArray(), diamond_ids.as_raw__InputArray(), border_color) }.into_result()
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
/// * border_color: Scalar(0, 255, 0)
pub fn draw_detected_markers(image: &mut dyn core::ToInputOutputArray, corners: &dyn core::ToInputArray, ids: &dyn core::ToInputArray, border_color: core::Scalar) -> Result<()> {
    input_output_array_arg!(image);
    input_array_arg!(corners);
    input_array_arg!(ids);
    unsafe { sys::cv_aruco_drawDetectedMarkers__InputOutputArray__InputArray__InputArray_Scalar(image.as_raw__InputOutputArray(), corners.as_raw__InputArray(), ids.as_raw__InputArray(), border_color) }.into_result()
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
pub fn draw_marker(dictionary: &types::PtrOfDictionary, id: i32, side_pixels: i32, img: &mut dyn core::ToOutputArray, border_bits: i32) -> Result<()> {
    output_array_arg!(img);
    unsafe { sys::cv_aruco_drawMarker_PtrOfDictionary_int_int__OutputArray_int(dictionary.as_raw_PtrOfDictionary(), id, side_pixels, img.as_raw__OutputArray(), border_bits) }.into_result()
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
pub fn draw_planar_board(board: &types::PtrOfBoard, out_size: core::Size, img: &mut dyn core::ToOutputArray, margin_size: i32, border_bits: i32) -> Result<()> {
    output_array_arg!(img);
    unsafe { sys::cv_aruco_drawPlanarBoard_PtrOfBoard_Size__OutputArray_int_int(board.as_raw_PtrOfBoard(), out_size, img.as_raw__OutputArray(), margin_size, border_bits) }.into_result()
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
/// ![inline formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f_x%20%26%200%20%26%20c_x%5C%5C%200%20%26%20f_y%20%26%20c_y%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D)
/// * distCoeffs: vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%28k_1%2C%20k_2%2C%20p_1%2C%20p_2%5B%2C%20k_3%5B%2C%20k_4%2C%20k_5%2C%20k_6%5D%2C%5Bs_1%2C%20s_2%2C%20s_3%2C%20s_4%5D%5D%29) of 4, 5, 8 or 12 elements
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
pub fn estimate_pose_board(corners: &dyn core::ToInputArray, ids: &dyn core::ToInputArray, board: &types::PtrOfBoard, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, rvec: &mut dyn core::ToOutputArray, tvec: &mut dyn core::ToOutputArray, use_extrinsic_guess: bool) -> Result<i32> {
    input_array_arg!(corners);
    input_array_arg!(ids);
    input_array_arg!(camera_matrix);
    input_array_arg!(dist_coeffs);
    output_array_arg!(rvec);
    output_array_arg!(tvec);
    unsafe { sys::cv_aruco_estimatePoseBoard__InputArray__InputArray_PtrOfBoard__InputArray__InputArray__OutputArray__OutputArray_bool(corners.as_raw__InputArray(), ids.as_raw__InputArray(), board.as_raw_PtrOfBoard(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), use_extrinsic_guess) }.into_result()
}

/// Pose estimation for a ChArUco board given some of their corners
/// ## Parameters
/// * charucoCorners: vector of detected charuco corners
/// * charucoIds: list of identifiers for each corner in charucoCorners
/// * board: layout of ChArUco board.
/// * cameraMatrix: input 3x3 floating-point camera matrix
/// ![inline formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f_x%20%26%200%20%26%20c_x%5C%5C%200%20%26%20f_y%20%26%20c_y%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D)
/// * distCoeffs: vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%28k_1%2C%20k_2%2C%20p_1%2C%20p_2%5B%2C%20k_3%5B%2C%20k_4%2C%20k_5%2C%20k_6%5D%2C%5Bs_1%2C%20s_2%2C%20s_3%2C%20s_4%5D%5D%29) of 4, 5, 8 or 12 elements
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
pub fn estimate_pose_charuco_board(charuco_corners: &dyn core::ToInputArray, charuco_ids: &dyn core::ToInputArray, board: &types::PtrOfCharucoBoard, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, rvec: &mut dyn core::ToOutputArray, tvec: &mut dyn core::ToOutputArray, use_extrinsic_guess: bool) -> Result<bool> {
    input_array_arg!(charuco_corners);
    input_array_arg!(charuco_ids);
    input_array_arg!(camera_matrix);
    input_array_arg!(dist_coeffs);
    output_array_arg!(rvec);
    output_array_arg!(tvec);
    unsafe { sys::cv_aruco_estimatePoseCharucoBoard__InputArray__InputArray_PtrOfCharucoBoard__InputArray__InputArray__OutputArray__OutputArray_bool(charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvec.as_raw__OutputArray(), tvec.as_raw__OutputArray(), use_extrinsic_guess) }.into_result()
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
/// ![inline formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f_x%20%26%200%20%26%20c_x%5C%5C%200%20%26%20f_y%20%26%20c_y%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D)
/// * distCoeffs: vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%28k_1%2C%20k_2%2C%20p_1%2C%20p_2%5B%2C%20k_3%5B%2C%20k_4%2C%20k_5%2C%20k_6%5D%2C%5Bs_1%2C%20s_2%2C%20s_3%2C%20s_4%5D%5D%29) of 4, 5, 8 or 12 elements
/// * rvecs: array of output rotation vectors ( Rodrigues) (e.g. std::vector<cv::Vec3d>).
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
    unsafe { sys::cv_aruco_estimatePoseSingleMarkers__InputArray_float__InputArray__InputArray__OutputArray__OutputArray__OutputArray(corners.as_raw__InputArray(), marker_length, camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), rvecs.as_raw__OutputArray(), tvecs.as_raw__OutputArray(), _obj_points.as_raw__OutputArray()) }.into_result()
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
pub fn generate_custom_dictionary_with_base(n_markers: i32, marker_size: i32, base_dictionary: &types::PtrOfDictionary, random_seed: i32) -> Result<types::PtrOfDictionary> {
    unsafe { sys::cv_aruco_generateCustomDictionary_int_int_PtrOfDictionary_int(n_markers, marker_size, base_dictionary.as_raw_PtrOfDictionary(), random_seed) }.into_result().map(|ptr| types::PtrOfDictionary { ptr })
}

/// @see generateCustomDictionary
///
/// ## C++ default parameters
/// * random_seed: 0
pub fn generate_custom_dictionary(n_markers: i32, marker_size: i32, random_seed: i32) -> Result<types::PtrOfDictionary> {
    unsafe { sys::cv_aruco_generateCustomDictionary_int_int_int(n_markers, marker_size, random_seed) }.into_result().map(|ptr| types::PtrOfDictionary { ptr })
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
pub fn get_board_object_and_image_points(board: &types::PtrOfBoard, detected_corners: &dyn core::ToInputArray, detected_ids: &dyn core::ToInputArray, obj_points: &mut dyn core::ToOutputArray, img_points: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(detected_corners);
    input_array_arg!(detected_ids);
    output_array_arg!(obj_points);
    output_array_arg!(img_points);
    unsafe { sys::cv_aruco_getBoardObjectAndImagePoints_PtrOfBoard__InputArray__InputArray__OutputArray__OutputArray(board.as_raw_PtrOfBoard(), detected_corners.as_raw__InputArray(), detected_ids.as_raw__InputArray(), obj_points.as_raw__OutputArray(), img_points.as_raw__OutputArray()) }.into_result()
}

/// Returns one of the predefined dictionaries defined in PREDEFINED_DICTIONARY_NAME
pub fn get_predefined_dictionary(name: crate::aruco::PREDEFINED_DICTIONARY_NAME) -> Result<types::PtrOfDictionary> {
    unsafe { sys::cv_aruco_getPredefinedDictionary_PREDEFINED_DICTIONARY_NAME(name) }.into_result().map(|ptr| types::PtrOfDictionary { ptr })
}

/// Returns one of the predefined dictionaries referenced by DICT_*.
pub fn get_predefined_dictionary_i32(dict: i32) -> Result<types::PtrOfDictionary> {
    unsafe { sys::cv_aruco_getPredefinedDictionary_int(dict) }.into_result().map(|ptr| types::PtrOfDictionary { ptr })
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
/// ![inline formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f_x%20%26%200%20%26%20c_x%5C%5C%200%20%26%20f_y%20%26%20c_y%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D)
/// * distCoeffs: optional vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%28k_1%2C%20k_2%2C%20p_1%2C%20p_2%5B%2C%20k_3%5B%2C%20k_4%2C%20k_5%2C%20k_6%5D%2C%5Bs_1%2C%20s_2%2C%20s_3%2C%20s_4%5D%5D%29) of 4, 5, 8 or 12 elements
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
pub fn interpolate_corners_charuco(marker_corners: &dyn core::ToInputArray, marker_ids: &dyn core::ToInputArray, image: &dyn core::ToInputArray, board: &types::PtrOfCharucoBoard, charuco_corners: &mut dyn core::ToOutputArray, charuco_ids: &mut dyn core::ToOutputArray, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, min_markers: i32) -> Result<i32> {
    input_array_arg!(marker_corners);
    input_array_arg!(marker_ids);
    input_array_arg!(image);
    output_array_arg!(charuco_corners);
    output_array_arg!(charuco_ids);
    input_array_arg!(camera_matrix);
    input_array_arg!(dist_coeffs);
    unsafe { sys::cv_aruco_interpolateCornersCharuco__InputArray__InputArray__InputArray_PtrOfCharucoBoard__OutputArray__OutputArray__InputArray__InputArray_int(marker_corners.as_raw__InputArray(), marker_ids.as_raw__InputArray(), image.as_raw__InputArray(), board.as_raw_PtrOfCharucoBoard(), charuco_corners.as_raw__OutputArray(), charuco_ids.as_raw__OutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), min_markers) }.into_result()
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
/// ![inline formula](https://latex.codecogs.com/png.latex?A%20%3D%20%5Cbegin%7Bbmatrix%7D%20f_x%20%26%200%20%26%20c_x%5C%5C%200%20%26%20f_y%20%26%20c_y%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D)
/// * distCoeffs: optional vector of distortion coefficients
/// ![inline formula](https://latex.codecogs.com/png.latex?%28k_1%2C%20k_2%2C%20p_1%2C%20p_2%5B%2C%20k_3%5B%2C%20k_4%2C%20k_5%2C%20k_6%5D%2C%5Bs_1%2C%20s_2%2C%20s_3%2C%20s_4%5D%5D%29) of 4, 5, 8 or 12 elements
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
pub fn refine_detected_markers(image: &dyn core::ToInputArray, board: &types::PtrOfBoard, detected_corners: &mut dyn core::ToInputOutputArray, detected_ids: &mut dyn core::ToInputOutputArray, rejected_corners: &mut dyn core::ToInputOutputArray, camera_matrix: &dyn core::ToInputArray, dist_coeffs: &dyn core::ToInputArray, min_rep_distance: f32, error_correction_rate: f32, check_all_orders: bool, recovered_idxs: &mut dyn core::ToOutputArray, parameters: &types::PtrOfDetectorParameters) -> Result<()> {
    input_array_arg!(image);
    input_output_array_arg!(detected_corners);
    input_output_array_arg!(detected_ids);
    input_output_array_arg!(rejected_corners);
    input_array_arg!(camera_matrix);
    input_array_arg!(dist_coeffs);
    output_array_arg!(recovered_idxs);
    unsafe { sys::cv_aruco_refineDetectedMarkers__InputArray_PtrOfBoard__InputOutputArray__InputOutputArray__InputOutputArray__InputArray__InputArray_float_float_bool__OutputArray_PtrOfDetectorParameters(image.as_raw__InputArray(), board.as_raw_PtrOfBoard(), detected_corners.as_raw__InputOutputArray(), detected_ids.as_raw__InputOutputArray(), rejected_corners.as_raw__InputOutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), min_rep_distance, error_correction_rate, check_all_orders, recovered_idxs.as_raw__OutputArray(), parameters.as_raw_PtrOfDetectorParameters()) }.into_result()
}

// Generating impl for trait crate::aruco::Board
/// Board of markers
///
/// A board is a set of markers in the 3D space with a common coordinate system.
/// The common form of a board of marker is a planar (2D) board, however any 3D layout can be used.
/// A Board object is composed by:
/// - The object points of the marker corners, i.e. their coordinates respect to the board system.
/// - The dictionary which indicates the type of markers of the board
/// - The identifier of all the markers in the board.
pub trait BoardTrait {
    fn as_raw_Board(&self) -> *mut c_void;
}

// boxed class cv::aruco::Board
/// Board of markers
///
/// A board is a set of markers in the 3D space with a common coordinate system.
/// The common form of a board of marker is a planar (2D) board, however any 3D layout can be used.
/// A Board object is composed by:
/// - The object points of the marker corners, i.e. their coordinates respect to the board system.
/// - The dictionary which indicates the type of markers of the board
/// - The identifier of all the markers in the board.
pub struct Board {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for Board {
    fn drop(&mut self) {
        unsafe { sys::cv_Board_delete(self.ptr) };
    }
}

impl Board {
    #[inline(always)] pub fn as_raw_Board(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for Board {}

impl crate::aruco::BoardTrait for Board {
    #[inline(always)] fn as_raw_Board(&self) -> *mut c_void { self.ptr }
}

impl Board {
    /// Provide way to create Board by passing necessary data. Specially needed in Python.
    ///
    /// ## Parameters
    /// * objPoints: array of object points of all the marker corners in the board
    /// * dictionary: the dictionary of markers employed for this board
    /// * ids: vector of the identifiers of the markers in the board
    pub fn create(obj_points: &dyn core::ToInputArray, dictionary: &types::PtrOfDictionary, ids: &dyn core::ToInputArray) -> Result<types::PtrOfBoard> {
        input_array_arg!(obj_points);
        input_array_arg!(ids);
        unsafe { sys::cv_aruco_Board_create__InputArray_PtrOfDictionary__InputArray(obj_points.as_raw__InputArray(), dictionary.as_raw_PtrOfDictionary(), ids.as_raw__InputArray()) }.into_result().map(|ptr| types::PtrOfBoard { ptr })
    }
    
}

// boxed class cv::aruco::CharucoBoard
/// ChArUco board
/// Specific class for ChArUco boards. A ChArUco board is a planar board where the markers are placed
/// inside the white squares of a chessboard. The benefits of ChArUco boards is that they provide
/// both, ArUco markers versatility and chessboard corner precision, which is important for
/// calibration and pose estimation.
/// This class also allows the easy creation and drawing of ChArUco boards.
pub struct CharucoBoard {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for CharucoBoard {
    fn drop(&mut self) {
        unsafe { sys::cv_CharucoBoard_delete(self.ptr) };
    }
}

impl CharucoBoard {
    #[inline(always)] pub fn as_raw_CharucoBoard(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for CharucoBoard {}

impl crate::aruco::BoardTrait for CharucoBoard {
    #[inline(always)] fn as_raw_Board(&self) -> *mut c_void { self.ptr }
}

impl CharucoBoard {
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
    pub fn draw(&mut self, out_size: core::Size, img: &mut dyn core::ToOutputArray, margin_size: i32, border_bits: i32) -> Result<()> {
        output_array_arg!(img);
        unsafe { sys::cv_aruco_CharucoBoard_draw_Size__OutputArray_int_int(self.as_raw_CharucoBoard(), out_size, img.as_raw__OutputArray(), margin_size, border_bits) }.into_result()
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
    pub fn create(squares_x: i32, squares_y: i32, square_length: f32, marker_length: f32, dictionary: &types::PtrOfDictionary) -> Result<types::PtrOfCharucoBoard> {
        unsafe { sys::cv_aruco_CharucoBoard_create_int_int_float_float_PtrOfDictionary(squares_x, squares_y, square_length, marker_length, dictionary.as_raw_PtrOfDictionary()) }.into_result().map(|ptr| types::PtrOfCharucoBoard { ptr })
    }
    
    
    pub fn get_chessboard_size(&self) -> Result<core::Size> {
        unsafe { sys::cv_aruco_CharucoBoard_getChessboardSize_const(self.as_raw_CharucoBoard()) }.into_result()
    }
    
    
    pub fn get_square_length(&self) -> Result<f32> {
        unsafe { sys::cv_aruco_CharucoBoard_getSquareLength_const(self.as_raw_CharucoBoard()) }.into_result()
    }
    
    
    pub fn get_marker_length(&self) -> Result<f32> {
        unsafe { sys::cv_aruco_CharucoBoard_getMarkerLength_const(self.as_raw_CharucoBoard()) }.into_result()
    }
    
}

// boxed class cv::aruco::DetectorParameters
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
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for DetectorParameters {
    fn drop(&mut self) {
        unsafe { sys::cv_DetectorParameters_delete(self.ptr) };
    }
}

impl DetectorParameters {
    #[inline(always)] pub fn as_raw_DetectorParameters(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for DetectorParameters {}

impl DetectorParameters {
    pub fn adaptive_thresh_win_size_min(&self) -> Result<i32> {
        unsafe { sys::cv_aruco_DetectorParameters_adaptiveThreshWinSizeMin_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_adaptive_thresh_win_size_min(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_adaptiveThreshWinSizeMin_int(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn adaptive_thresh_win_size_max(&self) -> Result<i32> {
        unsafe { sys::cv_aruco_DetectorParameters_adaptiveThreshWinSizeMax_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_adaptive_thresh_win_size_max(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_adaptiveThreshWinSizeMax_int(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn adaptive_thresh_win_size_step(&self) -> Result<i32> {
        unsafe { sys::cv_aruco_DetectorParameters_adaptiveThreshWinSizeStep_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_adaptive_thresh_win_size_step(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_adaptiveThreshWinSizeStep_int(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn adaptive_thresh_constant(&self) -> Result<f64> {
        unsafe { sys::cv_aruco_DetectorParameters_adaptiveThreshConstant_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_adaptive_thresh_constant(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_adaptiveThreshConstant_double(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn min_marker_perimeter_rate(&self) -> Result<f64> {
        unsafe { sys::cv_aruco_DetectorParameters_minMarkerPerimeterRate_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_min_marker_perimeter_rate(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_minMarkerPerimeterRate_double(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn max_marker_perimeter_rate(&self) -> Result<f64> {
        unsafe { sys::cv_aruco_DetectorParameters_maxMarkerPerimeterRate_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_max_marker_perimeter_rate(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_maxMarkerPerimeterRate_double(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn polygonal_approx_accuracy_rate(&self) -> Result<f64> {
        unsafe { sys::cv_aruco_DetectorParameters_polygonalApproxAccuracyRate_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_polygonal_approx_accuracy_rate(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_polygonalApproxAccuracyRate_double(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn min_corner_distance_rate(&self) -> Result<f64> {
        unsafe { sys::cv_aruco_DetectorParameters_minCornerDistanceRate_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_min_corner_distance_rate(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_minCornerDistanceRate_double(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn min_distance_to_border(&self) -> Result<i32> {
        unsafe { sys::cv_aruco_DetectorParameters_minDistanceToBorder_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_min_distance_to_border(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_minDistanceToBorder_int(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn min_marker_distance_rate(&self) -> Result<f64> {
        unsafe { sys::cv_aruco_DetectorParameters_minMarkerDistanceRate_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_min_marker_distance_rate(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_minMarkerDistanceRate_double(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn corner_refinement_method(&self) -> Result<i32> {
        unsafe { sys::cv_aruco_DetectorParameters_cornerRefinementMethod_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_corner_refinement_method(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_cornerRefinementMethod_int(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn corner_refinement_win_size(&self) -> Result<i32> {
        unsafe { sys::cv_aruco_DetectorParameters_cornerRefinementWinSize_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_corner_refinement_win_size(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_cornerRefinementWinSize_int(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn corner_refinement_max_iterations(&self) -> Result<i32> {
        unsafe { sys::cv_aruco_DetectorParameters_cornerRefinementMaxIterations_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_corner_refinement_max_iterations(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_cornerRefinementMaxIterations_int(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn corner_refinement_min_accuracy(&self) -> Result<f64> {
        unsafe { sys::cv_aruco_DetectorParameters_cornerRefinementMinAccuracy_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_corner_refinement_min_accuracy(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_cornerRefinementMinAccuracy_double(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn marker_border_bits(&self) -> Result<i32> {
        unsafe { sys::cv_aruco_DetectorParameters_markerBorderBits_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_marker_border_bits(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_markerBorderBits_int(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn perspective_remove_pixel_per_cell(&self) -> Result<i32> {
        unsafe { sys::cv_aruco_DetectorParameters_perspectiveRemovePixelPerCell_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_perspective_remove_pixel_per_cell(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_perspectiveRemovePixelPerCell_int(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn perspective_remove_ignored_margin_per_cell(&self) -> Result<f64> {
        unsafe { sys::cv_aruco_DetectorParameters_perspectiveRemoveIgnoredMarginPerCell_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_perspective_remove_ignored_margin_per_cell(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_perspectiveRemoveIgnoredMarginPerCell_double(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn max_erroneous_bits_in_border_rate(&self) -> Result<f64> {
        unsafe { sys::cv_aruco_DetectorParameters_maxErroneousBitsInBorderRate_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_max_erroneous_bits_in_border_rate(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_maxErroneousBitsInBorderRate_double(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn min_otsu_std_dev(&self) -> Result<f64> {
        unsafe { sys::cv_aruco_DetectorParameters_minOtsuStdDev_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_min_otsu_std_dev(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_minOtsuStdDev_double(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn error_correction_rate(&self) -> Result<f64> {
        unsafe { sys::cv_aruco_DetectorParameters_errorCorrectionRate_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_error_correction_rate(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_errorCorrectionRate_double(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn april_tag_quad_decimate(&self) -> Result<f32> {
        unsafe { sys::cv_aruco_DetectorParameters_aprilTagQuadDecimate_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_april_tag_quad_decimate(&mut self, val: f32) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_aprilTagQuadDecimate_float(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn april_tag_quad_sigma(&self) -> Result<f32> {
        unsafe { sys::cv_aruco_DetectorParameters_aprilTagQuadSigma_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_april_tag_quad_sigma(&mut self, val: f32) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_aprilTagQuadSigma_float(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn april_tag_min_cluster_pixels(&self) -> Result<i32> {
        unsafe { sys::cv_aruco_DetectorParameters_aprilTagMinClusterPixels_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_april_tag_min_cluster_pixels(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_aprilTagMinClusterPixels_int(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn april_tag_max_nmaxima(&self) -> Result<i32> {
        unsafe { sys::cv_aruco_DetectorParameters_aprilTagMaxNmaxima_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_april_tag_max_nmaxima(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_aprilTagMaxNmaxima_int(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn april_tag_critical_rad(&self) -> Result<f32> {
        unsafe { sys::cv_aruco_DetectorParameters_aprilTagCriticalRad_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_april_tag_critical_rad(&mut self, val: f32) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_aprilTagCriticalRad_float(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn april_tag_max_line_fit_mse(&self) -> Result<f32> {
        unsafe { sys::cv_aruco_DetectorParameters_aprilTagMaxLineFitMse_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_april_tag_max_line_fit_mse(&mut self, val: f32) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_aprilTagMaxLineFitMse_float(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn april_tag_min_white_black_diff(&self) -> Result<i32> {
        unsafe { sys::cv_aruco_DetectorParameters_aprilTagMinWhiteBlackDiff_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_april_tag_min_white_black_diff(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_aprilTagMinWhiteBlackDiff_int(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn april_tag_deglitch(&self) -> Result<i32> {
        unsafe { sys::cv_aruco_DetectorParameters_aprilTagDeglitch_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_april_tag_deglitch(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_aprilTagDeglitch_int(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn detect_inverted_marker(&self) -> Result<bool> {
        unsafe { sys::cv_aruco_DetectorParameters_detectInvertedMarker_const(self.as_raw_DetectorParameters()) }.into_result()
    }
    
    pub fn set_detect_inverted_marker(&mut self, val: bool) -> Result<()> {
        unsafe { sys::cv_aruco_DetectorParameters_set_detectInvertedMarker_bool(self.as_raw_DetectorParameters(), val) }.into_result()
    }
    
    pub fn default() -> Result<crate::aruco::DetectorParameters> {
        unsafe { sys::cv_aruco_DetectorParameters_DetectorParameters() }.into_result().map(|ptr| crate::aruco::DetectorParameters { ptr })
    }
    
    pub fn create() -> Result<types::PtrOfDetectorParameters> {
        unsafe { sys::cv_aruco_DetectorParameters_create() }.into_result().map(|ptr| types::PtrOfDetectorParameters { ptr })
    }
    
}

// boxed class cv::aruco::Dictionary
/// Dictionary/Set of markers. It contains the inner codification
///
/// bytesList contains the marker codewords where
/// - bytesList.rows is the dictionary size
/// - each marker is encoded using `nbytes = ceil(markerSize*markerSize/8.)`
/// - each row contains all 4 rotations of the marker, so its length is `4*nbytes`
///
/// `bytesList.ptr(i)[k*nbytes + j]` is then the j-th byte of i-th marker, in its k-th rotation.
pub struct Dictionary {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for Dictionary {
    fn drop(&mut self) {
        unsafe { sys::cv_Dictionary_delete(self.ptr) };
    }
}

impl Dictionary {
    #[inline(always)] pub fn as_raw_Dictionary(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for Dictionary {}

impl Dictionary {
    pub fn bytes_list(&mut self) -> Result<core::Mat> {
        unsafe { sys::cv_aruco_Dictionary_bytesList(self.as_raw_Dictionary()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    pub fn set_bytes_list(&mut self, val: core::Mat) -> Result<()> {
        unsafe { sys::cv_aruco_Dictionary_set_bytesList_Mat(self.as_raw_Dictionary(), val.as_raw_Mat()) }.into_result()
    }
    
    pub fn marker_size(&self) -> Result<i32> {
        unsafe { sys::cv_aruco_Dictionary_markerSize_const(self.as_raw_Dictionary()) }.into_result()
    }
    
    pub fn set_marker_size(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_aruco_Dictionary_set_markerSize_int(self.as_raw_Dictionary(), val) }.into_result()
    }
    
    pub fn max_correction_bits(&self) -> Result<i32> {
        unsafe { sys::cv_aruco_Dictionary_maxCorrectionBits_const(self.as_raw_Dictionary()) }.into_result()
    }
    
    pub fn set_max_correction_bits(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_aruco_Dictionary_set_maxCorrectionBits_int(self.as_raw_Dictionary(), val) }.into_result()
    }
    
    ///
    /// ## C++ default parameters
    /// * _bytes_list: Mat()
    /// * _marker_size: 0
    /// * _maxcorr: 0
    pub fn new(_bytes_list: &core::Mat, _marker_size: i32, _maxcorr: i32) -> Result<crate::aruco::Dictionary> {
        unsafe { sys::cv_aruco_Dictionary_Dictionary_Mat_int_int(_bytes_list.as_raw_Mat(), _marker_size, _maxcorr) }.into_result().map(|ptr| crate::aruco::Dictionary { ptr })
    }
    
    pub fn copy(_dictionary: &types::PtrOfDictionary) -> Result<crate::aruco::Dictionary> {
        unsafe { sys::cv_aruco_Dictionary_Dictionary_PtrOfDictionary(_dictionary.as_raw_PtrOfDictionary()) }.into_result().map(|ptr| crate::aruco::Dictionary { ptr })
    }
    
    /// @see generateCustomDictionary
    ///
    /// ## C++ default parameters
    /// * random_seed: 0
    pub fn create(n_markers: i32, marker_size: i32, random_seed: i32) -> Result<types::PtrOfDictionary> {
        unsafe { sys::cv_aruco_Dictionary_create_int_int_int(n_markers, marker_size, random_seed) }.into_result().map(|ptr| types::PtrOfDictionary { ptr })
    }
    
    /// @see generateCustomDictionary
    ///
    /// ## C++ default parameters
    /// * random_seed: 0
    pub fn create_with_base(n_markers: i32, marker_size: i32, base_dictionary: &types::PtrOfDictionary, random_seed: i32) -> Result<types::PtrOfDictionary> {
        unsafe { sys::cv_aruco_Dictionary_create_int_int_PtrOfDictionary_int(n_markers, marker_size, base_dictionary.as_raw_PtrOfDictionary(), random_seed) }.into_result().map(|ptr| types::PtrOfDictionary { ptr })
    }
    
    /// @see getPredefinedDictionary
    pub fn get(dict: i32) -> Result<types::PtrOfDictionary> {
        unsafe { sys::cv_aruco_Dictionary_get_int(dict) }.into_result().map(|ptr| types::PtrOfDictionary { ptr })
    }
    
    /// Given a matrix of bits. Returns whether if marker is identified or not.
    /// It returns by reference the correct id (if any) and the correct rotation
    pub fn identify(&self, only_bits: &core::Mat, idx: &mut i32, rotation: &mut i32, max_correction_rate: f64) -> Result<bool> {
        unsafe { sys::cv_aruco_Dictionary_identify_const_Mat_int_int_double(self.as_raw_Dictionary(), only_bits.as_raw_Mat(), idx, rotation, max_correction_rate) }.into_result()
    }
    
    /// Returns the distance of the input bits to the specific id. If allRotations is true,
    /// the four posible bits rotation are considered
    ///
    /// ## C++ default parameters
    /// * all_rotations: true
    pub fn get_distance_to_id(&self, bits: &dyn core::ToInputArray, id: i32, all_rotations: bool) -> Result<i32> {
        input_array_arg!(bits);
        unsafe { sys::cv_aruco_Dictionary_getDistanceToId_const__InputArray_int_bool(self.as_raw_Dictionary(), bits.as_raw__InputArray(), id, all_rotations) }.into_result()
    }
    
    /// Draw a canonical marker image
    ///
    /// ## C++ default parameters
    /// * border_bits: 1
    pub fn draw_marker(&self, id: i32, side_pixels: i32, _img: &mut dyn core::ToOutputArray, border_bits: i32) -> Result<()> {
        output_array_arg!(_img);
        unsafe { sys::cv_aruco_Dictionary_drawMarker_const_int_int__OutputArray_int(self.as_raw_Dictionary(), id, side_pixels, _img.as_raw__OutputArray(), border_bits) }.into_result()
    }
    
    /// Transform matrix of bits to list of bytes in the 4 rotations
    pub fn get_byte_list_from_bits(bits: &core::Mat) -> Result<core::Mat> {
        unsafe { sys::cv_aruco_Dictionary_getByteListFromBits_Mat(bits.as_raw_Mat()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// Transform list of bytes to matrix of bits
    pub fn get_bits_from_byte_list(byte_list: &core::Mat, marker_size: i32) -> Result<core::Mat> {
        unsafe { sys::cv_aruco_Dictionary_getBitsFromByteList_Mat_int(byte_list.as_raw_Mat(), marker_size) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
}

// boxed class cv::aruco::GridBoard
/// Planar board with grid arrangement of markers
/// More common type of board. All markers are placed in the same plane in a grid arrangement.
/// The board can be drawn using drawPlanarBoard() function (## See also
/// drawPlanarBoard)
pub struct GridBoard {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for GridBoard {
    fn drop(&mut self) {
        unsafe { sys::cv_GridBoard_delete(self.ptr) };
    }
}

impl GridBoard {
    #[inline(always)] pub fn as_raw_GridBoard(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for GridBoard {}

impl crate::aruco::BoardTrait for GridBoard {
    #[inline(always)] fn as_raw_Board(&self) -> *mut c_void { self.ptr }
}

impl GridBoard {
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
    pub fn draw(&mut self, out_size: core::Size, img: &mut dyn core::ToOutputArray, margin_size: i32, border_bits: i32) -> Result<()> {
        output_array_arg!(img);
        unsafe { sys::cv_aruco_GridBoard_draw_Size__OutputArray_int_int(self.as_raw_GridBoard(), out_size, img.as_raw__OutputArray(), margin_size, border_bits) }.into_result()
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
    pub fn create(markers_x: i32, markers_y: i32, marker_length: f32, marker_separation: f32, dictionary: &types::PtrOfDictionary, first_marker: i32) -> Result<types::PtrOfGridBoard> {
        unsafe { sys::cv_aruco_GridBoard_create_int_int_float_float_PtrOfDictionary_int(markers_x, markers_y, marker_length, marker_separation, dictionary.as_raw_PtrOfDictionary(), first_marker) }.into_result().map(|ptr| types::PtrOfGridBoard { ptr })
    }
    
    
    pub fn get_grid_size(&self) -> Result<core::Size> {
        unsafe { sys::cv_aruco_GridBoard_getGridSize_const(self.as_raw_GridBoard()) }.into_result()
    }
    
    
    pub fn get_marker_length(&self) -> Result<f32> {
        unsafe { sys::cv_aruco_GridBoard_getMarkerLength_const(self.as_raw_GridBoard()) }.into_result()
    }
    
    
    pub fn get_marker_separation(&self) -> Result<f32> {
        unsafe { sys::cv_aruco_GridBoard_getMarkerSeparation_const(self.as_raw_GridBoard()) }.into_result()
    }
    
}

