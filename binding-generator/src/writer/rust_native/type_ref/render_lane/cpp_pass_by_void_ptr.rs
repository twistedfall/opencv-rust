use std::borrow::Cow;

use crate::{CppNameStyle, NameStyle};
use crate::type_ref::{Constness, ExternDir, FishStyle, TypeRef};
use crate::writer::rust_native::type_ref::{Lifetime, NullabilityExt, TypeRefExt};

use super::{RenderLaneTrait, rust_arg_func_decl, rust_self_func_decl};

pub struct CppPassByVoidPtrRenderLane<'tu, 'ge> {
	non_canonical: TypeRef<'tu, 'ge>,
}

impl<'tu, 'ge> CppPassByVoidPtrRenderLane<'tu, 'ge> {
	pub fn from_non_canonical(non_canonical: TypeRef<'tu, 'ge>) -> Self {
		Self { non_canonical }
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

	fn rust_arg_func_call(&self, name: &str) -> String {
		rust_arg_func_call(&self.non_canonical, name)
	}

	fn rust_extern_arg_func_decl(&self, name: &str) -> String {
		rust_arg_func_decl(name, Constness::Const, &self.non_canonical.rust_extern(ExternDir::ToCpp))
	}

	fn cpp_arg_func_decl(&self, name: &str) -> String {
		if self.non_canonical.kind().as_pointer_reference_move().is_some() {
			self
				.non_canonical
				.cpp_name_ext(CppNameStyle::Reference, name, true)
				.into_owned()
		} else {
			TypeRef::new_pointer(self.non_canonical.with_inherent_constness(self.non_canonical.constness()))
				.cpp_name_ext(CppNameStyle::Reference, name, true)
				.into_owned()
		}
	}

	fn cpp_arg_func_call(&self, name: &str) -> String {
		let kind = self.non_canonical.kind();
		let name = if kind.as_pointer().is_some() {
			Cow::Borrowed(name)
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
	let src_typ = type_ref.source();
	let constness = type_ref.constness();
	let by_ptr = format!("{name}.{as_raw}()", as_raw = src_typ.rust_as_raw_name(constness));
	type_ref
		.type_hint()
		.nullability()
		.rust_wrap_nullable_func_call(name, by_ptr.into(), constness)
		.into_owned()
}
