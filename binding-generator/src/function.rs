use std::borrow::Cow;
use std::fmt;

use clang::{Entity, EntityKind, EntityVisitResult, Type};

use crate::type_ref::CppNameStyle;
use crate::{Element, Field, GeneratorEnv, IteratorExt, TypeRef};

#[derive(Clone)]
pub struct Function<'tu, 'ge> {
	type_ref: Type<'tu>,
	parent_entity: Entity<'tu>,
	gen_env: &'ge GeneratorEnv<'tu>,
}

impl<'tu, 'ge> Function<'tu, 'ge> {
	pub fn new(type_ref: Type<'tu>, parent_entity: Entity<'tu>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self {
			type_ref,
			parent_entity,
			gen_env,
		}
	}

	pub fn arguments(&self) -> Vec<Field<'tu, 'ge>> {
		let mut out = Vec::with_capacity(10);
		self.parent_entity.visit_children(|c, _| {
			if c.get_kind() == EntityKind::ParmDecl {
				out.push(Field::new(c, self.gen_env));
			}
			EntityVisitResult::Continue
		});
		out
	}

	pub fn has_userdata(&self) -> bool {
		self.arguments().into_iter().any(|f| f.is_user_data())
	}

	pub fn return_type(&self) -> TypeRef<'tu, 'ge> {
		TypeRef::new(self.type_ref.get_result_type().expect("Can't get result type"), self.gen_env)
	}
}

impl Element for Function<'_, '_> {
	fn is_system(&self) -> bool {
		false
	}

	fn is_public(&self) -> bool {
		true
	}

	fn doc_comment(&self) -> Cow<str> {
		"".into()
	}

	fn cpp_namespace(&self) -> Cow<str> {
		"<unset>".into()
	}

	fn cpp_name(&self, _style: CppNameStyle) -> Cow<str> {
		let args = self
			.arguments()
			.into_iter()
			.map(|a| a.type_ref().cpp_name_ext(CppNameStyle::Reference, "", false).into_owned())
			.join(", ");
		let ret = self.return_type();
		format!("{ret} (*)({args})", args = args, ret = ret.cpp_name(CppNameStyle::Reference)).into()
	}
}

impl PartialEq for Function<'_, '_> {
	fn eq(&self, other: &Self) -> bool {
		self.type_ref == other.type_ref && self.parent_entity == other.parent_entity
	}
}

impl fmt::Display for Function<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.cpp_name(CppNameStyle::Reference))
	}
}

impl fmt::Debug for Function<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut debug_struct = f.debug_struct("Function");
		self
			.update_debug_struct(&mut debug_struct)
			.field("arguments", &self.arguments())
			.finish()
	}
}
