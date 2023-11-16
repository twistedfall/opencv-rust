use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::class::ClassDesc;
use crate::func::{FuncCppBody, FuncDesc, FuncKind, FuncRustBody, ReturnKind};
use crate::type_ref::{Constness, TypeRef, TypeRefDesc};
use crate::Func;

pub type FuncFactory = fn() -> Func<'static, 'static>;

/// (module name, FuncFactory)
pub static FUNC_INJECT: Lazy<HashMap<&str, Vec<FuncFactory>>> = Lazy::new(|| {
	HashMap::from([
		(
			"core",
			vec![
				(|| {
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
				}) as FuncFactory, // todo: remove this cast when MSRV allows
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
						FuncKind::InstanceMethod(ClassDesc::cv_umat()),
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
