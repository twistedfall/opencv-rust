use std::borrow::Cow;
use std::fmt;
use std::fmt::Write;

use clang::token::{Token, TokenKind};
use clang::{Entity, EntityKind, EvaluationResult};

use crate::debug::LocationName;
use crate::element::ExcludeKind;
use crate::type_ref::CppNameStyle;
use crate::{settings, DefaultElement, Element, EntityElement, NameDebug};

pub fn render_constant_rust<'f>(tokens: impl IntoIterator<Item = Token<'f>>) -> Option<Value> {
	let mut out = Value {
		kind: ValueKind::Integer,
		value: String::with_capacity(128),
	};
	for t in tokens {
		match t.get_kind() {
			TokenKind::Comment => {
				write!(out.value, "/* {} */", t.get_spelling()).expect("write! to String shouldn't fail");
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
				if spelling.contains(|c| c == '"' || c == '\'') {
					out.kind = ValueKind::String;
				} else if spelling.contains('.') {
					out.kind = ValueKind::Float;
				} else if spelling.ends_with(&['U', 'u'][..]) {
					out.kind = ValueKind::UnsignedInteger;
					out.value += &spelling[..spelling.len() - 1];
					continue;
				}
				out.value += &spelling;
			}
			TokenKind::Punctuation => {
				let spelling = t.get_spelling();
				if spelling == "{" || spelling == "}" {
					return None;
				}
				out.value += &t.get_spelling();
			}
		}
	}
	Some(out)
}

pub fn render_constant_cpp<'f>(tokens: impl IntoIterator<Item = Token<'f>>) -> String {
	tokens.into_iter().fold(String::new(), |out, x| out + &x.get_spelling())
}

pub fn render_evaluation_result_rust(result: EvaluationResult) -> Value {
	match result {
		EvaluationResult::Unexposed => {
			panic!("Can't render evaluation result")
		}
		EvaluationResult::SignedInteger(x) => Value {
			kind: ValueKind::Integer,
			value: x.to_string(),
		},
		EvaluationResult::UnsignedInteger(x) => Value {
			kind: ValueKind::UnsignedInteger,
			value: x.to_string(),
		},
		EvaluationResult::Float(x) => Value {
			kind: ValueKind::Float,
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
pub struct Const<'tu> {
	entity: Entity<'tu>,
}

impl<'tu> Const<'tu> {
	pub fn new(entity: Entity<'tu>) -> Self {
		Self { entity }
	}

	pub fn value(&self) -> Option<Value> {
		match self.entity.get_kind() {
			EntityKind::MacroDefinition => {
				let mut tokens = self.entity.get_range().expect("Can't get macro definition range").tokenize();
				if tokens.len() <= 1 {
					None
				} else if let Some(ident_tok) = tokens.get(0) {
					if ident_tok.get_kind() == TokenKind::Identifier {
						render_constant_rust(tokens.drain(1..))
					} else {
						None
					}
				} else {
					None
				}
			}
			EntityKind::EnumConstantDecl => Some(Value {
				kind: ValueKind::Integer,
				value: self
					.entity
					.get_enum_constant_value()
					.expect("Can't get enum constant value")
					.0
					.to_string(),
			}),
			EntityKind::VarDecl => self.entity.evaluate().map(render_evaluation_result_rust),
			_ => {
				unreachable!("Invalid entity type for constant")
			}
		}
	}
}

impl<'tu> EntityElement<'tu> for Const<'tu> {
	fn entity(&self) -> Entity<'tu> {
		self.entity
	}
}

impl Element for Const<'_> {
	fn exclude_kind(&self) -> ExcludeKind {
		DefaultElement::exclude_kind(self).with_is_excluded(|| {
			self.entity.is_function_like_macro()
				&& !settings::IMPLEMENTED_FUNCTION_LIKE_MACROS.contains(self.cpp_name(CppNameStyle::Reference).as_ref())
		})
	}

	fn is_system(&self) -> bool {
		DefaultElement::is_system(self.entity)
	}

	fn is_public(&self) -> bool {
		DefaultElement::is_public(self.entity)
	}

	fn doc_comment(&self) -> Cow<str> {
		self.entity.get_comment().unwrap_or_default().into()
	}

	fn cpp_namespace(&self) -> Cow<str> {
		DefaultElement::cpp_namespace(self.entity).into()
	}

	fn cpp_name(&self, style: CppNameStyle) -> Cow<str> {
		DefaultElement::cpp_name(self, self.entity(), style)
	}
}

impl<'me> NameDebug<'me> for &'me Const<'_> {
	fn file_line_name(self) -> LocationName<'me> {
		self.entity.file_line_name()
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueKind {
	Integer,
	UnsignedInteger,
	Float,
	String,
}

#[derive(Clone, Debug)]
pub struct Value {
	pub kind: ValueKind,
	pub value: String,
}

impl fmt::Display for Value {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.kind == ValueKind::Float && !self.value.contains('.') {
			write!(f, "{}.", self.value)
		} else {
			write!(f, "{}", self.value)
		}
	}
}
