use std::{
	borrow::Cow,
	cmp,
	collections::HashSet,
	fmt,
	hash,
	iter,
};

use clang::Entity;

use crate::{
	Const,
	Constness,
	DefaultElement,
	DefinitionLocation,
	DependentType,
	DependentTypeMode,
	Element,
	EntityElement,
	EntityExt,
	Field,
	Func,
	FunctionTypeHint,
	GeneratorEnv,
	settings,
	StrExt,
	type_ref::{FishStyle, NameStyle},
	TypeRef,
};
use crate::return_type_wrapper::ReturnTypeWrapper;

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
pub enum Kind {
	Simple,
	Boxed,
	System,
	Excluded,
}

#[derive(Clone)]
pub struct Class<'tu, 'ge> {
	entity: Entity<'tu>,
	custom_fullname: Option<String>,
	gen_env: &'ge GeneratorEnv<'tu>,
}

impl<'tu, 'ge> Class<'tu, 'ge> {
	pub fn new(entity: Entity<'tu>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self { entity, custom_fullname: None, gen_env }
	}

	pub fn new_ext(entity: Entity<'tu>, custom_fullname: String, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self { entity, custom_fullname: Some(custom_fullname), gen_env }
	}

	pub fn kind(&self) -> Kind {
		let cpp_fullname = self.cpp_fullname();
		if settings::ELEMENT_EXCLUDE.contains(cpp_fullname.as_ref()) {
			return Kind::Excluded
		}
		match self.gen_env.get_export_config(self.entity).map(|c| c.simple) {
			Some(true) => {
				let has_non_copy_fields = self.entity.walk_fields_while(|f| {
					let type_ref = Field::new(f, self.gen_env).type_ref();
					let non_copy_field = type_ref.as_string().is_some()
						|| type_ref.as_vector().is_some()
						|| type_ref.as_class().map_or(false, |c| !c.is_simple())
						|| self.gen_env.descendants.contains_key(cpp_fullname.as_ref());
					!non_copy_field
				});
				if has_non_copy_fields {
					Kind::Boxed
				} else {
					Kind::Simple
				}
			},
			Some(false) => Kind::Boxed,
			None => {
				if self.is_system() {
					Kind::System
				} else if let Some(kind) = self.gen_env.get_class_kind(self.entity) {
					kind
				} else {
					Kind::Excluded
				}
			}
		}
	}

	pub fn type_ref(&self) -> TypeRef<'tu, 'ge> {
		TypeRef::new(self.entity.get_type().expect("Can't get class type"), self.gen_env)
	}

	pub fn detect_class_simplicity(&self) -> bool {
		!self.has_bases()
			&& self.fields().into_iter()
			.map(|f| f.type_ref())
			.all(|t| t.is_copy())
	}

	pub fn as_template(&self) -> Option<Class<'tu, 'ge>> {
		self.entity.get_template()
			.map(|t| Class::new(t, self.gen_env))
	}

	pub fn is_simple(&self) -> bool {
		self.kind() == Kind::Simple
	}

	pub fn is_boxed(&self) -> bool {
		self.kind() == Kind::Boxed
	}

	pub fn is_template(&self) -> bool {
		self.entity.get_template_kind().is_some()
	}

	pub fn is_template_specialization(&self) -> bool {
		self.entity.get_template().is_some()
	}

	pub fn is_abstract(&self) -> bool {
		self.entity.is_abstract_record()
		// fixme, or maybe we want?
		// is_abstract_record() also check parent classes for presence of pure virtual methods, we don't want that
//		self.entity.walk_methods_while(|child| !Func::new(child, self.gen_env).is_abstract())
	}

	pub fn is_polymorphic(&self) -> bool {
		self.entity.walk_methods_while(|f| !f.is_virtual_method())
	}

	pub fn is_trait(&self) -> bool {
		self.is_boxed()
//		self.is_abstract() || self.has_descendants() || settings::FORCE_CLASS_TRAIT.contains(self.cpp_fullname().as_ref())
	}

	pub fn is_by_ptr(&self) -> bool {
		match self.kind() {
			Kind::Boxed => true,
			Kind::Simple | Kind::System | Kind::Excluded => false,
		}
	}

	pub fn has_clone(&self) -> bool {
		self.for_each_method(|m| !m.is_clone())
	}

	pub fn rust_trait_name(&self, style: NameStyle, constness: Constness) -> Cow<str> {
		let mut out = self.rust_name(style);
		if self.is_trait() && !self.is_abstract() {
			out.to_mut().push_str("Trait");
		}
		if constness.is_const() {
			out.to_mut().push_str("Const");
		}
		out
	}

	pub fn has_bases(&self) -> bool {
		self.entity.walk_bases_while(|_| false)
	}

	pub fn bases(&self) -> Vec<Class<'tu, 'ge>> {
		let mut out = vec![];
		let entity = if let Some(entity) = self.entity.get_template() {
			entity
		} else {
			self.entity
		};
		entity.walk_bases_while(|child| {
			out.push(Class::new(child.get_definition().expect("Can't get base class definition"), self.gen_env));
			true
		});
		out
	}

	pub fn all_bases(&self) -> HashSet<Class<'tu, 'ge>> {
		self.bases().into_iter()
			.flat_map(|b| {
				let mut out = b.all_bases();
				out.insert(b);
				out
			})
			.collect()
	}

	pub fn descendants(&self) -> impl Iterator<Item=Class<'tu, 'ge>> {
		let gen_env = self.gen_env;
		gen_env.descendants.get(self.cpp_fullname().as_ref()).into_iter()
			.flat_map(move |desc| desc.iter().map(move |e| Class::new(*e, gen_env)))
	}

	pub fn has_methods(&self) -> bool {
		self.entity.walk_methods_while(|_| false)
	}

	#[inline]
	pub fn for_each_method(&self, mut predicate: impl FnMut(Func<'tu, 'ge>) -> bool) -> bool {
		self.entity.walk_methods_while(|f| predicate(Func::new(f, self.gen_env)))
	}

	pub fn methods(&self, constness_filter: Option<Constness>) -> Vec<Func<'tu, 'ge>> {
		let mut out = Vec::with_capacity(64);
		self.for_each_method(|func| {
			if constness_filter.map_or(true, |c| c == func.constness()) {
				if func.is_generic() {
					if let Some(specs) = settings::FUNC_SPECIALIZE.get(func.identifier().as_ref()) {
						for spec in specs {
							out.push(Func::new_ext(func.entity(), FunctionTypeHint::Specialized(spec), None, self.gen_env));
						}
						return true;
					}
				}
				out.push(func);
			}
			true
		});
		out
	}

	pub fn has_fields(&self) -> bool {
		self.entity.walk_fields_while(|_| false)
	}

	#[inline]
	pub fn for_each_field(&self, mut predicate: impl FnMut(Field<'tu, 'ge>) -> bool) -> bool {
		self.entity.walk_fields_while(|f| predicate(Field::new(f, self.gen_env)))
	}

	pub fn fields(&self) -> Vec<Field<'tu, 'ge>> {
		let mut out = Vec::with_capacity(32);
		self.for_each_field(|f| {
			out.push(f);
			true
		});
		out
	}

	pub fn consts(&self) -> Vec<Const<'tu>> {
		let mut out = Vec::with_capacity(8);
		self.entity.walk_consts_while(|child| {
			out.push(Const::new(child));
			true
		});
		out
	}

	pub fn field_methods<'f>(&self, fields: impl Iterator<Item=&'f Field<'tu, 'ge>>, constness_filter: Option<Constness>) -> Vec<Func<'tu, 'ge>> where 'tu: 'f, 'ge: 'f {
		let mut out = Vec::with_capacity(fields.size_hint().1.map_or(8, |x| x * 2));
		out.extend(fields
			.flat_map(|fld| {
				iter::from_fn({
					let fld_type_ref = fld.type_ref();
					let read_func = Func::new(fld.entity(), self.gen_env);
					let mut read_yield = if constness_filter.map_or(true, |c| c == read_func.constness()) {
						Some(read_func)
					} else {
						None
					};
					let mut write_yield = if constness_filter.map_or(true, |c| c.is_mut()) && !fld_type_ref.constness().is_const() && !fld_type_ref.as_fixed_array().is_some() {
						Some(Func::new_ext(fld.entity(), FunctionTypeHint::FieldSetter, None, self.gen_env))
					} else {
						None
					};
					move || read_yield.take().or_else(|| write_yield.take())
				})
			})
		);
		out
	}

	pub fn is_definition(&self) -> bool {
		let class_loc = self.entity.get_location();
		let def_loc = self.entity.get_definition().and_then(|d| d.get_location());
		match (class_loc, def_loc) {
			(Some(class_loc), Some(def_loc)) => {
				class_loc == def_loc
			}
			(_, None) => {
				false
			}
			_ => {
				true
			}
		}
	}

	pub fn dependent_types(&self) -> Vec<DependentType<'tu, 'ge>> {
		let dep_types = self.fields().into_iter()
			.filter(|f| !f.is_excluded())
			.flat_map(|f| f.type_ref().dependent_types(DependentTypeMode::ForReturn(DefinitionLocation::Module)))
			.chain(self.methods(None).into_iter()
				.filter(|m| !m.is_excluded())
				.flat_map(|m| m.dependent_types())
			);
		if !self.is_simple() {
			dep_types.chain(self.descendants().into_iter()
				.filter(|b| !b.is_excluded() && !b.is_simple() && !b.is_abstract())
				.map(|b| DependentType::from_return_type_wrapper(ReturnTypeWrapper::new(
					b.type_ref(),
					DefinitionLocation::Type,
					self.gen_env,
				)))
			)
				.collect()
		} else {
			dep_types.collect()
		}
	}
}

impl<'tu> EntityElement<'tu> for Class<'tu, '_> {
	fn entity(&self) -> Entity<'tu> {
		self.entity
	}
}

impl Element for Class<'_, '_> {
	fn is_excluded(&self) -> bool {
		DefaultElement::is_excluded(self)
			|| matches!(self.kind(), Kind::Excluded)
			|| self.cpp_namespace() == "" // we don't process out of namespace (legacy C) items, so mark them as excluded
	}

	fn is_ignored(&self) -> bool {
		DefaultElement::is_ignored(self)
			|| self.is_template()
			|| self.is_template_specialization() &&
			{
				let cpp_fullname = self.cpp_fullname();
				!settings::IMPLEMENTED_GENERICS.contains(cpp_fullname.as_ref())
					&& !settings::IMPLEMENTED_CONST_GENERICS.contains(cpp_fullname.as_ref())
			}
			|| !self.is_template_specialization() && !self.is_definition()
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
		if self.custom_fullname.is_some() {
			self.cpp_fullname().namespace().to_string().into()
		} else {
			DefaultElement::cpp_namespace(self).into()
		}
	}

	fn cpp_localname(&self) -> Cow<str> {
		if self.custom_fullname.is_some() {
			self.cpp_fullname().localname().to_string().into()
		} else {
			DefaultElement::cpp_localname(self)
		}
	}

	fn cpp_fullname(&self) -> Cow<str> {
		if let Some(custom_fullname) = &self.custom_fullname {
			custom_fullname.into()
		} else {
			DefaultElement::cpp_fullname(self)
		}
	}

	fn rust_module(&self) -> Cow<str> {
		DefaultElement::rust_module(self)
	}

	fn rust_leafname(&self, _fish_style: FishStyle) -> Cow<str> {
		if self.type_ref().as_string().is_some() {
			"String".into()
		} else {
			let cpp_localname = self.cpp_localname();
			if cpp_localname == "Vec" {
				"VecN".into()
			} else {
				cpp_localname
			}
		}
	}

	fn rust_localname(&self, fish_style: FishStyle) -> Cow<str> {
		DefaultElement::rust_localname(self, fish_style)
	}
}

impl hash::Hash for Class<'_, '_> {
	fn hash<H: hash::Hasher>(&self, state: &mut H) {
		self.entity.hash(state)
	}
}

impl cmp::PartialEq for Class<'_, '_> {
	fn eq(&self, other: &Self) -> bool {
		self.entity.eq(&other.entity)
	}
}

impl cmp::Eq for Class<'_, '_> {}

impl fmt::Display for Class<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.entity.get_display_name().expect("Can't get display name"))
	}
}

impl fmt::Debug for Class<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut props = vec![];
		if self.is_template() {
			props.push("template");
		}
		if self.is_template_specialization() {
			props.push("template_specialization");
		}
		if self.is_abstract() {
			props.push("abstract");
		}
		if self.is_trait() {
			props.push("trait");
		}
		if self.is_polymorphic() {
			props.push("polymorphic");
		}
		if self.is_by_ptr() {
			props.push("by_ptr");
		}
		if self.is_simple() {
			props.push("simple");
		}
		if self.has_methods() {
			props.push("has_methods");
		}
		if self.has_fields() {
			props.push("has_fields");
		}
		if self.has_bases() {
			props.push("has_bases");
		}
		let mut debug_struct = f.debug_struct("Class");
		self.update_debug_struct(&mut debug_struct)
			.field("export_config", &self.gen_env.get_export_config(self.entity))
			.field("kind", &self.kind())
			.field("props", &props.join(", "))
			.field("as_template", &self.as_template())
//			.field("type_ref", &self.type_ref())
//			.field("bases", &self.bases())
//			.field("methods", &self.methods())
//			.field("fields", &self.fields())
			.finish()
	}
}
