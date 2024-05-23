//! # Object Detection
//!
//! Haar Feature-based Cascade Classifier for Object Detection
//! ----------------------------------------------------------
//!
//! The object detector described below has been initially proposed by Paul Viola [Viola01](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Viola01) and
//! improved by Rainer Lienhart [Lienhart02](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Lienhart02) .
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
//! ![image](https://docs.opencv.org/3.4.20/haarfeatures.png)
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
//! To see the object detector at work, have a look at the facedetect demo:
//! <https://github.com/opencv/opencv/tree/3.4/samples/cpp/dbt_face_detection.cpp>
//!
//! The following reference is for the detection part only. There is a separate application called
//! opencv_traincascade that can train a cascade of boosted classifiers from a set of samples.
//!
//!
//! Note: In the new C++ interface it is also possible to use LBP (local binary pattern) features in
//! addition to Haar-like features. .. [Viola01] Paul Viola and Michael J. Jones. Rapid Object Detection
//! using a Boosted Cascade of Simple Features. IEEE CVPR, 2001. The paper is available online at
//! <http://research.microsoft.com/en-us/um/people/viola/Pubs/Detect/violaJones_CVPR2001.pdf>
//!    # C API
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{BaseCascadeClassifierTrait, BaseCascadeClassifierTraitConst, BaseCascadeClassifier_MaskGeneratorTrait, BaseCascadeClassifier_MaskGeneratorTraitConst, CascadeClassifierTrait, CascadeClassifierTraitConst, DetectionBasedTrackerTrait, DetectionBasedTrackerTraitConst, DetectionBasedTracker_ExtObjectTrait, DetectionBasedTracker_ExtObjectTraitConst, DetectionBasedTracker_IDetectorTrait, DetectionBasedTracker_IDetectorTraitConst, DetectionBasedTracker_ParametersTrait, DetectionBasedTracker_ParametersTraitConst, DetectionROITrait, DetectionROITraitConst, HOGDescriptorTrait, HOGDescriptorTraitConst, QRCodeDetectorTrait, QRCodeDetectorTraitConst, QRCodeEncoderTrait, QRCodeEncoderTraitConst, SimilarRectsTrait, SimilarRectsTraitConst};
}

// CASCADE_DO_CANNY_PRUNING /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:168
pub const CASCADE_DO_CANNY_PRUNING: i32 = 1;
// CASCADE_DO_ROUGH_SEARCH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:171
pub const CASCADE_DO_ROUGH_SEARCH: i32 = 8;
// CASCADE_FIND_BIGGEST_OBJECT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:170
pub const CASCADE_FIND_BIGGEST_OBJECT: i32 = 4;
// CASCADE_SCALE_IMAGE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:169
pub const CASCADE_SCALE_IMAGE: i32 = 2;
// CV_HAAR_DO_CANNY_PRUNING /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/objdetect_c.h:131
pub const CV_HAAR_DO_CANNY_PRUNING: i32 = 1;
// CV_HAAR_DO_ROUGH_SEARCH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/objdetect_c.h:134
pub const CV_HAAR_DO_ROUGH_SEARCH: i32 = 8;
// CV_HAAR_FEATURE_MAX /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/objdetect_c.h:71
pub const CV_HAAR_FEATURE_MAX: i32 = 3;
// CV_HAAR_FIND_BIGGEST_OBJECT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/objdetect_c.h:133
pub const CV_HAAR_FIND_BIGGEST_OBJECT: i32 = 4;
// CV_HAAR_MAGIC_VAL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/objdetect_c.h:64
pub const CV_HAAR_MAGIC_VAL: i32 = 0x42500000;
// CV_HAAR_SCALE_IMAGE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/objdetect_c.h:132
pub const CV_HAAR_SCALE_IMAGE: i32 = 2;
// CV_HAAR_STAGE_MAX /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/objdetect_c.h:72
pub const CV_HAAR_STAGE_MAX: i32 = 1000;
// CV_TYPE_NAME_HAAR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/objdetect_c.h:65
pub const CV_TYPE_NAME_HAAR: &str = "opencv-haar-classifier";
// DETECTED /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:145
pub const DetectionBasedTracker_DETECTED: i32 = 1;
// DETECTED_NOT_SHOWN_YET /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:144
pub const DetectionBasedTracker_DETECTED_NOT_SHOWN_YET: i32 = 0;
// DETECTED_TEMPORARY_LOST /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:146
pub const DetectionBasedTracker_DETECTED_TEMPORARY_LOST: i32 = 2;
// WRONG_OBJECT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:147
pub const DetectionBasedTracker_WRONG_OBJECT: i32 = 3;
/// Default nlevels value.
// DEFAULT_NLEVELS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:377
pub const HOGDescriptor_DEFAULT_NLEVELS: i32 = 64;
/// Default histogramNormType
// L2Hys /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:375
pub const HOGDescriptor_L2Hys: i32 = 0;
// CORRECT_LEVEL_H /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:697
pub const QRCodeEncoder_CORRECT_LEVEL_H: i32 = 3;
// CORRECT_LEVEL_L /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:694
pub const QRCodeEncoder_CORRECT_LEVEL_L: i32 = 0;
// CORRECT_LEVEL_M /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:695
pub const QRCodeEncoder_CORRECT_LEVEL_M: i32 = 1;
// CORRECT_LEVEL_Q /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:696
pub const QRCodeEncoder_CORRECT_LEVEL_Q: i32 = 2;
// ECI_UTF8 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:701
pub const QRCodeEncoder_ECI_UTF8: i32 = 26;
// MODE_ALPHANUMERIC /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:686
pub const QRCodeEncoder_MODE_ALPHANUMERIC: i32 = 2;
// MODE_AUTO /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:684
pub const QRCodeEncoder_MODE_AUTO: i32 = -1;
// MODE_BYTE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:687
pub const QRCodeEncoder_MODE_BYTE: i32 = 4;
// MODE_ECI /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:688
pub const QRCodeEncoder_MODE_ECI: i32 = 7;
// MODE_KANJI /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:689
pub const QRCodeEncoder_MODE_KANJI: i32 = 8;
// MODE_NUMERIC /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:685
pub const QRCodeEncoder_MODE_NUMERIC: i32 = 1;
// MODE_STRUCTURED_APPEND /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:690
pub const QRCodeEncoder_MODE_STRUCTURED_APPEND: i32 = 3;
// ObjectStatus /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:142
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DetectionBasedTracker_ObjectStatus {
	DETECTED_NOT_SHOWN_YET = 0,
	DETECTED = 1,
	DETECTED_TEMPORARY_LOST = 2,
	WRONG_OBJECT = 3,
}

impl TryFrom<i32> for DetectionBasedTracker_ObjectStatus {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::DETECTED_NOT_SHOWN_YET),
			1 => Ok(Self::DETECTED),
			2 => Ok(Self::DETECTED_TEMPORARY_LOST),
			3 => Ok(Self::WRONG_OBJECT),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::objdetect::DetectionBasedTracker_ObjectStatus"))),
		}
	}
}

opencv_type_enum! { crate::objdetect::DetectionBasedTracker_ObjectStatus }

// CorrectionLevel /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:693
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum QRCodeEncoder_CorrectionLevel {
	CORRECT_LEVEL_L = 0,
	CORRECT_LEVEL_M = 1,
	CORRECT_LEVEL_Q = 2,
	CORRECT_LEVEL_H = 3,
}

impl TryFrom<i32> for QRCodeEncoder_CorrectionLevel {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::CORRECT_LEVEL_L),
			1 => Ok(Self::CORRECT_LEVEL_M),
			2 => Ok(Self::CORRECT_LEVEL_Q),
			3 => Ok(Self::CORRECT_LEVEL_H),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::objdetect::QRCodeEncoder_CorrectionLevel"))),
		}
	}
}

opencv_type_enum! { crate::objdetect::QRCodeEncoder_CorrectionLevel }

// ECIEncodings /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:700
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum QRCodeEncoder_ECIEncodings {
	ECI_UTF8 = 26,
}

impl TryFrom<i32> for QRCodeEncoder_ECIEncodings {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			26 => Ok(Self::ECI_UTF8),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::objdetect::QRCodeEncoder_ECIEncodings"))),
		}
	}
}

opencv_type_enum! { crate::objdetect::QRCodeEncoder_ECIEncodings }

// EncodeMode /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:683
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

impl TryFrom<i32> for QRCodeEncoder_EncodeMode {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			-1 => Ok(Self::MODE_AUTO),
			1 => Ok(Self::MODE_NUMERIC),
			2 => Ok(Self::MODE_ALPHANUMERIC),
			4 => Ok(Self::MODE_BYTE),
			7 => Ok(Self::MODE_ECI),
			8 => Ok(Self::MODE_KANJI),
			3 => Ok(Self::MODE_STRUCTURED_APPEND),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::objdetect::QRCodeEncoder_EncodeMode"))),
		}
	}
}

opencv_type_enum! { crate::objdetect::QRCodeEncoder_EncodeMode }

// Object /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:138
pub type DetectionBasedTracker_Object = core::Tuple<(core::Rect, i32)>;
// createFaceDetectionMaskGenerator()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:340
// ("cv::createFaceDetectionMaskGenerator", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn create_face_detection_mask_generator() -> Result<core::Ptr<crate::objdetect::BaseCascadeClassifier_MaskGenerator>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createFaceDetectionMaskGenerator(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::objdetect::BaseCascadeClassifier_MaskGenerator>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Decode QR code on a curved surface in image and return text that is encrypted in QR code.
/// ## Parameters
/// * in: Matrix of the type CV_8UC1 containing an image where QR code are detected.
/// * points: Input vector of vertices of a quadrangle of minimal area that describes QR code.
/// * decoded_info: String information that is encrypted in QR code.
/// * straight_qrcode: Matrix of the type CV_8UC1 containing an binary straight QR code.
///
/// ## Note
/// This alternative version of [decode_curved_qr_code] function uses the following default values for its arguments:
/// * straight_qrcode: noArray()
// cv::decodeCurvedQRCode(InputArray, InputArray, OutString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:896
// ("cv::decodeCurvedQRCode", vec![(pred!(mut, ["in", "points", "decoded_info"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::string*"]), _)]),
#[inline]
pub fn decode_curved_qr_code_def(in_: &impl ToInputArray, points: &impl ToInputArray, decoded_info: &mut String) -> Result<bool> {
	input_array_arg!(in_);
	input_array_arg!(points);
	string_arg_output_send!(via decoded_info_via);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_decodeCurvedQRCode_const__InputArrayR_const__InputArrayR_stringR(in_.as_raw__InputArray(), points.as_raw__InputArray(), &mut decoded_info_via, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	string_arg_output_receive!(decoded_info_via => decoded_info);
	Ok(ret)
}

/// Decode QR code on a curved surface in image and return text that is encrypted in QR code.
/// ## Parameters
/// * in: Matrix of the type CV_8UC1 containing an image where QR code are detected.
/// * points: Input vector of vertices of a quadrangle of minimal area that describes QR code.
/// * decoded_info: String information that is encrypted in QR code.
/// * straight_qrcode: Matrix of the type CV_8UC1 containing an binary straight QR code.
///
/// ## C++ default parameters
/// * straight_qrcode: noArray()
// decodeCurvedQRCode(InputArray, InputArray, std::string &, OutputArray)(InputArray, InputArray, OutString, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:896
// ("cv::decodeCurvedQRCode", vec![(pred!(mut, ["in", "points", "decoded_info", "straight_qrcode"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::string*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn decode_curved_qr_code(in_: &impl ToInputArray, points: &impl ToInputArray, decoded_info: &mut String, straight_qrcode: &mut impl ToOutputArray) -> Result<bool> {
	input_array_arg!(in_);
	input_array_arg!(points);
	string_arg_output_send!(via decoded_info_via);
	output_array_arg!(straight_qrcode);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_decodeCurvedQRCode_const__InputArrayR_const__InputArrayR_stringR_const__OutputArrayR(in_.as_raw__InputArray(), points.as_raw__InputArray(), &mut decoded_info_via, straight_qrcode.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	string_arg_output_receive!(decoded_info_via => decoded_info);
	Ok(ret)
}

/// Decode QR code in image and return text that is encrypted in QR code.
/// ## Parameters
/// * in: Matrix of the type CV_8UC1 containing an image where QR code are detected.
/// * points: Input vector of vertices of a quadrangle of minimal area that describes QR code.
/// * decoded_info: String information that is encrypted in QR code.
/// * straight_qrcode: Matrix of the type CV_8UC1 containing an binary straight QR code.
///
/// ## Note
/// This alternative version of [decode_qr_code] function uses the following default values for its arguments:
/// * straight_qrcode: noArray()
// cv::decodeQRCode(InputArray, InputArray, OutString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:888
// ("cv::decodeQRCode", vec![(pred!(mut, ["in", "points", "decoded_info"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::string*"]), _)]),
#[inline]
pub fn decode_qr_code_def(in_: &impl ToInputArray, points: &impl ToInputArray, decoded_info: &mut Vec<u8>) -> Result<bool> {
	input_array_arg!(in_);
	input_array_arg!(points);
	string_arg_output_send!(via decoded_info_via);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_decodeQRCode_const__InputArrayR_const__InputArrayR_stringR(in_.as_raw__InputArray(), points.as_raw__InputArray(), &mut decoded_info_via, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	string_arg_output_receive!(decoded_info_via => decoded_info);
	Ok(ret)
}

/// Decode QR code in image and return text that is encrypted in QR code.
/// ## Parameters
/// * in: Matrix of the type CV_8UC1 containing an image where QR code are detected.
/// * points: Input vector of vertices of a quadrangle of minimal area that describes QR code.
/// * decoded_info: String information that is encrypted in QR code.
/// * straight_qrcode: Matrix of the type CV_8UC1 containing an binary straight QR code.
///
/// ## C++ default parameters
/// * straight_qrcode: noArray()
// decodeQRCode(InputArray, InputArray, std::string &, OutputArray)(InputArray, InputArray, OutString, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:888
// ("cv::decodeQRCode", vec![(pred!(mut, ["in", "points", "decoded_info", "straight_qrcode"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::string*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn decode_qr_code(in_: &impl ToInputArray, points: &impl ToInputArray, decoded_info: &mut Vec<u8>, straight_qrcode: &mut impl ToOutputArray) -> Result<bool> {
	input_array_arg!(in_);
	input_array_arg!(points);
	string_arg_output_send!(via decoded_info_via);
	output_array_arg!(straight_qrcode);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_decodeQRCode_const__InputArrayR_const__InputArrayR_stringR_const__OutputArrayR(in_.as_raw__InputArray(), points.as_raw__InputArray(), &mut decoded_info_via, straight_qrcode.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	string_arg_output_receive!(decoded_info_via => decoded_info);
	Ok(ret)
}

/// Detect QR code in image and return minimum area of quadrangle that describes QR code.
/// ## Parameters
/// * in: Matrix of the type CV_8UC1 containing an image where QR code are detected.
/// * points: Output vector of vertices of a quadrangle of minimal area that describes QR code.
/// * eps_x: Epsilon neighborhood, which allows you to determine the horizontal pattern of the scheme 1:1:3:1:1 according to QR code standard.
/// * eps_y: Epsilon neighborhood, which allows you to determine the vertical pattern of the scheme 1:1:3:1:1 according to QR code standard.
///
/// ## Note
/// This alternative version of [detect_qr_code] function uses the following default values for its arguments:
/// * eps_x: 0.2
/// * eps_y: 0.1
// cv::detectQRCode(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:880
// ("cv::detectQRCode", vec![(pred!(mut, ["in", "points"], ["const cv::_InputArray*", "std::vector<cv::Point>*"]), _)]),
#[inline]
pub fn detect_qr_code_def(in_: &impl ToInputArray, points: &mut core::Vector<core::Point>) -> Result<bool> {
	input_array_arg!(in_);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detectQRCode_const__InputArrayR_vectorLPointGR(in_.as_raw__InputArray(), points.as_raw_mut_VectorOfPoint(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Detect QR code in image and return minimum area of quadrangle that describes QR code.
/// ## Parameters
/// * in: Matrix of the type CV_8UC1 containing an image where QR code are detected.
/// * points: Output vector of vertices of a quadrangle of minimal area that describes QR code.
/// * eps_x: Epsilon neighborhood, which allows you to determine the horizontal pattern of the scheme 1:1:3:1:1 according to QR code standard.
/// * eps_y: Epsilon neighborhood, which allows you to determine the vertical pattern of the scheme 1:1:3:1:1 according to QR code standard.
///
/// ## C++ default parameters
/// * eps_x: 0.2
/// * eps_y: 0.1
// detectQRCode(InputArray, std::vector<Point> &, double, double)(InputArray, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:880
// ("cv::detectQRCode", vec![(pred!(mut, ["in", "points", "eps_x", "eps_y"], ["const cv::_InputArray*", "std::vector<cv::Point>*", "double", "double"]), _)]),
#[inline]
pub fn detect_qr_code(in_: &impl ToInputArray, points: &mut core::Vector<core::Point>, eps_x: f64, eps_y: f64) -> Result<bool> {
	input_array_arg!(in_);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detectQRCode_const__InputArrayR_vectorLPointGR_double_double(in_.as_raw__InputArray(), points.as_raw_mut_VectorOfPoint(), eps_x, eps_y, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// @overload
///
/// ## Note
/// This alternative version of [group_rectangles_meanshift] function uses the following default values for its arguments:
/// * detect_threshold: 0.0
/// * win_det_size: Size(64,128)
// cv::groupRectangles_meanshift(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:162
// ("cv::groupRectangles_meanshift", vec![(pred!(mut, ["rectList", "foundWeights", "foundScales"], ["std::vector<cv::Rect>*", "std::vector<double>*", "std::vector<double>*"]), _)]),
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
// groupRectangles_meanshift(std::vector<Rect> &, std::vector<double> &, std::vector<double> &, double, Size)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:162
// ("cv::groupRectangles_meanshift", vec![(pred!(mut, ["rectList", "foundWeights", "foundScales", "detectThreshold", "winDetSize"], ["std::vector<cv::Rect>*", "std::vector<double>*", "std::vector<double>*", "double", "cv::Size"]), _)]),
#[inline]
pub fn group_rectangles_meanshift(rect_list: &mut core::Vector<core::Rect>, found_weights: &mut core::Vector<f64>, found_scales: &mut core::Vector<f64>, detect_threshold: f64, win_det_size: core::Size) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_groupRectangles_meanshift_vectorLRectGR_vectorLdoubleGR_vectorLdoubleGR_double_Size(rect_list.as_raw_mut_VectorOfRect(), found_weights.as_raw_mut_VectorOff64(), found_scales.as_raw_mut_VectorOff64(), detect_threshold, &win_det_size, ocvrs_return.as_mut_ptr()) };
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
// cv::groupRectangles(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:151
// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "groupThreshold"], ["std::vector<cv::Rect>*", "int"]), _)]),
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
// groupRectangles(std::vector<Rect> &, int, double)(CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:151
// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "groupThreshold", "eps"], ["std::vector<cv::Rect>*", "int", "double"]), _)]),
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
// groupRectangles(std::vector<Rect> &, int, double, std::vector<int> *, std::vector<double> *)(CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:156
// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "groupThreshold", "eps", "weights", "levelWeights"], ["std::vector<cv::Rect>*", "int", "double", "std::vector<int>*", "std::vector<double>*"]), _)]),
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
// cv::groupRectangles(CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:153
// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "weights", "groupThreshold"], ["std::vector<cv::Rect>*", "std::vector<int>*", "int"]), _)]),
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
// groupRectangles(std::vector<Rect> &, std::vector<int> &, int, double)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:153
// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "weights", "groupThreshold", "eps"], ["std::vector<cv::Rect>*", "std::vector<int>*", "int", "double"]), _)]),
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
// cv::groupRectangles(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:159
// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "rejectLevels", "levelWeights", "groupThreshold"], ["std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*", "int"]), _)]),
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
// groupRectangles(std::vector<Rect> &, std::vector<int> &, std::vector<double> &, int, double)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:159
// ("cv::groupRectangles", vec![(pred!(mut, ["rectList", "rejectLevels", "levelWeights", "groupThreshold", "eps"], ["std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*", "int", "double"]), _)]),
#[inline]
pub fn group_rectangles_levels(rect_list: &mut core::Vector<core::Rect>, reject_levels: &mut core::Vector<i32>, level_weights: &mut core::Vector<f64>, group_threshold: i32, eps: f64) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_groupRectangles_vectorLRectGR_vectorLintGR_vectorLdoubleGR_int_double(rect_list.as_raw_mut_VectorOfRect(), reject_levels.as_raw_mut_VectorOfi32(), level_weights.as_raw_mut_VectorOff64(), group_threshold, eps, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Constant methods for [crate::objdetect::BaseCascadeClassifier]
// BaseCascadeClassifier /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:174
pub trait BaseCascadeClassifierTraitConst: core::AlgorithmTraitConst {
	fn as_raw_BaseCascadeClassifier(&self) -> *const c_void;

	// empty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:178
	// ("cv::BaseCascadeClassifier::empty", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BaseCascadeClassifier_empty_const(self.as_raw_BaseCascadeClassifier(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// isOldFormatCascade()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:202
	// ("cv::BaseCascadeClassifier::isOldFormatCascade", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn is_old_format_cascade(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BaseCascadeClassifier_isOldFormatCascade_const(self.as_raw_BaseCascadeClassifier(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getOriginalWindowSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:203
	// ("cv::BaseCascadeClassifier::getOriginalWindowSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_original_window_size(&self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BaseCascadeClassifier_getOriginalWindowSize_const(self.as_raw_BaseCascadeClassifier(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getFeatureType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:204
	// ("cv::BaseCascadeClassifier::getFeatureType", vec![(pred!(const, [], []), _)]),
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

	// load(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:179
	// ("cv::BaseCascadeClassifier::load", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	#[inline]
	fn load(&mut self, filename: &str) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BaseCascadeClassifier_load_const_StringR(self.as_raw_mut_BaseCascadeClassifier(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, double, int, int, Size, Size)(InputArray, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:180
	// ("cv::BaseCascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "double", "int", "int", "cv::Size", "cv::Size"]), _)]),
	#[inline]
	fn detect_multi_scale(&mut self, image: &impl ToInputArray, objects: &mut core::Vector<core::Rect>, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_double_int_int_Size_Size(self.as_raw_mut_BaseCascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_mut_VectorOfRect(), scale_factor, min_neighbors, flags, &min_size, &max_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<int> &, double, int, int, Size, Size)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:186
	// ("cv::BaseCascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "numDetections", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "double", "int", "int", "cv::Size", "cv::Size"]), _)]),
	#[inline]
	fn detect_multi_scale_num(&mut self, image: &impl ToInputArray, objects: &mut core::Vector<core::Rect>, num_detections: &mut core::Vector<i32>, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_double_int_int_Size_Size(self.as_raw_mut_BaseCascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_mut_VectorOfRect(), num_detections.as_raw_mut_VectorOfi32(), scale_factor, min_neighbors, flags, &min_size, &max_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<int> &, std::vector<double> &, double, int, int, Size, Size, bool)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:193
	// ("cv::BaseCascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "rejectLevels", "levelWeights", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize", "outputRejectLevels"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*", "double", "int", "int", "cv::Size", "cv::Size", "bool"]), _)]),
	#[inline]
	fn detect_multi_scale_levels(&mut self, image: &impl ToInputArray, objects: &mut core::Vector<core::Rect>, reject_levels: &mut core::Vector<i32>, level_weights: &mut core::Vector<f64>, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size, output_reject_levels: bool) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_vectorLdoubleGR_double_int_int_Size_Size_bool(self.as_raw_mut_BaseCascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_mut_VectorOfRect(), reject_levels.as_raw_mut_VectorOfi32(), level_weights.as_raw_mut_VectorOff64(), scale_factor, min_neighbors, flags, &min_size, &max_size, output_reject_levels, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getOldCascade()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:205
	// ("cv::BaseCascadeClassifier::getOldCascade", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_old_cascade(&mut self) -> Result<*mut c_void> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BaseCascadeClassifier_getOldCascade(self.as_raw_mut_BaseCascadeClassifier(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaskGenerator(const Ptr<MaskGenerator> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:214
	// ("cv::BaseCascadeClassifier::setMaskGenerator", vec![(pred!(mut, ["maskGenerator"], ["const cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*"]), _)]),
	#[inline]
	fn set_mask_generator(&mut self, mask_generator: &core::Ptr<crate::objdetect::BaseCascadeClassifier_MaskGenerator>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BaseCascadeClassifier_setMaskGenerator_const_PtrLMaskGeneratorGR(self.as_raw_mut_BaseCascadeClassifier(), mask_generator.as_raw_PtrOfBaseCascadeClassifier_MaskGenerator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaskGenerator()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:215
	// ("cv::BaseCascadeClassifier::getMaskGenerator", vec![(pred!(mut, [], []), _)]),
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

// BaseCascadeClassifier /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:174
pub struct BaseCascadeClassifier {
	ptr: *mut c_void,
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

boxed_ref! { BaseCascadeClassifier, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::objdetect::BaseCascadeClassifierTraitConst for BaseCascadeClassifier {
	#[inline] fn as_raw_BaseCascadeClassifier(&self) -> *const c_void { self.as_raw() }
}

impl crate::objdetect::BaseCascadeClassifierTrait for BaseCascadeClassifier {
	#[inline] fn as_raw_mut_BaseCascadeClassifier(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BaseCascadeClassifier, crate::objdetect::BaseCascadeClassifierTraitConst, as_raw_BaseCascadeClassifier, crate::objdetect::BaseCascadeClassifierTrait, as_raw_mut_BaseCascadeClassifier }

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
// MaskGenerator /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:207
pub trait BaseCascadeClassifier_MaskGeneratorTraitConst {
	fn as_raw_BaseCascadeClassifier_MaskGenerator(&self) -> *const c_void;

}

/// Mutable methods for [crate::objdetect::BaseCascadeClassifier_MaskGenerator]
pub trait BaseCascadeClassifier_MaskGeneratorTrait: crate::objdetect::BaseCascadeClassifier_MaskGeneratorTraitConst {
	fn as_raw_mut_BaseCascadeClassifier_MaskGenerator(&mut self) -> *mut c_void;

	// generateMask(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:211
	// ("cv::BaseCascadeClassifier::MaskGenerator::generateMask", vec![(pred!(mut, ["src"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn generate_mask(&mut self, src: &impl core::MatTraitConst) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BaseCascadeClassifier_MaskGenerator_generateMask_const_MatR(self.as_raw_mut_BaseCascadeClassifier_MaskGenerator(), src.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	// initializeMask(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:212
	// ("cv::BaseCascadeClassifier::MaskGenerator::initializeMask", vec![(pred!(mut, ["unnamed"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn initialize_mask(&mut self, unnamed: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BaseCascadeClassifier_MaskGenerator_initializeMask_const_MatR(self.as_raw_mut_BaseCascadeClassifier_MaskGenerator(), unnamed.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// MaskGenerator /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:207
pub struct BaseCascadeClassifier_MaskGenerator {
	ptr: *mut c_void,
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

boxed_ref! { BaseCascadeClassifier_MaskGenerator, crate::objdetect::BaseCascadeClassifier_MaskGeneratorTraitConst, as_raw_BaseCascadeClassifier_MaskGenerator, crate::objdetect::BaseCascadeClassifier_MaskGeneratorTrait, as_raw_mut_BaseCascadeClassifier_MaskGenerator }

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
// CascadeClassifier /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:224
pub trait CascadeClassifierTraitConst {
	fn as_raw_CascadeClassifier(&self) -> *const c_void;

	/// Checks whether the classifier has been loaded.
	// empty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:236
	// ("cv::CascadeClassifier::empty", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CascadeClassifier_empty_const(self.as_raw_CascadeClassifier(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// isOldFormatCascade()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:327
	// ("cv::CascadeClassifier::isOldFormatCascade", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn is_old_format_cascade(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CascadeClassifier_isOldFormatCascade_const(self.as_raw_CascadeClassifier(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getOriginalWindowSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:328
	// ("cv::CascadeClassifier::getOriginalWindowSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_original_window_size(&self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CascadeClassifier_getOriginalWindowSize_const(self.as_raw_CascadeClassifier(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getFeatureType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:329
	// ("cv::CascadeClassifier::getFeatureType", vec![(pred!(const, [], []), _)]),
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

	// cv::CascadeClassifier::cc() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:337
	// ("cv::CascadeClassifier::cc", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn cc(&mut self) -> core::Ptr<crate::objdetect::BaseCascadeClassifier> {
		let ret = unsafe { sys::cv_CascadeClassifier_propCc(self.as_raw_mut_CascadeClassifier()) };
		let ret = unsafe { core::Ptr::<crate::objdetect::BaseCascadeClassifier>::opencv_from_extern(ret) };
		ret
	}

	// cv::CascadeClassifier::setCc(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:337
	// ("cv::CascadeClassifier::setCc", vec![(pred!(mut, ["val"], ["const cv::Ptr<cv::BaseCascadeClassifier>"]), _)]),
	#[inline]
	fn set_cc(&mut self, val: core::Ptr<crate::objdetect::BaseCascadeClassifier>) {
		let ret = unsafe { sys::cv_CascadeClassifier_propCc_const_PtrLBaseCascadeClassifierG(self.as_raw_mut_CascadeClassifier(), val.as_raw_PtrOfBaseCascadeClassifier()) };
		ret
	}

	/// Loads a classifier from a file.
	///
	/// ## Parameters
	/// * filename: Name of the file from which the classifier is loaded. The file may contain an old
	/// HAAR classifier trained by the haartraining application or a new cascade classifier trained by the
	/// traincascade application.
	// load(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:243
	// ("cv::CascadeClassifier::load", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
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
	/// Note: The file may contain a new cascade classifier (trained traincascade application) only.
	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:248
	// ("cv::CascadeClassifier::read", vec![(pred!(mut, ["node"], ["const cv::FileNode*"]), _)]),
	#[inline]
	fn read(&mut self, node: &impl core::FileNodeTraitConst) -> Result<bool> {
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
	/// The function is parallelized with the TBB library.
	///
	///
	/// Note:
	///    *   (Python) A face detection example using cascade classifiers can be found at
	///        opencv_source_code/samples/python/facedetect.py
	///
	/// ## C++ default parameters
	/// * scale_factor: 1.1
	/// * min_neighbors: 3
	/// * flags: 0
	/// * min_size: Size()
	/// * max_size: Size()
	// detectMultiScale(InputArray, std::vector<Rect> &, double, int, int, Size, Size)(InputArray, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:270
	// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "double", "int", "int", "cv::Size", "cv::Size"]), _)]),
	#[inline]
	fn detect_multi_scale(&mut self, image: &impl ToInputArray, objects: &mut core::Vector<core::Rect>, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_double_int_int_Size_Size(self.as_raw_mut_CascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_mut_VectorOfRect(), scale_factor, min_neighbors, flags, &min_size, &max_size, ocvrs_return.as_mut_ptr()) };
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
	/// The function is parallelized with the TBB library.
	///
	///
	/// Note:
	///    *   (Python) A face detection example using cascade classifiers can be found at
	///        opencv_source_code/samples/python/facedetect.py
	///
	/// ## Note
	/// This alternative version of [CascadeClassifierTrait::detect_multi_scale] function uses the following default values for its arguments:
	/// * scale_factor: 1.1
	/// * min_neighbors: 3
	/// * flags: 0
	/// * min_size: Size()
	/// * max_size: Size()
	// cv::CascadeClassifier::detectMultiScale(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:270
	// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects"], ["const cv::_InputArray*", "std::vector<cv::Rect>*"]), _)]),
	#[inline]
	fn detect_multi_scale_def(&mut self, image: &impl ToInputArray, objects: &mut core::Vector<core::Rect>) -> Result<()> {
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
	/// The function is parallelized with the TBB library.
	///
	///
	/// Note:
	///    *   (Python) A face detection example using cascade classifiers can be found at
	///        opencv_source_code/samples/python/facedetect.py
	///
	/// ## Overloaded parameters
	///
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
	/// ## C++ default parameters
	/// * scale_factor: 1.1
	/// * min_neighbors: 3
	/// * flags: 0
	/// * min_size: Size()
	/// * max_size: Size()
	// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<int> &, double, int, int, Size, Size)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:292
	// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "numDetections", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "double", "int", "int", "cv::Size", "cv::Size"]), _)]),
	#[inline]
	fn detect_multi_scale2(&mut self, image: &impl ToInputArray, objects: &mut core::Vector<core::Rect>, num_detections: &mut core::Vector<i32>, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_double_int_int_Size_Size(self.as_raw_mut_CascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_mut_VectorOfRect(), num_detections.as_raw_mut_VectorOfi32(), scale_factor, min_neighbors, flags, &min_size, &max_size, ocvrs_return.as_mut_ptr()) };
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
	/// This alternative version of [CascadeClassifierTrait::detect_multi_scale2] function uses the following default values for its arguments:
	/// * scale_factor: 1.1
	/// * min_neighbors: 3
	/// * flags: 0
	/// * min_size: Size()
	/// * max_size: Size()
	// cv::CascadeClassifier::detectMultiScale(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:292
	// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "numDetections"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*"]), _)]),
	#[inline]
	fn detect_multi_scale2_def(&mut self, image: &impl ToInputArray, objects: &mut core::Vector<core::Rect>, num_detections: &mut core::Vector<i32>) -> Result<()> {
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
	/// The function is parallelized with the TBB library.
	///
	///
	/// Note:
	///    *   (Python) A face detection example using cascade classifiers can be found at
	///        opencv_source_code/samples/python/facedetect.py
	///
	/// ## Overloaded parameters
	///
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
	/// ## C++ default parameters
	/// * scale_factor: 1.1
	/// * min_neighbors: 3
	/// * flags: 0
	/// * min_size: Size()
	/// * max_size: Size()
	/// * output_reject_levels: false
	// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<int> &, std::vector<double> &, double, int, int, Size, Size, bool)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:317
	// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "rejectLevels", "levelWeights", "scaleFactor", "minNeighbors", "flags", "minSize", "maxSize", "outputRejectLevels"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*", "double", "int", "int", "cv::Size", "cv::Size", "bool"]), _)]),
	#[inline]
	fn detect_multi_scale3(&mut self, image: &impl ToInputArray, objects: &mut core::Vector<core::Rect>, reject_levels: &mut core::Vector<i32>, level_weights: &mut core::Vector<f64>, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size, output_reject_levels: bool) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_vectorLdoubleGR_double_int_int_Size_Size_bool(self.as_raw_mut_CascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_mut_VectorOfRect(), reject_levels.as_raw_mut_VectorOfi32(), level_weights.as_raw_mut_VectorOff64(), scale_factor, min_neighbors, flags, &min_size, &max_size, output_reject_levels, ocvrs_return.as_mut_ptr()) };
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
	/// This alternative version of [CascadeClassifierTrait::detect_multi_scale3] function uses the following default values for its arguments:
	/// * scale_factor: 1.1
	/// * min_neighbors: 3
	/// * flags: 0
	/// * min_size: Size()
	/// * max_size: Size()
	/// * output_reject_levels: false
	// cv::CascadeClassifier::detectMultiScale(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:317
	// ("cv::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "rejectLevels", "levelWeights"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<int>*", "std::vector<double>*"]), _)]),
	#[inline]
	fn detect_multi_scale3_def(&mut self, image: &impl ToInputArray, objects: &mut core::Vector<core::Rect>, reject_levels: &mut core::Vector<i32>, level_weights: &mut core::Vector<f64>) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLintGR_vectorLdoubleGR(self.as_raw_mut_CascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_mut_VectorOfRect(), reject_levels.as_raw_mut_VectorOfi32(), level_weights.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getOldCascade()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:330
	// ("cv::CascadeClassifier::getOldCascade", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_old_cascade(&mut self) -> Result<*mut c_void> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CascadeClassifier_getOldCascade(self.as_raw_mut_CascadeClassifier(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaskGenerator(const Ptr<BaseCascadeClassifier::MaskGenerator> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:334
	// ("cv::CascadeClassifier::setMaskGenerator", vec![(pred!(mut, ["maskGenerator"], ["const cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*"]), _)]),
	#[inline]
	fn set_mask_generator(&mut self, mask_generator: &core::Ptr<crate::objdetect::BaseCascadeClassifier_MaskGenerator>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CascadeClassifier_setMaskGenerator_const_PtrLMaskGeneratorGR(self.as_raw_mut_CascadeClassifier(), mask_generator.as_raw_PtrOfBaseCascadeClassifier_MaskGenerator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaskGenerator()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:335
	// ("cv::CascadeClassifier::getMaskGenerator", vec![(pred!(mut, [], []), _)]),
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
// CascadeClassifier /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:224
pub struct CascadeClassifier {
	ptr: *mut c_void,
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

boxed_ref! { CascadeClassifier, crate::objdetect::CascadeClassifierTraitConst, as_raw_CascadeClassifier, crate::objdetect::CascadeClassifierTrait, as_raw_mut_CascadeClassifier }

impl CascadeClassifier {
	// CascadeClassifier()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:227
	// ("cv::CascadeClassifier::CascadeClassifier", vec![(pred!(mut, [], []), _)]),
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
	// CascadeClassifier(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:232
	// ("cv::CascadeClassifier::CascadeClassifier", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
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

	// convert(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:332
	// ("cv::CascadeClassifier::convert", vec![(pred!(mut, ["oldcascade", "newcascade"], ["const cv::String*", "const cv::String*"]), _)]),
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
// DetectionBasedTracker /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:61
pub trait DetectionBasedTrackerTraitConst {
	fn as_raw_DetectionBasedTracker(&self) -> *const c_void;

	// getParameters()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:135
	// ("cv::DetectionBasedTracker::getParameters", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_parameters(&self) -> Result<crate::objdetect::DetectionBasedTracker_Parameters> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DetectionBasedTracker_getParameters_const(self.as_raw_DetectionBasedTracker(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::objdetect::DetectionBasedTracker_Parameters::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getObjects(std::vector<cv::Rect> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:139
	// ("cv::DetectionBasedTracker::getObjects", vec![(pred!(const, ["result"], ["std::vector<cv::Rect>*"]), _)]),
	#[inline]
	fn get_objects(&self, result: &mut core::Vector<core::Rect>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DetectionBasedTracker_getObjects_const_vectorLRectGR(self.as_raw_DetectionBasedTracker(), result.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getObjects(std::vector<Object> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:140
	// ("cv::DetectionBasedTracker::getObjects", vec![(pred!(const, ["result"], ["std::vector<cv::DetectionBasedTracker::Object>*"]), _)]),
	#[inline]
	fn get_objects_1(&self, result: &mut core::Vector<crate::objdetect::DetectionBasedTracker_Object>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DetectionBasedTracker_getObjects_const_vectorLObjectGR(self.as_raw_DetectionBasedTracker(), result.as_raw_mut_VectorOfDetectionBasedTracker_Object(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getObjects(std::vector<ExtObject> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:159
	// ("cv::DetectionBasedTracker::getObjects", vec![(pred!(const, ["result"], ["std::vector<cv::DetectionBasedTracker::ExtObject>*"]), _)]),
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

	// run()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:128
	// ("cv::DetectionBasedTracker::run", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn run(&mut self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DetectionBasedTracker_run(self.as_raw_mut_DetectionBasedTracker(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// stop()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:129
	// ("cv::DetectionBasedTracker::stop", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn stop(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DetectionBasedTracker_stop(self.as_raw_mut_DetectionBasedTracker(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// resetTracking()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:130
	// ("cv::DetectionBasedTracker::resetTracking", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn reset_tracking(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DetectionBasedTracker_resetTracking(self.as_raw_mut_DetectionBasedTracker(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// process(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:132
	// ("cv::DetectionBasedTracker::process", vec![(pred!(mut, ["imageGray"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn process(&mut self, image_gray: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DetectionBasedTracker_process_const_MatR(self.as_raw_mut_DetectionBasedTracker(), image_gray.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setParameters(const Parameters &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:134
	// ("cv::DetectionBasedTracker::setParameters", vec![(pred!(mut, ["params"], ["const cv::DetectionBasedTracker::Parameters*"]), _)]),
	#[inline]
	fn set_parameters(&mut self, params: &impl crate::objdetect::DetectionBasedTracker_ParametersTraitConst) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DetectionBasedTracker_setParameters_const_ParametersR(self.as_raw_mut_DetectionBasedTracker(), params.as_raw_DetectionBasedTracker_Parameters(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// addObject(const cv::Rect &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:162
	// ("cv::DetectionBasedTracker::addObject", vec![(pred!(mut, ["location"], ["const cv::Rect*"]), _)]),
	#[inline]
	fn add_object(&mut self, location: core::Rect) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DetectionBasedTracker_addObject_const_RectR(self.as_raw_mut_DetectionBasedTracker(), &location, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// DetectionBasedTracker /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:61
pub struct DetectionBasedTracker {
	ptr: *mut c_void,
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

boxed_ref! { DetectionBasedTracker, crate::objdetect::DetectionBasedTrackerTraitConst, as_raw_DetectionBasedTracker, crate::objdetect::DetectionBasedTrackerTrait, as_raw_mut_DetectionBasedTracker }

impl DetectionBasedTracker {
	// DetectionBasedTracker(cv::Ptr<IDetector>, cv::Ptr<IDetector>, const Parameters &)(CppPassByVoidPtr, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:125
	// ("cv::DetectionBasedTracker::DetectionBasedTracker", vec![(pred!(mut, ["mainDetector", "trackingDetector", "params"], ["cv::Ptr<cv::DetectionBasedTracker::IDetector>", "cv::Ptr<cv::DetectionBasedTracker::IDetector>", "const cv::DetectionBasedTracker::Parameters*"]), _)]),
	#[inline]
	pub fn new(mut main_detector: core::Ptr<crate::objdetect::DetectionBasedTracker_IDetector>, mut tracking_detector: core::Ptr<crate::objdetect::DetectionBasedTracker_IDetector>, params: &impl crate::objdetect::DetectionBasedTracker_ParametersTraitConst) -> Result<crate::objdetect::DetectionBasedTracker> {
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
// ExtObject /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:149
pub trait DetectionBasedTracker_ExtObjectTraitConst {
	fn as_raw_DetectionBasedTracker_ExtObject(&self) -> *const c_void;

	// cv::DetectionBasedTracker::ExtObject::id() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:151
	// ("cv::DetectionBasedTracker::ExtObject::id", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn id(&self) -> i32 {
		let ret = unsafe { sys::cv_DetectionBasedTracker_ExtObject_propId_const(self.as_raw_DetectionBasedTracker_ExtObject()) };
		ret
	}

	// cv::DetectionBasedTracker::ExtObject::location() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:152
	// ("cv::DetectionBasedTracker::ExtObject::location", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn location(&self) -> core::Rect {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DetectionBasedTracker_ExtObject_propLocation_const(self.as_raw_DetectionBasedTracker_ExtObject(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	// cv::DetectionBasedTracker::ExtObject::status() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:153
	// ("cv::DetectionBasedTracker::ExtObject::status", vec![(pred!(const, [], []), _)]),
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

	// cv::DetectionBasedTracker::ExtObject::setId(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:151
	// ("cv::DetectionBasedTracker::ExtObject::setId", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_id(&mut self, val: i32) {
		let ret = unsafe { sys::cv_DetectionBasedTracker_ExtObject_propId_const_int(self.as_raw_mut_DetectionBasedTracker_ExtObject(), val) };
		ret
	}

	// cv::DetectionBasedTracker::ExtObject::setLocation(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:152
	// ("cv::DetectionBasedTracker::ExtObject::setLocation", vec![(pred!(mut, ["val"], ["const cv::Rect"]), _)]),
	#[inline]
	fn set_location(&mut self, val: core::Rect) {
		let ret = unsafe { sys::cv_DetectionBasedTracker_ExtObject_propLocation_const_Rect(self.as_raw_mut_DetectionBasedTracker_ExtObject(), &val) };
		ret
	}

	// cv::DetectionBasedTracker::ExtObject::setStatus(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:153
	// ("cv::DetectionBasedTracker::ExtObject::setStatus", vec![(pred!(mut, ["val"], ["const cv::DetectionBasedTracker::ObjectStatus"]), _)]),
	#[inline]
	fn set_status(&mut self, val: crate::objdetect::DetectionBasedTracker_ObjectStatus) {
		let ret = unsafe { sys::cv_DetectionBasedTracker_ExtObject_propStatus_const_ObjectStatus(self.as_raw_mut_DetectionBasedTracker_ExtObject(), val) };
		ret
	}

}

// ExtObject /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:149
pub struct DetectionBasedTracker_ExtObject {
	ptr: *mut c_void,
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

boxed_ref! { DetectionBasedTracker_ExtObject, crate::objdetect::DetectionBasedTracker_ExtObjectTraitConst, as_raw_DetectionBasedTracker_ExtObject, crate::objdetect::DetectionBasedTracker_ExtObjectTrait, as_raw_mut_DetectionBasedTracker_ExtObject }

impl DetectionBasedTracker_ExtObject {
	// ExtObject(int, cv::Rect, ObjectStatus)(Primitive, SimpleClass, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:154
	// ("cv::DetectionBasedTracker::ExtObject::ExtObject", vec![(pred!(mut, ["_id", "_location", "_status"], ["int", "cv::Rect", "cv::DetectionBasedTracker::ObjectStatus"]), _)]),
	#[inline]
	pub fn new(_id: i32, _location: core::Rect, _status: crate::objdetect::DetectionBasedTracker_ObjectStatus) -> Result<crate::objdetect::DetectionBasedTracker_ExtObject> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DetectionBasedTracker_ExtObject_ExtObject_int_Rect_ObjectStatus(_id, &_location, _status, ocvrs_return.as_mut_ptr()) };
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
// IDetector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:72
pub trait DetectionBasedTracker_IDetectorTraitConst {
	fn as_raw_DetectionBasedTracker_IDetector(&self) -> *const c_void;

	// getMinObjectSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:92
	// ("cv::DetectionBasedTracker::IDetector::getMinObjectSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_object_size(&self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DetectionBasedTracker_IDetector_getMinObjectSize_const(self.as_raw_DetectionBasedTracker_IDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxObjectSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:96
	// ("cv::DetectionBasedTracker::IDetector::getMaxObjectSize", vec![(pred!(const, [], []), _)]),
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

	// detect(const cv::Mat &, std::vector<cv::Rect> &)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:82
	// ("cv::DetectionBasedTracker::IDetector::detect", vec![(pred!(mut, ["image", "objects"], ["const cv::Mat*", "std::vector<cv::Rect>*"]), _)]),
	#[inline]
	fn detect(&mut self, image: &impl core::MatTraitConst, objects: &mut core::Vector<core::Rect>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DetectionBasedTracker_IDetector_detect_const_MatR_vectorLRectGR(self.as_raw_mut_DetectionBasedTracker_IDetector(), image.as_raw_Mat(), objects.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinObjectSize(const cv::Size &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:84
	// ("cv::DetectionBasedTracker::IDetector::setMinObjectSize", vec![(pred!(mut, ["min"], ["const cv::Size*"]), _)]),
	#[inline]
	fn set_min_object_size(&mut self, min: core::Size) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DetectionBasedTracker_IDetector_setMinObjectSize_const_SizeR(self.as_raw_mut_DetectionBasedTracker_IDetector(), &min, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxObjectSize(const cv::Size &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:88
	// ("cv::DetectionBasedTracker::IDetector::setMaxObjectSize", vec![(pred!(mut, ["max"], ["const cv::Size*"]), _)]),
	#[inline]
	fn set_max_object_size(&mut self, max: core::Size) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DetectionBasedTracker_IDetector_setMaxObjectSize_const_SizeR(self.as_raw_mut_DetectionBasedTracker_IDetector(), &max, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:100
	// ("cv::DetectionBasedTracker::IDetector::getScaleFactor", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_scale_factor(&mut self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DetectionBasedTracker_IDetector_getScaleFactor(self.as_raw_mut_DetectionBasedTracker_IDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setScaleFactor(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:104
	// ("cv::DetectionBasedTracker::IDetector::setScaleFactor", vec![(pred!(mut, ["value"], ["float"]), _)]),
	#[inline]
	fn set_scale_factor(&mut self, value: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DetectionBasedTracker_IDetector_setScaleFactor_float(self.as_raw_mut_DetectionBasedTracker_IDetector(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMinNeighbours()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:108
	// ("cv::DetectionBasedTracker::IDetector::getMinNeighbours", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_min_neighbours(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DetectionBasedTracker_IDetector_getMinNeighbours(self.as_raw_mut_DetectionBasedTracker_IDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinNeighbours(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:112
	// ("cv::DetectionBasedTracker::IDetector::setMinNeighbours", vec![(pred!(mut, ["value"], ["int"]), _)]),
	#[inline]
	fn set_min_neighbours(&mut self, value: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DetectionBasedTracker_IDetector_setMinNeighbours_int(self.as_raw_mut_DetectionBasedTracker_IDetector(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// IDetector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:72
pub struct DetectionBasedTracker_IDetector {
	ptr: *mut c_void,
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

boxed_ref! { DetectionBasedTracker_IDetector, crate::objdetect::DetectionBasedTracker_IDetectorTraitConst, as_raw_DetectionBasedTracker_IDetector, crate::objdetect::DetectionBasedTracker_IDetectorTrait, as_raw_mut_DetectionBasedTracker_IDetector }

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
// Parameters /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:64
pub trait DetectionBasedTracker_ParametersTraitConst {
	fn as_raw_DetectionBasedTracker_Parameters(&self) -> *const c_void;

	// cv::DetectionBasedTracker::Parameters::maxTrackLifetime() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:66
	// ("cv::DetectionBasedTracker::Parameters::maxTrackLifetime", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn max_track_lifetime(&self) -> i32 {
		let ret = unsafe { sys::cv_DetectionBasedTracker_Parameters_propMaxTrackLifetime_const(self.as_raw_DetectionBasedTracker_Parameters()) };
		ret
	}

	// cv::DetectionBasedTracker::Parameters::minDetectionPeriod() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:67
	// ("cv::DetectionBasedTracker::Parameters::minDetectionPeriod", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn min_detection_period(&self) -> i32 {
		let ret = unsafe { sys::cv_DetectionBasedTracker_Parameters_propMinDetectionPeriod_const(self.as_raw_DetectionBasedTracker_Parameters()) };
		ret
	}

}

/// Mutable methods for [crate::objdetect::DetectionBasedTracker_Parameters]
pub trait DetectionBasedTracker_ParametersTrait: crate::objdetect::DetectionBasedTracker_ParametersTraitConst {
	fn as_raw_mut_DetectionBasedTracker_Parameters(&mut self) -> *mut c_void;

	// cv::DetectionBasedTracker::Parameters::setMaxTrackLifetime(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:66
	// ("cv::DetectionBasedTracker::Parameters::setMaxTrackLifetime", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_max_track_lifetime(&mut self, val: i32) {
		let ret = unsafe { sys::cv_DetectionBasedTracker_Parameters_propMaxTrackLifetime_const_int(self.as_raw_mut_DetectionBasedTracker_Parameters(), val) };
		ret
	}

	// cv::DetectionBasedTracker::Parameters::setMinDetectionPeriod(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:67
	// ("cv::DetectionBasedTracker::Parameters::setMinDetectionPeriod", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_min_detection_period(&mut self, val: i32) {
		let ret = unsafe { sys::cv_DetectionBasedTracker_Parameters_propMinDetectionPeriod_const_int(self.as_raw_mut_DetectionBasedTracker_Parameters(), val) };
		ret
	}

}

// Parameters /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:64
pub struct DetectionBasedTracker_Parameters {
	ptr: *mut c_void,
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

boxed_ref! { DetectionBasedTracker_Parameters, crate::objdetect::DetectionBasedTracker_ParametersTraitConst, as_raw_DetectionBasedTracker_Parameters, crate::objdetect::DetectionBasedTracker_ParametersTrait, as_raw_mut_DetectionBasedTracker_Parameters }

impl DetectionBasedTracker_Parameters {
	// Parameters()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect/detection_based_tracker.hpp:69
	// ("cv::DetectionBasedTracker::Parameters::Parameters", vec![(pred!(mut, [], []), _)]),
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
// DetectionROI /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:345
pub trait DetectionROITraitConst {
	fn as_raw_DetectionROI(&self) -> *const c_void;

	/// scale(size) of the bounding box
	// cv::DetectionROI::scale() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:348
	// ("cv::DetectionROI::scale", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn scale(&self) -> f64 {
		let ret = unsafe { sys::cv_DetectionROI_propScale_const(self.as_raw_DetectionROI()) };
		ret
	}

	/// set of requested locations to be evaluated
	// cv::DetectionROI::locations() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:350
	// ("cv::DetectionROI::locations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn locations(&self) -> core::Vector<core::Point> {
		let ret = unsafe { sys::cv_DetectionROI_propLocations_const(self.as_raw_DetectionROI()) };
		let ret = unsafe { core::Vector::<core::Point>::opencv_from_extern(ret) };
		ret
	}

	/// vector that will contain confidence values for each location
	// cv::DetectionROI::confidences() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:352
	// ("cv::DetectionROI::confidences", vec![(pred!(const, [], []), _)]),
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
	// cv::DetectionROI::setScale(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:348
	// ("cv::DetectionROI::setScale", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_scale(&mut self, val: f64) {
		let ret = unsafe { sys::cv_DetectionROI_propScale_const_double(self.as_raw_mut_DetectionROI(), val) };
		ret
	}

	/// set of requested locations to be evaluated
	// cv::DetectionROI::setLocations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:350
	// ("cv::DetectionROI::setLocations", vec![(pred!(mut, ["val"], ["const std::vector<cv::Point>"]), _)]),
	#[inline]
	fn set_locations(&mut self, val: core::Vector<core::Point>) {
		let ret = unsafe { sys::cv_DetectionROI_propLocations_const_vectorLPointG(self.as_raw_mut_DetectionROI(), val.as_raw_VectorOfPoint()) };
		ret
	}

	/// vector that will contain confidence values for each location
	// cv::DetectionROI::setConfidences(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:352
	// ("cv::DetectionROI::setConfidences", vec![(pred!(mut, ["val"], ["const std::vector<double>"]), _)]),
	#[inline]
	fn set_confidences(&mut self, val: core::Vector<f64>) {
		let ret = unsafe { sys::cv_DetectionROI_propConfidences_const_vectorLdoubleG(self.as_raw_mut_DetectionROI(), val.as_raw_VectorOff64()) };
		ret
	}

}

/// struct for detection region of interest (ROI)
// DetectionROI /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:345
pub struct DetectionROI {
	ptr: *mut c_void,
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

boxed_ref! { DetectionROI, crate::objdetect::DetectionROITraitConst, as_raw_DetectionROI, crate::objdetect::DetectionROITrait, as_raw_mut_DetectionROI }

impl DetectionROI {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_DetectionROI_defaultNew_const()) }
	}

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

impl Default for DetectionROI {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::objdetect::HOGDescriptor]
// HOGDescriptor /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:372
pub trait HOGDescriptorTraitConst {
	fn as_raw_HOGDescriptor(&self) -> *const c_void;

	/// Detection window size. Align to block size and block stride. Default value is Size(64,128).
	// cv::HOGDescriptor::winSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:589
	// ("cv::HOGDescriptor::winSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn win_size(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HOGDescriptor_propWinSize_const(self.as_raw_HOGDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// Block size in pixels. Align to cell size. Default value is Size(16,16).
	// cv::HOGDescriptor::blockSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:592
	// ("cv::HOGDescriptor::blockSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn block_size(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HOGDescriptor_propBlockSize_const(self.as_raw_HOGDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// Block stride. It must be a multiple of cell size. Default value is Size(8,8).
	// cv::HOGDescriptor::blockStride() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:595
	// ("cv::HOGDescriptor::blockStride", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn block_stride(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HOGDescriptor_propBlockStride_const(self.as_raw_HOGDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// Cell size. Default value is Size(8,8).
	// cv::HOGDescriptor::cellSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:598
	// ("cv::HOGDescriptor::cellSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn cell_size(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HOGDescriptor_propCellSize_const(self.as_raw_HOGDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// Number of bins used in the calculation of histogram of gradients. Default value is 9.
	// cv::HOGDescriptor::nbins() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:601
	// ("cv::HOGDescriptor::nbins", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn nbins(&self) -> i32 {
		let ret = unsafe { sys::cv_HOGDescriptor_propNbins_const(self.as_raw_HOGDescriptor()) };
		ret
	}

	/// not documented
	// cv::HOGDescriptor::derivAperture() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:604
	// ("cv::HOGDescriptor::derivAperture", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn deriv_aperture(&self) -> i32 {
		let ret = unsafe { sys::cv_HOGDescriptor_propDerivAperture_const(self.as_raw_HOGDescriptor()) };
		ret
	}

	/// Gaussian smoothing window parameter.
	// cv::HOGDescriptor::winSigma() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:607
	// ("cv::HOGDescriptor::winSigma", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn win_sigma(&self) -> f64 {
		let ret = unsafe { sys::cv_HOGDescriptor_propWinSigma_const(self.as_raw_HOGDescriptor()) };
		ret
	}

	/// histogramNormType
	// cv::HOGDescriptor::histogramNormType() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:610
	// ("cv::HOGDescriptor::histogramNormType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn histogram_norm_type(&self) -> i32 {
		let ret = unsafe { sys::cv_HOGDescriptor_propHistogramNormType_const(self.as_raw_HOGDescriptor()) };
		ret
	}

	/// L2-Hys normalization method shrinkage.
	// cv::HOGDescriptor::L2HysThreshold() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:613
	// ("cv::HOGDescriptor::L2HysThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn l2_hys_threshold(&self) -> f64 {
		let ret = unsafe { sys::cv_HOGDescriptor_propL2HysThreshold_const(self.as_raw_HOGDescriptor()) };
		ret
	}

	/// Flag to specify whether the gamma correction preprocessing is required or not.
	// cv::HOGDescriptor::gammaCorrection() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:616
	// ("cv::HOGDescriptor::gammaCorrection", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn gamma_correction(&self) -> bool {
		let ret = unsafe { sys::cv_HOGDescriptor_propGammaCorrection_const(self.as_raw_HOGDescriptor()) };
		ret
	}

	/// coefficients for the linear SVM classifier.
	// cv::HOGDescriptor::svmDetector() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:619
	// ("cv::HOGDescriptor::svmDetector", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn svm_detector(&self) -> core::Vector<f32> {
		let ret = unsafe { sys::cv_HOGDescriptor_propSvmDetector_const(self.as_raw_HOGDescriptor()) };
		let ret = unsafe { core::Vector::<f32>::opencv_from_extern(ret) };
		ret
	}

	/// coefficients for the linear SVM classifier used when OpenCL is enabled
	// cv::HOGDescriptor::oclSvmDetector() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:622
	// ("cv::HOGDescriptor::oclSvmDetector", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn ocl_svm_detector(&self) -> core::UMat {
		let ret = unsafe { sys::cv_HOGDescriptor_propOclSvmDetector_const(self.as_raw_HOGDescriptor()) };
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		ret
	}

	/// not documented
	// cv::HOGDescriptor::free_coef() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:625
	// ("cv::HOGDescriptor::free_coef", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn free_coef(&self) -> f32 {
		let ret = unsafe { sys::cv_HOGDescriptor_propFree_coef_const(self.as_raw_HOGDescriptor()) };
		ret
	}

	/// Maximum number of detection window increases. Default value is 64
	// cv::HOGDescriptor::nlevels() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:628
	// ("cv::HOGDescriptor::nlevels", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn nlevels(&self) -> i32 {
		let ret = unsafe { sys::cv_HOGDescriptor_propNlevels_const(self.as_raw_HOGDescriptor()) };
		ret
	}

	/// Indicates signed gradient will be used or not
	// cv::HOGDescriptor::signedGradient() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:631
	// ("cv::HOGDescriptor::signedGradient", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn signed_gradient(&self) -> bool {
		let ret = unsafe { sys::cv_HOGDescriptor_propSignedGradient_const(self.as_raw_HOGDescriptor()) };
		ret
	}

	/// Returns the number of coefficients required for the classification.
	// getDescriptorSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:438
	// ("cv::HOGDescriptor::getDescriptorSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_descriptor_size(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HOGDescriptor_getDescriptorSize_const(self.as_raw_HOGDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Checks if detector size equal to descriptor size.
	// checkDetectorSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:442
	// ("cv::HOGDescriptor::checkDetectorSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn check_detector_size(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HOGDescriptor_checkDetectorSize_const(self.as_raw_HOGDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns winSigma value
	// getWinSigma()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:446
	// ("cv::HOGDescriptor::getWinSigma", vec![(pred!(const, [], []), _)]),
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
	// write(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:464
	// ("cv::HOGDescriptor::write", vec![(pred!(const, ["fs", "objname"], ["cv::FileStorage*", "const cv::String*"]), _)]),
	#[inline]
	fn write(&self, fs: &mut impl core::FileStorageTrait, objname: &str) -> Result<()> {
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
	// save(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:476
	// ("cv::HOGDescriptor::save", vec![(pred!(const, ["filename", "objname"], ["const cv::String*", "const cv::String*"]), _)]),
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
	/// This alternative version of [HOGDescriptorTraitConst::save] function uses the following default values for its arguments:
	/// * objname: String()
	// cv::HOGDescriptor::save(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:476
	// ("cv::HOGDescriptor::save", vec![(pred!(const, ["filename"], ["const cv::String*"]), _)]),
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
	// copyTo(HOGDescriptor &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:481
	// ("cv::HOGDescriptor::copyTo", vec![(pred!(const, ["c"], ["cv::HOGDescriptor*"]), _)]),
	#[inline]
	fn copy_to(&self, c: &mut impl crate::objdetect::HOGDescriptorTrait) -> Result<()> {
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
	// compute(InputArray, std::vector<float> &, Size, Size, const std::vector<Point> &)(InputArray, CppPassByVoidPtr, SimpleClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:492
	// ("cv::HOGDescriptor::compute", vec![(pred!(const, ["img", "descriptors", "winStride", "padding", "locations"], ["const cv::_InputArray*", "std::vector<float>*", "cv::Size", "cv::Size", "const std::vector<cv::Point>*"]), _)]),
	#[inline]
	fn compute(&self, img: &impl ToInputArray, descriptors: &mut core::Vector<f32>, win_stride: core::Size, padding: core::Size, locations: &core::Vector<core::Point>) -> Result<()> {
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HOGDescriptor_compute_const_const__InputArrayR_vectorLfloatGR_Size_Size_const_vectorLPointGR(self.as_raw_HOGDescriptor(), img.as_raw__InputArray(), descriptors.as_raw_mut_VectorOff32(), &win_stride, &padding, locations.as_raw_VectorOfPoint(), ocvrs_return.as_mut_ptr()) };
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
	/// This alternative version of [HOGDescriptorTraitConst::compute] function uses the following default values for its arguments:
	/// * win_stride: Size()
	/// * padding: Size()
	/// * locations: std::vector<Point>()
	// cv::HOGDescriptor::compute(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:492
	// ("cv::HOGDescriptor::compute", vec![(pred!(const, ["img", "descriptors"], ["const cv::_InputArray*", "std::vector<float>*"]), _)]),
	#[inline]
	fn compute_def(&self, img: &impl ToInputArray, descriptors: &mut core::Vector<f32>) -> Result<()> {
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
	// detect(const Mat &, std::vector<Point> &, std::vector<double> &, double, Size, Size, const std::vector<Point> &)(TraitClass, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:508
	// ("cv::HOGDescriptor::detect", vec![(pred!(const, ["img", "foundLocations", "weights", "hitThreshold", "winStride", "padding", "searchLocations"], ["const cv::Mat*", "std::vector<cv::Point>*", "std::vector<double>*", "double", "cv::Size", "cv::Size", "const std::vector<cv::Point>*"]), _)]),
	#[inline]
	fn detect_weights(&self, img: &impl core::MatTraitConst, found_locations: &mut core::Vector<core::Point>, weights: &mut core::Vector<f64>, hit_threshold: f64, win_stride: core::Size, padding: core::Size, search_locations: &core::Vector<core::Point>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HOGDescriptor_detect_const_const_MatR_vectorLPointGR_vectorLdoubleGR_double_Size_Size_const_vectorLPointGR(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), found_locations.as_raw_mut_VectorOfPoint(), weights.as_raw_mut_VectorOff64(), hit_threshold, &win_stride, &padding, search_locations.as_raw_VectorOfPoint(), ocvrs_return.as_mut_ptr()) };
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
	/// This alternative version of [HOGDescriptorTraitConst::detect_weights] function uses the following default values for its arguments:
	/// * hit_threshold: 0
	/// * win_stride: Size()
	/// * padding: Size()
	/// * search_locations: std::vector<Point>()
	// cv::HOGDescriptor::detect(TraitClass, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:508
	// ("cv::HOGDescriptor::detect", vec![(pred!(const, ["img", "foundLocations", "weights"], ["const cv::Mat*", "std::vector<cv::Point>*", "std::vector<double>*"]), _)]),
	#[inline]
	fn detect_weights_def(&self, img: &impl core::MatTraitConst, found_locations: &mut core::Vector<core::Point>, weights: &mut core::Vector<f64>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HOGDescriptor_detect_const_const_MatR_vectorLPointGR_vectorLdoubleGR(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), found_locations.as_raw_mut_VectorOfPoint(), weights.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
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
	// detect(const Mat &, std::vector<Point> &, double, Size, Size, const std::vector<Point> &)(TraitClass, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:524
	// ("cv::HOGDescriptor::detect", vec![(pred!(const, ["img", "foundLocations", "hitThreshold", "winStride", "padding", "searchLocations"], ["const cv::Mat*", "std::vector<cv::Point>*", "double", "cv::Size", "cv::Size", "const std::vector<cv::Point>*"]), _)]),
	#[inline]
	fn detect(&self, img: &impl core::MatTraitConst, found_locations: &mut core::Vector<core::Point>, hit_threshold: f64, win_stride: core::Size, padding: core::Size, search_locations: &core::Vector<core::Point>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HOGDescriptor_detect_const_const_MatR_vectorLPointGR_double_Size_Size_const_vectorLPointGR(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), found_locations.as_raw_mut_VectorOfPoint(), hit_threshold, &win_stride, &padding, search_locations.as_raw_VectorOfPoint(), ocvrs_return.as_mut_ptr()) };
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
	/// This alternative version of [HOGDescriptorTraitConst::detect] function uses the following default values for its arguments:
	/// * hit_threshold: 0
	/// * win_stride: Size()
	/// * padding: Size()
	/// * search_locations: std::vector<Point>()
	// cv::HOGDescriptor::detect(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:524
	// ("cv::HOGDescriptor::detect", vec![(pred!(const, ["img", "foundLocations"], ["const cv::Mat*", "std::vector<cv::Point>*"]), _)]),
	#[inline]
	fn detect_def(&self, img: &impl core::MatTraitConst, found_locations: &mut core::Vector<core::Point>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HOGDescriptor_detect_const_const_MatR_vectorLPointGR(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), found_locations.as_raw_mut_VectorOfPoint(), ocvrs_return.as_mut_ptr()) };
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
	// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<double> &, double, Size, Size, double, double, bool)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:544
	// ("cv::HOGDescriptor::detectMultiScale", vec![(pred!(const, ["img", "foundLocations", "foundWeights", "hitThreshold", "winStride", "padding", "scale", "groupThreshold", "useMeanshiftGrouping"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<double>*", "double", "cv::Size", "cv::Size", "double", "double", "bool"]), _)]),
	#[inline]
	fn detect_multi_scale_weights(&self, img: &impl ToInputArray, found_locations: &mut core::Vector<core::Rect>, found_weights: &mut core::Vector<f64>, hit_threshold: f64, win_stride: core::Size, padding: core::Size, scale: f64, group_threshold: f64, use_meanshift_grouping: bool) -> Result<()> {
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vectorLRectGR_vectorLdoubleGR_double_Size_Size_double_double_bool(self.as_raw_HOGDescriptor(), img.as_raw__InputArray(), found_locations.as_raw_mut_VectorOfRect(), found_weights.as_raw_mut_VectorOff64(), hit_threshold, &win_stride, &padding, scale, group_threshold, use_meanshift_grouping, ocvrs_return.as_mut_ptr()) };
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
	/// This alternative version of [HOGDescriptorTraitConst::detect_multi_scale_weights] function uses the following default values for its arguments:
	/// * hit_threshold: 0
	/// * win_stride: Size()
	/// * padding: Size()
	/// * scale: 1.05
	/// * group_threshold: 2.0
	/// * use_meanshift_grouping: false
	// cv::HOGDescriptor::detectMultiScale(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:544
	// ("cv::HOGDescriptor::detectMultiScale", vec![(pred!(const, ["img", "foundLocations", "foundWeights"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<double>*"]), _)]),
	#[inline]
	fn detect_multi_scale_weights_def(&self, img: &impl ToInputArray, found_locations: &mut core::Vector<core::Rect>, found_weights: &mut core::Vector<f64>) -> Result<()> {
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
	// detectMultiScale(InputArray, std::vector<Rect> &, double, Size, Size, double, double, bool)(InputArray, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:563
	// ("cv::HOGDescriptor::detectMultiScale", vec![(pred!(const, ["img", "foundLocations", "hitThreshold", "winStride", "padding", "scale", "groupThreshold", "useMeanshiftGrouping"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "double", "cv::Size", "cv::Size", "double", "double", "bool"]), _)]),
	#[inline]
	fn detect_multi_scale(&self, img: &impl ToInputArray, found_locations: &mut core::Vector<core::Rect>, hit_threshold: f64, win_stride: core::Size, padding: core::Size, scale: f64, group_threshold: f64, use_meanshift_grouping: bool) -> Result<()> {
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vectorLRectGR_double_Size_Size_double_double_bool(self.as_raw_HOGDescriptor(), img.as_raw__InputArray(), found_locations.as_raw_mut_VectorOfRect(), hit_threshold, &win_stride, &padding, scale, group_threshold, use_meanshift_grouping, ocvrs_return.as_mut_ptr()) };
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
	/// This alternative version of [HOGDescriptorTraitConst::detect_multi_scale] function uses the following default values for its arguments:
	/// * hit_threshold: 0
	/// * win_stride: Size()
	/// * padding: Size()
	/// * scale: 1.05
	/// * group_threshold: 2.0
	/// * use_meanshift_grouping: false
	// cv::HOGDescriptor::detectMultiScale(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:563
	// ("cv::HOGDescriptor::detectMultiScale", vec![(pred!(const, ["img", "foundLocations"], ["const cv::_InputArray*", "std::vector<cv::Rect>*"]), _)]),
	#[inline]
	fn detect_multi_scale_def(&self, img: &impl ToInputArray, found_locations: &mut core::Vector<core::Rect>) -> Result<()> {
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
	// computeGradient(const Mat &, Mat &, Mat &, Size, Size)(TraitClass, TraitClass, TraitClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:575
	// ("cv::HOGDescriptor::computeGradient", vec![(pred!(const, ["img", "grad", "angleOfs", "paddingTL", "paddingBR"], ["const cv::Mat*", "cv::Mat*", "cv::Mat*", "cv::Size", "cv::Size"]), _)]),
	#[inline]
	fn compute_gradient(&self, img: &impl core::MatTraitConst, grad: &mut impl core::MatTrait, angle_ofs: &mut impl core::MatTrait, padding_tl: core::Size, padding_br: core::Size) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HOGDescriptor_computeGradient_const_const_MatR_MatR_MatR_Size_Size(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), grad.as_raw_mut_Mat(), angle_ofs.as_raw_mut_Mat(), &padding_tl, &padding_br, ocvrs_return.as_mut_ptr()) };
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
	/// This alternative version of [HOGDescriptorTraitConst::compute_gradient] function uses the following default values for its arguments:
	/// * padding_tl: Size()
	/// * padding_br: Size()
	// cv::HOGDescriptor::computeGradient(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:575
	// ("cv::HOGDescriptor::computeGradient", vec![(pred!(const, ["img", "grad", "angleOfs"], ["const cv::Mat*", "cv::Mat*", "cv::Mat*"]), _)]),
	#[inline]
	fn compute_gradient_def(&self, img: &impl core::MatTraitConst, grad: &mut impl core::MatTrait, angle_ofs: &mut impl core::MatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HOGDescriptor_computeGradient_const_const_MatR_MatR_MatR(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), grad.as_raw_mut_Mat(), angle_ofs.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
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
	// detectROI(const cv::Mat &, const std::vector<cv::Point> &, std::vector<cv::Point> &, std::vector<double> &, double, cv::Size, cv::Size)(TraitClass, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:644
	// ("cv::HOGDescriptor::detectROI", vec![(pred!(const, ["img", "locations", "foundLocations", "confidences", "hitThreshold", "winStride", "padding"], ["const cv::Mat*", "const std::vector<cv::Point>*", "std::vector<cv::Point>*", "std::vector<double>*", "double", "cv::Size", "cv::Size"]), _)]),
	#[inline]
	fn detect_roi(&self, img: &impl core::MatTraitConst, locations: &core::Vector<core::Point>, found_locations: &mut core::Vector<core::Point>, confidences: &mut core::Vector<f64>, hit_threshold: f64, win_stride: core::Size, padding: core::Size) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HOGDescriptor_detectROI_const_const_MatR_const_vectorLPointGR_vectorLPointGR_vectorLdoubleGR_double_Size_Size(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), locations.as_raw_VectorOfPoint(), found_locations.as_raw_mut_VectorOfPoint(), confidences.as_raw_mut_VectorOff64(), hit_threshold, &win_stride, &padding, ocvrs_return.as_mut_ptr()) };
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
	/// This alternative version of [HOGDescriptorTraitConst::detect_roi] function uses the following default values for its arguments:
	/// * hit_threshold: 0
	/// * win_stride: Size()
	/// * padding: Size()
	// cv::HOGDescriptor::detectROI(TraitClass, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:644
	// ("cv::HOGDescriptor::detectROI", vec![(pred!(const, ["img", "locations", "foundLocations", "confidences"], ["const cv::Mat*", "const std::vector<cv::Point>*", "std::vector<cv::Point>*", "std::vector<double>*"]), _)]),
	#[inline]
	fn detect_roi_def(&self, img: &impl core::MatTraitConst, locations: &core::Vector<core::Point>, found_locations: &mut core::Vector<core::Point>, confidences: &mut core::Vector<f64>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HOGDescriptor_detectROI_const_const_MatR_const_vectorLPointGR_vectorLPointGR_vectorLdoubleGR(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), locations.as_raw_VectorOfPoint(), found_locations.as_raw_mut_VectorOfPoint(), confidences.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
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
	// detectMultiScaleROI(const cv::Mat &, std::vector<cv::Rect> &, std::vector<DetectionROI> &, double, int)(TraitClass, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:657
	// ("cv::HOGDescriptor::detectMultiScaleROI", vec![(pred!(const, ["img", "foundLocations", "locations", "hitThreshold", "groupThreshold"], ["const cv::Mat*", "std::vector<cv::Rect>*", "std::vector<cv::DetectionROI>*", "double", "int"]), _)]),
	#[inline]
	fn detect_multi_scale_roi(&self, img: &impl core::MatTraitConst, found_locations: &mut core::Vector<core::Rect>, locations: &mut core::Vector<crate::objdetect::DetectionROI>, hit_threshold: f64, group_threshold: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HOGDescriptor_detectMultiScaleROI_const_const_MatR_vectorLRectGR_vectorLDetectionROIGR_double_int(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), found_locations.as_raw_mut_VectorOfRect(), locations.as_raw_mut_VectorOfDetectionROI(), hit_threshold, group_threshold, ocvrs_return.as_mut_ptr()) };
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
	/// This alternative version of [HOGDescriptorTraitConst::detect_multi_scale_roi] function uses the following default values for its arguments:
	/// * hit_threshold: 0
	/// * group_threshold: 0
	// cv::HOGDescriptor::detectMultiScaleROI(TraitClass, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:657
	// ("cv::HOGDescriptor::detectMultiScaleROI", vec![(pred!(const, ["img", "foundLocations", "locations"], ["const cv::Mat*", "std::vector<cv::Rect>*", "std::vector<cv::DetectionROI>*"]), _)]),
	#[inline]
	fn detect_multi_scale_roi_def(&self, img: &impl core::MatTraitConst, found_locations: &mut core::Vector<core::Rect>, locations: &mut core::Vector<crate::objdetect::DetectionROI>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HOGDescriptor_detectMultiScaleROI_const_const_MatR_vectorLRectGR_vectorLDetectionROIGR(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), found_locations.as_raw_mut_VectorOfRect(), locations.as_raw_mut_VectorOfDetectionROI(), ocvrs_return.as_mut_ptr()) };
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
	// groupRectangles(std::vector<cv::Rect> &, std::vector<double> &, int, double)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:674
	// ("cv::HOGDescriptor::groupRectangles", vec![(pred!(const, ["rectList", "weights", "groupThreshold", "eps"], ["std::vector<cv::Rect>*", "std::vector<double>*", "int", "double"]), _)]),
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
	// cv::HOGDescriptor::setWinSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:589
	// ("cv::HOGDescriptor::setWinSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	#[inline]
	fn set_win_size(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_HOGDescriptor_propWinSize_const_Size(self.as_raw_mut_HOGDescriptor(), &val) };
		ret
	}

	/// Block size in pixels. Align to cell size. Default value is Size(16,16).
	// cv::HOGDescriptor::setBlockSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:592
	// ("cv::HOGDescriptor::setBlockSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	#[inline]
	fn set_block_size(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_HOGDescriptor_propBlockSize_const_Size(self.as_raw_mut_HOGDescriptor(), &val) };
		ret
	}

	/// Block stride. It must be a multiple of cell size. Default value is Size(8,8).
	// cv::HOGDescriptor::setBlockStride(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:595
	// ("cv::HOGDescriptor::setBlockStride", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	#[inline]
	fn set_block_stride(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_HOGDescriptor_propBlockStride_const_Size(self.as_raw_mut_HOGDescriptor(), &val) };
		ret
	}

	/// Cell size. Default value is Size(8,8).
	// cv::HOGDescriptor::setCellSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:598
	// ("cv::HOGDescriptor::setCellSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	#[inline]
	fn set_cell_size(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_HOGDescriptor_propCellSize_const_Size(self.as_raw_mut_HOGDescriptor(), &val) };
		ret
	}

	/// Number of bins used in the calculation of histogram of gradients. Default value is 9.
	// cv::HOGDescriptor::setNbins(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:601
	// ("cv::HOGDescriptor::setNbins", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_nbins(&mut self, val: i32) {
		let ret = unsafe { sys::cv_HOGDescriptor_propNbins_const_int(self.as_raw_mut_HOGDescriptor(), val) };
		ret
	}

	/// not documented
	// cv::HOGDescriptor::setDerivAperture(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:604
	// ("cv::HOGDescriptor::setDerivAperture", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_deriv_aperture(&mut self, val: i32) {
		let ret = unsafe { sys::cv_HOGDescriptor_propDerivAperture_const_int(self.as_raw_mut_HOGDescriptor(), val) };
		ret
	}

	/// Gaussian smoothing window parameter.
	// cv::HOGDescriptor::setWinSigma(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:607
	// ("cv::HOGDescriptor::setWinSigma", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_win_sigma(&mut self, val: f64) {
		let ret = unsafe { sys::cv_HOGDescriptor_propWinSigma_const_double(self.as_raw_mut_HOGDescriptor(), val) };
		ret
	}

	/// histogramNormType
	// cv::HOGDescriptor::setHistogramNormType(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:610
	// ("cv::HOGDescriptor::setHistogramNormType", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_histogram_norm_type(&mut self, val: i32) {
		let ret = unsafe { sys::cv_HOGDescriptor_propHistogramNormType_const_int(self.as_raw_mut_HOGDescriptor(), val) };
		ret
	}

	/// L2-Hys normalization method shrinkage.
	// cv::HOGDescriptor::setL2HysThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:613
	// ("cv::HOGDescriptor::setL2HysThreshold", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_l2_hys_threshold(&mut self, val: f64) {
		let ret = unsafe { sys::cv_HOGDescriptor_propL2HysThreshold_const_double(self.as_raw_mut_HOGDescriptor(), val) };
		ret
	}

	/// Flag to specify whether the gamma correction preprocessing is required or not.
	// cv::HOGDescriptor::setGammaCorrection(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:616
	// ("cv::HOGDescriptor::setGammaCorrection", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_gamma_correction(&mut self, val: bool) {
		let ret = unsafe { sys::cv_HOGDescriptor_propGammaCorrection_const_bool(self.as_raw_mut_HOGDescriptor(), val) };
		ret
	}

	/// coefficients for the linear SVM classifier.
	// cv::HOGDescriptor::setSvmDetector(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:619
	// ("cv::HOGDescriptor::setSvmDetector", vec![(pred!(mut, ["val"], ["const std::vector<float>"]), _)]),
	#[inline]
	fn set_svm_detector(&mut self, val: core::Vector<f32>) {
		let ret = unsafe { sys::cv_HOGDescriptor_propSvmDetector_const_vectorLfloatG(self.as_raw_mut_HOGDescriptor(), val.as_raw_VectorOff32()) };
		ret
	}

	/// coefficients for the linear SVM classifier used when OpenCL is enabled
	// cv::HOGDescriptor::setOclSvmDetector(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:622
	// ("cv::HOGDescriptor::setOclSvmDetector", vec![(pred!(mut, ["val"], ["const cv::UMat"]), _)]),
	#[inline]
	fn set_ocl_svm_detector(&mut self, val: core::UMat) {
		let ret = unsafe { sys::cv_HOGDescriptor_propOclSvmDetector_const_UMat(self.as_raw_mut_HOGDescriptor(), val.as_raw_UMat()) };
		ret
	}

	/// not documented
	// cv::HOGDescriptor::setFree_coef(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:625
	// ("cv::HOGDescriptor::setFree_coef", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_free_coef(&mut self, val: f32) {
		let ret = unsafe { sys::cv_HOGDescriptor_propFree_coef_const_float(self.as_raw_mut_HOGDescriptor(), val) };
		ret
	}

	/// Maximum number of detection window increases. Default value is 64
	// cv::HOGDescriptor::setNlevels(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:628
	// ("cv::HOGDescriptor::setNlevels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_nlevels(&mut self, val: i32) {
		let ret = unsafe { sys::cv_HOGDescriptor_propNlevels_const_int(self.as_raw_mut_HOGDescriptor(), val) };
		ret
	}

	/// Indicates signed gradient will be used or not
	// cv::HOGDescriptor::setSignedGradient(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:631
	// ("cv::HOGDescriptor::setSignedGradient", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_signed_gradient(&mut self, val: bool) {
		let ret = unsafe { sys::cv_HOGDescriptor_propSignedGradient_const_bool(self.as_raw_mut_HOGDescriptor(), val) };
		ret
	}

	/// @example samples/cpp/peopledetect.cpp
	/// /
	/// Sets coefficients for the linear SVM classifier.
	/// ## Parameters
	/// * _svmdetector: coefficients for the linear SVM classifier.
	// setSVMDetector(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:453
	// ("cv::HOGDescriptor::setSVMDetector", vec![(pred!(mut, ["_svmdetector"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn set_svm_detector_input_array(&mut self, _svmdetector: &impl ToInputArray) -> Result<()> {
		input_array_arg!(_svmdetector);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HOGDescriptor_setSVMDetector_const__InputArrayR(self.as_raw_mut_HOGDescriptor(), _svmdetector.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Reads HOGDescriptor parameters and coefficients for the linear SVM classifier from a file node.
	/// ## Parameters
	/// * fn: File node
	// read(FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:458
	// ("cv::HOGDescriptor::read", vec![(pred!(mut, ["fn"], ["cv::FileNode*"]), _)]),
	#[inline]
	fn read(&mut self, fn_: &mut impl core::FileNodeTrait) -> Result<bool> {
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
	// load(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:470
	// ("cv::HOGDescriptor::load", vec![(pred!(mut, ["filename", "objname"], ["const cv::String*", "const cv::String*"]), _)]),
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
	/// This alternative version of [HOGDescriptorTrait::load] function uses the following default values for its arguments:
	/// * objname: String()
	// cv::HOGDescriptor::load(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:470
	// ("cv::HOGDescriptor::load", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	#[inline]
	fn load_def(&mut self, filename: &str) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HOGDescriptor_load_const_StringR(self.as_raw_mut_HOGDescriptor(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// read/parse Dalal's alt model file
	/// ## Parameters
	/// * modelfile: Path of Dalal's alt model file.
	// readALTModel(String)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:666
	// ("cv::HOGDescriptor::readALTModel", vec![(pred!(mut, ["modelfile"], ["cv::String"]), _)]),
	#[inline]
	fn read_alt_model(&mut self, modelfile: &str) -> Result<()> {
		extern_container_arg!(modelfile);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HOGDescriptor_readALTModel_String(self.as_raw_mut_HOGDescriptor(), modelfile.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Implementation of HOG (Histogram of Oriented Gradients) descriptor and object detector.
///
/// the HOG descriptor algorithm introduced by Navneet Dalal and Bill Triggs [Dalal2005](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Dalal2005) .
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
// HOGDescriptor /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:372
pub struct HOGDescriptor {
	ptr: *mut c_void,
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

boxed_ref! { HOGDescriptor, crate::objdetect::HOGDescriptorTraitConst, as_raw_HOGDescriptor, crate::objdetect::HOGDescriptorTrait, as_raw_mut_HOGDescriptor }

impl HOGDescriptor {
	/// Creates the HOG descriptor and detector with default parameters.
	///
	/// aqual to HOGDescriptor(Size(64,128), Size(16,16), Size(8,8), Size(8,8), 9, 1 )
	// HOGDescriptor()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:383
	// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, [], []), _)]),
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
	/// aqual to HOGDescriptor(Size(64,128), Size(16,16), Size(8,8), Size(8,8), 9, 1 )
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
	// HOGDescriptor(Size, Size, Size, Size, int, int, double, int, double, bool, int, bool)(SimpleClass, SimpleClass, SimpleClass, SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:403
	// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, ["_winSize", "_blockSize", "_blockStride", "_cellSize", "_nbins", "_derivAperture", "_winSigma", "_histogramNormType", "_L2HysThreshold", "_gammaCorrection", "_nlevels", "_signedGradient"], ["cv::Size", "cv::Size", "cv::Size", "cv::Size", "int", "int", "double", "int", "double", "bool", "int", "bool"]), _)]),
	#[inline]
	pub fn new(_win_size: core::Size, _block_size: core::Size, _block_stride: core::Size, _cell_size: core::Size, _nbins: i32, _deriv_aperture: i32, _win_sigma: f64, _histogram_norm_type: i32, _l2_hys_threshold: f64, _gamma_correction: bool, _nlevels: i32, _signed_gradient: bool) -> Result<crate::objdetect::HOGDescriptor> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HOGDescriptor_HOGDescriptor_Size_Size_Size_Size_int_int_double_int_double_bool_int_bool(&_win_size, &_block_size, &_block_stride, &_cell_size, _nbins, _deriv_aperture, _win_sigma, _histogram_norm_type, _l2_hys_threshold, _gamma_correction, _nlevels, _signed_gradient, ocvrs_return.as_mut_ptr()) };
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
	// cv::HOGDescriptor::HOGDescriptor(SimpleClass, SimpleClass, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:403
	// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, ["_winSize", "_blockSize", "_blockStride", "_cellSize", "_nbins"], ["cv::Size", "cv::Size", "cv::Size", "cv::Size", "int"]), _)]),
	#[inline]
	pub fn new_def(_win_size: core::Size, _block_size: core::Size, _block_stride: core::Size, _cell_size: core::Size, _nbins: i32) -> Result<crate::objdetect::HOGDescriptor> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HOGDescriptor_HOGDescriptor_Size_Size_Size_Size_int(&_win_size, &_block_size, &_block_stride, &_cell_size, _nbins, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::objdetect::HOGDescriptor::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates the HOG descriptor and detector with default parameters.
	///
	/// aqual to HOGDescriptor(Size(64,128), Size(16,16), Size(8,8), Size(8,8), 9, 1 )
	///
	/// ## Overloaded parameters
	///
	///
	/// Creates the HOG descriptor and detector and loads HOGDescriptor parameters and coefficients for the linear SVM classifier from a file.
	/// ## Parameters
	/// * filename: the file name containing  HOGDescriptor properties and coefficients of the trained classifier
	// HOGDescriptor(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:419
	// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
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
	/// aqual to HOGDescriptor(Size(64,128), Size(16,16), Size(8,8), Size(8,8), 9, 1 )
	///
	/// ## Overloaded parameters
	///
	/// ## Parameters
	/// * d: the HOGDescriptor which cloned to create a new one.
	// HOGDescriptor(const HOGDescriptor &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:427
	// ("cv::HOGDescriptor::HOGDescriptor", vec![(pred!(mut, ["d"], ["const cv::HOGDescriptor*"]), _)]),
	#[inline]
	pub fn copy(d: &impl crate::objdetect::HOGDescriptorTraitConst) -> Result<crate::objdetect::HOGDescriptor> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HOGDescriptor_HOGDescriptor_const_HOGDescriptorR(d.as_raw_HOGDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::objdetect::HOGDescriptor::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Returns coefficients of the classifier trained for people detection (for 64x128 windows).
	// getDefaultPeopleDetector()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:580
	// ("cv::HOGDescriptor::getDefaultPeopleDetector", vec![(pred!(mut, [], []), _)]),
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
	// getDaimlerPeopleDetector()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:586
	// ("cv::HOGDescriptor::getDaimlerPeopleDetector", vec![(pred!(mut, [], []), _)]),
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
// QRCodeDetector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:740
pub trait QRCodeDetectorTraitConst {
	fn as_raw_QRCodeDetector(&self) -> *const c_void;

	/// Detects QR code in image and returns the quadrangle containing the code.
	/// ## Parameters
	/// * img: grayscale or color (BGR) image containing (or not) QR code.
	/// * points: Output vector of vertices of the minimum-area quadrangle containing the code.
	// detect(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:761
	// ("cv::QRCodeDetector::detect", vec![(pred!(const, ["img", "points"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn detect(&self, img: &impl ToInputArray, points: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(img);
		output_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_QRCodeDetector_detect_const_const__InputArrayR_const__OutputArrayR(self.as_raw_QRCodeDetector(), img.as_raw__InputArray(), points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Detects QR codes in image and returns the vector of the quadrangles containing the codes.
	/// ## Parameters
	/// * img: grayscale or color (BGR) image containing (or not) QR codes.
	/// * points: Output vector of vector of vertices of the minimum-area quadrangle containing the codes.
	// detectMulti(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:804
	// ("cv::QRCodeDetector::detectMulti", vec![(pred!(const, ["img", "points"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn detect_multi(&self, img: &impl ToInputArray, points: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(img);
		output_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_QRCodeDetector_detectMulti_const_const__InputArrayR_const__OutputArrayR(self.as_raw_QRCodeDetector(), img.as_raw__InputArray(), points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Decodes QR codes in image once it's found by the detect() method.
	/// ## Parameters
	/// * img: grayscale or color (BGR) image containing QR codes.
	/// * decoded_info: UTF8-encoded output vector of string or empty vector of string if the codes cannot be decoded.
	/// * points: vector of Quadrangle vertices found by detect() method (or some other algorithm).
	/// * straight_qrcode: The optional output vector of images containing rectified and binarized QR codes
	///
	/// ## C++ default parameters
	/// * straight_qrcode: noArray()
	// decodeMulti(InputArray, InputArray, std::vector<cv::String> &, OutputArrayOfArrays)(InputArray, InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:813
	// ("cv::QRCodeDetector::decodeMulti", vec![(pred!(const, ["img", "points", "decoded_info", "straight_qrcode"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::String>*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn decode_multi(&self, img: &impl ToInputArray, points: &impl ToInputArray, decoded_info: &mut core::Vector<String>, straight_qrcode: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(img);
		input_array_arg!(points);
		output_array_arg!(straight_qrcode);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_QRCodeDetector_decodeMulti_const_const__InputArrayR_const__InputArrayR_vectorLStringGR_const__OutputArrayR(self.as_raw_QRCodeDetector(), img.as_raw__InputArray(), points.as_raw__InputArray(), decoded_info.as_raw_mut_VectorOfString(), straight_qrcode.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Decodes QR codes in image once it's found by the detect() method.
	/// ## Parameters
	/// * img: grayscale or color (BGR) image containing QR codes.
	/// * decoded_info: UTF8-encoded output vector of string or empty vector of string if the codes cannot be decoded.
	/// * points: vector of Quadrangle vertices found by detect() method (or some other algorithm).
	/// * straight_qrcode: The optional output vector of images containing rectified and binarized QR codes
	///
	/// ## Note
	/// This alternative version of [QRCodeDetectorTraitConst::decode_multi] function uses the following default values for its arguments:
	/// * straight_qrcode: noArray()
	// cv::QRCodeDetector::decodeMulti(InputArray, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:813
	// ("cv::QRCodeDetector::decodeMulti", vec![(pred!(const, ["img", "points", "decoded_info"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::String>*"]), _)]),
	#[inline]
	fn decode_multi_def(&self, img: &impl ToInputArray, points: &impl ToInputArray, decoded_info: &mut core::Vector<String>) -> Result<bool> {
		input_array_arg!(img);
		input_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_QRCodeDetector_decodeMulti_const_const__InputArrayR_const__InputArrayR_vectorLStringGR(self.as_raw_QRCodeDetector(), img.as_raw__InputArray(), points.as_raw__InputArray(), decoded_info.as_raw_mut_VectorOfString(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Both detects and decodes QR codes
	/// ## Parameters
	/// * img: grayscale or color (BGR) image containing QR codes.
	/// * decoded_info: UTF8-encoded output vector of string or empty vector of string if the codes cannot be decoded.
	/// * points: optional output vector of vertices of the found QR code quadrangles. Will be empty if not found.
	/// * straight_qrcode: The optional output vector of images containing rectified and binarized QR codes
	///
	/// ## C++ default parameters
	/// * points: noArray()
	/// * straight_qrcode: noArray()
	// detectAndDecodeMulti(InputArray, std::vector<cv::String> &, OutputArray, OutputArrayOfArrays)(InputArray, CppPassByVoidPtr, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:826
	// ("cv::QRCodeDetector::detectAndDecodeMulti", vec![(pred!(const, ["img", "decoded_info", "points", "straight_qrcode"], ["const cv::_InputArray*", "std::vector<cv::String>*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn detect_and_decode_multi(&self, img: &impl ToInputArray, decoded_info: &mut core::Vector<String>, points: &mut impl ToOutputArray, straight_qrcode: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(img);
		output_array_arg!(points);
		output_array_arg!(straight_qrcode);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_QRCodeDetector_detectAndDecodeMulti_const_const__InputArrayR_vectorLStringGR_const__OutputArrayR_const__OutputArrayR(self.as_raw_QRCodeDetector(), img.as_raw__InputArray(), decoded_info.as_raw_mut_VectorOfString(), points.as_raw__OutputArray(), straight_qrcode.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Both detects and decodes QR codes
	/// ## Parameters
	/// * img: grayscale or color (BGR) image containing QR codes.
	/// * decoded_info: UTF8-encoded output vector of string or empty vector of string if the codes cannot be decoded.
	/// * points: optional output vector of vertices of the found QR code quadrangles. Will be empty if not found.
	/// * straight_qrcode: The optional output vector of images containing rectified and binarized QR codes
	///
	/// ## Note
	/// This alternative version of [QRCodeDetectorTraitConst::detect_and_decode_multi] function uses the following default values for its arguments:
	/// * points: noArray()
	/// * straight_qrcode: noArray()
	// cv::QRCodeDetector::detectAndDecodeMulti(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:826
	// ("cv::QRCodeDetector::detectAndDecodeMulti", vec![(pred!(const, ["img", "decoded_info"], ["const cv::_InputArray*", "std::vector<cv::String>*"]), _)]),
	#[inline]
	fn detect_and_decode_multi_def(&self, img: &impl ToInputArray, decoded_info: &mut core::Vector<String>) -> Result<bool> {
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_QRCodeDetector_detectAndDecodeMulti_const_const__InputArrayR_vectorLStringGR(self.as_raw_QRCodeDetector(), img.as_raw__InputArray(), decoded_info.as_raw_mut_VectorOfString(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * straight_qrcode: noArray()
	// decodeMulti(InputArray, InputArray, std::vector<std::string> &, OutputArrayOfArrays)(InputArray, InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:834
	// ("cv::QRCodeDetector::decodeMulti", vec![(pred!(const, ["img", "points", "decoded_info", "straight_qrcode"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::string>*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn decode_multi_1(&self, img: &impl ToInputArray, points: &impl ToInputArray, decoded_info: &mut core::Vector<String>, straight_qrcode: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(img);
		input_array_arg!(points);
		output_array_arg!(straight_qrcode);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_QRCodeDetector_decodeMulti_const_const__InputArrayR_const__InputArrayR_vectorLstringGR_const__OutputArrayR(self.as_raw_QRCodeDetector(), img.as_raw__InputArray(), points.as_raw__InputArray(), decoded_info.as_raw_mut_VectorOfString(), straight_qrcode.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [QRCodeDetectorTraitConst::decode_multi] function uses the following default values for its arguments:
	/// * straight_qrcode: noArray()
	// cv::QRCodeDetector::decodeMulti(InputArray, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:834
	// ("cv::QRCodeDetector::decodeMulti", vec![(pred!(const, ["img", "points", "decoded_info"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::string>*"]), _)]),
	#[inline]
	fn decode_multi_def_1(&self, img: &impl ToInputArray, points: &impl ToInputArray, decoded_info: &mut core::Vector<String>) -> Result<bool> {
		input_array_arg!(img);
		input_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_QRCodeDetector_decodeMulti_const_const__InputArrayR_const__InputArrayR_vectorLstringGR(self.as_raw_QRCodeDetector(), img.as_raw__InputArray(), points.as_raw__InputArray(), decoded_info.as_raw_mut_VectorOfString(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::objdetect::QRCodeDetector]
pub trait QRCodeDetectorTrait: crate::objdetect::QRCodeDetectorTraitConst {
	fn as_raw_mut_QRCodeDetector(&mut self) -> *mut c_void;

	/// sets the epsilon used during the horizontal scan of QR code stop marker detection.
	/// ## Parameters
	/// * epsX: Epsilon neighborhood, which allows you to determine the horizontal pattern
	/// of the scheme 1:1:3:1:1 according to QR code standard.
	// setEpsX(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:750
	// ("cv::QRCodeDetector::setEpsX", vec![(pred!(mut, ["epsX"], ["double"]), _)]),
	#[inline]
	fn set_eps_x(&mut self, eps_x: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_QRCodeDetector_setEpsX_double(self.as_raw_mut_QRCodeDetector(), eps_x, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// sets the epsilon used during the vertical scan of QR code stop marker detection.
	/// ## Parameters
	/// * epsY: Epsilon neighborhood, which allows you to determine the vertical pattern
	/// of the scheme 1:1:3:1:1 according to QR code standard.
	// setEpsY(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:755
	// ("cv::QRCodeDetector::setEpsY", vec![(pred!(mut, ["epsY"], ["double"]), _)]),
	#[inline]
	fn set_eps_y(&mut self, eps_y: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_QRCodeDetector_setEpsY_double(self.as_raw_mut_QRCodeDetector(), eps_y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Decodes QR code in image once it's found by the detect() method.
	///
	/// Returns UTF8-encoded output string or empty string if the code cannot be decoded.
	/// ## Parameters
	/// * img: grayscale or color (BGR) image containing QR code.
	/// * points: Quadrangle vertices found by detect() method (or some other algorithm).
	/// * straight_qrcode: The optional output image containing rectified and binarized QR code
	///
	/// ## C++ default parameters
	/// * straight_qrcode: noArray()
	// decode(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:770
	// ("cv::QRCodeDetector::decode", vec![(pred!(mut, ["img", "points", "straight_qrcode"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn decode(&mut self, img: &impl ToInputArray, points: &impl ToInputArray, straight_qrcode: &mut impl ToOutputArray) -> Result<Vec<u8>> {
		input_array_arg!(img);
		input_array_arg!(points);
		output_array_arg!(straight_qrcode);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_QRCodeDetector_decode_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_QRCodeDetector(), img.as_raw__InputArray(), points.as_raw__InputArray(), straight_qrcode.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { Vec::<u8>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Decodes QR code in image once it's found by the detect() method.
	///
	/// Returns UTF8-encoded output string or empty string if the code cannot be decoded.
	/// ## Parameters
	/// * img: grayscale or color (BGR) image containing QR code.
	/// * points: Quadrangle vertices found by detect() method (or some other algorithm).
	/// * straight_qrcode: The optional output image containing rectified and binarized QR code
	///
	/// ## Note
	/// This alternative version of [QRCodeDetectorTrait::decode] function uses the following default values for its arguments:
	/// * straight_qrcode: noArray()
	// cv::QRCodeDetector::decode(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:770
	// ("cv::QRCodeDetector::decode", vec![(pred!(mut, ["img", "points"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn decode_def(&mut self, img: &impl ToInputArray, points: &impl ToInputArray) -> Result<Vec<u8>> {
		input_array_arg!(img);
		input_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_QRCodeDetector_decode_const__InputArrayR_const__InputArrayR(self.as_raw_mut_QRCodeDetector(), img.as_raw__InputArray(), points.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
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
	/// ## C++ default parameters
	/// * straight_qrcode: noArray()
	// decodeCurved(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:779
	// ("cv::QRCodeDetector::decodeCurved", vec![(pred!(mut, ["img", "points", "straight_qrcode"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn decode_curved(&mut self, img: &impl ToInputArray, points: &impl ToInputArray, straight_qrcode: &mut impl ToOutputArray) -> Result<Vec<u8>> {
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
	/// This alternative version of [QRCodeDetectorTrait::decode_curved] function uses the following default values for its arguments:
	/// * straight_qrcode: noArray()
	// cv::QRCodeDetector::decodeCurved(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:779
	// ("cv::QRCodeDetector::decodeCurved", vec![(pred!(mut, ["img", "points"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn decode_curved_def(&mut self, img: &impl ToInputArray, points: &impl ToInputArray) -> Result<Vec<u8>> {
		input_array_arg!(img);
		input_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_QRCodeDetector_decodeCurved_const__InputArrayR_const__InputArrayR(self.as_raw_mut_QRCodeDetector(), img.as_raw__InputArray(), points.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { Vec::<u8>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Both detects and decodes QR code
	///
	/// ## Parameters
	/// * img: grayscale or color (BGR) image containing QR code.
	/// * points: optional output array of vertices of the found QR code quadrangle. Will be empty if not found.
	/// * straight_qrcode: The optional output image containing rectified and binarized QR code
	///
	/// ## C++ default parameters
	/// * points: noArray()
	/// * straight_qrcode: noArray()
	// detectAndDecode(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:787
	// ("cv::QRCodeDetector::detectAndDecode", vec![(pred!(mut, ["img", "points", "straight_qrcode"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn detect_and_decode(&mut self, img: &impl ToInputArray, points: &mut impl ToOutputArray, straight_qrcode: &mut impl ToOutputArray) -> Result<Vec<u8>> {
		input_array_arg!(img);
		output_array_arg!(points);
		output_array_arg!(straight_qrcode);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_QRCodeDetector_detectAndDecode_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_QRCodeDetector(), img.as_raw__InputArray(), points.as_raw__OutputArray(), straight_qrcode.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { Vec::<u8>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Both detects and decodes QR code
	///
	/// ## Parameters
	/// * img: grayscale or color (BGR) image containing QR code.
	/// * points: optional output array of vertices of the found QR code quadrangle. Will be empty if not found.
	/// * straight_qrcode: The optional output image containing rectified and binarized QR code
	///
	/// ## Note
	/// This alternative version of [QRCodeDetectorTrait::detect_and_decode] function uses the following default values for its arguments:
	/// * points: noArray()
	/// * straight_qrcode: noArray()
	// cv::QRCodeDetector::detectAndDecode(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:787
	// ("cv::QRCodeDetector::detectAndDecode", vec![(pred!(mut, ["img"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn detect_and_decode_def(&mut self, img: &impl ToInputArray) -> Result<Vec<u8>> {
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_QRCodeDetector_detectAndDecode_const__InputArrayR(self.as_raw_mut_QRCodeDetector(), img.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
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
	// detectAndDecodeCurved(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:796
	// ("cv::QRCodeDetector::detectAndDecodeCurved", vec![(pred!(mut, ["img", "points", "straight_qrcode"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn detect_and_decode_curved(&mut self, img: &impl ToInputArray, points: &mut impl ToOutputArray, straight_qrcode: &mut impl ToOutputArray) -> Result<Vec<u8>> {
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
	/// This alternative version of [QRCodeDetectorTrait::detect_and_decode_curved] function uses the following default values for its arguments:
	/// * points: noArray()
	/// * straight_qrcode: noArray()
	// cv::QRCodeDetector::detectAndDecodeCurved(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:796
	// ("cv::QRCodeDetector::detectAndDecodeCurved", vec![(pred!(mut, ["img"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn detect_and_decode_curved_def(&mut self, img: &impl ToInputArray) -> Result<Vec<u8>> {
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_QRCodeDetector_detectAndDecodeCurved_const__InputArrayR(self.as_raw_mut_QRCodeDetector(), img.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { Vec::<u8>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

// QRCodeDetector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:740
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

impl crate::objdetect::QRCodeDetectorTraitConst for QRCodeDetector {
	#[inline] fn as_raw_QRCodeDetector(&self) -> *const c_void { self.as_raw() }
}

impl crate::objdetect::QRCodeDetectorTrait for QRCodeDetector {
	#[inline] fn as_raw_mut_QRCodeDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { QRCodeDetector, crate::objdetect::QRCodeDetectorTraitConst, as_raw_QRCodeDetector, crate::objdetect::QRCodeDetectorTrait, as_raw_mut_QRCodeDetector }

impl QRCodeDetector {
	// QRCodeDetector()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:743
	// ("cv::QRCodeDetector::QRCodeDetector", vec![(pred!(mut, [], []), _)]),
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

impl std::fmt::Debug for QRCodeDetector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("QRCodeDetector")
			.finish()
	}
}

/// Constant methods for [crate::objdetect::QRCodeEncoder]
// QRCodeEncoder /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:677
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
	// encode(const String &, OutputArray)(InString, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:730
	// ("cv::QRCodeEncoder::encode", vec![(pred!(mut, ["encoded_info", "qrcode"], ["const cv::String*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn encode(&mut self, encoded_info: &str, qrcode: &mut impl ToOutputArray) -> Result<()> {
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
	// encodeStructuredAppend(const String &, OutputArrayOfArrays)(InString, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:736
	// ("cv::QRCodeEncoder::encodeStructuredAppend", vec![(pred!(mut, ["encoded_info", "qrcodes"], ["const cv::String*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn encode_structured_append(&mut self, encoded_info: &str, qrcodes: &mut impl ToOutputArray) -> Result<()> {
		extern_container_arg!(encoded_info);
		output_array_arg!(qrcodes);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_QRCodeEncoder_encodeStructuredAppend_const_StringR_const__OutputArrayR(self.as_raw_mut_QRCodeEncoder(), encoded_info.opencv_as_extern(), qrcodes.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// QRCodeEncoder /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:677
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

impl crate::objdetect::QRCodeEncoderTraitConst for QRCodeEncoder {
	#[inline] fn as_raw_QRCodeEncoder(&self) -> *const c_void { self.as_raw() }
}

impl crate::objdetect::QRCodeEncoderTrait for QRCodeEncoder {
	#[inline] fn as_raw_mut_QRCodeEncoder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { QRCodeEncoder, crate::objdetect::QRCodeEncoderTraitConst, as_raw_QRCodeEncoder, crate::objdetect::QRCodeEncoderTrait, as_raw_mut_QRCodeEncoder }

impl QRCodeEncoder {
	/// Constructor
	/// ## Parameters
	/// * parameters: QR code encoder parameters QRCodeEncoder::Params
	///
	/// ## C++ default parameters
	/// * parameters: QRCodeEncoder::Params()
	// create(const QRCodeEncoder::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:724
	// ("cv::QRCodeEncoder::create", vec![(pred!(mut, ["parameters"], ["const cv::QRCodeEncoder::Params*"]), _)]),
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
	/// This alternative version of [QRCodeEncoder::create] function uses the following default values for its arguments:
	/// * parameters: QRCodeEncoder::Params()
	// cv::QRCodeEncoder::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:724
	// ("cv::QRCodeEncoder::create", vec![(pred!(mut, [], []), _)]),
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
// Params /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:711
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
	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:713
	// ("cv::QRCodeEncoder::Params::Params", vec![(pred!(mut, [], []), _)]),
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
// SimilarRects /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:121
pub trait SimilarRectsTraitConst {
	fn as_raw_SimilarRects(&self) -> *const c_void;

	// cv::SimilarRects::eps() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:133
	// ("cv::SimilarRects::eps", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn eps(&self) -> f64 {
		let ret = unsafe { sys::cv_SimilarRects_propEps_const(self.as_raw_SimilarRects()) };
		ret
	}

	// operator()(const Rect &, const Rect &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:125
	// ("cv::SimilarRects::operator()", vec![(pred!(const, ["r1", "r2"], ["const cv::Rect*", "const cv::Rect*"]), _)]),
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

	// cv::SimilarRects::setEps(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:133
	// ("cv::SimilarRects::setEps", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_eps(&mut self, val: f64) {
		let ret = unsafe { sys::cv_SimilarRects_propEps_const_double(self.as_raw_mut_SimilarRects(), val) };
		ret
	}

}

/// class for grouping object candidates, detected by Cascade Classifier, HOG etc.
/// instance of the class is to be passed to cv::partition (see cxoperations.hpp)
// SimilarRects /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:121
pub struct SimilarRects {
	ptr: *mut c_void,
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

boxed_ref! { SimilarRects, crate::objdetect::SimilarRectsTraitConst, as_raw_SimilarRects, crate::objdetect::SimilarRectsTrait, as_raw_mut_SimilarRects }

impl SimilarRects {
	// SimilarRects(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/objdetect.hpp:124
	// ("cv::SimilarRects::SimilarRects", vec![(pred!(mut, ["_eps"], ["double"]), _)]),
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
