use std::borrow::Cow;
use std::fmt;

use clang::Entity;

use crate::type_ref::{CppNameStyle, NameStyle, TypeRefTypeHint};
use crate::writer::rust_native::element::RustElement;
use crate::writer::rust_native::type_ref::TypeRefExt;
use crate::{settings, DefaultElement, Element, EntityElement, GeneratedType, GeneratorEnv, TypeRef};

#[derive(Clone)]
pub struct Typedef<'tu, 'ge> {
	entity: Entity<'tu>,
	gen_env: &'ge GeneratorEnv<'tu>,
}

impl<'tu, 'ge> Typedef<'tu, 'ge> {
	pub fn new(entity: Entity<'tu>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self { entity, gen_env }
	}

	pub fn type_ref(&self) -> TypeRef<'tu, 'ge> {
		TypeRef::new(self.entity.get_type().expect("Can't get typedef type"), self.gen_env)
	}

	pub fn underlying_type_ref(&self) -> TypeRef<'tu, 'ge> {
		TypeRef::new_ext(
			self
				.entity
				.get_typedef_underlying_type()
				.expect("Can't get typedef underlying type"),
			TypeRefTypeHint::None,
			Some(self.entity),
			self.gen_env,
		)
	}

	pub fn generated_types(&self) -> Vec<GeneratedType<'tu, 'ge>> {
		self.underlying_type_ref().generated_types()
	}
}

impl<'tu> EntityElement<'tu> for Typedef<'tu, '_> {
	fn entity(&self) -> Entity<'tu> {
		self.entity
	}
}

impl Element for Typedef<'_, '_> {
	fn is_excluded(&self) -> bool {
		DefaultElement::is_excluded(self)
			|| settings::PRIMITIVE_TYPEDEFS.contains_key(self.cpp_name(CppNameStyle::Reference).as_ref())
			|| {
				let underlying_type = self.underlying_type_ref();
				// fixes recursive typedefs like Cv16suf or GKernelPackage
				// fixme: don't rely on rust name to disconnect this generic module from rust_native
				self.rust_name(NameStyle::ref_()) == underlying_type.rust_name(NameStyle::ref_()) || underlying_type.is_ignored()
			}
	}

	fn is_ignored(&self) -> bool {
		DefaultElement::is_ignored(self) || self.underlying_type_ref().is_ignored()
	}

	fn is_system(&self) -> bool {
		DefaultElement::is_system(self)
	}

	fn is_public(&self) -> bool {
		DefaultElement::is_public(self)
	}

	fn usr(&self) -> Cow<str> {
		DefaultElement::usr(self)
	}

	fn cpp_namespace(&self) -> Cow<str> {
		DefaultElement::cpp_namespace(self).into()
	}

	fn cpp_name(&self, style: CppNameStyle) -> Cow<str> {
		DefaultElement::cpp_name(self, style)
	}
}

impl fmt::Display for Typedef<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.entity.get_display_name().expect("Can't get display name"))
	}
}

impl fmt::Debug for Typedef<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut debug_struct = f.debug_struct("Typedef");
		self
			.update_debug_struct(&mut debug_struct)
			.field("export_config", &self.gen_env.get_export_config(self.entity))
			.field("underlying_type_ref", &self.underlying_type_ref())
			.finish()
	}
}
