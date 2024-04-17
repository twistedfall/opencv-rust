use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::type_ref::Constness::{Const, Mut};
use crate::type_ref::TypeRefTypeHint;
use crate::writer::rust_native::type_ref::Lifetime;
use crate::FuncId;

pub const ARG_OVERRIDE_SELF: &str = "this";

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
		(
			FuncId::new_const("cv::MatConstIterator::pos", ["_idx"]),
			HashMap::from([("_idx", TypeRefTypeHint::PrimitivePtrAsRaw)]),
		),
		(
			FuncId::new_mut("cv::MatSize::MatSize", ["_p"]),
			HashMap::from([("_p", TypeRefTypeHint::PrimitivePtrAsRaw)]),
		),
	])
});

pub static RETURN_OVERRIDE: Lazy<HashMap<FuncId, TypeRefTypeHint>> = Lazy::new(|| {
	HashMap::from([
		// todo MatExpr?
		// todo Mat::set_matexpr unsafe
		// Mat
		(
			FuncId::new_mut("cv::Mat::Mat", ["m"]),
			TypeRefTypeHint::BoxedAsRef(Mut, "m", Lifetime::Elided),
		),
		(
			FuncId::new_mut("cv::Mat::Mat", ["m", "roi"]),
			TypeRefTypeHint::BoxedAsRef(Mut, "m", Lifetime::Elided),
		),
		(
			FuncId::new_mut("cv::Mat::Mat", ["m", "ranges"]),
			TypeRefTypeHint::BoxedAsRef(Mut, "m", Lifetime::Custom("boxed")),
		),
		(
			FuncId::new_mut("cv::Mat::Mat", ["m", "rowRange", "colRange"]),
			TypeRefTypeHint::BoxedAsRef(Mut, "m", Lifetime::Custom("boxed")),
		),
		(
			FuncId::new_const("cv::Mat::reshape", ["cn", "rows"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::Mat::reshape", ["cn", "newndims", "newsz"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::Mat::reshape", ["cn", "newshape"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::Mat::row", ["y"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::Mat::col", ["x"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::Mat::rowRange", ["startrow", "endrow"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::Mat::rowRange", ["r"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::Mat::colRange", ["startcol", "endcol"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::Mat::colRange", ["r"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::Mat::diag", ["d"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::Mat::operator()", ["roi"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::Mat::operator()", ["rowRange", "colRange"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::Mat::operator()", ["ranges"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		// GpuMatND
		(
			FuncId::new_const("cv::cuda::GpuMatND::createGpuMatHeader", ["idx", "rowRange", "colRange"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::cuda::GpuMatND::createGpuMatHeader", []),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::cuda::GpuMatND::operator()", ["idx", "rowRange", "colRange"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::cuda::GpuMatND::operator()", ["ranges"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		// GpuMat
		(
			FuncId::new_mut("cv::cuda::GpuMat::GpuMat", ["m", "rowRange", "colRange"]),
			TypeRefTypeHint::BoxedAsRef(Mut, "m", Lifetime::Elided),
		),
		(
			FuncId::new_mut("cv::cuda::GpuMat::GpuMat", ["m", "roi"]),
			TypeRefTypeHint::BoxedAsRef(Mut, "m", Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::cuda::GpuMat::row", ["y"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::cuda::GpuMat::col", ["x"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::cuda::GpuMat::rowRange", ["startrow", "endrow"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::cuda::GpuMat::rowRange", ["r"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::cuda::GpuMat::colRange", ["startcol", "endcol"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::cuda::GpuMat::colRange", ["r"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::cuda::GpuMat::reshape", ["cn", "rows"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::cuda::GpuMat::operator()", ["rowRange", "colRange"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::cuda::GpuMat::operator()", ["roi"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		// UMat
		(
			FuncId::new_mut("cv::UMat::UMat", ["m", "rowRange", "colRange"]),
			TypeRefTypeHint::BoxedAsRef(Mut, "m", Lifetime::Custom("boxed")),
		),
		(
			FuncId::new_mut("cv::UMat::UMat", ["m", "roi"]),
			TypeRefTypeHint::BoxedAsRef(Mut, "m", Lifetime::Elided),
		),
		(
			FuncId::new_mut("cv::UMat::UMat", ["m", "ranges"]),
			TypeRefTypeHint::BoxedAsRef(Mut, "m", Lifetime::Custom("boxed")),
		),
		(
			FuncId::new_const("cv::UMat::row", ["y"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::UMat::col", ["x"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::UMat::rowRange", ["startrow", "endrow"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::UMat::rowRange", ["r"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::UMat::colRange", ["startcol", "endcol"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::UMat::colRange", ["r"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::UMat::diag", ["d"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::UMat::reshape", ["cn", "rows"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::UMat::reshape", ["cn", "newndims", "newsz"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::UMat::operator()", ["rowRange", "colRange"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::UMat::operator()", ["roi"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_const("cv::UMat::operator()", ["ranges"]),
			TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
		),
		(
			FuncId::new_mut("cv::_InputArray::_InputArray", ["m"]),
			TypeRefTypeHint::BoxedAsRef(Const, "m", Lifetime::Elided),
		),
		(
			FuncId::new_mut("cv::_InputArray::_InputArray", ["expr"]),
			TypeRefTypeHint::BoxedAsRef(Const, "expr", Lifetime::Elided),
		),
		(
			FuncId::new_mut("cv::_InputArray::_InputArray", ["vec"]),
			TypeRefTypeHint::BoxedAsRef(Const, "vec", Lifetime::Elided),
		),
		(
			FuncId::new_mut("cv::_InputArray::_InputArray", ["val"]),
			TypeRefTypeHint::BoxedAsRef(Const, "val", Lifetime::Elided),
		),
		(
			FuncId::new_mut("cv::_InputArray::_InputArray", ["d_mat"]),
			TypeRefTypeHint::BoxedAsRef(Const, "d_mat", Lifetime::Elided),
		),
		(
			FuncId::new_mut("cv::_InputArray::_InputArray", ["d_mat_array"]),
			TypeRefTypeHint::BoxedAsRef(Const, "d_mat_array", Lifetime::Elided),
		),
		(
			FuncId::new_mut("cv::_InputArray::_InputArray", ["buf"]),
			TypeRefTypeHint::BoxedAsRef(Const, "buf", Lifetime::Elided),
		),
		(
			FuncId::new_mut("cv::_InputArray::_InputArray", ["cuda_mem"]),
			TypeRefTypeHint::BoxedAsRef(Const, "cuda_mem", Lifetime::Elided),
		),
		(
			FuncId::new_mut("cv::_InputArray::_InputArray", ["um"]),
			TypeRefTypeHint::BoxedAsRef(Const, "um", Lifetime::Elided),
		),
		(
			FuncId::new_mut("cv::_InputArray::_InputArray", ["umv"]),
			TypeRefTypeHint::BoxedAsRef(Const, "umv", Lifetime::Elided),
		),
		(
			FuncId::new_mut("cv::_OutputArray::_OutputArray", ["m"]),
			TypeRefTypeHint::BoxedAsRef(Const, "m", Lifetime::Elided),
		),
		(
			FuncId::new_mut("cv::_OutputArray::_OutputArray", ["vec"]),
			TypeRefTypeHint::BoxedAsRef(Const, "vec", Lifetime::Elided),
		),
		(
			FuncId::new_mut("cv::_OutputArray::_OutputArray", ["d_mat"]),
			TypeRefTypeHint::BoxedAsRef(Const, "d_mat", Lifetime::Elided),
		),
		(
			FuncId::new_mut("cv::_OutputArray::_OutputArray", ["buf"]),
			TypeRefTypeHint::BoxedAsRef(Const, "buf", Lifetime::Elided),
		),
		(
			FuncId::new_mut("cv::_OutputArray::_OutputArray", ["cuda_mem"]),
			TypeRefTypeHint::BoxedAsRef(Const, "cuda_mem", Lifetime::Elided),
		),
		(
			FuncId::new_mut("cv::_InputOutputArray::_InputOutputArray", ["m"]),
			TypeRefTypeHint::BoxedAsRef(Const, "m", Lifetime::Elided),
		),
		(
			FuncId::new_mut("cv::_InputOutputArray::_InputOutputArray", ["vec"]),
			TypeRefTypeHint::BoxedAsRef(Const, "vec", Lifetime::Elided),
		),
		(
			FuncId::new_mut("cv::_InputOutputArray::_InputOutputArray", ["d_mat"]),
			TypeRefTypeHint::BoxedAsRef(Const, "d_mat", Lifetime::Elided),
		),
		(
			FuncId::new_mut("cv::_InputOutputArray::_InputOutputArray", ["buf"]),
			TypeRefTypeHint::BoxedAsRef(Const, "buf", Lifetime::Elided),
		),
		(
			FuncId::new_mut("cv::_InputOutputArray::_InputOutputArray", ["cuda_mem"]),
			TypeRefTypeHint::BoxedAsRef(Const, "cuda_mem", Lifetime::Elided),
		),
		(
			FuncId::new_mut("cv::QRCodeDetector::decode", ["img", "points", "straight_qrcode"]),
			TypeRefTypeHint::StringAsBytes(None),
		),
		(
			FuncId::new_mut("cv::QRCodeDetector::decodeCurved", ["img", "points", "straight_qrcode"]),
			TypeRefTypeHint::StringAsBytes(None),
		),
		(
			FuncId::new_mut("cv::QRCodeDetector::detectAndDecode", ["img", "points", "straight_qrcode"]),
			TypeRefTypeHint::StringAsBytes(None),
		),
		(
			FuncId::new_mut(
				"cv::QRCodeDetector::detectAndDecodeCurved",
				["img", "points", "straight_qrcode"],
			),
			TypeRefTypeHint::StringAsBytes(None),
		),
		(
			FuncId::new_const("cv::GraphicalCodeDetector::decode", ["img", "points", "straight_code"]),
			TypeRefTypeHint::StringAsBytes(None),
		),
		(
			FuncId::new_const(
				"cv::GraphicalCodeDetector::detectAndDecode",
				["img", "points", "straight_code"],
			),
			TypeRefTypeHint::StringAsBytes(None),
		),
	])
});
