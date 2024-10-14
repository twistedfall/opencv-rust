use crate::class::ClassDesc;
use crate::field::{Field, FieldDesc};
use crate::func::FuncKind::{Constructor, InstanceMethod};
use crate::func::ReturnKind::{Fallible, InfallibleNaked};
use crate::func::{FuncCppBody, FuncDesc};
use crate::type_ref::Constness::{Const, Mut};
use crate::type_ref::{TypeRef, TypeRefDesc, TypeRefTypeHint};
use crate::writer::rust_native::type_ref::Lifetime;
use crate::Func;

pub type FuncInject = Vec<FuncFactory>;

pub type FuncFactory = fn() -> Func<'static, 'static>;

pub fn func_inject_factory(module: &str) -> FuncInject {
	match module {
		"core" => vec![
			(|| {
				Func::new_desc(
					FuncDesc::new(
						InstanceMethod(ClassDesc::cv_matconstiterator()),
						Const,
						InfallibleNaked,
						"type",
						"core",
						[],
						TypeRefDesc::int(),
					)
					.cpp_body(FuncCppBody::ManualCall("instance->m->type()".into())),
				)
			}) as FuncFactory, // todo: remove this cast when MSRV allows
			|| {
				Func::new_desc(FuncDesc::new(
					InstanceMethod(ClassDesc::cv_mat()),
					Const,
					Fallible,
					"size",
					"core",
					[],
					TypeRefDesc::cv_size(),
				))
			},
			|| {
				Func::new_desc(
					FuncDesc::new(
						InstanceMethod(ClassDesc::cv_mat()),
						Const,
						Fallible,
						"getDataDump",
						"core",
						[],
						TypeRefDesc::std_string(),
					)
					.cpp_body(FuncCppBody::ManualCall(
						"std::string();\nstd::ostringstream oss;\noss << *instance;\nret = oss.str()".into(),
					))
					.doc_comment("Return the dump of the Mat's data"),
				)
			},
			|| {
				Func::new_desc(FuncDesc::new(
					InstanceMethod(ClassDesc::cv_umat()),
					Const,
					Fallible,
					"size",
					"core",
					[],
					TypeRefDesc::cv_size(),
				))
			},
			|| {
				Func::new_desc(
					FuncDesc::new(
						Constructor(ClassDesc::cv_input_array()),
						Mut,
						Fallible,
						"_InputArray",
						"core",
						[
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
						TypeRefDesc::cv_input_array()
							.with_inherent_constness(Const)
							.with_type_hint(TypeRefTypeHint::BoxedAsRef(Const, "vec", Lifetime::Elided)),
					)
					.rust_custom_leafname("from_byte_slice"),
				)
			},
		],
		_ => vec![],
	}
}
