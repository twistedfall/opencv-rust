// todo add doccomments

macro_rules! pred {
	($args: expr $(,)?) => {
		[$crate::func::Pred::ArgNames(&$args)].as_slice()
	};
	($args: expr, $types: expr $(,)?) => {
		[$crate::func::Pred::ArgNames(&$args), $crate::func::Pred::ArgTypes(&$types)].as_slice()
	};
	(const, $args: expr $(,)?) => {
		[
			$crate::func::Pred::Constness($crate::type_ref::Constness::Const),
			$crate::func::Pred::ArgNames(&$args),
		]
		.as_slice()
	};
	(const, $args: expr, $types: expr $(,)?) => {
		[
			$crate::func::Pred::Constness($crate::type_ref::Constness::Const),
			$crate::func::Pred::ArgNames(&$args),
			$crate::func::Pred::ArgTypes(&$types),
		]
		.as_slice()
	};
	(mut, $args: expr $(,)?) => {
		[
			$crate::func::Pred::Constness($crate::type_ref::Constness::Mut),
			$crate::func::Pred::ArgNames(&$args),
		]
		.as_slice()
	};
	(mut, $args: expr, $types: expr $(,)?) => {
		[
			$crate::func::Pred::Constness($crate::type_ref::Constness::Mut),
			$crate::func::Pred::ArgNames(&$args),
			$crate::func::Pred::ArgTypes(&$types),
		]
		.as_slice()
	};
}

use std::collections::{BTreeSet, HashMap, HashSet};

pub use argument_names::{ARGUMENT_NAMES_MULTIPLE_SLICE, ARGUMENT_NAMES_NOT_SLICE, ARGUMENT_NAMES_USERDATA};
pub use argument_override::{
	arg_override_factory, property_override_factory, return_override_factory, ArgOverride, PropertyOverride, ReturnOverride,
	ARG_OVERRIDE_SELF,
};
pub use element_exclude_kind::ELEMENT_EXCLUDE_KIND;
pub use element_export_tweak::ELEMENT_EXPORT_TWEAK;
pub use force_infallible::{force_infallible_factory, ForceInfallible};
pub use func_cfg_attr::FUNC_CFG_ATTR;
pub use func_companion_tweak::{func_companion_tweak_factory, CompanionTweak, FuncCompanionTweak};
pub use func_exclude::{func_exclude_factory, FuncExclude};
pub use func_inject::{func_inject_factory, FuncFactory, FuncInject};
pub use func_rename::{func_rename_factory, FuncRename};
pub use func_replace::{func_replace_factory, FuncInheritFactory, FuncReplace};
pub use func_specialize::{func_specialize_factory, FuncSpec, FuncSpecialize};
pub use func_unsafe::{func_unsafe_factory, FuncUnsafe};
pub use generator_module_tweaks::{generator_module_tweaks_factory, ModuleTweak};
pub use implemented::{
	IMPLEMENTED_CONST_GENERICS, IMPLEMENTED_FUNCTION_LIKE_MACROS, IMPLEMENTED_GENERICS, IMPLEMENTED_MANUAL_DEBUG,
	IMPLEMENTED_SYSTEM_CLASSES,
};
use once_cell::sync::Lazy;
pub use property_tweaks::{property_tweaks_factory, PropertyReadWrite, PropertyTweak, PropertyTweaks};

use crate::func::{FuncMatcher, UsageTracker};
use crate::type_ref::TypeRef;

mod argument_names;
mod argument_override;
mod element_exclude_kind;
mod element_export_tweak;
mod force_infallible;
mod func_cfg_attr;
mod func_companion_tweak;
mod func_exclude;
mod func_inject;
mod func_rename;
mod func_replace;
mod func_specialize;
mod func_unsafe;
mod generator_module_tweaks;
mod implemented;
mod property_tweaks;

pub type TypeRefFactory = fn() -> TypeRef<'static, 'static>;

/// Injectable global and module level overrides, todo: migrate the global statics to this over time
#[derive(Debug)]
pub struct Settings {
	pub arg_override: ArgOverride,
	pub return_override: ReturnOverride,
	pub force_infallible: ForceInfallible,
	pub func_companion_tweak: FuncCompanionTweak,
	pub func_exclude: FuncExclude,
	pub func_inject: FuncInject,
	pub func_rename: FuncRename,
	pub func_replace: FuncReplace,
	pub func_specialize: FuncSpecialize,
	pub func_unsafe: FuncUnsafe,
	pub generator_module_tweaks: ModuleTweak<'static>,
	pub property_override: PropertyOverride,
	pub property_tweaks: PropertyTweaks,
}

impl Settings {
	pub fn empty() -> Self {
		Self {
			arg_override: ArgOverride::empty(),
			return_override: ReturnOverride::empty(),
			force_infallible: ForceInfallible::empty(),
			func_companion_tweak: FuncCompanionTweak::empty(),
			func_exclude: FuncExclude::default(),
			func_inject: FuncInject::default(),
			func_rename: FuncRename::default(),
			func_replace: FuncReplace::empty(),
			func_specialize: FuncMatcher::empty(),
			func_unsafe: FuncUnsafe::empty(),
			generator_module_tweaks: ModuleTweak::empty(),
			property_override: PropertyOverride::default(),
			property_tweaks: PropertyTweaks::default(),
		}
	}

	pub fn for_module(module: &str) -> Self {
		Self {
			arg_override: arg_override_factory(module),
			return_override: return_override_factory(module),
			force_infallible: force_infallible_factory(module),
			func_companion_tweak: func_companion_tweak_factory(module),
			func_exclude: func_exclude_factory(module),
			func_inject: func_inject_factory(module),
			func_rename: func_rename_factory(module),
			func_replace: func_replace_factory(module),
			func_specialize: func_specialize_factory(module),
			func_unsafe: func_unsafe_factory(module),
			generator_module_tweaks: generator_module_tweaks_factory(module),
			property_override: property_override_factory(module),
			property_tweaks: property_tweaks_factory(module),
		}
	}

	pub fn start_usage_tracking(&mut self) {
		self.arg_override.start_usage_tracking();
		self.return_override.start_usage_tracking();
		self.force_infallible.start_usage_tracking();
		self.func_companion_tweak.start_usage_tracking();
		self.func_replace.start_usage_tracking();
		self.func_specialize.start_usage_tracking();
		self.func_unsafe.start_usage_tracking();
	}

	pub fn finish_usage_tracking(&mut self) -> HashMap<&'static str, HashSet<UsageTracker>> {
		HashMap::from([
			("ARG_OVERRIDE", self.arg_override.finish_usage_tracking()),
			("RETURN_OVERRIDE", self.return_override.finish_usage_tracking()),
			("FORCE_INFALLIBLE", self.force_infallible.finish_usage_tracking()),
			("FUNC_COMPANION_TWEAK", self.func_companion_tweak.finish_usage_tracking()),
			("FUNC_REPLACE", self.func_replace.finish_usage_tracking()),
			("FUNC_SPECIALIZE", self.func_specialize.finish_usage_tracking()),
			("FUNC_UNSAFE", self.func_unsafe.finish_usage_tracking()),
		])
	}
}

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

/// Types that can be used as `Mat` element
/// cpp_name(Reference)
pub static DATA_TYPES: Lazy<HashSet<&str>> = Lazy::new(|| {
	HashSet::from([
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
