use std::{
	borrow::Cow,
	fmt,
};

use clang::{Entity, EntityKind, EntityVisitResult, Type};

use crate::{
	Element,
	Field,
	GeneratorEnv,
	IteratorExt,
	type_ref::{ConstnessOverride, FishStyle, NameStyle},
	TypeRef,
};

#[derive(Clone)]
pub struct Function<'tu, 'ge> {
	type_ref: Type<'tu>,
	parent_entity: Entity<'tu>,
	gen_env: &'ge GeneratorEnv<'tu>,
}

impl<'tu, 'ge> Function<'tu, 'ge> {
	pub fn new(type_ref: Type<'tu>, parent_entity: Entity<'tu>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self { type_ref, parent_entity, gen_env }
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

	/// arguments without userdata
	pub fn rust_arguments(&self) -> Vec<Field<'tu, 'ge>> {
		self.arguments().into_iter()
			.filter(|a| !a.is_user_data())
			.collect()
	}

	pub fn has_userdata(&self) -> bool {
		self.arguments().into_iter().any(|f| f.is_user_data())
	}

	pub fn return_type(&self) -> TypeRef<'tu, 'ge> {
		TypeRef::new(self.type_ref.get_result_type().expect("Can't get result type"), self.gen_env)
	}

	pub fn rust_extern(&self) -> Cow<str> {
		let args = self.arguments().into_iter()
			.map(|a| a.type_ref().rust_extern(ConstnessOverride::No).into_owned())
			.join(", ");
		let ret = self.return_type();
		format!(r#"Option<unsafe extern "C" fn({args}) -> {ret}>"#, args=args, ret=ret.rust_extern(ConstnessOverride::No)).into()
	}
}

impl Element for Function<'_, '_> {
	fn is_system(&self) -> bool {
		false
	}

	fn is_public(&self) -> bool {
		true
	}

	fn usr(&self) -> Cow<str> {
		"".into()
	}

	fn rendered_doc_comment_with_prefix(&self, _prefix: &str, _opencv_version: &str) -> String {
		"".to_string()
	}

	fn cpp_namespace(&self) -> Cow<str> {
		"<unset>".into()
	}

	fn cpp_name(&self, _style: NameStyle) -> Cow<str> {
		self.cpp_localname()
	}

	fn cpp_localname(&self) -> Cow<str> {
		let args = self.arguments().into_iter()
			.map(|a| a.type_ref().cpp_full_ext("", false).into_owned())
			.join(", ");
		let ret = self.return_type();
		format!("{ret} (*)({args})", args=args, ret=ret.cpp_full()).into()
	}

	fn rust_module(&self) -> Cow<str> {
		"<unset>".into()
	}

	fn rust_name(&self, _style: NameStyle) -> Cow<str> {
		self.rust_localname(FishStyle::No)
	}

	fn rust_localname(&self, fish_style: FishStyle) -> Cow<str> {
		let ret = self.return_type();
		if self.has_userdata() {
			let args = self.rust_arguments().into_iter()
				.map(|a| a.type_ref().rust_extern(ConstnessOverride::No).into_owned())
				.join(", ");
			format!(
				"Option{fish}<Box{fish}<dyn FnMut({args}) -> {ret} + Send + Sync + 'static>>",
				fish=fish_style.rust_qual(),
				args=args,
				ret=ret.rust_extern(ConstnessOverride::No),
			).into()
		} else {
			self.rust_extern()
		}
	}
}

impl fmt::Display for Function<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.cpp_fullname())
	}
}

impl fmt::Debug for Function<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut debug_struct = f.debug_struct("Function");
		self.update_debug_struct(&mut debug_struct)
			.field("arguments", &self.arguments())
			.finish()
	}
}
