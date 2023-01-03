use std::borrow::Cow;
use std::collections::HashSet;
use std::{fmt, hash, iter};

use clang::Entity;

use crate::entity::{WalkAction, WalkResult};
use crate::type_ref::{Constness, CppNameStyle};
use crate::{
	settings, Const, DefaultElement, Element, EntityElement, EntityExt, Field, Func, FunctionTypeHint, GeneratedType,
	GeneratorEnv, StrExt, TypeRef,
};

#[derive(Clone, Copy, Debug)]
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
	pub(crate) gen_env: &'ge GeneratorEnv<'tu>,
}

impl<'tu, 'ge> Class<'tu, 'ge> {
	pub fn new(entity: Entity<'tu>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self {
			entity,
			custom_fullname: None,
			gen_env,
		}
	}

	pub fn new_ext(entity: Entity<'tu>, custom_fullname: String, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self {
			entity,
			custom_fullname: Some(custom_fullname),
			gen_env,
		}
	}

	pub fn can_be_simple(&self) -> bool {
		let cpp_refname = self.cpp_name(CppNameStyle::Reference);
		if settings::IMPLEMENTED_GENERICS.contains(cpp_refname.as_ref())
			|| settings::IMPLEMENTED_CONST_GENERICS.contains(cpp_refname.as_ref())
		{
			return true;
		}
		self.has_fields()
			&& !self.has_descendants()
			&& !self.has_bases()
			&& !self
				.for_each_field(|field| WalkAction::continue_until(!field.type_ref().is_copy()))
				.is_interrupted()
	}

	pub fn kind(&self) -> Kind {
		if settings::ELEMENT_EXCLUDE.contains(self.cpp_name(CppNameStyle::Reference).as_ref()) {
			return Kind::Excluded;
		}
		match self.gen_env.get_export_config(self.entity).map(|c| c.simple) {
			Some(true) => {
				if self.can_be_simple() {
					Kind::Simple
				} else {
					Kind::Boxed
				}
			}
			Some(false) => Kind::Boxed,
			None => {
				if self.is_system() {
					Kind::System
				} else if let Some(kind) = self.gen_env.get_class_kind(self.entity) {
					match kind {
						Kind::Simple if !self.can_be_simple() => Kind::Boxed,
						_ => kind,
					}
				} else {
					Kind::Excluded
				}
			}
		}
	}

	pub fn type_ref(&self) -> TypeRef<'tu, 'ge> {
		TypeRef::new(self.entity.get_type().expect("Can't get class type"), self.gen_env)
	}

	pub fn as_template_specialization(&self) -> Option<Class<'tu, 'ge>> {
		self.entity.get_template().map(|t| Class::new(t, self.gen_env))
	}

	pub fn is_simple(&self) -> bool {
		matches!(self.kind(), Kind::Simple)
	}

	pub fn is_boxed(&self) -> bool {
		matches!(self.kind(), Kind::Boxed)
	}

	pub fn is_template(&self) -> bool {
		self.entity.get_template_kind().is_some()
	}

	/// This class is a specific instance of a template class, e.g. Point_<int>
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
		self
			.entity
			.walk_methods_while(|f| WalkAction::continue_until(f.is_virtual_method()))
			.is_interrupted()
	}

	pub fn is_trait(&self) -> bool {
		self.is_boxed()
		//		self.is_abstract() || self.has_descendants() || settings::FORCE_CLASS_TRAIT.contains(self.cpp_name(CppNameStyle::Reference).as_ref())
	}

	pub fn has_clone(&self) -> bool {
		self
			.for_each_method(|m| WalkAction::continue_until(m.is_clone()))
			.is_interrupted()
	}

	pub fn has_bases(&self) -> bool {
		self.entity.walk_bases_while(|_| WalkAction::Interrupt).is_interrupted()
	}

	pub fn bases(&self) -> Vec<Class<'tu, 'ge>> {
		let mut out = vec![];
		let entity = if let Some(entity) = self.entity.get_template() {
			entity
		} else {
			self.entity
		};
		entity.walk_bases_while(|child| {
			out.push(Class::new(Self::definition_entity(child), self.gen_env));
			WalkAction::Continue
		});
		out
	}

	pub fn all_bases(&self) -> HashSet<Class<'tu, 'ge>> {
		self
			.bases()
			.into_iter()
			.flat_map(|b| {
				let mut out = b.all_bases();
				out.insert(b);
				out
			})
			.collect()
	}

	pub fn has_descendants(&self) -> bool {
		self
			.gen_env
			.descendants
			.contains_key(self.cpp_name(CppNameStyle::Reference).as_ref())
	}

	pub fn descendants(&self) -> impl Iterator<Item = Class<'tu, 'ge>> {
		let gen_env = self.gen_env;
		gen_env
			.descendants
			.get(self.cpp_name(CppNameStyle::Reference).as_ref())
			.into_iter()
			.flat_map(move |desc| desc.iter().map(move |e| Class::new(*e, gen_env)))
	}

	pub fn has_methods(&self) -> bool {
		self.entity.walk_methods_while(|_| WalkAction::Interrupt).is_interrupted()
	}

	#[inline]
	pub fn for_each_method(&self, mut predicate: impl FnMut(Func<'tu, 'ge>) -> WalkAction) -> WalkResult {
		self.entity.walk_methods_while(|f| predicate(Func::new(f, self.gen_env)))
	}

	pub fn methods(&self, constness_filter: Option<Constness>) -> Vec<Func<'tu, 'ge>> {
		let mut out = Vec::with_capacity(64);
		self.for_each_method(|func| {
			if constness_filter.map_or(true, |c| c == func.constness()) {
				if func.is_generic() {
					if let Some(specs) = settings::FUNC_SPECIALIZE.get(func.identifier().as_ref()) {
						for spec in specs {
							out.push(Func::new_ext(
								func.entity(),
								FunctionTypeHint::Specialized(spec),
								None,
								self.gen_env,
							));
						}
						return WalkAction::Continue;
					}
				}
				out.push(func);
			}
			WalkAction::Continue
		});
		out
	}

	pub fn has_fields(&self) -> bool {
		Self::definition_entity(self.entity)
			.walk_fields_while(|_| WalkAction::Interrupt)
			.is_interrupted()
	}

	#[inline]
	pub fn for_each_field(&self, mut predicate: impl FnMut(Field<'tu, 'ge>) -> WalkAction) -> WalkResult {
		self.entity.walk_fields_while(|f| predicate(Field::new(f, self.gen_env)))
	}

	pub fn fields(&self) -> Vec<Field<'tu, 'ge>> {
		let mut out = Vec::with_capacity(32);
		self.for_each_field(|f| {
			out.push(f);
			WalkAction::Continue
		});
		out
	}

	pub fn consts(&self) -> Vec<Const<'tu>> {
		let mut out = Vec::with_capacity(8);
		self.entity.walk_consts_while(|child| {
			out.push(Const::new(child));
			WalkAction::Continue
		});
		out
	}

	pub fn field_methods<'f>(
		&self,
		fields: impl Iterator<Item = &'f Field<'tu, 'ge>>,
		constness_filter: Option<Constness>,
	) -> Vec<Func<'tu, 'ge>>
	where
		'tu: 'f,
		'ge: 'f,
	{
		let mut out = Vec::with_capacity(fields.size_hint().1.map_or(8, |x| x * 2));
		out.extend(fields.flat_map(|fld| {
			iter::from_fn({
				let fld_type_ref = fld.type_ref();
				let read_func = Func::new(fld.entity(), self.gen_env);
				let mut read_yield = if constness_filter.map_or(true, |c| c == read_func.constness()) {
					Some(read_func)
				} else {
					None
				};
				let mut write_yield = if constness_filter.map_or(true, |c| c.is_mut())
					&& !fld_type_ref.constness().is_const()
					&& !fld_type_ref.as_fixed_array().is_some()
				{
					Some(Func::new_ext(fld.entity(), FunctionTypeHint::FieldSetter, None, self.gen_env))
				} else {
					None
				};
				move || read_yield.take().or_else(|| write_yield.take())
			})
		}));
		out
	}

	/// Returns an entity that defines current class, for specialized classes (Point_<int>) it's the template (Point_<T>), for
	/// not fully defined classes it goes to its definition location.
	fn definition_entity(entity: Entity<'tu>) -> Entity<'tu> {
		entity.get_template().unwrap_or(entity).get_definition().unwrap_or(entity)
	}

	pub fn is_definition(&self) -> bool {
		let class_loc = self.entity.get_location();
		let def_loc = self.entity.get_definition().and_then(|d| d.get_location());
		match (class_loc, def_loc) {
			(Some(class_loc), Some(def_loc)) => class_loc == def_loc,
			(_, None) => false,
			_ => true,
		}
	}

	pub fn generated_types(&self) -> Vec<GeneratedType<'tu, 'ge>> {
		self
			.fields()
			.into_iter()
			.filter(|f| !f.is_excluded())
			.flat_map(|f| f.type_ref().generated_types())
			.chain(
				self
					.methods(None)
					.into_iter()
					.filter(|m| !m.is_excluded())
					.flat_map(|m| m.generated_types()),
			)
			.collect()
	}
}

impl<'tu> EntityElement<'tu> for Class<'tu, '_> {
	fn entity(&self) -> Entity<'tu> {
		self.entity
	}
}

impl Element for Class<'_, '_> {
	fn is_excluded(&self) -> bool {
		DefaultElement::is_excluded(self) || matches!(self.kind(), Kind::Excluded) || self.cpp_namespace() == ""
		// we don't process out of namespace (legacy C) items, so mark them as excluded
	}

	fn is_ignored(&self) -> bool {
		DefaultElement::is_ignored(self)
			|| self.is_template()
			|| self.is_template_specialization() && {
				let cpp_refname = self.cpp_name(CppNameStyle::Reference);
				!settings::IMPLEMENTED_GENERICS.contains(cpp_refname.as_ref())
					&& !settings::IMPLEMENTED_CONST_GENERICS.contains(cpp_refname.as_ref())
			} || !self.is_template_specialization() && !self.is_definition()
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

	fn cpp_namespace(&self) -> Cow<str> {
		if let Some(custom_fullname) = &self.custom_fullname {
			custom_fullname.namespace().into()
		} else {
			DefaultElement::cpp_namespace(self).into()
		}
	}

	fn cpp_name(&self, style: CppNameStyle) -> Cow<str> {
		if let Some(custom_fullname) = &self.custom_fullname {
			match style {
				CppNameStyle::Declaration => custom_fullname.localname().into(),
				CppNameStyle::Reference => custom_fullname.into(),
			}
		} else {
			DefaultElement::cpp_name(self, style)
		}
	}
}

impl hash::Hash for Class<'_, '_> {
	fn hash<H: hash::Hasher>(&self, state: &mut H) {
		self.entity.hash(state)
	}
}

impl PartialEq for Class<'_, '_> {
	fn eq(&self, other: &Self) -> bool {
		self.entity.eq(&other.entity)
	}
}

impl Eq for Class<'_, '_> {}

impl fmt::Display for Class<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.entity.get_display_name().expect("Can't get display name"))
	}
}

impl fmt::Debug for Class<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut props = vec![];
		if self.is_boxed() {
			props.push("boxed");
		}
		if self.is_simple() {
			props.push("simple");
		}
		if self.is_template() {
			props.push("template");
		}
		if self.is_template_specialization() {
			props.push("template_specialization");
		}
		if self.is_abstract() {
			props.push("abstract");
		}
		if self.is_polymorphic() {
			props.push("polymorphic");
		}
		if self.is_trait() {
			props.push("trait");
		}
		if self.has_clone() {
			props.push("has_clone");
		}
		if self.has_bases() {
			props.push("has_bases");
		}
		if self.has_descendants() {
			props.push("has_descendants");
		}
		if self.has_methods() {
			props.push("has_methods");
		}
		if self.has_fields() {
			props.push("has_fields");
		}
		let mut debug_struct = f.debug_struct("Class");
		self.update_debug_struct(&mut debug_struct)
			.field("export_config", &self.gen_env.get_export_config(self.entity))
			.field("kind", &self.kind())
			.field("props", &props.join(", "))
			.field("as_template", &self.as_template_specialization())
//			.field("type_ref", &self.type_ref())
//			.field("bases", &self.bases())
//			.field("methods", &self.methods())
//			.field("fields", &self.fields())
			.finish()
	}
}
