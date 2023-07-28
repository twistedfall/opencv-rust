use std::borrow::Cow;
use std::fmt;

use clang::{Entity, Type};

use crate::element::ExcludeKind;
use crate::type_ref::{Constness, CppNameStyle, TemplateArg};
use crate::{DefaultElement, Element, EntityElement, GeneratorEnv, TypeRef};

#[derive(Clone)]
pub struct Tuple<'tu, 'ge> {
	type_ref: Type<'tu>,
	gen_env: &'ge GeneratorEnv<'tu>,
	/// true for C++ representation as `pair` instead of `tuple`
	tuple_type: &'static str,
}

impl<'tu, 'ge> Tuple<'tu, 'ge> {
	pub fn new(type_ref: Type<'tu>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self {
			type_ref,
			gen_env,
			tuple_type: "tuple",
		}
	}

	pub fn pair(type_ref: Type<'tu>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self {
			type_ref,
			gen_env,
			tuple_type: "pair",
		}
	}

	pub fn type_ref(&self) -> TypeRef<'tu, 'ge> {
		TypeRef::new(self.type_ref, self.gen_env)
	}

	pub fn elements(&self) -> Vec<TypeRef<'tu, 'ge>> {
		#[allow(clippy::unnecessary_to_owned)]
		self
			.type_ref()
			.template_specialization_args()
			.into_owned()
			.into_iter()
			.filter_map(TemplateArg::into_typename)
			.collect()
	}

	pub fn constness(&self) -> Constness {
		Constness::from_is_const(self.type_ref.is_const_qualified())
	}
}

impl<'tu> EntityElement<'tu> for Tuple<'tu, '_> {
	fn entity(&self) -> Entity<'tu> {
		self.type_ref.get_declaration().expect("Can't get declaration")
	}
}

impl Element for Tuple<'_, '_> {
	fn exclude_kind(&self) -> ExcludeKind {
		DefaultElement::exclude_kind(self).with_exclude_kind(|| {
			self
				.elements()
				.into_iter()
				.map(|tref| tref.exclude_kind())
				.reduce(|left, right| left.with_exclude_kind(|| right))
				.unwrap_or(ExcludeKind::Included)
		})
	}

	fn is_system(&self) -> bool {
		DefaultElement::is_system(self.entity())
	}

	fn is_public(&self) -> bool {
		DefaultElement::is_public(self.entity())
	}

	fn doc_comment(&self) -> Cow<str> {
		self.entity().get_comment().unwrap_or_default().into()
	}

	fn cpp_namespace(&self) -> Cow<str> {
		"std".into()
	}

	fn cpp_name(&self, style: CppNameStyle) -> Cow<str> {
		match style {
			CppNameStyle::Declaration => self.tuple_type.into(),
			CppNameStyle::Reference => DefaultElement::cpp_decl_name_with_namespace(self, self.tuple_type),
		}
	}
}

impl PartialEq for Tuple<'_, '_> {
	fn eq(&self, other: &Self) -> bool {
		self.type_ref == other.type_ref && self.tuple_type == other.tuple_type
	}
}

impl fmt::Display for Tuple<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.entity().get_display_name().expect("Can't get display name"))
	}
}

impl fmt::Debug for Tuple<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut debug_struct = f.debug_struct("Tuple");
		self
			.update_debug_struct(&mut debug_struct)
			.field("export_config", &self.gen_env.get_export_config(self.entity()))
			.finish()
	}
}
