// todo public static properties like opencv2/core/base.hpp:384 Hamming::normType
// todo test returning reference to array like cv_MatStep_buf

use std::borrow::Cow;
use std::fmt;
use std::rc::Rc;

use clang::token::TokenKind;
use clang::{Entity, EntityKind, EntityVisitResult};

use crate::debug::{DefinitionLocation, LocationName, NameDebug};
use crate::element::{ExcludeKind, UNNAMED};
use crate::settings::ArgOverride;
use crate::type_ref::{Constness, CppNameStyle, TypeRef, TypeRefTypeHint};
use crate::{constant, DefaultElement, Element, GeneratorEnv, StrExt};

#[derive(Clone, Copy, Debug)]
pub enum FieldTypeHint {
	None,
	ArgOverride(ArgOverride),
}

#[derive(Clone)]
pub enum Field<'tu, 'ge> {
	Clang {
		entity: Entity<'tu>,
		type_hint: FieldTypeHint,
		gen_env: &'ge GeneratorEnv<'tu>,
	},
	Desc(Rc<FieldDesc<'tu, 'ge>>),
}

impl<'tu, 'ge> Field<'tu, 'ge> {
	pub fn new(entity: Entity<'tu>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self::new_ext(entity, FieldTypeHint::None, gen_env)
	}

	pub fn new_ext(entity: Entity<'tu>, type_hint: FieldTypeHint, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self::Clang {
			entity,
			type_hint,
			gen_env,
		}
	}

	pub fn new_desc(desc: FieldDesc<'tu, 'ge>) -> Self {
		Self::Desc(Rc::new(desc))
	}

	pub fn type_hint(&self) -> FieldTypeHint {
		match self {
			&Self::Clang { type_hint, .. } => type_hint,
			Self::Desc(desc) => desc.type_hint,
		}
	}

	pub fn type_ref(&self) -> TypeRef<'tu, 'ge> {
		match self {
			&Self::Clang { entity, gen_env, .. } => {
				let type_hint = match self.type_hint() {
					FieldTypeHint::ArgOverride(over) => TypeRefTypeHint::ArgOverride(over),
					_ => {
						let default_value_string = self
							.default_value()
							.map_or(false, |def| def.contains(|c| c == '"' || c == '\''));
						if default_value_string {
							TypeRefTypeHint::ArgOverride(ArgOverride::CharAsRustChar)
						} else {
							TypeRefTypeHint::None
						}
					}
				};

				TypeRef::new_ext(entity.get_type().expect("Can't get type"), type_hint, Some(entity), gen_env)
			}
			Self::Desc(desc) => desc.type_ref.clone(),
		}
	}

	pub fn constness(&self) -> Constness {
		let type_ref = self.type_ref();
		Constness::from_is_mut(
			type_ref.as_array().is_some()
				|| type_ref.as_smart_ptr().is_some()
				|| type_ref.as_pointer().map_or(false, |r| r.constness().is_mut()),
		)
	}

	pub fn default_value(&self) -> Option<Cow<str>> {
		match self {
			&Self::Clang { entity, .. } => {
				let mut children = vec![];
				let mut skipping_typeref = true;
				entity.visit_children(|c, _| {
					if skipping_typeref && c.get_kind() != EntityKind::TypeRef {
						skipping_typeref = false;
					}
					if !skipping_typeref {
						children.push(c);
						EntityVisitResult::Recurse
					} else {
						EntityVisitResult::Continue
					}
				});

				if let Some(range) = entity.get_range() {
					let mut tokens = range.tokenize();
					let equal_pos = tokens
						.iter()
						.position(|t| t.get_kind() == TokenKind::Punctuation && t.get_spelling() == "=");
					if let Some(equal_pos) = equal_pos {
						tokens.drain(..equal_pos + 1);
						return Some(constant::render_constant_cpp(tokens).into());
					}
				}
				None
			}
			Self::Desc(desc) => desc.default_value.as_ref().map(|v| v.as_ref().into()),
		}
	}

	/// whether argument is used for passing user data to callback
	pub fn is_user_data(&self) -> bool {
		let leafname = self.cpp_name(CppNameStyle::Declaration);
		(leafname == "userdata" || leafname == "userData" || leafname == "cookie" || leafname == UNNAMED)
			&& self.type_ref().is_void_ptr()
	}

	pub fn as_slice_len(&self) -> Option<(&'static str, usize)> {
		if let FieldTypeHint::ArgOverride(ArgOverride::LenForSlice(ptr_arg, len_div)) = self.type_hint() {
			Some((ptr_arg, len_div))
		} else {
			None
		}
	}
}

impl Element for Field<'_, '_> {
	fn exclude_kind(&self) -> ExcludeKind {
		DefaultElement::exclude_kind(self).with_reference_exclude_kind(|| self.type_ref().exclude_kind())
	}

	fn is_system(&self) -> bool {
		match self {
			&Self::Clang { entity, .. } => DefaultElement::is_system(entity),
			Self::Desc(_) => false,
		}
	}

	fn is_public(&self) -> bool {
		match self {
			&Self::Clang { entity, .. } => DefaultElement::is_public(entity),
			Self::Desc(_) => true,
		}
	}

	fn doc_comment(&self) -> Cow<str> {
		match self {
			Field::Clang { entity, .. } => entity.get_comment().unwrap_or_default().into(),
			Field::Desc(_) => "".into(),
		}
	}

	fn cpp_namespace(&self) -> Cow<str> {
		match self {
			&Self::Clang { entity, .. } => DefaultElement::cpp_namespace(entity).into(),
			Self::Desc(desc) => desc.cpp_fullname.namespace().into(),
		}
	}

	fn cpp_name(&self, style: CppNameStyle) -> Cow<str> {
		match self {
			&Self::Clang { entity, .. } => DefaultElement::cpp_name(self, entity, style),
			Self::Desc(desc) => desc.cpp_fullname.cpp_name_from_fullname(style).into(),
		}
	}
}

impl<'me> NameDebug<'me> for &'me Field<'_, '_> {
	fn file_line_name(self) -> LocationName<'me> {
		match self {
			Field::Clang { entity, .. } => entity.file_line_name(),
			Field::Desc(_) => LocationName::new(DefinitionLocation::Generated, self.cpp_name(CppNameStyle::Reference)),
		}
	}
}

impl fmt::Debug for Field<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct(match self {
			Self::Clang { .. } => "Field::Clang",
			Self::Desc(_) => "Field::Desc",
		})
		.field("cpp_fullname", &self.cpp_name(CppNameStyle::Reference))
		.field("type_hint", &self.type_hint())
		.field("type_ref", &self.type_ref())
		.field("default_value", &self.default_value())
		.finish()
	}
}

#[derive(Clone)]
pub struct FieldDesc<'tu, 'ge> {
	pub cpp_fullname: Rc<str>,
	pub type_ref: TypeRef<'tu, 'ge>,
	pub type_hint: FieldTypeHint,
	pub default_value: Option<Rc<str>>,
}

impl<'tu, 'ge> FieldDesc<'tu, 'ge> {
	pub fn new(name: impl Into<Rc<str>>, type_ref: TypeRef<'tu, 'ge>) -> Self {
		Self {
			cpp_fullname: name.into(),
			type_ref,
			type_hint: FieldTypeHint::None,
			default_value: None,
		}
	}
}
