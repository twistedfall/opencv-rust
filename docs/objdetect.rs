pub mod objdetect {
	//! # Object Detection
	//!    # Barcode detection and decoding
	//!    # QRCode detection and encoding
	//!    # DNN-based face detection and recognition
	//!
	//!    Check [tutorial_dnn_face] "the corresponding tutorial" for more details.
	//!
	//!    # Common functions and classes
	//!    # ArUco markers and boards detection for robust camera pose estimation
	//!        ArUco Marker Detection
	//!        Square fiducial markers (also known as Augmented Reality Markers) are useful for easy,
	//!        fast and robust camera pose estimation.
	//!
	//!        The main functionality of ArucoDetector class is detection of markers in an image. If the markers are grouped
	//!        as a board, then you can try to recover the missing markers with ArucoDetector::refineDetectedMarkers().
	//!        ArUco markers can also be used for advanced chessboard corner finding. To do this, group the markers in the
	//!        CharucoBoard and find the corners of the chessboard with the CharucoDetector::detectBoard().
	//!
	//!        The implementation is based on the ArUco Library by R. Muñoz-Salinas and S. Garrido-Jurado [Aruco2014](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Aruco2014).
	//!
	//!        Markers can also be detected based on the AprilTag 2 [wang2016iros](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_wang2016iros) fiducial marker detection method.
	//! ## See also
	//! [Aruco2014](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Aruco2014)
	//!        This code has been originally developed by Sergio Garrido-Jurado as a project
	//!        for Google Summer of Code 2015 (GSoC 15).
	//!
	//!        <br>
	//!
	//!        @warning In OpenCV, the order of the returned corners locations for the AprilTag family is not aligned with the ArUco one.
	//!
	//!        Note that this order is also different from the convention adopted by the official [AprilTag library](https://github.com/AprilRobotics/apriltag/).
	//!        ![](https://docs.opencv.org/5.0.0/AprilTag_corners_comparison_opencv_april.png) { width=80% }
	//!
	//!        <br>
	//!
	//!        An overview of the supported ArUco markers family is visible in the following image:
	//!        ![](https://docs.opencv.org/5.0.0/ArUco_family.png) { width=80% }
	//!
	//!        <br>
	//!
	//!        An overview of the supported AprilTag markers family is visible in the following image:
	//!        ![](https://docs.opencv.org/5.0.0/AprilTag_family.png) { width=80% }
	//!
	//!         
	//! Note: The generated images (in the above picture) using [aruco::generateImageMarker] for the AprilTag markers have been
	//!        rotated by 180 degree in order to match the official AprilTag images.
	//!        When using the [aruco::generateImageMarker] function, it will output by default a different image from the official AprilTag convention,
	//!        see the [AprilRobotics/apriltag-imgs](https://github.com/AprilRobotics/apriltag-imgs) repository.
	//!        This is the reason why you see a different corners order between ArUco and AprilTag in the above image.
	//!
	//!        <br>
	//!
	//!        For the ArUco marker family, the recommended family is the DICT_ARUCO_MIP_36h12 one, [see](https://stackoverflow.com/a/51511558).
	//!        In general, a smaller marker family (e.g. `4x4` vs `6x6`) should give you a better detection rate with respect to the camera distance,
	//!        at the expense of having more probability to have issues with false detection or marker id decoding error.
	//!        The number of marker ids in a family is also something to take into account with respect to the application use case and the ability
	//!        to correct wrong bits during the marker id decoding process.
	//!
	//!        You can download some pregenerated MIP_36h12 ArUco marker images from:
	//!           - <https://sourceforge.net/projects/aruco/files/>
	//!           - or use the `samples/cpp/tutorial_code/objectDetection/create_marker.cpp` sample to generate the marker image for your
	//!           desired marker family (which uses the [aruco::generateImageMarker] function)
	//!
	//!        For the AprilTag family, you can find some pregenerated marker images in the
	//!        [AprilRobotics/apriltag-imgs](https://github.com/AprilRobotics/apriltag-imgs) repository.
	//!
	//!         
	//! Note: For accurate corners location extraction, a white border (to have a strong gradient between white and black transition) around the marker is important.
	//!        This is necessary to precisely extract the marker contour in difficult conditions such as bad illumination, confusing color background, etc.
	//!
	//!        <br>
	//!
	//!        There are multiple parameters which can be tweaked to improve the marker detection rate or to be adapted to your use case (e.g. image resolution).
	//!        Please refer to the:
	//!           - [aruco::DetectorParameters]
	//!           - "Detector Parameters" section in the [tutorial_aruco_detection] tutorial or in the [tutorial_aruco_faq] page
	//!           - [ArUco Library Documentation](https://drive.google.com/file/d/1OiavRVYVJ-WH88sQg1LUsh8CuJZUQyrX) for additional information from the ArUco library
	//!
	//!        The corner refinement method can be changed according to the [aruco::CornerRefineMethod] to improve the corners location accuracy
	//!        at the expense of more computation time.
	//!
	//!        <br>
	//!
	//!        To estimate the marker pose with respect to the camera frame, we recommend you to look at the following sources of information:
	//!           - [tutorial_aruco_detection] for a tutorial about ArUco markers detection
	//!           - [_3d] for some theoretical background about the pinhole camera model and the [calib3d_solvePnP] page
	//!           - [solvePnP], [solvePnPGeneric], [solveP3P] for the relevant pose estimation methods
	use crate::mod_prelude::*;
	use crate::{core, sys, types};
	pub mod prelude {
		pub use super::{ArucoDetectorTrait, ArucoDetectorTraitConst, BarcodeDetectorTrait, BarcodeDetectorTraitConst, BoardTrait, BoardTraitConst, CCheckerDetectorTrait, CCheckerDetectorTraitConst, CCheckerTrait, CCheckerTraitConst, CharucoBoardTrait, CharucoBoardTraitConst, CharucoDetectorTrait, CharucoDetectorTraitConst, CharucoParametersTrait, CharucoParametersTraitConst, DetectorParametersTrait, DetectorParametersTraitConst, DictionaryTrait, DictionaryTraitConst, FaceDetectorYNTrait, FaceDetectorYNTraitConst, FaceRecognizerSFTrait, FaceRecognizerSFTraitConst, GraphicalCodeDetectorTrait, GraphicalCodeDetectorTraitConst, GridBoardTrait, GridBoardTraitConst, QRCodeDetectorArucoTrait, QRCodeDetectorArucoTraitConst, QRCodeDetectorTrait, QRCodeDetectorTraitConst, QRCodeEncoderTrait, QRCodeEncoderTraitConst};
	}

	pub const CALIB_CB_ACCURACY: i32 = 32;
	pub const CALIB_CB_ADAPTIVE_THRESH: i32 = 1;
	pub const CALIB_CB_ASYMMETRIC_GRID: i32 = 2;
	pub const CALIB_CB_CLUSTERING: i32 = 4;
	pub const CALIB_CB_EXHAUSTIVE: i32 = 16;
	pub const CALIB_CB_FAST_CHECK: i32 = 8;
	pub const CALIB_CB_FILTER_QUADS: i32 = 4;
	pub const CALIB_CB_LARGER: i32 = 64;
	pub const CALIB_CB_MARKER: i32 = 128;
	pub const CALIB_CB_NORMALIZE_IMAGE: i32 = 2;
	pub const CALIB_CB_PLAIN: i32 = 256;
	pub const CALIB_CB_SYMMETRIC_GRID: i32 = 1;
	/// Tag and corners detection based on the AprilTag 2 approach [wang2016iros](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_wang2016iros)
	pub const CORNER_REFINE_APRILTAG: i32 = 3;
	/// ArUco approach and refine the corners locations using the contour-points line fitting
	pub const CORNER_REFINE_CONTOUR: i32 = 2;
	/// Tag and corners detection based on the ArUco approach
	pub const CORNER_REFINE_NONE: i32 = 0;
	/// ArUco approach and refine the corners locations using corner subpixel accuracy
	pub const CORNER_REFINE_SUBPIX: i32 = 1;
	pub const CirclesGridFinderParameters_ASYMMETRIC_GRID: i32 = 1;
	pub const CirclesGridFinderParameters_SYMMETRIC_GRID: i32 = 0;
	pub const DEFAULT_VALID_BIT_ID_THRESHOLD: f64 = 0.49000000953674316;
	/// 4x4 bits, minimum hamming distance between any two codes = 3, 100 codes
	pub const DICT_4X4_100: i32 = 1;
	/// 4x4 bits, minimum hamming distance between any two codes = 2, 1000 codes
	pub const DICT_4X4_1000: i32 = 3;
	/// 4x4 bits, minimum hamming distance between any two codes = 3, 250 codes
	pub const DICT_4X4_250: i32 = 2;
	/// 4x4 bits, minimum hamming distance between any two codes = 4, 50 codes
	pub const DICT_4X4_50: i32 = 0;
	/// 5x5 bits, minimum hamming distance between any two codes = 7, 100 codes
	pub const DICT_5X5_100: i32 = 5;
	/// 5x5 bits, minimum hamming distance between any two codes = 5, 1000 codes
	pub const DICT_5X5_1000: i32 = 7;
	/// 5x5 bits, minimum hamming distance between any two codes = 6, 250 codes
	pub const DICT_5X5_250: i32 = 6;
	/// 5x5 bits, minimum hamming distance between any two codes = 8, 50 codes
	pub const DICT_5X5_50: i32 = 4;
	/// 6x6 bits, minimum hamming distance between any two codes = 12, 100 codes
	pub const DICT_6X6_100: i32 = 9;
	/// 6x6 bits, minimum hamming distance between any two codes = 9, 1000 codes
	pub const DICT_6X6_1000: i32 = 11;
	/// 6x6 bits, minimum hamming distance between any two codes = 11, 250 codes
	pub const DICT_6X6_250: i32 = 10;
	/// 6x6 bits, minimum hamming distance between any two codes = 13, 50 codes
	pub const DICT_6X6_50: i32 = 8;
	/// 7x7 bits, minimum hamming distance between any two codes = 18, 100 codes
	pub const DICT_7X7_100: i32 = 13;
	/// 7x7 bits, minimum hamming distance between any two codes = 14, 1000 codes
	pub const DICT_7X7_1000: i32 = 15;
	/// 7x7 bits, minimum hamming distance between any two codes = 17, 250 codes
	pub const DICT_7X7_250: i32 = 14;
	/// 7x7 bits, minimum hamming distance between any two codes = 19, 50 codes
	pub const DICT_7X7_50: i32 = 12;
	/// 4x4 bits, minimum hamming distance between any two codes = 5, 30 codes
	pub const DICT_APRILTAG_16h5: i32 = 17;
	/// 5x5 bits, minimum hamming distance between any two codes = 9, 35 codes
	pub const DICT_APRILTAG_25h9: i32 = 18;
	/// 6x6 bits, minimum hamming distance between any two codes = 10, 2320 codes
	pub const DICT_APRILTAG_36h10: i32 = 19;
	/// 6x6 bits, minimum hamming distance between any two codes = 11, 587 codes
	pub const DICT_APRILTAG_36h11: i32 = 20;
	/// 6x6 bits, minimum hamming distance between any two codes = 12, 250 codes
	pub const DICT_ARUCO_MIP_36h12: i32 = 21;
	/// 6x6 bits, minimum hamming distance between any two codes = 3, 1024 codes
	pub const DICT_ARUCO_ORIGINAL: i32 = 16;
	pub const FaceRecognizerSF_FR_COSINE: i32 = 0;
	pub const FaceRecognizerSF_FR_NORM_L2: i32 = 1;
	/// Standard Macbeth Chart with 24 squares
	pub const MCC24: i32 = 0;
	pub const QRCodeEncoder_CORRECT_LEVEL_H: i32 = 3;
	pub const QRCodeEncoder_CORRECT_LEVEL_L: i32 = 0;
	pub const QRCodeEncoder_CORRECT_LEVEL_M: i32 = 1;
	pub const QRCodeEncoder_CORRECT_LEVEL_Q: i32 = 2;
	pub const QRCodeEncoder_ECI_SHIFT_JIS: i32 = 20;
	pub const QRCodeEncoder_ECI_UTF8: i32 = 26;
	pub const QRCodeEncoder_MODE_ALPHANUMERIC: i32 = 2;
	pub const QRCodeEncoder_MODE_AUTO: i32 = -1;
	pub const QRCodeEncoder_MODE_BYTE: i32 = 4;
	pub const QRCodeEncoder_MODE_ECI: i32 = 7;
	pub const QRCodeEncoder_MODE_KANJI: i32 = 8;
	pub const QRCodeEncoder_MODE_NUMERIC: i32 = 1;
	pub const QRCodeEncoder_MODE_STRUCTURED_APPEND: i32 = 3;
	/// DigitalSG with 140 squares
	pub const SG140: i32 = 1;
	/// DKK color chart with 12 squares and 6 rectangle
	pub const VINYL18: i32 = 2;
	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum CirclesGridFinderParameters_GridType {
		SYMMETRIC_GRID = 0,
		ASYMMETRIC_GRID = 1,
	}

	opencv_type_enum! { crate::objdetect::CirclesGridFinderParameters_GridType { SYMMETRIC_GRID, ASYMMETRIC_GRID } }

	/// ColorChart
	///
	/// enum to hold the type of the checker
	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum ColorChart {
		/// Standard Macbeth Chart with 24 squares
		MCC24 = 0,
		/// DigitalSG with 140 squares
		SG140 = 1,
		/// DKK color chart with 12 squares and 6 rectangle
		VINYL18 = 2,
	}

	opencv_type_enum! { crate::objdetect::ColorChart { MCC24, SG140, VINYL18 } }

	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum CornerRefineMethod {
		/// Tag and corners detection based on the ArUco approach
		CORNER_REFINE_NONE = 0,
		/// ArUco approach and refine the corners locations using corner subpixel accuracy
		CORNER_REFINE_SUBPIX = 1,
		/// ArUco approach and refine the corners locations using the contour-points line fitting
		CORNER_REFINE_CONTOUR = 2,
		/// Tag and corners detection based on the AprilTag 2 approach [wang2016iros](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_wang2016iros)
		CORNER_REFINE_APRILTAG = 3,
	}

	opencv_type_enum! { crate::objdetect::CornerRefineMethod { CORNER_REFINE_NONE, CORNER_REFINE_SUBPIX, CORNER_REFINE_CONTOUR, CORNER_REFINE_APRILTAG } }

	/// Definition of distance used for calculating the distance between two face features
	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum FaceRecognizerSF_DisType {
		FR_COSINE = 0,
		FR_NORM_L2 = 1,
	}

	opencv_type_enum! { crate::objdetect::FaceRecognizerSF_DisType { FR_COSINE, FR_NORM_L2 } }

	/// Predefined markers dictionaries/sets
	///
	/// Each dictionary indicates the number of bits and the number of markers contained
	/// - DICT_ARUCO_ORIGINAL: standard ArUco Library Markers. 1024 markers, 5x5 bits, 0 minimum
	///                        distance
	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum PredefinedDictionaryType {
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
		/// 6x6 bits, minimum hamming distance between any two codes = 12, 250 codes
		DICT_ARUCO_MIP_36h12 = 21,
	}

	opencv_type_enum! { crate::objdetect::PredefinedDictionaryType { DICT_4X4_50, DICT_4X4_100, DICT_4X4_250, DICT_4X4_1000, DICT_5X5_50, DICT_5X5_100, DICT_5X5_250, DICT_5X5_1000, DICT_6X6_50, DICT_6X6_100, DICT_6X6_250, DICT_6X6_1000, DICT_7X7_50, DICT_7X7_100, DICT_7X7_250, DICT_7X7_1000, DICT_ARUCO_ORIGINAL, DICT_APRILTAG_16h5, DICT_APRILTAG_25h9, DICT_APRILTAG_36h10, DICT_APRILTAG_36h11, DICT_ARUCO_MIP_36h12 } }

	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum QRCodeEncoder_CorrectionLevel {
		CORRECT_LEVEL_L = 0,
		CORRECT_LEVEL_M = 1,
		CORRECT_LEVEL_Q = 2,
		CORRECT_LEVEL_H = 3,
	}

	opencv_type_enum! { crate::objdetect::QRCodeEncoder_CorrectionLevel { CORRECT_LEVEL_L, CORRECT_LEVEL_M, CORRECT_LEVEL_Q, CORRECT_LEVEL_H } }

	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum QRCodeEncoder_ECIEncodings {
		ECI_SHIFT_JIS = 20,
		ECI_UTF8 = 26,
	}

	opencv_type_enum! { crate::objdetect::QRCodeEncoder_ECIEncodings { ECI_SHIFT_JIS, ECI_UTF8 } }

	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum QRCodeEncoder_EncodeMode {
		MODE_AUTO = -1,
		MODE_NUMERIC = 1,
		MODE_ALPHANUMERIC = 2,
		MODE_BYTE = 4,
		MODE_ECI = 7,
		MODE_KANJI = 8,
		MODE_STRUCTURED_APPEND = 3,
	}

	opencv_type_enum! { crate::objdetect::QRCodeEncoder_EncodeMode { MODE_AUTO, MODE_NUMERIC, MODE_ALPHANUMERIC, MODE_BYTE, MODE_ECI, MODE_KANJI, MODE_STRUCTURED_APPEND } }

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
	#[inline]
	pub fn draw_detected_corners_charuco_def(image: &mut impl ToInputOutputArray, charuco_corners: &impl ToInputArray) -> Result<()> {
		input_output_array_arg!(image);
		input_array_arg!(charuco_corners);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_drawDetectedCornersCharuco_const__InputOutputArrayR_const__InputArrayR(image.as_raw__InputOutputArray(), charuco_corners.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
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
	#[inline]
	pub fn draw_detected_corners_charuco(image: &mut impl ToInputOutputArray, charuco_corners: &impl ToInputArray, charuco_ids: &impl ToInputArray, corner_color: core::Scalar) -> Result<()> {
		input_output_array_arg!(image);
		input_array_arg!(charuco_corners);
		input_array_arg!(charuco_ids);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_drawDetectedCornersCharuco_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(image.as_raw__InputOutputArray(), charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), &corner_color, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
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
	#[inline]
	pub fn draw_detected_diamonds_def(image: &mut impl ToInputOutputArray, diamond_corners: &impl ToInputArray) -> Result<()> {
		input_output_array_arg!(image);
		input_array_arg!(diamond_corners);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_drawDetectedDiamonds_const__InputOutputArrayR_const__InputArrayR(image.as_raw__InputOutputArray(), diamond_corners.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
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
	#[inline]
	pub fn draw_detected_diamonds(image: &mut impl ToInputOutputArray, diamond_corners: &impl ToInputArray, diamond_ids: &impl ToInputArray, border_color: core::Scalar) -> Result<()> {
		input_output_array_arg!(image);
		input_array_arg!(diamond_corners);
		input_array_arg!(diamond_ids);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_drawDetectedDiamonds_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(image.as_raw__InputOutputArray(), diamond_corners.as_raw__InputArray(), diamond_ids.as_raw__InputArray(), &border_color, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Draw detected markers in image
	///
	/// ## Parameters
	/// * image: input/output image. It must have 1 or 3 channels. The number of channels is not altered.
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
	#[inline]
	pub fn draw_detected_markers_def(image: &mut impl ToInputOutputArray, corners: &impl ToInputArray) -> Result<()> {
		input_output_array_arg!(image);
		input_array_arg!(corners);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_drawDetectedMarkers_const__InputOutputArrayR_const__InputArrayR(image.as_raw__InputOutputArray(), corners.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Draw detected markers in image
	///
	/// ## Parameters
	/// * image: input/output image. It must have 1 or 3 channels. The number of channels is not altered.
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
	#[inline]
	pub fn draw_detected_markers(image: &mut impl ToInputOutputArray, corners: &impl ToInputArray, ids: &impl ToInputArray, border_color: core::Scalar) -> Result<()> {
		input_output_array_arg!(image);
		input_array_arg!(corners);
		input_array_arg!(ids);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_drawDetectedMarkers_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(image.as_raw__InputOutputArray(), corners.as_raw__InputArray(), ids.as_raw__InputArray(), &border_color, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Extend base dictionary by new nMarkers
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
	/// This alternative version of [extend_dictionary] function uses the following default values for its arguments:
	/// * base_dictionary: Dictionary()
	/// * random_seed: 0
	#[inline]
	pub fn extend_dictionary_def(n_markers: i32, marker_size: i32) -> Result<crate::objdetect::Dictionary> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_extendDictionary_int_int(n_markers, marker_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::objdetect::Dictionary::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Extend base dictionary by new nMarkers
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
	/// * base_dictionary: Dictionary()
	/// * random_seed: 0
	#[inline]
	pub fn extend_dictionary(n_markers: i32, marker_size: i32, base_dictionary: &impl crate::objdetect::DictionaryTraitConst, random_seed: i32) -> Result<crate::objdetect::Dictionary> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_extendDictionary_int_int_const_DictionaryR_int(n_markers, marker_size, base_dictionary.as_raw_Dictionary(), random_seed, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::objdetect::Dictionary::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Generate a canonical marker image
	///
	/// ## Parameters
	/// * dictionary: dictionary of markers indicating the type of markers
	/// * id: identifier of the marker that will be returned. It has to be a valid id in the specified dictionary.
	/// * sidePixels: size of the image in pixels
	/// * img: output image with the marker
	/// * borderBits: width of the marker border.
	///
	/// This function returns a marker image in its canonical form (i.e. ready to be printed)
	///
	/// ## Note
	/// This alternative version of [generate_image_marker] function uses the following default values for its arguments:
	/// * border_bits: 1
	#[inline]
	pub fn generate_image_marker_def(dictionary: &impl crate::objdetect::DictionaryTraitConst, id: i32, side_pixels: i32, img: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_generateImageMarker_const_DictionaryR_int_int_const__OutputArrayR(dictionary.as_raw_Dictionary(), id, side_pixels, img.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Generate a canonical marker image
	///
	/// ## Parameters
	/// * dictionary: dictionary of markers indicating the type of markers
	/// * id: identifier of the marker that will be returned. It has to be a valid id in the specified dictionary.
	/// * sidePixels: size of the image in pixels
	/// * img: output image with the marker
	/// * borderBits: width of the marker border.
	///
	/// This function returns a marker image in its canonical form (i.e. ready to be printed)
	///
	/// ## C++ default parameters
	/// * border_bits: 1
	#[inline]
	pub fn generate_image_marker(dictionary: &impl crate::objdetect::DictionaryTraitConst, id: i32, side_pixels: i32, img: &mut impl ToOutputArray, border_bits: i32) -> Result<()> {
		output_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_generateImageMarker_const_DictionaryR_int_int_const__OutputArrayR_int(dictionary.as_raw_Dictionary(), id, side_pixels, img.as_raw__OutputArray(), border_bits, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns one of the predefined dictionaries defined in PredefinedDictionaryType
	#[inline]
	pub fn get_predefined_dictionary(name: crate::objdetect::PredefinedDictionaryType) -> Result<crate::objdetect::Dictionary> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_getPredefinedDictionary_PredefinedDictionaryType(name, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::objdetect::Dictionary::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Returns one of the predefined dictionaries referenced by DICT_*.
	#[inline]
	pub fn get_predefined_dictionary_i32(dict: i32) -> Result<crate::objdetect::Dictionary> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_getPredefinedDictionary_int(dict, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::objdetect::Dictionary::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Checks whether the image contains chessboard of the specific size or not.
	///
	/// ## Parameters
	/// * img: Source chessboard view.
	/// * size: Size of the chessboard.
	///
	/// ## Returns
	/// Whether a chessboard was found.
	#[inline]
	pub fn check_chessboard(img: &impl ToInputArray, size: core::Size) -> Result<bool> {
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_checkChessboard_const__InputArrayR_Size(img.as_raw__InputArray(), &size, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Renders the detected chessboard corners.
	///
	/// ## Parameters
	/// * image: Destination image. It must be an 8-bit color image.
	/// * patternSize: Number of inner corners per a chessboard row and column
	/// (patternSize = cv::Size(points_per_row,points_per_column)).
	/// * corners: Array of detected corners, the output of #findChessboardCorners.
	/// * patternWasFound: Parameter indicating whether the complete board was found or not. The
	/// return value of [find_chessboard_corners] should be passed here.
	///
	/// The function draws individual chessboard corners detected either as red circles if the board was not
	/// found, or as colored corners connected with lines if the board was found.
	#[inline]
	pub fn draw_chessboard_corners(image: &mut impl ToInputOutputArray, pattern_size: core::Size, corners: &impl ToInputArray, pattern_was_found: bool) -> Result<()> {
		input_output_array_arg!(image);
		input_array_arg!(corners);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_drawChessboardCorners_const__InputOutputArrayR_Size_const__InputArrayR_bool(image.as_raw__InputOutputArray(), &pattern_size, corners.as_raw__InputArray(), pattern_was_found, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Estimates the sharpness of a detected chessboard.
	///
	/// Image sharpness, as well as brightness, are a critical parameter for accuracte
	/// camera calibration. For accessing these parameters for filtering out
	/// problematic calibraiton images, this method calculates edge profiles by traveling from
	/// black to white chessboard cell centers. Based on this, the number of pixels is
	/// calculated required to transit from black to white. This width of the
	/// transition area is a good indication of how sharp the chessboard is imaged
	/// and should be below ~3.0 pixels.
	///
	/// ## Parameters
	/// * image: Gray image used to find chessboard corners
	/// * patternSize: Size of a found chessboard pattern
	/// * corners: Corners found by [find_chessboard_corners_sb]
	/// * rise_distance: Rise distance 0.8 means 10% ... 90% of the final signal strength
	/// * vertical: By default edge responses for horizontal lines are calculated
	/// * sharpness: Optional output array with a sharpness value for calculated edge responses (see description)
	///
	/// The optional sharpness array is of type CV_32FC1 and has for each calculated
	/// profile one row with the following five entries:
	/// * 0 = x coordinate of the underlying edge in the image
	/// * 1 = y coordinate of the underlying edge in the image
	/// * 2 = width of the transition area (sharpness)
	/// * 3 = signal strength in the black cell (min brightness)
	/// * 4 = signal strength in the white cell (max brightness)
	///
	/// ## Returns
	/// Scalar(average sharpness, average min brightness, average max brightness,0)
	///
	/// ## Note
	/// This alternative version of [estimate_chessboard_sharpness] function uses the following default values for its arguments:
	/// * rise_distance: 0.8F
	/// * vertical: false
	/// * sharpness: noArray()
	#[inline]
	pub fn estimate_chessboard_sharpness_def(image: &impl ToInputArray, pattern_size: core::Size, corners: &impl ToInputArray) -> Result<core::Scalar> {
		input_array_arg!(image);
		input_array_arg!(corners);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_estimateChessboardSharpness_const__InputArrayR_Size_const__InputArrayR(image.as_raw__InputArray(), &pattern_size, corners.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Estimates the sharpness of a detected chessboard.
	///
	/// Image sharpness, as well as brightness, are a critical parameter for accuracte
	/// camera calibration. For accessing these parameters for filtering out
	/// problematic calibraiton images, this method calculates edge profiles by traveling from
	/// black to white chessboard cell centers. Based on this, the number of pixels is
	/// calculated required to transit from black to white. This width of the
	/// transition area is a good indication of how sharp the chessboard is imaged
	/// and should be below ~3.0 pixels.
	///
	/// ## Parameters
	/// * image: Gray image used to find chessboard corners
	/// * patternSize: Size of a found chessboard pattern
	/// * corners: Corners found by [find_chessboard_corners_sb]
	/// * rise_distance: Rise distance 0.8 means 10% ... 90% of the final signal strength
	/// * vertical: By default edge responses for horizontal lines are calculated
	/// * sharpness: Optional output array with a sharpness value for calculated edge responses (see description)
	///
	/// The optional sharpness array is of type CV_32FC1 and has for each calculated
	/// profile one row with the following five entries:
	/// * 0 = x coordinate of the underlying edge in the image
	/// * 1 = y coordinate of the underlying edge in the image
	/// * 2 = width of the transition area (sharpness)
	/// * 3 = signal strength in the black cell (min brightness)
	/// * 4 = signal strength in the white cell (max brightness)
	///
	/// ## Returns
	/// Scalar(average sharpness, average min brightness, average max brightness,0)
	///
	/// ## C++ default parameters
	/// * rise_distance: 0.8F
	/// * vertical: false
	/// * sharpness: noArray()
	#[inline]
	pub fn estimate_chessboard_sharpness(image: &impl ToInputArray, pattern_size: core::Size, corners: &impl ToInputArray, rise_distance: f32, vertical: bool, sharpness: &mut impl ToOutputArray) -> Result<core::Scalar> {
		input_array_arg!(image);
		input_array_arg!(corners);
		output_array_arg!(sharpness);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_estimateChessboardSharpness_const__InputArrayR_Size_const__InputArrayR_float_bool_const__OutputArrayR(image.as_raw__InputArray(), &pattern_size, corners.as_raw__InputArray(), rise_distance, vertical, sharpness.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// finds subpixel-accurate positions of the chessboard corners
	#[inline]
	pub fn find4_quad_corner_subpix(img: &impl ToInputArray, corners: &mut impl ToInputOutputArray, region_size: core::Size) -> Result<bool> {
		input_array_arg!(img);
		input_output_array_arg!(corners);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_find4QuadCornerSubpix_const__InputArrayR_const__InputOutputArrayR_Size(img.as_raw__InputArray(), corners.as_raw__InputOutputArray(), &region_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds the positions of internal corners of the chessboard using a sector based approach.
	///
	/// ## Parameters
	/// * image: Source chessboard view. It must be an 8-bit grayscale or color image.
	/// * patternSize: Number of inner corners per a chessboard row and column
	/// ( patternSize = cv::Size(points_per_row,points_per_column) = cv::Size(columns,rows) ).
	/// * corners: Output array of detected corners.
	/// * flags: Various operation flags that can be zero or a combination of the following values:
	/// *   [CALIB_CB_NORMALIZE_IMAGE] Normalize the image gamma with equalizeHist before detection.
	/// *   [CALIB_CB_EXHAUSTIVE] Run an exhaustive search to improve detection rate.
	/// *   [CALIB_CB_ACCURACY] Up sample input image to improve sub-pixel accuracy due to aliasing effects.
	/// *   [CALIB_CB_LARGER] The detected pattern is allowed to be larger than patternSize (see description).
	/// *   [CALIB_CB_MARKER] The detected pattern must have a marker (see description).
	/// This should be used if an accurate camera calibration is required.
	/// * meta: Optional output array of detected corners (CV_8UC1 and size = cv::Size(columns,rows)).
	/// Each entry stands for one corner of the pattern and can have one of the following values:
	/// *   0 = no meta data attached
	/// *   1 = left-top corner of a black cell
	/// *   2 = left-top corner of a white cell
	/// *   3 = left-top corner of a black cell with a white marker dot
	/// *   4 = left-top corner of a white cell with a black marker dot (pattern origin in case of markers otherwise first corner)
	///
	/// The function is analog to [find_chessboard_corners] but uses a localized radon
	/// transformation approximated by box filters being more robust to all sort of
	/// noise, faster on larger images and is able to directly return the sub-pixel
	/// position of the internal chessboard corners. The Method is based on the paper
	/// [duda2018](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_duda2018) "Accurate Detection and Localization of Checkerboard Corners for
	/// Calibration" demonstrating that the returned sub-pixel positions are more
	/// accurate than the one returned by cornerSubPix allowing a precise camera
	/// calibration for demanding applications.
	///
	/// In the case, the flags [CALIB_CB_LARGER] or [CALIB_CB_MARKER] are given,
	/// the result can be recovered from the optional meta array. Both flags are
	/// helpful to use calibration patterns exceeding the field of view of the camera.
	/// These oversized patterns allow more accurate calibrations as corners can be
	/// utilized, which are as close as possible to the image borders.  For a
	/// consistent coordinate system across all images, the optional marker (see image
	/// below) can be used to move the origin of the board to the location where the
	/// black circle is located.
	///
	///
	/// Note: The function requires a white boarder with roughly the same width as one
	/// of the checkerboard fields around the whole board to improve the detection in
	/// various environments. In addition, because of the localized radon
	/// transformation it is beneficial to use round corners for the field corners
	/// which are located on the outside of the board. The following figure illustrates
	/// a sample checkerboard optimized for the detection. However, any other checkerboard
	/// can be used as well.
	///
	/// Use the `generate_pattern.py` Python script ([tutorial_camera_calibration_pattern])
	/// to create the corresponding checkerboard pattern:
	/// \image html pics/checkerboard_radon.png width=60%
	///
	/// ## Overloaded parameters
	///
	///
	/// ## Note
	/// This alternative version of [find_chessboard_corners_sb] function uses the following default values for its arguments:
	/// * flags: 0
	#[inline]
	pub fn find_chessboard_corners_sb_def(image: &impl ToInputArray, pattern_size: core::Size, corners: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(corners);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findChessboardCornersSB_const__InputArrayR_Size_const__OutputArrayR(image.as_raw__InputArray(), &pattern_size, corners.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds the positions of internal corners of the chessboard using a sector based approach.
	///
	/// ## Parameters
	/// * image: Source chessboard view. It must be an 8-bit grayscale or color image.
	/// * patternSize: Number of inner corners per a chessboard row and column
	/// ( patternSize = cv::Size(points_per_row,points_per_column) = cv::Size(columns,rows) ).
	/// * corners: Output array of detected corners.
	/// * flags: Various operation flags that can be zero or a combination of the following values:
	/// *   [CALIB_CB_NORMALIZE_IMAGE] Normalize the image gamma with equalizeHist before detection.
	/// *   [CALIB_CB_EXHAUSTIVE] Run an exhaustive search to improve detection rate.
	/// *   [CALIB_CB_ACCURACY] Up sample input image to improve sub-pixel accuracy due to aliasing effects.
	/// *   [CALIB_CB_LARGER] The detected pattern is allowed to be larger than patternSize (see description).
	/// *   [CALIB_CB_MARKER] The detected pattern must have a marker (see description).
	/// This should be used if an accurate camera calibration is required.
	/// * meta: Optional output array of detected corners (CV_8UC1 and size = cv::Size(columns,rows)).
	/// Each entry stands for one corner of the pattern and can have one of the following values:
	/// *   0 = no meta data attached
	/// *   1 = left-top corner of a black cell
	/// *   2 = left-top corner of a white cell
	/// *   3 = left-top corner of a black cell with a white marker dot
	/// *   4 = left-top corner of a white cell with a black marker dot (pattern origin in case of markers otherwise first corner)
	///
	/// The function is analog to [find_chessboard_corners] but uses a localized radon
	/// transformation approximated by box filters being more robust to all sort of
	/// noise, faster on larger images and is able to directly return the sub-pixel
	/// position of the internal chessboard corners. The Method is based on the paper
	/// [duda2018](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_duda2018) "Accurate Detection and Localization of Checkerboard Corners for
	/// Calibration" demonstrating that the returned sub-pixel positions are more
	/// accurate than the one returned by cornerSubPix allowing a precise camera
	/// calibration for demanding applications.
	///
	/// In the case, the flags [CALIB_CB_LARGER] or [CALIB_CB_MARKER] are given,
	/// the result can be recovered from the optional meta array. Both flags are
	/// helpful to use calibration patterns exceeding the field of view of the camera.
	/// These oversized patterns allow more accurate calibrations as corners can be
	/// utilized, which are as close as possible to the image borders.  For a
	/// consistent coordinate system across all images, the optional marker (see image
	/// below) can be used to move the origin of the board to the location where the
	/// black circle is located.
	///
	///
	/// Note: The function requires a white boarder with roughly the same width as one
	/// of the checkerboard fields around the whole board to improve the detection in
	/// various environments. In addition, because of the localized radon
	/// transformation it is beneficial to use round corners for the field corners
	/// which are located on the outside of the board. The following figure illustrates
	/// a sample checkerboard optimized for the detection. However, any other checkerboard
	/// can be used as well.
	///
	/// Use the `generate_pattern.py` Python script ([tutorial_camera_calibration_pattern])
	/// to create the corresponding checkerboard pattern:
	/// \image html pics/checkerboard_radon.png width=60%
	///
	/// ## Overloaded parameters
	///
	/// ## C++ default parameters
	/// * flags: 0
	#[inline]
	pub fn find_chessboard_corners_sb(image: &impl ToInputArray, pattern_size: core::Size, corners: &mut impl ToOutputArray, flags: i32) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(corners);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findChessboardCornersSB_const__InputArrayR_Size_const__OutputArrayR_int(image.as_raw__InputArray(), &pattern_size, corners.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds the positions of internal corners of the chessboard using a sector based approach.
	///
	/// ## Parameters
	/// * image: Source chessboard view. It must be an 8-bit grayscale or color image.
	/// * patternSize: Number of inner corners per a chessboard row and column
	/// ( patternSize = cv::Size(points_per_row,points_per_column) = cv::Size(columns,rows) ).
	/// * corners: Output array of detected corners.
	/// * flags: Various operation flags that can be zero or a combination of the following values:
	/// *   [CALIB_CB_NORMALIZE_IMAGE] Normalize the image gamma with equalizeHist before detection.
	/// *   [CALIB_CB_EXHAUSTIVE] Run an exhaustive search to improve detection rate.
	/// *   [CALIB_CB_ACCURACY] Up sample input image to improve sub-pixel accuracy due to aliasing effects.
	/// *   [CALIB_CB_LARGER] The detected pattern is allowed to be larger than patternSize (see description).
	/// *   [CALIB_CB_MARKER] The detected pattern must have a marker (see description).
	/// This should be used if an accurate camera calibration is required.
	/// * meta: Optional output array of detected corners (CV_8UC1 and size = cv::Size(columns,rows)).
	/// Each entry stands for one corner of the pattern and can have one of the following values:
	/// *   0 = no meta data attached
	/// *   1 = left-top corner of a black cell
	/// *   2 = left-top corner of a white cell
	/// *   3 = left-top corner of a black cell with a white marker dot
	/// *   4 = left-top corner of a white cell with a black marker dot (pattern origin in case of markers otherwise first corner)
	///
	/// The function is analog to [find_chessboard_corners] but uses a localized radon
	/// transformation approximated by box filters being more robust to all sort of
	/// noise, faster on larger images and is able to directly return the sub-pixel
	/// position of the internal chessboard corners. The Method is based on the paper
	/// [duda2018](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_duda2018) "Accurate Detection and Localization of Checkerboard Corners for
	/// Calibration" demonstrating that the returned sub-pixel positions are more
	/// accurate than the one returned by cornerSubPix allowing a precise camera
	/// calibration for demanding applications.
	///
	/// In the case, the flags [CALIB_CB_LARGER] or [CALIB_CB_MARKER] are given,
	/// the result can be recovered from the optional meta array. Both flags are
	/// helpful to use calibration patterns exceeding the field of view of the camera.
	/// These oversized patterns allow more accurate calibrations as corners can be
	/// utilized, which are as close as possible to the image borders.  For a
	/// consistent coordinate system across all images, the optional marker (see image
	/// below) can be used to move the origin of the board to the location where the
	/// black circle is located.
	///
	///
	/// Note: The function requires a white boarder with roughly the same width as one
	/// of the checkerboard fields around the whole board to improve the detection in
	/// various environments. In addition, because of the localized radon
	/// transformation it is beneficial to use round corners for the field corners
	/// which are located on the outside of the board. The following figure illustrates
	/// a sample checkerboard optimized for the detection. However, any other checkerboard
	/// can be used as well.
	///
	/// Use the `generate_pattern.py` Python script ([tutorial_camera_calibration_pattern])
	/// to create the corresponding checkerboard pattern:
	/// \image html pics/checkerboard_radon.png width=60%
	#[inline]
	pub fn find_chessboard_corners_sb_with_meta(image: &impl ToInputArray, pattern_size: core::Size, corners: &mut impl ToOutputArray, flags: i32, meta: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(corners);
		output_array_arg!(meta);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findChessboardCornersSB_const__InputArrayR_Size_const__OutputArrayR_int_const__OutputArrayR(image.as_raw__InputArray(), &pattern_size, corners.as_raw__OutputArray(), flags, meta.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds the positions of internal corners of the chessboard.
	///
	/// ## Parameters
	/// * image: Source chessboard view. It must be an 8-bit grayscale or color image.
	/// * patternSize: Number of inner corners per a chessboard row and column
	/// ( patternSize = cv::Size(points_per_row,points_per_column) = cv::Size(columns,rows) ).
	/// * corners: Output array of detected corners.
	/// * flags: Various operation flags that can be zero or a combination of the following values:
	/// *   [CALIB_CB_ADAPTIVE_THRESH] Use adaptive thresholding to convert the image to black
	/// and white, rather than a fixed threshold level (computed from the average image brightness).
	/// *   [CALIB_CB_NORMALIZE_IMAGE] Normalize the image gamma with equalizeHist before
	/// applying fixed or adaptive thresholding.
	/// *   [CALIB_CB_FILTER_QUADS] Use additional criteria (like contour area, perimeter,
	/// square-like shape) to filter out false quads extracted at the contour retrieval stage.
	/// *   [CALIB_CB_FAST_CHECK] Run a fast check on the image that looks for chessboard corners,
	/// and shortcut the call if none is found. This can drastically speed up the call in the
	/// degenerate condition when no chessboard is observed.
	/// *   [CALIB_CB_PLAIN] All other flags are ignored. The input image is taken as is.
	/// No image processing is done to improve to find the checkerboard. This has the effect of speeding up the
	/// execution of the function but could lead to not recognizing the checkerboard if the image
	/// is not previously binarized in the appropriate manner.
	///
	/// ## Returns
	/// True if all of the corners are found and placed in a certain order (row by row,
	/// left to right in every row). Otherwise, if the function fails to find all the corners or reorder them,
	/// it returns false.
	///
	/// The function attempts to determine whether the input image is a view of the chessboard pattern and
	/// locate the internal chessboard corners. For example, a regular chessboard has 8 x 8 squares and
	/// 7 x 7 internal corners, that is, points where the black squares touch each other. The detected
	/// coordinates are approximate, and to determine their positions more accurately, the function
	/// calls #cornerSubPix. You also may use the function [corner_sub_pix] with different parameters if
	/// returned coordinates are not accurate enough.
	///
	/// Sample usage of detecting and drawing chessboard corners: :
	/// ```C++
	///    Size patternsize(8,6); //interior number of corners
	///    Mat gray = ....; //source image
	///    vector<Point2f> corners; //this will be filled by the detected corners
	///
	///    //CALIB_CB_FAST_CHECK saves a lot of time on images
	///    //that do not contain any chessboard corners
	///    bool patternfound = findChessboardCorners(gray, patternsize, corners,
	///            CALIB_CB_ADAPTIVE_THRESH + CALIB_CB_NORMALIZE_IMAGE
	///            + CALIB_CB_FAST_CHECK);
	///
	///    if(patternfound)
	///       cornerSubPix(gray, corners, Size(11, 11), Size(-1, -1),
	///        TermCriteria(CV_TERMCRIT_EPS + CV_TERMCRIT_ITER, 30, 0.1));
	///
	///    drawChessboardCorners(img, patternsize, Mat(corners), patternfound);
	/// ```
	///
	///
	/// Note: The function requires white space (like a square-thick border, the wider the better) around
	/// the board to make the detection more robust in various environments. Otherwise, if there is no
	/// border and the background is dark, the outer black squares cannot be segmented properly and so the
	/// square grouping and ordering algorithm fails.
	///
	/// Use the `generate_pattern.py` Python script ([tutorial_camera_calibration_pattern])
	/// to create the desired checkerboard pattern.
	///
	/// ## Note
	/// This alternative version of [find_chessboard_corners] function uses the following default values for its arguments:
	/// * flags: CALIB_CB_ADAPTIVE_THRESH+CALIB_CB_NORMALIZE_IMAGE
	#[inline]
	pub fn find_chessboard_corners_def(image: &impl ToInputArray, pattern_size: core::Size, corners: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(corners);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findChessboardCorners_const__InputArrayR_Size_const__OutputArrayR(image.as_raw__InputArray(), &pattern_size, corners.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds the positions of internal corners of the chessboard.
	///
	/// ## Parameters
	/// * image: Source chessboard view. It must be an 8-bit grayscale or color image.
	/// * patternSize: Number of inner corners per a chessboard row and column
	/// ( patternSize = cv::Size(points_per_row,points_per_column) = cv::Size(columns,rows) ).
	/// * corners: Output array of detected corners.
	/// * flags: Various operation flags that can be zero or a combination of the following values:
	/// *   [CALIB_CB_ADAPTIVE_THRESH] Use adaptive thresholding to convert the image to black
	/// and white, rather than a fixed threshold level (computed from the average image brightness).
	/// *   [CALIB_CB_NORMALIZE_IMAGE] Normalize the image gamma with equalizeHist before
	/// applying fixed or adaptive thresholding.
	/// *   [CALIB_CB_FILTER_QUADS] Use additional criteria (like contour area, perimeter,
	/// square-like shape) to filter out false quads extracted at the contour retrieval stage.
	/// *   [CALIB_CB_FAST_CHECK] Run a fast check on the image that looks for chessboard corners,
	/// and shortcut the call if none is found. This can drastically speed up the call in the
	/// degenerate condition when no chessboard is observed.
	/// *   [CALIB_CB_PLAIN] All other flags are ignored. The input image is taken as is.
	/// No image processing is done to improve to find the checkerboard. This has the effect of speeding up the
	/// execution of the function but could lead to not recognizing the checkerboard if the image
	/// is not previously binarized in the appropriate manner.
	///
	/// ## Returns
	/// True if all of the corners are found and placed in a certain order (row by row,
	/// left to right in every row). Otherwise, if the function fails to find all the corners or reorder them,
	/// it returns false.
	///
	/// The function attempts to determine whether the input image is a view of the chessboard pattern and
	/// locate the internal chessboard corners. For example, a regular chessboard has 8 x 8 squares and
	/// 7 x 7 internal corners, that is, points where the black squares touch each other. The detected
	/// coordinates are approximate, and to determine their positions more accurately, the function
	/// calls #cornerSubPix. You also may use the function [corner_sub_pix] with different parameters if
	/// returned coordinates are not accurate enough.
	///
	/// Sample usage of detecting and drawing chessboard corners: :
	/// ```C++
	///    Size patternsize(8,6); //interior number of corners
	///    Mat gray = ....; //source image
	///    vector<Point2f> corners; //this will be filled by the detected corners
	///
	///    //CALIB_CB_FAST_CHECK saves a lot of time on images
	///    //that do not contain any chessboard corners
	///    bool patternfound = findChessboardCorners(gray, patternsize, corners,
	///            CALIB_CB_ADAPTIVE_THRESH + CALIB_CB_NORMALIZE_IMAGE
	///            + CALIB_CB_FAST_CHECK);
	///
	///    if(patternfound)
	///       cornerSubPix(gray, corners, Size(11, 11), Size(-1, -1),
	///        TermCriteria(CV_TERMCRIT_EPS + CV_TERMCRIT_ITER, 30, 0.1));
	///
	///    drawChessboardCorners(img, patternsize, Mat(corners), patternfound);
	/// ```
	///
	///
	/// Note: The function requires white space (like a square-thick border, the wider the better) around
	/// the board to make the detection more robust in various environments. Otherwise, if there is no
	/// border and the background is dark, the outer black squares cannot be segmented properly and so the
	/// square grouping and ordering algorithm fails.
	///
	/// Use the `generate_pattern.py` Python script ([tutorial_camera_calibration_pattern])
	/// to create the desired checkerboard pattern.
	///
	/// ## C++ default parameters
	/// * flags: CALIB_CB_ADAPTIVE_THRESH+CALIB_CB_NORMALIZE_IMAGE
	#[inline]
	pub fn find_chessboard_corners(image: &impl ToInputArray, pattern_size: core::Size, corners: &mut impl ToOutputArray, flags: i32) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(corners);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findChessboardCorners_const__InputArrayR_Size_const__OutputArrayR_int(image.as_raw__InputArray(), &pattern_size, corners.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds centers in the grid of circles.
	///
	/// ## Parameters
	/// * image: grid view of input circles; it must be an 8-bit grayscale or color image.
	/// * patternSize: number of circles per row and column
	/// ( patternSize = Size(points_per_row, points_per_column) ).
	/// * centers: output array of detected centers.
	/// * flags: various operation flags that can be one of the following values:
	/// *   [CALIB_CB_SYMMETRIC_GRID] uses symmetric pattern of circles.
	/// *   [CALIB_CB_ASYMMETRIC_GRID] uses asymmetric pattern of circles.
	/// *   [CALIB_CB_CLUSTERING] uses a special algorithm for grid detection. It is more robust to
	/// perspective distortions but much more sensitive to background clutter.
	/// * blobDetector: feature detector that finds blobs like dark circles on light background.
	///                    If `blobDetector` is NULL then `image` represents Point2f array of candidates.
	/// * parameters: struct for finding circles in a grid pattern.
	///
	/// return True if all of the centers have been found and they have been placed in a certain order
	/// (row by row, left to right in every row). Otherwise, if the function fails to find all the corners
	/// or reorder them, it returns false.
	///
	/// The function attempts to determine whether the input image contains a grid of circles. If it is, the
	/// function locates centers of the circles.
	///
	/// Sample usage of detecting and drawing the centers of circles: :
	/// ```C++
	///    Size patternsize(7,7); //number of centers
	///    Mat gray = ...; //source image
	///    vector<Point2f> centers; //this will be filled by the detected centers
	///
	///    bool patternfound = findCirclesGrid(gray, patternsize, centers);
	///
	///    drawChessboardCorners(img, patternsize, Mat(centers), patternfound);
	/// ```
	///
	///
	/// Note: The function requires white space (like a square-thick border, the wider the better) around
	/// the board to make the detection more robust in various environments.
	///
	/// ## Overloaded parameters
	///
	///
	/// ## Note
	/// This alternative version of [find_circles_grid] function uses the following default values for its arguments:
	/// * flags: CALIB_CB_SYMMETRIC_GRID
	/// * blob_detector: cv::SimpleBlobDetector::create()
	#[inline]
	pub fn find_circles_grid_def(image: &impl ToInputArray, pattern_size: core::Size, centers: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(centers);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findCirclesGrid_const__InputArrayR_Size_const__OutputArrayR(image.as_raw__InputArray(), &pattern_size, centers.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds centers in the grid of circles.
	///
	/// ## Parameters
	/// * image: grid view of input circles; it must be an 8-bit grayscale or color image.
	/// * patternSize: number of circles per row and column
	/// ( patternSize = Size(points_per_row, points_per_column) ).
	/// * centers: output array of detected centers.
	/// * flags: various operation flags that can be one of the following values:
	/// *   [CALIB_CB_SYMMETRIC_GRID] uses symmetric pattern of circles.
	/// *   [CALIB_CB_ASYMMETRIC_GRID] uses asymmetric pattern of circles.
	/// *   [CALIB_CB_CLUSTERING] uses a special algorithm for grid detection. It is more robust to
	/// perspective distortions but much more sensitive to background clutter.
	/// * blobDetector: feature detector that finds blobs like dark circles on light background.
	///                    If `blobDetector` is NULL then `image` represents Point2f array of candidates.
	/// * parameters: struct for finding circles in a grid pattern.
	///
	/// return True if all of the centers have been found and they have been placed in a certain order
	/// (row by row, left to right in every row). Otherwise, if the function fails to find all the corners
	/// or reorder them, it returns false.
	///
	/// The function attempts to determine whether the input image contains a grid of circles. If it is, the
	/// function locates centers of the circles.
	///
	/// Sample usage of detecting and drawing the centers of circles: :
	/// ```C++
	///    Size patternsize(7,7); //number of centers
	///    Mat gray = ...; //source image
	///    vector<Point2f> centers; //this will be filled by the detected centers
	///
	///    bool patternfound = findCirclesGrid(gray, patternsize, centers);
	///
	///    drawChessboardCorners(img, patternsize, Mat(centers), patternfound);
	/// ```
	///
	///
	/// Note: The function requires white space (like a square-thick border, the wider the better) around
	/// the board to make the detection more robust in various environments.
	///
	/// ## Overloaded parameters
	///
	/// ## C++ default parameters
	/// * flags: CALIB_CB_SYMMETRIC_GRID
	/// * blob_detector: cv::SimpleBlobDetector::create()
	#[inline]
	pub fn find_circles_grid(image: &impl ToInputArray, pattern_size: core::Size, centers: &mut impl ToOutputArray, flags: i32, blob_detector: Option<&core::Ptr<crate::features::Feature2D>>) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(centers);
		smart_ptr_option_arg!(ref blob_detector);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findCirclesGrid_const__InputArrayR_Size_const__OutputArrayR_int_const_PtrLFeature2DGR(image.as_raw__InputArray(), &pattern_size, centers.as_raw__OutputArray(), flags, blob_detector.as_raw_PtrOfFeature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds centers in the grid of circles.
	///
	/// ## Parameters
	/// * image: grid view of input circles; it must be an 8-bit grayscale or color image.
	/// * patternSize: number of circles per row and column
	/// ( patternSize = Size(points_per_row, points_per_column) ).
	/// * centers: output array of detected centers.
	/// * flags: various operation flags that can be one of the following values:
	/// *   [CALIB_CB_SYMMETRIC_GRID] uses symmetric pattern of circles.
	/// *   [CALIB_CB_ASYMMETRIC_GRID] uses asymmetric pattern of circles.
	/// *   [CALIB_CB_CLUSTERING] uses a special algorithm for grid detection. It is more robust to
	/// perspective distortions but much more sensitive to background clutter.
	/// * blobDetector: feature detector that finds blobs like dark circles on light background.
	///                    If `blobDetector` is NULL then `image` represents Point2f array of candidates.
	/// * parameters: struct for finding circles in a grid pattern.
	///
	/// return True if all of the centers have been found and they have been placed in a certain order
	/// (row by row, left to right in every row). Otherwise, if the function fails to find all the corners
	/// or reorder them, it returns false.
	///
	/// The function attempts to determine whether the input image contains a grid of circles. If it is, the
	/// function locates centers of the circles.
	///
	/// Sample usage of detecting and drawing the centers of circles: :
	/// ```C++
	///    Size patternsize(7,7); //number of centers
	///    Mat gray = ...; //source image
	///    vector<Point2f> centers; //this will be filled by the detected centers
	///
	///    bool patternfound = findCirclesGrid(gray, patternsize, centers);
	///
	///    drawChessboardCorners(img, patternsize, Mat(centers), patternfound);
	/// ```
	///
	///
	/// Note: The function requires white space (like a square-thick border, the wider the better) around
	/// the board to make the detection more robust in various environments.
	#[inline]
	pub fn find_circles_grid_with_params(image: &impl ToInputArray, pattern_size: core::Size, centers: &mut impl ToOutputArray, flags: i32, blob_detector: Option<&core::Ptr<crate::features::Feature2D>>, parameters: crate::objdetect::CirclesGridFinderParameters) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(centers);
		smart_ptr_option_arg!(ref blob_detector);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findCirclesGrid_const__InputArrayR_Size_const__OutputArrayR_int_const_PtrLFeature2DGR_const_CirclesGridFinderParametersR(image.as_raw__InputArray(), &pattern_size, centers.as_raw__OutputArray(), flags, blob_detector.as_raw_PtrOfFeature2D(), &parameters, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq)]
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
		pub grid_type: crate::objdetect::CirclesGridFinderParameters_GridType,
		/// Distance between two adjacent points. Used by CALIB_CB_CLUSTERING.
		pub square_size: f32,
		/// Max deviation from prediction. Used by CALIB_CB_CLUSTERING.
		pub max_rectified_distance: f32,
	}

	opencv_type_simple! { crate::objdetect::CirclesGridFinderParameters }

	impl CirclesGridFinderParameters {
		#[inline]
		pub fn default() -> Result<crate::objdetect::CirclesGridFinderParameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_CirclesGridFinderParameters_CirclesGridFinderParameters(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// DNN-based face detector
	///
	/// model download link: <https://github.com/opencv/opencv_zoo/tree/master/models/face_detection_yunet>
	pub struct FaceDetectorYN {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { FaceDetectorYN }

	impl Drop for FaceDetectorYN {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_FaceDetectorYN_delete(self.as_raw_mut_FaceDetectorYN()) };
		}
	}

	unsafe impl Send for FaceDetectorYN {}

	impl FaceDetectorYN {
		/// Creates an instance of face detector class with given parameters
		///
		/// ## Parameters
		/// * model: the path to the requested model
		/// * config: the path to the config file for compatibility, which is not requested for ONNX models
		/// * input_size: the size of the input image
		/// * score_threshold: the threshold to filter out bounding boxes of score smaller than the given value
		/// * nms_threshold: the threshold to suppress bounding boxes of IoU bigger than the given value
		/// * top_k: keep top K bboxes before NMS
		/// * backend_id: the id of backend
		/// * target_id: the id of target device
		///
		/// ## C++ default parameters
		/// * score_threshold: 0.9f
		/// * nms_threshold: 0.3f
		/// * top_k: 5000
		/// * backend_id: 0
		/// * target_id: 0
		#[inline]
		pub fn create(model: &str, config: &str, input_size: core::Size, score_threshold: f32, nms_threshold: f32, top_k: i32, backend_id: i32, target_id: i32) -> Result<core::Ptr<crate::objdetect::FaceDetectorYN>> {
			extern_container_arg!(model);
			extern_container_arg!(config);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceDetectorYN_create_const_StringR_const_StringR_const_SizeR_float_float_int_int_int(model.opencv_as_extern(), config.opencv_as_extern(), &input_size, score_threshold, nms_threshold, top_k, backend_id, target_id, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::objdetect::FaceDetectorYN>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Creates an instance of face detector class with given parameters
		///
		/// ## Parameters
		/// * model: the path to the requested model
		/// * config: the path to the config file for compatibility, which is not requested for ONNX models
		/// * input_size: the size of the input image
		/// * score_threshold: the threshold to filter out bounding boxes of score smaller than the given value
		/// * nms_threshold: the threshold to suppress bounding boxes of IoU bigger than the given value
		/// * top_k: keep top K bboxes before NMS
		/// * backend_id: the id of backend
		/// * target_id: the id of target device
		///
		/// ## Note
		/// This alternative version of [FaceDetectorYN::create] function uses the following default values for its arguments:
		/// * score_threshold: 0.9f
		/// * nms_threshold: 0.3f
		/// * top_k: 5000
		/// * backend_id: 0
		/// * target_id: 0
		#[inline]
		pub fn create_def(model: &str, config: &str, input_size: core::Size) -> Result<core::Ptr<crate::objdetect::FaceDetectorYN>> {
			extern_container_arg!(model);
			extern_container_arg!(config);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceDetectorYN_create_const_StringR_const_StringR_const_SizeR(model.opencv_as_extern(), config.opencv_as_extern(), &input_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::objdetect::FaceDetectorYN>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Creates an instance of face detector class with given parameters
		///
		/// ## Parameters
		/// * model: the path to the requested model
		/// * config: the path to the config file for compatibility, which is not requested for ONNX models
		/// * input_size: the size of the input image
		/// * score_threshold: the threshold to filter out bounding boxes of score smaller than the given value
		/// * nms_threshold: the threshold to suppress bounding boxes of IoU bigger than the given value
		/// * top_k: keep top K bboxes before NMS
		/// * backend_id: the id of backend
		/// * target_id: the id of target device
		///
		/// ## Overloaded parameters
		///
		///
		/// * framework: Name of origin framework
		/// * bufferModel: A buffer with a content of binary file with weights
		/// * bufferConfig: A buffer with a content of text file contains network configuration
		/// * input_size: the size of the input image
		/// * score_threshold: the threshold to filter out bounding boxes of score smaller than the given value
		/// * nms_threshold: the threshold to suppress bounding boxes of IoU bigger than the given value
		/// * top_k: keep top K bboxes before NMS
		/// * backend_id: the id of backend
		/// * target_id: the id of target device
		///
		/// ## C++ default parameters
		/// * score_threshold: 0.9f
		/// * nms_threshold: 0.3f
		/// * top_k: 5000
		/// * backend_id: 0
		/// * target_id: 0
		#[inline]
		pub fn create_1(framework: &str, buffer_model: &core::Vector<u8>, buffer_config: &core::Vector<u8>, input_size: core::Size, score_threshold: f32, nms_threshold: f32, top_k: i32, backend_id: i32, target_id: i32) -> Result<core::Ptr<crate::objdetect::FaceDetectorYN>> {
			extern_container_arg!(framework);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceDetectorYN_create_const_StringR_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR_const_SizeR_float_float_int_int_int(framework.opencv_as_extern(), buffer_model.as_raw_VectorOfu8(), buffer_config.as_raw_VectorOfu8(), &input_size, score_threshold, nms_threshold, top_k, backend_id, target_id, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::objdetect::FaceDetectorYN>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Creates an instance of face detector class with given parameters
		///
		/// ## Parameters
		/// * model: the path to the requested model
		/// * config: the path to the config file for compatibility, which is not requested for ONNX models
		/// * input_size: the size of the input image
		/// * score_threshold: the threshold to filter out bounding boxes of score smaller than the given value
		/// * nms_threshold: the threshold to suppress bounding boxes of IoU bigger than the given value
		/// * top_k: keep top K bboxes before NMS
		/// * backend_id: the id of backend
		/// * target_id: the id of target device
		///
		/// ## Overloaded parameters
		///
		///
		/// * framework: Name of origin framework
		/// * bufferModel: A buffer with a content of binary file with weights
		/// * bufferConfig: A buffer with a content of text file contains network configuration
		/// * input_size: the size of the input image
		/// * score_threshold: the threshold to filter out bounding boxes of score smaller than the given value
		/// * nms_threshold: the threshold to suppress bounding boxes of IoU bigger than the given value
		/// * top_k: keep top K bboxes before NMS
		/// * backend_id: the id of backend
		/// * target_id: the id of target device
		///
		/// ## Note
		/// This alternative version of [FaceDetectorYN::create] function uses the following default values for its arguments:
		/// * score_threshold: 0.9f
		/// * nms_threshold: 0.3f
		/// * top_k: 5000
		/// * backend_id: 0
		/// * target_id: 0
		#[inline]
		pub fn create_def_1(framework: &str, buffer_model: &core::Vector<u8>, buffer_config: &core::Vector<u8>, input_size: core::Size) -> Result<core::Ptr<crate::objdetect::FaceDetectorYN>> {
			extern_container_arg!(framework);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceDetectorYN_create_const_StringR_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR_const_SizeR(framework.opencv_as_extern(), buffer_model.as_raw_VectorOfu8(), buffer_config.as_raw_VectorOfu8(), &input_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::objdetect::FaceDetectorYN>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::objdetect::FaceDetectorYN]
	pub trait FaceDetectorYNTraitConst {
		fn as_raw_FaceDetectorYN(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::objdetect::FaceDetectorYN]
	pub trait FaceDetectorYNTrait: crate::objdetect::FaceDetectorYNTraitConst {
		fn as_raw_mut_FaceDetectorYN(&mut self) -> *mut c_void;

		/// Set the size for the network input, which overwrites the input size of creating model. Call this method when the size of input image does not match the input size when creating model
		///
		/// ## Parameters
		/// * input_size: the size of the input image
		#[inline]
		fn set_input_size(&mut self, input_size: core::Size) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceDetectorYN_setInputSize_const_SizeR(self.as_raw_mut_FaceDetectorYN(), &input_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_input_size(&mut self) -> Result<core::Size> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceDetectorYN_getInputSize(self.as_raw_mut_FaceDetectorYN(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set the score threshold to filter out bounding boxes of score less than the given value
		///
		/// ## Parameters
		/// * score_threshold: threshold for filtering out bounding boxes
		#[inline]
		fn set_score_threshold(&mut self, score_threshold: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceDetectorYN_setScoreThreshold_float(self.as_raw_mut_FaceDetectorYN(), score_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_score_threshold(&mut self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceDetectorYN_getScoreThreshold(self.as_raw_mut_FaceDetectorYN(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set the Non-maximum-suppression threshold to suppress bounding boxes that have IoU greater than the given value
		///
		/// ## Parameters
		/// * nms_threshold: threshold for NMS operation
		#[inline]
		fn set_nms_threshold(&mut self, nms_threshold: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceDetectorYN_setNMSThreshold_float(self.as_raw_mut_FaceDetectorYN(), nms_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_nms_threshold(&mut self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceDetectorYN_getNMSThreshold(self.as_raw_mut_FaceDetectorYN(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Set the number of bounding boxes preserved before NMS
		///
		/// ## Parameters
		/// * top_k: the number of bounding boxes to preserve from top rank based on score
		#[inline]
		fn set_top_k(&mut self, top_k: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceDetectorYN_setTopK_int(self.as_raw_mut_FaceDetectorYN(), top_k, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_top_k(&mut self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceDetectorYN_getTopK(self.as_raw_mut_FaceDetectorYN(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Detects faces in the input image. Following is an example output.
		///
		/// * ![image](https://docs.opencv.org/5.0.0/lena-face-detection.jpg)
		///
		/// ## Parameters
		/// * image: an image to detect
		/// * faces: detection results stored in a 2D cv::Mat of shape [num_faces, 15]
		/// *  - 0-1: x, y of bbox top left corner
		/// *  - 2-3: width, height of bbox
		/// *  - 4-5: x, y of right eye (blue point in the example image)
		/// *  - 6-7: x, y of left eye (red point in the example image)
		/// *  - 8-9: x, y of nose tip (green point in the example image)
		/// *  - 10-11: x, y of right corner of mouth (pink point in the example image)
		/// *  - 12-13: x, y of left corner of mouth (yellow point in the example image)
		/// *  - 14: face score
		#[inline]
		fn detect(&mut self, image: &impl ToInputArray, faces: &mut impl ToOutputArray) -> Result<i32> {
			input_array_arg!(image);
			output_array_arg!(faces);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceDetectorYN_detect_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_FaceDetectorYN(), image.as_raw__InputArray(), faces.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for FaceDetectorYN {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("FaceDetectorYN")
				.finish()
		}
	}

	impl crate::objdetect::FaceDetectorYNTraitConst for FaceDetectorYN {
		#[inline] fn as_raw_FaceDetectorYN(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::objdetect::FaceDetectorYNTrait for FaceDetectorYN {
		#[inline] fn as_raw_mut_FaceDetectorYN(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { FaceDetectorYN, crate::objdetect::FaceDetectorYNTraitConst, as_raw_FaceDetectorYN, crate::objdetect::FaceDetectorYNTrait, as_raw_mut_FaceDetectorYN }

	/// DNN-based face recognizer
	///
	/// model download link: <https://github.com/opencv/opencv_zoo/tree/master/models/face_recognition_sface>
	pub struct FaceRecognizerSF {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { FaceRecognizerSF }

	impl Drop for FaceRecognizerSF {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_FaceRecognizerSF_delete(self.as_raw_mut_FaceRecognizerSF()) };
		}
	}

	unsafe impl Send for FaceRecognizerSF {}

	impl FaceRecognizerSF {
		/// Creates an instance of this class with given parameters
		/// ## Parameters
		/// * model: the path of the onnx model used for face recognition
		/// * config: the path to the config file for compatibility, which is not requested for ONNX models
		/// * backend_id: the id of backend
		/// * target_id: the id of target device
		///
		/// ## C++ default parameters
		/// * backend_id: 0
		/// * target_id: 0
		#[inline]
		pub fn create(model: &str, config: &str, backend_id: i32, target_id: i32) -> Result<core::Ptr<crate::objdetect::FaceRecognizerSF>> {
			extern_container_arg!(model);
			extern_container_arg!(config);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceRecognizerSF_create_const_StringR_const_StringR_int_int(model.opencv_as_extern(), config.opencv_as_extern(), backend_id, target_id, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::objdetect::FaceRecognizerSF>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Creates an instance of this class with given parameters
		/// ## Parameters
		/// * model: the path of the onnx model used for face recognition
		/// * config: the path to the config file for compatibility, which is not requested for ONNX models
		/// * backend_id: the id of backend
		/// * target_id: the id of target device
		///
		/// ## Note
		/// This alternative version of [FaceRecognizerSF::create] function uses the following default values for its arguments:
		/// * backend_id: 0
		/// * target_id: 0
		#[inline]
		pub fn create_def(model: &str, config: &str) -> Result<core::Ptr<crate::objdetect::FaceRecognizerSF>> {
			extern_container_arg!(model);
			extern_container_arg!(config);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceRecognizerSF_create_const_StringR_const_StringR(model.opencv_as_extern(), config.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::objdetect::FaceRecognizerSF>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Creates an instance of this class from a buffer containing the model weights and configuration.
		/// ## Parameters
		/// * framework: Name of the framework (ONNX, etc.)
		/// * bufferModel: A buffer containing the binary model weights.
		/// * bufferConfig: A buffer containing the network configuration.
		/// * backend_id: The id of the backend.
		/// * target_id: The id of the target device.
		///
		/// ## Returns
		/// A pointer to the created instance of FaceRecognizerSF.
		///
		/// ## C++ default parameters
		/// * backend_id: 0
		/// * target_id: 0
		#[inline]
		pub fn create_1(framework: &str, buffer_model: &core::Vector<u8>, buffer_config: &core::Vector<u8>, backend_id: i32, target_id: i32) -> Result<core::Ptr<crate::objdetect::FaceRecognizerSF>> {
			extern_container_arg!(framework);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceRecognizerSF_create_const_StringR_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR_int_int(framework.opencv_as_extern(), buffer_model.as_raw_VectorOfu8(), buffer_config.as_raw_VectorOfu8(), backend_id, target_id, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::objdetect::FaceRecognizerSF>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Creates an instance of this class from a buffer containing the model weights and configuration.
		/// ## Parameters
		/// * framework: Name of the framework (ONNX, etc.)
		/// * bufferModel: A buffer containing the binary model weights.
		/// * bufferConfig: A buffer containing the network configuration.
		/// * backend_id: The id of the backend.
		/// * target_id: The id of the target device.
		///
		/// ## Returns
		/// A pointer to the created instance of FaceRecognizerSF.
		///
		/// ## Note
		/// This alternative version of [FaceRecognizerSF::create] function uses the following default values for its arguments:
		/// * backend_id: 0
		/// * target_id: 0
		#[inline]
		pub fn create_def_1(framework: &str, buffer_model: &core::Vector<u8>, buffer_config: &core::Vector<u8>) -> Result<core::Ptr<crate::objdetect::FaceRecognizerSF>> {
			extern_container_arg!(framework);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceRecognizerSF_create_const_StringR_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(framework.opencv_as_extern(), buffer_model.as_raw_VectorOfu8(), buffer_config.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::objdetect::FaceRecognizerSF>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::objdetect::FaceRecognizerSF]
	pub trait FaceRecognizerSFTraitConst {
		fn as_raw_FaceRecognizerSF(&self) -> *const c_void;

		/// Aligns detected face with the source input image and crops it
		/// ## Parameters
		/// * src_img: input image
		/// * face_box: the detected face result from the input image
		/// * aligned_img: output aligned image
		#[inline]
		fn align_crop(&self, src_img: &impl ToInputArray, face_box: &impl ToInputArray, aligned_img: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(src_img);
			input_array_arg!(face_box);
			output_array_arg!(aligned_img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceRecognizerSF_alignCrop_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_FaceRecognizerSF(), src_img.as_raw__InputArray(), face_box.as_raw__InputArray(), aligned_img.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Calculates the distance between two face features
		/// ## Parameters
		/// * face_feature1: the first input feature
		/// * face_feature2: the second input feature of the same size and the same type as face_feature1
		/// * dis_type: defines how to calculate the distance between two face features with optional values "FR_COSINE" or "FR_NORM_L2"
		///
		/// ## C++ default parameters
		/// * dis_type: FaceRecognizerSF::FR_COSINE
		#[inline]
		fn match_(&self, face_feature1: &impl ToInputArray, face_feature2: &impl ToInputArray, dis_type: i32) -> Result<f64> {
			input_array_arg!(face_feature1);
			input_array_arg!(face_feature2);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceRecognizerSF_match_const_const__InputArrayR_const__InputArrayR_int(self.as_raw_FaceRecognizerSF(), face_feature1.as_raw__InputArray(), face_feature2.as_raw__InputArray(), dis_type, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Calculates the distance between two face features
		/// ## Parameters
		/// * face_feature1: the first input feature
		/// * face_feature2: the second input feature of the same size and the same type as face_feature1
		/// * dis_type: defines how to calculate the distance between two face features with optional values "FR_COSINE" or "FR_NORM_L2"
		///
		/// ## Note
		/// This alternative version of [FaceRecognizerSFTraitConst::match_] function uses the following default values for its arguments:
		/// * dis_type: FaceRecognizerSF::FR_COSINE
		#[inline]
		fn match__def(&self, face_feature1: &impl ToInputArray, face_feature2: &impl ToInputArray) -> Result<f64> {
			input_array_arg!(face_feature1);
			input_array_arg!(face_feature2);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceRecognizerSF_match_const_const__InputArrayR_const__InputArrayR(self.as_raw_FaceRecognizerSF(), face_feature1.as_raw__InputArray(), face_feature2.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::objdetect::FaceRecognizerSF]
	pub trait FaceRecognizerSFTrait: crate::objdetect::FaceRecognizerSFTraitConst {
		fn as_raw_mut_FaceRecognizerSF(&mut self) -> *mut c_void;

		/// Extracts face feature from aligned image
		/// ## Parameters
		/// * aligned_img: input aligned image
		/// * face_feature: output face feature
		#[inline]
		fn feature(&mut self, aligned_img: &impl ToInputArray, face_feature: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(aligned_img);
			output_array_arg!(face_feature);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceRecognizerSF_feature_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_FaceRecognizerSF(), aligned_img.as_raw__InputArray(), face_feature.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for FaceRecognizerSF {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("FaceRecognizerSF")
				.finish()
		}
	}

	impl crate::objdetect::FaceRecognizerSFTraitConst for FaceRecognizerSF {
		#[inline] fn as_raw_FaceRecognizerSF(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::objdetect::FaceRecognizerSFTrait for FaceRecognizerSF {
		#[inline] fn as_raw_mut_FaceRecognizerSF(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { FaceRecognizerSF, crate::objdetect::FaceRecognizerSFTraitConst, as_raw_FaceRecognizerSF, crate::objdetect::FaceRecognizerSFTrait, as_raw_mut_FaceRecognizerSF }

	pub struct GraphicalCodeDetector {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { GraphicalCodeDetector }

	impl Drop for GraphicalCodeDetector {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_GraphicalCodeDetector_delete(self.as_raw_mut_GraphicalCodeDetector()) };
		}
	}

	unsafe impl Send for GraphicalCodeDetector {}

	impl GraphicalCodeDetector {
		#[inline]
		pub fn default() -> Result<crate::objdetect::GraphicalCodeDetector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GraphicalCodeDetector_GraphicalCodeDetector(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::GraphicalCodeDetector::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		pub fn copy(unnamed: &impl crate::objdetect::GraphicalCodeDetectorTraitConst) -> crate::objdetect::GraphicalCodeDetector {
			let ret = unsafe { sys::cv_GraphicalCodeDetector_GraphicalCodeDetector_const_GraphicalCodeDetectorR(unnamed.as_raw_GraphicalCodeDetector()) };
			let ret = unsafe { crate::objdetect::GraphicalCodeDetector::opencv_from_extern(ret) };
			ret
		}

		#[inline]
		pub fn copy_mut(mut unnamed: crate::objdetect::GraphicalCodeDetector) -> crate::objdetect::GraphicalCodeDetector {
			let ret = unsafe { sys::cv_GraphicalCodeDetector_GraphicalCodeDetector_GraphicalCodeDetectorRR(unnamed.as_raw_mut_GraphicalCodeDetector()) };
			let ret = unsafe { crate::objdetect::GraphicalCodeDetector::opencv_from_extern(ret) };
			ret
		}

	}

	/// Constant methods for [crate::objdetect::GraphicalCodeDetector]
	pub trait GraphicalCodeDetectorTraitConst {
		fn as_raw_GraphicalCodeDetector(&self) -> *const c_void;

		/// Detects graphical code in image and returns the quadrangle containing the code.
		/// ## Parameters
		/// * img: grayscale or color (BGR) image containing (or not) graphical code.
		/// * points: Output vector of vertices of the minimum-area quadrangle containing the code.
		#[inline]
		fn detect(&self, img: &impl ToInputArray, points: &mut impl ToOutputArray) -> Result<bool> {
			input_array_arg!(img);
			output_array_arg!(points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GraphicalCodeDetector_detect_const_const__InputArrayR_const__OutputArrayR(self.as_raw_GraphicalCodeDetector(), img.as_raw__InputArray(), points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Decodes graphical code in image once it's found by the detect() method.
		///
		/// Returns UTF8-encoded output string or empty string if the code cannot be decoded.
		/// ## Parameters
		/// * img: grayscale or color (BGR) image containing graphical code.
		/// * points: Quadrangle vertices found by detect() method (or some other algorithm).
		/// * straight_code: The optional output image containing binarized code, will be empty if not found.
		///
		/// ## C++ default parameters
		/// * straight_code: noArray()
		#[inline]
		fn decode(&self, img: &impl ToInputArray, points: &impl ToInputArray, straight_code: &mut impl ToOutputArray) -> Result<Vec<u8>> {
			input_array_arg!(img);
			input_array_arg!(points);
			output_array_arg!(straight_code);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GraphicalCodeDetector_decode_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_GraphicalCodeDetector(), img.as_raw__InputArray(), points.as_raw__InputArray(), straight_code.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { Vec::<u8>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Decodes graphical code in image once it's found by the detect() method.
		///
		/// Returns UTF8-encoded output string or empty string if the code cannot be decoded.
		/// ## Parameters
		/// * img: grayscale or color (BGR) image containing graphical code.
		/// * points: Quadrangle vertices found by detect() method (or some other algorithm).
		/// * straight_code: The optional output image containing binarized code, will be empty if not found.
		///
		/// ## Note
		/// This alternative version of [GraphicalCodeDetectorTraitConst::decode] function uses the following default values for its arguments:
		/// * straight_code: noArray()
		#[inline]
		fn decode_def(&self, img: &impl ToInputArray, points: &impl ToInputArray) -> Result<Vec<u8>> {
			input_array_arg!(img);
			input_array_arg!(points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GraphicalCodeDetector_decode_const_const__InputArrayR_const__InputArrayR(self.as_raw_GraphicalCodeDetector(), img.as_raw__InputArray(), points.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { Vec::<u8>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Both detects and decodes graphical code
		///
		/// ## Parameters
		/// * img: grayscale or color (BGR) image containing graphical code.
		/// * points: optional output array of vertices of the found graphical code quadrangle, will be empty if not found.
		/// * straight_code: The optional output image containing binarized code
		///
		/// ## C++ default parameters
		/// * points: noArray()
		/// * straight_code: noArray()
		#[inline]
		fn detect_and_decode(&self, img: &impl ToInputArray, points: &mut impl ToOutputArray, straight_code: &mut impl ToOutputArray) -> Result<Vec<u8>> {
			input_array_arg!(img);
			output_array_arg!(points);
			output_array_arg!(straight_code);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GraphicalCodeDetector_detectAndDecode_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_GraphicalCodeDetector(), img.as_raw__InputArray(), points.as_raw__OutputArray(), straight_code.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { Vec::<u8>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Both detects and decodes graphical code
		///
		/// ## Parameters
		/// * img: grayscale or color (BGR) image containing graphical code.
		/// * points: optional output array of vertices of the found graphical code quadrangle, will be empty if not found.
		/// * straight_code: The optional output image containing binarized code
		///
		/// ## Note
		/// This alternative version of [GraphicalCodeDetectorTraitConst::detect_and_decode] function uses the following default values for its arguments:
		/// * points: noArray()
		/// * straight_code: noArray()
		#[inline]
		fn detect_and_decode_def(&self, img: &impl ToInputArray) -> Result<Vec<u8>> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GraphicalCodeDetector_detectAndDecode_const_const__InputArrayR(self.as_raw_GraphicalCodeDetector(), img.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { Vec::<u8>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Detects graphical codes in image and returns the vector of the quadrangles containing the codes.
		/// ## Parameters
		/// * img: grayscale or color (BGR) image containing (or not) graphical codes.
		/// * points: Output vector of vector of vertices of the minimum-area quadrangle containing the codes.
		#[inline]
		fn detect_multi(&self, img: &impl ToInputArray, points: &mut impl ToOutputArray) -> Result<bool> {
			input_array_arg!(img);
			output_array_arg!(points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GraphicalCodeDetector_detectMulti_const_const__InputArrayR_const__OutputArrayR(self.as_raw_GraphicalCodeDetector(), img.as_raw__InputArray(), points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Decodes graphical codes in image once it's found by the detect() method.
		/// ## Parameters
		/// * img: grayscale or color (BGR) image containing graphical codes.
		/// * decoded_info: UTF8-encoded output vector of string or empty vector of string if the codes cannot be decoded.
		/// * points: vector of Quadrangle vertices found by detect() method (or some other algorithm).
		/// * straight_code: The optional output vector of images containing binarized codes
		///
		/// ## C++ default parameters
		/// * straight_code: noArray()
		#[inline]
		fn decode_multi(&self, img: &impl ToInputArray, points: &impl ToInputArray, decoded_info: &mut core::Vector<String>, straight_code: &mut impl ToOutputArray) -> Result<bool> {
			input_array_arg!(img);
			input_array_arg!(points);
			output_array_arg!(straight_code);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GraphicalCodeDetector_decodeMulti_const_const__InputArrayR_const__InputArrayR_vectorLstringGR_const__OutputArrayR(self.as_raw_GraphicalCodeDetector(), img.as_raw__InputArray(), points.as_raw__InputArray(), decoded_info.as_raw_mut_VectorOfString(), straight_code.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Decodes graphical codes in image once it's found by the detect() method.
		/// ## Parameters
		/// * img: grayscale or color (BGR) image containing graphical codes.
		/// * decoded_info: UTF8-encoded output vector of string or empty vector of string if the codes cannot be decoded.
		/// * points: vector of Quadrangle vertices found by detect() method (or some other algorithm).
		/// * straight_code: The optional output vector of images containing binarized codes
		///
		/// ## Note
		/// This alternative version of [GraphicalCodeDetectorTraitConst::decode_multi] function uses the following default values for its arguments:
		/// * straight_code: noArray()
		#[inline]
		fn decode_multi_def(&self, img: &impl ToInputArray, points: &impl ToInputArray, decoded_info: &mut core::Vector<String>) -> Result<bool> {
			input_array_arg!(img);
			input_array_arg!(points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GraphicalCodeDetector_decodeMulti_const_const__InputArrayR_const__InputArrayR_vectorLstringGR(self.as_raw_GraphicalCodeDetector(), img.as_raw__InputArray(), points.as_raw__InputArray(), decoded_info.as_raw_mut_VectorOfString(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Both detects and decodes graphical codes
		/// ## Parameters
		/// * img: grayscale or color (BGR) image containing graphical codes.
		/// * decoded_info: UTF8-encoded output vector of string or empty vector of string if the codes cannot be decoded.
		/// * points: optional output vector of vertices of the found graphical code quadrangles. Will be empty if not found.
		/// * straight_code: The optional vector of images containing binarized codes
		///
		/// - If there are QR codes encoded with a Structured Append mode on the image and all of them detected and decoded correctly,
		/// method writes a full message to position corresponds to 0-th code in a sequence. The rest of QR codes from the same sequence
		/// have empty string.
		///
		/// ## C++ default parameters
		/// * points: noArray()
		/// * straight_code: noArray()
		#[inline]
		fn detect_and_decode_multi(&self, img: &impl ToInputArray, decoded_info: &mut core::Vector<String>, points: &mut impl ToOutputArray, straight_code: &mut impl ToOutputArray) -> Result<bool> {
			input_array_arg!(img);
			output_array_arg!(points);
			output_array_arg!(straight_code);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GraphicalCodeDetector_detectAndDecodeMulti_const_const__InputArrayR_vectorLstringGR_const__OutputArrayR_const__OutputArrayR(self.as_raw_GraphicalCodeDetector(), img.as_raw__InputArray(), decoded_info.as_raw_mut_VectorOfString(), points.as_raw__OutputArray(), straight_code.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Both detects and decodes graphical codes
		/// ## Parameters
		/// * img: grayscale or color (BGR) image containing graphical codes.
		/// * decoded_info: UTF8-encoded output vector of string or empty vector of string if the codes cannot be decoded.
		/// * points: optional output vector of vertices of the found graphical code quadrangles. Will be empty if not found.
		/// * straight_code: The optional vector of images containing binarized codes
		///
		/// - If there are QR codes encoded with a Structured Append mode on the image and all of them detected and decoded correctly,
		/// method writes a full message to position corresponds to 0-th code in a sequence. The rest of QR codes from the same sequence
		/// have empty string.
		///
		/// ## Note
		/// This alternative version of [GraphicalCodeDetectorTraitConst::detect_and_decode_multi] function uses the following default values for its arguments:
		/// * points: noArray()
		/// * straight_code: noArray()
		#[inline]
		fn detect_and_decode_multi_def(&self, img: &impl ToInputArray, decoded_info: &mut core::Vector<String>) -> Result<bool> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GraphicalCodeDetector_detectAndDecodeMulti_const_const__InputArrayR_vectorLstringGR(self.as_raw_GraphicalCodeDetector(), img.as_raw__InputArray(), decoded_info.as_raw_mut_VectorOfString(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::objdetect::GraphicalCodeDetector]
	pub trait GraphicalCodeDetectorTrait: crate::objdetect::GraphicalCodeDetectorTraitConst {
		fn as_raw_mut_GraphicalCodeDetector(&mut self) -> *mut c_void;

		#[inline]
		fn set(&mut self, unnamed: &impl crate::objdetect::GraphicalCodeDetectorTraitConst) {
			let ret = unsafe { sys::cv_GraphicalCodeDetector_operatorST_const_GraphicalCodeDetectorR(self.as_raw_mut_GraphicalCodeDetector(), unnamed.as_raw_GraphicalCodeDetector()) };
			ret
		}

		#[inline]
		fn set_1(&mut self, mut unnamed: crate::objdetect::GraphicalCodeDetector) {
			let ret = unsafe { sys::cv_GraphicalCodeDetector_operatorST_GraphicalCodeDetectorRR(self.as_raw_mut_GraphicalCodeDetector(), unnamed.as_raw_mut_GraphicalCodeDetector()) };
			ret
		}

	}

	impl Clone for GraphicalCodeDetector {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_GraphicalCodeDetector_implicitClone_const(self.as_raw_GraphicalCodeDetector())) }
		}
	}

	impl std::fmt::Debug for GraphicalCodeDetector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GraphicalCodeDetector")
				.finish()
		}
	}

	impl crate::objdetect::GraphicalCodeDetectorTraitConst for GraphicalCodeDetector {
		#[inline] fn as_raw_GraphicalCodeDetector(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::objdetect::GraphicalCodeDetectorTrait for GraphicalCodeDetector {
		#[inline] fn as_raw_mut_GraphicalCodeDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { GraphicalCodeDetector, crate::objdetect::GraphicalCodeDetectorTraitConst, as_raw_GraphicalCodeDetector, crate::objdetect::GraphicalCodeDetectorTrait, as_raw_mut_GraphicalCodeDetector }

	/// QR code detector.
	pub struct QRCodeDetector {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { QRCodeDetector }

	impl Drop for QRCodeDetector {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_QRCodeDetector_delete(self.as_raw_mut_QRCodeDetector()) };
		}
	}

	unsafe impl Send for QRCodeDetector {}

	impl QRCodeDetector {
		#[inline]
		pub fn default() -> Result<crate::objdetect::QRCodeDetector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeDetector_QRCodeDetector(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::QRCodeDetector::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::objdetect::QRCodeDetector]
	pub trait QRCodeDetectorTraitConst: crate::objdetect::GraphicalCodeDetectorTraitConst {
		fn as_raw_QRCodeDetector(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::objdetect::QRCodeDetector]
	pub trait QRCodeDetectorTrait: crate::objdetect::GraphicalCodeDetectorTrait + crate::objdetect::QRCodeDetectorTraitConst {
		fn as_raw_mut_QRCodeDetector(&mut self) -> *mut c_void;

		/// sets the epsilon used during the horizontal scan of QR code stop marker detection.
		/// ## Parameters
		/// * epsX: Epsilon neighborhood, which allows you to determine the horizontal pattern
		/// of the scheme 1:1:3:1:1 according to QR code standard.
		#[inline]
		fn set_eps_x(&mut self, eps_x: f64) -> Result<crate::objdetect::QRCodeDetector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeDetector_setEpsX_double(self.as_raw_mut_QRCodeDetector(), eps_x, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::QRCodeDetector::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// sets the epsilon used during the vertical scan of QR code stop marker detection.
		/// ## Parameters
		/// * epsY: Epsilon neighborhood, which allows you to determine the vertical pattern
		/// of the scheme 1:1:3:1:1 according to QR code standard.
		#[inline]
		fn set_eps_y(&mut self, eps_y: f64) -> Result<crate::objdetect::QRCodeDetector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeDetector_setEpsY_double(self.as_raw_mut_QRCodeDetector(), eps_y, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::QRCodeDetector::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// use markers to improve the position of the corners of the QR code
		///
		/// alignmentMarkers using by default
		#[inline]
		fn set_use_alignment_markers(&mut self, use_alignment_markers: bool) -> Result<crate::objdetect::QRCodeDetector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeDetector_setUseAlignmentMarkers_bool(self.as_raw_mut_QRCodeDetector(), use_alignment_markers, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::QRCodeDetector::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Decodes QR code on a curved surface in image once it's found by the detect() method.
		///
		/// Returns UTF8-encoded output string or empty string if the code cannot be decoded.
		/// ## Parameters
		/// * img: grayscale or color (BGR) image containing QR code.
		/// * points: Quadrangle vertices found by detect() method (or some other algorithm).
		/// * straight_qrcode: The optional output image containing rectified and binarized QR code
		///
		/// ## C++ default parameters
		/// * straight_qrcode: noArray()
		#[inline]
		fn decode_curved(&mut self, img: &impl ToInputArray, points: &impl ToInputArray, straight_qrcode: &mut impl ToOutputArray) -> Result<Vec<u8>> {
			input_array_arg!(img);
			input_array_arg!(points);
			output_array_arg!(straight_qrcode);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeDetector_decodeCurved_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_QRCodeDetector(), img.as_raw__InputArray(), points.as_raw__InputArray(), straight_qrcode.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { Vec::<u8>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Decodes QR code on a curved surface in image once it's found by the detect() method.
		///
		/// Returns UTF8-encoded output string or empty string if the code cannot be decoded.
		/// ## Parameters
		/// * img: grayscale or color (BGR) image containing QR code.
		/// * points: Quadrangle vertices found by detect() method (or some other algorithm).
		/// * straight_qrcode: The optional output image containing rectified and binarized QR code
		///
		/// ## Note
		/// This alternative version of [QRCodeDetectorTrait::decode_curved] function uses the following default values for its arguments:
		/// * straight_qrcode: noArray()
		#[inline]
		fn decode_curved_def(&mut self, img: &impl ToInputArray, points: &impl ToInputArray) -> Result<Vec<u8>> {
			input_array_arg!(img);
			input_array_arg!(points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeDetector_decodeCurved_const__InputArrayR_const__InputArrayR(self.as_raw_mut_QRCodeDetector(), img.as_raw__InputArray(), points.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { Vec::<u8>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Both detects and decodes QR code on a curved surface
		///
		/// ## Parameters
		/// * img: grayscale or color (BGR) image containing QR code.
		/// * points: optional output array of vertices of the found QR code quadrangle. Will be empty if not found.
		/// * straight_qrcode: The optional output image containing rectified and binarized QR code
		///
		/// ## C++ default parameters
		/// * points: noArray()
		/// * straight_qrcode: noArray()
		#[inline]
		fn detect_and_decode_curved(&mut self, img: &impl ToInputArray, points: &mut impl ToOutputArray, straight_qrcode: &mut impl ToOutputArray) -> Result<Vec<u8>> {
			input_array_arg!(img);
			output_array_arg!(points);
			output_array_arg!(straight_qrcode);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeDetector_detectAndDecodeCurved_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_QRCodeDetector(), img.as_raw__InputArray(), points.as_raw__OutputArray(), straight_qrcode.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { Vec::<u8>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Both detects and decodes QR code on a curved surface
		///
		/// ## Parameters
		/// * img: grayscale or color (BGR) image containing QR code.
		/// * points: optional output array of vertices of the found QR code quadrangle. Will be empty if not found.
		/// * straight_qrcode: The optional output image containing rectified and binarized QR code
		///
		/// ## Note
		/// This alternative version of [QRCodeDetectorTrait::detect_and_decode_curved] function uses the following default values for its arguments:
		/// * points: noArray()
		/// * straight_qrcode: noArray()
		#[inline]
		fn detect_and_decode_curved_def(&mut self, img: &impl ToInputArray) -> Result<Vec<u8>> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeDetector_detectAndDecodeCurved_const__InputArrayR(self.as_raw_mut_QRCodeDetector(), img.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { Vec::<u8>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Returns a kind of encoding for the decoded info from the latest [decode] or [detectAndDecode] call
		/// ## Parameters
		/// * codeIdx: an index of the previously decoded QR code.
		///                When [decode] or [detectAndDecode] is used, valid value is zero.
		///                For [decodeMulti] or [detectAndDecodeMulti] use indices corresponding to the output order.
		///
		/// ## C++ default parameters
		/// * code_idx: 0
		#[inline]
		fn get_encoding(&mut self, code_idx: i32) -> Result<crate::objdetect::QRCodeEncoder_ECIEncodings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeDetector_getEncoding_int(self.as_raw_mut_QRCodeDetector(), code_idx, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns a kind of encoding for the decoded info from the latest [decode] or [detectAndDecode] call
		/// ## Parameters
		/// * codeIdx: an index of the previously decoded QR code.
		///                When [decode] or [detectAndDecode] is used, valid value is zero.
		///                For [decodeMulti] or [detectAndDecodeMulti] use indices corresponding to the output order.
		///
		/// ## Note
		/// This alternative version of [QRCodeDetectorTrait::get_encoding] function uses the following default values for its arguments:
		/// * code_idx: 0
		#[inline]
		fn get_encoding_def(&mut self) -> Result<crate::objdetect::QRCodeEncoder_ECIEncodings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeDetector_getEncoding(self.as_raw_mut_QRCodeDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl Clone for QRCodeDetector {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_QRCodeDetector_implicitClone_const(self.as_raw_QRCodeDetector())) }
		}
	}

	impl std::fmt::Debug for QRCodeDetector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("QRCodeDetector")
				.finish()
		}
	}

	boxed_cast_base! { QRCodeDetector, crate::objdetect::GraphicalCodeDetector, cv_QRCodeDetector_to_GraphicalCodeDetector }

	impl crate::objdetect::GraphicalCodeDetectorTraitConst for QRCodeDetector {
		#[inline] fn as_raw_GraphicalCodeDetector(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::objdetect::GraphicalCodeDetectorTrait for QRCodeDetector {
		#[inline] fn as_raw_mut_GraphicalCodeDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { QRCodeDetector, crate::objdetect::GraphicalCodeDetectorTraitConst, as_raw_GraphicalCodeDetector, crate::objdetect::GraphicalCodeDetectorTrait, as_raw_mut_GraphicalCodeDetector }

	impl crate::objdetect::QRCodeDetectorTraitConst for QRCodeDetector {
		#[inline] fn as_raw_QRCodeDetector(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::objdetect::QRCodeDetectorTrait for QRCodeDetector {
		#[inline] fn as_raw_mut_QRCodeDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { QRCodeDetector, crate::objdetect::QRCodeDetectorTraitConst, as_raw_QRCodeDetector, crate::objdetect::QRCodeDetectorTrait, as_raw_mut_QRCodeDetector }

	/// QR code detector based on Aruco markers detection code.
	pub struct QRCodeDetectorAruco {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { QRCodeDetectorAruco }

	impl Drop for QRCodeDetectorAruco {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_QRCodeDetectorAruco_delete(self.as_raw_mut_QRCodeDetectorAruco()) };
		}
	}

	unsafe impl Send for QRCodeDetectorAruco {}

	impl QRCodeDetectorAruco {
		#[inline]
		pub fn default() -> Result<crate::objdetect::QRCodeDetectorAruco> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeDetectorAruco_QRCodeDetectorAruco(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::QRCodeDetectorAruco::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// QR code detector constructor for Aruco-based algorithm. See cv::QRCodeDetectorAruco::Params
		#[inline]
		pub fn new(params: crate::objdetect::QRCodeDetectorAruco_Params) -> Result<crate::objdetect::QRCodeDetectorAruco> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeDetectorAruco_QRCodeDetectorAruco_const_ParamsR(&params, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::QRCodeDetectorAruco::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::objdetect::QRCodeDetectorAruco]
	pub trait QRCodeDetectorArucoTraitConst: crate::objdetect::GraphicalCodeDetectorTraitConst {
		fn as_raw_QRCodeDetectorAruco(&self) -> *const c_void;

		/// Detector parameters getter. See cv::QRCodeDetectorAruco::Params
		#[inline]
		fn get_detector_parameters(&self) -> Result<crate::objdetect::QRCodeDetectorAruco_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeDetectorAruco_getDetectorParameters_const(self.as_raw_QRCodeDetectorAruco(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Aruco detector parameters are used to search for the finder patterns.
		#[inline]
		fn get_aruco_parameters(&self) -> Result<crate::objdetect::DetectorParameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeDetectorAruco_getArucoParameters_const(self.as_raw_QRCodeDetectorAruco(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::DetectorParameters::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::objdetect::QRCodeDetectorAruco]
	pub trait QRCodeDetectorArucoTrait: crate::objdetect::GraphicalCodeDetectorTrait + crate::objdetect::QRCodeDetectorArucoTraitConst {
		fn as_raw_mut_QRCodeDetectorAruco(&mut self) -> *mut c_void;

		/// Detector parameters setter. See cv::QRCodeDetectorAruco::Params
		#[inline]
		fn set_detector_parameters(&mut self, params: crate::objdetect::QRCodeDetectorAruco_Params) -> Result<crate::objdetect::QRCodeDetectorAruco> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeDetectorAruco_setDetectorParameters_const_ParamsR(self.as_raw_mut_QRCodeDetectorAruco(), &params, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::QRCodeDetectorAruco::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Aruco detector parameters are used to search for the finder patterns.
		#[inline]
		fn set_aruco_parameters(&mut self, params: &impl crate::objdetect::DetectorParametersTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeDetectorAruco_setArucoParameters_const_DetectorParametersR(self.as_raw_mut_QRCodeDetectorAruco(), params.as_raw_DetectorParameters(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl Clone for QRCodeDetectorAruco {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_QRCodeDetectorAruco_implicitClone_const(self.as_raw_QRCodeDetectorAruco())) }
		}
	}

	impl std::fmt::Debug for QRCodeDetectorAruco {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("QRCodeDetectorAruco")
				.finish()
		}
	}

	boxed_cast_base! { QRCodeDetectorAruco, crate::objdetect::GraphicalCodeDetector, cv_QRCodeDetectorAruco_to_GraphicalCodeDetector }

	impl crate::objdetect::GraphicalCodeDetectorTraitConst for QRCodeDetectorAruco {
		#[inline] fn as_raw_GraphicalCodeDetector(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::objdetect::GraphicalCodeDetectorTrait for QRCodeDetectorAruco {
		#[inline] fn as_raw_mut_GraphicalCodeDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { QRCodeDetectorAruco, crate::objdetect::GraphicalCodeDetectorTraitConst, as_raw_GraphicalCodeDetector, crate::objdetect::GraphicalCodeDetectorTrait, as_raw_mut_GraphicalCodeDetector }

	impl crate::objdetect::QRCodeDetectorArucoTraitConst for QRCodeDetectorAruco {
		#[inline] fn as_raw_QRCodeDetectorAruco(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::objdetect::QRCodeDetectorArucoTrait for QRCodeDetectorAruco {
		#[inline] fn as_raw_mut_QRCodeDetectorAruco(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { QRCodeDetectorAruco, crate::objdetect::QRCodeDetectorArucoTraitConst, as_raw_QRCodeDetectorAruco, crate::objdetect::QRCodeDetectorArucoTrait, as_raw_mut_QRCodeDetectorAruco }

	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq)]
	pub struct QRCodeDetectorAruco_Params {
		/// The minimum allowed pixel size of a QR module in the smallest image in the image pyramid, default 4.f
		pub min_module_size_in_pyramid: f32,
		/// The maximum allowed relative rotation for finder patterns in the same QR code, default pi/12
		pub max_rotation: f32,
		/// The maximum allowed relative mismatch in module sizes for finder patterns in the same QR code, default 1.75f
		pub max_module_size_mismatch: f32,
		/// The maximum allowed module relative mismatch for timing pattern module, default 2.f
		///
		/// If relative mismatch of timing pattern module more this value, penalty points will be added.
		/// If a lot of penalty points are added, QR code will be rejected.
		pub max_timing_pattern_mismatch: f32,
		/// The maximum allowed percentage of penalty points out of total pins in timing pattern, default 0.4f
		pub max_penalties: f32,
		/// The maximum allowed relative color mismatch in the timing pattern, default 0.2f
		pub max_colors_mismatch: f32,
		/// The algorithm find QR codes with almost minimum timing pattern score and minimum size, default 0.9f
		///
		/// The QR code with the minimum "timing pattern score" and minimum "size" is selected as the best QR code.
		/// If for the current QR code "timing pattern score" * scaleTimingPatternScore < "previous timing pattern score" and "size" < "previous size", then
		/// current QR code set as the best QR code.
		pub scale_timing_pattern_score: f32,
	}

	opencv_type_simple! { crate::objdetect::QRCodeDetectorAruco_Params }

	impl QRCodeDetectorAruco_Params {
		#[inline]
		pub fn default() -> Result<crate::objdetect::QRCodeDetectorAruco_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeDetectorAruco_Params_Params(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// QR code encoder.
	pub struct QRCodeEncoder {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { QRCodeEncoder }

	impl Drop for QRCodeEncoder {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_QRCodeEncoder_delete(self.as_raw_mut_QRCodeEncoder()) };
		}
	}

	unsafe impl Send for QRCodeEncoder {}

	impl QRCodeEncoder {
		/// Constructor
		/// ## Parameters
		/// * parameters: QR code encoder parameters QRCodeEncoder::Params
		///
		/// ## C++ default parameters
		/// * parameters: QRCodeEncoder::Params()
		#[inline]
		pub fn create(parameters: crate::objdetect::QRCodeEncoder_Params) -> Result<core::Ptr<crate::objdetect::QRCodeEncoder>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeEncoder_create_const_ParamsR(&parameters, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::objdetect::QRCodeEncoder>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Constructor
		/// ## Parameters
		/// * parameters: QR code encoder parameters QRCodeEncoder::Params
		///
		/// ## Note
		/// This alternative version of [QRCodeEncoder::create] function uses the following default values for its arguments:
		/// * parameters: QRCodeEncoder::Params()
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::objdetect::QRCodeEncoder>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeEncoder_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::objdetect::QRCodeEncoder>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::objdetect::QRCodeEncoder]
	pub trait QRCodeEncoderTraitConst {
		fn as_raw_QRCodeEncoder(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::objdetect::QRCodeEncoder]
	pub trait QRCodeEncoderTrait: crate::objdetect::QRCodeEncoderTraitConst {
		fn as_raw_mut_QRCodeEncoder(&mut self) -> *mut c_void;

		/// Generates QR code from input string.
		/// ## Parameters
		/// * encoded_info: Input string to encode.
		/// * qrcode: Generated QR code.
		#[inline]
		fn encode(&mut self, encoded_info: &str, qrcode: &mut impl ToOutputArray) -> Result<()> {
			extern_container_arg!(encoded_info);
			output_array_arg!(qrcode);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeEncoder_encode_const_StringR_const__OutputArrayR(self.as_raw_mut_QRCodeEncoder(), encoded_info.opencv_as_extern(), qrcode.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Generates QR code from input string in Structured Append mode. The encoded message is splitting over a number of QR codes.
		/// ## Parameters
		/// * encoded_info: Input string to encode.
		/// * qrcodes: Vector of generated QR codes.
		#[inline]
		fn encode_structured_append(&mut self, encoded_info: &str, qrcodes: &mut impl ToOutputArray) -> Result<()> {
			extern_container_arg!(encoded_info);
			output_array_arg!(qrcodes);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeEncoder_encodeStructuredAppend_const_StringR_const__OutputArrayR(self.as_raw_mut_QRCodeEncoder(), encoded_info.opencv_as_extern(), qrcodes.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for QRCodeEncoder {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("QRCodeEncoder")
				.finish()
		}
	}

	impl crate::objdetect::QRCodeEncoderTraitConst for QRCodeEncoder {
		#[inline] fn as_raw_QRCodeEncoder(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::objdetect::QRCodeEncoderTrait for QRCodeEncoder {
		#[inline] fn as_raw_mut_QRCodeEncoder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { QRCodeEncoder, crate::objdetect::QRCodeEncoderTraitConst, as_raw_QRCodeEncoder, crate::objdetect::QRCodeEncoderTrait, as_raw_mut_QRCodeEncoder }

	/// QR code encoder parameters.
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq)]
	pub struct QRCodeEncoder_Params {
		/// The optional version of QR code (by default - maximum possible depending on the length of the string).
		pub version: i32,
		/// The optional level of error correction (by default - the lowest).
		pub correction_level: crate::objdetect::QRCodeEncoder_CorrectionLevel,
		/// The optional encoding mode - Numeric, Alphanumeric, Byte, Kanji, ECI or Structured Append.
		pub mode: crate::objdetect::QRCodeEncoder_EncodeMode,
		/// The optional number of QR codes to generate in Structured Append mode.
		pub structure_number: i32,
	}

	opencv_type_simple! { crate::objdetect::QRCodeEncoder_Params }

	impl QRCodeEncoder_Params {
		#[inline]
		pub fn default() -> Result<crate::objdetect::QRCodeEncoder_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeEncoder_Params_Params(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// The main functionality of ArucoDetector class is detection of markers in an image with detectMarkers() method.
	///
	/// After detecting some markers in the image, you can try to find undetected markers from this dictionary with
	/// refineDetectedMarkers() method.
	/// ## See also
	/// DetectorParameters, RefineParameters
	pub struct ArucoDetector {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { ArucoDetector }

	impl Drop for ArucoDetector {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_aruco_ArucoDetector_delete(self.as_raw_mut_ArucoDetector()) };
		}
	}

	unsafe impl Send for ArucoDetector {}

	impl ArucoDetector {
		/// Basic ArucoDetector constructor
		///
		/// ## Parameters
		/// * dictionary: indicates the type of markers that will be searched
		/// * detectorParams: marker detection parameters
		/// * refineParams: marker refine detection parameters
		///
		/// ## C++ default parameters
		/// * dictionary: getPredefinedDictionary(cv::aruco::DICT_4X4_50)
		/// * detector_params: DetectorParameters()
		/// * refine_params: RefineParameters()
		#[inline]
		pub fn new(dictionary: &impl crate::objdetect::DictionaryTraitConst, detector_params: &impl crate::objdetect::DetectorParametersTraitConst, refine_params: crate::objdetect::RefineParameters) -> Result<crate::objdetect::ArucoDetector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_ArucoDetector_const_DictionaryR_const_DetectorParametersR_const_RefineParametersR(dictionary.as_raw_Dictionary(), detector_params.as_raw_DetectorParameters(), &refine_params, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::ArucoDetector::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Basic ArucoDetector constructor
		///
		/// ## Parameters
		/// * dictionary: indicates the type of markers that will be searched
		/// * detectorParams: marker detection parameters
		/// * refineParams: marker refine detection parameters
		///
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * dictionary: getPredefinedDictionary(cv::aruco::DICT_4X4_50)
		/// * detector_params: DetectorParameters()
		/// * refine_params: RefineParameters()
		#[inline]
		pub fn new_def() -> Result<crate::objdetect::ArucoDetector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_ArucoDetector(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::ArucoDetector::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// ArucoDetector constructor for multiple dictionaries
		///
		/// ## Parameters
		/// * dictionaries: indicates the type of markers that will be searched. Empty dictionaries will throw an error.
		/// * detectorParams: marker detection parameters
		/// * refineParams: marker refine detection parameters
		///
		/// ## C++ default parameters
		/// * detector_params: DetectorParameters()
		/// * refine_params: RefineParameters()
		#[inline]
		pub fn new_1(dictionaries: &core::Vector<crate::objdetect::Dictionary>, detector_params: &impl crate::objdetect::DetectorParametersTraitConst, refine_params: crate::objdetect::RefineParameters) -> Result<crate::objdetect::ArucoDetector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_ArucoDetector_const_vectorLDictionaryGR_const_DetectorParametersR_const_RefineParametersR(dictionaries.as_raw_VectorOfDictionary(), detector_params.as_raw_DetectorParameters(), &refine_params, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::ArucoDetector::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// ArucoDetector constructor for multiple dictionaries
		///
		/// ## Parameters
		/// * dictionaries: indicates the type of markers that will be searched. Empty dictionaries will throw an error.
		/// * detectorParams: marker detection parameters
		/// * refineParams: marker refine detection parameters
		///
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * detector_params: DetectorParameters()
		/// * refine_params: RefineParameters()
		#[inline]
		pub fn new_def_1(dictionaries: &core::Vector<crate::objdetect::Dictionary>) -> Result<crate::objdetect::ArucoDetector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_ArucoDetector_const_vectorLDictionaryGR(dictionaries.as_raw_VectorOfDictionary(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::ArucoDetector::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::objdetect::ArucoDetector]
	pub trait ArucoDetectorTraitConst: core::AlgorithmTraitConst {
		fn as_raw_ArucoDetector(&self) -> *const c_void;

		/// Basic marker detection
		///
		/// ## Parameters
		/// * image: input image
		/// * corners: vector of detected marker corners. For each marker, its four corners
		/// are provided, (e.g std::vector<std::vector<cv::Point2f> > ). For N detected markers,
		/// the dimensions of this array is Nx4. The order of the corners is clockwise.
		/// * ids: vector of identifiers of the detected markers. The identifier is of type int
		/// (e.g. std::vector<int>). For N detected markers, the size of ids is also N.
		/// The identifiers have the same order than the markers in the imgPoints array.
		/// * rejectedImgPoints: contains the imgPoints of those squares whose inner code has not a
		/// correct codification. Useful for debugging purposes.
		///
		/// Performs marker detection in the input image. Only markers included in the first specified dictionary
		/// are searched. For each detected marker, it returns the 2D position of its corner in the image
		/// and its corresponding identifier.
		/// Note that this function does not perform pose estimation.
		///
		/// Note: The function does not correct lens distortion or takes it into account. It's recommended to undistort
		/// input image with corresponding camera model, if camera parameters are known
		/// ## See also
		/// undistort, estimatePoseSingleMarkers,  estimatePoseBoard
		///
		/// ## C++ default parameters
		/// * rejected_img_points: noArray()
		#[inline]
		fn detect_markers(&self, image: &impl ToInputArray, corners: &mut impl ToOutputArray, ids: &mut impl ToOutputArray, rejected_img_points: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(corners);
			output_array_arg!(ids);
			output_array_arg!(rejected_img_points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_detectMarkers_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_ArucoDetector(), image.as_raw__InputArray(), corners.as_raw__OutputArray(), ids.as_raw__OutputArray(), rejected_img_points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Basic marker detection
		///
		/// ## Parameters
		/// * image: input image
		/// * corners: vector of detected marker corners. For each marker, its four corners
		/// are provided, (e.g std::vector<std::vector<cv::Point2f> > ). For N detected markers,
		/// the dimensions of this array is Nx4. The order of the corners is clockwise.
		/// * ids: vector of identifiers of the detected markers. The identifier is of type int
		/// (e.g. std::vector<int>). For N detected markers, the size of ids is also N.
		/// The identifiers have the same order than the markers in the imgPoints array.
		/// * rejectedImgPoints: contains the imgPoints of those squares whose inner code has not a
		/// correct codification. Useful for debugging purposes.
		///
		/// Performs marker detection in the input image. Only markers included in the first specified dictionary
		/// are searched. For each detected marker, it returns the 2D position of its corner in the image
		/// and its corresponding identifier.
		/// Note that this function does not perform pose estimation.
		///
		/// Note: The function does not correct lens distortion or takes it into account. It's recommended to undistort
		/// input image with corresponding camera model, if camera parameters are known
		/// ## See also
		/// undistort, estimatePoseSingleMarkers,  estimatePoseBoard
		///
		/// ## Note
		/// This alternative version of [ArucoDetectorTraitConst::detect_markers] function uses the following default values for its arguments:
		/// * rejected_img_points: noArray()
		#[inline]
		fn detect_markers_def(&self, image: &impl ToInputArray, corners: &mut impl ToOutputArray, ids: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(corners);
			output_array_arg!(ids);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_detectMarkers_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_ArucoDetector(), image.as_raw__InputArray(), corners.as_raw__OutputArray(), ids.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Marker detection with confidence computation
		///
		/// ## Parameters
		/// * image: input image
		/// * corners: vector of detected marker corners. For each marker, its four corners
		/// are provided, (e.g std::vector<std::vector<cv::Point2f> > ). For N detected markers,
		/// the dimensions of this array is Nx4. The order of the corners is clockwise.
		/// * ids: vector of identifiers of the detected markers. The identifier is of type int
		/// (e.g. std::vector<int>). For N detected markers, the size of ids is also N.
		/// The identifiers have the same order than the markers in the imgPoints array.
		/// * markersConfidence: contains the normalized confidence [0;1] of the markers' detection,
		/// defined as 1 minus the normalized uncertainty (percentage of incorrect pixel detections),
		/// with 1 describing a pixel perfect detection. The confidence values are of type float
		/// (e.g. std::vector<float>)
		/// * rejectedImgPoints: contains the imgPoints of those squares whose inner code has not a
		/// correct codification. Useful for debugging purposes.
		///
		/// Performs marker detection in the input image. Only markers included in the first specified dictionary
		/// are searched. For each detected marker, it returns the 2D position of its corner in the image
		/// and its corresponding identifier.
		/// Note that this function does not perform pose estimation.
		///
		/// Note: The function does not correct lens distortion or takes it into account. It's recommended to undistort
		/// input image with corresponding camera model, if camera parameters are known
		/// ## See also
		/// undistort, estimatePoseSingleMarkers,  estimatePoseBoard
		///
		/// ## C++ default parameters
		/// * rejected_img_points: noArray()
		#[inline]
		fn detect_markers_with_confidence(&self, image: &impl ToInputArray, corners: &mut impl ToOutputArray, ids: &mut impl ToOutputArray, markers_confidence: &mut impl ToOutputArray, rejected_img_points: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(corners);
			output_array_arg!(ids);
			output_array_arg!(markers_confidence);
			output_array_arg!(rejected_img_points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_detectMarkersWithConfidence_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_ArucoDetector(), image.as_raw__InputArray(), corners.as_raw__OutputArray(), ids.as_raw__OutputArray(), markers_confidence.as_raw__OutputArray(), rejected_img_points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Marker detection with confidence computation
		///
		/// ## Parameters
		/// * image: input image
		/// * corners: vector of detected marker corners. For each marker, its four corners
		/// are provided, (e.g std::vector<std::vector<cv::Point2f> > ). For N detected markers,
		/// the dimensions of this array is Nx4. The order of the corners is clockwise.
		/// * ids: vector of identifiers of the detected markers. The identifier is of type int
		/// (e.g. std::vector<int>). For N detected markers, the size of ids is also N.
		/// The identifiers have the same order than the markers in the imgPoints array.
		/// * markersConfidence: contains the normalized confidence [0;1] of the markers' detection,
		/// defined as 1 minus the normalized uncertainty (percentage of incorrect pixel detections),
		/// with 1 describing a pixel perfect detection. The confidence values are of type float
		/// (e.g. std::vector<float>)
		/// * rejectedImgPoints: contains the imgPoints of those squares whose inner code has not a
		/// correct codification. Useful for debugging purposes.
		///
		/// Performs marker detection in the input image. Only markers included in the first specified dictionary
		/// are searched. For each detected marker, it returns the 2D position of its corner in the image
		/// and its corresponding identifier.
		/// Note that this function does not perform pose estimation.
		///
		/// Note: The function does not correct lens distortion or takes it into account. It's recommended to undistort
		/// input image with corresponding camera model, if camera parameters are known
		/// ## See also
		/// undistort, estimatePoseSingleMarkers,  estimatePoseBoard
		///
		/// ## Note
		/// This alternative version of [ArucoDetectorTraitConst::detect_markers_with_confidence] function uses the following default values for its arguments:
		/// * rejected_img_points: noArray()
		#[inline]
		fn detect_markers_with_confidence_def(&self, image: &impl ToInputArray, corners: &mut impl ToOutputArray, ids: &mut impl ToOutputArray, markers_confidence: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(corners);
			output_array_arg!(ids);
			output_array_arg!(markers_confidence);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_detectMarkersWithConfidence_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_ArucoDetector(), image.as_raw__InputArray(), corners.as_raw__OutputArray(), ids.as_raw__OutputArray(), markers_confidence.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Refine not detected markers based on the already detected and the board layout
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
		/// * recoveredIdxs: Optional array to returns the indexes of the recovered candidates in the
		/// original rejectedCorners array.
		///
		/// This function tries to find markers that were not detected in the basic detecMarkers function.
		/// First, based on the current detected marker and the board layout, the function interpolates
		/// the position of the missing markers. Then it tries to find correspondence between the reprojected
		/// markers and the rejected candidates based on the minRepDistance and errorCorrectionRate parameters.
		/// If camera parameters and distortion coefficients are provided, missing markers are reprojected
		/// using projectPoint function. If not, missing marker projections are interpolated using global
		/// homography, and all the marker corners in the board must have the same Z coordinate.
		///
		/// Note: This function assumes that the board only contains markers from one dictionary, so only the
		/// first configured dictionary is used. It has to match the dictionary of the board to work properly.
		///
		/// ## C++ default parameters
		/// * camera_matrix: noArray()
		/// * dist_coeffs: noArray()
		/// * recovered_idxs: noArray()
		#[inline]
		fn refine_detected_markers(&self, image: &impl ToInputArray, board: &impl crate::objdetect::BoardTraitConst, detected_corners: &mut impl ToInputOutputArray, detected_ids: &mut impl ToInputOutputArray, rejected_corners: &mut impl ToInputOutputArray, camera_matrix: &impl ToInputArray, dist_coeffs: &impl ToInputArray, recovered_idxs: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			input_output_array_arg!(detected_corners);
			input_output_array_arg!(detected_ids);
			input_output_array_arg!(rejected_corners);
			input_array_arg!(camera_matrix);
			input_array_arg!(dist_coeffs);
			output_array_arg!(recovered_idxs);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_refineDetectedMarkers_const_const__InputArrayR_const_BoardR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_ArucoDetector(), image.as_raw__InputArray(), board.as_raw_Board(), detected_corners.as_raw__InputOutputArray(), detected_ids.as_raw__InputOutputArray(), rejected_corners.as_raw__InputOutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), recovered_idxs.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Refine not detected markers based on the already detected and the board layout
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
		/// * recoveredIdxs: Optional array to returns the indexes of the recovered candidates in the
		/// original rejectedCorners array.
		///
		/// This function tries to find markers that were not detected in the basic detecMarkers function.
		/// First, based on the current detected marker and the board layout, the function interpolates
		/// the position of the missing markers. Then it tries to find correspondence between the reprojected
		/// markers and the rejected candidates based on the minRepDistance and errorCorrectionRate parameters.
		/// If camera parameters and distortion coefficients are provided, missing markers are reprojected
		/// using projectPoint function. If not, missing marker projections are interpolated using global
		/// homography, and all the marker corners in the board must have the same Z coordinate.
		///
		/// Note: This function assumes that the board only contains markers from one dictionary, so only the
		/// first configured dictionary is used. It has to match the dictionary of the board to work properly.
		///
		/// ## Note
		/// This alternative version of [ArucoDetectorTraitConst::refine_detected_markers] function uses the following default values for its arguments:
		/// * camera_matrix: noArray()
		/// * dist_coeffs: noArray()
		/// * recovered_idxs: noArray()
		#[inline]
		fn refine_detected_markers_def(&self, image: &impl ToInputArray, board: &impl crate::objdetect::BoardTraitConst, detected_corners: &mut impl ToInputOutputArray, detected_ids: &mut impl ToInputOutputArray, rejected_corners: &mut impl ToInputOutputArray) -> Result<()> {
			input_array_arg!(image);
			input_output_array_arg!(detected_corners);
			input_output_array_arg!(detected_ids);
			input_output_array_arg!(rejected_corners);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_refineDetectedMarkers_const_const__InputArrayR_const_BoardR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(self.as_raw_ArucoDetector(), image.as_raw__InputArray(), board.as_raw_Board(), detected_corners.as_raw__InputOutputArray(), detected_ids.as_raw__InputOutputArray(), rejected_corners.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Basic marker detection
		///
		/// ## Parameters
		/// * image: input image
		/// * corners: vector of detected marker corners. For each marker, its four corners
		/// are provided, (e.g std::vector<std::vector<cv::Point2f> > ). For N detected markers,
		/// the dimensions of this array is Nx4. The order of the corners is clockwise.
		/// * ids: vector of identifiers of the detected markers. The identifier is of type int
		/// (e.g. std::vector<int>). For N detected markers, the size of ids is also N.
		/// The identifiers have the same order than the markers in the imgPoints array.
		/// * rejectedImgPoints: contains the imgPoints of those squares whose inner code has not a
		/// correct codification. Useful for debugging purposes.
		/// * dictIndices: vector of dictionary indices for each detected marker. Use getDictionaries() to get the
		/// list of corresponding dictionaries.
		///
		/// Performs marker detection in the input image. Only markers included in the specific dictionaries
		/// are searched. For each detected marker, it returns the 2D position of its corner in the image
		/// and its corresponding identifier.
		/// Note that this function does not perform pose estimation.
		///
		/// Note: The function does not correct lens distortion or takes it into account. It's recommended to undistort
		/// input image with corresponding camera model, if camera parameters are known
		/// ## See also
		/// undistort, estimatePoseSingleMarkers,  estimatePoseBoard
		///
		/// ## C++ default parameters
		/// * rejected_img_points: noArray()
		/// * dict_indices: noArray()
		#[inline]
		fn detect_markers_multi_dict(&self, image: &impl ToInputArray, corners: &mut impl ToOutputArray, ids: &mut impl ToOutputArray, rejected_img_points: &mut impl ToOutputArray, dict_indices: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(corners);
			output_array_arg!(ids);
			output_array_arg!(rejected_img_points);
			output_array_arg!(dict_indices);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_detectMarkersMultiDict_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_ArucoDetector(), image.as_raw__InputArray(), corners.as_raw__OutputArray(), ids.as_raw__OutputArray(), rejected_img_points.as_raw__OutputArray(), dict_indices.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Basic marker detection
		///
		/// ## Parameters
		/// * image: input image
		/// * corners: vector of detected marker corners. For each marker, its four corners
		/// are provided, (e.g std::vector<std::vector<cv::Point2f> > ). For N detected markers,
		/// the dimensions of this array is Nx4. The order of the corners is clockwise.
		/// * ids: vector of identifiers of the detected markers. The identifier is of type int
		/// (e.g. std::vector<int>). For N detected markers, the size of ids is also N.
		/// The identifiers have the same order than the markers in the imgPoints array.
		/// * rejectedImgPoints: contains the imgPoints of those squares whose inner code has not a
		/// correct codification. Useful for debugging purposes.
		/// * dictIndices: vector of dictionary indices for each detected marker. Use getDictionaries() to get the
		/// list of corresponding dictionaries.
		///
		/// Performs marker detection in the input image. Only markers included in the specific dictionaries
		/// are searched. For each detected marker, it returns the 2D position of its corner in the image
		/// and its corresponding identifier.
		/// Note that this function does not perform pose estimation.
		///
		/// Note: The function does not correct lens distortion or takes it into account. It's recommended to undistort
		/// input image with corresponding camera model, if camera parameters are known
		/// ## See also
		/// undistort, estimatePoseSingleMarkers,  estimatePoseBoard
		///
		/// ## Note
		/// This alternative version of [ArucoDetectorTraitConst::detect_markers_multi_dict] function uses the following default values for its arguments:
		/// * rejected_img_points: noArray()
		/// * dict_indices: noArray()
		#[inline]
		fn detect_markers_multi_dict_def(&self, image: &impl ToInputArray, corners: &mut impl ToOutputArray, ids: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(corners);
			output_array_arg!(ids);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_detectMarkersMultiDict_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_ArucoDetector(), image.as_raw__InputArray(), corners.as_raw__OutputArray(), ids.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns first dictionary from internal list used for marker detection.
		///
		/// ## Returns
		/// The first dictionary from the configured ArucoDetector.
		#[inline]
		fn get_dictionary(&self) -> Result<crate::objdetect::Dictionary> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_getDictionary_const(self.as_raw_ArucoDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::Dictionary::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Returns all dictionaries currently used for marker detection as a vector.
		///
		/// ## Returns
		/// A std::vector<Dictionary> containing all dictionaries used by the ArucoDetector.
		#[inline]
		fn get_dictionaries(&self) -> Result<core::Vector<crate::objdetect::Dictionary>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_getDictionaries_const(self.as_raw_ArucoDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<crate::objdetect::Dictionary>::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn get_detector_parameters(&self) -> Result<crate::objdetect::DetectorParameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_getDetectorParameters_const(self.as_raw_ArucoDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::DetectorParameters::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn get_refine_parameters(&self) -> Result<crate::objdetect::RefineParameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_getRefineParameters_const(self.as_raw_ArucoDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Stores algorithm parameters in a file storage
		#[inline]
		fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_write_const_FileStorageR(self.as_raw_ArucoDetector(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::objdetect::ArucoDetector]
	pub trait ArucoDetectorTrait: core::AlgorithmTrait + crate::objdetect::ArucoDetectorTraitConst {
		fn as_raw_mut_ArucoDetector(&mut self) -> *mut c_void;

		/// Sets and replaces the first dictionary in internal list to be used for marker detection.
		///
		/// ## Parameters
		/// * dictionary: The new dictionary that will replace the first dictionary in the internal list.
		#[inline]
		fn set_dictionary(&mut self, dictionary: &impl crate::objdetect::DictionaryTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_setDictionary_const_DictionaryR(self.as_raw_mut_ArucoDetector(), dictionary.as_raw_Dictionary(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Sets the entire collection of dictionaries to be used for marker detection, replacing any existing dictionaries.
		///
		/// ## Parameters
		/// * dictionaries: A std::vector<Dictionary> containing the new set of dictionaries to be used.
		///
		/// Configures the ArucoDetector to use the provided vector of dictionaries for marker detection.
		/// This method replaces any dictionaries that were previously set.
		///
		/// Note: Setting an empty vector of dictionaries will throw an error.
		#[inline]
		fn set_dictionaries(&mut self, dictionaries: &core::Vector<crate::objdetect::Dictionary>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_setDictionaries_const_vectorLDictionaryGR(self.as_raw_mut_ArucoDetector(), dictionaries.as_raw_VectorOfDictionary(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_detector_parameters(&mut self, detector_parameters: &impl crate::objdetect::DetectorParametersTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_setDetectorParameters_const_DetectorParametersR(self.as_raw_mut_ArucoDetector(), detector_parameters.as_raw_DetectorParameters(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_refine_parameters(&mut self, refine_parameters: crate::objdetect::RefineParameters) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_setRefineParameters_const_RefineParametersR(self.as_raw_mut_ArucoDetector(), &refine_parameters, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// simplified API for language bindings
		#[inline]
		fn write_1(&mut self, fs: &mut impl core::FileStorageTrait, name: &str) -> Result<()> {
			extern_container_arg!(name);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_write_FileStorageR_const_StringR(self.as_raw_mut_ArucoDetector(), fs.as_raw_mut_FileStorage(), name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Reads algorithm parameters from a file storage
		#[inline]
		fn read(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_read_const_FileNodeR(self.as_raw_mut_ArucoDetector(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for ArucoDetector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ArucoDetector")
				.finish()
		}
	}

	boxed_cast_base! { ArucoDetector, core::Algorithm, cv_aruco_ArucoDetector_to_Algorithm }

	impl core::AlgorithmTraitConst for ArucoDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for ArucoDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { ArucoDetector, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::objdetect::ArucoDetectorTraitConst for ArucoDetector {
		#[inline] fn as_raw_ArucoDetector(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::objdetect::ArucoDetectorTrait for ArucoDetector {
		#[inline] fn as_raw_mut_ArucoDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { ArucoDetector, crate::objdetect::ArucoDetectorTraitConst, as_raw_ArucoDetector, crate::objdetect::ArucoDetectorTrait, as_raw_mut_ArucoDetector }

	/// Board of ArUco markers
	///
	/// A board is a set of markers in the 3D space with a common coordinate system.
	/// The common form of a board of marker is a planar (2D) board, however any 3D layout can be used.
	/// A Board object is composed by:
	/// - The object points of the marker corners, i.e. their coordinates respect to the board system.
	/// - The dictionary which indicates the type of markers of the board
	/// - The identifier of all the markers in the board.
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

	impl Board {
		/// Common Board constructor
		///
		/// ## Parameters
		/// * objPoints: array of object points of all the marker corners in the board
		/// * dictionary: the dictionary of markers employed for this board
		/// * ids: vector of the identifiers of the markers in the board
		#[inline]
		pub fn new(obj_points: &impl ToInputArray, dictionary: &impl crate::objdetect::DictionaryTraitConst, ids: &impl ToInputArray) -> Result<crate::objdetect::Board> {
			input_array_arg!(obj_points);
			input_array_arg!(ids);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Board_Board_const__InputArrayR_const_DictionaryR_const__InputArrayR(obj_points.as_raw__InputArray(), dictionary.as_raw_Dictionary(), ids.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::Board::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		pub fn default() -> Result<crate::objdetect::Board> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Board_Board(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::Board::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::objdetect::Board]
	pub trait BoardTraitConst {
		fn as_raw_Board(&self) -> *const c_void;

		/// return the Dictionary of markers employed for this board
		#[inline]
		fn get_dictionary(&self) -> Result<crate::objdetect::Dictionary> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Board_getDictionary_const(self.as_raw_Board(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::Dictionary::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// return array of object points of all the marker corners in the board.
		///
		/// Each marker include its 4 corners in this order:
		/// *   objPoints[i][0] - left-top point of i-th marker
		/// *   objPoints[i][1] - right-top point of i-th marker
		/// *   objPoints[i][2] - right-bottom point of i-th marker
		/// *   objPoints[i][3] - left-bottom point of i-th marker
		///
		/// Markers are placed in a certain order - row by row, left to right in every row. For M markers, the size is Mx4.
		#[inline]
		fn get_obj_points(&self) -> Result<core::Vector<core::Vector<core::Point3f>>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Board_getObjPoints_const(self.as_raw_Board(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Vector<core::Point3f>>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// vector of the identifiers of the markers in the board (should be the same size as objPoints)
		/// ## Returns
		/// vector of the identifiers of the markers
		#[inline]
		fn get_ids(&self) -> Result<core::Vector<i32>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Board_getIds_const(self.as_raw_Board(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// get coordinate of the bottom right corner of the board, is set when calling the function create()
		#[inline]
		fn get_right_bottom_corner(&self) -> Result<core::Point3f> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Board_getRightBottomCorner_const(self.as_raw_Board(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Given a board configuration and a set of detected markers, returns the corresponding
		/// image points and object points, can be used in solvePnP()
		///
		/// ## Parameters
		/// * detectedCorners: List of detected marker corners of the board.
		/// For cv::Board and cv::GridBoard the method expects std::vector<std::vector<Point2f>> or std::vector<Mat> with Aruco marker corners.
		/// For cv::CharucoBoard the method expects std::vector<Point2f> or Mat with ChAruco corners (chess board corners matched with Aruco markers).
		///
		/// * detectedIds: List of identifiers for each marker or charuco corner.
		/// For any Board class the method expects std::vector<int> or Mat.
		///
		/// * objPoints: Vector of marker points in the board coordinate space.
		/// For any Board class the method expects std::vector<cv::Point3f> objectPoints or cv::Mat
		///
		/// * imgPoints: Vector of marker points in the image coordinate space.
		/// For any Board class the method expects std::vector<cv::Point2f> objectPoints or cv::Mat
		/// ## See also
		/// solvePnP
		#[inline]
		fn match_image_points(&self, detected_corners: &impl ToInputArray, detected_ids: &impl ToInputArray, obj_points: &mut impl ToOutputArray, img_points: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(detected_corners);
			input_array_arg!(detected_ids);
			output_array_arg!(obj_points);
			output_array_arg!(img_points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Board_matchImagePoints_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_Board(), detected_corners.as_raw__InputArray(), detected_ids.as_raw__InputArray(), obj_points.as_raw__OutputArray(), img_points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Draw a planar board
		///
		/// ## Parameters
		/// * outSize: size of the output image in pixels.
		/// * img: output image with the board. The size of this image will be outSize
		/// and the board will be on the center, keeping the board proportions.
		/// * marginSize: minimum margins (in pixels) of the board in the output image
		/// * borderBits: width of the marker borders.
		///
		/// This function return the image of the board, ready to be printed.
		///
		/// ## C++ default parameters
		/// * margin_size: 0
		/// * border_bits: 1
		#[inline]
		fn generate_image(&self, out_size: core::Size, img: &mut impl ToOutputArray, margin_size: i32, border_bits: i32) -> Result<()> {
			output_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Board_generateImage_const_Size_const__OutputArrayR_int_int(self.as_raw_Board(), &out_size, img.as_raw__OutputArray(), margin_size, border_bits, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Draw a planar board
		///
		/// ## Parameters
		/// * outSize: size of the output image in pixels.
		/// * img: output image with the board. The size of this image will be outSize
		/// and the board will be on the center, keeping the board proportions.
		/// * marginSize: minimum margins (in pixels) of the board in the output image
		/// * borderBits: width of the marker borders.
		///
		/// This function return the image of the board, ready to be printed.
		///
		/// ## Note
		/// This alternative version of [BoardTraitConst::generate_image] function uses the following default values for its arguments:
		/// * margin_size: 0
		/// * border_bits: 1
		#[inline]
		fn generate_image_def(&self, out_size: core::Size, img: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Board_generateImage_const_Size_const__OutputArrayR(self.as_raw_Board(), &out_size, img.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::objdetect::Board]
	pub trait BoardTrait: crate::objdetect::BoardTraitConst {
		fn as_raw_mut_Board(&mut self) -> *mut c_void;

	}

	impl Clone for Board {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_aruco_Board_implicitClone_const(self.as_raw_Board())) }
		}
	}

	impl std::fmt::Debug for Board {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Board")
				.finish()
		}
	}

	impl crate::objdetect::BoardTraitConst for Board {
		#[inline] fn as_raw_Board(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::objdetect::BoardTrait for Board {
		#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Board, crate::objdetect::BoardTraitConst, as_raw_Board, crate::objdetect::BoardTrait, as_raw_mut_Board }

	/// ChArUco board is a planar chessboard where the markers are placed inside the white squares of a chessboard.
	///
	/// The benefits of ChArUco boards is that they provide both, ArUco markers versatility and chessboard corner precision,
	/// which is important for calibration and pose estimation. The board image can be drawn using generateImage() method.
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

	impl CharucoBoard {
		/// CharucoBoard constructor
		///
		/// ## Parameters
		/// * size: number of chessboard squares in x and y directions
		/// * squareLength: squareLength chessboard square side length (normally in meters)
		/// * markerLength: marker side length (same unit than squareLength)
		/// * dictionary: dictionary of markers indicating the type of markers
		/// * ids: array of id used markers
		/// The first markers in the dictionary are used to fill the white chessboard squares.
		///
		/// ## C++ default parameters
		/// * ids: noArray()
		#[inline]
		pub fn new(size: core::Size, square_length: f32, marker_length: f32, dictionary: &impl crate::objdetect::DictionaryTraitConst, ids: &impl ToInputArray) -> Result<crate::objdetect::CharucoBoard> {
			input_array_arg!(ids);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoBoard_CharucoBoard_const_SizeR_float_float_const_DictionaryR_const__InputArrayR(&size, square_length, marker_length, dictionary.as_raw_Dictionary(), ids.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::CharucoBoard::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// CharucoBoard constructor
		///
		/// ## Parameters
		/// * size: number of chessboard squares in x and y directions
		/// * squareLength: squareLength chessboard square side length (normally in meters)
		/// * markerLength: marker side length (same unit than squareLength)
		/// * dictionary: dictionary of markers indicating the type of markers
		/// * ids: array of id used markers
		/// The first markers in the dictionary are used to fill the white chessboard squares.
		///
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * ids: noArray()
		#[inline]
		pub fn new_def(size: core::Size, square_length: f32, marker_length: f32, dictionary: &impl crate::objdetect::DictionaryTraitConst) -> Result<crate::objdetect::CharucoBoard> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoBoard_CharucoBoard_const_SizeR_float_float_const_DictionaryR(&size, square_length, marker_length, dictionary.as_raw_Dictionary(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::CharucoBoard::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		pub fn default() -> Result<crate::objdetect::CharucoBoard> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoBoard_CharucoBoard(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::CharucoBoard::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::objdetect::CharucoBoard]
	pub trait CharucoBoardTraitConst: crate::objdetect::BoardTraitConst {
		fn as_raw_CharucoBoard(&self) -> *const c_void;

		#[inline]
		fn get_legacy_pattern(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoBoard_getLegacyPattern_const(self.as_raw_CharucoBoard(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_chessboard_size(&self) -> Result<core::Size> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoBoard_getChessboardSize_const(self.as_raw_CharucoBoard(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_square_length(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoBoard_getSquareLength_const(self.as_raw_CharucoBoard(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_marker_length(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoBoard_getMarkerLength_const(self.as_raw_CharucoBoard(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// get CharucoBoard::chessboardCorners
		#[inline]
		fn get_chessboard_corners(&self) -> Result<core::Vector<core::Point3f>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoBoard_getChessboardCorners_const(self.as_raw_CharucoBoard(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Point3f>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// get CharucoBoard::nearestMarkerIdx, for each charuco corner, nearest marker index in ids array
		#[inline]
		fn get_nearest_marker_idx(&self) -> Result<core::Vector<core::Vector<i32>>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoBoard_getNearestMarkerIdx_const(self.as_raw_CharucoBoard(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Vector<i32>>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// get CharucoBoard::nearestMarkerCorners, for each charuco corner, nearest marker corner id of each marker
		#[inline]
		fn get_nearest_marker_corners(&self) -> Result<core::Vector<core::Vector<i32>>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoBoard_getNearestMarkerCorners_const(self.as_raw_CharucoBoard(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Vector<i32>>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// check whether the ChArUco markers are collinear
		///
		/// ## Parameters
		/// * charucoIds: list of identifiers for each corner in charucoCorners per frame.
		/// ## Returns
		/// bool value, 1 (true) if detected corners form a line, 0 (false) if they do not.
		/// solvePnP, calibration functions will fail if the corners are collinear (true).
		///
		/// The number of ids in charucoIDs should be <= the number of chessboard corners in the board.
		/// This functions checks whether the charuco corners are on a straight line (returns true, if so), or not (false).
		/// Axis parallel, as well as diagonal and other straight lines detected.  Degenerate cases:
		/// for number of charucoIDs <= 2,the function returns true.
		#[inline]
		fn check_charuco_corners_collinear(&self, charuco_ids: &impl ToInputArray) -> Result<bool> {
			input_array_arg!(charuco_ids);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoBoard_checkCharucoCornersCollinear_const_const__InputArrayR(self.as_raw_CharucoBoard(), charuco_ids.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::objdetect::CharucoBoard]
	pub trait CharucoBoardTrait: crate::objdetect::BoardTrait + crate::objdetect::CharucoBoardTraitConst {
		fn as_raw_mut_CharucoBoard(&mut self) -> *mut c_void;

		/// set legacy chessboard pattern.
		///
		/// Legacy setting creates chessboard patterns starting with a white box in the upper left corner
		/// if there is an even row count of chessboard boxes, otherwise it starts with a black box.
		/// This setting ensures compatibility to patterns created with OpenCV versions prior OpenCV 4.6.0.
		/// See <https://github.com/opencv/opencv/issues/23152>.
		///
		/// Default value: false.
		#[inline]
		fn set_legacy_pattern(&mut self, legacy_pattern: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoBoard_setLegacyPattern_bool(self.as_raw_mut_CharucoBoard(), legacy_pattern, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl Clone for CharucoBoard {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_aruco_CharucoBoard_implicitClone_const(self.as_raw_CharucoBoard())) }
		}
	}

	impl std::fmt::Debug for CharucoBoard {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CharucoBoard")
				.finish()
		}
	}

	boxed_cast_base! { CharucoBoard, crate::objdetect::Board, cv_aruco_CharucoBoard_to_Board }

	impl crate::objdetect::BoardTraitConst for CharucoBoard {
		#[inline] fn as_raw_Board(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::objdetect::BoardTrait for CharucoBoard {
		#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CharucoBoard, crate::objdetect::BoardTraitConst, as_raw_Board, crate::objdetect::BoardTrait, as_raw_mut_Board }

	impl crate::objdetect::CharucoBoardTraitConst for CharucoBoard {
		#[inline] fn as_raw_CharucoBoard(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::objdetect::CharucoBoardTrait for CharucoBoard {
		#[inline] fn as_raw_mut_CharucoBoard(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CharucoBoard, crate::objdetect::CharucoBoardTraitConst, as_raw_CharucoBoard, crate::objdetect::CharucoBoardTrait, as_raw_mut_CharucoBoard }

	pub struct CharucoDetector {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { CharucoDetector }

	impl Drop for CharucoDetector {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_aruco_CharucoDetector_delete(self.as_raw_mut_CharucoDetector()) };
		}
	}

	unsafe impl Send for CharucoDetector {}

	impl CharucoDetector {
		/// Basic CharucoDetector constructor
		///
		/// ## Parameters
		/// * board: ChAruco board
		/// * charucoParams: charuco detection parameters
		/// * detectorParams: marker detection parameters
		/// * refineParams: marker refine detection parameters
		///
		/// ## C++ default parameters
		/// * charuco_params: CharucoParameters()
		/// * detector_params: DetectorParameters()
		/// * refine_params: RefineParameters()
		#[inline]
		pub fn new(board: &impl crate::objdetect::CharucoBoardTraitConst, charuco_params: &impl crate::objdetect::CharucoParametersTraitConst, detector_params: &impl crate::objdetect::DetectorParametersTraitConst, refine_params: crate::objdetect::RefineParameters) -> Result<crate::objdetect::CharucoDetector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_CharucoDetector_const_CharucoBoardR_const_CharucoParametersR_const_DetectorParametersR_const_RefineParametersR(board.as_raw_CharucoBoard(), charuco_params.as_raw_CharucoParameters(), detector_params.as_raw_DetectorParameters(), &refine_params, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::CharucoDetector::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Basic CharucoDetector constructor
		///
		/// ## Parameters
		/// * board: ChAruco board
		/// * charucoParams: charuco detection parameters
		/// * detectorParams: marker detection parameters
		/// * refineParams: marker refine detection parameters
		///
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * charuco_params: CharucoParameters()
		/// * detector_params: DetectorParameters()
		/// * refine_params: RefineParameters()
		#[inline]
		pub fn new_def(board: &impl crate::objdetect::CharucoBoardTraitConst) -> Result<crate::objdetect::CharucoDetector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_CharucoDetector_const_CharucoBoardR(board.as_raw_CharucoBoard(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::CharucoDetector::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::objdetect::CharucoDetector]
	pub trait CharucoDetectorTraitConst: core::AlgorithmTraitConst {
		fn as_raw_CharucoDetector(&self) -> *const c_void;

		#[inline]
		fn get_board(&self) -> Result<crate::objdetect::CharucoBoard> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_getBoard_const(self.as_raw_CharucoDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::CharucoBoard::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn get_charuco_parameters(&self) -> Result<crate::objdetect::CharucoParameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_getCharucoParameters_const(self.as_raw_CharucoDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::CharucoParameters::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn get_detector_parameters(&self) -> Result<crate::objdetect::DetectorParameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_getDetectorParameters_const(self.as_raw_CharucoDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::DetectorParameters::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn get_refine_parameters(&self) -> Result<crate::objdetect::RefineParameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_getRefineParameters_const(self.as_raw_CharucoDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// detect aruco markers and interpolate position of ChArUco board corners
		/// ## Parameters
		/// * image: input image necessary for corner refinement. Note that markers are not detected and
		/// should be sent in corners and ids parameters.
		/// * charucoCorners: interpolated chessboard corners.
		/// * charucoIds: interpolated chessboard corners identifiers.
		/// * markerCorners: vector of already detected markers corners. For each marker, its four
		/// corners are provided, (e.g std::vector<std::vector<cv::Point2f> > ). For N detected markers, the
		/// dimensions of this array should be Nx4. The order of the corners should be clockwise.
		/// If markerCorners and markerCorners are empty, the function detect aruco markers and ids.
		/// * markerIds: list of identifiers for each marker in corners.
		///  If markerCorners and markerCorners are empty, the function detect aruco markers and ids.
		///
		/// This function receives the detected markers and returns the 2D position of the chessboard corners
		/// from a ChArUco board using the detected Aruco markers.
		///
		/// If markerCorners and markerCorners are empty, the detectMarkers() will run and detect aruco markers and ids.
		///
		/// If camera parameters are provided, the process is based in an approximated pose estimation, else it is based on local homography.
		/// Only visible corners are returned. For each corner, its corresponding identifier is also returned in charucoIds.
		/// ## See also
		/// findChessboardCorners
		///
		/// Note: After OpenCV 4.6.0, there was an incompatible change in the ChArUco pattern generation algorithm for even row counts.
		/// Use cv::aruco::CharucoBoard::setLegacyPattern() to ensure compatibility with patterns created using OpenCV versions prior to 4.6.0.
		/// For more information, see the issue: <https://github.com/opencv/opencv/issues/23152>
		///
		/// ## C++ default parameters
		/// * marker_corners: noArray()
		/// * marker_ids: noArray()
		#[inline]
		fn detect_board(&self, image: &impl ToInputArray, charuco_corners: &mut impl ToOutputArray, charuco_ids: &mut impl ToOutputArray, marker_corners: &mut impl ToInputOutputArray, marker_ids: &mut impl ToInputOutputArray) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(charuco_corners);
			output_array_arg!(charuco_ids);
			input_output_array_arg!(marker_corners);
			input_output_array_arg!(marker_ids);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_detectBoard_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(self.as_raw_CharucoDetector(), image.as_raw__InputArray(), charuco_corners.as_raw__OutputArray(), charuco_ids.as_raw__OutputArray(), marker_corners.as_raw__InputOutputArray(), marker_ids.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// detect aruco markers and interpolate position of ChArUco board corners
		/// ## Parameters
		/// * image: input image necessary for corner refinement. Note that markers are not detected and
		/// should be sent in corners and ids parameters.
		/// * charucoCorners: interpolated chessboard corners.
		/// * charucoIds: interpolated chessboard corners identifiers.
		/// * markerCorners: vector of already detected markers corners. For each marker, its four
		/// corners are provided, (e.g std::vector<std::vector<cv::Point2f> > ). For N detected markers, the
		/// dimensions of this array should be Nx4. The order of the corners should be clockwise.
		/// If markerCorners and markerCorners are empty, the function detect aruco markers and ids.
		/// * markerIds: list of identifiers for each marker in corners.
		///  If markerCorners and markerCorners are empty, the function detect aruco markers and ids.
		///
		/// This function receives the detected markers and returns the 2D position of the chessboard corners
		/// from a ChArUco board using the detected Aruco markers.
		///
		/// If markerCorners and markerCorners are empty, the detectMarkers() will run and detect aruco markers and ids.
		///
		/// If camera parameters are provided, the process is based in an approximated pose estimation, else it is based on local homography.
		/// Only visible corners are returned. For each corner, its corresponding identifier is also returned in charucoIds.
		/// ## See also
		/// findChessboardCorners
		///
		/// Note: After OpenCV 4.6.0, there was an incompatible change in the ChArUco pattern generation algorithm for even row counts.
		/// Use cv::aruco::CharucoBoard::setLegacyPattern() to ensure compatibility with patterns created using OpenCV versions prior to 4.6.0.
		/// For more information, see the issue: <https://github.com/opencv/opencv/issues/23152>
		///
		/// ## Note
		/// This alternative version of [CharucoDetectorTraitConst::detect_board] function uses the following default values for its arguments:
		/// * marker_corners: noArray()
		/// * marker_ids: noArray()
		#[inline]
		fn detect_board_def(&self, image: &impl ToInputArray, charuco_corners: &mut impl ToOutputArray, charuco_ids: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(charuco_corners);
			output_array_arg!(charuco_ids);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_detectBoard_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_CharucoDetector(), image.as_raw__InputArray(), charuco_corners.as_raw__OutputArray(), charuco_ids.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Detect ChArUco Diamond markers
		///
		/// ## Parameters
		/// * image: input image necessary for corner subpixel.
		/// * diamondCorners: output list of detected diamond corners (4 corners per diamond). The order
		/// is the same than in marker corners: top left, top right, bottom right and bottom left. Similar
		/// format than the corners returned by detectMarkers (e.g std::vector<std::vector<cv::Point2f> > ).
		/// * diamondIds: ids of the diamonds in diamondCorners. The id of each diamond is in fact of
		/// type Vec4i, so each diamond has 4 ids, which are the ids of the aruco markers composing the
		/// diamond.
		/// * markerCorners: list of detected marker corners from detectMarkers function.
		/// If markerCorners and markerCorners are empty, the function detect aruco markers and ids.
		/// * markerIds: list of marker ids in markerCorners.
		/// If markerCorners and markerCorners are empty, the function detect aruco markers and ids.
		///
		/// This function detects Diamond markers from the previous detected ArUco markers. The diamonds
		/// are returned in the diamondCorners and diamondIds parameters. If camera calibration parameters
		/// are provided, the diamond search is based on reprojection. If not, diamond search is based on
		/// homography. Homography is faster than reprojection, but less accurate.
		///
		/// ## C++ default parameters
		/// * marker_corners: noArray()
		/// * marker_ids: noArray()
		#[inline]
		fn detect_diamonds(&self, image: &impl ToInputArray, diamond_corners: &mut impl ToOutputArray, diamond_ids: &mut impl ToOutputArray, marker_corners: &mut impl ToInputOutputArray, marker_ids: &mut impl ToInputOutputArray) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(diamond_corners);
			output_array_arg!(diamond_ids);
			input_output_array_arg!(marker_corners);
			input_output_array_arg!(marker_ids);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_detectDiamonds_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(self.as_raw_CharucoDetector(), image.as_raw__InputArray(), diamond_corners.as_raw__OutputArray(), diamond_ids.as_raw__OutputArray(), marker_corners.as_raw__InputOutputArray(), marker_ids.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Detect ChArUco Diamond markers
		///
		/// ## Parameters
		/// * image: input image necessary for corner subpixel.
		/// * diamondCorners: output list of detected diamond corners (4 corners per diamond). The order
		/// is the same than in marker corners: top left, top right, bottom right and bottom left. Similar
		/// format than the corners returned by detectMarkers (e.g std::vector<std::vector<cv::Point2f> > ).
		/// * diamondIds: ids of the diamonds in diamondCorners. The id of each diamond is in fact of
		/// type Vec4i, so each diamond has 4 ids, which are the ids of the aruco markers composing the
		/// diamond.
		/// * markerCorners: list of detected marker corners from detectMarkers function.
		/// If markerCorners and markerCorners are empty, the function detect aruco markers and ids.
		/// * markerIds: list of marker ids in markerCorners.
		/// If markerCorners and markerCorners are empty, the function detect aruco markers and ids.
		///
		/// This function detects Diamond markers from the previous detected ArUco markers. The diamonds
		/// are returned in the diamondCorners and diamondIds parameters. If camera calibration parameters
		/// are provided, the diamond search is based on reprojection. If not, diamond search is based on
		/// homography. Homography is faster than reprojection, but less accurate.
		///
		/// ## Note
		/// This alternative version of [CharucoDetectorTraitConst::detect_diamonds] function uses the following default values for its arguments:
		/// * marker_corners: noArray()
		/// * marker_ids: noArray()
		#[inline]
		fn detect_diamonds_def(&self, image: &impl ToInputArray, diamond_corners: &mut impl ToOutputArray, diamond_ids: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(diamond_corners);
			output_array_arg!(diamond_ids);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_detectDiamonds_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_CharucoDetector(), image.as_raw__InputArray(), diamond_corners.as_raw__OutputArray(), diamond_ids.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::objdetect::CharucoDetector]
	pub trait CharucoDetectorTrait: core::AlgorithmTrait + crate::objdetect::CharucoDetectorTraitConst {
		fn as_raw_mut_CharucoDetector(&mut self) -> *mut c_void;

		#[inline]
		fn set_board(&mut self, board: &impl crate::objdetect::CharucoBoardTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_setBoard_const_CharucoBoardR(self.as_raw_mut_CharucoDetector(), board.as_raw_CharucoBoard(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_charuco_parameters(&mut self, charuco_parameters: &mut impl crate::objdetect::CharucoParametersTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_setCharucoParameters_CharucoParametersR(self.as_raw_mut_CharucoDetector(), charuco_parameters.as_raw_mut_CharucoParameters(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_detector_parameters(&mut self, detector_parameters: &impl crate::objdetect::DetectorParametersTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_setDetectorParameters_const_DetectorParametersR(self.as_raw_mut_CharucoDetector(), detector_parameters.as_raw_DetectorParameters(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_refine_parameters(&mut self, refine_parameters: crate::objdetect::RefineParameters) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_setRefineParameters_const_RefineParametersR(self.as_raw_mut_CharucoDetector(), &refine_parameters, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for CharucoDetector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CharucoDetector")
				.finish()
		}
	}

	boxed_cast_base! { CharucoDetector, core::Algorithm, cv_aruco_CharucoDetector_to_Algorithm }

	impl core::AlgorithmTraitConst for CharucoDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for CharucoDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CharucoDetector, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::objdetect::CharucoDetectorTraitConst for CharucoDetector {
		#[inline] fn as_raw_CharucoDetector(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::objdetect::CharucoDetectorTrait for CharucoDetector {
		#[inline] fn as_raw_mut_CharucoDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CharucoDetector, crate::objdetect::CharucoDetectorTraitConst, as_raw_CharucoDetector, crate::objdetect::CharucoDetectorTrait, as_raw_mut_CharucoDetector }

	pub struct CharucoParameters {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { CharucoParameters }

	impl Drop for CharucoParameters {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_aruco_CharucoParameters_delete(self.as_raw_mut_CharucoParameters()) };
		}
	}

	unsafe impl Send for CharucoParameters {}

	impl CharucoParameters {
		#[inline]
		pub fn default() -> Result<crate::objdetect::CharucoParameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoParameters_CharucoParameters(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::CharucoParameters::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::objdetect::CharucoParameters]
	pub trait CharucoParametersTraitConst {
		fn as_raw_CharucoParameters(&self) -> *const c_void;

		/// cameraMatrix optional 3x3 floating-point camera matrix
		#[inline]
		fn camera_matrix(&self) -> core::Mat {
			let ret = unsafe { sys::cv_aruco_CharucoParameters_propCameraMatrix_const(self.as_raw_CharucoParameters()) };
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			ret
		}

		/// distCoeffs optional vector of distortion coefficients
		#[inline]
		fn dist_coeffs(&self) -> core::Mat {
			let ret = unsafe { sys::cv_aruco_CharucoParameters_propDistCoeffs_const(self.as_raw_CharucoParameters()) };
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			ret
		}

		/// minMarkers number of adjacent markers that must be detected to return a charuco corner, default = 2
		#[inline]
		fn min_markers(&self) -> i32 {
			let ret = unsafe { sys::cv_aruco_CharucoParameters_propMinMarkers_const(self.as_raw_CharucoParameters()) };
			ret
		}

		/// try to use refine board, default false
		#[inline]
		fn try_refine_markers(&self) -> bool {
			let ret = unsafe { sys::cv_aruco_CharucoParameters_propTryRefineMarkers_const(self.as_raw_CharucoParameters()) };
			ret
		}

		/// run check to verify that markers belong to the same board, default true
		#[inline]
		fn check_markers(&self) -> bool {
			let ret = unsafe { sys::cv_aruco_CharucoParameters_propCheckMarkers_const(self.as_raw_CharucoParameters()) };
			ret
		}

	}

	/// Mutable methods for [crate::objdetect::CharucoParameters]
	pub trait CharucoParametersTrait: crate::objdetect::CharucoParametersTraitConst {
		fn as_raw_mut_CharucoParameters(&mut self) -> *mut c_void;

		/// cameraMatrix optional 3x3 floating-point camera matrix
		#[inline]
		fn set_camera_matrix(&mut self, val: core::Mat) {
			let ret = unsafe { sys::cv_aruco_CharucoParameters_propCameraMatrix_const_Mat(self.as_raw_mut_CharucoParameters(), val.as_raw_Mat()) };
			ret
		}

		/// distCoeffs optional vector of distortion coefficients
		#[inline]
		fn set_dist_coeffs(&mut self, val: core::Mat) {
			let ret = unsafe { sys::cv_aruco_CharucoParameters_propDistCoeffs_const_Mat(self.as_raw_mut_CharucoParameters(), val.as_raw_Mat()) };
			ret
		}

		/// minMarkers number of adjacent markers that must be detected to return a charuco corner, default = 2
		#[inline]
		fn set_min_markers(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_CharucoParameters_propMinMarkers_const_int(self.as_raw_mut_CharucoParameters(), val) };
			ret
		}

		/// try to use refine board, default false
		#[inline]
		fn set_try_refine_markers(&mut self, val: bool) {
			let ret = unsafe { sys::cv_aruco_CharucoParameters_propTryRefineMarkers_const_bool(self.as_raw_mut_CharucoParameters(), val) };
			ret
		}

		/// run check to verify that markers belong to the same board, default true
		#[inline]
		fn set_check_markers(&mut self, val: bool) {
			let ret = unsafe { sys::cv_aruco_CharucoParameters_propCheckMarkers_const_bool(self.as_raw_mut_CharucoParameters(), val) };
			ret
		}

	}

	impl Clone for CharucoParameters {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_aruco_CharucoParameters_implicitClone_const(self.as_raw_CharucoParameters())) }
		}
	}

	impl std::fmt::Debug for CharucoParameters {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CharucoParameters")
				.field("camera_matrix", &crate::objdetect::CharucoParametersTraitConst::camera_matrix(self))
				.field("dist_coeffs", &crate::objdetect::CharucoParametersTraitConst::dist_coeffs(self))
				.field("min_markers", &crate::objdetect::CharucoParametersTraitConst::min_markers(self))
				.field("try_refine_markers", &crate::objdetect::CharucoParametersTraitConst::try_refine_markers(self))
				.field("check_markers", &crate::objdetect::CharucoParametersTraitConst::check_markers(self))
				.finish()
		}
	}

	impl crate::objdetect::CharucoParametersTraitConst for CharucoParameters {
		#[inline] fn as_raw_CharucoParameters(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::objdetect::CharucoParametersTrait for CharucoParameters {
		#[inline] fn as_raw_mut_CharucoParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CharucoParameters, crate::objdetect::CharucoParametersTraitConst, as_raw_CharucoParameters, crate::objdetect::CharucoParametersTrait, as_raw_mut_CharucoParameters }

	/// struct DetectorParameters is used by ArucoDetector
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

	impl DetectorParameters {
		#[inline]
		pub fn default() -> Result<crate::objdetect::DetectorParameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_DetectorParameters_DetectorParameters(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::DetectorParameters::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::objdetect::DetectorParameters]
	pub trait DetectorParametersTraitConst {
		fn as_raw_DetectorParameters(&self) -> *const c_void;

		/// minimum window size for adaptive thresholding before finding contours (default 3).
		#[inline]
		fn adaptive_thresh_win_size_min(&self) -> i32 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMin_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// maximum window size for adaptive thresholding before finding contours (default 23).
		#[inline]
		fn adaptive_thresh_win_size_max(&self) -> i32 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMax_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// increments from adaptiveThreshWinSizeMin to adaptiveThreshWinSizeMax during the thresholding (default 10).
		#[inline]
		fn adaptive_thresh_win_size_step(&self) -> i32 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeStep_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// constant for adaptive thresholding before finding contours (default 7)
		#[inline]
		fn adaptive_thresh_constant(&self) -> f64 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAdaptiveThreshConstant_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// determine minimum perimeter for marker contour to be detected.
		///
		/// This is defined as a rate respect to the maximum dimension of the input image (default 0.03).
		#[inline]
		fn min_marker_perimeter_rate(&self) -> f64 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinMarkerPerimeterRate_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// determine maximum perimeter for marker contour to be detected.
		///
		/// This is defined as a rate respect to the maximum dimension of the input image (default 4.0).
		#[inline]
		fn max_marker_perimeter_rate(&self) -> f64 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMaxMarkerPerimeterRate_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// minimum accuracy during the polygonal approximation process to determine which contours are squares. (default 0.03)
		#[inline]
		fn polygonal_approx_accuracy_rate(&self) -> f64 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propPolygonalApproxAccuracyRate_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// minimum distance between corners for detected markers relative to its perimeter (default 0.05)
		#[inline]
		fn min_corner_distance_rate(&self) -> f64 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinCornerDistanceRate_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// minimum distance of any corner to the image border for detected markers (in pixels) (default 3)
		#[inline]
		fn min_distance_to_border(&self) -> i32 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinDistanceToBorder_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// minimum average distance between the corners of the two markers to be grouped (default 0.125).
		///
		/// The rate is relative to the smaller perimeter of the two markers.
		/// Two markers are grouped if average distance between the corners of the two markers is less than
		/// min(MarkerPerimeter1, MarkerPerimeter2)*minMarkerDistanceRate.
		///
		/// default value is 0.125 because 0.125*MarkerPerimeter = (MarkerPerimeter / 4) * 0.5 = half the side of the marker.
		///
		///
		/// Note: default value was changed from 0.05 after 4.8.1 release, because the filtering algorithm has been changed.
		/// Now a few candidates from the same group can be added to the list of candidates if they are far from each other.
		/// ## See also
		/// minGroupDistance.
		#[inline]
		fn min_marker_distance_rate(&self) -> f64 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinMarkerDistanceRate_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// minimum average distance between the corners of the two markers in group to add them to the list of candidates
		///
		/// The average distance between the corners of the two markers is calculated relative to its module size (default 0.21).
		#[inline]
		fn min_group_distance(&self) -> f32 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinGroupDistance_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// default value CORNER_REFINE_NONE
		#[inline]
		fn corner_refinement_method(&self) -> i32 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propCornerRefinementMethod_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// maximum window size for the corner refinement process (in pixels) (default 5).
		///
		/// The window size may decrease if the ArUco marker is too small, check relativeCornerRefinmentWinSize.
		/// The final window size is calculated as:
		/// min(cornerRefinementWinSize, averageArucoModuleSize*relativeCornerRefinmentWinSize),
		/// where averageArucoModuleSize is average module size of ArUco marker in pixels.
		/// (ArUco marker is composed of black and white modules)
		#[inline]
		fn corner_refinement_win_size(&self) -> i32 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propCornerRefinementWinSize_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// Dynamic window size for corner refinement relative to Aruco module size (default 0.3).
		///
		/// The final window size is calculated as:
		/// min(cornerRefinementWinSize, averageArucoModuleSize*relativeCornerRefinmentWinSize),
		/// where averageArucoModuleSize is average module size of ArUco marker in pixels.
		/// (ArUco marker is composed of black and white modules)
		/// In the case of markers located far from each other, it may be useful to increase the value of the parameter to 0.4-0.5.
		/// In the case of markers located close to each other, it may be useful to decrease the parameter value to 0.1-0.2.
		#[inline]
		fn relative_corner_refinment_win_size(&self) -> f32 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propRelativeCornerRefinmentWinSize_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// maximum number of iterations for stop criteria of the corner refinement process (default 30).
		#[inline]
		fn corner_refinement_max_iterations(&self) -> i32 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propCornerRefinementMaxIterations_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// minimum error for the stop cristeria of the corner refinement process (default: 0.1)
		#[inline]
		fn corner_refinement_min_accuracy(&self) -> f64 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propCornerRefinementMinAccuracy_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// number of bits of the marker border, i.e. marker border width (default 1).
		#[inline]
		fn marker_border_bits(&self) -> i32 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMarkerBorderBits_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// number of bits (per dimension) for each cell of the marker when removing the perspective (default 4).
		#[inline]
		fn perspective_remove_pixel_per_cell(&self) -> i32 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propPerspectiveRemovePixelPerCell_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// width of the margin of pixels on each cell not considered for the determination of the cell bit.
		///
		/// Represents the rate respect to the total size of the cell, i.e. perspectiveRemovePixelPerCell (default 0.13)
		#[inline]
		fn perspective_remove_ignored_margin_per_cell(&self) -> f64 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propPerspectiveRemoveIgnoredMarginPerCell_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// maximum number of accepted erroneous bits in the border (i.e. number of allowed white bits in the border).
		///
		/// Represented as a rate respect to the total number of bits per marker (default 0.35).
		#[inline]
		fn max_erroneous_bits_in_border_rate(&self) -> f64 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMaxErroneousBitsInBorderRate_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// minimum standard deviation in pixels values during the decodification step to apply Otsu
		/// thresholding (otherwise, all the bits are set to 0 or 1 depending on mean higher than 128 or not) (default 5.0)
		#[inline]
		fn min_otsu_std_dev(&self) -> f64 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinOtsuStdDev_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// error correction rate respect to the maximum error correction capability for each dictionary (default 0.6).
		#[inline]
		fn error_correction_rate(&self) -> f64 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propErrorCorrectionRate_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// April :: User-configurable parameters.
		///
		/// Detection of quads can be done on a lower-resolution image, improving speed at a cost of
		/// pose accuracy and a slight decrease in detection rate. Decoding the binary payload is still
		#[inline]
		fn april_tag_quad_decimate(&self) -> f32 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagQuadDecimate_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// what Gaussian blur should be applied to the segmented image (used for quad detection?)
		#[inline]
		fn april_tag_quad_sigma(&self) -> f32 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagQuadSigma_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// reject quads containing too few pixels (default 5).
		#[inline]
		fn april_tag_min_cluster_pixels(&self) -> i32 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagMinClusterPixels_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// how many corner candidates to consider when segmenting a group of pixels into a quad (default 10).
		#[inline]
		fn april_tag_max_nmaxima(&self) -> i32 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagMaxNmaxima_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// reject quads where pairs of edges have angles that are close to straight or close to 180 degrees.
		///
		/// Zero means that no quads are rejected. (In radians) (default 10*PI/180)
		#[inline]
		fn april_tag_critical_rad(&self) -> f32 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagCriticalRad_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// when fitting lines to the contours, what is the maximum mean squared error
		#[inline]
		fn april_tag_max_line_fit_mse(&self) -> f32 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagMaxLineFitMse_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// add an extra check that the white model must be (overall) brighter than the black model.
		///
		/// When we build our model of black & white pixels, we add an extra check that the white model must be (overall)
		/// brighter than the black model. How much brighter? (in pixel values, [0,255]), (default 5)
		#[inline]
		fn april_tag_min_white_black_diff(&self) -> i32 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagMinWhiteBlackDiff_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// should the thresholded image be deglitched? Only useful for very noisy images (default 0).
		#[inline]
		fn april_tag_deglitch(&self) -> i32 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagDeglitch_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// to check if there is a white marker.
		///
		/// In order to generate a "white" marker just invert a normal marker by using a tilde, ~markerImage. (default false)
		#[inline]
		fn detect_inverted_marker(&self) -> bool {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propDetectInvertedMarker_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// enable the new and faster Aruco detection strategy.
		///
		/// Proposed in the paper:
		/// Romero-Ramirez et al: Speeded up detection of squared fiducial markers (2018)
		/// <https://www.researchgate.net/publication/325787310_Speeded_Up_Detection_of_Squared_Fiducial_Markers>
		#[inline]
		fn use_aruco3_detection(&self) -> bool {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propUseAruco3Detection_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// minimum side length of a marker in the canonical image. Latter is the binarized image in which contours are searched.
		#[inline]
		fn min_side_length_canonical_img(&self) -> i32 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinSideLengthCanonicalImg_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// range [0,1], eq (2) from paper. The parameter tau_i has a direct influence on the processing speed.
		#[inline]
		fn min_marker_length_ratio_original_img(&self) -> f32 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinMarkerLengthRatioOriginalImg_const(self.as_raw_DetectorParameters()) };
			ret
		}

		/// range [0,1], define the acceptable threshold when comparing the detected marker to the dictionary during marker identification.
		#[inline]
		fn valid_bit_id_threshold(&self) -> f32 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propValidBitIdThreshold_const(self.as_raw_DetectorParameters()) };
			ret
		}

	}

	/// Mutable methods for [crate::objdetect::DetectorParameters]
	pub trait DetectorParametersTrait: crate::objdetect::DetectorParametersTraitConst {
		fn as_raw_mut_DetectorParameters(&mut self) -> *mut c_void;

		/// minimum window size for adaptive thresholding before finding contours (default 3).
		#[inline]
		fn set_adaptive_thresh_win_size_min(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMin_const_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// maximum window size for adaptive thresholding before finding contours (default 23).
		#[inline]
		fn set_adaptive_thresh_win_size_max(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMax_const_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// increments from adaptiveThreshWinSizeMin to adaptiveThreshWinSizeMax during the thresholding (default 10).
		#[inline]
		fn set_adaptive_thresh_win_size_step(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeStep_const_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// constant for adaptive thresholding before finding contours (default 7)
		#[inline]
		fn set_adaptive_thresh_constant(&mut self, val: f64) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAdaptiveThreshConstant_const_double(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// determine minimum perimeter for marker contour to be detected.
		///
		/// This is defined as a rate respect to the maximum dimension of the input image (default 0.03).
		#[inline]
		fn set_min_marker_perimeter_rate(&mut self, val: f64) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinMarkerPerimeterRate_const_double(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// determine maximum perimeter for marker contour to be detected.
		///
		/// This is defined as a rate respect to the maximum dimension of the input image (default 4.0).
		#[inline]
		fn set_max_marker_perimeter_rate(&mut self, val: f64) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMaxMarkerPerimeterRate_const_double(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// minimum accuracy during the polygonal approximation process to determine which contours are squares. (default 0.03)
		#[inline]
		fn set_polygonal_approx_accuracy_rate(&mut self, val: f64) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propPolygonalApproxAccuracyRate_const_double(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// minimum distance between corners for detected markers relative to its perimeter (default 0.05)
		#[inline]
		fn set_min_corner_distance_rate(&mut self, val: f64) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinCornerDistanceRate_const_double(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// minimum distance of any corner to the image border for detected markers (in pixels) (default 3)
		#[inline]
		fn set_min_distance_to_border(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinDistanceToBorder_const_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// minimum average distance between the corners of the two markers to be grouped (default 0.125).
		///
		/// The rate is relative to the smaller perimeter of the two markers.
		/// Two markers are grouped if average distance between the corners of the two markers is less than
		/// min(MarkerPerimeter1, MarkerPerimeter2)*minMarkerDistanceRate.
		///
		/// default value is 0.125 because 0.125*MarkerPerimeter = (MarkerPerimeter / 4) * 0.5 = half the side of the marker.
		///
		///
		/// Note: default value was changed from 0.05 after 4.8.1 release, because the filtering algorithm has been changed.
		/// Now a few candidates from the same group can be added to the list of candidates if they are far from each other.
		/// ## See also
		/// minGroupDistance.
		#[inline]
		fn set_min_marker_distance_rate(&mut self, val: f64) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinMarkerDistanceRate_const_double(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// minimum average distance between the corners of the two markers in group to add them to the list of candidates
		///
		/// The average distance between the corners of the two markers is calculated relative to its module size (default 0.21).
		///
		/// ## C++ default parameters
		/// * val: 0.21f
		#[inline]
		fn set_min_group_distance(&mut self, val: f32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinGroupDistance_const_float(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// default value CORNER_REFINE_NONE
		#[inline]
		fn set_corner_refinement_method(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propCornerRefinementMethod_const_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// maximum window size for the corner refinement process (in pixels) (default 5).
		///
		/// The window size may decrease if the ArUco marker is too small, check relativeCornerRefinmentWinSize.
		/// The final window size is calculated as:
		/// min(cornerRefinementWinSize, averageArucoModuleSize*relativeCornerRefinmentWinSize),
		/// where averageArucoModuleSize is average module size of ArUco marker in pixels.
		/// (ArUco marker is composed of black and white modules)
		#[inline]
		fn set_corner_refinement_win_size(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propCornerRefinementWinSize_const_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// Dynamic window size for corner refinement relative to Aruco module size (default 0.3).
		///
		/// The final window size is calculated as:
		/// min(cornerRefinementWinSize, averageArucoModuleSize*relativeCornerRefinmentWinSize),
		/// where averageArucoModuleSize is average module size of ArUco marker in pixels.
		/// (ArUco marker is composed of black and white modules)
		/// In the case of markers located far from each other, it may be useful to increase the value of the parameter to 0.4-0.5.
		/// In the case of markers located close to each other, it may be useful to decrease the parameter value to 0.1-0.2.
		#[inline]
		fn set_relative_corner_refinment_win_size(&mut self, val: f32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propRelativeCornerRefinmentWinSize_const_float(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// maximum number of iterations for stop criteria of the corner refinement process (default 30).
		#[inline]
		fn set_corner_refinement_max_iterations(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propCornerRefinementMaxIterations_const_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// minimum error for the stop cristeria of the corner refinement process (default: 0.1)
		#[inline]
		fn set_corner_refinement_min_accuracy(&mut self, val: f64) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propCornerRefinementMinAccuracy_const_double(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// number of bits of the marker border, i.e. marker border width (default 1).
		#[inline]
		fn set_marker_border_bits(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMarkerBorderBits_const_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// number of bits (per dimension) for each cell of the marker when removing the perspective (default 4).
		#[inline]
		fn set_perspective_remove_pixel_per_cell(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propPerspectiveRemovePixelPerCell_const_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// width of the margin of pixels on each cell not considered for the determination of the cell bit.
		///
		/// Represents the rate respect to the total size of the cell, i.e. perspectiveRemovePixelPerCell (default 0.13)
		#[inline]
		fn set_perspective_remove_ignored_margin_per_cell(&mut self, val: f64) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propPerspectiveRemoveIgnoredMarginPerCell_const_double(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// maximum number of accepted erroneous bits in the border (i.e. number of allowed white bits in the border).
		///
		/// Represented as a rate respect to the total number of bits per marker (default 0.35).
		#[inline]
		fn set_max_erroneous_bits_in_border_rate(&mut self, val: f64) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMaxErroneousBitsInBorderRate_const_double(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// minimum standard deviation in pixels values during the decodification step to apply Otsu
		/// thresholding (otherwise, all the bits are set to 0 or 1 depending on mean higher than 128 or not) (default 5.0)
		#[inline]
		fn set_min_otsu_std_dev(&mut self, val: f64) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinOtsuStdDev_const_double(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// error correction rate respect to the maximum error correction capability for each dictionary (default 0.6).
		#[inline]
		fn set_error_correction_rate(&mut self, val: f64) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propErrorCorrectionRate_const_double(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// April :: User-configurable parameters.
		///
		/// Detection of quads can be done on a lower-resolution image, improving speed at a cost of
		/// pose accuracy and a slight decrease in detection rate. Decoding the binary payload is still
		#[inline]
		fn set_april_tag_quad_decimate(&mut self, val: f32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagQuadDecimate_const_float(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// what Gaussian blur should be applied to the segmented image (used for quad detection?)
		#[inline]
		fn set_april_tag_quad_sigma(&mut self, val: f32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagQuadSigma_const_float(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// reject quads containing too few pixels (default 5).
		#[inline]
		fn set_april_tag_min_cluster_pixels(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagMinClusterPixels_const_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// how many corner candidates to consider when segmenting a group of pixels into a quad (default 10).
		#[inline]
		fn set_april_tag_max_nmaxima(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagMaxNmaxima_const_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// reject quads where pairs of edges have angles that are close to straight or close to 180 degrees.
		///
		/// Zero means that no quads are rejected. (In radians) (default 10*PI/180)
		#[inline]
		fn set_april_tag_critical_rad(&mut self, val: f32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagCriticalRad_const_float(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// when fitting lines to the contours, what is the maximum mean squared error
		#[inline]
		fn set_april_tag_max_line_fit_mse(&mut self, val: f32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagMaxLineFitMse_const_float(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// add an extra check that the white model must be (overall) brighter than the black model.
		///
		/// When we build our model of black & white pixels, we add an extra check that the white model must be (overall)
		/// brighter than the black model. How much brighter? (in pixel values, [0,255]), (default 5)
		#[inline]
		fn set_april_tag_min_white_black_diff(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagMinWhiteBlackDiff_const_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// should the thresholded image be deglitched? Only useful for very noisy images (default 0).
		#[inline]
		fn set_april_tag_deglitch(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagDeglitch_const_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// to check if there is a white marker.
		///
		/// In order to generate a "white" marker just invert a normal marker by using a tilde, ~markerImage. (default false)
		#[inline]
		fn set_detect_inverted_marker(&mut self, val: bool) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propDetectInvertedMarker_const_bool(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// enable the new and faster Aruco detection strategy.
		///
		/// Proposed in the paper:
		/// Romero-Ramirez et al: Speeded up detection of squared fiducial markers (2018)
		/// <https://www.researchgate.net/publication/325787310_Speeded_Up_Detection_of_Squared_Fiducial_Markers>
		#[inline]
		fn set_use_aruco3_detection(&mut self, val: bool) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propUseAruco3Detection_const_bool(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// minimum side length of a marker in the canonical image. Latter is the binarized image in which contours are searched.
		#[inline]
		fn set_min_side_length_canonical_img(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinSideLengthCanonicalImg_const_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// range [0,1], eq (2) from paper. The parameter tau_i has a direct influence on the processing speed.
		#[inline]
		fn set_min_marker_length_ratio_original_img(&mut self, val: f32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinMarkerLengthRatioOriginalImg_const_float(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// range [0,1], define the acceptable threshold when comparing the detected marker to the dictionary during marker identification.
		#[inline]
		fn set_valid_bit_id_threshold(&mut self, val: f32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propValidBitIdThreshold_const_float(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}

		/// Read a new set of DetectorParameters from FileNode (use FileStorage.root()).
		#[inline]
		fn read_detector_parameters(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_DetectorParameters_readDetectorParameters_const_FileNodeR(self.as_raw_mut_DetectorParameters(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Write a set of DetectorParameters to FileStorage
		///
		/// ## C++ default parameters
		/// * name: String()
		#[inline]
		fn write_detector_parameters(&mut self, fs: &mut impl core::FileStorageTrait, name: &str) -> Result<bool> {
			extern_container_arg!(name);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_DetectorParameters_writeDetectorParameters_FileStorageR_const_StringR(self.as_raw_mut_DetectorParameters(), fs.as_raw_mut_FileStorage(), name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Write a set of DetectorParameters to FileStorage
		///
		/// ## Note
		/// This alternative version of [DetectorParametersTrait::write_detector_parameters] function uses the following default values for its arguments:
		/// * name: String()
		#[inline]
		fn write_detector_parameters_def(&mut self, fs: &mut impl core::FileStorageTrait) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_DetectorParameters_writeDetectorParameters_FileStorageR(self.as_raw_mut_DetectorParameters(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
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
				.field("adaptive_thresh_win_size_min", &crate::objdetect::DetectorParametersTraitConst::adaptive_thresh_win_size_min(self))
				.field("adaptive_thresh_win_size_max", &crate::objdetect::DetectorParametersTraitConst::adaptive_thresh_win_size_max(self))
				.field("adaptive_thresh_win_size_step", &crate::objdetect::DetectorParametersTraitConst::adaptive_thresh_win_size_step(self))
				.field("adaptive_thresh_constant", &crate::objdetect::DetectorParametersTraitConst::adaptive_thresh_constant(self))
				.field("min_marker_perimeter_rate", &crate::objdetect::DetectorParametersTraitConst::min_marker_perimeter_rate(self))
				.field("max_marker_perimeter_rate", &crate::objdetect::DetectorParametersTraitConst::max_marker_perimeter_rate(self))
				.field("polygonal_approx_accuracy_rate", &crate::objdetect::DetectorParametersTraitConst::polygonal_approx_accuracy_rate(self))
				.field("min_corner_distance_rate", &crate::objdetect::DetectorParametersTraitConst::min_corner_distance_rate(self))
				.field("min_distance_to_border", &crate::objdetect::DetectorParametersTraitConst::min_distance_to_border(self))
				.field("min_marker_distance_rate", &crate::objdetect::DetectorParametersTraitConst::min_marker_distance_rate(self))
				.field("min_group_distance", &crate::objdetect::DetectorParametersTraitConst::min_group_distance(self))
				.field("corner_refinement_method", &crate::objdetect::DetectorParametersTraitConst::corner_refinement_method(self))
				.field("corner_refinement_win_size", &crate::objdetect::DetectorParametersTraitConst::corner_refinement_win_size(self))
				.field("relative_corner_refinment_win_size", &crate::objdetect::DetectorParametersTraitConst::relative_corner_refinment_win_size(self))
				.field("corner_refinement_max_iterations", &crate::objdetect::DetectorParametersTraitConst::corner_refinement_max_iterations(self))
				.field("corner_refinement_min_accuracy", &crate::objdetect::DetectorParametersTraitConst::corner_refinement_min_accuracy(self))
				.field("marker_border_bits", &crate::objdetect::DetectorParametersTraitConst::marker_border_bits(self))
				.field("perspective_remove_pixel_per_cell", &crate::objdetect::DetectorParametersTraitConst::perspective_remove_pixel_per_cell(self))
				.field("perspective_remove_ignored_margin_per_cell", &crate::objdetect::DetectorParametersTraitConst::perspective_remove_ignored_margin_per_cell(self))
				.field("max_erroneous_bits_in_border_rate", &crate::objdetect::DetectorParametersTraitConst::max_erroneous_bits_in_border_rate(self))
				.field("min_otsu_std_dev", &crate::objdetect::DetectorParametersTraitConst::min_otsu_std_dev(self))
				.field("error_correction_rate", &crate::objdetect::DetectorParametersTraitConst::error_correction_rate(self))
				.field("april_tag_quad_decimate", &crate::objdetect::DetectorParametersTraitConst::april_tag_quad_decimate(self))
				.field("april_tag_quad_sigma", &crate::objdetect::DetectorParametersTraitConst::april_tag_quad_sigma(self))
				.field("april_tag_min_cluster_pixels", &crate::objdetect::DetectorParametersTraitConst::april_tag_min_cluster_pixels(self))
				.field("april_tag_max_nmaxima", &crate::objdetect::DetectorParametersTraitConst::april_tag_max_nmaxima(self))
				.field("april_tag_critical_rad", &crate::objdetect::DetectorParametersTraitConst::april_tag_critical_rad(self))
				.field("april_tag_max_line_fit_mse", &crate::objdetect::DetectorParametersTraitConst::april_tag_max_line_fit_mse(self))
				.field("april_tag_min_white_black_diff", &crate::objdetect::DetectorParametersTraitConst::april_tag_min_white_black_diff(self))
				.field("april_tag_deglitch", &crate::objdetect::DetectorParametersTraitConst::april_tag_deglitch(self))
				.field("detect_inverted_marker", &crate::objdetect::DetectorParametersTraitConst::detect_inverted_marker(self))
				.field("use_aruco3_detection", &crate::objdetect::DetectorParametersTraitConst::use_aruco3_detection(self))
				.field("min_side_length_canonical_img", &crate::objdetect::DetectorParametersTraitConst::min_side_length_canonical_img(self))
				.field("min_marker_length_ratio_original_img", &crate::objdetect::DetectorParametersTraitConst::min_marker_length_ratio_original_img(self))
				.field("valid_bit_id_threshold", &crate::objdetect::DetectorParametersTraitConst::valid_bit_id_threshold(self))
				.finish()
		}
	}

	impl crate::objdetect::DetectorParametersTraitConst for DetectorParameters {
		#[inline] fn as_raw_DetectorParameters(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::objdetect::DetectorParametersTrait for DetectorParameters {
		#[inline] fn as_raw_mut_DetectorParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { DetectorParameters, crate::objdetect::DetectorParametersTraitConst, as_raw_DetectorParameters, crate::objdetect::DetectorParametersTrait, as_raw_mut_DetectorParameters }

	/// Dictionary is a set of unique ArUco markers of the same size
	///
	/// `bytesList` storing as 2-dimensions Mat with 4-th channels (CV_8UC4 type was used) and contains the marker codewords where:
	/// - bytesList.rows is the dictionary size
	/// - each marker is encoded using `nbytes = ceil(markerSize*markerSize/8.)` bytes
	/// - each row contains all 4 rotations of the marker, so its length is `4*nbytes`
	/// - the byte order in the bytesList[i] row:
	/// `//bytes without rotation/bytes with rotation 1/bytes with rotation 2/bytes with rotation 3//`
	/// So `bytesList.ptr(i)[k*nbytes + j]` is the j-th byte of i-th marker, in its k-th rotation.
	///
	/// Note: Python bindings generate matrix with shape of bytesList `dictionary_size x nbytes x 4`,
	/// but it should be indexed like C++ version. Python example for j-th byte of i-th marker, in its k-th rotation:
	/// `aruco_dict.bytesList[id].ravel()[k*nbytes + j]`
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

	impl Dictionary {
		#[inline]
		pub fn default() -> Result<crate::objdetect::Dictionary> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_Dictionary(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::Dictionary::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Basic ArUco dictionary constructor
		///
		/// ## Parameters
		/// * bytesList: bits for all ArUco markers in dictionary see memory layout in the class description
		/// * _markerSize: ArUco marker size in units
		/// * maxcorr: maximum number of bits that can be corrected
		///
		/// ## C++ default parameters
		/// * maxcorr: 0
		#[inline]
		pub fn new(bytes_list: &impl core::MatTraitConst, _marker_size: i32, maxcorr: i32) -> Result<crate::objdetect::Dictionary> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_Dictionary_const_MatR_int_int(bytes_list.as_raw_Mat(), _marker_size, maxcorr, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::Dictionary::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Basic ArUco dictionary constructor
		///
		/// ## Parameters
		/// * bytesList: bits for all ArUco markers in dictionary see memory layout in the class description
		/// * _markerSize: ArUco marker size in units
		/// * maxcorr: maximum number of bits that can be corrected
		///
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * maxcorr: 0
		#[inline]
		pub fn new_def(bytes_list: &impl core::MatTraitConst, _marker_size: i32) -> Result<crate::objdetect::Dictionary> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_Dictionary_const_MatR_int(bytes_list.as_raw_Mat(), _marker_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::Dictionary::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Transform matrix of bits to list of bytes with 4 marker rotations
		#[inline]
		pub fn get_byte_list_from_bits(bits: &impl core::MatTraitConst) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_getByteListFromBits_const_MatR(bits.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Transform list of bytes to matrix of bits
		///
		/// ## C++ default parameters
		/// * rotation_id: 0
		#[inline]
		pub fn get_bits_from_byte_list(byte_list: &impl core::MatTraitConst, marker_size: i32, rotation_id: i32) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_getBitsFromByteList_const_MatR_int_int(byte_list.as_raw_Mat(), marker_size, rotation_id, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Transform list of bytes to matrix of bits
		///
		/// ## Note
		/// This alternative version of [Dictionary::get_bits_from_byte_list] function uses the following default values for its arguments:
		/// * rotation_id: 0
		#[inline]
		pub fn get_bits_from_byte_list_def(byte_list: &impl core::MatTraitConst, marker_size: i32) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_getBitsFromByteList_const_MatR_int(byte_list.as_raw_Mat(), marker_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::objdetect::Dictionary]
	pub trait DictionaryTraitConst {
		fn as_raw_Dictionary(&self) -> *const c_void;

		/// marker code information. See class description for more details
		#[inline]
		fn bytes_list(&self) -> core::Mat {
			let ret = unsafe { sys::cv_aruco_Dictionary_propBytesList_const(self.as_raw_Dictionary()) };
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			ret
		}

		/// number of bits per dimension
		#[inline]
		fn marker_size(&self) -> i32 {
			let ret = unsafe { sys::cv_aruco_Dictionary_propMarkerSize_const(self.as_raw_Dictionary()) };
			ret
		}

		/// maximum number of bits that can be corrected
		#[inline]
		fn max_correction_bits(&self) -> i32 {
			let ret = unsafe { sys::cv_aruco_Dictionary_propMaxCorrectionBits_const(self.as_raw_Dictionary()) };
			ret
		}

		/// Given a matrix of bits. Returns whether if marker is identified or not.
		///
		/// Returns reference to the marker id in the dictionary (if any) and its rotation.
		#[inline]
		fn identify(&self, only_bits: &impl core::MatTraitConst, idx: &mut i32, rotation: &mut i32, max_correction_rate: f64) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_identify_const_const_MatR_intR_intR_double(self.as_raw_Dictionary(), only_bits.as_raw_Mat(), idx, rotation, max_correction_rate, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Given a matrix of pixel ratio raging from 0 to 1. Returns whether if marker is identified or not.
		///
		/// Returns reference to the marker id in the dictionary (if any) and its rotation.
		#[inline]
		fn identify_1(&self, only_cell_pixel_ratio: &impl core::MatTraitConst, idx: &mut i32, rotation: &mut i32, max_correction_rate: f64, valid_bit_id_threshold: f32) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_identify_const_const_MatR_intR_intR_double_float(self.as_raw_Dictionary(), only_cell_pixel_ratio.as_raw_Mat(), idx, rotation, max_correction_rate, valid_bit_id_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns Hamming distance of the input bits to the specific id.
		///
		/// If `allRotations` flag is set, the four possible marker rotations are considered
		///
		/// ## C++ default parameters
		/// * all_rotations: true
		#[inline]
		fn get_distance_to_id(&self, bits: &impl ToInputArray, id: i32, all_rotations: bool) -> Result<i32> {
			input_array_arg!(bits);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_getDistanceToId_const_const__InputArrayR_int_bool(self.as_raw_Dictionary(), bits.as_raw__InputArray(), id, all_rotations, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns Hamming distance of the input bits to the specific id.
		///
		/// If `allRotations` flag is set, the four possible marker rotations are considered
		///
		/// ## Note
		/// This alternative version of [DictionaryTraitConst::get_distance_to_id] function uses the following default values for its arguments:
		/// * all_rotations: true
		#[inline]
		fn get_distance_to_id_def(&self, bits: &impl ToInputArray, id: i32) -> Result<i32> {
			input_array_arg!(bits);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_getDistanceToId_const_const__InputArrayR_int(self.as_raw_Dictionary(), bits.as_raw__InputArray(), id, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Generate a canonical marker image
		///
		/// ## C++ default parameters
		/// * border_bits: 1
		#[inline]
		fn generate_image_marker(&self, id: i32, side_pixels: i32, _img: &mut impl ToOutputArray, border_bits: i32) -> Result<()> {
			output_array_arg!(_img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_generateImageMarker_const_int_int_const__OutputArrayR_int(self.as_raw_Dictionary(), id, side_pixels, _img.as_raw__OutputArray(), border_bits, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Generate a canonical marker image
		///
		/// ## Note
		/// This alternative version of [DictionaryTraitConst::generate_image_marker] function uses the following default values for its arguments:
		/// * border_bits: 1
		#[inline]
		fn generate_image_marker_def(&self, id: i32, side_pixels: i32, _img: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(_img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_generateImageMarker_const_int_int_const__OutputArrayR(self.as_raw_Dictionary(), id, side_pixels, _img.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get ground truth bits float
		///
		/// ## C++ default parameters
		/// * rotation_id: 0
		#[inline]
		fn get_marker_bits(&self, marker_id: i32, rotation_id: i32) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_getMarkerBits_const_int_int(self.as_raw_Dictionary(), marker_id, rotation_id, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Get ground truth bits float
		///
		/// ## Note
		/// This alternative version of [DictionaryTraitConst::get_marker_bits] function uses the following default values for its arguments:
		/// * rotation_id: 0
		#[inline]
		fn get_marker_bits_def(&self, marker_id: i32) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_getMarkerBits_const_int(self.as_raw_Dictionary(), marker_id, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::objdetect::Dictionary]
	pub trait DictionaryTrait: crate::objdetect::DictionaryTraitConst {
		fn as_raw_mut_Dictionary(&mut self) -> *mut c_void;

		/// marker code information. See class description for more details
		#[inline]
		fn set_bytes_list(&mut self, val: core::Mat) {
			let ret = unsafe { sys::cv_aruco_Dictionary_propBytesList_const_Mat(self.as_raw_mut_Dictionary(), val.as_raw_Mat()) };
			ret
		}

		/// number of bits per dimension
		#[inline]
		fn set_marker_size(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_Dictionary_propMarkerSize_const_int(self.as_raw_mut_Dictionary(), val) };
			ret
		}

		/// maximum number of bits that can be corrected
		#[inline]
		fn set_max_correction_bits(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_Dictionary_propMaxCorrectionBits_const_int(self.as_raw_mut_Dictionary(), val) };
			ret
		}

		/// Read a new dictionary from FileNode.
		///
		/// Dictionary example in YAML format:
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
		#[inline]
		fn read_dictionary(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_readDictionary_const_FileNodeR(self.as_raw_mut_Dictionary(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Write a dictionary to FileStorage, format is the same as in readDictionary().
		///
		/// ## C++ default parameters
		/// * name: String()
		#[inline]
		fn write_dictionary(&mut self, fs: &mut impl core::FileStorageTrait, name: &str) -> Result<()> {
			extern_container_arg!(name);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_writeDictionary_FileStorageR_const_StringR(self.as_raw_mut_Dictionary(), fs.as_raw_mut_FileStorage(), name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Write a dictionary to FileStorage, format is the same as in readDictionary().
		///
		/// ## Note
		/// This alternative version of [DictionaryTrait::write_dictionary] function uses the following default values for its arguments:
		/// * name: String()
		#[inline]
		fn write_dictionary_def(&mut self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_writeDictionary_FileStorageR(self.as_raw_mut_Dictionary(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl Clone for Dictionary {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_aruco_Dictionary_implicitClone_const(self.as_raw_Dictionary())) }
		}
	}

	impl std::fmt::Debug for Dictionary {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Dictionary")
				.field("bytes_list", &crate::objdetect::DictionaryTraitConst::bytes_list(self))
				.field("marker_size", &crate::objdetect::DictionaryTraitConst::marker_size(self))
				.field("max_correction_bits", &crate::objdetect::DictionaryTraitConst::max_correction_bits(self))
				.finish()
		}
	}

	impl crate::objdetect::DictionaryTraitConst for Dictionary {
		#[inline] fn as_raw_Dictionary(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::objdetect::DictionaryTrait for Dictionary {
		#[inline] fn as_raw_mut_Dictionary(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Dictionary, crate::objdetect::DictionaryTraitConst, as_raw_Dictionary, crate::objdetect::DictionaryTrait, as_raw_mut_Dictionary }

	/// Planar board with grid arrangement of markers
	///
	/// More common type of board. All markers are placed in the same plane in a grid arrangement.
	/// The board image can be drawn using generateImage() method.
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

	impl GridBoard {
		/// GridBoard constructor
		///
		/// ## Parameters
		/// * size: number of markers in x and y directions
		/// * markerLength: marker side length (normally in meters)
		/// * markerSeparation: separation between two markers (same unit as markerLength)
		/// * dictionary: dictionary of markers indicating the type of markers
		/// * ids: set of marker ids in dictionary to use on board.
		///
		/// ## C++ default parameters
		/// * ids: noArray()
		#[inline]
		pub fn new(size: core::Size, marker_length: f32, marker_separation: f32, dictionary: &impl crate::objdetect::DictionaryTraitConst, ids: &impl ToInputArray) -> Result<crate::objdetect::GridBoard> {
			input_array_arg!(ids);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_GridBoard_GridBoard_const_SizeR_float_float_const_DictionaryR_const__InputArrayR(&size, marker_length, marker_separation, dictionary.as_raw_Dictionary(), ids.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::GridBoard::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// GridBoard constructor
		///
		/// ## Parameters
		/// * size: number of markers in x and y directions
		/// * markerLength: marker side length (normally in meters)
		/// * markerSeparation: separation between two markers (same unit as markerLength)
		/// * dictionary: dictionary of markers indicating the type of markers
		/// * ids: set of marker ids in dictionary to use on board.
		///
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * ids: noArray()
		#[inline]
		pub fn new_def(size: core::Size, marker_length: f32, marker_separation: f32, dictionary: &impl crate::objdetect::DictionaryTraitConst) -> Result<crate::objdetect::GridBoard> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_GridBoard_GridBoard_const_SizeR_float_float_const_DictionaryR(&size, marker_length, marker_separation, dictionary.as_raw_Dictionary(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::GridBoard::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		pub fn default() -> Result<crate::objdetect::GridBoard> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_GridBoard_GridBoard(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::GridBoard::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::objdetect::GridBoard]
	pub trait GridBoardTraitConst: crate::objdetect::BoardTraitConst {
		fn as_raw_GridBoard(&self) -> *const c_void;

		#[inline]
		fn get_grid_size(&self) -> Result<core::Size> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_GridBoard_getGridSize_const(self.as_raw_GridBoard(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_marker_length(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_GridBoard_getMarkerLength_const(self.as_raw_GridBoard(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_marker_separation(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_GridBoard_getMarkerSeparation_const(self.as_raw_GridBoard(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::objdetect::GridBoard]
	pub trait GridBoardTrait: crate::objdetect::BoardTrait + crate::objdetect::GridBoardTraitConst {
		fn as_raw_mut_GridBoard(&mut self) -> *mut c_void;

	}

	impl Clone for GridBoard {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_aruco_GridBoard_implicitClone_const(self.as_raw_GridBoard())) }
		}
	}

	impl std::fmt::Debug for GridBoard {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GridBoard")
				.finish()
		}
	}

	boxed_cast_base! { GridBoard, crate::objdetect::Board, cv_aruco_GridBoard_to_Board }

	impl crate::objdetect::BoardTraitConst for GridBoard {
		#[inline] fn as_raw_Board(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::objdetect::BoardTrait for GridBoard {
		#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { GridBoard, crate::objdetect::BoardTraitConst, as_raw_Board, crate::objdetect::BoardTrait, as_raw_mut_Board }

	impl crate::objdetect::GridBoardTraitConst for GridBoard {
		#[inline] fn as_raw_GridBoard(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::objdetect::GridBoardTrait for GridBoard {
		#[inline] fn as_raw_mut_GridBoard(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { GridBoard, crate::objdetect::GridBoardTraitConst, as_raw_GridBoard, crate::objdetect::GridBoardTrait, as_raw_mut_GridBoard }

	/// struct RefineParameters is used by ArucoDetector
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq)]
	pub struct RefineParameters {
		/// minRepDistance minimum distance between the corners of the rejected candidate and the reprojected marker
		/// in order to consider it as a correspondence.
		pub min_rep_distance: f32,
		/// errorCorrectionRate rate of allowed erroneous bits respect to the error correction capability of the used dictionary.
		///
		/// -1 ignores the error correction step.
		pub error_correction_rate: f32,
		/// checkAllOrders consider the four possible corner orders in the rejectedCorners array.
		///
		/// If it set to false, only the provided corner order is considered (default true).
		pub check_all_orders: bool,
	}

	opencv_type_simple! { crate::objdetect::RefineParameters }

	impl RefineParameters {
		/// ## C++ default parameters
		/// * min_rep_distance: 10.f
		/// * error_correction_rate: 3.f
		/// * check_all_orders: true
		#[inline]
		pub fn new(min_rep_distance: f32, error_correction_rate: f32, check_all_orders: bool) -> Result<crate::objdetect::RefineParameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_RefineParameters_RefineParameters_float_float_bool(min_rep_distance, error_correction_rate, check_all_orders, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * min_rep_distance: 10.f
		/// * error_correction_rate: 3.f
		/// * check_all_orders: true
		#[inline]
		pub fn new_def() -> Result<crate::objdetect::RefineParameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_RefineParameters_RefineParameters(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Read a new set of RefineParameters from FileNode (use FileStorage.root()).
		#[inline]
		pub fn read_refine_parameters(self, fn_: &impl core::FileNodeTraitConst) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_RefineParameters_readRefineParameters_const_FileNodeR(&self, fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Write a set of RefineParameters to FileStorage
		///
		/// ## C++ default parameters
		/// * name: String()
		#[inline]
		pub fn write_refine_parameters(self, fs: &mut impl core::FileStorageTrait, name: &str) -> Result<bool> {
			extern_container_arg!(name);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_RefineParameters_writeRefineParameters_FileStorageR_const_StringR(&self, fs.as_raw_mut_FileStorage(), name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Write a set of RefineParameters to FileStorage
		///
		/// ## Note
		/// This alternative version of [RefineParameters::write_refine_parameters] function uses the following default values for its arguments:
		/// * name: String()
		#[inline]
		pub fn write_refine_parameters_def(self, fs: &mut impl core::FileStorageTrait) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_RefineParameters_writeRefineParameters_FileStorageR(&self, fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	pub struct BarcodeDetector {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { BarcodeDetector }

	impl Drop for BarcodeDetector {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_barcode_BarcodeDetector_delete(self.as_raw_mut_BarcodeDetector()) };
		}
	}

	unsafe impl Send for BarcodeDetector {}

	impl BarcodeDetector {
		/// Initialize the BarcodeDetector. Super resolution is disabled.
		#[inline]
		pub fn default() -> Result<crate::objdetect::BarcodeDetector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_barcode_BarcodeDetector_BarcodeDetector(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::BarcodeDetector::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Initialize the BarcodeDetector with a Super Resolution model.
		///
		/// Loads a Super Resolution DNN model in ONNX format, used to upscale small/low-quality
		/// barcode crops before decoding for better quality.
		///
		///
		/// Note: Caffe models (`sr.prototxt` / `sr.caffemodel`) are no longer supported; convert
		/// the model to ONNX (a converted `sr.onnx` is available from
		/// <https://github.com/WeChatCV/opencv_3rdparty/tree/wechat_qrcode>).
		///
		/// ## Parameters
		/// * super_resolution_model_path: path to a single-file ONNX Super Resolution model.
		#[inline]
		pub fn new(super_resolution_model_path: &str) -> Result<crate::objdetect::BarcodeDetector> {
			extern_container_arg!(super_resolution_model_path);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_barcode_BarcodeDetector_BarcodeDetector_const_stringR(super_resolution_model_path.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::BarcodeDetector::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::objdetect::BarcodeDetector]
	pub trait BarcodeDetectorTraitConst: crate::objdetect::GraphicalCodeDetectorTraitConst {
		fn as_raw_BarcodeDetector(&self) -> *const c_void;

		/// Decodes barcode in image once it's found by the detect() method.
		///
		/// ## Parameters
		/// * img: grayscale or color (BGR) image containing bar code.
		/// * points: vector of rotated rectangle vertices found by detect() method (or some other algorithm).
		/// For N detected barcodes, the dimensions of this array should be [N][4].
		/// Order of four points in vector<Point2f> is bottomLeft, topLeft, topRight, bottomRight.
		/// * decoded_info: UTF8-encoded output vector of string or empty vector of string if the codes cannot be decoded.
		/// * decoded_type: vector strings, specifies the type of these barcodes
		/// ## Returns
		/// true if at least one valid barcode have been found
		#[inline]
		fn decode_with_type(&self, img: &impl ToInputArray, points: &impl ToInputArray, decoded_info: &mut core::Vector<String>, decoded_type: &mut core::Vector<String>) -> Result<bool> {
			input_array_arg!(img);
			input_array_arg!(points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_barcode_BarcodeDetector_decodeWithType_const_const__InputArrayR_const__InputArrayR_vectorLstringGR_vectorLstringGR(self.as_raw_BarcodeDetector(), img.as_raw__InputArray(), points.as_raw__InputArray(), decoded_info.as_raw_mut_VectorOfString(), decoded_type.as_raw_mut_VectorOfString(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Both detects and decodes barcode
		///
		/// ## Parameters
		/// * img: grayscale or color (BGR) image containing barcode.
		/// * decoded_info: UTF8-encoded output vector of string(s) or empty vector of string if the codes cannot be decoded.
		/// * decoded_type: vector of strings, specifies the type of these barcodes
		/// * points: optional output vector of vertices of the found  barcode rectangle. Will be empty if not found.
		/// ## Returns
		/// true if at least one valid barcode have been found
		///
		/// ## C++ default parameters
		/// * points: noArray()
		#[inline]
		fn detect_and_decode_with_type(&self, img: &impl ToInputArray, decoded_info: &mut core::Vector<String>, decoded_type: &mut core::Vector<String>, points: &mut impl ToOutputArray) -> Result<bool> {
			input_array_arg!(img);
			output_array_arg!(points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_barcode_BarcodeDetector_detectAndDecodeWithType_const_const__InputArrayR_vectorLstringGR_vectorLstringGR_const__OutputArrayR(self.as_raw_BarcodeDetector(), img.as_raw__InputArray(), decoded_info.as_raw_mut_VectorOfString(), decoded_type.as_raw_mut_VectorOfString(), points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Both detects and decodes barcode
		///
		/// ## Parameters
		/// * img: grayscale or color (BGR) image containing barcode.
		/// * decoded_info: UTF8-encoded output vector of string(s) or empty vector of string if the codes cannot be decoded.
		/// * decoded_type: vector of strings, specifies the type of these barcodes
		/// * points: optional output vector of vertices of the found  barcode rectangle. Will be empty if not found.
		/// ## Returns
		/// true if at least one valid barcode have been found
		///
		/// ## Note
		/// This alternative version of [BarcodeDetectorTraitConst::detect_and_decode_with_type] function uses the following default values for its arguments:
		/// * points: noArray()
		#[inline]
		fn detect_and_decode_with_type_def(&self, img: &impl ToInputArray, decoded_info: &mut core::Vector<String>, decoded_type: &mut core::Vector<String>) -> Result<bool> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_barcode_BarcodeDetector_detectAndDecodeWithType_const_const__InputArrayR_vectorLstringGR_vectorLstringGR(self.as_raw_BarcodeDetector(), img.as_raw__InputArray(), decoded_info.as_raw_mut_VectorOfString(), decoded_type.as_raw_mut_VectorOfString(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get detector downsampling threshold.
		///
		/// ## Returns
		/// detector downsampling threshold
		#[inline]
		fn get_downsampling_threshold(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_barcode_BarcodeDetector_getDownsamplingThreshold_const(self.as_raw_BarcodeDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns detector box filter sizes.
		///
		/// ## Parameters
		/// * sizes: output parameter for returning the sizes.
		#[inline]
		fn get_detector_scales(&self, sizes: &mut core::Vector<f32>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_barcode_BarcodeDetector_getDetectorScales_const_vectorLfloatGR(self.as_raw_BarcodeDetector(), sizes.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get detector gradient magnitude threshold.
		///
		/// ## Returns
		/// detector gradient magnitude threshold.
		#[inline]
		fn get_gradient_threshold(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_barcode_BarcodeDetector_getGradientThreshold_const(self.as_raw_BarcodeDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::objdetect::BarcodeDetector]
	pub trait BarcodeDetectorTrait: crate::objdetect::BarcodeDetectorTraitConst + crate::objdetect::GraphicalCodeDetectorTrait {
		fn as_raw_mut_BarcodeDetector(&mut self) -> *mut c_void;

		/// Set detector downsampling threshold.
		///
		/// By default, the detect method resizes the input image to this limit if the smallest image size is is greater than the threshold.
		/// Increasing this value can improve detection accuracy and the number of results at the expense of performance.
		/// Correlates with detector scales. Setting this to a large value will disable downsampling.
		/// ## Parameters
		/// * thresh: downsampling limit to apply (default 512)
		/// ## See also
		/// setDetectorScales
		#[inline]
		fn set_downsampling_threshold(&mut self, thresh: f64) -> Result<crate::objdetect::BarcodeDetector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_barcode_BarcodeDetector_setDownsamplingThreshold_double(self.as_raw_mut_BarcodeDetector(), thresh, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::BarcodeDetector::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Set detector box filter sizes.
		///
		/// Adjusts the value and the number of box filters used in the detect step.
		/// The filter sizes directly correlate with the expected line widths for a barcode. Corresponds to expected barcode distance.
		/// If the downsampling limit is increased, filter sizes need to be adjusted in an inversely proportional way.
		/// ## Parameters
		/// * sizes: box filter sizes, relative to minimum dimension of the image (default [0.01, 0.03, 0.06, 0.08])
		#[inline]
		fn set_detector_scales(&mut self, sizes: &core::Vector<f32>) -> Result<crate::objdetect::BarcodeDetector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_barcode_BarcodeDetector_setDetectorScales_const_vectorLfloatGR(self.as_raw_mut_BarcodeDetector(), sizes.as_raw_VectorOff32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::BarcodeDetector::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Set detector gradient magnitude threshold.
		///
		/// Sets the coherence threshold for detected bounding boxes.
		/// Increasing this value will generate a closer fitted bounding box width and can reduce false-positives.
		/// Values between 16 and 1024 generally work, while too high of a value will remove valid detections.
		/// ## Parameters
		/// * thresh: gradient magnitude threshold (default 64).
		#[inline]
		fn set_gradient_threshold(&mut self, thresh: f64) -> Result<crate::objdetect::BarcodeDetector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_barcode_BarcodeDetector_setGradientThreshold_double(self.as_raw_mut_BarcodeDetector(), thresh, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::BarcodeDetector::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl Clone for BarcodeDetector {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_barcode_BarcodeDetector_implicitClone_const(self.as_raw_BarcodeDetector())) }
		}
	}

	impl std::fmt::Debug for BarcodeDetector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("BarcodeDetector")
				.finish()
		}
	}

	boxed_cast_base! { BarcodeDetector, crate::objdetect::GraphicalCodeDetector, cv_barcode_BarcodeDetector_to_GraphicalCodeDetector }

	impl crate::objdetect::GraphicalCodeDetectorTraitConst for BarcodeDetector {
		#[inline] fn as_raw_GraphicalCodeDetector(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::objdetect::GraphicalCodeDetectorTrait for BarcodeDetector {
		#[inline] fn as_raw_mut_GraphicalCodeDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { BarcodeDetector, crate::objdetect::GraphicalCodeDetectorTraitConst, as_raw_GraphicalCodeDetector, crate::objdetect::GraphicalCodeDetectorTrait, as_raw_mut_GraphicalCodeDetector }

	impl crate::objdetect::BarcodeDetectorTraitConst for BarcodeDetector {
		#[inline] fn as_raw_BarcodeDetector(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::objdetect::BarcodeDetectorTrait for BarcodeDetector {
		#[inline] fn as_raw_mut_BarcodeDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { BarcodeDetector, crate::objdetect::BarcodeDetectorTraitConst, as_raw_BarcodeDetector, crate::objdetect::BarcodeDetectorTrait, as_raw_mut_BarcodeDetector }

	/// CChecker
	///
	/// checker object
	///
	///    This class contains the information about the detected checkers,i.e, their
	///    type, the corners of the chart, the color profile, the cost, centers chart,
	///    etc.
	pub struct CChecker {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { CChecker }

	impl Drop for CChecker {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_mcc_CChecker_delete(self.as_raw_mut_CChecker()) };
		}
	}

	unsafe impl Send for CChecker {}

	impl CChecker {
		/// Create a new CChecker object.
		///
		/// ## Returns
		/// A pointer to the implementation of the CChecker
		#[inline]
		pub fn create() -> Result<core::Ptr<crate::objdetect::CChecker>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CChecker_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::objdetect::CChecker>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::objdetect::CChecker]
	pub trait CCheckerTraitConst: core::AlgorithmTraitConst {
		fn as_raw_CChecker(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::objdetect::CChecker]
	pub trait CCheckerTrait: core::AlgorithmTrait + crate::objdetect::CCheckerTraitConst {
		fn as_raw_mut_CChecker(&mut self) -> *mut c_void;

		#[inline]
		fn set_target(&mut self, _target: crate::objdetect::ColorChart) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CChecker_setTarget_ColorChart(self.as_raw_mut_CChecker(), _target, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_box(&mut self, mut _box: core::Vector<core::Point2f>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CChecker_setBox_vectorLPoint2fG(self.as_raw_mut_CChecker(), _box.as_raw_mut_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_charts_rgb(&mut self, mut _charts_rgb: impl core::MatTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CChecker_setChartsRGB_Mat(self.as_raw_mut_CChecker(), _charts_rgb.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_charts_y_cb_cr(&mut self, mut _charts_y_cb_cr: impl core::MatTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CChecker_setChartsYCbCr_Mat(self.as_raw_mut_CChecker(), _charts_y_cb_cr.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_cost(&mut self, _cost: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CChecker_setCost_float(self.as_raw_mut_CChecker(), _cost, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_center(&mut self, _center: core::Point2f) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CChecker_setCenter_Point2f(self.as_raw_mut_CChecker(), &_center, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_target(&mut self) -> Result<crate::objdetect::ColorChart> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CChecker_getTarget(self.as_raw_mut_CChecker(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_box(&mut self) -> Result<core::Vector<core::Point2f>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CChecker_getBox(self.as_raw_mut_CChecker(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Point2f>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Computes and returns the coordinates of the central parts of the charts modules.
		///
		/// This method computes transformation matrix from the checkers's coordinates (`CChecker::getBox()`)
		/// and find by this the coordinates of the central parts of the charts modules.
		/// It is used in `CCheckerDetector::draw()` and in `ChartsRGB` calculation.
		#[inline]
		fn get_color_charts(&mut self) -> Result<core::Vector<core::Point2f>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CChecker_getColorCharts(self.as_raw_mut_CChecker(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Point2f>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// ## C++ default parameters
		/// * get_stats: true
		#[inline]
		fn get_charts_rgb(&mut self, get_stats: bool) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CChecker_getChartsRGB_bool(self.as_raw_mut_CChecker(), get_stats, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// ## Note
		/// This alternative version of [CCheckerTrait::get_charts_rgb] function uses the following default values for its arguments:
		/// * get_stats: true
		#[inline]
		fn get_charts_rgb_def(&mut self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CChecker_getChartsRGB(self.as_raw_mut_CChecker(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn get_charts_y_cb_cr(&mut self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CChecker_getChartsYCbCr(self.as_raw_mut_CChecker(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn get_cost(&mut self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CChecker_getCost(self.as_raw_mut_CChecker(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_center(&mut self) -> Result<core::Point2f> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CChecker_getCenter(self.as_raw_mut_CChecker(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for CChecker {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CChecker")
				.finish()
		}
	}

	boxed_cast_base! { CChecker, core::Algorithm, cv_mcc_CChecker_to_Algorithm }

	impl core::AlgorithmTraitConst for CChecker {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for CChecker {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CChecker, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::objdetect::CCheckerTraitConst for CChecker {
		#[inline] fn as_raw_CChecker(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::objdetect::CCheckerTrait for CChecker {
		#[inline] fn as_raw_mut_CChecker(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CChecker, crate::objdetect::CCheckerTraitConst, as_raw_CChecker, crate::objdetect::CCheckerTrait, as_raw_mut_CChecker }

	/// A class to find the positions of the ColorCharts in the image.
	pub struct CCheckerDetector {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { CCheckerDetector }

	impl Drop for CCheckerDetector {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_mcc_CCheckerDetector_delete(self.as_raw_mut_CCheckerDetector()) };
		}
	}

	unsafe impl Send for CCheckerDetector {}

	impl CCheckerDetector {
		/// Returns the implementation of the CCheckerDetector.
		#[inline]
		pub fn create() -> Result<core::Ptr<crate::objdetect::CCheckerDetector>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CCheckerDetector_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::objdetect::CCheckerDetector>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Set the net which will be used to find the approximate
		///        bounding boxes for the color charts. And returns the implementation of the CCheckerDetector.
		///
		/// It is not necessary to use this, but this usually results in
		/// better detection rate.
		///
		/// ## Parameters
		/// * net: the neural network, if the network in empty, then
		///            the function will return false.
		#[inline]
		pub fn create_1(net: &impl crate::dnn::NetTraitConst) -> Result<core::Ptr<crate::objdetect::CCheckerDetector>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CCheckerDetector_create_const_NetR(net.as_raw_Net(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::objdetect::CCheckerDetector>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::objdetect::CCheckerDetector]
	pub trait CCheckerDetectorTraitConst: core::AlgorithmTraitConst {
		fn as_raw_CCheckerDetector(&self) -> *const c_void;

		#[inline]
		fn get_use_dnn_model(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CCheckerDetector_getUseDnnModel_const(self.as_raw_CCheckerDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_detection_params(&self) -> Result<crate::objdetect::DetectorParametersMCC> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CCheckerDetector_getDetectionParams_const(self.as_raw_CCheckerDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_color_chart_type(&self) -> Result<crate::objdetect::ColorChart> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CCheckerDetector_getColorChartType_const(self.as_raw_CCheckerDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::objdetect::CCheckerDetector]
	pub trait CCheckerDetectorTrait: core::AlgorithmTrait + crate::objdetect::CCheckerDetectorTraitConst {
		fn as_raw_mut_CCheckerDetector(&mut self) -> *mut c_void;

		/// Find the ColorCharts in the given image.
		///
		/// The found charts are not returned but instead stored in the
		/// detector, these can be accessed later on using getBestColorChecker()
		/// and getListColorChecker()
		/// ## Parameters
		/// * image: image in color space BGR
		/// * regionsOfInterest: regions of image to look for the chart, if
		///                          it is empty, charts are looked for in the
		///                          entire image
		/// * nc: number of charts in the image, if you don't know the exact
		///           then keeping this number high helps.
		/// ## Returns
		/// true if atleast one chart is detected otherwise false
		///
		/// ## C++ default parameters
		/// * nc: 1
		#[inline]
		fn process_with_roi(&mut self, image: &impl ToInputArray, regions_of_interest: &core::Vector<core::Rect>, nc: i32) -> Result<bool> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CCheckerDetector_process_const__InputArrayR_const_vectorLRectGR_const_int(self.as_raw_mut_CCheckerDetector(), image.as_raw__InputArray(), regions_of_interest.as_raw_VectorOfRect(), nc, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Find the ColorCharts in the given image.
		///
		/// The found charts are not returned but instead stored in the
		/// detector, these can be accessed later on using getBestColorChecker()
		/// and getListColorChecker()
		/// ## Parameters
		/// * image: image in color space BGR
		/// * regionsOfInterest: regions of image to look for the chart, if
		///                          it is empty, charts are looked for in the
		///                          entire image
		/// * nc: number of charts in the image, if you don't know the exact
		///           then keeping this number high helps.
		/// ## Returns
		/// true if atleast one chart is detected otherwise false
		///
		/// ## Note
		/// This alternative version of [CCheckerDetectorTrait::process_with_roi] function uses the following default values for its arguments:
		/// * nc: 1
		#[inline]
		fn process_with_roi_def(&mut self, image: &impl ToInputArray, regions_of_interest: &core::Vector<core::Rect>) -> Result<bool> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CCheckerDetector_process_const__InputArrayR_const_vectorLRectGR(self.as_raw_mut_CCheckerDetector(), image.as_raw__InputArray(), regions_of_interest.as_raw_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Find the ColorCharts in the given image.
		///
		/// Differs from the above one only in the arguments.
		///
		/// This version searches for the chart in the full image.
		///
		/// The found charts are not returned but instead stored in the
		/// detector, these can be accessed later on using getBestColorChecker()
		/// and getListColorChecker()
		/// ## Parameters
		/// * image: image in color space BGR
		/// * nc: number of charts in the image, if you don't know the exact
		///           then keeping this number high helps.
		/// ## Returns
		/// true if atleast one chart is detected otherwise false
		///
		/// ## C++ default parameters
		/// * nc: 1
		#[inline]
		fn process(&mut self, image: &impl ToInputArray, nc: i32) -> Result<bool> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CCheckerDetector_process_const__InputArrayR_const_int(self.as_raw_mut_CCheckerDetector(), image.as_raw__InputArray(), nc, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Find the ColorCharts in the given image.
		///
		/// Differs from the above one only in the arguments.
		///
		/// This version searches for the chart in the full image.
		///
		/// The found charts are not returned but instead stored in the
		/// detector, these can be accessed later on using getBestColorChecker()
		/// and getListColorChecker()
		/// ## Parameters
		/// * image: image in color space BGR
		/// * nc: number of charts in the image, if you don't know the exact
		///           then keeping this number high helps.
		/// ## Returns
		/// true if atleast one chart is detected otherwise false
		///
		/// ## Note
		/// This alternative version of [CCheckerDetectorTrait::process] function uses the following default values for its arguments:
		/// * nc: 1
		#[inline]
		fn process_def(&mut self, image: &impl ToInputArray) -> Result<bool> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CCheckerDetector_process_const__InputArrayR(self.as_raw_mut_CCheckerDetector(), image.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get the best color checker. By the best it means the one
		///        detected with the highest confidence.
		/// ## Returns
		/// checker A single colorchecker, if atleast one colorchecker
		///                was detected, 'nullptr' otherwise.
		#[inline]
		fn get_best_color_checker(&mut self) -> Result<core::Ptr<crate::objdetect::CChecker>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CCheckerDetector_getBestColorChecker(self.as_raw_mut_CCheckerDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::objdetect::CChecker>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Get the list of all detected colorcheckers
		/// ## Returns
		/// checkers vector of colorcheckers
		#[inline]
		fn get_list_color_checker(&mut self) -> Result<core::Vector<core::Ptr<crate::objdetect::CChecker>>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CCheckerDetector_getListColorChecker(self.as_raw_mut_CCheckerDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Ptr<crate::objdetect::CChecker>>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Draws the checker to the given image.
		/// ## Parameters
		/// * img: image in color space BGR
		/// * checkers: The checkers which will be drawn by this object.
		/// * color: The color by with which the squares of the checker
		///        will be drawn
		/// * thickness: The thickness with which the sqaures will be
		///        drawn
		///
		/// ## C++ default parameters
		/// * color: CV_RGB(0,250,0)
		/// * thickness: 2
		#[inline]
		fn draw(&mut self, checkers: &mut core::Vector<core::Ptr<crate::objdetect::CChecker>>, img: &mut impl ToInputOutputArray, color: core::Scalar, thickness: i32) -> Result<()> {
			input_output_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CCheckerDetector_draw_vectorLPtrLCCheckerGGR_const__InputOutputArrayR_const_Scalar_const_int(self.as_raw_mut_CCheckerDetector(), checkers.as_raw_mut_VectorOfPtrOfCChecker(), img.as_raw__InputOutputArray(), &color, thickness, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Draws the checker to the given image.
		/// ## Parameters
		/// * img: image in color space BGR
		/// * checkers: The checkers which will be drawn by this object.
		/// * color: The color by with which the squares of the checker
		///        will be drawn
		/// * thickness: The thickness with which the sqaures will be
		///        drawn
		///
		/// ## Note
		/// This alternative version of [CCheckerDetectorTrait::draw] function uses the following default values for its arguments:
		/// * color: CV_RGB(0,250,0)
		/// * thickness: 2
		#[inline]
		fn draw_def(&mut self, checkers: &mut core::Vector<core::Ptr<crate::objdetect::CChecker>>, img: &mut impl ToInputOutputArray) -> Result<()> {
			input_output_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CCheckerDetector_draw_vectorLPtrLCCheckerGGR_const__InputOutputArrayR(self.as_raw_mut_CCheckerDetector(), checkers.as_raw_mut_VectorOfPtrOfCChecker(), img.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Gets the reference color for chart.
		#[inline]
		fn get_ref_colors(&mut self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CCheckerDetector_getRefColors(self.as_raw_mut_CCheckerDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Sets the detection paramaters for mcc.
		/// ## Parameters
		/// * params: DetectorParametersMCC structure containing detection configuration parameters.
		#[inline]
		fn set_detection_params(&mut self, params: crate::objdetect::DetectorParametersMCC) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CCheckerDetector_setDetectionParams_const_DetectorParametersMCCR(self.as_raw_mut_CCheckerDetector(), &params, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Sets the color chart type for MCC detection.
		/// ## Parameters
		/// * chartType: ColorChart enum specifying the type of color chart to detect.
		#[inline]
		fn set_color_chart_type(&mut self, chart_type: crate::objdetect::ColorChart) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CCheckerDetector_setColorChartType_ColorChart(self.as_raw_mut_CCheckerDetector(), chart_type, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Enables or disables the use of the neural network for detection.
		/// ## Parameters
		/// * useDnn: Boolean flag to indicate whether to use neural network (true) or not (false).
		#[inline]
		fn set_use_dnn_model(&mut self, use_dnn: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_CCheckerDetector_setUseDnnModel_bool(self.as_raw_mut_CCheckerDetector(), use_dnn, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for CCheckerDetector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CCheckerDetector")
				.finish()
		}
	}

	boxed_cast_base! { CCheckerDetector, core::Algorithm, cv_mcc_CCheckerDetector_to_Algorithm }

	impl core::AlgorithmTraitConst for CCheckerDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for CCheckerDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CCheckerDetector, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::objdetect::CCheckerDetectorTraitConst for CCheckerDetector {
		#[inline] fn as_raw_CCheckerDetector(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::objdetect::CCheckerDetectorTrait for CCheckerDetector {
		#[inline] fn as_raw_mut_CCheckerDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CCheckerDetector, crate::objdetect::CCheckerDetectorTraitConst, as_raw_CCheckerDetector, crate::objdetect::CCheckerDetectorTrait, as_raw_mut_CCheckerDetector }

	/// struct DetectorParametersMCC is used by CCheckerDetector
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq)]
	pub struct DetectorParametersMCC {
		/// minimum window size for adaptive thresholding before finding contours (default 23).
		pub adaptive_thresh_win_size_min: i32,
		/// maximum window size for adaptive thresholding before finding contours (default 153).
		pub adaptive_thresh_win_size_max: i32,
		/// increments from adaptiveThreshWinSizeMin to adaptiveThreshWinSizeMax during the thresholding (default 16).
		pub adaptive_thresh_win_size_step: i32,
		/// constant for adaptive thresholding before finding contours (default 7)
		pub adaptive_thresh_constant: f64,
		/// determine minimum area for marker contour to be detected
		///
		/// This is defined as a rate respect to the area of the input image. Used only if neural network is used (default 0.03).
		pub min_contours_area_rate: f64,
		/// determine minimum area for marker contour to be detected
		///
		/// This is defined as the actual area. Used only if neural network is used (default 100).
		pub min_contours_area: f64,
		/// minimum confidence for a bounding box detected by neural network to classify as detection.(default 0.5) (0<=confidenceThreshold<=1)
		pub confidence_threshold: f64,
		/// minimum solidity of a contour for it be detected as a square in the chart. (default 0.9).
		pub min_contour_solidity: f64,
		/// multipler to be used in ApproxPolyDP function (default 0.05)
		pub find_candidates_approx_poly_dp_eps_multiplier: f64,
		/// width of the padding used to pass the inital neural network detection in the succeeding system.(default 0)
		pub border_width: i32,
		/// distance between two neighboring squares of the same chart as a ratio of the large dimension of a square (default 1.25).
		pub b0factor: f32,
		/// maximum allowed error in the detection of a chart (default 0.1).
		pub max_error: f32,
		/// minimum points in a detected contour (default 4).
		pub min_contour_points_allowed: i32,
		/// minimum length of a contour (default 100).
		pub min_contour_length_allowed: i32,
		/// minimum distance between two contours (default 100).
		pub min_inter_contour_distance: i32,
		/// minimum distance between two checkers (default 10000).
		pub min_inter_checker_distance: i32,
		/// minimum size of the smaller dimension of the image (default 1000).
		pub min_image_size: i32,
		/// minimum number of squares in a chart that must be detected (default 4).
		pub min_group_size: i32,
	}

	opencv_type_simple! { crate::objdetect::DetectorParametersMCC }

	impl DetectorParametersMCC {
		#[inline]
		pub fn default() -> Result<crate::objdetect::DetectorParametersMCC> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_mcc_DetectorParametersMCC_DetectorParametersMCC(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

}
