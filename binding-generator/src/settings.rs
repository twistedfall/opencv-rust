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
pub use class_tweaks::{class_tweaks_factory, ClassTweak, ClassTweaks};
pub use const_tweak::CONST_TYPE_OVERRIDE;
pub use element_exclude_kind::ELEMENT_EXCLUDE_KIND;
pub use element_export_tweak::ELEMENT_EXPORT_TWEAK;
pub use force_infallible::{force_infallible_factory, ForceInfallible};
pub use func_cfg_attr::{func_cfg_attr_factory, FuncCfgAttr, CFG_ATTR_NOT_ON_WINDOWS, CFG_ATTR_ONLY_OPENCV_5};
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
use crate::SupportedModule;

mod argument_names;
mod argument_override;
mod class_tweaks;
mod const_tweak;
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
	pub func_cfg_attr: FuncCfgAttr,
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
	pub class_tweak: ClassTweaks,
}

impl Settings {
	pub fn empty() -> Self {
		Self {
			arg_override: ArgOverride::empty(),
			return_override: ReturnOverride::empty(),
			force_infallible: ForceInfallible::empty(),
			func_cfg_attr: FuncCfgAttr::empty(),
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
			class_tweak: ClassTweaks::default(),
		}
	}

	pub fn for_module(module: SupportedModule) -> Self {
		Self {
			arg_override: arg_override_factory(module),
			return_override: return_override_factory(module),
			force_infallible: force_infallible_factory(module),
			func_cfg_attr: func_cfg_attr_factory(module),
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
			class_tweak: class_tweaks_factory(module),
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
		("where", "where_"),
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

pub static STATIC_RUST_MODULES: Lazy<BTreeSet<&str>> = Lazy::new(|| BTreeSet::from(["core", "sys", "types"]));

/// Types that can be used as `Mat` element
/// cpp_name(Reference)
pub static DATA_TYPES: Lazy<HashSet<&str>> = Lazy::new(|| {
	HashSet::from([
		"unsigned char",
		"char",
		"uint8_t",
		"int8_t",
		"unsigned short",
		"short",
		"uint16_t",
		"int16_t",
		"unsigned int",
		"int",
		"int32_t",
		"float",
		"double",
		"hfloat",
		"float16_t",
		"__fp16",
		"cv::Vec",
		"cv::Scalar_",
		"cv::Point_",
		"cv::Point3_",
		"cv::Size_",
		"cv::Rect_",
	])
});

/// Types that can be used as `Mat` element since OpenCV 5.0
/// cpp_name(Reference)
pub static DATA_TYPES_5_0: Lazy<HashSet<&str>> =
	Lazy::new(|| HashSet::from(["uint32_t", "bfloat", "bfloat16_t", "uint64_t", "int64_t", "bool"]));

pub static NO_SKIP_NAMESPACE_IN_LOCALNAME: Lazy<HashMap<Option<SupportedModule>, HashMap<&str, &str>>> = Lazy::new(|| {
	HashMap::from([
		(None, HashMap::from([("detail", "Detail")])),
		(Some(SupportedModule::Calib3d), HashMap::from([("fisheye", "Fisheye")])),
		(Some(SupportedModule::CudaBgSegm), HashMap::from([("cuda", "CUDA")])),
		(Some(SupportedModule::CudaCodec), HashMap::from([("cudacodec", "CUDA")])),
		(Some(SupportedModule::CudaFeatures2d), HashMap::from([("cuda", "CUDA")])),
		(Some(SupportedModule::CudaImgProc), HashMap::from([("cuda", "CUDA")])),
		(Some(SupportedModule::CudaLegacy), HashMap::from([("cuda", "CUDA")])),
		(Some(SupportedModule::CudaObjDetect), HashMap::from([("cuda", "CUDA")])),
		(Some(SupportedModule::CudaOptFlow), HashMap::from([("cuda", "CUDA")])),
		(Some(SupportedModule::CudaStereo), HashMap::from([("cuda", "CUDA")])),
		(Some(SupportedModule::Gapi), HashMap::from([("imgproc", "ImgProc")])),
		(Some(SupportedModule::Mcc), HashMap::from([("mcc", "MCC")])),
		(Some(SupportedModule::Rapid), HashMap::from([("rapid", "Rapid")])),
		(
			Some(SupportedModule::Rgbd),
			HashMap::from([
				("dynafu", "Dynafu"),
				("kinfu", "Kinfu"),
				("colored_kinfu", "ColoredKinfu"),
				("linemod", "LineMod"),
			]),
		),
		(Some(SupportedModule::Stitching), HashMap::from([("fisheye", "Fisheye")])),
		(Some(SupportedModule::SuperRes), HashMap::from([("superres", "SuperRes")])),
	])
});

pub static PREVENT_VECTOR_TYPEDEF_GENERATION: Lazy<HashSet<&str>> = Lazy::new(|| {
	HashSet::from([
		// `MatShape` is an alias to `Vector<i32>` and this leads to duplication of definition for the `Vector<Vector<i32>>` type
		"cv::dnn::MatShape",
		// `MatType` is an alias `i32` and we don't want to duplicate `Vector<i32>` with `Vector<MatType>`
		"cv::dnn::MatType",
	])
});
