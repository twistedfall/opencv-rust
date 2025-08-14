use std::borrow::Cow;

use super::{rust_arg_func_decl, Indirection, RenderLaneTrait};
use crate::type_ref::{Constness, ExternDir, FishStyle, TypeRef};
use crate::writer::rust_native::type_ref::{Lifetime, NullabilityExt, TypeRefExt};
use crate::{CppNameStyle, NameStyle};

pub struct SimpleClassRenderLane<'tu, 'ge> {
	non_canonical: TypeRef<'tu, 'ge>,
	indirection: Indirection,
	constness: Constness,
}

impl<'tu, 'ge> SimpleClassRenderLane<'tu, 'ge> {
	pub fn from_non_canonical_indirection(non_canonical: TypeRef<'tu, 'ge>, indirection: Indirection) -> Self {
		let constness = non_canonical.constness();
		Self {
			non_canonical,
			indirection,
			constness,
		}
	}
}

impl RenderLaneTrait for SimpleClassRenderLane<'_, '_> {
	fn rust_self_func_decl(&self, _lifetime: Lifetime) -> Cow<'static, str> {
		"self".into()
	}

	fn rust_arg_func_decl(&self, name: &str, lifetime: Lifetime) -> String {
		// const references to simple classes are passed by value for performance
		// fixme: it kind of works now, but probably it's not the best idea
		//  because some functions can potentially save the pointer to the value, but it will be destroyed after function call
		match (self.indirection, self.constness) {
			(Indirection::None, _) | (Indirection::Reference, Constness::Const) => rust_arg_func_decl(
				name,
				Constness::Const,
				&self
					.non_canonical
					.rust_name_ext(NameStyle::Reference(FishStyle::No), lifetime),
			),
			(Indirection::Pointer, _) | (Indirection::Reference, Constness::Mut) => rust_arg_func_decl(
				name,
				Constness::Const,
				&TypeRef::new_pointer(self.non_canonical.clone())
					.with_type_hint(self.non_canonical.type_hint().clone())
					.rust_name_ext(NameStyle::Reference(FishStyle::No), lifetime),
			),
		}
	}

	fn rust_arg_func_call(&self, name: &str) -> String {
		match (self.indirection, self.constness) {
			(Indirection::None, _) | (Indirection::Reference, Constness::Const) => format!("&{name}"),
			(Indirection::Pointer, _) => self
				.non_canonical
				.type_hint()
				.nullability()
				.rust_wrap_nullable_func_call(name, name.into(), self.constness)
				.into_owned(),
			(Indirection::Reference, Constness::Mut) => name.to_string(),
		}
	}

	fn rust_extern_arg_func_decl(&self, name: &str) -> String {
		let constness = match self.indirection {
			Indirection::None => Constness::Const,
			Indirection::Pointer | Indirection::Reference => self.constness,
		};
		let typ = TypeRef::new_pointer(self.non_canonical.clone()).with_inherent_constness(constness);
		rust_arg_func_decl(name, Constness::Const, &typ.rust_extern(ExternDir::ToCpp))
	}

	fn cpp_arg_func_decl(&self, name: &str) -> Cow<'_, str> {
		TypeRef::new_pointer(self.non_canonical.clone())
			.cpp_name_ext(CppNameStyle::Reference, name, true)
			.into_owned()
			.into()
	}

	fn cpp_arg_func_call(&self, name: &str) -> String {
		match self.indirection {
			Indirection::Pointer => name.to_string(),
			Indirection::None | Indirection::Reference => {
				format!("*{name}")
			}
		}
	}
}
