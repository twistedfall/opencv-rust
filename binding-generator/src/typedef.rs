use std::borrow::Cow;
use std::fmt;

use clang::Entity;

use crate::type_ref::{CppNameStyle, DependentTypeMode, FishStyle, Kind, NameStyle, TypeRefTypeHint};
use crate::{settings, DefaultElement, DependentType, Element, EntityElement, GeneratorEnv, TypeRef};

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

	pub fn dependent_types(&self) -> Vec<DependentType<'tu, 'ge>> {
		self.underlying_type_ref().dependent_types(DependentTypeMode::None)
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
			|| self.rust_name(NameStyle::ref_()) == self.underlying_type_ref().rust_name(NameStyle::ref_()) // fixes recursive typedefs like Cv16suf
			|| settings::PRIMITIVE_TYPEDEFS.contains_key(self.cpp_name(CppNameStyle::Reference).as_ref())
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

	fn rendered_doc_comment_with_prefix(&self, prefix: &str, opencv_version: &str) -> String {
		DefaultElement::rendered_doc_comment_with_prefix(self, prefix, opencv_version)
	}

	fn cpp_namespace(&self) -> Cow<str> {
		DefaultElement::cpp_namespace(self).into()
	}

	fn cpp_name(&self, style: CppNameStyle) -> Cow<str> {
		DefaultElement::cpp_name(self, style)
	}

	fn rust_module(&self) -> Cow<str> {
		DefaultElement::rust_module(self)
	}

	fn rust_name(&self, style: NameStyle) -> Cow<str> {
		DefaultElement::rust_name(self, style)
	}

	fn rust_leafname(&self, _fish_style: FishStyle) -> Cow<str> {
		match self.underlying_type_ref().source().kind() {
			Kind::Class(..) | Kind::Function(..) | Kind::StdVector(..) | Kind::SmartPtr(..) => {
				DefaultElement::cpp_name(self, CppNameStyle::Declaration)
			}
			_ => DefaultElement::rust_leafname(self),
		}
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
