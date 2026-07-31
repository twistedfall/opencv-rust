use std::borrow::Cow;
use std::borrow::Cow::{Borrowed, Owned};

use super::{Indirection, RenderLaneTrait, rust_arg_func_decl, rust_self_func_decl, void_ptr_rust_arg_func_call};
use crate::type_ref::{Constness, ExternDir, FishStyle, TypeRef, TypeRefTypeHint};
use crate::writer::rust_native::class::ClassExt;
use crate::writer::rust_native::type_ref::{Lifetime, NullabilityExt, TypeRefExt};
use crate::{Class, CowMapBorrowedExt, CppNameStyle, NameStyle};

pub struct TraitClassRenderLane<'tu, 'ge> {
	non_canonical: TypeRef<'tu, 'ge>,
	class: Class<'tu, 'ge>,
	indirection: Indirection,
}

impl<'tu, 'ge> TraitClassRenderLane<'tu, 'ge> {
	pub fn from_non_canonical_class_indirection(
		non_canonical: TypeRef<'tu, 'ge>,
		class: Class<'tu, 'ge>,
		indirection: Indirection,
	) -> Self {
		Self {
			non_canonical,
			class,
			indirection,
		}
	}
}

impl RenderLaneTrait for TraitClassRenderLane<'_, '_> {
	fn rust_self_func_decl(&self, lifetime: Lifetime) -> Cow<'static, str> {
		rust_self_func_decl(self.non_canonical.constness(), lifetime)
	}

	fn rust_arg_func_decl(&self, name: &str, lifetime: Lifetime) -> String {
		let type_hint = self.non_canonical.type_hint();
		let inner = if matches!(type_hint, TypeRefTypeHint::TraitClassConcrete) {
			self.non_canonical.rust_name(NameStyle::Reference(FishStyle::No)).into_owned()
		} else {
			format!(
				"impl {trait_name}",
				trait_name = self
					.class
					.rust_trait_name(NameStyle::Reference(FishStyle::No), self.non_canonical.constness())
			)
		};
		let (typ, constness) = match self.indirection {
			Indirection::None => (inner, self.non_canonical.constness()),
			Indirection::Pointer | Indirection::Reference => (
				format!(
					"&{lifetime: <}{cnst}{inner}",
					cnst = self.non_canonical.constness().rust_qual()
				),
				Constness::Const,
			),
		};
		let typ = type_hint
			.nullability()
			.rust_wrap_nullable_func_decl(typ.into(), NameStyle::Reference(FishStyle::No));
		rust_arg_func_decl(name, constness, &typ)
	}

	fn rust_arg_func_call(&self, name: &str) -> String {
		let typ = void_ptr_rust_arg_func_call(
			&self
				.non_canonical
				.source()
				.into_owned()
				.with_inherent_constness(self.non_canonical.constness()),
			name,
		);
		self
			.non_canonical
			.type_hint()
			.nullability()
			.rust_wrap_nullable_func_call(name, typ.into(), self.non_canonical.constness())
			.into_owned()
	}

	fn rust_extern_arg_func_decl(&self, name: &str) -> String {
		rust_arg_func_decl(name, Constness::Const, &self.non_canonical.rust_extern(ExternDir::ToCpp))
	}

	fn cpp_arg_func_decl(&self, name: &str) -> Cow<'_, str> {
		let typ = match self.indirection {
			Indirection::None => Owned(TypeRef::new_pointer(
				self
					.non_canonical
					.clone()
					.with_inherent_constness(self.non_canonical.constness()),
			)),
			Indirection::Pointer | Indirection::Reference => Borrowed(&self.non_canonical),
		};
		typ.map_borrowed(|typ| typ.cpp_name_ext(CppNameStyle::Reference, name, true))
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
