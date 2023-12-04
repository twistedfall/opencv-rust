// todo public static properties like opencv2/core/base.hpp:384 Hamming::normType
// todo test returning reference to array like cv_MatStep_buf

use std::borrow::Cow;
use std::fmt;
use std::rc::Rc;

use clang::token::TokenKind;
use clang::{Entity, EntityKind, EntityVisitResult};

use crate::comment::strip_doxygen_comment_markers;
use crate::debug::{DefinitionLocation, LocationName, NameDebug};
use crate::element::ExcludeKind;
use crate::settings::ARGUMENT_NAMES_USERDATA;
use crate::type_ref::{Constness, CppNameStyle, TypeRef, TypeRefKind, TypeRefTypeHint};
use crate::{constant, DefaultElement, Element, GeneratorEnv, StrExt};

/// Represents a field of a struct or a class or a function argument. Basically a name + type.
#[derive(Clone)]
pub enum Field<'tu, 'ge> {
	Clang {
		entity: Entity<'tu>,
		type_ref_type_hint: TypeRefTypeHint,
		gen_env: &'ge GeneratorEnv<'tu>,
	},
	Desc(Rc<FieldDesc<'tu, 'ge>>),
}

impl<'tu, 'ge> Field<'tu, 'ge> {
	pub fn new(entity: Entity<'tu>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self::new_ext(entity, TypeRefTypeHint::None, gen_env)
	}

	pub fn new_ext(entity: Entity<'tu>, type_ref_type_hint: TypeRefTypeHint, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self::Clang {
			entity,
			type_ref_type_hint,
			gen_env,
		}
	}

	pub fn new_desc(desc: FieldDesc<'tu, 'ge>) -> Self {
		Self::Desc(Rc::new(desc))
	}

	pub fn type_ref_type_hint(&self) -> &TypeRefTypeHint {
		match self {
			Self::Clang { type_ref_type_hint, .. } => type_ref_type_hint,
			Self::Desc(desc) => &desc.type_ref_type_hint,
		}
	}

	pub fn set_type_ref_type_hint(&mut self, type_ref_type_hint: TypeRefTypeHint) {
		match self {
			Self::Clang {
				type_ref_type_hint: self_type_ref_type_hint,
				..
			} => {
				if *self_type_ref_type_hint != type_ref_type_hint {
					*self_type_ref_type_hint = type_ref_type_hint;
				}
			}
			Self::Desc(desc) => {
				if desc.type_ref_type_hint != type_ref_type_hint {
					Rc::make_mut(desc).type_ref_type_hint = type_ref_type_hint;
				}
			}
		}
	}

	pub fn type_ref(&self) -> Cow<TypeRef<'tu, 'ge>> {
		match self {
			Self::Clang {
				entity,
				type_ref_type_hint,
				gen_env,
				..
			} => {
				let type_ref_type_hint = type_ref_type_hint.clone().something_or_else(|| {
					let default_value_string = self
						.default_value()
						.map_or(false, |def| def.contains(|c| c == '"' || c == '\''));
					if default_value_string {
						TypeRefTypeHint::CharAsRustChar
					} else {
						TypeRefTypeHint::None
					}
				});
				Cow::Owned(TypeRef::new_ext(
					entity.get_type().expect("Can't get type"),
					type_ref_type_hint,
					Some(*entity),
					gen_env,
				))
			}
			Self::Desc(desc) => Cow::Borrowed(&desc.type_ref),
		}
	}

	pub fn constness(&self) -> Constness {
		let type_ref = self.type_ref();
		match type_ref.canonical().kind().as_ref() {
			TypeRefKind::Array(_, _) | TypeRefKind::SmartPtr(_) => Constness::Mut,
			TypeRefKind::Pointer(pointee) | TypeRefKind::Reference(pointee) => pointee.constness(),
			TypeRefKind::Primitive(_, _)
			| TypeRefKind::StdVector(_)
			| TypeRefKind::StdTuple(_)
			| TypeRefKind::RValueReference(_)
			| TypeRefKind::Class(_)
			| TypeRefKind::Enum(_)
			| TypeRefKind::Function(_)
			| TypeRefKind::Typedef(_)
			| TypeRefKind::Generic(_)
			| TypeRefKind::Ignored => Constness::Const,
		}
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
					let tokens = range.tokenize();
					let equal_pos = tokens
						.iter()
						.position(|t| t.get_kind() == TokenKind::Punctuation && t.get_spelling() == "=");
					if let Some(equal_pos) = equal_pos {
						return Some(constant::render_constant_cpp(&tokens[equal_pos + 1..]).into());
					}
				}
				None
			}
			Self::Desc(desc) => desc.default_value.as_ref().map(|v| v.as_ref().into()),
		}
	}

	/// whether argument is used for passing user data to callback
	pub fn is_user_data(&self) -> bool {
		self.type_ref().is_void_ptr() && ARGUMENT_NAMES_USERDATA.contains(self.cpp_name(CppNameStyle::Declaration).as_ref())
	}

	pub fn can_be_slice_arg(&self) -> bool {
		let type_ref = self.type_ref();
		type_ref.constness().is_const() && type_ref.as_pointer().is_some()
	}

	pub fn can_be_slice_arg_len(&self) -> bool {
		let name = self.cpp_name(CppNameStyle::Declaration);
		let type_ref = self.type_ref();
		type_ref
			.as_primitive()
			.map_or(false, |(_, cpp)| cpp == "int" || cpp == "size_t")
			&& (name.ends_with('s') && name.contains('n') || name.contains("dims"))
	}

	pub fn as_slice_len(&self) -> Option<(&str, usize)> {
		if let TypeRefTypeHint::LenForSlice(ptr_arg, len_div) = self.type_ref_type_hint() {
			Some((ptr_arg.as_str(), *len_div))
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
			Field::Clang { entity, .. } => strip_doxygen_comment_markers(&entity.get_comment().unwrap_or_default()).into(),
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
		.field("type_ref_type_hint", &self.type_ref_type_hint())
		.field("type_ref", &self.type_ref())
		.field("default_value", &self.default_value())
		.finish()
	}
}

#[derive(Clone)]
pub struct FieldDesc<'tu, 'ge> {
	pub cpp_fullname: Rc<str>,
	pub type_ref: TypeRef<'tu, 'ge>,
	pub type_ref_type_hint: TypeRefTypeHint,
	pub default_value: Option<Rc<str>>,
}

impl<'tu, 'ge> FieldDesc<'tu, 'ge> {
	pub fn new(name: impl Into<Rc<str>>, type_ref: TypeRef<'tu, 'ge>) -> Self {
		Self {
			cpp_fullname: name.into(),
			type_ref,
			type_ref_type_hint: TypeRefTypeHint::None,
			default_value: None,
		}
	}
}
