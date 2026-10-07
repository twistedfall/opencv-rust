use std::borrow::Cow;
use std::borrow::Cow::{Borrowed, Owned};

use super::{FunctionProps, Ownership, RenderLaneTrait, rust_arg_func_decl, rust_self_func_decl, void_ptr_rust_arg_func_call};
use crate::type_ref::{Constness, ExternDir, FishStyle, TypeRef};
use crate::writer::rust_native::type_ref::{Lifetime, NullabilityExt, TypeRefExt};
use crate::{CowMapBorrowedExt, CppNameStyle, NameStyle};

pub struct CppPassByVoidPtrRenderLane<'tu, 'ge> {
	non_canonical: TypeRef<'tu, 'ge>,
	ownership: Ownership,
}

impl<'tu, 'ge> CppPassByVoidPtrRenderLane<'tu, 'ge> {
	pub fn from_non_canonical(non_canonical: TypeRef<'tu, 'ge>, ownership: Ownership) -> Self {
		Self {
			non_canonical,
			ownership,
		}
	}
}

impl RenderLaneTrait for CppPassByVoidPtrRenderLane<'_, '_> {
	fn rust_self_func_decl(&self, lifetime: Lifetime) -> Cow<'static, str> {
		rust_self_func_decl(self.non_canonical.constness(), lifetime)
	}

	fn rust_arg_func_decl(&self, name: &str, lifetime: Lifetime) -> String {
		let constness =
			Constness::from_is_mut(self.non_canonical.constness().is_mut() && !self.non_canonical.kind().is_pointer_reference());
		rust_arg_func_decl(
			name,
			constness,
			&self
				.non_canonical
				.rust_name_ext(NameStyle::Reference(FishStyle::No), lifetime),
		)
	}

	fn rust_arg_pre_call(&self, name: &str, _function_props: &FunctionProps) -> String {
		let is_nullable = self.non_canonical.type_hint().nullability().is_nullable();
		if is_nullable && self.non_canonical.source().kind().as_smart_ptr().is_some() {
			let ref_spec = match self.ownership {
				Ownership::BorrowedByPointer | Ownership::BorrowedByReference => "ref ",
				Ownership::Owned | Ownership::OwnedButConst => "",
			};
			format!("smart_ptr_option_arg!({ref_spec}{name})")
		} else {
			"".to_string()
		}
	}

	fn rust_arg_func_call(&self, name: &str) -> String {
		let constness = self.non_canonical.constness();
		let source = self.non_canonical.source();
		let call = void_ptr_rust_arg_func_call(&source, name);
		let nullability = self.non_canonical.type_hint().nullability();
		if nullability.is_nullable() && source.kind().as_smart_ptr().is_some() {
			call
		} else {
			nullability
				.rust_wrap_nullable_func_call(name, call.into(), constness)
				.into_owned()
		}
	}

	fn rust_extern_arg_func_decl(&self, name: &str) -> String {
		rust_arg_func_decl(name, Constness::Const, &self.non_canonical.rust_extern(ExternDir::ToCpp))
	}

	fn cpp_arg_func_decl(&self, name: &str) -> Cow<'_, str> {
		let typ = match self.ownership {
			// fixme: an additional pointer appears for && types, investigate why
			Ownership::Owned | Ownership::OwnedButConst if self.non_canonical.kind().as_by_move().is_some() => {
				Borrowed(&self.non_canonical)
			}
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
