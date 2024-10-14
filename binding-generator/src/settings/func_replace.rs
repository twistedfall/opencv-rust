use std::borrow::Cow;
use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::class::ClassDesc;
use crate::func::{FuncCppBody, FuncDesc, FuncKind, FuncRustBody, FuncRustExtern, InheritConfig, ReturnKind};
use crate::type_ref::{Constness, TypeRef};
use crate::{Func, FuncId};

pub type FuncInheritFactory = for<'tu, 'ge> fn(&Func<'tu, 'ge>) -> Func<'tu, 'ge>;

pub static FUNC_REPLACE: Lazy<HashMap<FuncId, FuncInheritFactory>> = Lazy::new(|| {
	const MAT_FORWARD_INHERIT_CONFIG: InheritConfig = InheritConfig {
		kind: false,
		name: false,
		doc_comment: true,
		arguments: true,
		return_type_ref: false,
		definition_location: true,
	};

	fn make_at_forward(constness: Constness) -> FuncDesc<'static, 'static> {
		FuncDesc::new(
			FuncKind::InstanceMethod(ClassDesc::cv_mat()),
			constness,
			ReturnKind::Fallible,
			"at",
			"core",
			[],
			TypeRef::new_pointer(TypeRef::new_generic("T").with_inherent_constness(constness)),
		)
		.cpp_body(FuncCppBody::Absent)
		.rust_body(FuncRustBody::ManualCallReturn(Cow::Borrowed(
			"core::mat_forward::{{name}}(self, {{forward_args}})",
		)))
		.rust_extern_definition(FuncRustExtern::Absent)
		.rust_generic_decls([("T".to_string(), "core::DataType".to_string())])
	}

	HashMap::from([
		(
			FuncId::new_mut("cv::Mat::at", ["i0"]),
			(|f| {
				Func::new_desc(make_at_forward(Constness::Mut).rust_custom_leafname("at_mut"))
					.inheriting(f, MAT_FORWARD_INHERIT_CONFIG)
			}) as FuncInheritFactory,
		),
		(
			FuncId::new_const("cv::Mat::at", ["i0"]),
			(|f| Func::new_desc(make_at_forward(Constness::Const)).inheriting(f, MAT_FORWARD_INHERIT_CONFIG)) as FuncInheritFactory,
		),
		(
			FuncId::new_mut("cv::Mat::at", ["row", "col"]),
			(|f| {
				Func::new_desc(make_at_forward(Constness::Mut).rust_custom_leafname("at_2d_mut"))
					.inheriting(f, MAT_FORWARD_INHERIT_CONFIG)
			}) as FuncInheritFactory,
		),
		(
			FuncId::new_const("cv::Mat::at", ["row", "col"]),
			(|f| {
				Func::new_desc(make_at_forward(Constness::Const).rust_custom_leafname("at_2d"))
					.inheriting(f, MAT_FORWARD_INHERIT_CONFIG)
			}) as FuncInheritFactory,
		),
		(
			FuncId::new_mut("cv::Mat::at", ["i0", "i1", "i2"]),
			(|f| {
				Func::new_desc(make_at_forward(Constness::Mut).rust_custom_leafname("at_3d_mut"))
					.inheriting(f, MAT_FORWARD_INHERIT_CONFIG)
			}) as FuncInheritFactory,
		),
		(
			FuncId::new_const("cv::Mat::at", ["i0", "i1", "i2"]),
			(|f| {
				Func::new_desc(make_at_forward(Constness::Const).rust_custom_leafname("at_3d"))
					.inheriting(f, MAT_FORWARD_INHERIT_CONFIG)
			}) as FuncInheritFactory,
		),
		(
			FuncId::new_mut("cv::Mat::at", ["pt"]),
			(|f| {
				Func::new_desc(make_at_forward(Constness::Mut).rust_custom_leafname("at_pt_mut"))
					.inheriting(f, MAT_FORWARD_INHERIT_CONFIG)
			}) as FuncInheritFactory,
		),
		(
			FuncId::new_const("cv::Mat::at", ["pt"]),
			(|f| {
				Func::new_desc(make_at_forward(Constness::Const).rust_custom_leafname("at_pt"))
					.inheriting(f, MAT_FORWARD_INHERIT_CONFIG)
			}) as FuncInheritFactory,
		),
		(
			FuncId::new_mut("cv::Mat::at", ["idx"]),
			(|f| {
				Func::new_desc(make_at_forward(Constness::Mut).rust_custom_leafname("at_nd_mut"))
					.inheriting(f, MAT_FORWARD_INHERIT_CONFIG)
			}) as FuncInheritFactory,
		),
		(
			FuncId::new_const("cv::Mat::at", ["idx"]),
			(|f| {
				Func::new_desc(make_at_forward(Constness::Const).rust_custom_leafname("at_nd"))
					.inheriting(f, MAT_FORWARD_INHERIT_CONFIG)
			}) as FuncInheritFactory,
		),
	])
});
