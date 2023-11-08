use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::FuncId;

pub const RETURN_HINT: &str = "return";

#[derive(Clone, Debug, PartialEq)]
pub enum ArgOverride {
	Nullable,
	NullableSlice,
	Slice,
	LenForSlice(String, usize),
	/// Treat C++ string as a byte buffer (`Vec<u8>`) instead of an actual string
	StringAsBytes,
	/// when C++ char needs to be represented as Rust char
	CharAsRustChar,
	/// for the cases when `char *` should not be treated as string, but as a pointer to a single char
	CharPtrNotString,
}

pub static ARGUMENT_OVERRIDE: Lazy<HashMap<FuncId, HashMap<&str, ArgOverride>>> = Lazy::new(|| {
	HashMap::from([
		(
			FuncId::new_const("cv::Mat::at", ["idx"]),
			HashMap::from([("idx", ArgOverride::Slice)]),
		),
		(
			FuncId::new_mut("cv::Mat::at", ["idx"]),
			HashMap::from([("idx", ArgOverride::Slice)]),
		),
		(
			FuncId::new_const("cv::Mat::ptr", ["idx"]),
			HashMap::from([("idx", ArgOverride::Slice)]),
		),
		(
			FuncId::new_mut("cv::Mat::ptr", ["idx"]),
			HashMap::from([("idx", ArgOverride::Slice)]),
		),
		(
			FuncId::new_mut("cv::Mat::Mat", ["sizes", "type", "data", "steps"]),
			HashMap::from([("steps", ArgOverride::NullableSlice)]),
		),
		(
			FuncId::new_mut("cv::Mat::Mat", ["ndims", "sizes", "type", "s"]),
			HashMap::from([("steps", ArgOverride::NullableSlice)]),
		),
		(
			FuncId::new_mut("cv::Mat::Mat", ["ndims", "sizes", "type", "data", "steps"]),
			HashMap::from([("steps", ArgOverride::NullableSlice)]),
		),
		(
			FuncId::new_mut(
				"cv::createTrackbar",
				["trackbarname", "winname", "value", "count", "onChange", "userdata"],
			),
			HashMap::from([("value", ArgOverride::Nullable)]),
		),
		(
			FuncId::new_mut("cv::minMaxLoc", ["src", "minVal", "maxVal", "minLoc", "maxLoc", "mask"]),
			HashMap::from([
				("minVal", ArgOverride::Nullable),
				("maxVal", ArgOverride::Nullable),
				("minLoc", ArgOverride::Nullable),
				("maxLoc", ArgOverride::Nullable),
			]),
		),
		(
			FuncId::new_mut("cv::minMaxLoc", ["a", "minVal", "maxVal", "minIdx", "maxIdx"]),
			HashMap::from([
				("minVal", ArgOverride::Nullable),
				("maxVal", ArgOverride::Nullable),
				("minIdx", ArgOverride::Nullable),
				("maxIdx", ArgOverride::Nullable),
			]),
		),
		(
			FuncId::new_mut("cv::minMaxIdx", ["src", "minVal", "maxVal", "minIdx", "maxIdx", "mask"]),
			HashMap::from([
				("minVal", ArgOverride::Nullable),
				("maxVal", ArgOverride::Nullable),
				("minIdx", ArgOverride::Nullable),
				("maxIdx", ArgOverride::Nullable),
			]),
		),
		(
			FuncId::new_mut(
				"cv::EMD",
				["signature1", "signature2", "distType", "cost", "lowerBound", "flow"],
			),
			HashMap::from([("lowerBound", ArgOverride::Nullable)]),
		),
		(
			FuncId::new_mut("cv::decodeQRCode", ["in", "points", "decoded_info", "straight_qrcode"]),
			HashMap::from([("decoded_info", ArgOverride::StringAsBytes)]),
		),
		(
			FuncId::new_mut("cv::QRCodeDetector::decode", ["img", "points", "straight_qrcode"]),
			HashMap::from([(RETURN_HINT, ArgOverride::StringAsBytes)]),
		),
		(
			FuncId::new_mut("cv::QRCodeDetector::decodeCurved", ["img", "points", "straight_qrcode"]),
			HashMap::from([(RETURN_HINT, ArgOverride::StringAsBytes)]),
		),
		(
			FuncId::new_mut("cv::QRCodeDetector::detectAndDecode", ["img", "points", "straight_qrcode"]),
			HashMap::from([(RETURN_HINT, ArgOverride::StringAsBytes)]),
		),
		(
			FuncId::new_mut(
				"cv::QRCodeDetector::detectAndDecodeCurved",
				["img", "points", "straight_qrcode"],
			),
			HashMap::from([(RETURN_HINT, ArgOverride::StringAsBytes)]),
		),
		(
			FuncId::new_const("cv::GraphicalCodeDetector::decode", ["img", "points", "straight_code"]),
			HashMap::from([(RETURN_HINT, ArgOverride::StringAsBytes)]),
		),
		(
			FuncId::new_const(
				"cv::GraphicalCodeDetector::detectAndDecode",
				["img", "points", "straight_code"],
			),
			HashMap::from([(RETURN_HINT, ArgOverride::StringAsBytes)]),
		),
		(
			FuncId::new_mut(
				"cv::getOptimalNewCameraMatrix",
				[
					"cameraMatrix",
					"distCoeffs",
					"imageSize",
					"alpha",
					"newImgSize",
					"validPixROI",
					"centerPrincipalPoint",
				],
			),
			HashMap::from([("validPixROI", ArgOverride::Nullable)]),
		),
		(
			FuncId::new_mut("cv::VideoWriter::fourcc", ["c1", "c2", "c3", "c4"]),
			HashMap::from([
				("c1", ArgOverride::CharAsRustChar),
				("c2", ArgOverride::CharAsRustChar),
				("c3", ArgOverride::CharAsRustChar),
				("c4", ArgOverride::CharAsRustChar),
			]),
		),
		(
			FuncId::new_mut("cv::ximgproc::createStructuredEdgeDetection", ["model", "howToGetFeatures"]),
			HashMap::from([("howToGetFeatures", ArgOverride::Nullable)]),
		),
	])
});
