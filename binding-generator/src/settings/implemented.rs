use std::collections::HashSet;
use std::sync::LazyLock;

pub static IMPLEMENTED_FUNCTION_LIKE_MACROS: LazyLock<HashSet<&str>> = LazyLock::new(|| HashSet::from(["CV_MAKETYPE"]));

pub static IMPLEMENTED_SYSTEM_CLASSES: LazyLock<HashSet<&str>> = LazyLock::new(|| {
	HashSet::from([
		"std::pair",
		"std::string",
		"std::tuple",
		"std::vector",
		"ID3D11Device",
		"ID3D11Texture2D",
		"ID3D10Device",
		"ID3D10Texture2D",
		"IDirect3DDevice9",
		"IDirect3DDevice9Ex",
		"IDirect3DSurface9",
	])
});

/// classes that have a manual `Debug` implementation, element is cpp_name(Reference)
pub static IMPLEMENTED_MANUAL_DEBUG: LazyLock<HashSet<&str>> =
	LazyLock::new(|| HashSet::from(["cv::Mat", "cv::MatSize", "cv::dnn::DictValue"]));

/// cpp_name(Reference)
pub static IMPLEMENTED_CONST_GENERICS: LazyLock<HashSet<&str>> = LazyLock::new(|| HashSet::from(["cv::Vec"]));

/// cpp_name(Reference)
pub static IMPLEMENTED_GENERICS: LazyLock<HashSet<&str>> = LazyLock::new(|| {
	let mut out = HashSet::from([
		"cv::Affine3",
		"cv::Mat_",
		"cv::Matx",
		"cv::Point3_",
		"cv::Point_",
		"cv::Rect_",
		"cv::Scalar_",
		"cv::Size_",
	]);
	out.extend(&*IMPLEMENTED_CONST_GENERICS);
	out
});
