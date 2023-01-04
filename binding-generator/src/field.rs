// todo public static properties like opencv2/core/base.hpp:384 Hamming::normType
// todo test returning reference to array like cv_MatStep_buf

use std::borrow::Cow;
use std::fmt;

use clang::token::TokenKind;
use clang::{Entity, EntityKind, EntityVisitResult, Type};

use crate::settings::ArgOverride;
use crate::type_ref::{CppNameStyle, TypeRefTypeHint};
use crate::{constant, Class, DefaultElement, Element, EntityElement, GeneratorEnv, TypeRef};

#[derive(Clone, Copy, Debug)]
pub enum FieldTypeHint<'tu> {
	None,
	ArgOverride(ArgOverride),
	FieldSetter,
	Specialized(Type<'tu>),
}

impl Default for FieldTypeHint<'_> {
	fn default() -> Self {
		FieldTypeHint::None
	}
}

#[derive(Clone)]
pub struct Field<'tu, 'ge> {
	entity: Entity<'tu>,
	type_hint: FieldTypeHint<'tu>,
	gen_env: &'ge GeneratorEnv<'tu>,
}

impl<'tu, 'ge> Field<'tu, 'ge> {
	pub fn new(entity: Entity<'tu>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self::new_ext(entity, Default::default(), gen_env)
	}

	pub fn new_ext(entity: Entity<'tu>, type_hint: FieldTypeHint<'tu>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self {
			entity,
			type_hint,
			gen_env,
		}
	}

	pub fn type_hint(&self) -> FieldTypeHint<'tu> {
		self.type_hint
	}

	pub fn type_ref(&self) -> TypeRef<'tu, 'ge> {
		let type_hint = match self.type_hint {
			FieldTypeHint::ArgOverride(over) => TypeRefTypeHint::ArgOverride(over),
			FieldTypeHint::Specialized(typ) => TypeRefTypeHint::Specialized(typ),
			FieldTypeHint::FieldSetter => TypeRefTypeHint::PrimitiveRefAsPointer,
			_ => TypeRefTypeHint::None,
		};
		TypeRef::new_ext(
			self.entity.get_type().expect("Can't get type"),
			type_hint,
			Some(self.entity),
			self.gen_env,
		)
	}

	pub fn default_value(&self) -> Option<String> {
		let mut children = vec![];
		let mut skipping_typeref = true;
		self.entity.visit_children(|c, _| {
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

		if let Some(range) = self.entity.get_range() {
			let mut tokens = range.tokenize();
			let equal_pos = tokens
				.iter()
				.position(|t| t.get_kind() == TokenKind::Punctuation && t.get_spelling() == "=");
			if let Some(equal_pos) = equal_pos {
				tokens.drain(..equal_pos + 1);
				return Some(constant::render_constant_cpp(tokens));
			}
		}
		None
	}

	pub fn parent(&self) -> Class<'tu, 'ge> {
		let parent_entity = self.entity.get_semantic_parent().expect("Can't get parent of field");
		match parent_entity.get_kind() {
			EntityKind::ClassDecl | EntityKind::StructDecl | EntityKind::ClassTemplate => Class::new(parent_entity, self.gen_env),
			_ => {
				panic!("Unexpected field parent entity: {:#?}", parent_entity);
			}
		}
	}

	/// whether argument is used for passing user data to callback
	pub fn is_user_data(&self) -> bool {
		if matches!(self.type_hint, FieldTypeHint::FieldSetter) {
			return false;
		}
		let type_ref = self.type_ref();
		let leafname = self.cpp_name(CppNameStyle::Declaration);
		(leafname == "userdata" || leafname == "userData" || leafname == "cookie" || leafname == "unnamed")
			&& type_ref.is_void_ptr()
	}

	pub fn as_slice_len(&self) -> Option<(&'static str, usize)> {
		if let FieldTypeHint::ArgOverride(ArgOverride::LenForSlice(ptr_arg, len_div)) = self.type_hint {
			Some((ptr_arg, len_div))
		} else {
			None
		}
	}
}

impl<'tu> EntityElement<'tu> for Field<'tu, '_> {
	fn entity(&self) -> Entity<'tu> {
		self.entity
	}
}

impl Element for Field<'_, '_> {
	fn is_ignored(&self) -> bool {
		DefaultElement::is_ignored(self) || self.type_ref().is_ignored()
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
		if matches!(self.type_hint, FieldTypeHint::FieldSetter) {
			"val".into()
		} else {
			DefaultElement::cpp_name(self, style)
		}
	}
}

impl fmt::Display for Field<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.entity.get_display_name().expect("Can't get display name"))
	}
}

impl fmt::Debug for Field<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Field")
			.field("type_hint", &self.type_hint)
			.field("type_ref", &self.type_ref())
			.field("default_value", &self.default_value())
			.finish()
	}
}
