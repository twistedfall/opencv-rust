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
	DefinitionLocation,
	DependentTypeMode,
	Element,
	EntityElement,
	GeneratedElement,
	GeneratorEnv,
	StrExt,
	TypeRef,
};

#[derive(Clone)]
pub struct SmartPtr<'tu, 'g> {
	entity: Entity<'tu>,
	element: TypeRef<'tu, 'g>,
	gen_env: &'g GeneratorEnv<'tu>,
}

impl<'tu, 'g> SmartPtr<'tu, 'g> {
	pub fn new(entity: Entity<'tu>, element: TypeRef<'tu, 'g>, gen_env: &'g GeneratorEnv<'tu>) -> Self {
		Self { entity, element, gen_env }
	}

	pub fn type_ref(&self) -> TypeRef<'tu, 'g> {
		TypeRef::new(self.entity.get_type().expect("Can't get smartptr type"), self.gen_env)
	}

	pub fn pointee(&self) -> TypeRef<'tu, 'g> {
		self.element.clone()
	}

	pub fn canonical(&self) -> Self {
		Self::new(self.entity, self.element.canonical(), self.gen_env)
	}

	pub fn dependent_types(&self) -> Vec<Box<dyn GeneratedElement + 'g>> {
		self.element.dependent_types_with_mode(DependentTypeMode::ForReturn(DefinitionLocation::Module))
	}
}

impl<'tu> EntityElement<'tu> for SmartPtr<'tu, '_> {
	fn entity(&self) -> Entity<'tu> {
		self.entity
	}
}

impl Element for SmartPtr<'_, '_> {
	fn is_ignored(&self) -> bool {
		DefaultElement::is_ignored(self) || self.pointee().is_ignored()
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
		self.element.rust_module()
	}

	fn rust_leafname(&self) -> Cow<str> {
		format!("PtrOf{typ}", typ=self.element.rust_safe_id()).into()
	}

	fn rust_localname(&self) -> Cow<str> {
		DefaultElement::rust_localname(self)
	}

	fn rust_fullname(&self) -> Cow<str> {
		format!("types::{}", self.rust_localname()).into()
	}
}

impl GeneratedElement for SmartPtr<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module(), self.rust_localname())
	}

	fn gen_rust(&self, _opencv_version: &str) -> String {
		static COMMON_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/smart_ptr/rust.tpl.rs").compile_interpolation()
		);

		static TRAIT_CAST_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/smart_ptr/trait_cast.tpl.rs").compile_interpolation()
		);

		static METHODS_NON_TRAIT_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/smart_ptr/methods_non_trait.tpl.rs").compile_interpolation()
		);

		let type_ref = self.type_ref();
		let pointee_type = self.pointee();

		let mut inter_vars = hashmap! {
			"rust_local" => self.rust_localname(),
			"rust_extern" => type_ref.rust_extern(),
		};

		let mut impls = String::new();
		let mut methods = String::new();
		if let Some(cls) = pointee_type.as_class() {
			if cls.is_trait() {
				let mut all_bases = cls.all_bases();
				all_bases.insert(cls);
				let mut all_bases = all_bases.into_iter()
					.filter(|b| !b.is_excluded())
					.collect::<Vec<_>>();
				all_bases.sort_unstable_by(|a, b| a.cpp_localname().cmp(&b.cpp_localname()));
				for base in all_bases {
					let mut vars = inter_vars.clone();
					vars.insert("base_rust_local", base.rust_localname());
					vars.insert("base_rust_full", base.rust_trait_fullname());
					impls += &TRAIT_CAST_TPL.interpolate(&vars);
				}
			} else if cls.is_boxed() {
				let mut vars = inter_vars.clone();
				vars.insert("rust_full", type_ref.rust_full());
				vars.insert("inner_rust_local", pointee_type.rust_local());
				vars.insert("inner_rust_full", pointee_type.rust_full());
				methods += &METHODS_NON_TRAIT_TPL.interpolate(&vars);
			}
		};
		inter_vars.insert("methods", methods.into());
		inter_vars.insert("impls", impls.into());

		COMMON_TPL.interpolate(&inter_vars)
	}

	fn gen_cpp(&self) -> String {
		static TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/smart_ptr/cpp.tpl.cpp").compile_interpolation()
		);

		static BOXED_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/smart_ptr/cpp_boxed.tpl.cpp").compile_interpolation()
		);

		let type_ref = self.type_ref();
		let pointee_type = self.pointee();

		if pointee_type.is_by_ptr() {
			&BOXED_TPL
		} else {
			&TPL
		}.interpolate(&hashmap! {
			"rust_local" => self.rust_localname(),
			"cpp_extern" => type_ref.cpp_extern(),
			"cpp_full" => type_ref.cpp_full(),
			"inner_cpp_extern" => pointee_type.cpp_extern_return(),
		})
	}
}

impl fmt::Display for SmartPtr<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.entity.get_display_name().expect("Can't get display name"))
	}
}

impl fmt::Debug for SmartPtr<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut debug_struct = f.debug_struct("SmartPtr");
		self.update_debug_struct(&mut debug_struct)
			.field("export_config", &self.gen_env.get_export_config(self.entity))
			.field("pointee", &self.pointee())
			.finish()
	}
}
