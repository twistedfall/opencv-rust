use std::borrow::Cow;

use super::{rust_arg_func_decl, rust_self_func_decl, RenderLaneTrait};
use crate::type_ref::{Constness, ExternDir, FishStyle, TypeRef};
use crate::writer::rust_native::type_ref::{Lifetime, TypeRefExt};
use crate::{CppNameStyle, NameStyle};

pub struct ByMoveRenderLane<'tu, 'ge> {
	non_canonical: TypeRef<'tu, 'ge>,
}

impl<'tu, 'ge> ByMoveRenderLane<'tu, 'ge> {
	pub fn from_non_canonical(non_canonical: TypeRef<'tu, 'ge>) -> Self {
		Self { non_canonical }
	}
}

impl RenderLaneTrait for ByMoveRenderLane<'_, '_> {
	fn rust_self_func_decl(&self, lifetime: Lifetime) -> Cow<'static, str> {
		rust_self_func_decl(self.non_canonical.constness(), lifetime)
	}

	fn rust_arg_func_decl(&self, name: &str, lifetime: Lifetime) -> String {
		rust_arg_func_decl(
			name,
			Constness::Mut,
			&self
				.non_canonical
				.rust_name_ext(NameStyle::Reference(FishStyle::No), lifetime),
		)
	}

	fn rust_arg_func_call(&self, name: &str) -> String {
		rust_arg_func_call(&self.non_canonical, name)
	}

	fn rust_extern_arg_func_decl(&self, name: &str) -> String {
		rust_arg_func_decl(name, Constness::Const, &self.non_canonical.rust_extern(ExternDir::ToCpp))
	}

	fn cpp_arg_func_decl(&self, name: &str) -> Cow<str> {
		self.non_canonical.cpp_name_ext(CppNameStyle::Reference, name, true)
	}

	fn cpp_arg_func_call(&self, name: &str) -> String {
		let name = if self.non_canonical.kind().as_pointer().is_some() {
			name.to_string()
		} else {
			format!("*{name}")
		};
		format!("std::move({name})")
	}
}

fn rust_arg_func_call(type_ref: &TypeRef, name: &str) -> String {
	format!("&{cnst}{name}", cnst = type_ref.constness().rust_qual())
}
