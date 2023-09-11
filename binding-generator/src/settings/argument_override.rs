use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::FuncId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ArgOverride {
	Nullable,
	NullableSlice,
	Slice,
	LenForSlice(&'static str, usize),
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
			FuncId::new("cv::Mat::at", ["idx"]),
			HashMap::from([("idx", ArgOverride::Slice)]),
		),
		(
			FuncId::new("cv::Mat::ptr", ["idx"]),
			HashMap::from([("idx", ArgOverride::Slice)]),
		),
		(
			FuncId::new("cv::Mat::Mat", ["sizes", "type", "data", "steps"]),
			HashMap::from([
				("steps", ArgOverride::NullableSlice),
				("sizes", ArgOverride::Slice),
				("ndims", ArgOverride::LenForSlice("sizes", 1)),
			]),
		),
		(
			FuncId::new("cv::Mat::Mat", ["ndims", "sizes", "type", "s"]),
			HashMap::from([
				("steps", ArgOverride::NullableSlice),
				("sizes", ArgOverride::Slice),
				("ndims", ArgOverride::LenForSlice("sizes", 1)),
			]),
		),
		(
			FuncId::new("cv::Mat::Mat", ["ndims", "sizes", "type", "data", "steps"]),
			HashMap::from([
				("steps", ArgOverride::NullableSlice),
				("sizes", ArgOverride::Slice),
				("ndims", ArgOverride::LenForSlice("sizes", 1)),
			]),
		),
		(
			FuncId::new("cv::Mat::zeros", ["ndims", "sz", "type"]),
			HashMap::from([("sz", ArgOverride::Slice), ("ndims", ArgOverride::LenForSlice("sz", 1))]),
		),
		(
			FuncId::new("cv::Mat::ones", ["ndims", "sz", "type"]),
			HashMap::from([("sz", ArgOverride::Slice), ("ndims", ArgOverride::LenForSlice("sz", 1))]),
		),
		(
			FuncId::new("cv::Mat::create", ["ndims", "sizes", "type"]),
			HashMap::from([("sizes", ArgOverride::Slice), ("ndims", ArgOverride::LenForSlice("sizes", 1))]),
		),
		(
			FuncId::new("cv::Mat::reshape", ["cn", "newndims", "newsz"]),
			HashMap::from([
				("newsz", ArgOverride::Slice),
				("newndims", ArgOverride::LenForSlice("newsz", 1)),
			]),
		),
		(
			FuncId::new("cv::SparseMat::Hdr::Hdr", ["_dims", "_sizes", "_type"]),
			HashMap::from([
				("_sizes", ArgOverride::Slice),
				("_dims", ArgOverride::LenForSlice("_sizes", 1)),
			]),
		),
		(
			FuncId::new("cv::UMat::UMat", ["ndims", "sizes", "type", "usageFlags"]),
			HashMap::from([("sizes", ArgOverride::Slice), ("ndims", ArgOverride::LenForSlice("sizes", 1))]),
		),
		(
			FuncId::new("cv::UMat::UMat", ["ndims", "sizes", "type", "s", "usageFlags"]),
			HashMap::from([("sizes", ArgOverride::Slice), ("ndims", ArgOverride::LenForSlice("sizes", 1))]),
		),
		(
			FuncId::new("cv::UMat::create", ["ndims", "sizes", "type", "usageFlags"]),
			HashMap::from([("sizes", ArgOverride::Slice), ("ndims", ArgOverride::LenForSlice("sizes", 1))]),
		),
		(
			FuncId::new(
				"cv::_OutputArray::create",
				["dims", "size", "type", "i", "allowTransposed", "fixedDepthMask"],
			),
			HashMap::from([("size", ArgOverride::Slice), ("dims", ArgOverride::LenForSlice("size", 1))]),
		),
		(
			FuncId::new("cv::mixChannels", ["src", "dst", "fromTo", "npairs"]),
			HashMap::from([
				("fromTo", ArgOverride::Slice),
				("npairs", ArgOverride::LenForSlice("from_to", 2)),
			]),
		),
		(
			FuncId::new(
				"cv::createTrackbar",
				["trackbarname", "winname", "value", "count", "onChange", "userdata"],
			),
			HashMap::from([("value", ArgOverride::Nullable)]),
		),
		(
			FuncId::new("cv::minMaxLoc", ["src", "minVal", "maxVal", "minLoc", "maxLoc", "mask"]),
			HashMap::from([
				("minVal", ArgOverride::Nullable),
				("maxVal", ArgOverride::Nullable),
				("minLoc", ArgOverride::Nullable),
				("maxLoc", ArgOverride::Nullable),
			]),
		),
		(
			FuncId::new("cv::minMaxLoc", ["a", "minVal", "maxVal", "minIdx", "maxIdx"]),
			HashMap::from([
				("minVal", ArgOverride::Nullable),
				("maxVal", ArgOverride::Nullable),
				("minIdx", ArgOverride::Nullable),
				("maxIdx", ArgOverride::Nullable),
			]),
		),
		(
			FuncId::new("cv::minMaxIdx", ["src", "minVal", "maxVal", "minIdx", "maxIdx", "mask"]),
			HashMap::from([
				("minVal", ArgOverride::Nullable),
				("maxVal", ArgOverride::Nullable),
				("minIdx", ArgOverride::Nullable),
				("maxIdx", ArgOverride::Nullable),
			]),
		),
		(
			FuncId::new(
				"cv::EMD",
				["signature1", "signature2", "distType", "cost", "lowerBound", "flow"],
			),
			HashMap::from([("lowerBound", ArgOverride::Nullable)]),
		),
		(
			FuncId::new("cv::decodeQRCode", ["in", "points", "decoded_info", "straight_qrcode"]),
			HashMap::from([("decoded_info", ArgOverride::StringAsBytes)]),
		),
		(
			FuncId::new("cv::QRCodeDetector::decode", ["img", "points", "straight_qrcode"]),
			HashMap::from([("return", ArgOverride::StringAsBytes)]),
		),
		(
			FuncId::new("cv::QRCodeDetector::decodeCurved", ["img", "points", "straight_qrcode"]),
			HashMap::from([("return", ArgOverride::StringAsBytes)]),
		),
		(
			FuncId::new("cv::QRCodeDetector::detectAndDecode", ["img", "points", "straight_qrcode"]),
			HashMap::from([("return", ArgOverride::StringAsBytes)]),
		),
		(
			FuncId::new(
				"cv::QRCodeDetector::detectAndDecodeCurved",
				["img", "points", "straight_qrcode"],
			),
			HashMap::from([("return", ArgOverride::StringAsBytes)]),
		),
		(
			FuncId::new("cv::GraphicalCodeDetector::decode", ["img", "points", "straight_code"]),
			HashMap::from([("return", ArgOverride::StringAsBytes)]),
		),
		(
			FuncId::new(
				"cv::GraphicalCodeDetector::detectAndDecode",
				["img", "points", "straight_code"],
			),
			HashMap::from([("return", ArgOverride::StringAsBytes)]),
		),
		(
			FuncId::new(
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
			FuncId::new("cv::VideoWriter::fourcc", ["c1", "c2", "c3", "c4"]),
			HashMap::from([
				("c1", ArgOverride::CharAsRustChar),
				("c2", ArgOverride::CharAsRustChar),
				("c3", ArgOverride::CharAsRustChar),
				("c4", ArgOverride::CharAsRustChar),
			]),
		),
		(
			FuncId::new("cv::ximgproc::createStructuredEdgeDetection", ["model", "howToGetFeatures"]),
			HashMap::from([("howToGetFeatures", ArgOverride::Nullable)]),
		),
	])
});
