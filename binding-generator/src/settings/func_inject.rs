use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::class::ClassDesc;
use crate::func::{FuncCppBody, FuncDesc, FuncKind, FuncRustBody, ReturnKind};
use crate::type_ref::{Constness, TypeRef, TypeRefDesc, TypeRefTypeHint};
use crate::Func;

pub type FuncFactory = fn() -> Func<'static, 'static>;

/// (module name, FuncFactory)
pub static FUNC_INJECT_MANUAL: Lazy<HashMap<&str, Vec<FuncFactory>>> = Lazy::new(|| {
	HashMap::from([
		(
			"core",
			vec![
				(|| {
					Func::new_desc(FuncDesc::new(
						FuncKind::InstanceMethod(ClassDesc::cv_matconstiterator()),
						Constness::Const,
						ReturnKind::InfallibleNaked,
						"hasElements",
						"core",
						vec![],
						FuncCppBody::ManualCall("instance->ptr != instance->sliceEnd".into()),
						FuncRustBody::Auto,
						TypeRefDesc::bool(),
					))
				}) as FuncFactory, // todo: remove this cast when MSRV allows
				|| {
					Func::new_desc(FuncDesc::new(
						FuncKind::InstanceMethod(ClassDesc::cv_matconstiterator()),
						Constness::Const,
						ReturnKind::InfallibleNaked,
						"type",
						"core",
						vec![],
						FuncCppBody::ManualCall("instance->m->type()".into()),
						FuncRustBody::Auto,
						TypeRefDesc::int(),
					))
				},
				|| {
					Func::new_desc(FuncDesc::new(
						FuncKind::InstanceMethod(ClassDesc::cv_mat()),
						Constness::Const,
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
						FuncKind::InstanceMethod(ClassDesc::cv_mat()),
						Constness::Const,
						ReturnKind::InfallibleNaked,
						"data",
						"core",
						vec![],
						FuncCppBody::ManualCall("instance->data".into()),
						FuncRustBody::Auto,
						TypeRef::new_pointer(TypeRefDesc::uchar().with_constness(Constness::Const))
							.with_type_hint(TypeRefTypeHint::PrimitiveRefAsPointer),
					))
				},
				|| {
					Func::new_desc(FuncDesc::new(
						FuncKind::InstanceMethod(ClassDesc::cv_umat()),
						Constness::Const,
						ReturnKind::Fallible,
						"size",
						"core",
						vec![],
						FuncCppBody::ManualCall("instance->size()".into()),
						FuncRustBody::Auto,
						TypeRefDesc::cv_size(),
					))
				},
			],
		),
		(
			"dnn",
			vec![|| {
				Func::new_desc(FuncDesc::new(
					FuncKind::Constructor(ClassDesc::cv_dnn_layerparams()),
					Constness::Const,
					ReturnKind::Fallible,
					"new",
					"dnn",
					vec![],
					FuncCppBody::Auto,
					FuncRustBody::Auto,
					TypeRef::new_class(ClassDesc::cv_dnn_layerparams()),
				))
			}],
		),
	])
});
