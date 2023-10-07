pub mod objdetect {
	//! # Object Detection
	//!    # Cascade Classifier for Object Detection
	//! 
	//! The object detector described below has been initially proposed by Paul Viola [Viola01](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Viola01) and
	//! improved by Rainer Lienhart [Lienhart02](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Lienhart02) .
	//! 
	//! First, a classifier (namely a *cascade of boosted classifiers working with haar-like features*) is
	//! trained with a few hundred sample views of a particular object (i.e., a face or a car), called
	//! positive examples, that are scaled to the same size (say, 20x20), and negative examples - arbitrary
	//! images of the same size.
	//! 
	//! After a classifier is trained, it can be applied to a region of interest (of the same size as used
	//! during the training) in an input image. The classifier outputs a "1" if the region is likely to show
	//! the object (i.e., face/car), and "0" otherwise. To search for the object in the whole image one can
	//! move the search window across the image and check every location using the classifier. The
	//! classifier is designed so that it can be easily "resized" in order to be able to find the objects of
	//! interest at different sizes, which is more efficient than resizing the image itself. So, to find an
	//! object of an unknown size in the image the scan procedure should be done several times at different
	//! scales.
	//! 
	//! The word "cascade" in the classifier name means that the resultant classifier consists of several
	//! simpler classifiers (*stages*) that are applied subsequently to a region of interest until at some
	//! stage the candidate is rejected or all the stages are passed. The word "boosted" means that the
	//! classifiers at every stage of the cascade are complex themselves and they are built out of basic
	//! classifiers using one of four different boosting techniques (weighted voting). Currently Discrete
	//! Adaboost, Real Adaboost, Gentle Adaboost and Logitboost are supported. The basic classifiers are
	//! decision-tree classifiers with at least 2 leaves. Haar-like features are the input to the basic
	//! classifiers, and are calculated as described below. The current algorithm uses the following
	//! Haar-like features:
	//! 
	//! ![image](https://docs.opencv.org/4.8.1/haarfeatures.png)
	//! 
	//! The feature used in a particular classifier is specified by its shape (1a, 2b etc.), position within
	//! the region of interest and the scale (this scale is not the same as the scale used at the detection
	//! stage, though these two scales are multiplied). For example, in the case of the third line feature
	//! (2c) the response is calculated as the difference between the sum of image pixels under the
	//! rectangle covering the whole feature (including the two white stripes and the black stripe in the
	//! middle) and the sum of the image pixels under the black stripe multiplied by 3 in order to
	//! compensate for the differences in the size of areas. The sums of pixel values over a rectangular
	//! regions are calculated rapidly using integral images (see below and the integral description).
	//! 
	//! Check [tutorial_cascade_classifier] "the corresponding tutorial" for more details.
	//! 
	//! The following reference is for the detection part only. There is a separate application called
	//! opencv_traincascade that can train a cascade of boosted classifiers from a set of samples.
	//! 
	//! 
	//! Note: In the new C++ interface it is also possible to use LBP (local binary pattern) features in
	//! addition to Haar-like features. .. [Viola01] Paul Viola and Michael J. Jones. Rapid Object Detection
	//! using a Boosted Cascade of Simple Features. IEEE CVPR, 2001. The paper is available online at
	//! <https://github.com/SvHey/thesis/blob/master/Literature/ObjectDetection/violaJones_CVPR2001.pdf>
	//! 
	//!    # HOG (Histogram of Oriented Gradients) descriptor and object detector
	//!    # Barcode detection and decoding
	//!    # QRCode detection and encoding
	//!    # DNN-based face detection and recognition
	//! Check [tutorial_dnn_face] "the corresponding tutorial" for more details.
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
	//!        The implementation is based on the ArUco Library by R. Muñoz-Salinas and S. Garrido-Jurado [Aruco2014](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Aruco2014).
	//! 
	//!        Markers can also be detected based on the AprilTag 2 [wang2016iros](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_wang2016iros) fiducial detection method.
	//! ## See also
	//! [Aruco2014](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Aruco2014)
	//!        This code has been originally developed by Sergio Garrido-Jurado as a project
	//!        for Google Summer of Code 2015 (GSoC 15).
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::DictionaryTraitConst, super::DictionaryTrait, super::BoardTraitConst, super::BoardTrait, super::GridBoardTraitConst, super::GridBoardTrait, super::CharucoBoardTraitConst, super::CharucoBoardTrait, super::DetectorParametersTraitConst, super::DetectorParametersTrait, super::ArucoDetectorTraitConst, super::ArucoDetectorTrait, super::GraphicalCodeDetectorTraitConst, super::GraphicalCodeDetectorTrait, super::SimilarRectsTraitConst, super::SimilarRectsTrait, super::BaseCascadeClassifier_MaskGeneratorTraitConst, super::BaseCascadeClassifier_MaskGeneratorTrait, super::BaseCascadeClassifierTraitConst, super::BaseCascadeClassifierTrait, super::CascadeClassifierTraitConst, super::CascadeClassifierTrait, super::DetectionROITraitConst, super::DetectionROITrait, super::HOGDescriptorTraitConst, super::HOGDescriptorTrait, super::QRCodeEncoderTraitConst, super::QRCodeEncoderTrait, super::QRCodeDetectorTraitConst, super::QRCodeDetectorTrait, super::QRCodeDetectorArucoTraitConst, super::QRCodeDetectorArucoTrait, super::DetectionBasedTracker_ParametersTraitConst, super::DetectionBasedTracker_ParametersTrait, super::DetectionBasedTracker_IDetectorTraitConst, super::DetectionBasedTracker_IDetectorTrait, super::DetectionBasedTracker_ExtObjectTraitConst, super::DetectionBasedTracker_ExtObjectTrait, super::DetectionBasedTrackerTraitConst, super::DetectionBasedTrackerTrait, super::FaceDetectorYNTraitConst, super::FaceDetectorYNTrait, super::FaceRecognizerSFTraitConst, super::FaceRecognizerSFTrait, super::CharucoParametersTraitConst, super::CharucoParametersTrait, super::CharucoDetectorTraitConst, super::CharucoDetectorTrait, super::BarcodeDetectorTraitConst, super::BarcodeDetectorTrait };
	}
	
	pub const CASCADE_DO_CANNY_PRUNING: i32 = 1;
	pub const CASCADE_DO_ROUGH_SEARCH: i32 = 8;
	pub const CASCADE_FIND_BIGGEST_OBJECT: i32 = 4;
	pub const CASCADE_SCALE_IMAGE: i32 = 2;
	/// Tag and corners detection based on the AprilTag 2 approach [wang2016iros](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_wang2016iros)
	pub const CORNER_REFINE_APRILTAG: i32 = 3;
	/// ArUco approach and refine the corners locations using the contour-points line fitting
	pub const CORNER_REFINE_CONTOUR: i32 = 2;
	/// Tag and corners detection based on the ArUco approach
	pub const CORNER_REFINE_NONE: i32 = 0;
	/// ArUco approach and refine the corners locations using corner subpixel accuracy
	pub const CORNER_REFINE_SUBPIX: i32 = 1;
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
	pub const DetectionBasedTracker_DETECTED: i32 = 1;
	pub const DetectionBasedTracker_DETECTED_NOT_SHOWN_YET: i32 = 0;
	pub const DetectionBasedTracker_DETECTED_TEMPORARY_LOST: i32 = 2;
	pub const DetectionBasedTracker_WRONG_OBJECT: i32 = 3;
	pub const FaceRecognizerSF_FR_COSINE: i32 = 0;
	pub const FaceRecognizerSF_FR_NORM_L2: i32 = 1;
	/// Default nlevels value.
	pub const HOGDescriptor_DEFAULT_NLEVELS: i32 = 64;
	pub const HOGDescriptor_DESCR_FORMAT_COL_BY_COL: i32 = 0;
	pub const HOGDescriptor_DESCR_FORMAT_ROW_BY_ROW: i32 = 1;
	/// Default histogramNormType
	pub const HOGDescriptor_L2Hys: i32 = 0;
	pub const QRCodeEncoder_CORRECT_LEVEL_H: i32 = 3;
	pub const QRCodeEncoder_CORRECT_LEVEL_L: i32 = 0;
	pub const QRCodeEncoder_CORRECT_LEVEL_M: i32 = 1;
	pub const QRCodeEncoder_CORRECT_LEVEL_Q: i32 = 2;
	pub const QRCodeEncoder_ECI_UTF8: i32 = 26;
	pub const QRCodeEncoder_MODE_ALPHANUMERIC: i32 = 2;
	pub const QRCodeEncoder_MODE_AUTO: i32 = -1;
	pub const QRCodeEncoder_MODE_BYTE: i32 = 4;
	pub const QRCodeEncoder_MODE_ECI: i32 = 7;
	pub const QRCodeEncoder_MODE_KANJI: i32 = 8;
	pub const QRCodeEncoder_MODE_NUMERIC: i32 = 1;
	pub const QRCodeEncoder_MODE_STRUCTURED_APPEND: i32 = 3;
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum CornerRefineMethod {
		/// Tag and corners detection based on the ArUco approach
		CORNER_REFINE_NONE = 0,
		/// ArUco approach and refine the corners locations using corner subpixel accuracy
		CORNER_REFINE_SUBPIX = 1,
		/// ArUco approach and refine the corners locations using the contour-points line fitting
		CORNER_REFINE_CONTOUR = 2,
		/// Tag and corners detection based on the AprilTag 2 approach [wang2016iros](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_wang2016iros)
		CORNER_REFINE_APRILTAG = 3,
	}
	
	opencv_type_enum! { crate::objdetect::CornerRefineMethod }
	
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum DetectionBasedTracker_ObjectStatus {
		DETECTED_NOT_SHOWN_YET = 0,
		DETECTED = 1,
		DETECTED_TEMPORARY_LOST = 2,
		WRONG_OBJECT = 3,
	}
	
	opencv_type_enum! { crate::objdetect::DetectionBasedTracker_ObjectStatus }
	
	/// Definition of distance used for calculating the distance between two face features
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum FaceRecognizerSF_DisType {
		FR_COSINE = 0,
		FR_NORM_L2 = 1,
	}
	
	opencv_type_enum! { crate::objdetect::FaceRecognizerSF_DisType }
	
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum HOGDescriptor_DescriptorStorageFormat {
		DESCR_FORMAT_COL_BY_COL = 0,
		DESCR_FORMAT_ROW_BY_ROW = 1,
	}
	
	opencv_type_enum! { crate::objdetect::HOGDescriptor_DescriptorStorageFormat }
	
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum HOGDescriptor_HistogramNormType {
		/// Default histogramNormType
		L2Hys = 0,
	}
	
	opencv_type_enum! { crate::objdetect::HOGDescriptor_HistogramNormType }
	
	/// Predefined markers dictionaries/sets
	/// 
	/// Each dictionary indicates the number of bits and the number of markers contained
	/// - DICT_ARUCO_ORIGINAL: standard ArUco Library Markers. 1024 markers, 5x5 bits, 0 minimum
	///                        distance
	#[repr(C)]
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
	
	opencv_type_enum! { crate::objdetect::PredefinedDictionaryType }
	
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum QRCodeEncoder_CorrectionLevel {
		CORRECT_LEVEL_L = 0,
		CORRECT_LEVEL_M = 1,
		CORRECT_LEVEL_Q = 2,
		CORRECT_LEVEL_H = 3,
	}
	
	opencv_type_enum! { crate::objdetect::QRCodeEncoder_CorrectionLevel }
	
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum QRCodeEncoder_ECIEncodings {
		ECI_UTF8 = 26,
	}
	
	opencv_type_enum! { crate::objdetect::QRCodeEncoder_ECIEncodings }
	
	#[repr(C)]
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
	
	opencv_type_enum! { crate::objdetect::QRCodeEncoder_EncodeMode }
	
	pub type DetectionBasedTracker_Object = core::Tuple<(core::Rect, i32)>;
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
	pub fn draw_detected_corners_charuco_def(image: &mut impl core::ToInputOutputArray, charuco_corners: &impl core::ToInputArray) -> Result<()> {
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
	#[inline]
	pub fn draw_detected_corners_charuco(image: &mut impl core::ToInputOutputArray, charuco_corners: &impl core::ToInputArray, charuco_ids: &impl core::ToInputArray, corner_color: core::Scalar) -> Result<()> {
		input_output_array_arg!(image);
		input_array_arg!(charuco_corners);
		input_array_arg!(charuco_ids);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_drawDetectedCornersCharuco_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(image.as_raw__InputOutputArray(), charuco_corners.as_raw__InputArray(), charuco_ids.as_raw__InputArray(), corner_color.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
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
	#[inline]
	pub fn draw_detected_diamonds_def(image: &mut impl core::ToInputOutputArray, diamond_corners: &impl core::ToInputArray) -> Result<()> {
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
	#[inline]
	pub fn draw_detected_diamonds(image: &mut impl core::ToInputOutputArray, diamond_corners: &impl core::ToInputArray, diamond_ids: &impl core::ToInputArray, border_color: core::Scalar) -> Result<()> {
		input_output_array_arg!(image);
		input_array_arg!(diamond_corners);
		input_array_arg!(diamond_ids);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_drawDetectedDiamonds_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(image.as_raw__InputOutputArray(), diamond_corners.as_raw__InputArray(), diamond_ids.as_raw__InputArray(), border_color.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
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
	pub fn draw_detected_markers_def(image: &mut impl core::ToInputOutputArray, corners: &impl core::ToInputArray) -> Result<()> {
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
	pub fn draw_detected_markers(image: &mut impl core::ToInputOutputArray, corners: &impl core::ToInputArray, ids: &impl core::ToInputArray, border_color: core::Scalar) -> Result<()> {
		input_output_array_arg!(image);
		input_array_arg!(corners);
		input_array_arg!(ids);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_drawDetectedMarkers_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_Scalar(image.as_raw__InputOutputArray(), corners.as_raw__InputArray(), ids.as_raw__InputArray(), border_color.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
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
		return_receive!(unsafe ocvrs_return => ret);
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
	pub fn extend_dictionary(n_markers: i32, marker_size: i32, base_dictionary: &crate::objdetect::Dictionary, random_seed: i32) -> Result<crate::objdetect::Dictionary> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_extendDictionary_int_int_const_DictionaryR_int(n_markers, marker_size, base_dictionary.as_raw_Dictionary(), random_seed, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
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
	pub fn generate_image_marker_def(dictionary: &crate::objdetect::Dictionary, id: i32, side_pixels: i32, img: &mut impl core::ToOutputArray) -> Result<()> {
		output_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_generateImageMarker_const_DictionaryR_int_int_const__OutputArrayR(dictionary.as_raw_Dictionary(), id, side_pixels, img.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
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
	pub fn generate_image_marker(dictionary: &crate::objdetect::Dictionary, id: i32, side_pixels: i32, img: &mut impl core::ToOutputArray, border_bits: i32) -> Result<()> {
		output_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_generateImageMarker_const_DictionaryR_int_int_const__OutputArrayR_int(dictionary.as_raw_Dictionary(), id, side_pixels, img.as_raw__OutputArray(), border_bits, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns one of the predefined dictionaries defined in PredefinedDictionaryType
	#[inline]
	pub fn get_predefined_dictionary(name: crate::objdetect::PredefinedDictionaryType) -> Result<crate::objdetect::Dictionary> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_getPredefinedDictionary_PredefinedDictionaryType(name, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::objdetect::Dictionary::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Returns one of the predefined dictionaries referenced by DICT_*.
	#[inline]
	pub fn get_predefined_dictionary_i32(dict: i32) -> Result<crate::objdetect::Dictionary> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_getPredefinedDictionary_int(dict, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::objdetect::Dictionary::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn create_face_detection_mask_generator() -> Result<core::Ptr<crate::objdetect::BaseCascadeClassifier_MaskGenerator>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_createFaceDetectionMaskGenerator(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::objdetect::BaseCascadeClassifier_MaskGenerator>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// @overload
	/// 
	/// ## Note
	/// This alternative version of [group_rectangles_meanshift] function uses the following default values for its arguments:
	/// * detect_threshold: 0.0
	/// * win_det_size: Size(64,128)
	#[inline]
	pub fn group_rectangles_meanshift_def(rect_list: &mut core::Vector<core::Rect>, found_weights: &mut core::Vector<f64>, found_scales: &mut core::Vector<f64>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_groupRectangles_meanshift_vectorLRectGR_vectorLdoubleGR_vectorLdoubleGR(rect_list.as_raw_mut_VectorOfRect(), found_weights.as_raw_mut_VectorOff64(), found_scales.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
	/// 
	/// ## C++ default parameters
	/// * detect_threshold: 0.0
	/// * win_det_size: Size(64,128)
	#[inline]
	pub fn group_rectangles_meanshift(rect_list: &mut core::Vector<core::Rect>, found_weights: &mut core::Vector<f64>, found_scales: &mut core::Vector<f64>, detect_threshold: f64, win_det_size: core::Size) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_groupRectangles_meanshift_vectorLRectGR_vectorLdoubleGR_vectorLdoubleGR_double_Size(rect_list.as_raw_mut_VectorOfRect(), found_weights.as_raw_mut_VectorOff64(), found_scales.as_raw_mut_VectorOff64(), detect_threshold, win_det_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Groups the object candidate rectangles.
	/// 
	/// ## Parameters
	/// * rectList: Input/output vector of rectangles. Output vector includes retained and grouped
	/// rectangles. (The Python list is not modified in place.)
	/// * groupThreshold: Minimum possible number of rectangles minus 1. The threshold is used in a
	/// group of rectangles to retain it.
	/// * eps: Relative difference between sides of the rectangles to merge them into a group.
	/// 
	/// The function is a wrapper for the generic function partition . It clusters all the input rectangles
	/// using the rectangle equivalence criteria that combines rectangles with similar sizes and similar
	/// locations. The similarity is defined by eps. When eps=0 , no clustering is done at all. If
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Beps%7D%5Crightarrow%20%2B%5Cinf) , all the rectangles are put in one cluster. Then, the small
	/// clusters containing less than or equal to groupThreshold rectangles are rejected. In each other
	/// cluster, the average rectangle is computed and put into the output rectangle list.
	/// 
	/// ## Note
	/// This alternative version of [group_rectangles] function uses the following default values for its arguments:
	/// * eps: 0.2
	#[inline]
	pub fn group_rectangles_def(rect_list: &mut core::Vector<core::Rect>, group_threshold: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_groupRectangles_vectorLRectGR_int(rect_list.as_raw_mut_VectorOfRect(), group_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Groups the object candidate rectangles.
	/// 
	/// ## Parameters
	/// * rectList: Input/output vector of rectangles. Output vector includes retained and grouped
	/// rectangles. (The Python list is not modified in place.)
	/// * groupThreshold: Minimum possible number of rectangles minus 1. The threshold is used in a
	/// group of rectangles to retain it.
	/// * eps: Relative difference between sides of the rectangles to merge them into a group.
	/// 
	/// The function is a wrapper for the generic function partition . It clusters all the input rectangles
	/// using the rectangle equivalence criteria that combines rectangles with similar sizes and similar
	/// locations. The similarity is defined by eps. When eps=0 , no clustering is done at all. If
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Beps%7D%5Crightarrow%20%2B%5Cinf) , all the rectangles are put in one cluster. Then, the small
	/// clusters containing less than or equal to groupThreshold rectangles are rejected. In each other
	/// cluster, the average rectangle is computed and put into the output rectangle list.
	/// 
	/// ## C++ default parameters
	/// * eps: 0.2
	#[inline]
	pub fn group_rectangles(rect_list: &mut core::Vector<core::Rect>, group_threshold: i32, eps: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_groupRectangles_vectorLRectGR_int_double(rect_list.as_raw_mut_VectorOfRect(), group_threshold, eps, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Groups the object candidate rectangles.
	/// 
	/// ## Parameters
	/// * rectList: Input/output vector of rectangles. Output vector includes retained and grouped
	/// rectangles. (The Python list is not modified in place.)
	/// * groupThreshold: Minimum possible number of rectangles minus 1. The threshold is used in a
	/// group of rectangles to retain it.
	/// * eps: Relative difference between sides of the rectangles to merge them into a group.
	/// 
	/// The function is a wrapper for the generic function partition . It clusters all the input rectangles
	/// using the rectangle equivalence criteria that combines rectangles with similar sizes and similar
	/// locations. The similarity is defined by eps. When eps=0 , no clustering is done at all. If
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Beps%7D%5Crightarrow%20%2B%5Cinf) , all the rectangles are put in one cluster. Then, the small
	/// clusters containing less than or equal to groupThreshold rectangles are rejected. In each other
	/// cluster, the average rectangle is computed and put into the output rectangle list.
	/// 
	/// ## Overloaded parameters
	#[inline]
	pub fn group_rectangles_levelweights(rect_list: &mut core::Vector<core::Rect>, group_threshold: i32, eps: f64, weights: &mut core::Vector<i32>, level_weights: &mut core::Vector<f64>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_groupRectangles_vectorLRectGR_int_double_vectorLintGX_vectorLdoubleGX(rect_list.as_raw_mut_VectorOfRect(), group_threshold, eps, weights.as_raw_mut_VectorOfi32(), level_weights.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// @overload
	/// 
	/// ## Note
	/// This alternative version of [group_rectangles_weights] function uses the following default values for its arguments:
	/// * eps: 0.2
	#[inline]
	pub fn group_rectangles_weights_def(rect_list: &mut core::Vector<core::Rect>, weights: &mut core::Vector<i32>, group_threshold: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_groupRectangles_vectorLRectGR_vectorLintGR_int(rect_list.as_raw_mut_VectorOfRect(), weights.as_raw_mut_VectorOfi32(), group_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Groups the object candidate rectangles.
	/// 
	/// ## Parameters
	/// * rectList: Input/output vector of rectangles. Output vector includes retained and grouped
	/// rectangles. (The Python list is not modified in place.)
	/// * groupThreshold: Minimum possible number of rectangles minus 1. The threshold is used in a
	/// group of rectangles to retain it.
	/// * eps: Relative difference between sides of the rectangles to merge them into a group.
	/// 
	/// The function is a wrapper for the generic function partition . It clusters all the input rectangles
	/// using the rectangle equivalence criteria that combines rectangles with similar sizes and similar
	/// locations. The similarity is defined by eps. When eps=0 , no clustering is done at all. If
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Beps%7D%5Crightarrow%20%2B%5Cinf) , all the rectangles are put in one cluster. Then, the small
	/// clusters containing less than or equal to groupThreshold rectangles are rejected. In each other
	/// cluster, the average rectangle is computed and put into the output rectangle list.
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## C++ default parameters
	/// * eps: 0.2
	#[inline]
	pub fn group_rectangles_weights(rect_list: &mut core::Vector<core::Rect>, weights: &mut core::Vector<i32>, group_threshold: i32, eps: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_groupRectangles_vectorLRectGR_vectorLintGR_int_double(rect_list.as_raw_mut_VectorOfRect(), weights.as_raw_mut_VectorOfi32(), group_threshold, eps, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// @overload
	/// 
	/// ## Note
	/// This alternative version of [group_rectangles_levels] function uses the following default values for its arguments:
	/// * eps: 0.2
	#[inline]
	pub fn group_rectangles_levels_def(rect_list: &mut core::Vector<core::Rect>, reject_levels: &mut core::Vector<i32>, level_weights: &mut core::Vector<f64>, group_threshold: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_groupRectangles_vectorLRectGR_vectorLintGR_vectorLdoubleGR_int(rect_list.as_raw_mut_VectorOfRect(), reject_levels.as_raw_mut_VectorOfi32(), level_weights.as_raw_mut_VectorOff64(), group_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Groups the object candidate rectangles.
	/// 
	/// ## Parameters
	/// * rectList: Input/output vector of rectangles. Output vector includes retained and grouped
	/// rectangles. (The Python list is not modified in place.)
	/// * groupThreshold: Minimum possible number of rectangles minus 1. The threshold is used in a
	/// group of rectangles to retain it.
	/// * eps: Relative difference between sides of the rectangles to merge them into a group.
	/// 
	/// The function is a wrapper for the generic function partition . It clusters all the input rectangles
	/// using the rectangle equivalence criteria that combines rectangles with similar sizes and similar
	/// locations. The similarity is defined by eps. When eps=0 , no clustering is done at all. If
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Beps%7D%5Crightarrow%20%2B%5Cinf) , all the rectangles are put in one cluster. Then, the small
	/// clusters containing less than or equal to groupThreshold rectangles are rejected. In each other
	/// cluster, the average rectangle is computed and put into the output rectangle list.
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## C++ default parameters
	/// * eps: 0.2
	#[inline]
	pub fn group_rectangles_levels(rect_list: &mut core::Vector<core::Rect>, reject_levels: &mut core::Vector<i32>, level_weights: &mut core::Vector<f64>, group_threshold: i32, eps: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_groupRectangles_vectorLRectGR_vectorLintGR_vectorLdoubleGR_int_double(rect_list.as_raw_mut_VectorOfRect(), reject_levels.as_raw_mut_VectorOfi32(), level_weights.as_raw_mut_VectorOff64(), group_threshold, eps, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Constant methods for [crate::objdetect::BaseCascadeClassifier]
	pub trait BaseCascadeClassifierTraitConst: core::AlgorithmTraitConst {
		fn as_raw_BaseCascadeClassifier(&self) -> *const c_void;
	
		#[inline]
		fn empty(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BaseCascadeClassifier_empty_const(self.as_raw_BaseCascadeClassifier(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn is_old_format_cascade(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BaseCascadeClassifier_isOldFormatCascade_const(self.as_raw_BaseCascadeClassifier(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_original_window_size(&self) -> Result<core::Size> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BaseCascadeClassifier_getOriginalWindowSize_const(self.as_raw_BaseCascadeClassifier(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_feature_type(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BaseCascadeClassifier_getFeatureType_const(self.as_raw_BaseCascadeClassifier(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::objdetect::BaseCascadeClassifier]
	pub trait BaseCascadeClassifierTrait: core::AlgorithmTrait + crate::objdetect::BaseCascadeClassifierTraitConst {
		fn as_raw_mut_BaseCascadeClassifier(&mut self) -> *mut c_void;
	
		#[inline]
		fn load(&mut self, filename: &str) -> Result<bool> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BaseCascadeClassifier_load_const_StringR(self.as_raw_mut_BaseCascadeClassifier(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn detect_multi_scale(&mut self, image: &impl core::ToInputArray, objects: &mut core::Vector<core::Rect>, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size) -> Result<()> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_double_int_int_Size_Size(self.as_raw_mut_BaseCascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_mut_VectorOfRect(), scale_factor, min_neighbors, flags, min_size.opencv_as_extern(), max_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn detect_multi_scale_num(&mut self, image: &impl core::ToInputArray, objects: &mut core::Vector<core::Rect>, num_detections: &mut core::Vector<i32>, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size) -> Result<()> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_double_int_int_Size_Size(self.as_raw_mut_BaseCascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_mut_VectorOfRect(), num_detections.as_raw_mut_VectorOfi32(), scale_factor, min_neighbors, flags, min_size.opencv_as_extern(), max_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn detect_multi_scale_levels(&mut self, image: &impl core::ToInputArray, objects: &mut core::Vector<core::Rect>, reject_levels: &mut core::Vector<i32>, level_weights: &mut core::Vector<f64>, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size, output_reject_levels: bool) -> Result<()> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_vectorLdoubleGR_double_int_int_Size_Size_bool(self.as_raw_mut_BaseCascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_mut_VectorOfRect(), reject_levels.as_raw_mut_VectorOfi32(), level_weights.as_raw_mut_VectorOff64(), scale_factor, min_neighbors, flags, min_size.opencv_as_extern(), max_size.opencv_as_extern(), output_reject_levels, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_old_cascade(&mut self) -> Result<*mut c_void> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BaseCascadeClassifier_getOldCascade(self.as_raw_mut_BaseCascadeClassifier(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_mask_generator(&mut self, mask_generator: &core::Ptr<crate::objdetect::BaseCascadeClassifier_MaskGenerator>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BaseCascadeClassifier_setMaskGenerator_const_PtrLMaskGeneratorGR(self.as_raw_mut_BaseCascadeClassifier(), mask_generator.as_raw_PtrOfBaseCascadeClassifier_MaskGenerator(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_mask_generator(&mut self) -> Result<core::Ptr<crate::objdetect::BaseCascadeClassifier_MaskGenerator>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BaseCascadeClassifier_getMaskGenerator(self.as_raw_mut_BaseCascadeClassifier(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::objdetect::BaseCascadeClassifier_MaskGenerator>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	pub struct BaseCascadeClassifier {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { BaseCascadeClassifier }
	
	impl Drop for BaseCascadeClassifier {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_BaseCascadeClassifier_delete(self.as_raw_mut_BaseCascadeClassifier()) };
		}
	}
	
	unsafe impl Send for BaseCascadeClassifier {}
	
	impl core::AlgorithmTraitConst for BaseCascadeClassifier {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for BaseCascadeClassifier {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::objdetect::BaseCascadeClassifierTraitConst for BaseCascadeClassifier {
		#[inline] fn as_raw_BaseCascadeClassifier(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::BaseCascadeClassifierTrait for BaseCascadeClassifier {
		#[inline] fn as_raw_mut_BaseCascadeClassifier(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl BaseCascadeClassifier {
	}
	
	boxed_cast_base! { BaseCascadeClassifier, core::Algorithm, cv_BaseCascadeClassifier_to_Algorithm }
	
	impl std::fmt::Debug for BaseCascadeClassifier {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("BaseCascadeClassifier")
				.finish()
		}
	}
	
	/// Constant methods for [crate::objdetect::BaseCascadeClassifier_MaskGenerator]
	pub trait BaseCascadeClassifier_MaskGeneratorTraitConst {
		fn as_raw_BaseCascadeClassifier_MaskGenerator(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::objdetect::BaseCascadeClassifier_MaskGenerator]
	pub trait BaseCascadeClassifier_MaskGeneratorTrait: crate::objdetect::BaseCascadeClassifier_MaskGeneratorTraitConst {
		fn as_raw_mut_BaseCascadeClassifier_MaskGenerator(&mut self) -> *mut c_void;
	
		#[inline]
		fn generate_mask(&mut self, src: &core::Mat) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BaseCascadeClassifier_MaskGenerator_generateMask_const_MatR(self.as_raw_mut_BaseCascadeClassifier_MaskGenerator(), src.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn initialize_mask(&mut self, unnamed: &core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BaseCascadeClassifier_MaskGenerator_initializeMask_const_MatR(self.as_raw_mut_BaseCascadeClassifier_MaskGenerator(), unnamed.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct BaseCascadeClassifier_MaskGenerator {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { BaseCascadeClassifier_MaskGenerator }
	
	impl Drop for BaseCascadeClassifier_MaskGenerator {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_BaseCascadeClassifier_MaskGenerator_delete(self.as_raw_mut_BaseCascadeClassifier_MaskGenerator()) };
		}
	}
	
	unsafe impl Send for BaseCascadeClassifier_MaskGenerator {}
	
	impl crate::objdetect::BaseCascadeClassifier_MaskGeneratorTraitConst for BaseCascadeClassifier_MaskGenerator {
		#[inline] fn as_raw_BaseCascadeClassifier_MaskGenerator(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::BaseCascadeClassifier_MaskGeneratorTrait for BaseCascadeClassifier_MaskGenerator {
		#[inline] fn as_raw_mut_BaseCascadeClassifier_MaskGenerator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl BaseCascadeClassifier_MaskGenerator {
	}
	
	impl std::fmt::Debug for BaseCascadeClassifier_MaskGenerator {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("BaseCascadeClassifier_MaskGenerator")
				.finish()
		}
	}
	
	/// Constant methods for [crate::objdetect::CascadeClassifier]
	pub trait CascadeClassifierTraitConst {
		fn as_raw_CascadeClassifier(&self) -> *const c_void;
	
		/// Checks whether the classifier has been loaded.
		#[inline]
		fn empty(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_CascadeClassifier_empty_const(self.as_raw_CascadeClassifier(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn is_old_format_cascade(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_CascadeClassifier_isOldFormatCascade_const(self.as_raw_CascadeClassifier(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_original_window_size(&self) -> Result<core::Size> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_CascadeClassifier_getOriginalWindowSize_const(self.as_raw_CascadeClassifier(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_feature_type(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_CascadeClassifier_getFeatureType_const(self.as_raw_CascadeClassifier(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::objdetect::CascadeClassifier]
	pub trait CascadeClassifierTrait: crate::objdetect::CascadeClassifierTraitConst {
		fn as_raw_mut_CascadeClassifier(&mut self) -> *mut c_void;
	
		#[inline]
		fn cc(&mut self) -> core::Ptr<crate::objdetect::BaseCascadeClassifier> {
			let ret = unsafe { sys::cv_CascadeClassifier_propCc(self.as_raw_mut_CascadeClassifier()) };
			let ret = unsafe { core::Ptr::<crate::objdetect::BaseCascadeClassifier>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn set_cc(&mut self, mut val: core::Ptr<crate::objdetect::BaseCascadeClassifier>) {
			let ret = unsafe { sys::cv_CascadeClassifier_propCc_PtrLBaseCascadeClassifierG(self.as_raw_mut_CascadeClassifier(), val.as_raw_mut_PtrOfBaseCascadeClassifier()) };
			ret
		}
		
		/// Loads a classifier from a file.
		/// 
		/// ## Parameters
		/// * filename: Name of the file from which the classifier is loaded. The file may contain an old
		/// HAAR classifier trained by the haartraining application or a new cascade classifier trained by the
		/// traincascade application.
		#[inline]
		fn load(&mut self, filename: &str) -> Result<bool> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_CascadeClassifier_load_const_StringR(self.as_raw_mut_CascadeClassifier(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Reads a classifier from a FileStorage node.
		/// 
		/// 
		/// Note: The file may contain a new cascade classifier (trained by the traincascade application) only.
		#[inline]
		fn read(&mut self, node: &core::FileNode) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_CascadeClassifier_read_const_FileNodeR(self.as_raw_mut_CascadeClassifier(), node.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Detects objects of different sizes in the input image. The detected objects are returned as a list
		/// of rectangles.
		/// 
		/// ## Parameters
		/// * image: Matrix of the type CV_8U containing an image where objects are detected.
		/// * objects: Vector of rectangles where each rectangle contains the detected object, the
		/// rectangles may be partially outside the original image.
		/// * scaleFactor: Parameter specifying how much the image size is reduced at each image scale.
		/// * minNeighbors: Parameter specifying how many neighbors each candidate rectangle should have
		/// to retain it.
		/// * flags: Parameter with the same meaning for an old cascade as in the function
		/// cvHaarDetectObjects. It is not used for a new cascade.
		/// * minSize: Minimum possible object size. Objects smaller than that are ignored.
		/// * maxSize: Maximum possible object size. Objects larger than that are ignored. If `maxSize == minSize` model is evaluated on single scale.
		/// 
		/// ## C++ default parameters
		/// * scale_factor: 1.1
		/// * min_neighbors: 3
		/// * flags: 0
		/// * min_size: Size()
		/// * max_size: Size()
		#[inline]
		fn detect_multi_scale(&mut self, image: &impl core::ToInputArray, objects: &mut core::Vector<core::Rect>, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size) -> Result<()> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_double_int_int_Size_Size(self.as_raw_mut_CascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_mut_VectorOfRect(), scale_factor, min_neighbors, flags, min_size.opencv_as_extern(), max_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Detects objects of different sizes in the input image. The detected objects are returned as a list
		/// of rectangles.
		/// 
		/// ## Parameters
		/// * image: Matrix of the type CV_8U containing an image where objects are detected.
		/// * objects: Vector of rectangles where each rectangle contains the detected object, the
		/// rectangles may be partially outside the original image.
		/// * scaleFactor: Parameter specifying how much the image size is reduced at each image scale.
		/// * minNeighbors: Parameter specifying how many neighbors each candidate rectangle should have
		/// to retain it.
		/// * flags: Parameter with the same meaning for an old cascade as in the function
		/// cvHaarDetectObjects. It is not used for a new cascade.
		/// * minSize: Minimum possible object size. Objects smaller than that are ignored.
		/// * maxSize: Maximum possible object size. Objects larger than that are ignored. If `maxSize == minSize` model is evaluated on single scale.
		/// 
		/// ## Note
		/// This alternative version of [detect_multi_scale] function uses the following default values for its arguments:
		/// * scale_factor: 1.1
		/// * min_neighbors: 3
		/// * flags: 0
		/// * min_size: Size()
		/// * max_size: Size()
		#[inline]
		fn detect_multi_scale_def(&mut self, image: &impl core::ToInputArray, objects: &mut core::Vector<core::Rect>) -> Result<()> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR(self.as_raw_mut_CascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Detects objects of different sizes in the input image. The detected objects are returned as a list
		/// of rectangles.
		/// 
		/// ## Parameters
		/// * image: Matrix of the type CV_8U containing an image where objects are detected.
		/// * objects: Vector of rectangles where each rectangle contains the detected object, the
		/// rectangles may be partially outside the original image.
		/// * scaleFactor: Parameter specifying how much the image size is reduced at each image scale.
		/// * minNeighbors: Parameter specifying how many neighbors each candidate rectangle should have
		/// to retain it.
		/// * flags: Parameter with the same meaning for an old cascade as in the function
		/// cvHaarDetectObjects. It is not used for a new cascade.
		/// * minSize: Minimum possible object size. Objects smaller than that are ignored.
		/// * maxSize: Maximum possible object size. Objects larger than that are ignored. If `maxSize == minSize` model is evaluated on single scale.
		/// 
		/// ## Overloaded parameters
		/// 
		/// * image: Matrix of the type CV_8U containing an image where objects are detected.
		/// * objects: Vector of rectangles where each rectangle contains the detected object, the
		///    rectangles may be partially outside the original image.
		/// * numDetections: Vector of detection numbers for the corresponding objects. An object's number
		///    of detections is the number of neighboring positively classified rectangles that were joined
		///    together to form the object.
		/// * scaleFactor: Parameter specifying how much the image size is reduced at each image scale.
		/// * minNeighbors: Parameter specifying how many neighbors each candidate rectangle should have
		///    to retain it.
		/// * flags: Parameter with the same meaning for an old cascade as in the function
		///    cvHaarDetectObjects. It is not used for a new cascade.
		/// * minSize: Minimum possible object size. Objects smaller than that are ignored.
		/// * maxSize: Maximum possible object size. Objects larger than that are ignored. If `maxSize == minSize` model is evaluated on single scale.
		/// 
		/// ## C++ default parameters
		/// * scale_factor: 1.1
		/// * min_neighbors: 3
		/// * flags: 0
		/// * min_size: Size()
		/// * max_size: Size()
		#[inline]
		fn detect_multi_scale2(&mut self, image: &impl core::ToInputArray, objects: &mut core::Vector<core::Rect>, num_detections: &mut core::Vector<i32>, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size) -> Result<()> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_double_int_int_Size_Size(self.as_raw_mut_CascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_mut_VectorOfRect(), num_detections.as_raw_mut_VectorOfi32(), scale_factor, min_neighbors, flags, min_size.opencv_as_extern(), max_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// @overload
		/// ## Parameters
		/// * image: Matrix of the type CV_8U containing an image where objects are detected.
		/// * objects: Vector of rectangles where each rectangle contains the detected object, the
		/// rectangles may be partially outside the original image.
		/// * numDetections: Vector of detection numbers for the corresponding objects. An object's number
		/// of detections is the number of neighboring positively classified rectangles that were joined
		/// together to form the object.
		/// * scaleFactor: Parameter specifying how much the image size is reduced at each image scale.
		/// * minNeighbors: Parameter specifying how many neighbors each candidate rectangle should have
		/// to retain it.
		/// * flags: Parameter with the same meaning for an old cascade as in the function
		/// cvHaarDetectObjects. It is not used for a new cascade.
		/// * minSize: Minimum possible object size. Objects smaller than that are ignored.
		/// * maxSize: Maximum possible object size. Objects larger than that are ignored. If `maxSize == minSize` model is evaluated on single scale.
		/// 
		/// ## Note
		/// This alternative version of [detect_multi_scale2] function uses the following default values for its arguments:
		/// * scale_factor: 1.1
		/// * min_neighbors: 3
		/// * flags: 0
		/// * min_size: Size()
		/// * max_size: Size()
		#[inline]
		fn detect_multi_scale2_def(&mut self, image: &impl core::ToInputArray, objects: &mut core::Vector<core::Rect>, num_detections: &mut core::Vector<i32>) -> Result<()> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR(self.as_raw_mut_CascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_mut_VectorOfRect(), num_detections.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Detects objects of different sizes in the input image. The detected objects are returned as a list
		/// of rectangles.
		/// 
		/// ## Parameters
		/// * image: Matrix of the type CV_8U containing an image where objects are detected.
		/// * objects: Vector of rectangles where each rectangle contains the detected object, the
		/// rectangles may be partially outside the original image.
		/// * scaleFactor: Parameter specifying how much the image size is reduced at each image scale.
		/// * minNeighbors: Parameter specifying how many neighbors each candidate rectangle should have
		/// to retain it.
		/// * flags: Parameter with the same meaning for an old cascade as in the function
		/// cvHaarDetectObjects. It is not used for a new cascade.
		/// * minSize: Minimum possible object size. Objects smaller than that are ignored.
		/// * maxSize: Maximum possible object size. Objects larger than that are ignored. If `maxSize == minSize` model is evaluated on single scale.
		/// 
		/// ## Overloaded parameters
		/// 
		///    This function allows you to retrieve the final stage decision certainty of classification.
		///    For this, one needs to set `outputRejectLevels` on true and provide the `rejectLevels` and `levelWeights` parameter.
		///    For each resulting detection, `levelWeights` will then contain the certainty of classification at the final stage.
		///    This value can then be used to separate strong from weaker classifications.
		/// 
		///    A code sample on how to use it efficiently can be found below:
		///    ```C++
		///    Mat img;
		///    vector<double> weights;
		///    vector<int> levels;
		///    vector<Rect> detections;
		///    CascadeClassifier model("/path/to/your/model.xml");
		///    model.detectMultiScale(img, detections, levels, weights, 1.1, 3, 0, Size(), Size(), true);
		///    cerr << "Detection " << detections[0] << " with weight " << weights[0] << endl;
		///    ```
		/// 
		/// 
		/// ## C++ default parameters
		/// * scale_factor: 1.1
		/// * min_neighbors: 3
		/// * flags: 0
		/// * min_size: Size()
		/// * max_size: Size()
		/// * output_reject_levels: false
		#[inline]
		fn detect_multi_scale3(&mut self, image: &impl core::ToInputArray, objects: &mut core::Vector<core::Rect>, reject_levels: &mut core::Vector<i32>, level_weights: &mut core::Vector<f64>, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size, output_reject_levels: bool) -> Result<()> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_vectorLdoubleGR_double_int_int_Size_Size_bool(self.as_raw_mut_CascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_mut_VectorOfRect(), reject_levels.as_raw_mut_VectorOfi32(), level_weights.as_raw_mut_VectorOff64(), scale_factor, min_neighbors, flags, min_size.opencv_as_extern(), max_size.opencv_as_extern(), output_reject_levels, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// @overload
		/// This function allows you to retrieve the final stage decision certainty of classification.
		/// For this, one needs to set `outputRejectLevels` on true and provide the `rejectLevels` and `levelWeights` parameter.
		/// For each resulting detection, `levelWeights` will then contain the certainty of classification at the final stage.
		/// This value can then be used to separate strong from weaker classifications.
		/// 
		/// A code sample on how to use it efficiently can be found below:
		/// ```C++
		/// Mat img;
		/// vector<double> weights;
		/// vector<int> levels;
		/// vector<Rect> detections;
		/// CascadeClassifier model("/path/to/your/model.xml");
		/// model.detectMultiScale(img, detections, levels, weights, 1.1, 3, 0, Size(), Size(), true);
		/// cerr << "Detection " << detections[0] << " with weight " << weights[0] << endl;
		/// ```
		/// 
		/// 
		/// ## Note
		/// This alternative version of [detect_multi_scale3] function uses the following default values for its arguments:
		/// * scale_factor: 1.1
		/// * min_neighbors: 3
		/// * flags: 0
		/// * min_size: Size()
		/// * max_size: Size()
		/// * output_reject_levels: false
		#[inline]
		fn detect_multi_scale3_def(&mut self, image: &impl core::ToInputArray, objects: &mut core::Vector<core::Rect>, reject_levels: &mut core::Vector<i32>, level_weights: &mut core::Vector<f64>) -> Result<()> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_vectorLdoubleGR(self.as_raw_mut_CascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_mut_VectorOfRect(), reject_levels.as_raw_mut_VectorOfi32(), level_weights.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_old_cascade(&mut self) -> Result<*mut c_void> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_CascadeClassifier_getOldCascade(self.as_raw_mut_CascadeClassifier(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_mask_generator(&mut self, mask_generator: &core::Ptr<crate::objdetect::BaseCascadeClassifier_MaskGenerator>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_CascadeClassifier_setMaskGenerator_const_PtrLMaskGeneratorGR(self.as_raw_mut_CascadeClassifier(), mask_generator.as_raw_PtrOfBaseCascadeClassifier_MaskGenerator(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_mask_generator(&mut self) -> Result<core::Ptr<crate::objdetect::BaseCascadeClassifier_MaskGenerator>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_CascadeClassifier_getMaskGenerator(self.as_raw_mut_CascadeClassifier(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::objdetect::BaseCascadeClassifier_MaskGenerator>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// @example samples/cpp/facedetect.cpp
	/// This program demonstrates usage of the Cascade classifier class
	/// \image html Cascade_Classifier_Tutorial_Result_Haar.jpg "Sample screenshot" width=321 height=254
	/// 
	/// Cascade classifier class for object detection.
	pub struct CascadeClassifier {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CascadeClassifier }
	
	impl Drop for CascadeClassifier {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_CascadeClassifier_delete(self.as_raw_mut_CascadeClassifier()) };
		}
	}
	
	unsafe impl Send for CascadeClassifier {}
	
	impl crate::objdetect::CascadeClassifierTraitConst for CascadeClassifier {
		#[inline] fn as_raw_CascadeClassifier(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::CascadeClassifierTrait for CascadeClassifier {
		#[inline] fn as_raw_mut_CascadeClassifier(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CascadeClassifier {
		#[inline]
		pub fn default() -> Result<crate::objdetect::CascadeClassifier> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_CascadeClassifier_CascadeClassifier(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::CascadeClassifier::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Loads a classifier from a file.
		/// 
		/// ## Parameters
		/// * filename: Name of the file from which the classifier is loaded.
		#[inline]
		pub fn new(filename: &str) -> Result<crate::objdetect::CascadeClassifier> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_CascadeClassifier_CascadeClassifier_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::CascadeClassifier::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn convert(oldcascade: &str, newcascade: &str) -> Result<bool> {
			extern_container_arg!(oldcascade);
			extern_container_arg!(newcascade);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_CascadeClassifier_convert_const_StringR_const_StringR(oldcascade.opencv_as_extern(), newcascade.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	impl std::fmt::Debug for CascadeClassifier {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CascadeClassifier")
				.finish()
		}
	}
	
	/// Constant methods for [crate::objdetect::DetectionBasedTracker]
	pub trait DetectionBasedTrackerTraitConst {
		fn as_raw_DetectionBasedTracker(&self) -> *const c_void;
	
		#[inline]
		fn get_parameters(&self) -> Result<crate::objdetect::DetectionBasedTracker_Parameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DetectionBasedTracker_getParameters_const(self.as_raw_DetectionBasedTracker(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::DetectionBasedTracker_Parameters::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_objects(&self, result: &mut core::Vector<core::Rect>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DetectionBasedTracker_getObjects_const_vectorLRectGR(self.as_raw_DetectionBasedTracker(), result.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_objects_1(&self, result: &mut core::Vector<crate::objdetect::DetectionBasedTracker_Object>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DetectionBasedTracker_getObjects_const_vectorLObjectGR(self.as_raw_DetectionBasedTracker(), result.as_raw_mut_VectorOfDetectionBasedTracker_Object(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_objects_2(&self, result: &mut core::Vector<crate::objdetect::DetectionBasedTracker_ExtObject>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DetectionBasedTracker_getObjects_const_vectorLExtObjectGR(self.as_raw_DetectionBasedTracker(), result.as_raw_mut_VectorOfDetectionBasedTracker_ExtObject(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::objdetect::DetectionBasedTracker]
	pub trait DetectionBasedTrackerTrait: crate::objdetect::DetectionBasedTrackerTraitConst {
		fn as_raw_mut_DetectionBasedTracker(&mut self) -> *mut c_void;
	
		#[inline]
		fn run(&mut self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DetectionBasedTracker_run(self.as_raw_mut_DetectionBasedTracker(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn stop(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DetectionBasedTracker_stop(self.as_raw_mut_DetectionBasedTracker(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn reset_tracking(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DetectionBasedTracker_resetTracking(self.as_raw_mut_DetectionBasedTracker(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn process(&mut self, image_gray: &core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DetectionBasedTracker_process_const_MatR(self.as_raw_mut_DetectionBasedTracker(), image_gray.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_parameters(&mut self, params: &crate::objdetect::DetectionBasedTracker_Parameters) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DetectionBasedTracker_setParameters_const_ParametersR(self.as_raw_mut_DetectionBasedTracker(), params.as_raw_DetectionBasedTracker_Parameters(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn add_object(&mut self, location: core::Rect) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DetectionBasedTracker_addObject_const_RectR(self.as_raw_mut_DetectionBasedTracker(), &location, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct DetectionBasedTracker {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { DetectionBasedTracker }
	
	impl Drop for DetectionBasedTracker {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_DetectionBasedTracker_delete(self.as_raw_mut_DetectionBasedTracker()) };
		}
	}
	
	unsafe impl Send for DetectionBasedTracker {}
	
	impl crate::objdetect::DetectionBasedTrackerTraitConst for DetectionBasedTracker {
		#[inline] fn as_raw_DetectionBasedTracker(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::DetectionBasedTrackerTrait for DetectionBasedTracker {
		#[inline] fn as_raw_mut_DetectionBasedTracker(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl DetectionBasedTracker {
		#[inline]
		pub fn new(mut main_detector: core::Ptr<crate::objdetect::DetectionBasedTracker_IDetector>, mut tracking_detector: core::Ptr<crate::objdetect::DetectionBasedTracker_IDetector>, params: &crate::objdetect::DetectionBasedTracker_Parameters) -> Result<crate::objdetect::DetectionBasedTracker> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DetectionBasedTracker_DetectionBasedTracker_PtrLIDetectorG_PtrLIDetectorG_const_ParametersR(main_detector.as_raw_mut_PtrOfDetectionBasedTracker_IDetector(), tracking_detector.as_raw_mut_PtrOfDetectionBasedTracker_IDetector(), params.as_raw_DetectionBasedTracker_Parameters(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::DetectionBasedTracker::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl std::fmt::Debug for DetectionBasedTracker {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("DetectionBasedTracker")
				.finish()
		}
	}
	
	/// Constant methods for [crate::objdetect::DetectionBasedTracker_ExtObject]
	pub trait DetectionBasedTracker_ExtObjectTraitConst {
		fn as_raw_DetectionBasedTracker_ExtObject(&self) -> *const c_void;
	
		#[inline]
		fn id(&self) -> i32 {
			let ret = unsafe { sys::cv_DetectionBasedTracker_ExtObject_propId_const(self.as_raw_DetectionBasedTracker_ExtObject()) };
			ret
		}
		
		#[inline]
		fn location(&self) -> core::Rect {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DetectionBasedTracker_ExtObject_propLocation_const(self.as_raw_DetectionBasedTracker_ExtObject(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		#[inline]
		fn status(&self) -> crate::objdetect::DetectionBasedTracker_ObjectStatus {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DetectionBasedTracker_ExtObject_propStatus_const(self.as_raw_DetectionBasedTracker_ExtObject(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
	}
	
	/// Mutable methods for [crate::objdetect::DetectionBasedTracker_ExtObject]
	pub trait DetectionBasedTracker_ExtObjectTrait: crate::objdetect::DetectionBasedTracker_ExtObjectTraitConst {
		fn as_raw_mut_DetectionBasedTracker_ExtObject(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_id(&mut self, val: i32) {
			let ret = unsafe { sys::cv_DetectionBasedTracker_ExtObject_propId_int(self.as_raw_mut_DetectionBasedTracker_ExtObject(), val) };
			ret
		}
		
		#[inline]
		fn set_location(&mut self, val: core::Rect) {
			let ret = unsafe { sys::cv_DetectionBasedTracker_ExtObject_propLocation_Rect(self.as_raw_mut_DetectionBasedTracker_ExtObject(), val.opencv_as_extern()) };
			ret
		}
		
		#[inline]
		fn set_status(&mut self, val: crate::objdetect::DetectionBasedTracker_ObjectStatus) {
			let ret = unsafe { sys::cv_DetectionBasedTracker_ExtObject_propStatus_ObjectStatus(self.as_raw_mut_DetectionBasedTracker_ExtObject(), val) };
			ret
		}
		
	}
	
	pub struct DetectionBasedTracker_ExtObject {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { DetectionBasedTracker_ExtObject }
	
	impl Drop for DetectionBasedTracker_ExtObject {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_DetectionBasedTracker_ExtObject_delete(self.as_raw_mut_DetectionBasedTracker_ExtObject()) };
		}
	}
	
	unsafe impl Send for DetectionBasedTracker_ExtObject {}
	
	impl crate::objdetect::DetectionBasedTracker_ExtObjectTraitConst for DetectionBasedTracker_ExtObject {
		#[inline] fn as_raw_DetectionBasedTracker_ExtObject(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::DetectionBasedTracker_ExtObjectTrait for DetectionBasedTracker_ExtObject {
		#[inline] fn as_raw_mut_DetectionBasedTracker_ExtObject(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl DetectionBasedTracker_ExtObject {
		#[inline]
		pub fn new(_id: i32, _location: core::Rect, _status: crate::objdetect::DetectionBasedTracker_ObjectStatus) -> Result<crate::objdetect::DetectionBasedTracker_ExtObject> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DetectionBasedTracker_ExtObject_ExtObject_int_Rect_ObjectStatus(_id, _location.opencv_as_extern(), _status, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::DetectionBasedTracker_ExtObject::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl Clone for DetectionBasedTracker_ExtObject {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_DetectionBasedTracker_ExtObject_implicitClone_const(self.as_raw_DetectionBasedTracker_ExtObject())) }
		}
	}
	
	impl std::fmt::Debug for DetectionBasedTracker_ExtObject {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("DetectionBasedTracker_ExtObject")
				.field("id", &crate::objdetect::DetectionBasedTracker_ExtObjectTraitConst::id(self))
				.field("location", &crate::objdetect::DetectionBasedTracker_ExtObjectTraitConst::location(self))
				.field("status", &crate::objdetect::DetectionBasedTracker_ExtObjectTraitConst::status(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::objdetect::DetectionBasedTracker_IDetector]
	pub trait DetectionBasedTracker_IDetectorTraitConst {
		fn as_raw_DetectionBasedTracker_IDetector(&self) -> *const c_void;
	
		#[inline]
		fn get_min_object_size(&self) -> Result<core::Size> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DetectionBasedTracker_IDetector_getMinObjectSize_const(self.as_raw_DetectionBasedTracker_IDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_max_object_size(&self) -> Result<core::Size> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DetectionBasedTracker_IDetector_getMaxObjectSize_const(self.as_raw_DetectionBasedTracker_IDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::objdetect::DetectionBasedTracker_IDetector]
	pub trait DetectionBasedTracker_IDetectorTrait: crate::objdetect::DetectionBasedTracker_IDetectorTraitConst {
		fn as_raw_mut_DetectionBasedTracker_IDetector(&mut self) -> *mut c_void;
	
		#[inline]
		fn detect(&mut self, image: &core::Mat, objects: &mut core::Vector<core::Rect>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DetectionBasedTracker_IDetector_detect_const_MatR_vectorLRectGR(self.as_raw_mut_DetectionBasedTracker_IDetector(), image.as_raw_Mat(), objects.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_min_object_size(&mut self, min: core::Size) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DetectionBasedTracker_IDetector_setMinObjectSize_const_SizeR(self.as_raw_mut_DetectionBasedTracker_IDetector(), &min, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_max_object_size(&mut self, max: core::Size) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DetectionBasedTracker_IDetector_setMaxObjectSize_const_SizeR(self.as_raw_mut_DetectionBasedTracker_IDetector(), &max, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_scale_factor(&mut self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DetectionBasedTracker_IDetector_getScaleFactor(self.as_raw_mut_DetectionBasedTracker_IDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_scale_factor(&mut self, value: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DetectionBasedTracker_IDetector_setScaleFactor_float(self.as_raw_mut_DetectionBasedTracker_IDetector(), value, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_min_neighbours(&mut self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DetectionBasedTracker_IDetector_getMinNeighbours(self.as_raw_mut_DetectionBasedTracker_IDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_min_neighbours(&mut self, value: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DetectionBasedTracker_IDetector_setMinNeighbours_int(self.as_raw_mut_DetectionBasedTracker_IDetector(), value, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct DetectionBasedTracker_IDetector {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { DetectionBasedTracker_IDetector }
	
	impl Drop for DetectionBasedTracker_IDetector {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_DetectionBasedTracker_IDetector_delete(self.as_raw_mut_DetectionBasedTracker_IDetector()) };
		}
	}
	
	unsafe impl Send for DetectionBasedTracker_IDetector {}
	
	impl crate::objdetect::DetectionBasedTracker_IDetectorTraitConst for DetectionBasedTracker_IDetector {
		#[inline] fn as_raw_DetectionBasedTracker_IDetector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::DetectionBasedTracker_IDetectorTrait for DetectionBasedTracker_IDetector {
		#[inline] fn as_raw_mut_DetectionBasedTracker_IDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl DetectionBasedTracker_IDetector {
	}
	
	impl std::fmt::Debug for DetectionBasedTracker_IDetector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("DetectionBasedTracker_IDetector")
				.finish()
		}
	}
	
	/// Constant methods for [crate::objdetect::DetectionBasedTracker_Parameters]
	pub trait DetectionBasedTracker_ParametersTraitConst {
		fn as_raw_DetectionBasedTracker_Parameters(&self) -> *const c_void;
	
		#[inline]
		fn max_track_lifetime(&self) -> i32 {
			let ret = unsafe { sys::cv_DetectionBasedTracker_Parameters_propMaxTrackLifetime_const(self.as_raw_DetectionBasedTracker_Parameters()) };
			ret
		}
		
		#[inline]
		fn min_detection_period(&self) -> i32 {
			let ret = unsafe { sys::cv_DetectionBasedTracker_Parameters_propMinDetectionPeriod_const(self.as_raw_DetectionBasedTracker_Parameters()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::objdetect::DetectionBasedTracker_Parameters]
	pub trait DetectionBasedTracker_ParametersTrait: crate::objdetect::DetectionBasedTracker_ParametersTraitConst {
		fn as_raw_mut_DetectionBasedTracker_Parameters(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_max_track_lifetime(&mut self, val: i32) {
			let ret = unsafe { sys::cv_DetectionBasedTracker_Parameters_propMaxTrackLifetime_int(self.as_raw_mut_DetectionBasedTracker_Parameters(), val) };
			ret
		}
		
		#[inline]
		fn set_min_detection_period(&mut self, val: i32) {
			let ret = unsafe { sys::cv_DetectionBasedTracker_Parameters_propMinDetectionPeriod_int(self.as_raw_mut_DetectionBasedTracker_Parameters(), val) };
			ret
		}
		
	}
	
	pub struct DetectionBasedTracker_Parameters {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { DetectionBasedTracker_Parameters }
	
	impl Drop for DetectionBasedTracker_Parameters {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_DetectionBasedTracker_Parameters_delete(self.as_raw_mut_DetectionBasedTracker_Parameters()) };
		}
	}
	
	unsafe impl Send for DetectionBasedTracker_Parameters {}
	
	impl crate::objdetect::DetectionBasedTracker_ParametersTraitConst for DetectionBasedTracker_Parameters {
		#[inline] fn as_raw_DetectionBasedTracker_Parameters(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::DetectionBasedTracker_ParametersTrait for DetectionBasedTracker_Parameters {
		#[inline] fn as_raw_mut_DetectionBasedTracker_Parameters(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl DetectionBasedTracker_Parameters {
		#[inline]
		pub fn default() -> Result<crate::objdetect::DetectionBasedTracker_Parameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DetectionBasedTracker_Parameters_Parameters(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::DetectionBasedTracker_Parameters::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl std::fmt::Debug for DetectionBasedTracker_Parameters {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("DetectionBasedTracker_Parameters")
				.field("max_track_lifetime", &crate::objdetect::DetectionBasedTracker_ParametersTraitConst::max_track_lifetime(self))
				.field("min_detection_period", &crate::objdetect::DetectionBasedTracker_ParametersTraitConst::min_detection_period(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::objdetect::DetectionROI]
	pub trait DetectionROITraitConst {
		fn as_raw_DetectionROI(&self) -> *const c_void;
	
		/// scale(size) of the bounding box
		#[inline]
		fn scale(&self) -> f64 {
			let ret = unsafe { sys::cv_DetectionROI_propScale_const(self.as_raw_DetectionROI()) };
			ret
		}
		
		/// set of requested locations to be evaluated
		#[inline]
		fn locations(&self) -> core::Vector<core::Point> {
			let ret = unsafe { sys::cv_DetectionROI_propLocations_const(self.as_raw_DetectionROI()) };
			let ret = unsafe { core::Vector::<core::Point>::opencv_from_extern(ret) };
			ret
		}
		
		/// vector that will contain confidence values for each location
		#[inline]
		fn confidences(&self) -> core::Vector<f64> {
			let ret = unsafe { sys::cv_DetectionROI_propConfidences_const(self.as_raw_DetectionROI()) };
			let ret = unsafe { core::Vector::<f64>::opencv_from_extern(ret) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::objdetect::DetectionROI]
	pub trait DetectionROITrait: crate::objdetect::DetectionROITraitConst {
		fn as_raw_mut_DetectionROI(&mut self) -> *mut c_void;
	
		/// scale(size) of the bounding box
		#[inline]
		fn set_scale(&mut self, val: f64) {
			let ret = unsafe { sys::cv_DetectionROI_propScale_double(self.as_raw_mut_DetectionROI(), val) };
			ret
		}
		
		/// set of requested locations to be evaluated
		#[inline]
		fn set_locations(&mut self, mut val: core::Vector<core::Point>) {
			let ret = unsafe { sys::cv_DetectionROI_propLocations_vectorLPointG(self.as_raw_mut_DetectionROI(), val.as_raw_mut_VectorOfPoint()) };
			ret
		}
		
		/// vector that will contain confidence values for each location
		#[inline]
		fn set_confidences(&mut self, mut val: core::Vector<f64>) {
			let ret = unsafe { sys::cv_DetectionROI_propConfidences_vectorLdoubleG(self.as_raw_mut_DetectionROI(), val.as_raw_mut_VectorOff64()) };
			ret
		}
		
	}
	
	/// struct for detection region of interest (ROI)
	pub struct DetectionROI {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { DetectionROI }
	
	impl Drop for DetectionROI {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_DetectionROI_delete(self.as_raw_mut_DetectionROI()) };
		}
	}
	
	unsafe impl Send for DetectionROI {}
	
	impl crate::objdetect::DetectionROITraitConst for DetectionROI {
		#[inline] fn as_raw_DetectionROI(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::DetectionROITrait for DetectionROI {
		#[inline] fn as_raw_mut_DetectionROI(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl DetectionROI {
	}
	
	impl std::fmt::Debug for DetectionROI {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("DetectionROI")
				.field("scale", &crate::objdetect::DetectionROITraitConst::scale(self))
				.field("locations", &crate::objdetect::DetectionROITraitConst::locations(self))
				.field("confidences", &crate::objdetect::DetectionROITraitConst::confidences(self))
				.finish()
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
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_input_size(&mut self) -> Result<core::Size> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceDetectorYN_getInputSize(self.as_raw_mut_FaceDetectorYN(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_score_threshold(&mut self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceDetectorYN_getScoreThreshold(self.as_raw_mut_FaceDetectorYN(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_nms_threshold(&mut self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceDetectorYN_getNMSThreshold(self.as_raw_mut_FaceDetectorYN(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_top_k(&mut self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceDetectorYN_getTopK(self.as_raw_mut_FaceDetectorYN(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Detects faces in the input image. Following is an example output.
		/// 
		/// * ![image](https://docs.opencv.org/4.8.1/lena-face-detection.jpg)
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
		fn detect(&mut self, image: &impl core::ToInputArray, faces: &mut impl core::ToOutputArray) -> Result<i32> {
			input_array_arg!(image);
			output_array_arg!(faces);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceDetectorYN_detect_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_FaceDetectorYN(), image.as_raw__InputArray(), faces.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// DNN-based face detector
	/// 
	/// model download link: <https://github.com/opencv/opencv_zoo/tree/master/models/face_detection_yunet>
	pub struct FaceDetectorYN {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { FaceDetectorYN }
	
	impl Drop for FaceDetectorYN {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_FaceDetectorYN_delete(self.as_raw_mut_FaceDetectorYN()) };
		}
	}
	
	unsafe impl Send for FaceDetectorYN {}
	
	impl crate::objdetect::FaceDetectorYNTraitConst for FaceDetectorYN {
		#[inline] fn as_raw_FaceDetectorYN(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::FaceDetectorYNTrait for FaceDetectorYN {
		#[inline] fn as_raw_mut_FaceDetectorYN(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl FaceDetectorYN {
		/// Creates an instance of this class with given parameters
		/// 
		/// ## Parameters
		/// * model: the path to the requested model
		/// * config: the path to the config file for compability, which is not requested for ONNX models
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
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::objdetect::FaceDetectorYN>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Creates an instance of this class with given parameters
		/// 
		/// ## Parameters
		/// * model: the path to the requested model
		/// * config: the path to the config file for compability, which is not requested for ONNX models
		/// * input_size: the size of the input image
		/// * score_threshold: the threshold to filter out bounding boxes of score smaller than the given value
		/// * nms_threshold: the threshold to suppress bounding boxes of IoU bigger than the given value
		/// * top_k: keep top K bboxes before NMS
		/// * backend_id: the id of backend
		/// * target_id: the id of target device
		/// 
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
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
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::objdetect::FaceDetectorYN>::opencv_from_extern(ret) };
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
	
	/// Constant methods for [crate::objdetect::FaceRecognizerSF]
	pub trait FaceRecognizerSFTraitConst {
		fn as_raw_FaceRecognizerSF(&self) -> *const c_void;
	
		/// Aligning image to put face on the standard position
		/// ## Parameters
		/// * src_img: input image
		/// * face_box: the detection result used for indicate face in input image
		/// * aligned_img: output aligned image
		#[inline]
		fn align_crop(&self, src_img: &impl core::ToInputArray, face_box: &impl core::ToInputArray, aligned_img: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(src_img);
			input_array_arg!(face_box);
			output_array_arg!(aligned_img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceRecognizerSF_alignCrop_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_FaceRecognizerSF(), src_img.as_raw__InputArray(), face_box.as_raw__InputArray(), aligned_img.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Calculating the distance between two face features
		/// ## Parameters
		/// * face_feature1: the first input feature
		/// * face_feature2: the second input feature of the same size and the same type as face_feature1
		/// * dis_type: defining the similarity with optional values "FR_OSINE" or "FR_NORM_L2"
		/// 
		/// ## C++ default parameters
		/// * dis_type: FaceRecognizerSF::FR_COSINE
		#[inline]
		fn match_(&self, face_feature1: &impl core::ToInputArray, face_feature2: &impl core::ToInputArray, dis_type: i32) -> Result<f64> {
			input_array_arg!(face_feature1);
			input_array_arg!(face_feature2);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceRecognizerSF_match_const_const__InputArrayR_const__InputArrayR_int(self.as_raw_FaceRecognizerSF(), face_feature1.as_raw__InputArray(), face_feature2.as_raw__InputArray(), dis_type, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Calculating the distance between two face features
		/// ## Parameters
		/// * face_feature1: the first input feature
		/// * face_feature2: the second input feature of the same size and the same type as face_feature1
		/// * dis_type: defining the similarity with optional values "FR_OSINE" or "FR_NORM_L2"
		/// 
		/// ## Note
		/// This alternative version of [match_] function uses the following default values for its arguments:
		/// * dis_type: FaceRecognizerSF::FR_COSINE
		#[inline]
		fn match__def(&self, face_feature1: &impl core::ToInputArray, face_feature2: &impl core::ToInputArray) -> Result<f64> {
			input_array_arg!(face_feature1);
			input_array_arg!(face_feature2);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceRecognizerSF_match_const_const__InputArrayR_const__InputArrayR(self.as_raw_FaceRecognizerSF(), face_feature1.as_raw__InputArray(), face_feature2.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::objdetect::FaceRecognizerSF]
	pub trait FaceRecognizerSFTrait: crate::objdetect::FaceRecognizerSFTraitConst {
		fn as_raw_mut_FaceRecognizerSF(&mut self) -> *mut c_void;
	
		/// Extracting face feature from aligned image
		/// ## Parameters
		/// * aligned_img: input aligned image
		/// * face_feature: output face feature
		#[inline]
		fn feature(&mut self, aligned_img: &impl core::ToInputArray, face_feature: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(aligned_img);
			output_array_arg!(face_feature);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceRecognizerSF_feature_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_FaceRecognizerSF(), aligned_img.as_raw__InputArray(), face_feature.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// DNN-based face recognizer
	/// 
	/// model download link: <https://github.com/opencv/opencv_zoo/tree/master/models/face_recognition_sface>
	pub struct FaceRecognizerSF {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { FaceRecognizerSF }
	
	impl Drop for FaceRecognizerSF {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_FaceRecognizerSF_delete(self.as_raw_mut_FaceRecognizerSF()) };
		}
	}
	
	unsafe impl Send for FaceRecognizerSF {}
	
	impl crate::objdetect::FaceRecognizerSFTraitConst for FaceRecognizerSF {
		#[inline] fn as_raw_FaceRecognizerSF(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::FaceRecognizerSFTrait for FaceRecognizerSF {
		#[inline] fn as_raw_mut_FaceRecognizerSF(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl FaceRecognizerSF {
		/// Creates an instance of this class with given parameters
		/// ## Parameters
		/// * model: the path of the onnx model used for face recognition
		/// * config: the path to the config file for compability, which is not requested for ONNX models
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
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::objdetect::FaceRecognizerSF>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Creates an instance of this class with given parameters
		/// ## Parameters
		/// * model: the path of the onnx model used for face recognition
		/// * config: the path to the config file for compability, which is not requested for ONNX models
		/// * backend_id: the id of backend
		/// * target_id: the id of target device
		/// 
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * backend_id: 0
		/// * target_id: 0
		#[inline]
		pub fn create_def(model: &str, config: &str) -> Result<core::Ptr<crate::objdetect::FaceRecognizerSF>> {
			extern_container_arg!(model);
			extern_container_arg!(config);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FaceRecognizerSF_create_const_StringR_const_StringR(model.opencv_as_extern(), config.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::objdetect::FaceRecognizerSF>::opencv_from_extern(ret) };
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
	
	/// Constant methods for [crate::objdetect::GraphicalCodeDetector]
	pub trait GraphicalCodeDetectorTraitConst {
		fn as_raw_GraphicalCodeDetector(&self) -> *const c_void;
	
		/// Detects graphical code in image and returns the quadrangle containing the code.
		/// ## Parameters
		/// * img: grayscale or color (BGR) image containing (or not) graphical code.
		/// * points: Output vector of vertices of the minimum-area quadrangle containing the code.
		#[inline]
		fn detect(&self, img: &impl core::ToInputArray, points: &mut impl core::ToOutputArray) -> Result<bool> {
			input_array_arg!(img);
			output_array_arg!(points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GraphicalCodeDetector_detect_const_const__InputArrayR_const__OutputArrayR(self.as_raw_GraphicalCodeDetector(), img.as_raw__InputArray(), points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
		fn decode(&self, img: &impl core::ToInputArray, points: &impl core::ToInputArray, straight_code: &mut impl core::ToOutputArray) -> Result<Vec<u8>> {
			input_array_arg!(img);
			input_array_arg!(points);
			output_array_arg!(straight_code);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GraphicalCodeDetector_decode_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_GraphicalCodeDetector(), img.as_raw__InputArray(), points.as_raw__InputArray(), straight_code.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
		/// This alternative version of [decode] function uses the following default values for its arguments:
		/// * straight_code: noArray()
		#[inline]
		fn decode_def(&self, img: &impl core::ToInputArray, points: &impl core::ToInputArray) -> Result<Vec<u8>> {
			input_array_arg!(img);
			input_array_arg!(points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GraphicalCodeDetector_decode_const_const__InputArrayR_const__InputArrayR(self.as_raw_GraphicalCodeDetector(), img.as_raw__InputArray(), points.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
		fn detect_and_decode(&self, img: &impl core::ToInputArray, points: &mut impl core::ToOutputArray, straight_code: &mut impl core::ToOutputArray) -> Result<Vec<u8>> {
			input_array_arg!(img);
			output_array_arg!(points);
			output_array_arg!(straight_code);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GraphicalCodeDetector_detectAndDecode_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_GraphicalCodeDetector(), img.as_raw__InputArray(), points.as_raw__OutputArray(), straight_code.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
		/// This alternative version of [detect_and_decode] function uses the following default values for its arguments:
		/// * points: noArray()
		/// * straight_code: noArray()
		#[inline]
		fn detect_and_decode_def(&self, img: &impl core::ToInputArray) -> Result<Vec<u8>> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GraphicalCodeDetector_detectAndDecode_const_const__InputArrayR(self.as_raw_GraphicalCodeDetector(), img.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { Vec::<u8>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Detects graphical codes in image and returns the vector of the quadrangles containing the codes.
		/// ## Parameters
		/// * img: grayscale or color (BGR) image containing (or not) graphical codes.
		/// * points: Output vector of vector of vertices of the minimum-area quadrangle containing the codes.
		#[inline]
		fn detect_multi(&self, img: &impl core::ToInputArray, points: &mut impl core::ToOutputArray) -> Result<bool> {
			input_array_arg!(img);
			output_array_arg!(points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GraphicalCodeDetector_detectMulti_const_const__InputArrayR_const__OutputArrayR(self.as_raw_GraphicalCodeDetector(), img.as_raw__InputArray(), points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
		fn decode_multi(&self, img: &impl core::ToInputArray, points: &impl core::ToInputArray, decoded_info: &mut core::Vector<String>, straight_code: &mut impl core::ToOutputArray) -> Result<bool> {
			input_array_arg!(img);
			input_array_arg!(points);
			output_array_arg!(straight_code);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GraphicalCodeDetector_decodeMulti_const_const__InputArrayR_const__InputArrayR_vectorLstringGR_const__OutputArrayR(self.as_raw_GraphicalCodeDetector(), img.as_raw__InputArray(), points.as_raw__InputArray(), decoded_info.as_raw_mut_VectorOfString(), straight_code.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
		/// This alternative version of [decode_multi] function uses the following default values for its arguments:
		/// * straight_code: noArray()
		#[inline]
		fn decode_multi_def(&self, img: &impl core::ToInputArray, points: &impl core::ToInputArray, decoded_info: &mut core::Vector<String>) -> Result<bool> {
			input_array_arg!(img);
			input_array_arg!(points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GraphicalCodeDetector_decodeMulti_const_const__InputArrayR_const__InputArrayR_vectorLstringGR(self.as_raw_GraphicalCodeDetector(), img.as_raw__InputArray(), points.as_raw__InputArray(), decoded_info.as_raw_mut_VectorOfString(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
		/// ## C++ default parameters
		/// * points: noArray()
		/// * straight_code: noArray()
		#[inline]
		fn detect_and_decode_multi(&self, img: &impl core::ToInputArray, decoded_info: &mut core::Vector<String>, points: &mut impl core::ToOutputArray, straight_code: &mut impl core::ToOutputArray) -> Result<bool> {
			input_array_arg!(img);
			output_array_arg!(points);
			output_array_arg!(straight_code);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GraphicalCodeDetector_detectAndDecodeMulti_const_const__InputArrayR_vectorLstringGR_const__OutputArrayR_const__OutputArrayR(self.as_raw_GraphicalCodeDetector(), img.as_raw__InputArray(), decoded_info.as_raw_mut_VectorOfString(), points.as_raw__OutputArray(), straight_code.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
		/// ## Note
		/// This alternative version of [detect_and_decode_multi] function uses the following default values for its arguments:
		/// * points: noArray()
		/// * straight_code: noArray()
		#[inline]
		fn detect_and_decode_multi_def(&self, img: &impl core::ToInputArray, decoded_info: &mut core::Vector<String>) -> Result<bool> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GraphicalCodeDetector_detectAndDecodeMulti_const_const__InputArrayR_vectorLstringGR(self.as_raw_GraphicalCodeDetector(), img.as_raw__InputArray(), decoded_info.as_raw_mut_VectorOfString(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::objdetect::GraphicalCodeDetector]
	pub trait GraphicalCodeDetectorTrait: crate::objdetect::GraphicalCodeDetectorTraitConst {
		fn as_raw_mut_GraphicalCodeDetector(&mut self) -> *mut c_void;
	
	}
	
	pub struct GraphicalCodeDetector {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GraphicalCodeDetector }
	
	impl Drop for GraphicalCodeDetector {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_GraphicalCodeDetector_delete(self.as_raw_mut_GraphicalCodeDetector()) };
		}
	}
	
	unsafe impl Send for GraphicalCodeDetector {}
	
	impl crate::objdetect::GraphicalCodeDetectorTraitConst for GraphicalCodeDetector {
		#[inline] fn as_raw_GraphicalCodeDetector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::GraphicalCodeDetectorTrait for GraphicalCodeDetector {
		#[inline] fn as_raw_mut_GraphicalCodeDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GraphicalCodeDetector {
		#[inline]
		pub fn default() -> Result<crate::objdetect::GraphicalCodeDetector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GraphicalCodeDetector_GraphicalCodeDetector(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::GraphicalCodeDetector::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn copy(unnamed: &crate::objdetect::GraphicalCodeDetector) -> crate::objdetect::GraphicalCodeDetector {
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
	
	/// Constant methods for [crate::objdetect::HOGDescriptor]
	pub trait HOGDescriptorTraitConst {
		fn as_raw_HOGDescriptor(&self) -> *const c_void;
	
		/// Detection window size. Align to block size and block stride. Default value is Size(64,128).
		#[inline]
		fn win_size(&self) -> core::Size {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_propWinSize_const(self.as_raw_HOGDescriptor(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		/// Block size in pixels. Align to cell size. Default value is Size(16,16).
		#[inline]
		fn block_size(&self) -> core::Size {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_propBlockSize_const(self.as_raw_HOGDescriptor(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		/// Block stride. It must be a multiple of cell size. Default value is Size(8,8).
		#[inline]
		fn block_stride(&self) -> core::Size {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_propBlockStride_const(self.as_raw_HOGDescriptor(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		/// Cell size. Default value is Size(8,8).
		#[inline]
		fn cell_size(&self) -> core::Size {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_propCellSize_const(self.as_raw_HOGDescriptor(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		/// Number of bins used in the calculation of histogram of gradients. Default value is 9.
		#[inline]
		fn nbins(&self) -> i32 {
			let ret = unsafe { sys::cv_HOGDescriptor_propNbins_const(self.as_raw_HOGDescriptor()) };
			ret
		}
		
		/// not documented
		#[inline]
		fn deriv_aperture(&self) -> i32 {
			let ret = unsafe { sys::cv_HOGDescriptor_propDerivAperture_const(self.as_raw_HOGDescriptor()) };
			ret
		}
		
		/// Gaussian smoothing window parameter.
		#[inline]
		fn win_sigma(&self) -> f64 {
			let ret = unsafe { sys::cv_HOGDescriptor_propWinSigma_const(self.as_raw_HOGDescriptor()) };
			ret
		}
		
		/// histogramNormType
		#[inline]
		fn histogram_norm_type(&self) -> crate::objdetect::HOGDescriptor_HistogramNormType {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_propHistogramNormType_const(self.as_raw_HOGDescriptor(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		/// L2-Hys normalization method shrinkage.
		#[inline]
		fn l2_hys_threshold(&self) -> f64 {
			let ret = unsafe { sys::cv_HOGDescriptor_propL2HysThreshold_const(self.as_raw_HOGDescriptor()) };
			ret
		}
		
		/// Flag to specify whether the gamma correction preprocessing is required or not.
		#[inline]
		fn gamma_correction(&self) -> bool {
			let ret = unsafe { sys::cv_HOGDescriptor_propGammaCorrection_const(self.as_raw_HOGDescriptor()) };
			ret
		}
		
		/// coefficients for the linear SVM classifier.
		#[inline]
		fn svm_detector(&self) -> core::Vector<f32> {
			let ret = unsafe { sys::cv_HOGDescriptor_propSvmDetector_const(self.as_raw_HOGDescriptor()) };
			let ret = unsafe { core::Vector::<f32>::opencv_from_extern(ret) };
			ret
		}
		
		/// coefficients for the linear SVM classifier used when OpenCL is enabled
		#[inline]
		fn ocl_svm_detector(&self) -> core::UMat {
			let ret = unsafe { sys::cv_HOGDescriptor_propOclSvmDetector_const(self.as_raw_HOGDescriptor()) };
			let ret = unsafe { core::UMat::opencv_from_extern(ret) };
			ret
		}
		
		/// not documented
		#[inline]
		fn free_coef(&self) -> f32 {
			let ret = unsafe { sys::cv_HOGDescriptor_propFree_coef_const(self.as_raw_HOGDescriptor()) };
			ret
		}
		
		/// Maximum number of detection window increases. Default value is 64
		#[inline]
		fn nlevels(&self) -> i32 {
			let ret = unsafe { sys::cv_HOGDescriptor_propNlevels_const(self.as_raw_HOGDescriptor()) };
			ret
		}
		
		/// Indicates signed gradient will be used or not
		#[inline]
		fn signed_gradient(&self) -> bool {
			let ret = unsafe { sys::cv_HOGDescriptor_propSignedGradient_const(self.as_raw_HOGDescriptor()) };
			ret
		}
		
		/// Returns the number of coefficients required for the classification.
		#[inline]
		fn get_descriptor_size(&self) -> Result<size_t> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_getDescriptorSize_const(self.as_raw_HOGDescriptor(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Checks if detector size equal to descriptor size.
		#[inline]
		fn check_detector_size(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_checkDetectorSize_const(self.as_raw_HOGDescriptor(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Returns winSigma value
		#[inline]
		fn get_win_sigma(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_getWinSigma_const(self.as_raw_HOGDescriptor(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Stores HOGDescriptor parameters and coefficients for the linear SVM classifier in a file storage.
		/// ## Parameters
		/// * fs: File storage
		/// * objname: Object name
		#[inline]
		fn write(&self, fs: &mut core::FileStorage, objname: &str) -> Result<()> {
			extern_container_arg!(objname);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_write_const_FileStorageR_const_StringR(self.as_raw_HOGDescriptor(), fs.as_raw_mut_FileStorage(), objname.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// saves HOGDescriptor parameters and coefficients for the linear SVM classifier to a file
		/// ## Parameters
		/// * filename: File name
		/// * objname: Object name
		/// 
		/// ## C++ default parameters
		/// * objname: String()
		#[inline]
		fn save(&self, filename: &str, objname: &str) -> Result<()> {
			extern_container_arg!(filename);
			extern_container_arg!(objname);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_save_const_const_StringR_const_StringR(self.as_raw_HOGDescriptor(), filename.opencv_as_extern(), objname.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// saves HOGDescriptor parameters and coefficients for the linear SVM classifier to a file
		/// ## Parameters
		/// * filename: File name
		/// * objname: Object name
		/// 
		/// ## Note
		/// This alternative version of [save] function uses the following default values for its arguments:
		/// * objname: String()
		#[inline]
		fn save_def(&self, filename: &str) -> Result<()> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_save_const_const_StringR(self.as_raw_HOGDescriptor(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// clones the HOGDescriptor
		/// ## Parameters
		/// * c: cloned HOGDescriptor
		#[inline]
		fn copy_to(&self, c: &mut crate::objdetect::HOGDescriptor) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_copyTo_const_HOGDescriptorR(self.as_raw_HOGDescriptor(), c.as_raw_mut_HOGDescriptor(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// @example samples/cpp/train_HOG.cpp
		/// /
		/// Computes HOG descriptors of given image.
		/// ## Parameters
		/// * img: Matrix of the type CV_8U containing an image where HOG features will be calculated.
		/// * descriptors: Matrix of the type CV_32F
		/// * winStride: Window stride. It must be a multiple of block stride.
		/// * padding: Padding
		/// * locations: Vector of Point
		/// 
		/// ## C++ default parameters
		/// * win_stride: Size()
		/// * padding: Size()
		/// * locations: std::vector<Point>()
		#[inline]
		fn compute(&self, img: &impl core::ToInputArray, descriptors: &mut core::Vector<f32>, win_stride: core::Size, padding: core::Size, locations: &core::Vector<core::Point>) -> Result<()> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_compute_const_const__InputArrayR_vectorLfloatGR_Size_Size_const_vectorLPointGR(self.as_raw_HOGDescriptor(), img.as_raw__InputArray(), descriptors.as_raw_mut_VectorOff32(), win_stride.opencv_as_extern(), padding.opencv_as_extern(), locations.as_raw_VectorOfPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// @example samples/cpp/train_HOG.cpp
		/// /
		/// Computes HOG descriptors of given image.
		/// ## Parameters
		/// * img: Matrix of the type CV_8U containing an image where HOG features will be calculated.
		/// * descriptors: Matrix of the type CV_32F
		/// * winStride: Window stride. It must be a multiple of block stride.
		/// * padding: Padding
		/// * locations: Vector of Point
		/// 
		/// ## Note
		/// This alternative version of [compute] function uses the following default values for its arguments:
		/// * win_stride: Size()
		/// * padding: Size()
		/// * locations: std::vector<Point>()
		#[inline]
		fn compute_def(&self, img: &impl core::ToInputArray, descriptors: &mut core::Vector<f32>) -> Result<()> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_compute_const_const__InputArrayR_vectorLfloatGR(self.as_raw_HOGDescriptor(), img.as_raw__InputArray(), descriptors.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Performs object detection without a multi-scale window.
		/// ## Parameters
		/// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
		/// * foundLocations: Vector of point where each point contains left-top corner point of detected object boundaries.
		/// * weights: Vector that will contain confidence values for each detected object.
		/// * hitThreshold: Threshold for the distance between features and SVM classifying plane.
		/// Usually it is 0 and should be specified in the detector coefficients (as the last free coefficient).
		/// But if the free coefficient is omitted (which is allowed), you can specify it manually here.
		/// * winStride: Window stride. It must be a multiple of block stride.
		/// * padding: Padding
		/// * searchLocations: Vector of Point includes set of requested locations to be evaluated.
		/// 
		/// ## C++ default parameters
		/// * hit_threshold: 0
		/// * win_stride: Size()
		/// * padding: Size()
		/// * search_locations: std::vector<Point>()
		#[inline]
		fn detect_weights(&self, img: &impl core::ToInputArray, found_locations: &mut core::Vector<core::Point>, weights: &mut core::Vector<f64>, hit_threshold: f64, win_stride: core::Size, padding: core::Size, search_locations: &core::Vector<core::Point>) -> Result<()> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_detect_const_const__InputArrayR_vectorLPointGR_vectorLdoubleGR_double_Size_Size_const_vectorLPointGR(self.as_raw_HOGDescriptor(), img.as_raw__InputArray(), found_locations.as_raw_mut_VectorOfPoint(), weights.as_raw_mut_VectorOff64(), hit_threshold, win_stride.opencv_as_extern(), padding.opencv_as_extern(), search_locations.as_raw_VectorOfPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Performs object detection without a multi-scale window.
		/// ## Parameters
		/// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
		/// * foundLocations: Vector of point where each point contains left-top corner point of detected object boundaries.
		/// * weights: Vector that will contain confidence values for each detected object.
		/// * hitThreshold: Threshold for the distance between features and SVM classifying plane.
		/// Usually it is 0 and should be specified in the detector coefficients (as the last free coefficient).
		/// But if the free coefficient is omitted (which is allowed), you can specify it manually here.
		/// * winStride: Window stride. It must be a multiple of block stride.
		/// * padding: Padding
		/// * searchLocations: Vector of Point includes set of requested locations to be evaluated.
		/// 
		/// ## Note
		/// This alternative version of [detect_weights] function uses the following default values for its arguments:
		/// * hit_threshold: 0
		/// * win_stride: Size()
		/// * padding: Size()
		/// * search_locations: std::vector<Point>()
		#[inline]
		fn detect_weights_def(&self, img: &impl core::ToInputArray, found_locations: &mut core::Vector<core::Point>, weights: &mut core::Vector<f64>) -> Result<()> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_detect_const_const__InputArrayR_vectorLPointGR_vectorLdoubleGR(self.as_raw_HOGDescriptor(), img.as_raw__InputArray(), found_locations.as_raw_mut_VectorOfPoint(), weights.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Performs object detection without a multi-scale window.
		/// ## Parameters
		/// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
		/// * foundLocations: Vector of point where each point contains left-top corner point of detected object boundaries.
		/// * hitThreshold: Threshold for the distance between features and SVM classifying plane.
		/// Usually it is 0 and should be specified in the detector coefficients (as the last free coefficient).
		/// But if the free coefficient is omitted (which is allowed), you can specify it manually here.
		/// * winStride: Window stride. It must be a multiple of block stride.
		/// * padding: Padding
		/// * searchLocations: Vector of Point includes locations to search.
		/// 
		/// ## C++ default parameters
		/// * hit_threshold: 0
		/// * win_stride: Size()
		/// * padding: Size()
		/// * search_locations: std::vector<Point>()
		#[inline]
		fn detect(&self, img: &impl core::ToInputArray, found_locations: &mut core::Vector<core::Point>, hit_threshold: f64, win_stride: core::Size, padding: core::Size, search_locations: &core::Vector<core::Point>) -> Result<()> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_detect_const_const__InputArrayR_vectorLPointGR_double_Size_Size_const_vectorLPointGR(self.as_raw_HOGDescriptor(), img.as_raw__InputArray(), found_locations.as_raw_mut_VectorOfPoint(), hit_threshold, win_stride.opencv_as_extern(), padding.opencv_as_extern(), search_locations.as_raw_VectorOfPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Performs object detection without a multi-scale window.
		/// ## Parameters
		/// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
		/// * foundLocations: Vector of point where each point contains left-top corner point of detected object boundaries.
		/// * hitThreshold: Threshold for the distance between features and SVM classifying plane.
		/// Usually it is 0 and should be specified in the detector coefficients (as the last free coefficient).
		/// But if the free coefficient is omitted (which is allowed), you can specify it manually here.
		/// * winStride: Window stride. It must be a multiple of block stride.
		/// * padding: Padding
		/// * searchLocations: Vector of Point includes locations to search.
		/// 
		/// ## Note
		/// This alternative version of [detect] function uses the following default values for its arguments:
		/// * hit_threshold: 0
		/// * win_stride: Size()
		/// * padding: Size()
		/// * search_locations: std::vector<Point>()
		#[inline]
		fn detect_def(&self, img: &impl core::ToInputArray, found_locations: &mut core::Vector<core::Point>) -> Result<()> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_detect_const_const__InputArrayR_vectorLPointGR(self.as_raw_HOGDescriptor(), img.as_raw__InputArray(), found_locations.as_raw_mut_VectorOfPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Detects objects of different sizes in the input image. The detected objects are returned as a list
		/// of rectangles.
		/// ## Parameters
		/// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
		/// * foundLocations: Vector of rectangles where each rectangle contains the detected object.
		/// * foundWeights: Vector that will contain confidence values for each detected object.
		/// * hitThreshold: Threshold for the distance between features and SVM classifying plane.
		/// Usually it is 0 and should be specified in the detector coefficients (as the last free coefficient).
		/// But if the free coefficient is omitted (which is allowed), you can specify it manually here.
		/// * winStride: Window stride. It must be a multiple of block stride.
		/// * padding: Padding
		/// * scale: Coefficient of the detection window increase.
		/// * groupThreshold: Coefficient to regulate the similarity threshold. When detected, some objects can be covered
		/// by many rectangles. 0 means not to perform grouping.
		/// * useMeanshiftGrouping: indicates grouping algorithm
		/// 
		/// ## C++ default parameters
		/// * hit_threshold: 0
		/// * win_stride: Size()
		/// * padding: Size()
		/// * scale: 1.05
		/// * group_threshold: 2.0
		/// * use_meanshift_grouping: false
		#[inline]
		fn detect_multi_scale_weights(&self, img: &impl core::ToInputArray, found_locations: &mut core::Vector<core::Rect>, found_weights: &mut core::Vector<f64>, hit_threshold: f64, win_stride: core::Size, padding: core::Size, scale: f64, group_threshold: f64, use_meanshift_grouping: bool) -> Result<()> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vectorLRectGR_vectorLdoubleGR_double_Size_Size_double_double_bool(self.as_raw_HOGDescriptor(), img.as_raw__InputArray(), found_locations.as_raw_mut_VectorOfRect(), found_weights.as_raw_mut_VectorOff64(), hit_threshold, win_stride.opencv_as_extern(), padding.opencv_as_extern(), scale, group_threshold, use_meanshift_grouping, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Detects objects of different sizes in the input image. The detected objects are returned as a list
		/// of rectangles.
		/// ## Parameters
		/// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
		/// * foundLocations: Vector of rectangles where each rectangle contains the detected object.
		/// * foundWeights: Vector that will contain confidence values for each detected object.
		/// * hitThreshold: Threshold for the distance between features and SVM classifying plane.
		/// Usually it is 0 and should be specified in the detector coefficients (as the last free coefficient).
		/// But if the free coefficient is omitted (which is allowed), you can specify it manually here.
		/// * winStride: Window stride. It must be a multiple of block stride.
		/// * padding: Padding
		/// * scale: Coefficient of the detection window increase.
		/// * groupThreshold: Coefficient to regulate the similarity threshold. When detected, some objects can be covered
		/// by many rectangles. 0 means not to perform grouping.
		/// * useMeanshiftGrouping: indicates grouping algorithm
		/// 
		/// ## Note
		/// This alternative version of [detect_multi_scale_weights] function uses the following default values for its arguments:
		/// * hit_threshold: 0
		/// * win_stride: Size()
		/// * padding: Size()
		/// * scale: 1.05
		/// * group_threshold: 2.0
		/// * use_meanshift_grouping: false
		#[inline]
		fn detect_multi_scale_weights_def(&self, img: &impl core::ToInputArray, found_locations: &mut core::Vector<core::Rect>, found_weights: &mut core::Vector<f64>) -> Result<()> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vectorLRectGR_vectorLdoubleGR(self.as_raw_HOGDescriptor(), img.as_raw__InputArray(), found_locations.as_raw_mut_VectorOfRect(), found_weights.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Detects objects of different sizes in the input image. The detected objects are returned as a list
		/// of rectangles.
		/// ## Parameters
		/// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
		/// * foundLocations: Vector of rectangles where each rectangle contains the detected object.
		/// * hitThreshold: Threshold for the distance between features and SVM classifying plane.
		/// Usually it is 0 and should be specified in the detector coefficients (as the last free coefficient).
		/// But if the free coefficient is omitted (which is allowed), you can specify it manually here.
		/// * winStride: Window stride. It must be a multiple of block stride.
		/// * padding: Padding
		/// * scale: Coefficient of the detection window increase.
		/// * groupThreshold: Coefficient to regulate the similarity threshold. When detected, some objects can be covered
		/// by many rectangles. 0 means not to perform grouping.
		/// * useMeanshiftGrouping: indicates grouping algorithm
		/// 
		/// ## C++ default parameters
		/// * hit_threshold: 0
		/// * win_stride: Size()
		/// * padding: Size()
		/// * scale: 1.05
		/// * group_threshold: 2.0
		/// * use_meanshift_grouping: false
		#[inline]
		fn detect_multi_scale(&self, img: &impl core::ToInputArray, found_locations: &mut core::Vector<core::Rect>, hit_threshold: f64, win_stride: core::Size, padding: core::Size, scale: f64, group_threshold: f64, use_meanshift_grouping: bool) -> Result<()> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vectorLRectGR_double_Size_Size_double_double_bool(self.as_raw_HOGDescriptor(), img.as_raw__InputArray(), found_locations.as_raw_mut_VectorOfRect(), hit_threshold, win_stride.opencv_as_extern(), padding.opencv_as_extern(), scale, group_threshold, use_meanshift_grouping, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Detects objects of different sizes in the input image. The detected objects are returned as a list
		/// of rectangles.
		/// ## Parameters
		/// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
		/// * foundLocations: Vector of rectangles where each rectangle contains the detected object.
		/// * hitThreshold: Threshold for the distance between features and SVM classifying plane.
		/// Usually it is 0 and should be specified in the detector coefficients (as the last free coefficient).
		/// But if the free coefficient is omitted (which is allowed), you can specify it manually here.
		/// * winStride: Window stride. It must be a multiple of block stride.
		/// * padding: Padding
		/// * scale: Coefficient of the detection window increase.
		/// * groupThreshold: Coefficient to regulate the similarity threshold. When detected, some objects can be covered
		/// by many rectangles. 0 means not to perform grouping.
		/// * useMeanshiftGrouping: indicates grouping algorithm
		/// 
		/// ## Note
		/// This alternative version of [detect_multi_scale] function uses the following default values for its arguments:
		/// * hit_threshold: 0
		/// * win_stride: Size()
		/// * padding: Size()
		/// * scale: 1.05
		/// * group_threshold: 2.0
		/// * use_meanshift_grouping: false
		#[inline]
		fn detect_multi_scale_def(&self, img: &impl core::ToInputArray, found_locations: &mut core::Vector<core::Rect>) -> Result<()> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vectorLRectGR(self.as_raw_HOGDescriptor(), img.as_raw__InputArray(), found_locations.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Computes gradients and quantized gradient orientations.
		/// ## Parameters
		/// * img: Matrix contains the image to be computed
		/// * grad: Matrix of type CV_32FC2 contains computed gradients
		/// * angleOfs: Matrix of type CV_8UC2 contains quantized gradient orientations
		/// * paddingTL: Padding from top-left
		/// * paddingBR: Padding from bottom-right
		/// 
		/// ## C++ default parameters
		/// * padding_tl: Size()
		/// * padding_br: Size()
		#[inline]
		fn compute_gradient(&self, img: &impl core::ToInputArray, grad: &mut impl core::ToInputOutputArray, angle_ofs: &mut impl core::ToInputOutputArray, padding_tl: core::Size, padding_br: core::Size) -> Result<()> {
			input_array_arg!(img);
			input_output_array_arg!(grad);
			input_output_array_arg!(angle_ofs);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_computeGradient_const_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_Size(self.as_raw_HOGDescriptor(), img.as_raw__InputArray(), grad.as_raw__InputOutputArray(), angle_ofs.as_raw__InputOutputArray(), padding_tl.opencv_as_extern(), padding_br.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Computes gradients and quantized gradient orientations.
		/// ## Parameters
		/// * img: Matrix contains the image to be computed
		/// * grad: Matrix of type CV_32FC2 contains computed gradients
		/// * angleOfs: Matrix of type CV_8UC2 contains quantized gradient orientations
		/// * paddingTL: Padding from top-left
		/// * paddingBR: Padding from bottom-right
		/// 
		/// ## Note
		/// This alternative version of [compute_gradient] function uses the following default values for its arguments:
		/// * padding_tl: Size()
		/// * padding_br: Size()
		#[inline]
		fn compute_gradient_def(&self, img: &impl core::ToInputArray, grad: &mut impl core::ToInputOutputArray, angle_ofs: &mut impl core::ToInputOutputArray) -> Result<()> {
			input_array_arg!(img);
			input_output_array_arg!(grad);
			input_output_array_arg!(angle_ofs);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_computeGradient_const_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(self.as_raw_HOGDescriptor(), img.as_raw__InputArray(), grad.as_raw__InputOutputArray(), angle_ofs.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// evaluate specified ROI and return confidence value for each location
		/// ## Parameters
		/// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
		/// * locations: Vector of Point
		/// * foundLocations: Vector of Point where each Point is detected object's top-left point.
		/// * confidences: confidences
		/// * hitThreshold: Threshold for the distance between features and SVM classifying plane. Usually
		/// it is 0 and should be specified in the detector coefficients (as the last free coefficient). But if
		/// the free coefficient is omitted (which is allowed), you can specify it manually here
		/// * winStride: winStride
		/// * padding: padding
		/// 
		/// ## C++ default parameters
		/// * hit_threshold: 0
		/// * win_stride: Size()
		/// * padding: Size()
		#[inline]
		fn detect_roi(&self, img: &impl core::ToInputArray, locations: &core::Vector<core::Point>, found_locations: &mut core::Vector<core::Point>, confidences: &mut core::Vector<f64>, hit_threshold: f64, win_stride: core::Size, padding: core::Size) -> Result<()> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_detectROI_const_const__InputArrayR_const_vectorLPointGR_vectorLPointGR_vectorLdoubleGR_double_Size_Size(self.as_raw_HOGDescriptor(), img.as_raw__InputArray(), locations.as_raw_VectorOfPoint(), found_locations.as_raw_mut_VectorOfPoint(), confidences.as_raw_mut_VectorOff64(), hit_threshold, win_stride.opencv_as_extern(), padding.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// evaluate specified ROI and return confidence value for each location
		/// ## Parameters
		/// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
		/// * locations: Vector of Point
		/// * foundLocations: Vector of Point where each Point is detected object's top-left point.
		/// * confidences: confidences
		/// * hitThreshold: Threshold for the distance between features and SVM classifying plane. Usually
		/// it is 0 and should be specified in the detector coefficients (as the last free coefficient). But if
		/// the free coefficient is omitted (which is allowed), you can specify it manually here
		/// * winStride: winStride
		/// * padding: padding
		/// 
		/// ## Note
		/// This alternative version of [detect_roi] function uses the following default values for its arguments:
		/// * hit_threshold: 0
		/// * win_stride: Size()
		/// * padding: Size()
		#[inline]
		fn detect_roi_def(&self, img: &impl core::ToInputArray, locations: &core::Vector<core::Point>, found_locations: &mut core::Vector<core::Point>, confidences: &mut core::Vector<f64>) -> Result<()> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_detectROI_const_const__InputArrayR_const_vectorLPointGR_vectorLPointGR_vectorLdoubleGR(self.as_raw_HOGDescriptor(), img.as_raw__InputArray(), locations.as_raw_VectorOfPoint(), found_locations.as_raw_mut_VectorOfPoint(), confidences.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// evaluate specified ROI and return confidence value for each location in multiple scales
		/// ## Parameters
		/// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
		/// * foundLocations: Vector of rectangles where each rectangle contains the detected object.
		/// * locations: Vector of DetectionROI
		/// * hitThreshold: Threshold for the distance between features and SVM classifying plane. Usually it is 0 and should be specified
		/// in the detector coefficients (as the last free coefficient). But if the free coefficient is omitted (which is allowed), you can specify it manually here.
		/// * groupThreshold: Minimum possible number of rectangles minus 1. The threshold is used in a group of rectangles to retain it.
		/// 
		/// ## C++ default parameters
		/// * hit_threshold: 0
		/// * group_threshold: 0
		#[inline]
		fn detect_multi_scale_roi(&self, img: &impl core::ToInputArray, found_locations: &mut core::Vector<core::Rect>, locations: &mut core::Vector<crate::objdetect::DetectionROI>, hit_threshold: f64, group_threshold: i32) -> Result<()> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_detectMultiScaleROI_const_const__InputArrayR_vectorLRectGR_vectorLDetectionROIGR_double_int(self.as_raw_HOGDescriptor(), img.as_raw__InputArray(), found_locations.as_raw_mut_VectorOfRect(), locations.as_raw_mut_VectorOfDetectionROI(), hit_threshold, group_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// evaluate specified ROI and return confidence value for each location in multiple scales
		/// ## Parameters
		/// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
		/// * foundLocations: Vector of rectangles where each rectangle contains the detected object.
		/// * locations: Vector of DetectionROI
		/// * hitThreshold: Threshold for the distance between features and SVM classifying plane. Usually it is 0 and should be specified
		/// in the detector coefficients (as the last free coefficient). But if the free coefficient is omitted (which is allowed), you can specify it manually here.
		/// * groupThreshold: Minimum possible number of rectangles minus 1. The threshold is used in a group of rectangles to retain it.
		/// 
		/// ## Note
		/// This alternative version of [detect_multi_scale_roi] function uses the following default values for its arguments:
		/// * hit_threshold: 0
		/// * group_threshold: 0
		#[inline]
		fn detect_multi_scale_roi_def(&self, img: &impl core::ToInputArray, found_locations: &mut core::Vector<core::Rect>, locations: &mut core::Vector<crate::objdetect::DetectionROI>) -> Result<()> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_detectMultiScaleROI_const_const__InputArrayR_vectorLRectGR_vectorLDetectionROIGR(self.as_raw_HOGDescriptor(), img.as_raw__InputArray(), found_locations.as_raw_mut_VectorOfRect(), locations.as_raw_mut_VectorOfDetectionROI(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Groups the object candidate rectangles.
		/// ## Parameters
		/// * rectList: Input/output vector of rectangles. Output vector includes retained and grouped rectangles. (The Python list is not modified in place.)
		/// * weights: Input/output vector of weights of rectangles. Output vector includes weights of retained and grouped rectangles. (The Python list is not modified in place.)
		/// * groupThreshold: Minimum possible number of rectangles minus 1. The threshold is used in a group of rectangles to retain it.
		/// * eps: Relative difference between sides of the rectangles to merge them into a group.
		#[inline]
		fn group_rectangles(&self, rect_list: &mut core::Vector<core::Rect>, weights: &mut core::Vector<f64>, group_threshold: i32, eps: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_groupRectangles_const_vectorLRectGR_vectorLdoubleGR_int_double(self.as_raw_HOGDescriptor(), rect_list.as_raw_mut_VectorOfRect(), weights.as_raw_mut_VectorOff64(), group_threshold, eps, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::objdetect::HOGDescriptor]
	pub trait HOGDescriptorTrait: crate::objdetect::HOGDescriptorTraitConst {
		fn as_raw_mut_HOGDescriptor(&mut self) -> *mut c_void;
	
		/// Detection window size. Align to block size and block stride. Default value is Size(64,128).
		#[inline]
		fn set_win_size(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_HOGDescriptor_propWinSize_Size(self.as_raw_mut_HOGDescriptor(), val.opencv_as_extern()) };
			ret
		}
		
		/// Block size in pixels. Align to cell size. Default value is Size(16,16).
		#[inline]
		fn set_block_size(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_HOGDescriptor_propBlockSize_Size(self.as_raw_mut_HOGDescriptor(), val.opencv_as_extern()) };
			ret
		}
		
		/// Block stride. It must be a multiple of cell size. Default value is Size(8,8).
		#[inline]
		fn set_block_stride(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_HOGDescriptor_propBlockStride_Size(self.as_raw_mut_HOGDescriptor(), val.opencv_as_extern()) };
			ret
		}
		
		/// Cell size. Default value is Size(8,8).
		#[inline]
		fn set_cell_size(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_HOGDescriptor_propCellSize_Size(self.as_raw_mut_HOGDescriptor(), val.opencv_as_extern()) };
			ret
		}
		
		/// Number of bins used in the calculation of histogram of gradients. Default value is 9.
		#[inline]
		fn set_nbins(&mut self, val: i32) {
			let ret = unsafe { sys::cv_HOGDescriptor_propNbins_int(self.as_raw_mut_HOGDescriptor(), val) };
			ret
		}
		
		/// not documented
		#[inline]
		fn set_deriv_aperture(&mut self, val: i32) {
			let ret = unsafe { sys::cv_HOGDescriptor_propDerivAperture_int(self.as_raw_mut_HOGDescriptor(), val) };
			ret
		}
		
		/// Gaussian smoothing window parameter.
		#[inline]
		fn set_win_sigma(&mut self, val: f64) {
			let ret = unsafe { sys::cv_HOGDescriptor_propWinSigma_double(self.as_raw_mut_HOGDescriptor(), val) };
			ret
		}
		
		/// histogramNormType
		#[inline]
		fn set_histogram_norm_type(&mut self, val: crate::objdetect::HOGDescriptor_HistogramNormType) {
			let ret = unsafe { sys::cv_HOGDescriptor_propHistogramNormType_HistogramNormType(self.as_raw_mut_HOGDescriptor(), val) };
			ret
		}
		
		/// L2-Hys normalization method shrinkage.
		#[inline]
		fn set_l2_hys_threshold(&mut self, val: f64) {
			let ret = unsafe { sys::cv_HOGDescriptor_propL2HysThreshold_double(self.as_raw_mut_HOGDescriptor(), val) };
			ret
		}
		
		/// Flag to specify whether the gamma correction preprocessing is required or not.
		#[inline]
		fn set_gamma_correction(&mut self, val: bool) {
			let ret = unsafe { sys::cv_HOGDescriptor_propGammaCorrection_bool(self.as_raw_mut_HOGDescriptor(), val) };
			ret
		}
		
		/// coefficients for the linear SVM classifier.
		#[inline]
		fn set_svm_detector_vec(&mut self, mut val: core::Vector<f32>) {
			let ret = unsafe { sys::cv_HOGDescriptor_propSvmDetector_vectorLfloatG(self.as_raw_mut_HOGDescriptor(), val.as_raw_mut_VectorOff32()) };
			ret
		}
		
		/// coefficients for the linear SVM classifier used when OpenCL is enabled
		#[inline]
		fn set_ocl_svm_detector(&mut self, mut val: core::UMat) {
			let ret = unsafe { sys::cv_HOGDescriptor_propOclSvmDetector_UMat(self.as_raw_mut_HOGDescriptor(), val.as_raw_mut_UMat()) };
			ret
		}
		
		/// not documented
		#[inline]
		fn set_free_coef(&mut self, val: f32) {
			let ret = unsafe { sys::cv_HOGDescriptor_propFree_coef_float(self.as_raw_mut_HOGDescriptor(), val) };
			ret
		}
		
		/// Maximum number of detection window increases. Default value is 64
		#[inline]
		fn set_nlevels(&mut self, val: i32) {
			let ret = unsafe { sys::cv_HOGDescriptor_propNlevels_int(self.as_raw_mut_HOGDescriptor(), val) };
			ret
		}
		
		/// Indicates signed gradient will be used or not
		#[inline]
		fn set_signed_gradient(&mut self, val: bool) {
			let ret = unsafe { sys::cv_HOGDescriptor_propSignedGradient_bool(self.as_raw_mut_HOGDescriptor(), val) };
			ret
		}
		
		/// @example samples/cpp/peopledetect.cpp
		/// /
		/// Sets coefficients for the linear SVM classifier.
		/// ## Parameters
		/// * svmdetector: coefficients for the linear SVM classifier.
		#[inline]
		fn set_svm_detector(&mut self, svmdetector: &impl core::ToInputArray) -> Result<()> {
			input_array_arg!(svmdetector);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_setSVMDetector_const__InputArrayR(self.as_raw_mut_HOGDescriptor(), svmdetector.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Reads HOGDescriptor parameters and coefficients for the linear SVM classifier from a file node.
		/// ## Parameters
		/// * fn: File node
		#[inline]
		fn read(&mut self, fn_: &mut core::FileNode) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_read_FileNodeR(self.as_raw_mut_HOGDescriptor(), fn_.as_raw_mut_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// loads HOGDescriptor parameters and coefficients for the linear SVM classifier from a file
		/// ## Parameters
		/// * filename: Name of the file to read.
		/// * objname: The optional name of the node to read (if empty, the first top-level node will be used).
		/// 
		/// ## C++ default parameters
		/// * objname: String()
		#[inline]
		fn load(&mut self, filename: &str, objname: &str) -> Result<bool> {
			extern_container_arg!(filename);
			extern_container_arg!(objname);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_load_const_StringR_const_StringR(self.as_raw_mut_HOGDescriptor(), filename.opencv_as_extern(), objname.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// loads HOGDescriptor parameters and coefficients for the linear SVM classifier from a file
		/// ## Parameters
		/// * filename: Name of the file to read.
		/// * objname: The optional name of the node to read (if empty, the first top-level node will be used).
		/// 
		/// ## Note
		/// This alternative version of [load] function uses the following default values for its arguments:
		/// * objname: String()
		#[inline]
		fn load_def(&mut self, filename: &str) -> Result<bool> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_load_const_StringR(self.as_raw_mut_HOGDescriptor(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Implementation of HOG (Histogram of Oriented Gradients) descriptor and object detector.
	/// 
	/// the HOG descriptor algorithm introduced by Navneet Dalal and Bill Triggs [Dalal2005](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Dalal2005) .
	/// 
	/// useful links:
	/// 
	/// <https://hal.inria.fr/inria-00548512/document/>
	/// 
	/// <https://en.wikipedia.org/wiki/Histogram_of_oriented_gradients>
	/// 
	/// <https://software.intel.com/en-us/ipp-dev-reference-histogram-of-oriented-gradients-hog-descriptor>
	/// 
	/// <http://www.learnopencv.com/histogram-of-oriented-gradients>
	/// 
	/// <http://www.learnopencv.com/handwritten-digits-classification-an-opencv-c-python-tutorial>
	pub struct HOGDescriptor {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { HOGDescriptor }
	
	impl Drop for HOGDescriptor {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_HOGDescriptor_delete(self.as_raw_mut_HOGDescriptor()) };
		}
	}
	
	unsafe impl Send for HOGDescriptor {}
	
	impl crate::objdetect::HOGDescriptorTraitConst for HOGDescriptor {
		#[inline] fn as_raw_HOGDescriptor(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::HOGDescriptorTrait for HOGDescriptor {
		#[inline] fn as_raw_mut_HOGDescriptor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl HOGDescriptor {
		/// Creates the HOG descriptor and detector with default parameters.
		/// 
		/// aqual to HOGDescriptor(Size(64,128), Size(16,16), Size(8,8), Size(8,8), 9 )
		#[inline]
		pub fn default() -> Result<crate::objdetect::HOGDescriptor> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_HOGDescriptor(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::HOGDescriptor::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Creates the HOG descriptor and detector with default parameters.
		/// 
		/// aqual to HOGDescriptor(Size(64,128), Size(16,16), Size(8,8), Size(8,8), 9 )
		/// 
		/// ## Overloaded parameters
		/// 
		/// ## Parameters
		/// * _winSize: sets winSize with given value.
		/// * _blockSize: sets blockSize with given value.
		/// * _blockStride: sets blockStride with given value.
		/// * _cellSize: sets cellSize with given value.
		/// * _nbins: sets nbins with given value.
		/// * _derivAperture: sets derivAperture with given value.
		/// * _winSigma: sets winSigma with given value.
		/// * _histogramNormType: sets histogramNormType with given value.
		/// * _L2HysThreshold: sets L2HysThreshold with given value.
		/// * _gammaCorrection: sets gammaCorrection with given value.
		/// * _nlevels: sets nlevels with given value.
		/// * _signedGradient: sets signedGradient with given value.
		/// 
		/// ## C++ default parameters
		/// * _deriv_aperture: 1
		/// * _win_sigma: -1
		/// * _histogram_norm_type: HOGDescriptor::L2Hys
		/// * _l2_hys_threshold: 0.2
		/// * _gamma_correction: false
		/// * _nlevels: HOGDescriptor::DEFAULT_NLEVELS
		/// * _signed_gradient: false
		#[inline]
		pub fn new(_win_size: core::Size, _block_size: core::Size, _block_stride: core::Size, _cell_size: core::Size, _nbins: i32, _deriv_aperture: i32, _win_sigma: f64, _histogram_norm_type: crate::objdetect::HOGDescriptor_HistogramNormType, _l2_hys_threshold: f64, _gamma_correction: bool, _nlevels: i32, _signed_gradient: bool) -> Result<crate::objdetect::HOGDescriptor> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_HOGDescriptor_Size_Size_Size_Size_int_int_double_HistogramNormType_double_bool_int_bool(_win_size.opencv_as_extern(), _block_size.opencv_as_extern(), _block_stride.opencv_as_extern(), _cell_size.opencv_as_extern(), _nbins, _deriv_aperture, _win_sigma, _histogram_norm_type, _l2_hys_threshold, _gamma_correction, _nlevels, _signed_gradient, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::HOGDescriptor::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// @overload
		/// ## Parameters
		/// * _winSize: sets winSize with given value.
		/// * _blockSize: sets blockSize with given value.
		/// * _blockStride: sets blockStride with given value.
		/// * _cellSize: sets cellSize with given value.
		/// * _nbins: sets nbins with given value.
		/// * _derivAperture: sets derivAperture with given value.
		/// * _winSigma: sets winSigma with given value.
		/// * _histogramNormType: sets histogramNormType with given value.
		/// * _L2HysThreshold: sets L2HysThreshold with given value.
		/// * _gammaCorrection: sets gammaCorrection with given value.
		/// * _nlevels: sets nlevels with given value.
		/// * _signedGradient: sets signedGradient with given value.
		/// 
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * _deriv_aperture: 1
		/// * _win_sigma: -1
		/// * _histogram_norm_type: HOGDescriptor::L2Hys
		/// * _l2_hys_threshold: 0.2
		/// * _gamma_correction: false
		/// * _nlevels: HOGDescriptor::DEFAULT_NLEVELS
		/// * _signed_gradient: false
		#[inline]
		pub fn new_def(_win_size: core::Size, _block_size: core::Size, _block_stride: core::Size, _cell_size: core::Size, _nbins: i32) -> Result<crate::objdetect::HOGDescriptor> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_HOGDescriptor_Size_Size_Size_Size_int(_win_size.opencv_as_extern(), _block_size.opencv_as_extern(), _block_stride.opencv_as_extern(), _cell_size.opencv_as_extern(), _nbins, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::HOGDescriptor::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Creates the HOG descriptor and detector with default parameters.
		/// 
		/// aqual to HOGDescriptor(Size(64,128), Size(16,16), Size(8,8), Size(8,8), 9 )
		/// 
		/// ## Overloaded parameters
		/// 
		/// 
		///    Creates the HOG descriptor and detector and loads HOGDescriptor parameters and coefficients for the linear SVM classifier from a file.
		/// ## Parameters
		/// * filename: The file name containing HOGDescriptor properties and coefficients for the linear SVM classifier.
		#[inline]
		pub fn new_from_file(filename: &str) -> Result<crate::objdetect::HOGDescriptor> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_HOGDescriptor_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::HOGDescriptor::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Creates the HOG descriptor and detector with default parameters.
		/// 
		/// aqual to HOGDescriptor(Size(64,128), Size(16,16), Size(8,8), Size(8,8), 9 )
		/// 
		/// ## Overloaded parameters
		/// 
		/// ## Parameters
		/// * d: the HOGDescriptor which cloned to create a new one.
		#[inline]
		pub fn copy(d: &crate::objdetect::HOGDescriptor) -> Result<crate::objdetect::HOGDescriptor> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_HOGDescriptor_const_HOGDescriptorR(d.as_raw_HOGDescriptor(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::HOGDescriptor::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Returns coefficients of the classifier trained for people detection (for 64x128 windows).
		#[inline]
		pub fn get_default_people_detector() -> Result<core::Vector<f32>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_getDefaultPeopleDetector(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<f32>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// @example samples/tapi/hog.cpp
		/// /
		/// Returns coefficients of the classifier trained for people detection (for 48x96 windows).
		#[inline]
		pub fn get_daimler_people_detector() -> Result<core::Vector<f32>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_HOGDescriptor_getDaimlerPeopleDetector(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<f32>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl std::fmt::Debug for HOGDescriptor {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("HOGDescriptor")
				.field("win_size", &crate::objdetect::HOGDescriptorTraitConst::win_size(self))
				.field("block_size", &crate::objdetect::HOGDescriptorTraitConst::block_size(self))
				.field("block_stride", &crate::objdetect::HOGDescriptorTraitConst::block_stride(self))
				.field("cell_size", &crate::objdetect::HOGDescriptorTraitConst::cell_size(self))
				.field("nbins", &crate::objdetect::HOGDescriptorTraitConst::nbins(self))
				.field("deriv_aperture", &crate::objdetect::HOGDescriptorTraitConst::deriv_aperture(self))
				.field("win_sigma", &crate::objdetect::HOGDescriptorTraitConst::win_sigma(self))
				.field("histogram_norm_type", &crate::objdetect::HOGDescriptorTraitConst::histogram_norm_type(self))
				.field("l2_hys_threshold", &crate::objdetect::HOGDescriptorTraitConst::l2_hys_threshold(self))
				.field("gamma_correction", &crate::objdetect::HOGDescriptorTraitConst::gamma_correction(self))
				.field("svm_detector", &crate::objdetect::HOGDescriptorTraitConst::svm_detector(self))
				.field("ocl_svm_detector", &crate::objdetect::HOGDescriptorTraitConst::ocl_svm_detector(self))
				.field("free_coef", &crate::objdetect::HOGDescriptorTraitConst::free_coef(self))
				.field("nlevels", &crate::objdetect::HOGDescriptorTraitConst::nlevels(self))
				.field("signed_gradient", &crate::objdetect::HOGDescriptorTraitConst::signed_gradient(self))
				.finish()
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
			return_receive!(unsafe ocvrs_return => ret);
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
			return_receive!(unsafe ocvrs_return => ret);
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
			return_receive!(unsafe ocvrs_return => ret);
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
		fn decode_curved(&mut self, img: &impl core::ToInputArray, points: &impl core::ToInputArray, straight_qrcode: &mut impl core::ToOutputArray) -> Result<Vec<u8>> {
			input_array_arg!(img);
			input_array_arg!(points);
			output_array_arg!(straight_qrcode);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeDetector_decodeCurved_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_QRCodeDetector(), img.as_raw__InputArray(), points.as_raw__InputArray(), straight_qrcode.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
		/// This alternative version of [decode_curved] function uses the following default values for its arguments:
		/// * straight_qrcode: noArray()
		#[inline]
		fn decode_curved_def(&mut self, img: &impl core::ToInputArray, points: &impl core::ToInputArray) -> Result<Vec<u8>> {
			input_array_arg!(img);
			input_array_arg!(points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeDetector_decodeCurved_const__InputArrayR_const__InputArrayR(self.as_raw_mut_QRCodeDetector(), img.as_raw__InputArray(), points.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
		fn detect_and_decode_curved(&mut self, img: &impl core::ToInputArray, points: &mut impl core::ToOutputArray, straight_qrcode: &mut impl core::ToOutputArray) -> Result<Vec<u8>> {
			input_array_arg!(img);
			output_array_arg!(points);
			output_array_arg!(straight_qrcode);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeDetector_detectAndDecodeCurved_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_QRCodeDetector(), img.as_raw__InputArray(), points.as_raw__OutputArray(), straight_qrcode.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
		/// This alternative version of [detect_and_decode_curved] function uses the following default values for its arguments:
		/// * points: noArray()
		/// * straight_qrcode: noArray()
		#[inline]
		fn detect_and_decode_curved_def(&mut self, img: &impl core::ToInputArray) -> Result<Vec<u8>> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeDetector_detectAndDecodeCurved_const__InputArrayR(self.as_raw_mut_QRCodeDetector(), img.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { Vec::<u8>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	pub struct QRCodeDetector {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { QRCodeDetector }
	
	impl Drop for QRCodeDetector {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_QRCodeDetector_delete(self.as_raw_mut_QRCodeDetector()) };
		}
	}
	
	unsafe impl Send for QRCodeDetector {}
	
	impl crate::objdetect::GraphicalCodeDetectorTraitConst for QRCodeDetector {
		#[inline] fn as_raw_GraphicalCodeDetector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::GraphicalCodeDetectorTrait for QRCodeDetector {
		#[inline] fn as_raw_mut_GraphicalCodeDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::objdetect::QRCodeDetectorTraitConst for QRCodeDetector {
		#[inline] fn as_raw_QRCodeDetector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::QRCodeDetectorTrait for QRCodeDetector {
		#[inline] fn as_raw_mut_QRCodeDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl QRCodeDetector {
		#[inline]
		pub fn default() -> Result<crate::objdetect::QRCodeDetector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeDetector_QRCodeDetector(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::QRCodeDetector::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl Clone for QRCodeDetector {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_QRCodeDetector_implicitClone_const(self.as_raw_QRCodeDetector())) }
		}
	}
	
	boxed_cast_base! { QRCodeDetector, crate::objdetect::GraphicalCodeDetector, cv_QRCodeDetector_to_GraphicalCodeDetector }
	
	impl std::fmt::Debug for QRCodeDetector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("QRCodeDetector")
				.finish()
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
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
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
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::QRCodeDetectorAruco::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Aruco detector parameters are used to search for the finder patterns.
		#[inline]
		fn get_aruco_parameters(&mut self) -> Result<crate::objdetect::DetectorParameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeDetectorAruco_getArucoParameters(self.as_raw_mut_QRCodeDetectorAruco(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::DetectorParameters::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Aruco detector parameters are used to search for the finder patterns.
		#[inline]
		fn set_aruco_parameters(&mut self, params: &crate::objdetect::DetectorParameters) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeDetectorAruco_setArucoParameters_const_DetectorParametersR(self.as_raw_mut_QRCodeDetectorAruco(), params.as_raw_DetectorParameters(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct QRCodeDetectorAruco {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { QRCodeDetectorAruco }
	
	impl Drop for QRCodeDetectorAruco {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_QRCodeDetectorAruco_delete(self.as_raw_mut_QRCodeDetectorAruco()) };
		}
	}
	
	unsafe impl Send for QRCodeDetectorAruco {}
	
	impl crate::objdetect::GraphicalCodeDetectorTraitConst for QRCodeDetectorAruco {
		#[inline] fn as_raw_GraphicalCodeDetector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::GraphicalCodeDetectorTrait for QRCodeDetectorAruco {
		#[inline] fn as_raw_mut_GraphicalCodeDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::objdetect::QRCodeDetectorArucoTraitConst for QRCodeDetectorAruco {
		#[inline] fn as_raw_QRCodeDetectorAruco(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::QRCodeDetectorArucoTrait for QRCodeDetectorAruco {
		#[inline] fn as_raw_mut_QRCodeDetectorAruco(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl QRCodeDetectorAruco {
		#[inline]
		pub fn default() -> Result<crate::objdetect::QRCodeDetectorAruco> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeDetectorAruco_QRCodeDetectorAruco(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::QRCodeDetectorAruco::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// QR code detector constructor for Aruco-based algorithm. See cv::QRCodeDetectorAruco::Params
		#[inline]
		pub fn new(params: crate::objdetect::QRCodeDetectorAruco_Params) -> Result<crate::objdetect::QRCodeDetectorAruco> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeDetectorAruco_QRCodeDetectorAruco_const_ParamsR(&params, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::QRCodeDetectorAruco::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl Clone for QRCodeDetectorAruco {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_QRCodeDetectorAruco_implicitClone_const(self.as_raw_QRCodeDetectorAruco())) }
		}
	}
	
	boxed_cast_base! { QRCodeDetectorAruco, crate::objdetect::GraphicalCodeDetector, cv_QRCodeDetectorAruco_to_GraphicalCodeDetector }
	
	impl std::fmt::Debug for QRCodeDetectorAruco {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("QRCodeDetectorAruco")
				.finish()
		}
	}
	
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
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
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
		fn encode(&mut self, encoded_info: &str, qrcode: &mut impl core::ToOutputArray) -> Result<()> {
			extern_container_arg!(encoded_info);
			output_array_arg!(qrcode);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeEncoder_encode_const_StringR_const__OutputArrayR(self.as_raw_mut_QRCodeEncoder(), encoded_info.opencv_as_extern(), qrcode.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Generates QR code from input string in Structured Append mode. The encoded message is splitting over a number of QR codes.
		/// ## Parameters
		/// * encoded_info: Input string to encode.
		/// * qrcodes: Vector of generated QR codes.
		#[inline]
		fn encode_structured_append(&mut self, encoded_info: &str, qrcodes: &mut impl core::ToOutputArray) -> Result<()> {
			extern_container_arg!(encoded_info);
			output_array_arg!(qrcodes);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeEncoder_encodeStructuredAppend_const_StringR_const__OutputArrayR(self.as_raw_mut_QRCodeEncoder(), encoded_info.opencv_as_extern(), qrcodes.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct QRCodeEncoder {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { QRCodeEncoder }
	
	impl Drop for QRCodeEncoder {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_QRCodeEncoder_delete(self.as_raw_mut_QRCodeEncoder()) };
		}
	}
	
	unsafe impl Send for QRCodeEncoder {}
	
	impl crate::objdetect::QRCodeEncoderTraitConst for QRCodeEncoder {
		#[inline] fn as_raw_QRCodeEncoder(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::QRCodeEncoderTrait for QRCodeEncoder {
		#[inline] fn as_raw_mut_QRCodeEncoder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
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
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::objdetect::QRCodeEncoder>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Constructor
		/// ## Parameters
		/// * parameters: QR code encoder parameters QRCodeEncoder::Params
		/// 
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * parameters: QRCodeEncoder::Params()
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::objdetect::QRCodeEncoder>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeEncoder_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::objdetect::QRCodeEncoder>::opencv_from_extern(ret) };
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
	
	/// QR code encoder parameters.
	/// ## Parameters
	/// * version: The optional version of QR code (by default - maximum possible depending on
	///                the length of the string).
	/// * correction_level: The optional level of error correction (by default - the lowest).
	/// * mode: The optional encoding mode - Numeric, Alphanumeric, Byte, Kanji, ECI or Structured Append.
	/// * structure_number: The optional number of QR codes to generate in Structured Append mode.
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq)]
	pub struct QRCodeEncoder_Params {
		pub version: i32,
		pub correction_level: crate::objdetect::QRCodeEncoder_CorrectionLevel,
		pub mode: crate::objdetect::QRCodeEncoder_EncodeMode,
		pub structure_number: i32,
	}
	
	opencv_type_simple! { crate::objdetect::QRCodeEncoder_Params }
	
	impl QRCodeEncoder_Params {
		#[inline]
		pub fn default() -> Result<crate::objdetect::QRCodeEncoder_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_QRCodeEncoder_Params_Params(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Constant methods for [crate::objdetect::SimilarRects]
	pub trait SimilarRectsTraitConst {
		fn as_raw_SimilarRects(&self) -> *const c_void;
	
		#[inline]
		fn eps(&self) -> f64 {
			let ret = unsafe { sys::cv_SimilarRects_propEps_const(self.as_raw_SimilarRects()) };
			ret
		}
		
		#[inline]
		fn apply(&self, r1: core::Rect, r2: core::Rect) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SimilarRects_operator___const_const_RectR_const_RectR(self.as_raw_SimilarRects(), &r1, &r2, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::objdetect::SimilarRects]
	pub trait SimilarRectsTrait: crate::objdetect::SimilarRectsTraitConst {
		fn as_raw_mut_SimilarRects(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_eps(&mut self, val: f64) {
			let ret = unsafe { sys::cv_SimilarRects_propEps_double(self.as_raw_mut_SimilarRects(), val) };
			ret
		}
		
	}
	
	/// This class is used for grouping object candidates detected by Cascade Classifier, HOG etc.
	/// 
	/// instance of the class is to be passed to cv::partition
	pub struct SimilarRects {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { SimilarRects }
	
	impl Drop for SimilarRects {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_SimilarRects_delete(self.as_raw_mut_SimilarRects()) };
		}
	}
	
	unsafe impl Send for SimilarRects {}
	
	impl crate::objdetect::SimilarRectsTraitConst for SimilarRects {
		#[inline] fn as_raw_SimilarRects(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::SimilarRectsTrait for SimilarRects {
		#[inline] fn as_raw_mut_SimilarRects(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl SimilarRects {
		#[inline]
		pub fn new(_eps: f64) -> Result<crate::objdetect::SimilarRects> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SimilarRects_SimilarRects_double(_eps, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::SimilarRects::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl std::fmt::Debug for SimilarRects {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("SimilarRects")
				.field("eps", &crate::objdetect::SimilarRectsTraitConst::eps(self))
				.finish()
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
		/// Performs marker detection in the input image. Only markers included in the specific dictionary
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
		fn detect_markers(&self, image: &impl core::ToInputArray, corners: &mut impl core::ToOutputArray, ids: &mut impl core::ToOutputArray, rejected_img_points: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(corners);
			output_array_arg!(ids);
			output_array_arg!(rejected_img_points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_detectMarkers_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_ArucoDetector(), image.as_raw__InputArray(), corners.as_raw__OutputArray(), ids.as_raw__OutputArray(), rejected_img_points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
		/// Performs marker detection in the input image. Only markers included in the specific dictionary
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
		/// This alternative version of [detect_markers] function uses the following default values for its arguments:
		/// * rejected_img_points: noArray()
		#[inline]
		fn detect_markers_def(&self, image: &impl core::ToInputArray, corners: &mut impl core::ToOutputArray, ids: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(corners);
			output_array_arg!(ids);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_detectMarkers_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_ArucoDetector(), image.as_raw__InputArray(), corners.as_raw__OutputArray(), ids.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
		/// ## C++ default parameters
		/// * camera_matrix: noArray()
		/// * dist_coeffs: noArray()
		/// * recovered_idxs: noArray()
		#[inline]
		fn refine_detected_markers(&self, image: &impl core::ToInputArray, board: &crate::objdetect::Board, detected_corners: &mut impl core::ToInputOutputArray, detected_ids: &mut impl core::ToInputOutputArray, rejected_corners: &mut impl core::ToInputOutputArray, camera_matrix: &impl core::ToInputArray, dist_coeffs: &impl core::ToInputArray, recovered_idxs: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			input_output_array_arg!(detected_corners);
			input_output_array_arg!(detected_ids);
			input_output_array_arg!(rejected_corners);
			input_array_arg!(camera_matrix);
			input_array_arg!(dist_coeffs);
			output_array_arg!(recovered_idxs);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_refineDetectedMarkers_const_const__InputArrayR_const_BoardR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_ArucoDetector(), image.as_raw__InputArray(), board.as_raw_Board(), detected_corners.as_raw__InputOutputArray(), detected_ids.as_raw__InputOutputArray(), rejected_corners.as_raw__InputOutputArray(), camera_matrix.as_raw__InputArray(), dist_coeffs.as_raw__InputArray(), recovered_idxs.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
		/// ## Note
		/// This alternative version of [refine_detected_markers] function uses the following default values for its arguments:
		/// * camera_matrix: noArray()
		/// * dist_coeffs: noArray()
		/// * recovered_idxs: noArray()
		#[inline]
		fn refine_detected_markers_def(&self, image: &impl core::ToInputArray, board: &crate::objdetect::Board, detected_corners: &mut impl core::ToInputOutputArray, detected_ids: &mut impl core::ToInputOutputArray, rejected_corners: &mut impl core::ToInputOutputArray) -> Result<()> {
			input_array_arg!(image);
			input_output_array_arg!(detected_corners);
			input_output_array_arg!(detected_ids);
			input_output_array_arg!(rejected_corners);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_refineDetectedMarkers_const_const__InputArrayR_const_BoardR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(self.as_raw_ArucoDetector(), image.as_raw__InputArray(), board.as_raw_Board(), detected_corners.as_raw__InputOutputArray(), detected_ids.as_raw__InputOutputArray(), rejected_corners.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_dictionary(&self) -> Result<crate::objdetect::Dictionary> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_getDictionary_const(self.as_raw_ArucoDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::Dictionary::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_detector_parameters(&self) -> Result<crate::objdetect::DetectorParameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_getDetectorParameters_const(self.as_raw_ArucoDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::DetectorParameters::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_refine_parameters(&self) -> Result<crate::objdetect::RefineParameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_getRefineParameters_const(self.as_raw_ArucoDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Stores algorithm parameters in a file storage
		#[inline]
		fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_write_const_FileStorageR(self.as_raw_ArucoDetector(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::objdetect::ArucoDetector]
	pub trait ArucoDetectorTrait: core::AlgorithmTrait + crate::objdetect::ArucoDetectorTraitConst {
		fn as_raw_mut_ArucoDetector(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_dictionary(&mut self, dictionary: &crate::objdetect::Dictionary) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_setDictionary_const_DictionaryR(self.as_raw_mut_ArucoDetector(), dictionary.as_raw_Dictionary(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_detector_parameters(&mut self, detector_parameters: &crate::objdetect::DetectorParameters) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_setDetectorParameters_const_DetectorParametersR(self.as_raw_mut_ArucoDetector(), detector_parameters.as_raw_DetectorParameters(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_refine_parameters(&mut self, refine_parameters: crate::objdetect::RefineParameters) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_setRefineParameters_const_RefineParametersR(self.as_raw_mut_ArucoDetector(), &refine_parameters, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// simplified API for language bindings
		#[inline]
		fn write_1(&mut self, fs: &mut core::FileStorage, name: &str) -> Result<()> {
			extern_container_arg!(name);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_write_FileStorageR_const_StringR(self.as_raw_mut_ArucoDetector(), fs.as_raw_mut_FileStorage(), name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Reads algorithm parameters from a file storage
		#[inline]
		fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_read_const_FileNodeR(self.as_raw_mut_ArucoDetector(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { ArucoDetector }
	
	impl Drop for ArucoDetector {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_aruco_ArucoDetector_delete(self.as_raw_mut_ArucoDetector()) };
		}
	}
	
	unsafe impl Send for ArucoDetector {}
	
	impl core::AlgorithmTraitConst for ArucoDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for ArucoDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::objdetect::ArucoDetectorTraitConst for ArucoDetector {
		#[inline] fn as_raw_ArucoDetector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::ArucoDetectorTrait for ArucoDetector {
		#[inline] fn as_raw_mut_ArucoDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
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
		pub fn new(dictionary: &crate::objdetect::Dictionary, detector_params: &crate::objdetect::DetectorParameters, refine_params: crate::objdetect::RefineParameters) -> Result<crate::objdetect::ArucoDetector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_ArucoDetector_ArucoDetector_const_DictionaryR_const_DetectorParametersR_const_RefineParametersR(dictionary.as_raw_Dictionary(), detector_params.as_raw_DetectorParameters(), &refine_params, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::ArucoDetector::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { ArucoDetector, core::Algorithm, cv_aruco_ArucoDetector_to_Algorithm }
	
	impl std::fmt::Debug for ArucoDetector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ArucoDetector")
				.finish()
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
			return_receive!(unsafe ocvrs_return => ret);
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
			return_receive!(unsafe ocvrs_return => ret);
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
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// get coordinate of the bottom right corner of the board, is set when calling the function create()
		#[inline]
		fn get_right_bottom_corner(&self) -> Result<core::Point3f> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Board_getRightBottomCorner_const(self.as_raw_Board(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
		fn match_image_points(&self, detected_corners: &impl core::ToInputArray, detected_ids: &impl core::ToInputArray, obj_points: &mut impl core::ToOutputArray, img_points: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(detected_corners);
			input_array_arg!(detected_ids);
			output_array_arg!(obj_points);
			output_array_arg!(img_points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Board_matchImagePoints_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_Board(), detected_corners.as_raw__InputArray(), detected_ids.as_raw__InputArray(), obj_points.as_raw__OutputArray(), img_points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
		fn generate_image(&self, out_size: core::Size, img: &mut impl core::ToOutputArray, margin_size: i32, border_bits: i32) -> Result<()> {
			output_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Board_generateImage_const_Size_const__OutputArrayR_int_int(self.as_raw_Board(), out_size.opencv_as_extern(), img.as_raw__OutputArray(), margin_size, border_bits, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
		/// This alternative version of [generate_image] function uses the following default values for its arguments:
		/// * margin_size: 0
		/// * border_bits: 1
		#[inline]
		fn generate_image_def(&self, out_size: core::Size, img: &mut impl core::ToOutputArray) -> Result<()> {
			output_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Board_generateImage_const_Size_const__OutputArrayR(self.as_raw_Board(), out_size.opencv_as_extern(), img.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::objdetect::Board]
	pub trait BoardTrait: crate::objdetect::BoardTraitConst {
		fn as_raw_mut_Board(&mut self) -> *mut c_void;
	
	}
	
	/// Board of ArUco markers
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
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_aruco_Board_delete(self.as_raw_mut_Board()) };
		}
	}
	
	unsafe impl Send for Board {}
	
	impl crate::objdetect::BoardTraitConst for Board {
		#[inline] fn as_raw_Board(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::BoardTrait for Board {
		#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Board {
		/// Common Board constructor
		/// 
		/// ## Parameters
		/// * objPoints: array of object points of all the marker corners in the board
		/// * dictionary: the dictionary of markers employed for this board
		/// * ids: vector of the identifiers of the markers in the board
		#[inline]
		pub fn new(obj_points: &impl core::ToInputArray, dictionary: &crate::objdetect::Dictionary, ids: &impl core::ToInputArray) -> Result<crate::objdetect::Board> {
			input_array_arg!(obj_points);
			input_array_arg!(ids);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Board_Board_const__InputArrayR_const_DictionaryR_const__InputArrayR(obj_points.as_raw__InputArray(), dictionary.as_raw_Dictionary(), ids.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::Board::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn default() -> Result<crate::objdetect::Board> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Board_Board(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::Board::opencv_from_extern(ret) };
			Ok(ret)
		}
		
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
	
	/// Constant methods for [crate::objdetect::CharucoBoard]
	pub trait CharucoBoardTraitConst: crate::objdetect::BoardTraitConst {
		fn as_raw_CharucoBoard(&self) -> *const c_void;
	
		#[inline]
		fn get_legacy_pattern(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoBoard_getLegacyPattern_const(self.as_raw_CharucoBoard(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_chessboard_size(&self) -> Result<core::Size> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoBoard_getChessboardSize_const(self.as_raw_CharucoBoard(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_square_length(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoBoard_getSquareLength_const(self.as_raw_CharucoBoard(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_marker_length(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoBoard_getMarkerLength_const(self.as_raw_CharucoBoard(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// get CharucoBoard::chessboardCorners
		#[inline]
		fn get_chessboard_corners(&self) -> Result<core::Vector<core::Point3f>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoBoard_getChessboardCorners_const(self.as_raw_CharucoBoard(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Point3f>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// get CharucoBoard::nearestMarkerIdx
		#[inline]
		fn get_nearest_marker_idx(&self) -> Result<core::Vector<core::Vector<i32>>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoBoard_getNearestMarkerIdx_const(self.as_raw_CharucoBoard(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Vector<i32>>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// get CharucoBoard::nearestMarkerCorners
		#[inline]
		fn get_nearest_marker_corners(&self) -> Result<core::Vector<core::Vector<i32>>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoBoard_getNearestMarkerCorners_const(self.as_raw_CharucoBoard(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
		fn check_charuco_corners_collinear(&self, charuco_ids: &impl core::ToInputArray) -> Result<bool> {
			input_array_arg!(charuco_ids);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoBoard_checkCharucoCornersCollinear_const_const__InputArrayR(self.as_raw_CharucoBoard(), charuco_ids.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// ChArUco board is a planar chessboard where the markers are placed inside the white squares of a chessboard.
	/// 
	/// The benefits of ChArUco boards is that they provide both, ArUco markers versatility and chessboard corner precision,
	/// which is important for calibration and pose estimation. The board image can be drawn using generateImage() method.
	pub struct CharucoBoard {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CharucoBoard }
	
	impl Drop for CharucoBoard {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_aruco_CharucoBoard_delete(self.as_raw_mut_CharucoBoard()) };
		}
	}
	
	unsafe impl Send for CharucoBoard {}
	
	impl crate::objdetect::BoardTraitConst for CharucoBoard {
		#[inline] fn as_raw_Board(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::BoardTrait for CharucoBoard {
		#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::objdetect::CharucoBoardTraitConst for CharucoBoard {
		#[inline] fn as_raw_CharucoBoard(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::CharucoBoardTrait for CharucoBoard {
		#[inline] fn as_raw_mut_CharucoBoard(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
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
		pub fn new(size: core::Size, square_length: f32, marker_length: f32, dictionary: &crate::objdetect::Dictionary, ids: &impl core::ToInputArray) -> Result<crate::objdetect::CharucoBoard> {
			input_array_arg!(ids);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoBoard_CharucoBoard_const_SizeR_float_float_const_DictionaryR_const__InputArrayR(&size, square_length, marker_length, dictionary.as_raw_Dictionary(), ids.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
		pub fn new_def(size: core::Size, square_length: f32, marker_length: f32, dictionary: &crate::objdetect::Dictionary) -> Result<crate::objdetect::CharucoBoard> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoBoard_CharucoBoard_const_SizeR_float_float_const_DictionaryR(&size, square_length, marker_length, dictionary.as_raw_Dictionary(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::CharucoBoard::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn default() -> Result<crate::objdetect::CharucoBoard> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoBoard_CharucoBoard(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::CharucoBoard::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl Clone for CharucoBoard {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_aruco_CharucoBoard_implicitClone_const(self.as_raw_CharucoBoard())) }
		}
	}
	
	boxed_cast_base! { CharucoBoard, crate::objdetect::Board, cv_aruco_CharucoBoard_to_Board }
	
	impl std::fmt::Debug for CharucoBoard {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CharucoBoard")
				.finish()
		}
	}
	
	/// Constant methods for [crate::objdetect::CharucoDetector]
	pub trait CharucoDetectorTraitConst: core::AlgorithmTraitConst {
		fn as_raw_CharucoDetector(&self) -> *const c_void;
	
		#[inline]
		fn get_board(&self) -> Result<crate::objdetect::CharucoBoard> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_getBoard_const(self.as_raw_CharucoDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::CharucoBoard::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_charuco_parameters(&self) -> Result<crate::objdetect::CharucoParameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_getCharucoParameters_const(self.as_raw_CharucoDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::CharucoParameters::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_detector_parameters(&self) -> Result<crate::objdetect::DetectorParameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_getDetectorParameters_const(self.as_raw_CharucoDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::DetectorParameters::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_refine_parameters(&self) -> Result<crate::objdetect::RefineParameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_getRefineParameters_const(self.as_raw_CharucoDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// detect aruco markers and interpolate position of ChArUco board corners
		/// ## Parameters
		/// * image: input image necesary for corner refinement. Note that markers are not detected and
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
		/// ## C++ default parameters
		/// * marker_corners: noArray()
		/// * marker_ids: noArray()
		#[inline]
		fn detect_board(&self, image: &impl core::ToInputArray, charuco_corners: &mut impl core::ToOutputArray, charuco_ids: &mut impl core::ToOutputArray, marker_corners: &mut impl core::ToInputOutputArray, marker_ids: &mut impl core::ToInputOutputArray) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(charuco_corners);
			output_array_arg!(charuco_ids);
			input_output_array_arg!(marker_corners);
			input_output_array_arg!(marker_ids);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_detectBoard_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(self.as_raw_CharucoDetector(), image.as_raw__InputArray(), charuco_corners.as_raw__OutputArray(), charuco_ids.as_raw__OutputArray(), marker_corners.as_raw__InputOutputArray(), marker_ids.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// detect aruco markers and interpolate position of ChArUco board corners
		/// ## Parameters
		/// * image: input image necesary for corner refinement. Note that markers are not detected and
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
		/// ## Note
		/// This alternative version of [detect_board] function uses the following default values for its arguments:
		/// * marker_corners: noArray()
		/// * marker_ids: noArray()
		#[inline]
		fn detect_board_def(&self, image: &impl core::ToInputArray, charuco_corners: &mut impl core::ToOutputArray, charuco_ids: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(charuco_corners);
			output_array_arg!(charuco_ids);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_detectBoard_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_CharucoDetector(), image.as_raw__InputArray(), charuco_corners.as_raw__OutputArray(), charuco_ids.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
		fn detect_diamonds(&self, image: &impl core::ToInputArray, diamond_corners: &mut impl core::ToOutputArray, diamond_ids: &mut impl core::ToOutputArray, marker_corners: &mut impl core::ToInputOutputArray, marker_ids: &mut impl core::ToInputOutputArray) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(diamond_corners);
			output_array_arg!(diamond_ids);
			input_output_array_arg!(marker_corners);
			input_output_array_arg!(marker_ids);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_detectDiamonds_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(self.as_raw_CharucoDetector(), image.as_raw__InputArray(), diamond_corners.as_raw__OutputArray(), diamond_ids.as_raw__OutputArray(), marker_corners.as_raw__InputOutputArray(), marker_ids.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
		/// This alternative version of [detect_diamonds] function uses the following default values for its arguments:
		/// * marker_corners: noArray()
		/// * marker_ids: noArray()
		#[inline]
		fn detect_diamonds_def(&self, image: &impl core::ToInputArray, diamond_corners: &mut impl core::ToOutputArray, diamond_ids: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(diamond_corners);
			output_array_arg!(diamond_ids);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_detectDiamonds_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_CharucoDetector(), image.as_raw__InputArray(), diamond_corners.as_raw__OutputArray(), diamond_ids.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::objdetect::CharucoDetector]
	pub trait CharucoDetectorTrait: core::AlgorithmTrait + crate::objdetect::CharucoDetectorTraitConst {
		fn as_raw_mut_CharucoDetector(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_board(&mut self, board: &crate::objdetect::CharucoBoard) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_setBoard_const_CharucoBoardR(self.as_raw_mut_CharucoDetector(), board.as_raw_CharucoBoard(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_charuco_parameters(&mut self, charuco_parameters: &mut crate::objdetect::CharucoParameters) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_setCharucoParameters_CharucoParametersR(self.as_raw_mut_CharucoDetector(), charuco_parameters.as_raw_mut_CharucoParameters(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_detector_parameters(&mut self, detector_parameters: &crate::objdetect::DetectorParameters) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_setDetectorParameters_const_DetectorParametersR(self.as_raw_mut_CharucoDetector(), detector_parameters.as_raw_DetectorParameters(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_refine_parameters(&mut self, refine_parameters: crate::objdetect::RefineParameters) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_setRefineParameters_const_RefineParametersR(self.as_raw_mut_CharucoDetector(), &refine_parameters, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct CharucoDetector {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CharucoDetector }
	
	impl Drop for CharucoDetector {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_aruco_CharucoDetector_delete(self.as_raw_mut_CharucoDetector()) };
		}
	}
	
	unsafe impl Send for CharucoDetector {}
	
	impl core::AlgorithmTraitConst for CharucoDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for CharucoDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::objdetect::CharucoDetectorTraitConst for CharucoDetector {
		#[inline] fn as_raw_CharucoDetector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::CharucoDetectorTrait for CharucoDetector {
		#[inline] fn as_raw_mut_CharucoDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
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
		pub fn new(board: &crate::objdetect::CharucoBoard, charuco_params: &crate::objdetect::CharucoParameters, detector_params: &crate::objdetect::DetectorParameters, refine_params: crate::objdetect::RefineParameters) -> Result<crate::objdetect::CharucoDetector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_CharucoDetector_const_CharucoBoardR_const_CharucoParametersR_const_DetectorParametersR_const_RefineParametersR(board.as_raw_CharucoBoard(), charuco_params.as_raw_CharucoParameters(), detector_params.as_raw_DetectorParameters(), &refine_params, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
		pub fn new_def(board: &crate::objdetect::CharucoBoard) -> Result<crate::objdetect::CharucoDetector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoDetector_CharucoDetector_const_CharucoBoardR(board.as_raw_CharucoBoard(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::CharucoDetector::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { CharucoDetector, core::Algorithm, cv_aruco_CharucoDetector_to_Algorithm }
	
	impl std::fmt::Debug for CharucoDetector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CharucoDetector")
				.finish()
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
		
	}
	
	/// Mutable methods for [crate::objdetect::CharucoParameters]
	pub trait CharucoParametersTrait: crate::objdetect::CharucoParametersTraitConst {
		fn as_raw_mut_CharucoParameters(&mut self) -> *mut c_void;
	
		/// cameraMatrix optional 3x3 floating-point camera matrix
		#[inline]
		fn set_camera_matrix(&mut self, mut val: core::Mat) {
			let ret = unsafe { sys::cv_aruco_CharucoParameters_propCameraMatrix_Mat(self.as_raw_mut_CharucoParameters(), val.as_raw_mut_Mat()) };
			ret
		}
		
		/// distCoeffs optional vector of distortion coefficients
		#[inline]
		fn set_dist_coeffs(&mut self, mut val: core::Mat) {
			let ret = unsafe { sys::cv_aruco_CharucoParameters_propDistCoeffs_Mat(self.as_raw_mut_CharucoParameters(), val.as_raw_mut_Mat()) };
			ret
		}
		
		/// minMarkers number of adjacent markers that must be detected to return a charuco corner, default = 2
		#[inline]
		fn set_min_markers(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_CharucoParameters_propMinMarkers_int(self.as_raw_mut_CharucoParameters(), val) };
			ret
		}
		
		/// try to use refine board, default false
		#[inline]
		fn set_try_refine_markers(&mut self, val: bool) {
			let ret = unsafe { sys::cv_aruco_CharucoParameters_propTryRefineMarkers_bool(self.as_raw_mut_CharucoParameters(), val) };
			ret
		}
		
	}
	
	pub struct CharucoParameters {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CharucoParameters }
	
	impl Drop for CharucoParameters {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_aruco_CharucoParameters_delete(self.as_raw_mut_CharucoParameters()) };
		}
	}
	
	unsafe impl Send for CharucoParameters {}
	
	impl crate::objdetect::CharucoParametersTraitConst for CharucoParameters {
		#[inline] fn as_raw_CharucoParameters(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::CharucoParametersTrait for CharucoParameters {
		#[inline] fn as_raw_mut_CharucoParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CharucoParameters {
		#[inline]
		pub fn default() -> Result<crate::objdetect::CharucoParameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_CharucoParameters_CharucoParameters(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::CharucoParameters::opencv_from_extern(ret) };
			Ok(ret)
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
				.finish()
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
		
		/// minimum mean distance beetween two marker corners to be considered imilar, so that the smaller one is removed.
		/// 
		/// The rate is relative to the smaller perimeter of the two markers (default 0.05).
		#[inline]
		fn min_marker_distance_rate(&self) -> f64 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinMarkerDistanceRate_const(self.as_raw_DetectorParameters()) };
			ret
		}
		
		/// default value CORNER_REFINE_NONE
		#[inline]
		fn corner_refinement_method(&self) -> i32 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propCornerRefinementMethod_const(self.as_raw_DetectorParameters()) };
			ret
		}
		
		/// window size for the corner refinement process (in pixels) (default 5).
		#[inline]
		fn corner_refinement_win_size(&self) -> i32 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propCornerRefinementWinSize_const(self.as_raw_DetectorParameters()) };
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
		
		/// minimun standard deviation in pixels values during the decodification step to apply Otsu
		/// thresholding (otherwise, all the bits are set to 0 or 1 depending on mean higher than 128 or not) (default 5.0)
		#[inline]
		fn min_otsu_std_dev(&self) -> f64 {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinOtsuStdDev_const(self.as_raw_DetectorParameters()) };
			ret
		}
		
		/// error correction rate respect to the maximun error correction capability for each dictionary (default 0.6).
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
		
	}
	
	/// Mutable methods for [crate::objdetect::DetectorParameters]
	pub trait DetectorParametersTrait: crate::objdetect::DetectorParametersTraitConst {
		fn as_raw_mut_DetectorParameters(&mut self) -> *mut c_void;
	
		/// minimum window size for adaptive thresholding before finding contours (default 3).
		#[inline]
		fn set_adaptive_thresh_win_size_min(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMin_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// maximum window size for adaptive thresholding before finding contours (default 23).
		#[inline]
		fn set_adaptive_thresh_win_size_max(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeMax_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// increments from adaptiveThreshWinSizeMin to adaptiveThreshWinSizeMax during the thresholding (default 10).
		#[inline]
		fn set_adaptive_thresh_win_size_step(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAdaptiveThreshWinSizeStep_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// constant for adaptive thresholding before finding contours (default 7)
		#[inline]
		fn set_adaptive_thresh_constant(&mut self, val: f64) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAdaptiveThreshConstant_double(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// determine minimum perimeter for marker contour to be detected.
		/// 
		/// This is defined as a rate respect to the maximum dimension of the input image (default 0.03).
		#[inline]
		fn set_min_marker_perimeter_rate(&mut self, val: f64) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinMarkerPerimeterRate_double(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// determine maximum perimeter for marker contour to be detected.
		/// 
		/// This is defined as a rate respect to the maximum dimension of the input image (default 4.0).
		#[inline]
		fn set_max_marker_perimeter_rate(&mut self, val: f64) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMaxMarkerPerimeterRate_double(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// minimum accuracy during the polygonal approximation process to determine which contours are squares. (default 0.03)
		#[inline]
		fn set_polygonal_approx_accuracy_rate(&mut self, val: f64) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propPolygonalApproxAccuracyRate_double(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// minimum distance between corners for detected markers relative to its perimeter (default 0.05)
		#[inline]
		fn set_min_corner_distance_rate(&mut self, val: f64) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinCornerDistanceRate_double(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// minimum distance of any corner to the image border for detected markers (in pixels) (default 3)
		#[inline]
		fn set_min_distance_to_border(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinDistanceToBorder_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// minimum mean distance beetween two marker corners to be considered imilar, so that the smaller one is removed.
		/// 
		/// The rate is relative to the smaller perimeter of the two markers (default 0.05).
		#[inline]
		fn set_min_marker_distance_rate(&mut self, val: f64) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinMarkerDistanceRate_double(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// default value CORNER_REFINE_NONE
		#[inline]
		fn set_corner_refinement_method(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propCornerRefinementMethod_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// window size for the corner refinement process (in pixels) (default 5).
		#[inline]
		fn set_corner_refinement_win_size(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propCornerRefinementWinSize_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// maximum number of iterations for stop criteria of the corner refinement process (default 30).
		#[inline]
		fn set_corner_refinement_max_iterations(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propCornerRefinementMaxIterations_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// minimum error for the stop cristeria of the corner refinement process (default: 0.1)
		#[inline]
		fn set_corner_refinement_min_accuracy(&mut self, val: f64) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propCornerRefinementMinAccuracy_double(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// number of bits of the marker border, i.e. marker border width (default 1).
		#[inline]
		fn set_marker_border_bits(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMarkerBorderBits_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// number of bits (per dimension) for each cell of the marker when removing the perspective (default 4).
		#[inline]
		fn set_perspective_remove_pixel_per_cell(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propPerspectiveRemovePixelPerCell_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// width of the margin of pixels on each cell not considered for the determination of the cell bit.
		/// 
		/// Represents the rate respect to the total size of the cell, i.e. perspectiveRemovePixelPerCell (default 0.13)
		#[inline]
		fn set_perspective_remove_ignored_margin_per_cell(&mut self, val: f64) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propPerspectiveRemoveIgnoredMarginPerCell_double(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// maximum number of accepted erroneous bits in the border (i.e. number of allowed white bits in the border).
		/// 
		/// Represented as a rate respect to the total number of bits per marker (default 0.35).
		#[inline]
		fn set_max_erroneous_bits_in_border_rate(&mut self, val: f64) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMaxErroneousBitsInBorderRate_double(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// minimun standard deviation in pixels values during the decodification step to apply Otsu
		/// thresholding (otherwise, all the bits are set to 0 or 1 depending on mean higher than 128 or not) (default 5.0)
		#[inline]
		fn set_min_otsu_std_dev(&mut self, val: f64) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinOtsuStdDev_double(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// error correction rate respect to the maximun error correction capability for each dictionary (default 0.6).
		#[inline]
		fn set_error_correction_rate(&mut self, val: f64) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propErrorCorrectionRate_double(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// April :: User-configurable parameters.
		/// 
		/// Detection of quads can be done on a lower-resolution image, improving speed at a cost of
		/// pose accuracy and a slight decrease in detection rate. Decoding the binary payload is still
		#[inline]
		fn set_april_tag_quad_decimate(&mut self, val: f32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagQuadDecimate_float(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// what Gaussian blur should be applied to the segmented image (used for quad detection?)
		#[inline]
		fn set_april_tag_quad_sigma(&mut self, val: f32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagQuadSigma_float(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// reject quads containing too few pixels (default 5).
		#[inline]
		fn set_april_tag_min_cluster_pixels(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagMinClusterPixels_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// how many corner candidates to consider when segmenting a group of pixels into a quad (default 10).
		#[inline]
		fn set_april_tag_max_nmaxima(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagMaxNmaxima_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// reject quads where pairs of edges have angles that are close to straight or close to 180 degrees.
		/// 
		/// Zero means that no quads are rejected. (In radians) (default 10*PI/180)
		#[inline]
		fn set_april_tag_critical_rad(&mut self, val: f32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagCriticalRad_float(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// when fitting lines to the contours, what is the maximum mean squared error
		#[inline]
		fn set_april_tag_max_line_fit_mse(&mut self, val: f32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagMaxLineFitMse_float(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// add an extra check that the white model must be (overall) brighter than the black model.
		/// 
		/// When we build our model of black & white pixels, we add an extra check that the white model must be (overall)
		/// brighter than the black model. How much brighter? (in pixel values, [0,255]), (default 5)
		#[inline]
		fn set_april_tag_min_white_black_diff(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagMinWhiteBlackDiff_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// should the thresholded image be deglitched? Only useful for very noisy images (default 0).
		#[inline]
		fn set_april_tag_deglitch(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propAprilTagDeglitch_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// to check if there is a white marker.
		/// 
		/// In order to generate a "white" marker just invert a normal marker by using a tilde, ~markerImage. (default false)
		#[inline]
		fn set_detect_inverted_marker(&mut self, val: bool) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propDetectInvertedMarker_bool(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// enable the new and faster Aruco detection strategy.
		/// 
		/// Proposed in the paper:
		/// Romero-Ramirez et al: Speeded up detection of squared fiducial markers (2018)
		/// <https://www.researchgate.net/publication/325787310_Speeded_Up_Detection_of_Squared_Fiducial_Markers>
		#[inline]
		fn set_use_aruco3_detection(&mut self, val: bool) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propUseAruco3Detection_bool(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// minimum side length of a marker in the canonical image. Latter is the binarized image in which contours are searched.
		#[inline]
		fn set_min_side_length_canonical_img(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinSideLengthCanonicalImg_int(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// range [0,1], eq (2) from paper. The parameter tau_i has a direct influence on the processing speed.
		#[inline]
		fn set_min_marker_length_ratio_original_img(&mut self, val: f32) {
			let ret = unsafe { sys::cv_aruco_DetectorParameters_propMinMarkerLengthRatioOriginalImg_float(self.as_raw_mut_DetectorParameters(), val) };
			ret
		}
		
		/// Read a new set of DetectorParameters from FileNode (use FileStorage.root()).
		#[inline]
		fn read_detector_parameters(&mut self, fn_: &core::FileNode) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_DetectorParameters_readDetectorParameters_const_FileNodeR(self.as_raw_mut_DetectorParameters(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Write a set of DetectorParameters to FileStorage
		/// 
		/// ## C++ default parameters
		/// * name: String()
		#[inline]
		fn write_detector_parameters(&mut self, fs: &mut core::FileStorage, name: &str) -> Result<bool> {
			extern_container_arg!(name);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_DetectorParameters_writeDetectorParameters_FileStorageR_const_StringR(self.as_raw_mut_DetectorParameters(), fs.as_raw_mut_FileStorage(), name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Write a set of DetectorParameters to FileStorage
		/// 
		/// ## Note
		/// This alternative version of [write_detector_parameters] function uses the following default values for its arguments:
		/// * name: String()
		#[inline]
		fn write_detector_parameters_def(&mut self, fs: &mut core::FileStorage) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_DetectorParameters_writeDetectorParameters_FileStorageR(self.as_raw_mut_DetectorParameters(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// struct DetectorParameters is used by ArucoDetector
	pub struct DetectorParameters {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { DetectorParameters }
	
	impl Drop for DetectorParameters {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_aruco_DetectorParameters_delete(self.as_raw_mut_DetectorParameters()) };
		}
	}
	
	unsafe impl Send for DetectorParameters {}
	
	impl crate::objdetect::DetectorParametersTraitConst for DetectorParameters {
		#[inline] fn as_raw_DetectorParameters(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::DetectorParametersTrait for DetectorParameters {
		#[inline] fn as_raw_mut_DetectorParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl DetectorParameters {
		#[inline]
		pub fn default() -> Result<crate::objdetect::DetectorParameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_DetectorParameters_DetectorParameters(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::DetectorParameters::opencv_from_extern(ret) };
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
				.field("corner_refinement_method", &crate::objdetect::DetectorParametersTraitConst::corner_refinement_method(self))
				.field("corner_refinement_win_size", &crate::objdetect::DetectorParametersTraitConst::corner_refinement_win_size(self))
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
				.finish()
		}
	}
	
	/// Constant methods for [crate::objdetect::Dictionary]
	pub trait DictionaryTraitConst {
		fn as_raw_Dictionary(&self) -> *const c_void;
	
		#[inline]
		fn bytes_list(&self) -> core::Mat {
			let ret = unsafe { sys::cv_aruco_Dictionary_propBytesList_const(self.as_raw_Dictionary()) };
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn marker_size(&self) -> i32 {
			let ret = unsafe { sys::cv_aruco_Dictionary_propMarkerSize_const(self.as_raw_Dictionary()) };
			ret
		}
		
		#[inline]
		fn max_correction_bits(&self) -> i32 {
			let ret = unsafe { sys::cv_aruco_Dictionary_propMaxCorrectionBits_const(self.as_raw_Dictionary()) };
			ret
		}
		
		/// Given a matrix of bits. Returns whether if marker is identified or not.
		/// 
		/// It returns by reference the correct id (if any) and the correct rotation
		#[inline]
		fn identify(&self, only_bits: &core::Mat, idx: &mut i32, rotation: &mut i32, max_correction_rate: f64) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_identify_const_const_MatR_intR_intR_double(self.as_raw_Dictionary(), only_bits.as_raw_Mat(), idx, rotation, max_correction_rate, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Returns the distance of the input bits to the specific id.
		/// 
		/// If allRotations is true, the four posible bits rotation are considered
		/// 
		/// ## C++ default parameters
		/// * all_rotations: true
		#[inline]
		fn get_distance_to_id(&self, bits: &impl core::ToInputArray, id: i32, all_rotations: bool) -> Result<i32> {
			input_array_arg!(bits);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_getDistanceToId_const_const__InputArrayR_int_bool(self.as_raw_Dictionary(), bits.as_raw__InputArray(), id, all_rotations, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Returns the distance of the input bits to the specific id.
		/// 
		/// If allRotations is true, the four posible bits rotation are considered
		/// 
		/// ## Note
		/// This alternative version of [get_distance_to_id] function uses the following default values for its arguments:
		/// * all_rotations: true
		#[inline]
		fn get_distance_to_id_def(&self, bits: &impl core::ToInputArray, id: i32) -> Result<i32> {
			input_array_arg!(bits);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_getDistanceToId_const_const__InputArrayR_int(self.as_raw_Dictionary(), bits.as_raw__InputArray(), id, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Generate a canonical marker image
		/// 
		/// ## C++ default parameters
		/// * border_bits: 1
		#[inline]
		fn generate_image_marker(&self, id: i32, side_pixels: i32, _img: &mut impl core::ToOutputArray, border_bits: i32) -> Result<()> {
			output_array_arg!(_img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_generateImageMarker_const_int_int_const__OutputArrayR_int(self.as_raw_Dictionary(), id, side_pixels, _img.as_raw__OutputArray(), border_bits, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Generate a canonical marker image
		/// 
		/// ## Note
		/// This alternative version of [generate_image_marker] function uses the following default values for its arguments:
		/// * border_bits: 1
		#[inline]
		fn generate_image_marker_def(&self, id: i32, side_pixels: i32, _img: &mut impl core::ToOutputArray) -> Result<()> {
			output_array_arg!(_img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_generateImageMarker_const_int_int_const__OutputArrayR(self.as_raw_Dictionary(), id, side_pixels, _img.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::objdetect::Dictionary]
	pub trait DictionaryTrait: crate::objdetect::DictionaryTraitConst {
		fn as_raw_mut_Dictionary(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_bytes_list(&mut self, mut val: core::Mat) {
			let ret = unsafe { sys::cv_aruco_Dictionary_propBytesList_Mat(self.as_raw_mut_Dictionary(), val.as_raw_mut_Mat()) };
			ret
		}
		
		#[inline]
		fn set_marker_size(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_Dictionary_propMarkerSize_int(self.as_raw_mut_Dictionary(), val) };
			ret
		}
		
		#[inline]
		fn set_max_correction_bits(&mut self, val: i32) {
			let ret = unsafe { sys::cv_aruco_Dictionary_propMaxCorrectionBits_int(self.as_raw_mut_Dictionary(), val) };
			ret
		}
		
		/// Read a new dictionary from FileNode.
		/// 
		/// Dictionary format:
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
		fn read_dictionary(&mut self, fn_: &core::FileNode) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_readDictionary_const_FileNodeR(self.as_raw_mut_Dictionary(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Write a dictionary to FileStorage, format is the same as in readDictionary().
		/// 
		/// ## C++ default parameters
		/// * name: String()
		#[inline]
		fn write_dictionary(&mut self, fs: &mut core::FileStorage, name: &str) -> Result<()> {
			extern_container_arg!(name);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_writeDictionary_FileStorageR_const_StringR(self.as_raw_mut_Dictionary(), fs.as_raw_mut_FileStorage(), name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Write a dictionary to FileStorage, format is the same as in readDictionary().
		/// 
		/// ## Note
		/// This alternative version of [write_dictionary] function uses the following default values for its arguments:
		/// * name: String()
		#[inline]
		fn write_dictionary_def(&mut self, fs: &mut core::FileStorage) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_writeDictionary_FileStorageR(self.as_raw_mut_Dictionary(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Dictionary/Set of markers, it contains the inner codification
	/// 
	/// BytesList contains the marker codewords where:
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
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_aruco_Dictionary_delete(self.as_raw_mut_Dictionary()) };
		}
	}
	
	unsafe impl Send for Dictionary {}
	
	impl crate::objdetect::DictionaryTraitConst for Dictionary {
		#[inline] fn as_raw_Dictionary(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::DictionaryTrait for Dictionary {
		#[inline] fn as_raw_mut_Dictionary(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Dictionary {
		#[inline]
		pub fn default() -> Result<crate::objdetect::Dictionary> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_Dictionary(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::Dictionary::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * maxcorr: 0
		#[inline]
		pub fn new(bytes_list: &core::Mat, _marker_size: i32, maxcorr: i32) -> Result<crate::objdetect::Dictionary> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_Dictionary_const_MatR_int_int(bytes_list.as_raw_Mat(), _marker_size, maxcorr, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::Dictionary::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * maxcorr: 0
		#[inline]
		pub fn new_def(bytes_list: &core::Mat, _marker_size: i32) -> Result<crate::objdetect::Dictionary> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_Dictionary_const_MatR_int(bytes_list.as_raw_Mat(), _marker_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::Dictionary::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Transform matrix of bits to list of bytes in the 4 rotations
		#[inline]
		pub fn get_byte_list_from_bits(bits: &core::Mat) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_getByteListFromBits_const_MatR(bits.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Transform list of bytes to matrix of bits
		#[inline]
		pub fn get_bits_from_byte_list(byte_list: &core::Mat, marker_size: i32) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_Dictionary_getBitsFromByteList_const_MatR_int(byte_list.as_raw_Mat(), marker_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
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
	
	/// Constant methods for [crate::objdetect::GridBoard]
	pub trait GridBoardTraitConst: crate::objdetect::BoardTraitConst {
		fn as_raw_GridBoard(&self) -> *const c_void;
	
		#[inline]
		fn get_grid_size(&self) -> Result<core::Size> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_GridBoard_getGridSize_const(self.as_raw_GridBoard(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_marker_length(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_GridBoard_getMarkerLength_const(self.as_raw_GridBoard(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_marker_separation(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_GridBoard_getMarkerSeparation_const(self.as_raw_GridBoard(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::objdetect::GridBoard]
	pub trait GridBoardTrait: crate::objdetect::BoardTrait + crate::objdetect::GridBoardTraitConst {
		fn as_raw_mut_GridBoard(&mut self) -> *mut c_void;
	
	}
	
	/// Planar board with grid arrangement of markers
	/// 
	/// More common type of board. All markers are placed in the same plane in a grid arrangement.
	/// The board image can be drawn using generateImage() method.
	pub struct GridBoard {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GridBoard }
	
	impl Drop for GridBoard {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_aruco_GridBoard_delete(self.as_raw_mut_GridBoard()) };
		}
	}
	
	unsafe impl Send for GridBoard {}
	
	impl crate::objdetect::BoardTraitConst for GridBoard {
		#[inline] fn as_raw_Board(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::BoardTrait for GridBoard {
		#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::objdetect::GridBoardTraitConst for GridBoard {
		#[inline] fn as_raw_GridBoard(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::GridBoardTrait for GridBoard {
		#[inline] fn as_raw_mut_GridBoard(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
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
		pub fn new(size: core::Size, marker_length: f32, marker_separation: f32, dictionary: &crate::objdetect::Dictionary, ids: &impl core::ToInputArray) -> Result<crate::objdetect::GridBoard> {
			input_array_arg!(ids);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_GridBoard_GridBoard_const_SizeR_float_float_const_DictionaryR_const__InputArrayR(&size, marker_length, marker_separation, dictionary.as_raw_Dictionary(), ids.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
		pub fn new_def(size: core::Size, marker_length: f32, marker_separation: f32, dictionary: &crate::objdetect::Dictionary) -> Result<crate::objdetect::GridBoard> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_GridBoard_GridBoard_const_SizeR_float_float_const_DictionaryR(&size, marker_length, marker_separation, dictionary.as_raw_Dictionary(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::GridBoard::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn default() -> Result<crate::objdetect::GridBoard> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_GridBoard_GridBoard(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::GridBoard::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl Clone for GridBoard {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_aruco_GridBoard_implicitClone_const(self.as_raw_GridBoard())) }
		}
	}
	
	boxed_cast_base! { GridBoard, crate::objdetect::Board, cv_aruco_GridBoard_to_Board }
	
	impl std::fmt::Debug for GridBoard {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GridBoard")
				.finish()
		}
	}
	
	/// struct RefineParameters is used by ArucoDetector
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq)]
	pub struct RefineParameters {
		/// minRepDistance minimum distance between the corners of the rejected candidate and the reprojected marker
		/// in order to consider it as a correspondence.
		pub min_rep_distance: f32,
		/// minRepDistance rate of allowed erroneous bits respect to the error correction capability of the used dictionary.
		/// 
		/// -1 ignores the error correction step.
		pub error_correction_rate: f32,
		/// checkAllOrders consider the four posible corner orders in the rejectedCorners array.
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
			return_receive!(unsafe ocvrs_return => ret);
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
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Read a new set of RefineParameters from FileNode (use FileStorage.root()).
		#[inline]
		pub fn read_refine_parameters(self, fn_: &core::FileNode) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_RefineParameters_readRefineParameters_const_FileNodeR(self.opencv_as_extern(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Write a set of RefineParameters to FileStorage
		/// 
		/// ## C++ default parameters
		/// * name: String()
		#[inline]
		pub fn write_refine_parameters(self, fs: &mut core::FileStorage, name: &str) -> Result<bool> {
			extern_container_arg!(name);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_RefineParameters_writeRefineParameters_FileStorageR_const_StringR(self.opencv_as_extern(), fs.as_raw_mut_FileStorage(), name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Write a set of RefineParameters to FileStorage
		/// 
		/// ## Note
		/// This alternative version of [write_refine_parameters] function uses the following default values for its arguments:
		/// * name: String()
		#[inline]
		pub fn write_refine_parameters_def(self, fs: &mut core::FileStorage) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_aruco_RefineParameters_writeRefineParameters_FileStorageR(self.opencv_as_extern(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
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
		fn decode_with_type(&self, img: &impl core::ToInputArray, points: &impl core::ToInputArray, decoded_info: &mut core::Vector<String>, decoded_type: &mut core::Vector<String>) -> Result<bool> {
			input_array_arg!(img);
			input_array_arg!(points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_barcode_BarcodeDetector_decodeWithType_const_const__InputArrayR_const__InputArrayR_vectorLstringGR_vectorLstringGR(self.as_raw_BarcodeDetector(), img.as_raw__InputArray(), points.as_raw__InputArray(), decoded_info.as_raw_mut_VectorOfString(), decoded_type.as_raw_mut_VectorOfString(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
		fn detect_and_decode_with_type(&self, img: &impl core::ToInputArray, decoded_info: &mut core::Vector<String>, decoded_type: &mut core::Vector<String>, points: &mut impl core::ToOutputArray) -> Result<bool> {
			input_array_arg!(img);
			output_array_arg!(points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_barcode_BarcodeDetector_detectAndDecodeWithType_const_const__InputArrayR_vectorLstringGR_vectorLstringGR_const__OutputArrayR(self.as_raw_BarcodeDetector(), img.as_raw__InputArray(), decoded_info.as_raw_mut_VectorOfString(), decoded_type.as_raw_mut_VectorOfString(), points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
		/// This alternative version of [detect_and_decode_with_type] function uses the following default values for its arguments:
		/// * points: noArray()
		#[inline]
		fn detect_and_decode_with_type_def(&self, img: &impl core::ToInputArray, decoded_info: &mut core::Vector<String>, decoded_type: &mut core::Vector<String>) -> Result<bool> {
			input_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_barcode_BarcodeDetector_detectAndDecodeWithType_const_const__InputArrayR_vectorLstringGR_vectorLstringGR(self.as_raw_BarcodeDetector(), img.as_raw__InputArray(), decoded_info.as_raw_mut_VectorOfString(), decoded_type.as_raw_mut_VectorOfString(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::objdetect::BarcodeDetector]
	pub trait BarcodeDetectorTrait: crate::objdetect::BarcodeDetectorTraitConst + crate::objdetect::GraphicalCodeDetectorTrait {
		fn as_raw_mut_BarcodeDetector(&mut self) -> *mut c_void;
	
	}
	
	pub struct BarcodeDetector {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { BarcodeDetector }
	
	impl Drop for BarcodeDetector {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_barcode_BarcodeDetector_delete(self.as_raw_mut_BarcodeDetector()) };
		}
	}
	
	unsafe impl Send for BarcodeDetector {}
	
	impl crate::objdetect::GraphicalCodeDetectorTraitConst for BarcodeDetector {
		#[inline] fn as_raw_GraphicalCodeDetector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::GraphicalCodeDetectorTrait for BarcodeDetector {
		#[inline] fn as_raw_mut_GraphicalCodeDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::objdetect::BarcodeDetectorTraitConst for BarcodeDetector {
		#[inline] fn as_raw_BarcodeDetector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::objdetect::BarcodeDetectorTrait for BarcodeDetector {
		#[inline] fn as_raw_mut_BarcodeDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl BarcodeDetector {
		/// Initialize the BarcodeDetector.
		#[inline]
		pub fn default() -> Result<crate::objdetect::BarcodeDetector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_barcode_BarcodeDetector_BarcodeDetector(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::objdetect::BarcodeDetector::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Initialize the BarcodeDetector.
		/// 
		/// Parameters allow to load _optional_ Super Resolution DNN model for better quality.
		/// ## Parameters
		/// * prototxt_path: prototxt file path for the super resolution model
		/// * model_path: model file path for the super resolution model
		#[inline]
		pub fn new(prototxt_path: &str, model_path: &str) -> Result<crate::objdetect::BarcodeDetector> {
			extern_container_arg!(prototxt_path);
			extern_container_arg!(model_path);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_barcode_BarcodeDetector_BarcodeDetector_const_stringR_const_stringR(prototxt_path.opencv_as_extern(), model_path.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
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
	
	boxed_cast_base! { BarcodeDetector, crate::objdetect::GraphicalCodeDetector, cv_barcode_BarcodeDetector_to_GraphicalCodeDetector }
	
	impl std::fmt::Debug for BarcodeDetector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("BarcodeDetector")
				.finish()
		}
	}
}
