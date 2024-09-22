use std::borrow::Cow;
use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::class::ClassDesc;
use crate::func::{FuncCppBody, FuncDesc, FuncKind, FuncRustBody, FuncRustExtern, InheritConfig, ReturnKind};
use crate::type_ref::{Constness, TypeRef};
use crate::{Func, FuncId};

pub type FuncInheritFactory = for<'tu, 'ge> fn(&Func<'tu, 'ge>) -> Func<'tu, 'ge>;

pub static FUNC_REPLACE: Lazy<HashMap<FuncId, FuncInheritFactory>> = Lazy::new(|| {
	const MAT_FORWARD: Cow<str> = Cow::Borrowed("core::mat_forward::{{name}}(self, {{forward_args}})");
	const MAT_FORWARD_INHERIT_CONFIG: InheritConfig = InheritConfig {
		kind: false,
		name: false,
		doc_comment: true,
		arguments: true,
		return_type_ref: false,
		definition_location: true,
	};

	let forward_const = (|f| {
		let mut out = Func::new_desc(
			FuncDesc::new(
				FuncKind::InstanceMethod(ClassDesc::cv_mat()),
				Constness::Const,
				ReturnKind::Fallible,
				"at",
				"core",
				[],
				TypeRef::new_pointer(TypeRef::new_generic("T").with_inherent_constness(Constness::Const)),
			)
			.cpp_body(FuncCppBody::Absent)
			.rust_body(FuncRustBody::ManualCallReturn(MAT_FORWARD))
			.rust_extern_definition(FuncRustExtern::Absent)
			.rust_generic_decls([("T".to_string(), "core::DataType".to_string())]),
		);
		out.inherit(f, MAT_FORWARD_INHERIT_CONFIG);
		out
	}) as FuncInheritFactory;

	let forward_mut = (|f| {
		let mut out = Func::new_desc(
			FuncDesc::new(
				FuncKind::InstanceMethod(ClassDesc::cv_mat()),
				Constness::Mut,
				ReturnKind::Fallible,
				"at",
				"core",
				[],
				TypeRef::new_pointer(TypeRef::new_generic("T")),
			)
			.cpp_body(FuncCppBody::Absent)
			.rust_body(FuncRustBody::ManualCallReturn(MAT_FORWARD))
			.rust_extern_definition(FuncRustExtern::Absent)
			.rust_generic_decls([("T".to_string(), "core::DataType".to_string())]),
		);
		out.inherit(f, MAT_FORWARD_INHERIT_CONFIG);
		out
	}) as FuncInheritFactory;

	HashMap::from([
		(FuncId::new_mut("cv::Mat::at", ["i0"]), forward_mut),
		(FuncId::new_const("cv::Mat::at", ["i0"]), forward_const),
		(FuncId::new_mut("cv::Mat::at", ["row", "col"]), forward_mut),
		(FuncId::new_const("cv::Mat::at", ["row", "col"]), forward_const),
		(FuncId::new_mut("cv::Mat::at", ["i0", "i1", "i2"]), forward_mut),
		(FuncId::new_const("cv::Mat::at", ["i0", "i1", "i2"]), forward_const),
		(FuncId::new_mut("cv::Mat::at", ["pt"]), forward_mut),
		(FuncId::new_const("cv::Mat::at", ["pt"]), forward_const),
		(FuncId::new_mut("cv::Mat::at", ["idx"]), forward_mut),
		(FuncId::new_const("cv::Mat::at", ["idx"]), forward_const),
	])
});
