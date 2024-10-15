use std::borrow::Cow;
use std::collections::HashMap;

use crate::class::ClassDesc;
use crate::func::{FuncCppBody, FuncDesc, FuncKind, FuncMatcher, FuncRustBody, FuncRustExtern, InheritConfig, ReturnKind};
use crate::type_ref::{Constness, TypeRef};
use crate::Func;

pub type FuncReplace = FuncMatcher<'static, FuncInheritFactory>;

pub type FuncInheritFactory = for<'tu, 'ge> fn(&Func<'tu, 'ge>) -> Func<'tu, 'ge>;

pub fn func_replace_factory(module: &str) -> FuncReplace {
	match module {
		"core" => core_factory(),
		_ => FuncReplace::empty(),
	}
}

fn core_factory() -> FuncReplace {
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

	FuncMatcher::create(HashMap::from([(
		"cv::Mat::at",
		vec![
			(
				pred!(mut, ["i0"]),
				(|f| {
					Func::new_desc(make_at_forward(Constness::Mut).rust_custom_leafname("at_mut"))
						.inheriting(f, MAT_FORWARD_INHERIT_CONFIG)
				}) as FuncInheritFactory,
			),
			(
				pred!(const, ["i0"]),
				(|f| Func::new_desc(make_at_forward(Constness::Const)).inheriting(f, MAT_FORWARD_INHERIT_CONFIG))
					as FuncInheritFactory,
			),
			(
				pred!(mut, ["row", "col"]),
				(|f| {
					Func::new_desc(make_at_forward(Constness::Mut).rust_custom_leafname("at_2d_mut"))
						.inheriting(f, MAT_FORWARD_INHERIT_CONFIG)
				}) as FuncInheritFactory,
			),
			(
				pred!(const, ["row", "col"]),
				(|f| {
					Func::new_desc(make_at_forward(Constness::Const).rust_custom_leafname("at_2d"))
						.inheriting(f, MAT_FORWARD_INHERIT_CONFIG)
				}) as FuncInheritFactory,
			),
			(
				pred!(mut, ["i0", "i1", "i2"]),
				(|f| {
					Func::new_desc(make_at_forward(Constness::Mut).rust_custom_leafname("at_3d_mut"))
						.inheriting(f, MAT_FORWARD_INHERIT_CONFIG)
				}) as FuncInheritFactory,
			),
			(
				pred!(const, ["i0", "i1", "i2"]),
				(|f| {
					Func::new_desc(make_at_forward(Constness::Const).rust_custom_leafname("at_3d"))
						.inheriting(f, MAT_FORWARD_INHERIT_CONFIG)
				}) as FuncInheritFactory,
			),
			(
				pred!(mut, ["pt"]),
				(|f| {
					Func::new_desc(make_at_forward(Constness::Mut).rust_custom_leafname("at_pt_mut"))
						.inheriting(f, MAT_FORWARD_INHERIT_CONFIG)
				}) as FuncInheritFactory,
			),
			(
				pred!(const, ["pt"]),
				(|f| {
					Func::new_desc(make_at_forward(Constness::Const).rust_custom_leafname("at_pt"))
						.inheriting(f, MAT_FORWARD_INHERIT_CONFIG)
				}) as FuncInheritFactory,
			),
			(
				pred!(mut, ["idx"]),
				(|f| {
					Func::new_desc(make_at_forward(Constness::Mut).rust_custom_leafname("at_nd_mut"))
						.inheriting(f, MAT_FORWARD_INHERIT_CONFIG)
				}) as FuncInheritFactory,
			),
			(
				pred!(const, ["idx"]),
				(|f| {
					Func::new_desc(make_at_forward(Constness::Const).rust_custom_leafname("at_nd"))
						.inheriting(f, MAT_FORWARD_INHERIT_CONFIG)
				}) as FuncInheritFactory,
			),
		],
	)]))
}
