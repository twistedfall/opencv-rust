use std::borrow::Cow;

use super::{rust_arg_func_decl, rust_self_func_decl, RenderLaneTrait};
use crate::type_ref::{Constness, ExternDir, FishStyle, TypeRef};
use crate::writer::rust_native::renderer::RustRenderer;
use crate::writer::rust_native::type_ref::{Lifetime, NullabilityExt, TypeRefExt};
use crate::{CppNameStyle, NameStyle};

pub struct FixedArrayRenderLane<'tu, 'ge> {
	canonical: TypeRef<'tu, 'ge>,
	element: TypeRef<'tu, 'ge>,
	len: usize,
}

impl<'tu, 'ge> FixedArrayRenderLane<'tu, 'ge> {
	pub fn from_canonical_element_len(canonical: TypeRef<'tu, 'ge>, element: TypeRef<'tu, 'ge>, len: usize) -> Self {
		Self { canonical, element, len }
	}
}

impl RenderLaneTrait for FixedArrayRenderLane<'_, '_> {
	fn rust_self_func_decl(&self, lifetime: Lifetime) -> Cow<'static, str> {
		rust_self_func_decl(self.canonical.constness(), lifetime)
	}

	fn rust_arg_func_decl(&self, name: &str, lifetime: Lifetime) -> String {
		let typ = if self.element.kind().as_string(self.element.type_hint()).is_some() {
			RustRenderer::format_as_array(self.canonical.constness(), "&str", Some(self.len)).into()
		} else {
			self.canonical.rust_name_ext(NameStyle::Reference(FishStyle::No), lifetime)
		};
		rust_arg_func_decl(name, Constness::Const, &typ)
	}

	fn rust_arg_pre_call(&self, name: &str, _is_function_infallible: bool) -> String {
		if self.element.kind().as_string(self.element.type_hint()).is_some() {
			let const_qual = self.canonical.constness().rust_function_name_qual();
			format!("string_array_arg{const_qual}!({name})")
		} else {
			"".to_string()
		}
	}

	fn rust_arg_func_call(&self, name: &str) -> String {
		name.to_string()
	}

	fn rust_extern_arg_func_decl(&self, name: &str) -> String {
		rust_arg_func_decl(name, Constness::Const, &self.canonical.rust_extern(ExternDir::ToCpp))
	}

	fn cpp_arg_func_decl(&self, name: &str) -> Cow<str> {
		self.canonical.cpp_name_ext(CppNameStyle::Reference, name, true)
	}

	fn cpp_arg_func_call(&self, name: &str) -> String {
		format!("*{name}")
	}
}

pub struct VariableArrayRenderLane<'tu, 'ge> {
	pub canonical: TypeRef<'tu, 'ge>,
	pub element: TypeRef<'tu, 'ge>,
}

impl<'tu, 'ge> VariableArrayRenderLane<'tu, 'ge> {
	pub fn from_canonical_element(canonical: TypeRef<'tu, 'ge>, element: TypeRef<'tu, 'ge>) -> Self {
		Self { canonical, element }
	}
}

impl RenderLaneTrait for VariableArrayRenderLane<'_, '_> {
	fn rust_self_func_decl(&self, lifetime: Lifetime) -> Cow<'static, str> {
		rust_self_func_decl(self.canonical.constness(), lifetime)
	}

	fn rust_arg_func_decl(&self, name: &str, lifetime: Lifetime) -> String {
		let typ = if self.element.kind().as_string(self.element.type_hint()).is_some() {
			RustRenderer::format_as_array(self.canonical.constness(), "&str", None).into()
		} else {
			self.canonical.rust_name_ext(NameStyle::Reference(FishStyle::No), lifetime)
		};
		rust_arg_func_decl(name, Constness::Const, &typ)
	}

	fn rust_arg_pre_call(&self, name: &str, _is_function_infallible: bool) -> String {
		if self.element.kind().as_string(self.element.type_hint()).is_some() {
			let const_qual = self.canonical.constness().rust_function_name_qual();
			format!("string_array_arg{const_qual}!({name})")
		} else {
			"".to_string()
		}
	}

	fn rust_arg_func_call(&self, name: &str) -> String {
		rust_arg_func_call(&self.canonical, name)
	}

	fn rust_extern_arg_func_decl(&self, name: &str) -> String {
		rust_arg_func_decl(name, Constness::Const, &self.canonical.rust_extern(ExternDir::ToCpp))
	}

	fn cpp_arg_func_decl(&self, name: &str) -> Cow<str> {
		self.canonical.cpp_name_ext(CppNameStyle::Reference, name, true)
	}

	fn cpp_arg_func_call(&self, name: &str) -> String {
		name.to_string()
	}
}

fn rust_arg_func_call(type_ref: &TypeRef, name: &str) -> String {
	let constness = type_ref.constness();
	let arr = if constness.is_const() {
		format!("{name}.as_ptr()")
	} else {
		format!("{name}.as_mut_ptr()")
	};
	type_ref
		.type_hint()
		.nullability()
		.rust_wrap_nullable_func_call(name, arr.into(), constness)
		.into_owned()
}
