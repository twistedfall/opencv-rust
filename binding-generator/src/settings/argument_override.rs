use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::type_ref::TypeRefTypeHint;
use crate::FuncId;

pub const RETURN_HINT: &str = "return";

pub static ARGUMENT_OVERRIDE: Lazy<HashMap<FuncId, HashMap<&str, TypeRefTypeHint>>> = Lazy::new(|| {
	HashMap::from([
		(
			FuncId::new_const("cv::Mat::at", ["idx"]),
			HashMap::from([("idx", TypeRefTypeHint::Slice)]),
		),
		(
			FuncId::new_mut("cv::Mat::at", ["idx"]),
			HashMap::from([("idx", TypeRefTypeHint::Slice)]),
		),
		(
			FuncId::new_const("cv::Mat::ptr", ["idx"]),
			HashMap::from([("idx", TypeRefTypeHint::Slice)]),
		),
		(
			FuncId::new_mut("cv::Mat::ptr", ["idx"]),
			HashMap::from([("idx", TypeRefTypeHint::Slice)]),
		),
		(
			FuncId::new_mut("cv::Mat::Mat", ["sizes", "type", "data", "steps"]),
			HashMap::from([("steps", TypeRefTypeHint::NullableSlice)]),
		),
		(
			FuncId::new_mut("cv::Mat::Mat", ["ndims", "sizes", "type", "s"]),
			HashMap::from([("steps", TypeRefTypeHint::NullableSlice)]),
		),
		(
			FuncId::new_mut("cv::Mat::Mat", ["ndims", "sizes", "type", "data", "steps"]),
			HashMap::from([("steps", TypeRefTypeHint::NullableSlice)]),
		),
		(
			FuncId::new_mut(
				"cv::createTrackbar",
				["trackbarname", "winname", "value", "count", "onChange", "userdata"],
			),
			HashMap::from([("value", TypeRefTypeHint::Nullable)]),
		),
		(
			FuncId::new_mut("cv::minMaxLoc", ["src", "minVal", "maxVal", "minLoc", "maxLoc", "mask"]),
			HashMap::from([
				("minVal", TypeRefTypeHint::Nullable),
				("maxVal", TypeRefTypeHint::Nullable),
				("minLoc", TypeRefTypeHint::Nullable),
				("maxLoc", TypeRefTypeHint::Nullable),
			]),
		),
		(
			FuncId::new_mut("cv::minMaxLoc", ["a", "minVal", "maxVal", "minIdx", "maxIdx"]),
			HashMap::from([
				("minVal", TypeRefTypeHint::Nullable),
				("maxVal", TypeRefTypeHint::Nullable),
				("minIdx", TypeRefTypeHint::Nullable),
				("maxIdx", TypeRefTypeHint::Nullable),
			]),
		),
		(
			FuncId::new_mut("cv::minMaxIdx", ["src", "minVal", "maxVal", "minIdx", "maxIdx", "mask"]),
			HashMap::from([
				("minVal", TypeRefTypeHint::Nullable),
				("maxVal", TypeRefTypeHint::Nullable),
				("minIdx", TypeRefTypeHint::Nullable),
				("maxIdx", TypeRefTypeHint::Nullable),
			]),
		),
		(
			FuncId::new_const("cv::RotatedRect::points", ["pts"]),
			HashMap::from([("pts", TypeRefTypeHint::AddArrayLength(4))]),
		),
		(
			FuncId::new_mut("cv::getAffineTransform", ["src", "dst"]),
			HashMap::from([
				("src", TypeRefTypeHint::AddArrayLength(3)),
				("dst", TypeRefTypeHint::AddArrayLength(3)),
			]),
		),
		(
			FuncId::new_mut("cv::getPerspectiveTransform", ["src", "dst", "solveMethod"]),
			HashMap::from([
				("src", TypeRefTypeHint::AddArrayLength(4)),
				("dst", TypeRefTypeHint::AddArrayLength(4)),
			]),
		),
		(
			FuncId::new_mut("cv::getPerspectiveTransform", ["src", "dst"]), // 3.x
			HashMap::from([
				("src", TypeRefTypeHint::AddArrayLength(4)),
				("dst", TypeRefTypeHint::AddArrayLength(4)),
			]),
		),
		(
			FuncId::new_mut(
				"cv::EMD",
				["signature1", "signature2", "distType", "cost", "lowerBound", "flow"],
			),
			HashMap::from([("lowerBound", TypeRefTypeHint::Nullable)]),
		),
		(
			FuncId::new_mut("cv::decodeQRCode", ["in", "points", "decoded_info", "straight_qrcode"]),
			HashMap::from([("decoded_info", TypeRefTypeHint::StringAsBytes(None))]),
		),
		(
			FuncId::new_mut("cv::QRCodeDetector::decode", ["img", "points", "straight_qrcode"]),
			HashMap::from([(RETURN_HINT, TypeRefTypeHint::StringAsBytes(None))]),
		),
		(
			FuncId::new_mut("cv::QRCodeDetector::decodeCurved", ["img", "points", "straight_qrcode"]),
			HashMap::from([(RETURN_HINT, TypeRefTypeHint::StringAsBytes(None))]),
		),
		(
			FuncId::new_mut("cv::QRCodeDetector::detectAndDecode", ["img", "points", "straight_qrcode"]),
			HashMap::from([(RETURN_HINT, TypeRefTypeHint::StringAsBytes(None))]),
		),
		(
			FuncId::new_mut(
				"cv::QRCodeDetector::detectAndDecodeCurved",
				["img", "points", "straight_qrcode"],
			),
			HashMap::from([(RETURN_HINT, TypeRefTypeHint::StringAsBytes(None))]),
		),
		(
			FuncId::new_const("cv::GraphicalCodeDetector::decode", ["img", "points", "straight_code"]),
			HashMap::from([(RETURN_HINT, TypeRefTypeHint::StringAsBytes(None))]),
		),
		(
			FuncId::new_const(
				"cv::GraphicalCodeDetector::detectAndDecode",
				["img", "points", "straight_code"],
			),
			HashMap::from([(RETURN_HINT, TypeRefTypeHint::StringAsBytes(None))]),
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
			HashMap::from([("validPixROI", TypeRefTypeHint::Nullable)]),
		),
		(
			FuncId::new_mut("cv::VideoWriter::fourcc", ["c1", "c2", "c3", "c4"]),
			HashMap::from([
				("c1", TypeRefTypeHint::CharAsRustChar),
				("c2", TypeRefTypeHint::CharAsRustChar),
				("c3", TypeRefTypeHint::CharAsRustChar),
				("c4", TypeRefTypeHint::CharAsRustChar),
			]),
		),
		(
			FuncId::new_mut("cv::ximgproc::createStructuredEdgeDetection", ["model", "howToGetFeatures"]),
			HashMap::from([("howToGetFeatures", TypeRefTypeHint::Nullable)]),
		),
		(
			FuncId::new_mut("cv::freetype::FreeType2::loadFontData", ["pBuf", "bufSize", "idx"]),
			HashMap::from([("pBuf", TypeRefTypeHint::PrimitivePtrAsRaw)]),
		),
	])
});
