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
use crate::settings::{ARGUMENT_NAMES_MULTIPLE_SLICE, ARGUMENT_NAMES_NOT_SLICE, ARGUMENT_NAMES_USERDATA};
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

	pub(crate) fn to_desc(&self, new_typeref: Option<TypeRef<'tu, 'ge>>) -> Rc<FieldDesc<'tu, 'ge>> {
		match self {
			Self::Clang { .. } => Rc::new(FieldDesc {
				cpp_fullname: self.cpp_name(CppNameStyle::Reference).into(),
				type_ref: new_typeref.unwrap_or_else(|| self.type_ref().into_owned()),
				default_value: self.default_value().map(Rc::from),
			}),
			Self::Desc(desc) => {
				let mut out = Rc::clone(desc);
				if let Some(new_typeref) = new_typeref {
					Rc::make_mut(&mut out).type_ref = new_typeref;
				}
				out
			}
		}
	}

	pub fn type_ref_type_hint(&self) -> &TypeRefTypeHint {
		match self {
			Self::Clang { type_ref_type_hint, .. } => type_ref_type_hint,
			Self::Desc(desc) => desc.type_ref.type_hint(),
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
				if *desc.type_ref.type_hint() != type_ref_type_hint {
					Rc::make_mut(desc).type_ref.set_type_hint(type_ref_type_hint);
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
					let default_value_string = self.default_value().map_or(false, |def| def.contains(['"', '\'']));
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

	pub fn with_type_ref(mut self, type_ref: TypeRef<'tu, 'ge>) -> Self {
		self.set_type_ref(type_ref);
		self
	}

	pub fn set_type_ref(&mut self, type_ref: TypeRef<'tu, 'ge>) {
		match self {
			Self::Clang { .. } => *self = Self::Desc(self.to_desc(Some(type_ref))),
			Self::Desc(desc) => Rc::make_mut(desc).type_ref = type_ref,
		}
	}

	pub fn constness(&self) -> Constness {
		let type_ref = self.type_ref();
		match type_ref.kind().canonical().as_ref() {
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
				// fixme: clang-2.0.0 contains a bug that causes rust-1.78.0+ to panic when calling .tokenize()
				// https://github.com/KyleMayes/clang-rs/pull/58
				// this seems to be happening on entries marked as CV_DEPRECATED_EXTERNAL which don't have default args
				let has_literal_children = entity.visit_children(|c, _| match c.get_kind() {
					EntityKind::BoolLiteralExpr
					| EntityKind::CompoundLiteralExpr
					| EntityKind::UnexposedExpr
					| EntityKind::DeclRefExpr
					| EntityKind::CallExpr
					| EntityKind::StaticCastExpr
					| EntityKind::CStyleCastExpr
					| EntityKind::InitListExpr
					| EntityKind::ParenExpr
					| EntityKind::IntegerLiteral
					| EntityKind::FloatingLiteral
					| EntityKind::CharacterLiteral
					| EntityKind::StringLiteral
					| EntityKind::UnaryOperator
					| EntityKind::BinaryOperator => EntityVisitResult::Break,
					_ => EntityVisitResult::Continue,
				});

				if has_literal_children {
					if let Some(range) = entity.get_range() {
						let tokens = range.tokenize();
						let default_value_tokens = tokens
							.splitn(2, |t| t.get_kind() == TokenKind::Punctuation && t.get_spelling() == "=")
							.nth(1);
						if let Some(default_value_tokens) = default_value_tokens {
							return Some(constant::render_constant_cpp(default_value_tokens).into());
						}
					}
				}
				None
			}
			Self::Desc(desc) => desc.default_value.as_ref().map(|v| v.as_ref().into()),
		}
	}

	/// whether argument is used for passing user data to callback
	pub fn is_user_data(&self) -> bool {
		ARGUMENT_NAMES_USERDATA.contains(self.cpp_name(CppNameStyle::Declaration).as_ref()) && self.type_ref().kind().is_void_ptr()
	}

	pub fn slice_arg_eligibility(&self) -> SliceArgEligibility {
		self
			.type_ref()
			.kind()
			.as_pointer()
			.filter(|inner| inner.kind().is_copy(inner.type_hint()))
			.map_or(SliceArgEligibility::NotEligible, |_| {
				let name = self.cpp_name(CppNameStyle::Declaration);
				let name = name.as_ref();
				if ARGUMENT_NAMES_NOT_SLICE.contains(name) {
					SliceArgEligibility::NotEligible
				} else if ARGUMENT_NAMES_MULTIPLE_SLICE.contains(name) {
					SliceArgEligibility::EligibleWithMultiple
				} else {
					SliceArgEligibility::Eligible
				}
			})
	}

	pub fn can_be_slice_arg_len(&self) -> bool {
		let type_ref = self.type_ref();
		type_ref.kind().as_primitive().map_or(false, |(_, cpp)| {
			if cpp == "int" || cpp == "size_t" {
				let name = self.cpp_name(CppNameStyle::Declaration);
				name.ends_with('s') && name.contains('n') && name != "thickness" // fixme: have to exclude thickness
							|| name.contains("dims")
							|| name == "size"
							|| name.ends_with("Size")
							|| name == "len"
			} else {
				false
			}
		})
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
	pub default_value: Option<Rc<str>>,
}

impl<'tu, 'ge> FieldDesc<'tu, 'ge> {
	pub fn new(name: impl Into<Rc<str>>, type_ref: TypeRef<'tu, 'ge>) -> Self {
		Self {
			cpp_fullname: name.into(),
			type_ref,
			default_value: None,
		}
	}
}

pub enum SliceArgEligibility {
	NotEligible,
	Eligible,
	EligibleWithMultiple,
}
