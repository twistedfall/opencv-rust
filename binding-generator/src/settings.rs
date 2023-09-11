// todo add doccomments

use std::collections::{BTreeSet, HashMap, HashSet};

use once_cell::sync::Lazy;

pub use argument_override::{ArgOverride, ARGUMENT_OVERRIDE};
pub use element_exclude_kind::ELEMENT_EXCLUDE_KIND;
pub use element_export_tweak::ELEMENT_EXPORT_TWEAK;
pub use func_cfg_attr::FUNC_CFG_ATTR;
pub use func_exclude::FUNC_EXCLUDE;
pub use func_inject::{FuncFactory, FUNC_INJECT_MANUAL};
pub use func_rename::FUNC_RENAME;
pub use func_specialize::{TypeRefFactory, FUNC_SPECIALIZE};
pub use func_unsafe::FUNC_UNSAFE;
pub use generator_module_tweaks::{ModuleTweak, GENERATOR_MODULE_TWEAKS};

use crate::{CompiledInterpolation, FuncId, StrExt};

mod argument_override;
mod element_exclude_kind;
mod element_export_tweak;
mod func_cfg_attr;
mod func_exclude;
mod func_inject;
mod func_rename;
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

/// dict of functions with manual implementations
/// key: FuncInfo.identifier
/// value: dict
///   keys: "rust_safe", "rust_extern", "cpp", missing key means skip particular implementation
///   values: template to use for manual implementation or "~" to use default implementation
pub static FUNC_MANUAL: Lazy<HashMap<&str, CompiledInterpolation>> = Lazy::new(|| {
	HashMap::from([
		(
			"cv_Mat_at_int",
			include_str!("../tpl/settings/rust_mat_at_mut.tpl.rs").compile_interpolation(),
		),
		(
			"cv_Mat_at_const_int",
			include_str!("../tpl/settings/rust_mat_at_const.tpl.rs").compile_interpolation(),
		),
		(
			"cv_Mat_at_int_int",
			include_str!("../tpl/settings/rust_mat_at_mut.tpl.rs").compile_interpolation(),
		),
		(
			"cv_Mat_at_const_int_int",
			include_str!("../tpl/settings/rust_mat_at_const.tpl.rs").compile_interpolation(),
		),
		(
			"cv_Mat_at_Point",
			include_str!("../tpl/settings/rust_mat_at_mut.tpl.rs").compile_interpolation(),
		),
		(
			"cv_Mat_at_const_Point",
			include_str!("../tpl/settings/rust_mat_at_const.tpl.rs").compile_interpolation(),
		),
		(
			"cv_Mat_at_int_int_int",
			include_str!("../tpl/settings/rust_mat_at_mut.tpl.rs").compile_interpolation(),
		),
		(
			"cv_Mat_at_const_int_int_int",
			include_str!("../tpl/settings/rust_mat_at_const.tpl.rs").compile_interpolation(),
		),
		(
			"cv_Mat_at_const_intX",
			include_str!("../tpl/settings/rust_mat_at_mut.tpl.rs").compile_interpolation(),
		),
		(
			"cv_Mat_at_const_const_intX",
			include_str!("../tpl/settings/rust_mat_at_const.tpl.rs").compile_interpolation(),
		),
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
		FuncId::new("cv::noArray", []),
		FuncId::new("cv::getVersionMajor", []),
		FuncId::new("cv::getVersionMinor", []),
		FuncId::new("cv::getVersionRevision", []),
		// not doing anything that can cause an exception
		FuncId::new("cv::Mat::empty", []),
		FuncId::new("cv::Mat::total", []),
		FuncId::new("cv::Mat::isContinuous", []),
		FuncId::new("cv::Mat::isSubmatrix", []),
		FuncId::new("cv::Mat::elemSize1", []),
		FuncId::new("cv::Mat::type", []),
		FuncId::new("cv::Mat::depth", []),
		FuncId::new("cv::Mat::channels", []),
		FuncId::new("cv::UMat::empty", []),
		FuncId::new("cv::UMat::total", []),
		FuncId::new("cv::UMat::isContinuous", []),
		FuncId::new("cv::UMat::isSubmatrix", []),
		FuncId::new("cv::UMat::elemSize1", []),
		FuncId::new("cv::UMat::type", []),
		FuncId::new("cv::UMat::depth", []),
		FuncId::new("cv::UMat::channels", []),
		FuncId::new("cv::SparseMat::elemSize", []),
		FuncId::new("cv::SparseMat::elemSize1", []),
		FuncId::new("cv::SparseMat::type", []),
		FuncId::new("cv::SparseMat::depth", []),
		FuncId::new("cv::SparseMat::channels", []),
		// marked CV_NOEXCEPT since OpenCV 4.5.2, propagate those changes to earlier versions
		FuncId::new("cv::Mat::Mat", []),
		FuncId::new("cv::MatSize::MatSize", ["_p"]),
		FuncId::new("cv::MatSize::dims", []),
		FuncId::new("cv::MatSize::operator const int *", []),
		FuncId::new("cv::MatStep::MatStep", []),
		FuncId::new("cv::MatStep::operator[]", ["i"]),
		FuncId::new("cv::UMat::UMat", ["usageFlags"]),
		FuncId::new("cv::ocl::Context::Context", []),
		FuncId::new("cv::ocl::Device::Device", []),
		FuncId::new("cv::ocl::Image2D::Image2D", []),
		FuncId::new("cv::ocl::Kernel::Kernel", []),
		FuncId::new("cv::ocl::KernelArg::KernelArg", []),
		FuncId::new("cv::ocl::Platform::Platform", []),
		FuncId::new("cv::ocl::PlatformInfo::PlatformInfo", []),
		FuncId::new("cv::ocl::Program::Program", []),
		FuncId::new("cv::ocl::ProgramSource::ProgramSource", []),
		FuncId::new("cv::ocl::Queue::Queue", []),
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
