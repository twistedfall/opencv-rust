use std::borrow::Cow;

use crate::type_ref::{Constness, ExternDir, FishStyle, TypeRef};
use crate::writer::rust_native::type_ref::{Lifetime, NullabilityExt, TypeRefExt};
use crate::{CppNameStyle, NameStyle};

use super::{rust_arg_func_decl, rust_self_func_decl, Indirection, RenderLaneTrait};

pub struct IndirectRenderLane<'tu, 'ge> {
	pub non_canonical: TypeRef<'tu, 'ge>,
	pub pointee: TypeRef<'tu, 'ge>,
	pub indirection: Indirection,
}

impl<'tu, 'ge> IndirectRenderLane<'tu, 'ge> {
	pub fn from_non_canonical_pointee_indirection(
		non_canonical: TypeRef<'tu, 'ge>,
		pointee: TypeRef<'tu, 'ge>,
		indirection: Indirection,
	) -> Self {
		assert!(
			matches!(indirection, Indirection::Pointer | Indirection::Reference),
			"Can't build IndirectRenderLane from Indirection::None"
		);
		Self {
			non_canonical,
			pointee,
			indirection,
		}
	}
}

impl RenderLaneTrait for IndirectRenderLane<'_, '_> {
	fn rust_self_func_decl(&self, lifetime: Lifetime) -> Cow<'static, str> {
		rust_self_func_decl(self.non_canonical.constness(), lifetime)
	}

	fn rust_arg_func_decl(&self, name: &str, lifetime: Lifetime) -> String {
		rust_arg_func_decl(
			name,
			Constness::Const,
			&self
				.non_canonical
				.rust_name_ext(NameStyle::Reference(FishStyle::No), lifetime),
		)
	}

	fn rust_arg_func_call(&self, name: &str) -> String {
		self
			.non_canonical
			.type_hint()
			.nullability()
			.rust_wrap_nullable_func_call(name, name.into(), self.non_canonical.constness())
			.into_owned()
	}

	fn rust_extern_arg_func_decl(&self, name: &str) -> String {
		rust_arg_func_decl(name, Constness::Const, &self.non_canonical.rust_extern(ExternDir::ToCpp))
	}

	fn cpp_arg_func_decl(&self, name: &str) -> Cow<str> {
		self.non_canonical.cpp_name_ext(CppNameStyle::Reference, name, true)
	}

	fn cpp_arg_func_call(&self, name: &str) -> String {
		match self.indirection {
			Indirection::None | Indirection::Pointer => name.to_string(),
			Indirection::Reference => {
				format!("*{name}")
			}
		}
	}
}
