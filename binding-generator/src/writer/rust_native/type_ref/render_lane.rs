use std::borrow::Cow;
use std::fmt;

pub use array::{FixedArrayRenderLane, VariableArrayRenderLane};
pub use by_move::ByMoveRenderLane;
pub use cpp_pass_by_void_ptr::CppPassByVoidPtrRenderLane;
pub use enumeration::EnumRenderLane;
pub use function::FunctionRenderLane;
pub use indirect::IndirectRenderLane;
pub use input_output_array::{InputArrayRenderLane, InputOutputArrayRenderLane, OutputArrayRenderLane};
pub use primitive::PrimitiveRenderLane;
pub use simple_class::SimpleClassRenderLane;
pub use string::{InStringRenderLane, OutStringRenderLane};
pub use trait_class::TraitClassRenderLane;
pub use void_slice::VoidSliceRenderLane;

use crate::func::Safety;
use crate::type_ref::{Constness, TypeRef};
use crate::writer::rust_native::type_ref::{Lifetime, TypeRefExt};

mod array;
mod by_move;
mod cpp_pass_by_void_ptr;
mod enumeration;
mod function;
mod indirect;
mod input_output_array;
mod primitive;
mod simple_class;
mod string;
mod trait_class;
mod void_slice;

pub trait RenderLaneTrait {
	fn rust_self_func_decl(&self, lifetime: Lifetime) -> Cow<'static, str>;
	fn rust_arg_func_decl(&self, name: &str, lifetime: Lifetime) -> String;
	fn rust_arg_pre_call(&self, _name: &str, _function_props: &FunctionProps) -> String {
		"".to_string()
	}
	fn rust_arg_func_call(&self, name: &str) -> String;

	fn rust_arg_post_success_call(&self, _name: &str) -> String {
		"".to_string()
	}

	fn rust_extern_arg_func_decl(&self, name: &str) -> String;

	fn cpp_arg_func_decl(&self, name: &str) -> Cow<str>;

	fn cpp_arg_pre_call(&self, _name: &str) -> String {
		"".to_string()
	}

	fn cpp_arg_func_call(&self, name: &str) -> String;

	fn cpp_arg_post_call(&self, _name: &str) -> String {
		"".to_string()
	}
}

pub enum RenderLane<'tu, 'ge> {
	Primitive(PrimitiveRenderLane<'tu, 'ge>),
	InString(InStringRenderLane<'tu, 'ge>),
	OutString(OutStringRenderLane<'tu, 'ge>),
	Function(FunctionRenderLane<'tu, 'ge>),
	Indirect(IndirectRenderLane<'tu, 'ge>),
	CppPassByVoidPtr(CppPassByVoidPtrRenderLane<'tu, 'ge>),
	VoidSlice(VoidSliceRenderLane<'tu, 'ge>),
	ByMove(ByMoveRenderLane<'tu, 'ge>),
	InputArray(InputArrayRenderLane<'tu, 'ge>),
	OutputArray(OutputArrayRenderLane<'tu, 'ge>),
	InputOutputArray(InputOutputArrayRenderLane<'tu, 'ge>),
	VariableArray(VariableArrayRenderLane<'tu, 'ge>),
	FixedArray(FixedArrayRenderLane<'tu, 'ge>),
	TraitClass(TraitClassRenderLane<'tu, 'ge>),
	Enum(EnumRenderLane<'tu, 'ge>),
	SimpleClass(SimpleClassRenderLane<'tu, 'ge>),
}

impl<'tu, 'ge> RenderLane<'tu, 'ge> {
	#[inline(always)]
	pub fn to_dyn(&self) -> &dyn RenderLaneTrait {
		match self {
			RenderLane::Primitive(rlane) => rlane,
			RenderLane::InString(rlane) => rlane,
			RenderLane::OutString(rlane) => rlane,
			RenderLane::Function(rlane) => rlane,
			RenderLane::Indirect(rlane) => rlane,
			RenderLane::CppPassByVoidPtr(rlane) => rlane,
			RenderLane::VoidSlice(rlane) => rlane,
			RenderLane::ByMove(rlane) => rlane,
			RenderLane::InputArray(rlane) => rlane,
			RenderLane::OutputArray(rlane) => rlane,
			RenderLane::InputOutputArray(rlane) => rlane,
			RenderLane::VariableArray(rlane) => rlane,
			RenderLane::FixedArray(rlane) => rlane,
			RenderLane::TraitClass(rlane) => rlane,
			RenderLane::Enum(rlane) => rlane,
			RenderLane::SimpleClass(rlane) => rlane,
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub enum Indirection {
	None,
	Pointer,
	Reference,
}

impl fmt::Debug for RenderLane<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			RenderLane::Primitive(_) => f.write_str("Primitive"),
			RenderLane::InString(_) => f.write_str("InString"),
			RenderLane::OutString(_) => f.write_str("OutString"),
			RenderLane::Function(_) => f.write_str("Function"),
			RenderLane::Indirect(_) => f.write_str("Indirect"),
			RenderLane::CppPassByVoidPtr(_) => f.write_str("CppPassByVoidPtr"),
			RenderLane::VoidSlice(_) => f.write_str("VoidSlice"),
			RenderLane::ByMove(_) => f.write_str("ByMove"),
			RenderLane::InputArray(_) => f.write_str("InputArray"),
			RenderLane::OutputArray(_) => f.write_str("OutputArray"),
			RenderLane::InputOutputArray(_) => f.write_str("InputOutputArray"),
			RenderLane::VariableArray(_) => f.write_str("VariableArray"),
			RenderLane::FixedArray(_) => f.write_str("FixedArray"),
			RenderLane::TraitClass(_) => f.write_str("TraitClass"),
			RenderLane::Enum(_) => f.write_str("Enum"),
			RenderLane::SimpleClass(_) => f.write_str("SimpleClass"),
		}
	}
}

fn rust_self_func_decl(method_constness: Constness, lifetime: Lifetime) -> Cow<'static, str> {
	if lifetime.is_elided() {
		if method_constness.is_const() {
			"&self"
		} else {
			"&mut self"
		}
		.into()
	} else {
		if method_constness.is_const() {
			format!("&{lifetime: <}self")
		} else {
			format!("&mut {lifetime: <}self")
		}
		.into()
	}
}

pub struct FunctionProps {
	pub is_infallible: bool,
	pub safety: Safety,
}

fn rust_arg_func_decl(name: &str, argument_constness: Constness, typ: &str) -> String {
	format!("{cnst}{name}: {typ}", cnst = argument_constness.rust_qual())
}

fn void_ptr_rust_arg_func_call(type_ref: &TypeRef, name: &str) -> String {
	format!("{name}.{as_raw}()", as_raw = type_ref.rust_as_raw_name(type_ref.constness()))
}
