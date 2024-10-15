use std::collections::HashMap;

use crate::func::FuncMatcher;
use crate::type_ref::Constness::{Const, Mut};
use crate::type_ref::TypeRefTypeHint;
use crate::writer::rust_native::type_ref::Lifetime;

pub const ARG_OVERRIDE_SELF: &str = "this";

pub type ArgOverride = FuncMatcher<'static, HashMap<&'static str, TypeRefTypeHint>>;
pub type ReturnOverride = FuncMatcher<'static, TypeRefTypeHint>;

pub fn arg_override_factory(module: &str) -> ArgOverride {
	match module {
		"calib3d" => calib3d_arg_override_factory(),
		"core" => core_arg_override_factory(),
		"freetype" => freetype_arg_override_factory(),
		"highgui" => highgui_arg_override_factory(),
		"imgproc" => imgproc_arg_override_factory(),
		"objdetect" => objdetect_arg_override_factory(),
		"videoio" => videoio_arg_override_factory(),
		"ximgproc" => ximgproc_arg_override_factory(),
		_ => ArgOverride::empty(),
	}
}

pub fn return_override_factory(module: &str) -> ReturnOverride {
	match module {
		"core" => core_return_override_factory(),
		"objdetect" => objdetect_return_override_factory(),
		_ => ReturnOverride::empty(),
	}
}

fn calib3d_arg_override_factory() -> ArgOverride {
	FuncMatcher::create(HashMap::from([
		(
			"cv::getOptimalNewCameraMatrix",
			vec![(
				pred!(
					mut,
					[
						"cameraMatrix",
						"distCoeffs",
						"imageSize",
						"alpha",
						"newImgSize",
						"validPixROI",
						"centerPrincipalPoint",
					]
				),
				HashMap::from([("validPixROI", TypeRefTypeHint::Nullable)]),
			)],
		),
		(
			"cv::findCirclesGrid",
			vec![
				(
					pred!(
						mut,
						["image", "patternSize", "centers", "flags", "blobDetector", "parameters"]
					),
					HashMap::from([("blobDetector", TypeRefTypeHint::Nullable)]),
				),
				(
					pred!(mut, ["image", "patternSize", "centers", "flags", "blobDetector"]),
					HashMap::from([("blobDetector", TypeRefTypeHint::Nullable)]),
				),
			],
		),
	]))
}

fn core_arg_override_factory() -> ArgOverride {
	FuncMatcher::create(HashMap::from([
		(
			"cv::Mat::at",
			vec![(pred!(["idx"]), HashMap::from([("idx", TypeRefTypeHint::Slice)]))],
		),
		(
			"cv::Mat::ptr",
			vec![(pred!(["idx"]), HashMap::from([("idx", TypeRefTypeHint::Slice)]))],
		),
		(
			"cv::Mat::Mat",
			vec![
				(
					pred!(mut, ["sizes", "type", "data", "steps"]),
					HashMap::from([("steps", TypeRefTypeHint::NullableSlice)]),
				),
				(
					pred!(mut, ["ndims", "sizes", "type", "s"]),
					HashMap::from([("steps", TypeRefTypeHint::NullableSlice)]),
				),
				(
					pred!(mut, ["ndims", "sizes", "type", "data", "steps"]),
					HashMap::from([("steps", TypeRefTypeHint::NullableSlice)]),
				),
			],
		),
		(
			"cv::minMaxLoc",
			vec![
				(
					pred!(mut, ["src", "minVal", "maxVal", "minLoc", "maxLoc", "mask"]),
					HashMap::from([
						("minVal", TypeRefTypeHint::Nullable),
						("maxVal", TypeRefTypeHint::Nullable),
						("minLoc", TypeRefTypeHint::Nullable),
						("maxLoc", TypeRefTypeHint::Nullable),
					]),
				),
				(
					pred!(mut, ["a", "minVal", "maxVal", "minIdx", "maxIdx"]),
					HashMap::from([
						("minVal", TypeRefTypeHint::Nullable),
						("maxVal", TypeRefTypeHint::Nullable),
						("minIdx", TypeRefTypeHint::Nullable),
						("maxIdx", TypeRefTypeHint::Nullable),
					]),
				),
			],
		),
		(
			"cv::minMaxIdx",
			vec![(
				pred!(mut, ["src", "minVal", "maxVal", "minIdx", "maxIdx", "mask"]),
				HashMap::from([
					("minVal", TypeRefTypeHint::Nullable),
					("maxVal", TypeRefTypeHint::Nullable),
					("minIdx", TypeRefTypeHint::Nullable),
					("maxIdx", TypeRefTypeHint::Nullable),
				]),
			)],
		),
		(
			"cv::RotatedRect::points",
			vec![(
				pred!(const, ["pts"]),
				HashMap::from([("pts", TypeRefTypeHint::AddArrayLength(4))]),
			)],
		),
		(
			"cv::MatConstIterator::pos",
			vec![(
				pred!(const, ["_idx"]),
				HashMap::from([("_idx", TypeRefTypeHint::PrimitivePtrAsRaw)]),
			)],
		),
		(
			"cv::MatSize::MatSize",
			vec![(
				pred!(mut, ["_p"]),
				HashMap::from([("_p", TypeRefTypeHint::PrimitivePtrAsRaw)]),
			)],
		),
		(
			"cv::cuda::GpuMat::setDefaultAllocator",
			vec![(
				pred!(mut, ["allocator"]),
				HashMap::from([("allocator", TypeRefTypeHint::ExplicitLifetime(Lifetime::statik()))]),
			)],
		),
	]))
}

fn freetype_arg_override_factory() -> ArgOverride {
	FuncMatcher::create(HashMap::from([(
		"cv::freetype::FreeType2::loadFontData",
		vec![(
			pred!(mut, ["pBuf", "bufSize", "idx"]),
			HashMap::from([("pBuf", TypeRefTypeHint::PrimitivePtrAsRaw)]),
		)],
	)]))
}

fn highgui_arg_override_factory() -> ArgOverride {
	FuncMatcher::create(HashMap::from([(
		"cv::createTrackbar",
		vec![(
			pred!(mut, ["trackbarname", "winname", "value", "count", "onChange", "userdata"],),
			HashMap::from([("value", TypeRefTypeHint::Nullable)]),
		)],
	)]))
}

fn imgproc_arg_override_factory() -> ArgOverride {
	FuncMatcher::create(HashMap::from([
		(
			"cv::getAffineTransform",
			vec![(
				pred!(mut, ["src", "dst"]),
				HashMap::from([
					("src", TypeRefTypeHint::AddArrayLength(3)),
					("dst", TypeRefTypeHint::AddArrayLength(3)),
				]),
			)],
		),
		(
			"cv::getPerspectiveTransform",
			vec![
				(
					pred!(mut, ["src", "dst", "solveMethod"]),
					HashMap::from([
						("src", TypeRefTypeHint::AddArrayLength(4)),
						("dst", TypeRefTypeHint::AddArrayLength(4)),
					]),
				),
				(
					pred!(mut, ["src", "dst"]), // 3.x
					HashMap::from([
						("src", TypeRefTypeHint::AddArrayLength(4)),
						("dst", TypeRefTypeHint::AddArrayLength(4)),
					]),
				),
			],
		),
		(
			"cv::EMD",
			vec![(
				pred!(mut, ["signature1", "signature2", "distType", "cost", "lowerBound", "flow"],),
				HashMap::from([("lowerBound", TypeRefTypeHint::Nullable)]),
			)],
		),
	]))
}

fn objdetect_arg_override_factory() -> ArgOverride {
	FuncMatcher::create(HashMap::from([(
		"cv::decodeQRCode",
		vec![(
			pred!(mut, ["in", "points", "decoded_info", "straight_qrcode"]), // 3.4
			HashMap::from([("decoded_info", TypeRefTypeHint::StringAsBytes(None))]),
		)],
	)]))
}

fn videoio_arg_override_factory() -> ArgOverride {
	FuncMatcher::create(HashMap::from([(
		"cv::VideoWriter::fourcc",
		vec![(
			pred!(mut, ["c1", "c2", "c3", "c4"]),
			HashMap::from([
				("c1", TypeRefTypeHint::CharAsRustChar),
				("c2", TypeRefTypeHint::CharAsRustChar),
				("c3", TypeRefTypeHint::CharAsRustChar),
				("c4", TypeRefTypeHint::CharAsRustChar),
			]),
		)],
	)]))
}

fn ximgproc_arg_override_factory() -> ArgOverride {
	FuncMatcher::create(HashMap::from([(
		"cv::ximgproc::createStructuredEdgeDetection",
		vec![(
			pred!(mut, ["model", "howToGetFeatures"]),
			HashMap::from([("howToGetFeatures", TypeRefTypeHint::Nullable)]),
		)],
	)]))
}

fn core_return_override_factory() -> ReturnOverride {
	// todo MatExpr?
	FuncMatcher::create(HashMap::from([
		// Mat
		(
			"cv::Mat::Mat",
			vec![
				(
					pred!(mut, ["m"], ["const cv::Mat*"]),
					TypeRefTypeHint::BoxedAsRef(Mut, "m", Lifetime::Elided),
				),
				(
					pred!(mut, ["m", "roi"]),
					TypeRefTypeHint::BoxedAsRef(Mut, "m", Lifetime::Elided),
				),
				(
					pred!(mut, ["m", "ranges"]),
					TypeRefTypeHint::BoxedAsRef(Mut, "m", Lifetime::Custom("boxed")),
				),
				(
					pred!(mut, ["m", "rowRange", "colRange"]),
					TypeRefTypeHint::BoxedAsRef(Mut, "m", Lifetime::Custom("boxed")),
				),
			],
		),
		(
			"cv::Mat::reshape",
			vec![
				(
					pred!(const, ["cn", "rows"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
				(
					pred!(const, ["cn", "newndims", "newsz"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
				(
					pred!(const, ["cn", "newshape"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
			],
		),
		(
			"cv::Mat::row",
			vec![(
				pred!(const, ["y"]),
				TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
			)],
		),
		(
			"cv::Mat::col",
			vec![(
				pred!(const, ["x"]),
				TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
			)],
		),
		(
			"cv::Mat::rowRange",
			vec![
				(
					pred!(const, ["startrow", "endrow"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
				(
					pred!(const, ["r"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
			],
		),
		(
			"cv::Mat::colRange",
			vec![
				(
					pred!(const, ["startcol", "endcol"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
				(
					pred!(const, ["r"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
			],
		),
		(
			"cv::Mat::diag",
			vec![(
				pred!(const, ["d"]),
				TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
			)],
		),
		(
			"cv::Mat::operator()",
			vec![
				(
					pred!(const, ["roi"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
				(
					pred!(const, ["rowRange", "colRange"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
				(
					pred!(const, ["ranges"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
			],
		),
		// GpuMatND
		(
			"cv::cuda::GpuMatND::createGpuMatHeader",
			vec![
				(
					pred!(const, ["idx", "rowRange", "colRange"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
				(
					pred!(const, []),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
			],
		),
		(
			"cv::cuda::GpuMatND::operator()",
			vec![
				(
					pred!(const, ["idx", "rowRange", "colRange"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
				(
					pred!(const, ["ranges"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
			],
		),
		// GpuMat
		(
			"cv::cuda::GpuMat::GpuMat",
			vec![
				(
					pred!(mut, ["m", "rowRange", "colRange"]),
					TypeRefTypeHint::BoxedAsRef(Mut, "m", Lifetime::Elided),
				),
				(
					pred!(mut, ["m", "roi"]),
					TypeRefTypeHint::BoxedAsRef(Mut, "m", Lifetime::Elided),
				),
			],
		),
		(
			"cv::cuda::GpuMat::row",
			vec![(
				pred!(const, ["y"]),
				TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
			)],
		),
		(
			"cv::cuda::GpuMat::col",
			vec![(
				pred!(const, ["x"]),
				TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
			)],
		),
		(
			"cv::cuda::GpuMat::rowRange",
			vec![
				(
					pred!(const, ["startrow", "endrow"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
				(
					pred!(const, ["r"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
			],
		),
		(
			"cv::cuda::GpuMat::colRange",
			vec![
				(
					pred!(const, ["startcol", "endcol"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
				(
					pred!(const, ["r"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
			],
		),
		(
			"cv::cuda::GpuMat::reshape",
			vec![(
				pred!(const, ["cn", "rows"]),
				TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
			)],
		),
		(
			"cv::cuda::GpuMat::operator()",
			vec![
				(
					pred!(const, ["rowRange", "colRange"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
				(
					pred!(const, ["roi"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
			],
		),
		// UMat
		(
			"cv::UMat::UMat",
			vec![
				(
					pred!(mut, ["m", "rowRange", "colRange"]),
					TypeRefTypeHint::BoxedAsRef(Mut, "m", Lifetime::Custom("boxed")),
				),
				(
					pred!(mut, ["m", "roi"]),
					TypeRefTypeHint::BoxedAsRef(Mut, "m", Lifetime::Elided),
				),
				(
					pred!(mut, ["m", "ranges"]),
					TypeRefTypeHint::BoxedAsRef(Mut, "m", Lifetime::Custom("boxed")),
				),
			],
		),
		(
			"cv::UMat::row",
			vec![(
				pred!(const, ["y"]),
				TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
			)],
		),
		(
			"cv::UMat::col",
			vec![(
				pred!(const, ["x"]),
				TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
			)],
		),
		(
			"cv::UMat::rowRange",
			vec![
				(
					pred!(const, ["startrow", "endrow"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
				(
					pred!(const, ["r"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
			],
		),
		(
			"cv::UMat::colRange",
			vec![
				(
					pred!(const, ["startcol", "endcol"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
				(
					pred!(const, ["r"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
			],
		),
		(
			"cv::UMat::diag",
			vec![(
				pred!(const, ["d"]),
				TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
			)],
		),
		(
			"cv::UMat::reshape",
			vec![
				(
					pred!(const, ["cn", "rows"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
				(
					pred!(const, ["cn", "newndims", "newsz"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
			],
		),
		(
			"cv::UMat::operator()",
			vec![
				(
					pred!(const, ["rowRange", "colRange"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
				(
					pred!(const, ["roi"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
				(
					pred!(const, ["ranges"]),
					TypeRefTypeHint::BoxedAsRef(Mut, ARG_OVERRIDE_SELF, Lifetime::Elided),
				),
			],
		),
		(
			"cv::_InputArray::_InputArray",
			vec![
				(pred!(mut, ["m"]), TypeRefTypeHint::BoxedAsRef(Const, "m", Lifetime::Elided)),
				(
					pred!(mut, ["expr"]),
					TypeRefTypeHint::BoxedAsRef(Const, "expr", Lifetime::Elided),
				),
				(
					pred!(mut, ["vec"]),
					TypeRefTypeHint::BoxedAsRef(Const, "vec", Lifetime::Elided),
				),
				(
					pred!(mut, ["val"]),
					TypeRefTypeHint::BoxedAsRef(Const, "val", Lifetime::Elided),
				),
				(
					pred!(mut, ["d_mat"]),
					TypeRefTypeHint::BoxedAsRef(Const, "d_mat", Lifetime::Elided),
				),
				(
					pred!(mut, ["d_mat_array"]),
					TypeRefTypeHint::BoxedAsRef(Const, "d_mat_array", Lifetime::Elided),
				),
				(
					pred!(mut, ["buf"]),
					TypeRefTypeHint::BoxedAsRef(Const, "buf", Lifetime::Elided),
				),
				(
					pred!(mut, ["cuda_mem"]),
					TypeRefTypeHint::BoxedAsRef(Const, "cuda_mem", Lifetime::Elided),
				),
				(pred!(mut, ["um"]), TypeRefTypeHint::BoxedAsRef(Const, "um", Lifetime::Elided)),
				(
					pred!(mut, ["umv"]),
					TypeRefTypeHint::BoxedAsRef(Const, "umv", Lifetime::Elided),
				),
			],
		),
		(
			"cv::_OutputArray::_OutputArray",
			vec![
				(pred!(mut, ["m"]), TypeRefTypeHint::BoxedAsRef(Const, "m", Lifetime::Elided)),
				(
					pred!(mut, ["vec"]),
					TypeRefTypeHint::BoxedAsRef(Const, "vec", Lifetime::Elided),
				),
				(
					pred!(mut, ["d_mat"]),
					TypeRefTypeHint::BoxedAsRef(Const, "d_mat", Lifetime::Elided),
				),
				(
					pred!(mut, ["buf"]),
					TypeRefTypeHint::BoxedAsRef(Const, "buf", Lifetime::Elided),
				),
				(
					pred!(mut, ["cuda_mem"]),
					TypeRefTypeHint::BoxedAsRef(Const, "cuda_mem", Lifetime::Elided),
				),
			],
		),
		(
			"cv::_InputOutputArray::_InputOutputArray",
			vec![
				(pred!(mut, ["m"]), TypeRefTypeHint::BoxedAsRef(Const, "m", Lifetime::Elided)),
				(
					pred!(mut, ["vec"]),
					TypeRefTypeHint::BoxedAsRef(Const, "vec", Lifetime::Elided),
				),
				(
					pred!(mut, ["d_mat"]),
					TypeRefTypeHint::BoxedAsRef(Const, "d_mat", Lifetime::Elided),
				),
				(
					pred!(mut, ["buf"]),
					TypeRefTypeHint::BoxedAsRef(Const, "buf", Lifetime::Elided),
				),
				(
					pred!(mut, ["cuda_mem"]),
					TypeRefTypeHint::BoxedAsRef(Const, "cuda_mem", Lifetime::Elided),
				),
			],
		),
	]))
}

fn objdetect_return_override_factory() -> ReturnOverride {
	FuncMatcher::create(HashMap::from([
		(
			"cv::QRCodeDetector::decode",
			vec![(
				pred!(mut, ["img", "points", "straight_qrcode"]),
				TypeRefTypeHint::StringAsBytes(None),
			)],
		),
		(
			"cv::QRCodeDetector::decodeCurved",
			vec![(
				pred!(mut, ["img", "points", "straight_qrcode"]),
				TypeRefTypeHint::StringAsBytes(None),
			)],
		),
		(
			"cv::QRCodeDetector::detectAndDecode",
			vec![(
				pred!(mut, ["img", "points", "straight_qrcode"]),
				TypeRefTypeHint::StringAsBytes(None),
			)],
		),
		(
			"cv::QRCodeDetector::detectAndDecodeCurved",
			vec![(
				pred!(mut, ["img", "points", "straight_qrcode"],),
				TypeRefTypeHint::StringAsBytes(None),
			)],
		),
		(
			"cv::GraphicalCodeDetector::decode",
			vec![(
				pred!(const, ["img", "points", "straight_code"]),
				TypeRefTypeHint::StringAsBytes(None),
			)],
		),
		(
			"cv::GraphicalCodeDetector::detectAndDecode",
			vec![(
				pred!(const, ["img", "points", "straight_code"],),
				TypeRefTypeHint::StringAsBytes(None),
			)],
		),
	]))
}
