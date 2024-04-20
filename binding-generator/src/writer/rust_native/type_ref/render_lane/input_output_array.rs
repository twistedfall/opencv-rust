use std::borrow::Cow;

use crate::type_ref::{Constness, ExternDir, TypeRef};
use crate::writer::rust_native::type_ref::{Lifetime, TypeRefExt};
use crate::CppNameStyle;

use super::{rust_arg_func_decl, rust_self_func_decl, void_ptr_rust_arg_func_call, RenderLaneTrait};

pub struct InputArrayRenderLane<'tu, 'ge> {
	canonical: TypeRef<'tu, 'ge>,
}

impl<'tu, 'ge> InputArrayRenderLane<'tu, 'ge> {
	pub fn from_canonical(canonical: TypeRef<'tu, 'ge>) -> Self {
		Self { canonical }
	}
}

impl RenderLaneTrait for InputArrayRenderLane<'_, '_> {
	fn rust_self_func_decl(&self, lifetime: Lifetime) -> Cow<'static, str> {
		rust_self_func_decl(self.canonical.constness(), lifetime)
	}

	fn rust_arg_func_decl(&self, name: &str, _lifetime: Lifetime) -> String {
		rust_arg_func_decl(name, Constness::Const, "&impl ToInputArray")
	}

	fn rust_arg_pre_call(&self, name: &str, _is_function_infallible: bool) -> String {
		format!("input_array_arg!({name})")
	}

	fn rust_arg_func_call(&self, name: &str) -> String {
		void_ptr_rust_arg_func_call(&self.canonical, name)
	}

	fn rust_extern_arg_func_decl(&self, name: &str) -> String {
		rust_arg_func_decl(name, Constness::Const, &self.canonical.rust_extern(ExternDir::ToCpp))
	}

	fn cpp_arg_func_decl(&self, name: &str) -> String {
		if self.canonical.kind().as_reference().is_some() {
			self.canonical.cpp_name_ext(CppNameStyle::Reference, name, true).into_owned()
		} else {
			TypeRef::new_pointer(self.canonical.with_inherent_constness(self.canonical.constness()))
				.cpp_name_ext(CppNameStyle::Reference, name, true)
				.into_owned()
		}
	}

	fn cpp_arg_func_call(&self, name: &str) -> String {
		format!("*{name}")
	}
}

pub struct OutputArrayRenderLane<'tu, 'ge> {
	canonical: TypeRef<'tu, 'ge>,
}

impl<'tu, 'ge> OutputArrayRenderLane<'tu, 'ge> {
	pub fn from_canonical(canonical: TypeRef<'tu, 'ge>) -> Self {
		Self { canonical }
	}
}

impl RenderLaneTrait for OutputArrayRenderLane<'_, '_> {
	fn rust_self_func_decl(&self, lifetime: Lifetime) -> Cow<'static, str> {
		rust_self_func_decl(self.canonical.constness(), lifetime)
	}

	fn rust_arg_func_decl(&self, name: &str, _lifetime: Lifetime) -> String {
		rust_arg_func_decl(name, Constness::Const, "&mut impl ToOutputArray")
	}

	fn rust_arg_pre_call(&self, name: &str, _is_function_infallible: bool) -> String {
		format!("output_array_arg!({name})")
	}

	fn rust_arg_func_call(&self, name: &str) -> String {
		// todo make OutputArray be passed by mut
		void_ptr_rust_arg_func_call(&self.canonical, name)
	}

	fn rust_extern_arg_func_decl(&self, name: &str) -> String {
		rust_arg_func_decl(name, Constness::Const, &self.canonical.rust_extern(ExternDir::ToCpp))
	}

	fn cpp_arg_func_decl(&self, name: &str) -> String {
		if self.canonical.kind().as_reference().is_some() {
			self.canonical.cpp_name_ext(CppNameStyle::Reference, name, true).into_owned()
		} else {
			TypeRef::new_pointer(self.canonical.with_inherent_constness(self.canonical.constness()))
				.cpp_name_ext(CppNameStyle::Reference, name, true)
				.into_owned()
		}
	}

	fn cpp_arg_func_call(&self, name: &str) -> String {
		format!("*{name}")
	}
}

pub struct InputOutputArrayRenderLane<'tu, 'ge> {
	canonical: TypeRef<'tu, 'ge>,
}

impl<'tu, 'ge> InputOutputArrayRenderLane<'tu, 'ge> {
	pub fn from_canonical(canonical: TypeRef<'tu, 'ge>) -> Self {
		Self { canonical }
	}
}

impl RenderLaneTrait for InputOutputArrayRenderLane<'_, '_> {
	fn rust_self_func_decl(&self, lifetime: Lifetime) -> Cow<'static, str> {
		rust_self_func_decl(self.canonical.constness(), lifetime)
	}

	fn rust_arg_func_decl(&self, name: &str, _lifetime: Lifetime) -> String {
		rust_arg_func_decl(name, Constness::Const, "&mut impl ToInputOutputArray")
	}

	fn rust_arg_pre_call(&self, name: &str, _is_function_infallible: bool) -> String {
		format!("input_output_array_arg!({name})")
	}

	fn rust_arg_func_call(&self, name: &str) -> String {
		// todo make InputOutputArray be passed by mut
		void_ptr_rust_arg_func_call(&self.canonical, name)
	}

	fn rust_extern_arg_func_decl(&self, name: &str) -> String {
		rust_arg_func_decl(name, Constness::Const, &self.canonical.rust_extern(ExternDir::ToCpp))
	}

	fn cpp_arg_func_decl(&self, name: &str) -> String {
		if self.canonical.kind().as_reference().is_some() {
			self.canonical.cpp_name_ext(CppNameStyle::Reference, name, true).into_owned()
		} else {
			TypeRef::new_pointer(self.canonical.with_inherent_constness(self.canonical.constness()))
				.cpp_name_ext(CppNameStyle::Reference, name, true)
				.into_owned()
		}
	}

	fn cpp_arg_func_call(&self, name: &str) -> String {
		format!("*{name}")
	}
}
