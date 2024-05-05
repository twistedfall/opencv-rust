use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::class::ClassDesc;
use crate::field::{Field, FieldDesc};
use crate::func::{FuncCppBody, FuncDesc, FuncKind, FuncRustBody, ReturnKind};
use crate::type_ref::Constness::{Const, Mut};
use crate::type_ref::{TypeRef, TypeRefDesc, TypeRefTypeHint};
use crate::writer::rust_native::type_ref::Lifetime;
use crate::Func;

pub type FuncFactory = fn() -> Func<'static, 'static>;

/// (module name, FuncFactory)
pub static FUNC_INJECT: Lazy<HashMap<&str, Vec<FuncFactory>>> = Lazy::new(|| {
	HashMap::from([(
		"core",
		vec![
			(|| {
				Func::new_desc(FuncDesc::new(
					FuncKind::InstanceMethod(ClassDesc::cv_matconstiterator()),
					Const,
					ReturnKind::InfallibleNaked,
					"type",
					"core",
					vec![],
					FuncCppBody::ManualCall("instance->m->type()".into()),
					FuncRustBody::Auto,
					TypeRefDesc::int(),
				))
			}) as FuncFactory, // todo: remove this cast when MSRV allows
			|| {
				Func::new_desc(FuncDesc::new(
					FuncKind::InstanceMethod(ClassDesc::cv_mat()),
					Const,
					ReturnKind::Fallible,
					"size",
					"core",
					vec![],
					FuncCppBody::Auto,
					FuncRustBody::Auto,
					TypeRefDesc::cv_size(),
				))
			},
			|| {
				Func::new_desc(FuncDesc::new(
					FuncKind::InstanceMethod(ClassDesc::cv_umat()),
					Const,
					ReturnKind::Fallible,
					"size",
					"core",
					vec![],
					FuncCppBody::Auto,
					FuncRustBody::Auto,
					TypeRefDesc::cv_size(),
				))
			},
			|| {
				Func::new_desc(FuncDesc::new(
					FuncKind::Constructor(ClassDesc::cv_input_array()),
					Mut,
					ReturnKind::Fallible,
					"_InputArray",
					"core",
					vec![
						Field::new_desc(FieldDesc::new(
							"vec",
							TypeRef::new_array(TypeRefDesc::uchar().with_inherent_constness(Const), None),
						)),
						Field::new_desc(FieldDesc::new(
							"n",
							// todo, MSRV: remove `as_slice()` when MSRV allows
							TypeRefDesc::int().with_type_hint(TypeRefTypeHint::LenForSlice(["vec".to_string()].as_slice().into(), 1)),
						)),
					],
					FuncCppBody::Auto,
					FuncRustBody::Auto,
					TypeRefDesc::cv_input_array()
						.with_inherent_constness(Const)
						.with_type_hint(TypeRefTypeHint::BoxedAsRef(Const, "vec", Lifetime::Elided)),
				))
			},
		],
	)])
});
