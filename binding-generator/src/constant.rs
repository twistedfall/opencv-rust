use std::borrow::Cow;
use std::borrow::Cow::{Borrowed, Owned};
use std::rc::Rc;

use clang::token::{Token, TokenKind};
use clang::{Entity, EntityKind, EvaluationResult};
use desc::ConstDesc;

use crate::comment::strip_doxygen_comment_markers;
use crate::debug::{DefinitionLocation, LocationName};
use crate::element::ExcludeKind;
use crate::type_ref::CppNameStyle;
use crate::{settings, DefaultElement, Element, NameDebug, StrExt};

mod desc;

pub fn render_constant_cpp(tokens: &[Token]) -> String {
	tokens
		.iter()
		.fold(String::with_capacity(tokens.len() * 8), |out, x| out + &x.get_spelling())
}

pub fn render_evaluation_result_rust(result: EvaluationResult) -> Value {
	match result {
		EvaluationResult::Unexposed => panic!("Can't render evaluation result"),
		EvaluationResult::SignedInteger(x) => Value {
			kind: ValueKind::Integer,
			value: x.to_string(),
		},
		EvaluationResult::UnsignedInteger(x) => Value {
			kind: ValueKind::UnsignedInteger,
			value: x.to_string(),
		},
		EvaluationResult::Float(x) => Value {
			kind: ValueKind::Double,
			value: x.to_string(),
		},
		EvaluationResult::String(x)
		| EvaluationResult::ObjCString(x)
		| EvaluationResult::CFString(x)
		| EvaluationResult::Other(x) => Value {
			kind: ValueKind::String,
			value: format!(r#""{}""#, x.to_string_lossy()),
		},
	}
}

#[derive(Clone, Debug)]
pub enum Const<'tu> {
	Clang { entity: Entity<'tu> },
	Desc(Rc<ConstDesc>),
}

impl<'tu> Const<'tu> {
	pub fn new(entity: Entity<'tu>) -> Self {
		Self::Clang { entity }
	}

	pub fn value(&self) -> Option<Cow<'_, Value>> {
		match self {
			Self::Clang { entity } => match entity.get_kind() {
				EntityKind::MacroDefinition => {
					let tokens = entity.get_range().expect("Can't get macro definition range").tokenize();
					if tokens.len() <= 1 {
						None
					} else {
						Value::try_from_tokens(&tokens[1..]).map(Owned)
					}
				}
				EntityKind::EnumConstantDecl => Some(Owned(Value {
					kind: ValueKind::Integer,
					value: entity
						.get_enum_constant_value()
						.expect("Can't get enum constant value")
						.0
						.to_string(),
				})),
				EntityKind::VarDecl => entity.evaluate().map(render_evaluation_result_rust).map(Owned),
				_ => unreachable!("Invalid entity type for constant"),
			},
			Self::Desc(desc) => Some(Borrowed(desc.value.as_ref())),
		}
	}
}

impl Element for Const<'_> {
	fn exclude_kind(&self) -> ExcludeKind {
		match self {
			Self::Clang { entity } => DefaultElement::exclude_kind(self).with_is_excluded(|| {
				entity.is_function_like_macro()
					&& !settings::IMPLEMENTED_FUNCTION_LIKE_MACROS.contains(self.cpp_name(CppNameStyle::Reference).as_ref())
			}),
			Self::Desc(_) => ExcludeKind::Included,
		}
	}

	fn is_system(&self) -> bool {
		match self {
			&Self::Clang { entity } => DefaultElement::is_system(entity),
			Self::Desc(_) => false,
		}
	}

	fn is_public(&self) -> bool {
		match self {
			&Self::Clang { entity } => DefaultElement::is_public(entity),
			Self::Desc(_) => true,
		}
	}

	fn doc_comment(&self) -> Cow<'_, str> {
		match self {
			Self::Clang { entity } => Owned(strip_doxygen_comment_markers(&entity.get_comment().unwrap_or_default())),
			Self::Desc(_) => Borrowed(""),
		}
	}

	fn cpp_namespace(&self) -> Cow<'_, str> {
		match self {
			&Self::Clang { entity } => DefaultElement::cpp_namespace(entity).into(),
			Self::Desc(desc) => Borrowed(desc.cpp_fullname.namespace()),
		}
	}

	fn cpp_name(&self, style: CppNameStyle) -> Cow<'_, str> {
		match self {
			&Self::Clang { entity } => DefaultElement::cpp_name(self, entity, style),
			Self::Desc(desc) => Borrowed(desc.cpp_fullname.cpp_name_from_fullname(style)),
		}
	}
}

impl<'me> NameDebug<'me> for &'me Const<'_> {
	fn file_line_name(self) -> LocationName<'me> {
		match self {
			Const::Clang { entity } => entity.file_line_name(),
			Const::Desc(desc) => LocationName::new(DefinitionLocation::Generated, desc.cpp_fullname.as_ref()),
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueKind {
	Integer,
	UnsignedInteger,
	Usize,
	Float,
	Double,
	String,
}

#[derive(Clone, Debug)]
pub struct Value {
	pub kind: ValueKind,
	pub value: String,
}

impl Value {
	pub fn try_from_tokens(tokens: &[Token]) -> Option<Self> {
		let mut out = Value {
			kind: ValueKind::Integer,
			value: String::with_capacity(tokens.len() * 8),
		};
		for t in tokens {
			match t.get_kind() {
				TokenKind::Comment => {
					out.value.push_str("/* ");
					out.value.push_str(&t.get_spelling());
					out.value.push_str(" */");
				}
				TokenKind::Identifier => {
					let spelling = t.get_spelling();
					if let Some(entity) = t.get_location().get_entity() {
						if let EntityKind::MacroExpansion = entity.get_kind() {
							let cnst = Const::new(entity);
							if cnst.exclude_kind().is_excluded() {
								return None;
							}
						}
					}
					if spelling.starts_with("CV_") {
						out.value += &spelling;
					} else {
						return None;
					}
				}
				TokenKind::Keyword => {
					return None;
				}
				TokenKind::Literal => {
					let spelling = t.get_spelling();
					if spelling.contains(['"', '\'']) {
						out.kind = ValueKind::String;
						out.value += &spelling;
					} else if spelling.contains('.') {
						if let Some(float) = spelling.strip_suffix(['F', 'f']) {
							out.kind = ValueKind::Float;
							out.value += float;
						} else {
							out.kind = ValueKind::Double;
							out.value += &spelling;
						}
					} else if let Some(unsigned_value) = spelling.strip_suffix(['U', 'u']) {
						out.kind = ValueKind::UnsignedInteger;
						out.value += unsigned_value;
					} else {
						out.value += &spelling;
					}
				}
				TokenKind::Punctuation => {
					let spelling = t.get_spelling();
					if spelling == "{" || spelling == "}" {
						return None;
					}
					out.value += &spelling;
				}
			}
		}
		Some(out)
	}
}
