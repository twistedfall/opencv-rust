use std::{
	borrow::Cow,
	fmt,
};

use clang::{
	Entity,
	EntityKind,
	EvaluationResult,
	token::{Token, TokenKind},
};
use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::{
	CompiledInterpolation,
	DefaultElement,
	Element,
	EntityElement,
	GeneratedElement,
	get_debug,
	settings,
	StrExt,
};

pub fn render_constant_rust<'f>(tokens: impl IntoIterator<Item=Token<'f>>) -> Option<Value> {
	let mut out = Value {
		kind: ValueKind::Integer,
		value: String::with_capacity(128),
	};
	for t in tokens {
		match t.get_kind() {
			TokenKind::Comment => {
				out.value += &format!("/* {} */", t.get_spelling());
			}
			TokenKind::Identifier => {
				let spelling = t.get_spelling();
				if let Some(entity) = t.get_location().get_entity() {
					match entity.get_kind() {
						EntityKind::MacroExpansion => {
							let cnst = Const::new(entity);
							if cnst.is_excluded() {
								return None;
							}
						}
						_ => {}
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
				if spelling.contains('"') {
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

pub fn render_constant_cpp<'f>(tokens: impl IntoIterator<Item=Token<'f>>) -> String {
	tokens.into_iter()
		.fold(String::new(), |out, x| out + &x.get_spelling())
}

pub fn render_evaluation_result_rust(result: EvaluationResult) -> Value {
	match result {
		EvaluationResult::Unexposed => {
			panic!("Can't render evaluation result")
		}
		EvaluationResult::SignedInteger(x) => {
			Value {
				kind: ValueKind::Integer,
				value: x.to_string(),
			}
		}
		EvaluationResult::UnsignedInteger(x) => {
			Value {
				kind: ValueKind::UnsignedInteger,
				value: x.to_string(),
			}
		}
		EvaluationResult::Float(x) => {
			Value {
				kind: ValueKind::Float,
				value: x.to_string(),
			}
		}
		EvaluationResult::String(x) | EvaluationResult::ObjCString(x) | EvaluationResult::CFString(x)
		| EvaluationResult::Other(x) => {
			Value {
				kind: ValueKind::String,
				value: format!(r#""{}""#, x.to_string_lossy()),
			}
		}
	}
}

#[derive(Debug)]
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
				let mut tokens = self.entity.get_range().expect("Can't get macro definition range")
					.tokenize();
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
			EntityKind::EnumConstantDecl => {
				Some(Value {
					kind: ValueKind::Integer,
					value: self.entity.get_enum_constant_value().expect("Can't get enum constant value").0.to_string(),
				})
			}
			EntityKind::VarDecl => {
				self.entity.evaluate().map(render_evaluation_result_rust)
			}
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
	fn is_excluded(&self) -> bool {
		DefaultElement::is_excluded(self)
			|| (self.entity.is_function_like_macro() && !settings::IMPLEMENTED_FUNCTION_LIKE_MACROS.contains(self.cpp_fullname().as_ref()))
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
		DefaultElement::cpp_namespace(self)
	}

	fn cpp_localname(&self) -> Cow<str> {
		DefaultElement::cpp_localname(self)
	}

	fn rust_module(&self) -> Cow<str> {
		DefaultElement::rust_module(self)
	}

	fn rust_leafname(&self) -> Cow<str> {
		self.cpp_localname()
	}

	fn rust_localname(&self) -> Cow<str> {
		let mut out = DefaultElement::rust_localname(self);
		const SUFFIX: &str = "_OCVRS_OVERRIDE";
		if out.ends_with(SUFFIX) {
			let suffix_start = out.len() - SUFFIX.len();
			out.to_mut().drain(suffix_start..);
		}
		out
	}
}

impl GeneratedElement for Const<'_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module(), self.rust_localname())
	}

	fn gen_rust(&self, opencv_version: &str) -> String {
		static STRING_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/const/string.tpl.rs").compile_interpolation()
		);

		static INT_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/const/int.tpl.rs").compile_interpolation()
		);

		static UINT_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/const/uint.tpl.rs").compile_interpolation()
		);

		static USIZE_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/const/usize.tpl.rs").compile_interpolation()
		);

		static FLOAT_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/const/float.tpl.rs").compile_interpolation()
		);

		let name = self.rust_localname();

		if let Some(value) = self.value() {
			let tpl = if settings::CONST_TYPE_USIZE.contains(name.as_ref()) {
				&USIZE_TPL
			} else {
				match value.kind {
					ValueKind::Integer => &INT_TPL,
					ValueKind::UnsignedInteger => &UINT_TPL,
					ValueKind::Float => &FLOAT_TPL,
					ValueKind::String => &STRING_TPL,
				}
			};
			tpl.interpolate(&hashmap! {
				"doc_comment" => Cow::Owned(self.rendered_doc_comment(opencv_version)),
				"debug" => get_debug(self).into(),
				"name" => name,
				"value" => value.to_string().into(),
			})
		} else {
			String::new()
		}
	}
}

impl fmt::Display for Const<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.entity.get_display_name().expect("Can't get display name"))
	}
}

#[derive(Clone, Copy, Debug)]
pub enum ValueKind {
	Integer,
	UnsignedInteger,
	Float,
	String,
}

#[derive(Clone, Debug)]
pub struct Value {
	kind: ValueKind,
	value: String,
}

impl fmt::Display for Value {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.value)
	}
}

