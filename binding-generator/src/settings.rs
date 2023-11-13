// todo add doccomments

use std::collections::{BTreeSet, HashMap, HashSet};

use once_cell::sync::Lazy;

pub use argument_override::{ArgOverride, ARGUMENT_OVERRIDE};
pub use element_exclude_kind::ELEMENT_EXCLUDE_KIND;
pub use element_export_tweak::ELEMENT_EXPORT_TWEAK;
pub use func_cfg_attr::FUNC_CFG_ATTR;
pub use func_exclude::FUNC_EXCLUDE;
pub use func_inject::{FuncFactory, FUNC_INJECT};
pub use func_rename::FUNC_RENAME;
pub use func_replace::{FuncInheritFactory, FUNC_REPLACE};
pub use func_specialize::{TypeRefFactory, FUNC_SPECIALIZE};
pub use func_unsafe::FUNC_UNSAFE;
pub use generator_module_tweaks::{ModuleTweak, GENERATOR_MODULE_TWEAKS};

use crate::FuncId;

mod argument_override;
mod element_exclude_kind;
mod element_export_tweak;
mod func_cfg_attr;
mod func_exclude;
mod func_inject;
mod func_rename;
mod func_replace;
mod func_specialize;
mod func_unsafe;
mod generator_module_tweaks;

pub static IMPLEMENTED_FUNCTION_LIKE_MACROS: Lazy<HashSet<&str>> = Lazy::new(|| HashSet::from(["CV_MAKETYPE"]));

pub static IMPLEMENTED_SYSTEM_CLASSES: Lazy<HashSet<&str>> =
	Lazy::new(|| HashSet::from(["std::pair", "std::string", "std::tuple", "std::vector"]));

/// classes that have a manual `Debug` implementation, element is cpp_name(Reference)
pub static IMPLEMENTED_MANUAL_DEBUG: Lazy<HashSet<&str>> =
	Lazy::new(|| HashSet::from(["cv::Mat", "cv::MatSize", "cv::MatStep", "cv::dnn::DictValue"]));

// fixme, generalize, make it use constant::ValueKind
pub static CONST_TYPE_USIZE: Lazy<HashSet<&str>> = Lazy::new(|| HashSet::from(["Mat_AUTO_STEP"]));

/// map of reserved Rust keywords and their replacement to be used in var, function and class names
/// key: reserved keyword
/// value: replacement
pub static RESERVED_RENAME: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
	HashMap::from([
		("box", "box_"),
		("fn", "fn_"),
		("in", "in_"),
		("match", "match_"),
		("move", "move_"),
		("ref", "ref_"),
		("type", "typ"),
		("use", "use_"),
		("impl", "impl_"),
		("loop", "loop_"),
		("yield", "yield_"),
	])
});

/// set of classes that must be generated as traits, elements are Class.cpp_name(Reference)()
pub static FORCE_CLASS_ABSTRACT: Lazy<HashSet<&str>> = Lazy::new(|| HashSet::from(["cv::detail::BlocksCompensator"]));

/// cpp_name(Reference)
pub static FORCE_CONSTANT_METHOD: Lazy<HashSet<&str>> =
	Lazy::new(|| HashSet::from(["cv::Mat::size", "cv::Mat::step", "cv::UMat::size", "cv::UMat::step"]));

pub static FORCE_INFALLIBLE: Lazy<HashSet<FuncId>> = Lazy::new(|| {
	HashSet::from([
		// just returns static/constant data
		FuncId::new_mut("cv::noArray", []),
		FuncId::new_mut("cv::getVersionMajor", []),
		FuncId::new_mut("cv::getVersionMinor", []),
		FuncId::new_mut("cv::getVersionRevision", []),
		// not doing anything that can cause an exception
		FuncId::new_const("cv::Mat::empty", []),
		FuncId::new_const("cv::Mat::total", []),
		FuncId::new_const("cv::Mat::isContinuous", []),
		FuncId::new_const("cv::Mat::isSubmatrix", []),
		FuncId::new_const("cv::Mat::elemSize1", []),
		FuncId::new_const("cv::Mat::type", []),
		FuncId::new_const("cv::Mat::depth", []),
		FuncId::new_const("cv::Mat::channels", []),
		FuncId::new_const("cv::UMat::empty", []),
		FuncId::new_const("cv::UMat::total", []),
		FuncId::new_const("cv::UMat::isContinuous", []),
		FuncId::new_const("cv::UMat::isSubmatrix", []),
		FuncId::new_const("cv::UMat::elemSize1", []),
		FuncId::new_const("cv::UMat::type", []),
		FuncId::new_const("cv::UMat::depth", []),
		FuncId::new_const("cv::UMat::channels", []),
		FuncId::new_const("cv::SparseMat::elemSize", []),
		FuncId::new_const("cv::SparseMat::elemSize1", []),
		FuncId::new_const("cv::SparseMat::type", []),
		FuncId::new_const("cv::SparseMat::depth", []),
		FuncId::new_const("cv::SparseMat::channels", []),
		// marked CV_NOEXCEPT since OpenCV 4.5.2, propagate those changes to earlier versions
		FuncId::new_mut("cv::Mat::Mat", []),
		FuncId::new_mut("cv::MatSize::MatSize", ["_p"]),
		FuncId::new_const("cv::MatSize::dims", []),
		FuncId::new_const("cv::MatSize::operator const int *", []),
		FuncId::new_mut("cv::MatStep::MatStep", []),
		FuncId::new_mut("cv::MatStep::operator[]", ["i"]),
		FuncId::new_mut("cv::UMat::UMat", ["usageFlags"]),
		FuncId::new_mut("cv::ocl::Context::Context", []),
		FuncId::new_mut("cv::ocl::Device::Device", []),
		FuncId::new_mut("cv::ocl::Image2D::Image2D", []),
		FuncId::new_mut("cv::ocl::Kernel::Kernel", []),
		FuncId::new_mut("cv::ocl::KernelArg::KernelArg", []),
		FuncId::new_mut("cv::ocl::Platform::Platform", []),
		FuncId::new_mut("cv::ocl::PlatformInfo::PlatformInfo", []),
		FuncId::new_mut("cv::ocl::Program::Program", []),
		FuncId::new_mut("cv::ocl::ProgramSource::ProgramSource", []),
		FuncId::new_mut("cv::ocl::Queue::Queue", []),
	])
});

/// cpp_name(Reference) => ( rust_name(Reference(No)), cpp_name(Reference) )
pub static PRIMITIVE_TYPEDEFS: Lazy<HashMap<&str, (&str, &str)>> = Lazy::new(|| {
	HashMap::from([
		("size_t", ("size_t", "size_t")),
		("ptrdiff_t", ("ptrdiff_t", "ptrdiff_t")),
		("clock_t", ("clock_t", "clock_t")),
		("schar", ("i8", "signed char")),
		("uchar", ("u8", "unsigned char")),
		("uint8_t", ("u8", "uint8_t")),
		("uint16_t", ("u16", "uint16_t")),
		("uint", ("u32", "unsigned int")),
		("uint32_t", ("u32", "uint32_t")),
		("int64_t", ("i64", "int64_t")),
		("int64", ("i64", "int64_t")),
		("uint64_t", ("u64", "uint64_t")),
		("uint64", ("u64", "uint64_t")),
		("ushort", ("u16", "unsigned short")),
	])
});

pub static STATIC_MODULES: Lazy<BTreeSet<&str>> = Lazy::new(|| BTreeSet::from(["core", "sys", "types"]));

// fixme, it can be made better
pub static DATA_TYPES: Lazy<BTreeSet<&str>> = Lazy::new(|| {
	BTreeSet::from([
		"unsigned char",
		"char",
		"uint8_t",
		"unsigned short",
		"short",
		"int",
		"float",
		"double",
		"cv::Vec2b",
		"cv::Vec3b",
		"cv::Vec4b",
		"cv::Vec2s",
		"cv::Vec3s",
		"cv::Vec4s",
		"cv::Vec2w",
		"cv::Vec3w",
		"cv::Vec4w",
		"cv::Vec2i",
		"cv::Vec3i",
		"cv::Vec4i",
		"cv::Vec6i",
		"cv::Vec8i",
		"cv::Vec2f",
		"cv::Vec3f",
		"cv::Vec4f",
		"cv::Vec6f",
		"cv::Vec2d",
		"cv::Vec3d",
		"cv::Vec4d",
		"cv::Vec6d",
		"cv::Scalar",
		"cv::Point",
		"cv::Point2i",
		"cv::Point2f",
		"cv::Point2d",
		"cv::Point2l",
		"cv::Point3i",
		"cv::Point3f",
		"cv::Point3d",
		"cv::Size",
		"cv::Size2i",
		"cv::Size2f",
		"cv::Size2d",
		"cv::Size2l",
		"cv::Rect",
		"cv::Rect2i",
		"cv::Rect2f",
		"cv::Rect2d",
	])
});

/// cpp_name(Reference)
pub static IMPLEMENTED_CONST_GENERICS: Lazy<HashSet<&str>> = Lazy::new(|| HashSet::from(["cv::Vec"]));

/// cpp_name(Reference)
pub static IMPLEMENTED_GENERICS: Lazy<HashSet<&str>> = Lazy::new(|| {
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

pub static NO_SKIP_NAMESPACE_IN_LOCALNAME: Lazy<HashMap<&str, HashMap<&str, &str>>> = Lazy::new(|| {
	HashMap::from([
		("*", HashMap::from([("detail", "Detail")])),
		("calib3d", HashMap::from([("fisheye", "Fisheye")])),
		("cudabgsegm", HashMap::from([("cuda", "CUDA")])),
		("cudacodec", HashMap::from([("cudacodec", "CUDA")])),
		("cudafeatures2d", HashMap::from([("cuda", "CUDA")])),
		("cudaimgproc", HashMap::from([("cuda", "CUDA")])),
		("cudaobjdetect", HashMap::from([("cuda", "CUDA")])),
		("cudaoptflow", HashMap::from([("cuda", "CUDA")])),
		("cudastereo", HashMap::from([("cuda", "CUDA")])),
		("gapi", HashMap::from([("imgproc", "ImgProc")])),
		("mcc", HashMap::from([("mcc", "MCC")])),
		("rapid", HashMap::from([("rapid", "Rapid")])),
		(
			"rgbd",
			HashMap::from([
				("dynafu", "Dynafu"),
				("kinfu", "Kinfu"),
				("colored_kinfu", "ColoredKinfu"),
				("linemod", "LineMod"),
			]),
		),
		("stitching", HashMap::from([("fisheye", "Fisheye")])),
		("superres", HashMap::from([("superres", "SuperRes")])),
	])
});

pub static PREVENT_VECTOR_TYPEDEF_GENERATION: Lazy<HashSet<&str>> = Lazy::new(|| {
	HashSet::from([
		// `MatShape` is an alias to `Vector<i32>` and this leads to duplication of definition for that type
		"cv::dnn::MatShape",
	])
});
