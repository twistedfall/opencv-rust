use std::borrow::Cow;
use std::borrow::Cow::{Borrowed, Owned};

use super::{rust_arg_func_decl, rust_self_func_decl, FunctionProps, Indirection, RenderLaneTrait};
use crate::type_ref::{Constness, ExternDir, FishStyle, TypeRef};
use crate::writer::rust_native::type_ref::{Lifetime, TypeRefExt};
use crate::{CowMapBorrowedExt, CppNameStyle, NameStyle};

pub struct CppPassByVoidPtrRenderLane<'tu, 'ge> {
	non_canonical: TypeRef<'tu, 'ge>,
	indirection: Indirection,
}

impl<'tu, 'ge> CppPassByVoidPtrRenderLane<'tu, 'ge> {
	pub fn from_non_canonical_indirection(non_canonical: TypeRef<'tu, 'ge>, indirection: Indirection) -> Self {
		Self {
			non_canonical,
			indirection,
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
			let ref_spec = match self.indirection {
				Indirection::Pointer | Indirection::Reference => "ref ",
				Indirection::None => "",
			};
			format!("smart_ptr_option_arg!({ref_spec}{name})")
		} else {
			"".to_string()
		}
	}

	fn rust_arg_func_call(&self, name: &str) -> String {
		rust_arg_func_call(&self.non_canonical, name)
	}

	fn rust_extern_arg_func_decl(&self, name: &str) -> String {
		rust_arg_func_decl(name, Constness::Const, &self.non_canonical.rust_extern(ExternDir::ToCpp))
	}

	fn cpp_arg_func_decl(&self, name: &str) -> Cow<str> {
		let typ = if self.non_canonical.kind().as_pointer_reference_move().is_some() {
			Borrowed(&self.non_canonical)
		} else {
			Owned(TypeRef::new_pointer(
				self
					.non_canonical
					.clone()
					.with_inherent_constness(self.non_canonical.constness()),
			))
		};
		typ.map_borrowed(|typ| typ.cpp_name_ext(CppNameStyle::Reference, name, true))
	}

	fn cpp_arg_func_call(&self, name: &str) -> String {
		let kind = self.non_canonical.kind();
		let name = if kind.as_pointer().is_some() {
			Borrowed(name)
		} else {
			format!("*{name}").into()
		};
		if kind.as_by_move().is_some() {
			format!("std::move({name})")
		} else {
			name.into_owned()
		}
	}
}

fn rust_arg_func_call(type_ref: &TypeRef, name: &str) -> String {
	format!(
		"{name}.{as_raw}()",
		as_raw = type_ref.source().rust_as_raw_name(type_ref.constness())
	)
}
