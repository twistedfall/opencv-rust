use std::borrow::Cow;

use crate::{CppNameStyle, NameStyle};
use crate::type_ref::{Constness, ExternDir, FishStyle, TypeRef, TypeRefDesc};
use crate::writer::rust_native::type_ref::{Lifetime, NullabilityExt, TypeRefExt};

use super::{RenderLaneTrait, rust_arg_func_decl, rust_self_func_decl};

/// For when the type is a slice (judging by the type_hint) and its element is C++ `void`
///
/// We want to present such cases as `&[u8]` on the Rust side.
pub struct VoidSliceRenderLane<'tu, 'ge> {
	canonical: TypeRef<'tu, 'ge>,
}

impl<'tu, 'ge> VoidSliceRenderLane<'tu, 'ge> {
	pub fn from_canonical(canonical: TypeRef<'tu, 'ge>) -> Self {
		Self { canonical }
	}
}

impl RenderLaneTrait for VoidSliceRenderLane<'_, '_> {
	fn rust_self_func_decl(&self, lifetime: Lifetime) -> Cow<'static, str> {
		rust_self_func_decl(self.canonical.constness(), lifetime)
	}

	fn rust_arg_func_decl(&self, name: &str, lifetime: Lifetime) -> String {
		let typ = TypeRefDesc::array_uchar(None).with_inherent_constness(self.canonical.inherent_constness());
		rust_arg_func_decl(
			name,
			Constness::Const,
			&typ.rust_name_ext(NameStyle::Reference(FishStyle::No), lifetime),
		)
	}

	fn rust_arg_func_call(&self, name: &str) -> String {
		let constness = self.canonical.constness();
		let mut arr = if constness.is_const() {
			format!("{name}.as_ptr()")
		} else {
			format!("{name}.as_mut_ptr()")
		};
		arr.push_str(".cast()");
		self
			.canonical
			.type_hint()
			.nullability()
			.rust_wrap_nullable_func_call(name, arr.into(), constness)
			.into_owned()
	}

	fn rust_extern_arg_func_decl(&self, name: &str) -> String {
		rust_arg_func_decl(name, Constness::Const, &self.canonical.rust_extern(ExternDir::ToCpp))
	}

	fn cpp_arg_func_decl(&self, name: &str) -> String {
		self.canonical.cpp_name_ext(CppNameStyle::Reference, name, true).into_owned()
	}

	fn cpp_arg_func_call(&self, name: &str) -> String {
		name.to_string()
	}
}
