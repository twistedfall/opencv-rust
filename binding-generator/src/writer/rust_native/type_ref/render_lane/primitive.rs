use std::borrow::Cow;

use super::{rust_arg_func_decl, rust_self_func_decl, RenderLaneTrait};
use crate::type_ref::{Constness, ExternDir, FishStyle, TypeRef, TypeRefTypeHint};
use crate::writer::rust_native::type_ref::{Lifetime, TypeRefExt};
use crate::{CppNameStyle, NameStyle};

pub struct PrimitiveRenderLane<'tu, 'ge> {
	cpp: &'static str,
	non_canonical: TypeRef<'tu, 'ge>,
}

impl<'tu, 'ge> PrimitiveRenderLane<'tu, 'ge> {
	pub fn from_cpp_non_canonical(cpp: &'static str, non_canonical: TypeRef<'tu, 'ge>) -> Self {
		Self { cpp, non_canonical }
	}
}

impl RenderLaneTrait for PrimitiveRenderLane<'_, '_> {
	fn rust_self_func_decl(&self, lifetime: Lifetime) -> Cow<'static, str> {
		rust_self_func_decl(self.non_canonical.constness(), lifetime)
	}

	fn rust_arg_func_decl(&self, name: &str, lifetime: Lifetime) -> String {
		let typ = if self.cpp == "char" && matches!(self.non_canonical.type_hint(), TypeRefTypeHint::CharAsRustChar) {
			"char".into()
		} else {
			self
				.non_canonical
				.rust_name_ext(NameStyle::Reference(FishStyle::No), lifetime)
		};
		rust_arg_func_decl(name, Constness::Const, &typ)
	}

	fn rust_arg_func_call(&self, name: &str) -> String {
		if self.cpp == "char" && matches!(self.non_canonical.type_hint(), TypeRefTypeHint::CharAsRustChar) {
			format!("u8::try_from({name})? as c_char")
		} else {
			name.to_string()
		}
	}

	fn rust_extern_arg_func_decl(&self, name: &str) -> String {
		rust_arg_func_decl(name, Constness::Const, &self.non_canonical.rust_extern(ExternDir::ToCpp))
	}

	fn cpp_arg_func_decl(&self, name: &str) -> Cow<str> {
		self.non_canonical.cpp_name_ext(CppNameStyle::Reference, name, true)
	}

	fn cpp_arg_func_call(&self, name: &str) -> String {
		name.to_string()
	}
}
