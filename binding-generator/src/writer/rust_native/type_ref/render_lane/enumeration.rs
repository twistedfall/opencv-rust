use std::borrow::Cow;

use crate::{CppNameStyle, Enum, NameStyle};
use crate::type_ref::{Constness, ExternDir, FishStyle, TypeRef};
use crate::writer::rust_native::element::RustElement;
use crate::writer::rust_native::type_ref::{Lifetime, TypeRefExt};

use super::{Indirection, RenderLaneTrait, rust_arg_func_decl, rust_self_func_decl};

pub struct EnumRenderLane<'tu, 'ge> {
	pub(crate) non_canonical: TypeRef<'tu, 'ge>,
	pub(crate) enm: Enum<'tu>,
	pub(crate) indirection: Indirection,
}

impl<'tu, 'ge> EnumRenderLane<'tu, 'ge> {
	pub fn from_non_canonical_enum_indirection(
		non_canonical: TypeRef<'tu, 'ge>,
		enm: Enum<'tu>,
		indirection: Indirection,
	) -> Self {
		Self {
			non_canonical,
			enm,
			indirection,
		}
	}
}

impl RenderLaneTrait for EnumRenderLane<'_, '_> {
	fn rust_self_func_decl(&self, lifetime: Lifetime) -> Cow<'static, str> {
		rust_self_func_decl(self.non_canonical.constness(), lifetime)
	}

	fn rust_arg_func_decl(&self, name: &str, _lifetime: Lifetime) -> String {
		rust_arg_func_decl(
			name,
			Constness::Const,
			&self.enm.rust_name(NameStyle::Reference(FishStyle::No)),
		)
	}

	fn rust_arg_func_call(&self, name: &str) -> String {
		match self.indirection {
			Indirection::None | Indirection::Reference => name.to_string(),
			Indirection::Pointer => {
				format!("&{name}")
			}
		}
	}

	fn rust_extern_arg_func_decl(&self, name: &str) -> String {
		let typ = match self.indirection {
			Indirection::None | Indirection::Reference => Cow::Borrowed(&self.non_canonical),
			Indirection::Pointer => Cow::Owned(TypeRef::new_pointer(self.non_canonical.clone())),
		};
		rust_arg_func_decl(name, Constness::Const, &typ.rust_extern(ExternDir::ToCpp))
	}

	fn cpp_arg_func_decl(&self, name: &str) -> String {
		let typ = match self.indirection {
			Indirection::None | Indirection::Reference => Cow::Borrowed(&self.non_canonical),
			Indirection::Pointer => Cow::Owned(TypeRef::new_pointer(self.non_canonical.clone())),
		};
		typ.cpp_name_ext(CppNameStyle::Reference, name, true).into_owned()
	}

	fn cpp_arg_func_call(&self, name: &str) -> String {
		match self.indirection {
			Indirection::None | Indirection::Reference => name.to_string(),
			Indirection::Pointer => format!("&{name}"),
		}
	}
}
