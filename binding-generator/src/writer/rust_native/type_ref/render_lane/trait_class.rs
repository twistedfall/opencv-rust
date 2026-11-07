use std::borrow::Cow;
use std::borrow::Cow::{Borrowed, Owned};

use super::{Ownership, RenderLaneTrait, rust_arg_func_decl, rust_self_func_decl, void_ptr_rust_arg_func_call};
use crate::type_ref::{Constness, ExternDir, FishStyle, TypeRef, TypeRefTypeHint};
use crate::writer::rust_native::class::ClassExt;
use crate::writer::rust_native::type_ref::{Lifetime, NullabilityExt, TypeRefExt};
use crate::{Class, CowMapBorrowedExt, CppNameStyle, NameStyle};

pub struct TraitClassRenderLane<'tu, 'ge> {
	non_canonical: TypeRef<'tu, 'ge>,
	class: Class<'tu, 'ge>,
	ownership: Ownership,
}

impl<'tu, 'ge> TraitClassRenderLane<'tu, 'ge> {
	pub fn from_non_canonical_class(non_canonical: TypeRef<'tu, 'ge>, class: Class<'tu, 'ge>, ownership: Ownership) -> Self {
		Self {
			non_canonical,
			class,
			ownership,
		}
	}
}

impl RenderLaneTrait for TraitClassRenderLane<'_, '_> {
	fn rust_self_func_decl(&self, lifetime: Lifetime) -> Cow<'static, str> {
		rust_self_func_decl(self.non_canonical.constness(), lifetime)
	}

	fn rust_arg_func_decl(&self, name: &str, lifetime: Lifetime) -> String {
		let type_hint = self.non_canonical.type_hint();
		// raw pointers are also passed as concrete structs
		let inner = if matches!(type_hint, TypeRefTypeHint::PtrAsRaw | TypeRefTypeHint::TraitClassConcrete) {
			self.non_canonical.rust_name(NameStyle::Reference(FishStyle::No)).into_owned()
		} else {
			format!(
				"impl {trait_name}",
				trait_name = self
					.class
					.rust_trait_name(NameStyle::Reference(FishStyle::No), self.non_canonical.constness())
			)
		};
		let (typ, constness) = match self.ownership {
			Ownership::Owned => (inner, Constness::Mut),
			Ownership::OwnedButConst => (inner, Constness::Const),
			Ownership::BorrowedByPointer | Ownership::BorrowedByReference => {
				if self.non_canonical.kind().is_rust_by_ptr(type_hint) {
					// is_rust_by_ptr will render the pointer type, so we don't need to add a reference here
					(inner, Constness::Const)
				} else {
					(
						format!(
							"&{lifetime: <}{cnst}{inner}",
							cnst = self.non_canonical.constness().rust_qual()
						),
						Constness::Const,
					)
				}
			}
		};
		let typ = type_hint
			.nullability()
			.rust_wrap_nullable_func_decl(typ.into(), NameStyle::Reference(FishStyle::No));
		rust_arg_func_decl(name, constness, &typ)
	}

	fn rust_arg_func_call(&self, name: &str) -> String {
		let type_hint = self.non_canonical.type_hint();
		let typ = if self.non_canonical.kind().is_rust_by_ptr(type_hint) {
			format!("{name}.cast()")
		} else {
			void_ptr_rust_arg_func_call(
				&self
					.non_canonical
					.source()
					.into_owned()
					.with_inherent_constness(self.non_canonical.constness()),
				name,
			)
		};
		type_hint
			.nullability()
			.rust_wrap_nullable_func_call(name, typ.into(), self.non_canonical.constness())
			.into_owned()
	}

	fn rust_extern_arg_func_decl(&self, name: &str) -> String {
		rust_arg_func_decl(name, Constness::Const, &self.non_canonical.rust_extern(ExternDir::ToCpp))
	}

	fn cpp_arg_func_decl(&self, name: &str) -> Cow<'_, str> {
		let typ = match self.ownership {
			Ownership::Owned | Ownership::OwnedButConst => Owned(TypeRef::new_pointer(self.non_canonical.clone())),
			Ownership::BorrowedByPointer | Ownership::BorrowedByReference => Borrowed(&self.non_canonical),
		};
		typ.map_borrowed(|typ| typ.cpp_name_ext(CppNameStyle::Reference, name, true))
	}

	fn cpp_arg_func_call(&self, name: &str) -> String {
		match self.ownership {
			Ownership::Owned => format!("std::move(*{name})"),
			Ownership::BorrowedByPointer => name.to_string(),
			Ownership::OwnedButConst | Ownership::BorrowedByReference => format!("*{name}"),
		}
	}
}
