use std::{
	borrow::Cow,
	fmt,
};

use clang::Entity;
use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::{
	CompiledInterpolation,
	DefaultElement,
	Element,
	EntityElement,
	GeneratedElement,
	GeneratorEnv,
	get_debug,
	IteratorExt,
	settings,
	StrExt,
	type_ref::Kind,
	TypeRef,
	TypeRefTypeHint,
};

#[derive(Clone)]
pub struct Typedef<'tu, 'g> {
	entity: Entity<'tu>,
	gen_env: &'g GeneratorEnv<'tu>,
}

impl<'tu, 'g> Typedef<'tu, 'g> {
	pub fn new(entity: Entity<'tu>, gen_env: &'g GeneratorEnv<'tu>) -> Self {
		Self { entity, gen_env }
	}

	pub fn type_ref(&self) -> TypeRef<'tu, 'g> {
		TypeRef::new(self.entity.get_type().expect("Can't get typedef type"), self.gen_env)
	}

	pub fn underlying_type_ref(&self) -> TypeRef<'tu, 'g> {
		TypeRef::new_ext(
			self.entity.get_typedef_underlying_type().expect("Can't get typedef underlying type"),
			TypeRefTypeHint::None,
			Some(self.entity),
			self.gen_env,
		)
	}

	pub fn dependent_types(&self) -> Vec<Box<dyn GeneratedElement + 'g>> {
		self.underlying_type_ref().dependent_types()
	}
}

impl<'tu> EntityElement<'tu> for Typedef<'tu, '_> {
	fn entity(&self) -> Entity<'tu> {
		self.entity
	}
}

impl Element for Typedef<'_, '_> {
	fn is_excluded(&self) -> bool {
		DefaultElement::is_excluded(self)
			|| self.rust_fullname() == self.underlying_type_ref().rust_full() // fixes recursive typedefs like Cv16suf
			|| settings::PRIMITIVE_TYPEDEFS.contains_key(self.cpp_fullname().as_ref())
	}

	fn is_ignored(&self) -> bool {
		DefaultElement::is_ignored(self) || self.underlying_type_ref().is_ignored()
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
		match self.underlying_type_ref().source().kind() {
			Kind::Class(..) | Kind::Function(..) | Kind::StdVector(..)
			| Kind::SmartPtr(..) => {
				DefaultElement::cpp_localname(self)
			}
			_ => {
				DefaultElement::rust_leafname(self)
			}
		}
	}

	fn rust_localname(&self) -> Cow<str> {
		DefaultElement::rust_localname(self)
	}
}

impl GeneratedElement for Typedef<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module(), self.rust_localname())
	}

	fn gen_rust(&self, opencv_version: &str) -> String {
		static TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/typedef/tpl.rs").compile_interpolation()
		);
		let underlying_type = self.underlying_type_ref();

		let lifetimes = underlying_type.rust_lifetimes();
		let generic_args = if lifetimes.is_empty() {
			"".to_string()
		} else {
			format!("<{}>", lifetimes.into_iter().map(|l| l.to_string()).join(", "))
		};

		TPL.interpolate(&hashmap! {
			"doc_comment" => Cow::Owned(self.rendered_doc_comment(opencv_version)),
			"debug" => get_debug(self).into(),
			"rust_local" => self.rust_localname(),
			"generic_args" => generic_args.into(),
			"definition" => underlying_type.rust_full_ext(true, false),
		})
	}
}

impl fmt::Display for Typedef<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.entity.get_display_name().expect("Can't get display name"))
	}
}

impl fmt::Debug for Typedef<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut debug_struct = f.debug_struct("Typedef");
		self.update_debug_struct(&mut debug_struct)
			.field("export_config", &self.gen_env.get_export_config(self.entity))
			.field("underlying_type_ref", &self.underlying_type_ref())
			.finish()
	}
}
