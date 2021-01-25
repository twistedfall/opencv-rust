// todo public static properties like opencv2/core/base.hpp:384 Hamming::normType
// todo test returning reference to array like cv_MatStep_buf

use std::fmt;

use clang::{
	Entity,
	EntityKind,
	EntityVisitResult,
	token::TokenKind,
	Type,
};

use crate::{
	Class,
	constant,
	Cow,
	DefaultElement,
	Element,
	EntityElement,
	GeneratorEnv,
	NamePool,
	TypeRef,
	TypeRefTypeHint,
};

#[derive(Clone, Copy, Debug)]
pub enum FieldTypeHint<'tu> {
	None,
	Slice,
	NullableSlice,
	SliceLen(&'static str, usize),
	FieldSetter,
	Specialized(Type<'tu>),
}

impl Default for FieldTypeHint<'_> {
	fn default() -> Self {
		FieldTypeHint::None
	}
}

#[derive(Clone)]
pub struct Field<'tu> {
	entity: Entity<'tu>,
	type_hint: FieldTypeHint<'tu>,
	gen_env: &'tu GeneratorEnv<'tu>,
}

impl<'tu> Field<'tu> {
	pub fn new(entity: impl Into<Entity<'tu>>, gen_env: &'tu GeneratorEnv<'tu>) -> Self {
		Self::new_ext(entity, Default::default(), gen_env)
	}

	pub fn new_ext(entity: impl Into<Entity<'tu>>, type_hint: FieldTypeHint<'tu>, gen_env: &'tu GeneratorEnv<'tu>) -> Self {
		Self { entity: entity.into(), type_hint, gen_env }
	}

	pub fn rust_disambiguate_names<I: IntoIterator<Item=Field<'tu>>>(args: I) -> impl Iterator<Item=(String, Field<'tu>)> where I::IntoIter: 'tu {
		let args = args.into_iter();
		NamePool::with_capacity(args.size_hint().1.unwrap_or_default())
			.into_disambiguator(args, |f| f.rust_leafname())
	}

	pub fn cpp_disambiguate_names(args: impl IntoIterator<Item=Field<'tu>>) -> impl Iterator<Item=(String, Field<'tu>)> {
		let args = args.into_iter();
		NamePool::with_capacity(args.size_hint().1.unwrap_or_default())
			.into_disambiguator(args, |f| f.cpp_localname())
	}

	pub fn type_ref(&self) -> TypeRef<'tu> {
		let type_hint = match self.type_hint {
			FieldTypeHint::Slice => TypeRefTypeHint::Slice,
			FieldTypeHint::NullableSlice => TypeRefTypeHint::NullableSlice,
			FieldTypeHint::Specialized(typ) => TypeRefTypeHint::Specialized(typ),
			_ => TypeRefTypeHint::None,
		};
		TypeRef::new_ext(self.entity.get_type().expect("Can't get type"), type_hint, Some(self.entity), self.gen_env)
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
			let equal_pos = tokens.iter()
				.position(|t| t.get_kind() == TokenKind::Punctuation && t.get_spelling() == "=");
			if let Some(equal_pos) = equal_pos {
				tokens.drain(..equal_pos + 1);
				return Some(constant::render_constant_cpp(tokens));
			}
		}
		None
	}

	pub fn parent(&self) -> Class<'tu> {
		let parent_entity = self.entity.get_semantic_parent().expect("Can't get parent of field");
		match parent_entity.get_kind() {
			EntityKind::ClassDecl | EntityKind::StructDecl | EntityKind::ClassTemplate => {
				Class::new(parent_entity, self.gen_env)
			},
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
		let leafname = self.cpp_localname();
		(leafname == "userdata" || leafname == "userData" || leafname == "cookie" || leafname == "unnamed")
			&& type_ref.as_pointer().map_or(false, |inner| inner.is_void())
	}

	pub fn as_slice_len(&self) -> Option<(&'static str, usize)> {
		if let FieldTypeHint::SliceLen(ptr_arg, len_div) = self.type_hint {
			Some((ptr_arg, len_div))
		} else {
			None
		}
	}
}

impl<'tu> EntityElement<'tu> for Field<'tu> {
	fn entity(&self) -> Entity<'tu> {
		self.entity
	}
}

impl Element for Field<'_> {
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

	fn rendered_doc_comment_with_prefix(&self, prefix: &str, opencv_version: &str) -> String {
		DefaultElement::rendered_doc_comment_with_prefix(self, prefix, opencv_version)
	}

	fn cpp_namespace(&self) -> Cow<str> {
		DefaultElement::cpp_namespace(self)
	}

	fn cpp_localname(&self) -> Cow<str> {
		if matches!(self.type_hint, FieldTypeHint::FieldSetter) {
			"val".into()
		} else {
			DefaultElement::cpp_localname(self)
		}
	}

	fn rust_module(&self) -> Cow<str> {
		DefaultElement::rust_module(self)
	}

	fn rust_leafname(&self) -> Cow<str> {
		if matches!(self.type_hint, FieldTypeHint::FieldSetter) {
			"val".into()
		} else {
			DefaultElement::rust_leafname(self)
		}
	}

	fn rust_localname(&self) -> Cow<str> {
		DefaultElement::rust_localname(self)
	}
}

impl fmt::Display for Field<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.entity.get_display_name().expect("Can't get display name"))
	}
}

impl fmt::Debug for Field<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Field")
			.field("rust_name", &self.rust_localname())
			.field("type_hint", &self.type_hint)
			.field("type_ref", &self.type_ref())
			.field("default_value", &self.default_value())
			.finish()
	}
}
