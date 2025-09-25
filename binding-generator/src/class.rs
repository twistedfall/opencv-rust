use std::borrow::Cow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::ControlFlow;
use std::rc::Rc;
use std::{fmt, iter};

use clang::{Accessibility, Entity, EntityKind};
pub use desc::ClassDesc;

use crate::debug::{DefinitionLocation, LocationName};
use crate::element::ExcludeKind;
use crate::entity::{ControlFlowExt, ToEntity};
use crate::field::FieldDesc;
use crate::func::{FuncCppBody, FuncDesc, FuncKind, ReturnKind};
use crate::settings::PropertyReadWrite;
use crate::type_ref::{Constness, CppNameStyle, StrEnc, StrType, TypeRef, TypeRefDesc, TypeRefTypeHint};
use crate::writer::rust_native::element::RustElement;
use crate::{
	settings, ClassKindOverride, Const, DefaultElement, Element, EntityExt, Enum, Field, Func, GeneratedType, GeneratorEnv,
	NameDebug, StrExt,
};

mod desc;

#[derive(Clone)]
pub enum Class<'tu, 'ge> {
	Clang {
		entity: Entity<'tu>,
		custom_fullname: Option<Rc<str>>,
		gen_env: &'ge GeneratorEnv<'tu>,
	},
	Desc(Rc<ClassDesc<'tu, 'ge>>),
}

impl<'tu, 'ge> Class<'tu, 'ge> {
	pub fn new(entity: Entity<'tu>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self::Clang {
			entity,
			custom_fullname: None,
			gen_env,
		}
	}

	pub fn new_ext(entity: Entity<'tu>, custom_fullname: impl Into<Rc<str>>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self::Clang {
			entity,
			custom_fullname: Some(custom_fullname.into()),
			gen_env,
		}
	}

	pub fn new_desc(desc: ClassDesc<'tu, 'ge>) -> Self {
		Self::Desc(Rc::new(desc))
	}

	/// Checks whether a class can be simple on Rust side, i.e. represented by plain struct with fields
	pub fn can_be_simple(&self) -> bool {
		let cpp_refname = self.cpp_name(CppNameStyle::Reference);
		settings::IMPLEMENTED_GENERICS.contains(cpp_refname.as_ref())
			|| self.has_fields()
				&& !self.has_descendants()
				&& !self.has_bases()
				&& !self
					.for_each_field(|field| {
						let type_ref = field.type_ref();
						ControlFlow::continue_until(!type_ref.kind().is_copy(type_ref.type_hint()))
					})
					.is_break()
	}

	pub fn kind(&self) -> ClassKind {
		match self {
			&Self::Clang { entity, gen_env, .. } => {
				if settings::ELEMENT_EXCLUDE_KIND
					.get(self.cpp_name(CppNameStyle::Reference).as_ref())
					.is_some_and(|ek| ek.is_excluded())
				{
					return ClassKind::Other;
				}
				match gen_env.get_export_config(entity).map(|c| c.class_kind_override) {
					Some(ClassKindOverride::Simple) => {
						if self.can_be_simple() {
							ClassKind::Simple
						} else {
							ClassKind::BoxedForced
						}
					}
					Some(ClassKindOverride::Boxed) => ClassKind::Boxed,
					Some(ClassKindOverride::BoxedForced) => ClassKind::BoxedForced,
					Some(ClassKindOverride::System) => ClassKind::System,
					None => {
						if self.is_system() {
							ClassKind::System
						} else if let Some(kind) = gen_env.get_class_kind(entity) {
							match kind {
								ClassKind::Simple if !self.can_be_simple() => ClassKind::BoxedForced,
								_ => kind,
							}
						} else {
							ClassKind::Other
						}
					}
				}
			}
			Self::Desc(desc) => desc.kind,
		}
	}

	pub fn type_ref(&self) -> TypeRef<'tu, 'ge> {
		match self {
			&Self::Clang { entity, gen_env, .. } => TypeRef::new(entity.get_type().expect("Can't get class type"), gen_env),
			Self::Desc(desc) => TypeRef::guess(desc.cpp_fullname.as_ref(), desc.rust_module),
		}
	}

	/// Returns `Some` with the string type if the current class name refers to a C++ `std::string` or `cv::String`
	pub fn string_type(&self) -> Option<StrType> {
		let cpp_refname = self.cpp_name(CppNameStyle::Reference);
		if cpp_refname.starts_with("std::") && cpp_refname.ends_with("::string") {
			Some(StrType::StdString(StrEnc::Text))
		} else if cpp_refname == "cv::String" {
			Some(StrType::CvString(StrEnc::Text))
		} else {
			None
		}
	}

	pub fn template_kind(&self) -> TemplateKind<'tu, 'ge> {
		match self {
			&Self::Clang { entity, gen_env, .. } => {
				if entity.get_template_kind().is_some() {
					TemplateKind::Template
				} else if let Some(template_entity) = entity.get_template() {
					TemplateKind::Specialization(Self::new(template_entity, gen_env))
				} else {
					TemplateKind::No
				}
			}
			Self::Desc(desc) => desc.template_kind.clone(),
		}
	}

	pub fn is_abstract(&self) -> bool {
		match self {
			&Self::Clang { entity, .. } => entity.is_abstract_record(),
			Self::Desc(desc) => desc.is_abstract,
		}
	}

	/// True if a class has virtual methods
	pub fn is_polymorphic(&self) -> bool {
		match self {
			Self::Clang { entity, .. } => entity
				.walk_methods_while(|f| ControlFlow::continue_until(f.is_virtual_method() || f.is_pure_virtual_method()))
				.is_break(),
			Self::Desc(_) => false,
		}
	}

	/// Special case of an empty class with only an anonymous enum inside (e.g., DrawLinesMatchesFlags)
	pub fn as_enum(&self) -> Option<Enum<'tu, 'ge>> {
		match self {
			&Self::Clang { entity, gen_env, .. } => {
				if !self.has_methods() && !self.has_fields() && !self.has_descendants() && !self.has_bases() {
					let children = entity.get_children();
					if let [single] = children.as_slice() {
						if matches!(single.get_kind(), EntityKind::EnumDecl) {
							Some(Enum::new_ext(
								*single,
								self.cpp_name(CppNameStyle::Declaration).as_ref(),
								gen_env,
							))
						} else {
							None
						}
					} else {
						None
					}
				} else {
					None
				}
			}
			Self::Desc(_) => None,
		}
	}

	/// Class has an explicit method named `clone()`
	pub fn has_explicit_clone(&self) -> bool {
		self.for_each_method(|m| ControlFlow::continue_until(m.is_clone())).is_break()
	}

	/// Class is simple (i.e. constructor-copiable in C++), but can't be simple in Rust
	pub fn has_implicit_clone(&self) -> bool {
		!self.is_abstract() && matches!(self.kind(), ClassKind::BoxedForced) && !self.has_virtual_destructor()
	}

	pub fn has_implicit_default_constructor(&self) -> bool {
		match self {
			&Self::Clang { entity, .. } => !entity
				.walk_children_while(|f| {
					if matches!(f.get_kind(), EntityKind::Constructor) {
						ControlFlow::Break(())
					} else {
						ControlFlow::Continue(())
					}
				})
				.is_break(),
			Self::Desc(_) => false,
		}
	}

	pub fn has_virtual_destructor(&self) -> bool {
		match self {
			Class::Clang { entity, .. } => entity
				.walk_children_while(|f| ControlFlow::continue_until(f.get_kind() == EntityKind::Destructor && f.is_virtual_method()))
				.is_break(),
			Class::Desc(_) => false,
		}
	}

	pub fn has_private_destructor(&self) -> bool {
		match self {
			Class::Clang { entity, .. } => entity
				.walk_children_while(|f| {
					ControlFlow::continue_until(
						f.get_kind() == EntityKind::Destructor && f.get_accessibility() != Some(Accessibility::Public),
					)
				})
				.is_break(),
			Class::Desc(_) => false,
		}
	}

	pub fn has_bases(&self) -> bool {
		match self {
			&Self::Clang { entity, .. } => entity.walk_bases_while(|_| ControlFlow::Break(())).is_break(),
			Self::Desc(desc) => !desc.bases.is_empty(),
		}
	}

	pub fn bases(&self) -> Cow<'_, [Class<'tu, 'ge>]> {
		match self {
			&Self::Clang { entity, gen_env, .. } => {
				let mut out = vec![];
				let entity = entity.get_template().unwrap_or(entity);
				let _ = entity.walk_bases_while(|child| {
					out.push(Self::new(Self::definition_entity(child), gen_env));
					ControlFlow::Continue(())
				});
				out.into()
			}
			Self::Desc(desc) => desc.bases.as_ref().into(),
		}
	}

	pub fn all_bases(&self) -> HashSet<Class<'tu, 'ge>> {
		#![allow(clippy::mutable_key_type)]
		#[allow(clippy::unnecessary_to_owned)]
		self
			.bases()
			.into_owned()
			.into_iter()
			.flat_map(|b| {
				let mut out = b.all_bases();
				out.insert(b);
				out
			})
			.collect()
	}

	pub fn has_descendants(&self) -> bool {
		match self {
			&Self::Clang { gen_env, .. } => gen_env.descendants_of(&self.cpp_name(CppNameStyle::Reference)).is_some(),
			Self::Desc(_) => false,
		}
	}

	pub fn descendants(&self) -> HashSet<Class<'tu, 'ge>> {
		#![allow(clippy::mutable_key_type)]
		match self {
			&Self::Clang { gen_env, .. } => gen_env
				.descendants_of(&self.cpp_name(CppNameStyle::Reference))
				.into_iter()
				.flat_map(|desc| desc.iter().map(|e| Self::new(*e, gen_env)))
				.collect(),
			Self::Desc(_) => HashSet::new(),
		}
	}

	pub fn all_descendants(&self) -> HashSet<Class<'tu, 'ge>> {
		#![allow(clippy::mutable_key_type)]
		self
			.descendants()
			.into_iter()
			.flat_map(|descendant| {
				let mut out = descendant.all_descendants();
				out.insert(descendant);
				out
			})
			.collect()
	}

	pub fn all_family(&self) -> HashSet<Class<'tu, 'ge>> {
		#![allow(clippy::mutable_key_type)]
		fn collect<'tu, 'ge>(out: &mut HashSet<Class<'tu, 'ge>>, cls: Class<'tu, 'ge>) {
			if out.insert(cls.clone()) {
				#[allow(clippy::unnecessary_to_owned)]
				for base in cls.bases().into_owned() {
					collect(out, base);
				}
				for desc in cls.descendants() {
					collect(out, desc);
				}
			}
		}

		let mut out = HashSet::new();
		collect(&mut out, self.clone());
		out
	}

	pub fn has_methods(&self) -> bool {
		self.for_each_method(|_| ControlFlow::Break(())).is_break()
	}

	#[inline]
	pub fn for_each_method(&self, mut predicate: impl FnMut(Func<'tu, 'ge>) -> ControlFlow<()>) -> ControlFlow<()> {
		match self {
			&Self::Clang { entity, gen_env, .. } => entity.walk_methods_while(|f| predicate(Func::new(f, gen_env))),
			Self::Desc(_) => ControlFlow::Continue(()),
		}
	}

	pub fn methods(&self, filter: impl Fn(&Func) -> bool) -> Vec<Func<'tu, 'ge>> {
		match self {
			Class::Clang { entity, gen_env, .. } => {
				let mut out = Vec::with_capacity(32);
				let _ = entity.walk_methods_while(|func_entity| {
					let func = Func::new(func_entity, gen_env);
					let func: Func = if let Some(func_fact) = gen_env.settings.func_replace.get(&mut func.matcher()) {
						func_fact(&func)
					} else {
						func
					};
					if func.is_generic() {
						if let Some(specs) = gen_env.settings.func_specialize.get(&mut func.matcher()) {
							for spec in specs {
								let spec_func = func.clone().specialize(spec);
								if filter(&spec_func) {
									out.push(spec_func);
								}
							}
							return ControlFlow::Continue(());
						}
					}
					if filter(&func) {
						out.push(func);
					}
					ControlFlow::Continue(())
				});
				for inject_func_fact in &gen_env.settings.func_inject {
					let inject_func: Func = inject_func_fact();
					if let Some(cls) = inject_func.kind().as_class_method() {
						if cls == self && filter(&inject_func) {
							out.push(inject_func);
						}
					}
				}
				out
			}
			Class::Desc(_) => vec![],
		}
	}

	pub fn has_fields(&self) -> bool {
		self.for_each_field(|_| ControlFlow::Break(())).is_break()
	}

	#[inline]
	pub fn for_each_field(&self, mut predicate: impl FnMut(Field<'tu, 'ge>) -> ControlFlow<()>) -> ControlFlow<()> {
		match self {
			&Self::Clang { entity, gen_env, .. } => entity.walk_fields_while(|f| predicate(Field::new(f, gen_env))),
			Self::Desc(_) => ControlFlow::Continue(()),
		}
	}

	pub fn fields(&self, filter: impl Fn(&Field) -> bool) -> Vec<Field<'tu, 'ge>> {
		let mut out = Vec::with_capacity(32);
		let _ = self.for_each_field(|f| {
			if filter(&f) {
				out.push(f);
			}
			ControlFlow::Continue(())
		});
		out
	}

	#[inline]
	pub fn for_each_const(&self, mut predicate: impl FnMut(Const<'tu>) -> ControlFlow<()>) -> ControlFlow<()> {
		match self {
			&Self::Clang { entity, .. } => entity.walk_consts_while(|f| predicate(Const::new(f))),
			Self::Desc(_) => ControlFlow::Continue(()),
		}
	}

	pub fn consts(&self) -> Vec<Const<'tu>> {
		let mut out = Vec::with_capacity(8);
		let _ = self.for_each_const(|c| {
			out.push(c);
			ControlFlow::Continue(())
		});
		out
	}

	pub fn field_methods<'f>(
		&self,
		fields: &'f [Field<'tu, 'ge>],
		constness_filter: Option<Constness>,
	) -> impl Iterator<Item = Func<'tu, 'ge>> + 'f {
		match self {
			&Self::Clang { gen_env, .. } => {
				let cls = self.clone();
				let accessor_generator = move |fld: &Field<'tu, 'ge>| {
					let doc_comment = Rc::from(fld.doc_comment());
					let def_loc = fld.file_line_name().location;
					let rust_module = fld.rust_module();
					let mut fld_type_ref = fld.type_ref();
					let fld_refname = fld.cpp_name(CppNameStyle::Reference);
					if let Some(type_hint) = gen_env.settings.property_override.get(fld_refname.as_ref()) {
						fld_type_ref.to_mut().set_type_hint(type_hint.clone());
					} else {
						let fld_type_kind = fld_type_ref.kind();
						if fld_type_kind
							.as_pointer()
							.is_some_and(|inner| inner.kind().as_primitive().is_some())
							&& !fld_type_kind.is_char_ptr_string(fld_type_ref.type_hint())
						{
							fld_type_ref.to_mut().set_type_hint(TypeRefTypeHint::PrimitivePtrAsRaw);
						} else if fld_type_kind.as_class().is_some_and(|cls| cls.kind().is_trait()) {
							fld_type_ref.to_mut().set_type_hint(TypeRefTypeHint::TraitClassConcrete);
						}
					}
					let fld_type_kind = fld_type_ref.kind();
					let return_kind = ReturnKind::infallible(fld_type_kind.return_as_naked(fld_type_ref.type_hint()));
					let fld_const = fld.constness();
					let passed_by_ref = fld_type_kind.can_return_as_direct_reference();
					let prop_tweak = gen_env.settings.property_tweaks.get(fld_refname.as_ref());
					let rust_custom_leafname = prop_tweak.and_then(|tweak| tweak.rename);
					let read_write = prop_tweak
						.and_then(|tweak| tweak.read_write)
						.unwrap_or(PropertyReadWrite::ReadWrite);
					let fld_declname = fld_refname.localname();
					let (mut read_const_yield, mut read_mut_yield) = if read_write.is_read() {
						if fld_const.is_mut() && passed_by_ref {
							let read_const_func = if constness_filter.is_none_or(|c| c.is_const()) {
								Some(Func::new_desc(
									FuncDesc::new(
										FuncKind::FieldAccessor(cls.clone(), fld.clone()),
										Constness::Const,
										return_kind,
										fld_declname,
										rust_module,
										[],
										fld_type_ref.as_ref().clone().with_inherent_constness(Constness::Const),
									)
									.def_loc(def_loc.clone())
									.doc_comment(Rc::clone(&doc_comment))
									.cpp_body(FuncCppBody::ManualCall("{{name}}".into()))
									.maybe_rust_custom_leafname(rust_custom_leafname),
								))
							} else {
								None
							};
							let read_mut_func = if constness_filter.is_none_or(|c| c.is_mut()) {
								Some(Func::new_desc(
									FuncDesc::new(
										FuncKind::FieldAccessor(cls.clone(), fld.clone()),
										Constness::Mut,
										return_kind,
										format!("{fld_declname}Mut"),
										rust_module,
										[],
										fld_type_ref.as_ref().clone().with_inherent_constness(Constness::Mut),
									)
									.def_loc(def_loc.clone())
									.doc_comment(Rc::clone(&doc_comment))
									.cpp_body(FuncCppBody::ManualCall("{{name}}".into()))
									.maybe_rust_custom_leafname(rust_custom_leafname.map(|name| format!("{name}_mut"))),
								))
							} else {
								None
							};
							(read_const_func, read_mut_func)
						} else {
							let single_read_func = if constness_filter.is_none_or(|c| c == fld_const) {
								Some(Func::new_desc(
									FuncDesc::new(
										FuncKind::FieldAccessor(cls.clone(), fld.clone()),
										fld_const,
										return_kind,
										fld_declname,
										rust_module,
										[],
										fld_type_ref.as_ref().clone(),
									)
									.def_loc(def_loc.clone())
									.doc_comment(Rc::clone(&doc_comment))
									.cpp_body(FuncCppBody::ManualCall("{{name}}".into()))
									.maybe_rust_custom_leafname(rust_custom_leafname),
								))
							} else {
								None
							};
							(single_read_func, None)
						}
					} else {
						(None, None)
					};
					let mut write_yield = if read_write.is_write()
						&& constness_filter.is_none_or(|c| c.is_mut())
						&& !fld_type_ref.constness().is_const()
						&& !fld_type_kind.as_fixed_array().is_some()
					{
						let (first_letter, rest) = fld_declname.capitalize_first_ascii_letter().expect("Empty fld_declname");
						Some(Func::new_desc(
							FuncDesc::new(
								FuncKind::FieldAccessor(cls.clone(), fld.clone()),
								Constness::Mut,
								ReturnKind::InfallibleNaked,
								format!("set{first_letter}{rest}"),
								rust_module,
								[Field::new_desc(FieldDesc {
									cpp_fullname: "val".into(),
									type_ref: fld_type_ref.as_ref().clone().with_inherent_constness(Constness::Const),
									default_value: fld.default_value().map(|v| v.into()),
								})],
								TypeRefDesc::void(),
							)
							.doc_comment(doc_comment)
							.def_loc(def_loc)
							.cpp_body(FuncCppBody::ManualCall("{{name}} = {{args}}".into()))
							.maybe_rust_custom_leafname(rust_custom_leafname.map(|name| format!("set_{name}"))),
						))
					} else {
						None
					};
					iter::from_fn(move || {
						read_const_yield
							.take()
							.or_else(|| read_mut_yield.take())
							.or_else(|| write_yield.take())
					})
				};
				FieldMethodsIter::Clang(fields.iter().flat_map(accessor_generator))
			}
			Self::Desc(_) => FieldMethodsIter::Desc,
		}
	}

	/// Returns an entity that defines current class, for specialized classes (Point_<int>) it's the template (Point_<T>), for
	/// not fully defined classes it goes to its definition location.
	fn definition_entity(entity: Entity<'tu>) -> Entity<'tu> {
		entity.get_template().unwrap_or(entity).get_definition().unwrap_or(entity)
	}

	pub fn is_definition(&self) -> bool {
		match self {
			&Self::Clang { entity, .. } => {
				let class_loc = entity.get_location();
				let def_loc = entity.get_definition().and_then(|d| d.get_location());
				match (class_loc, def_loc) {
					(Some(class_loc), Some(def_loc)) => class_loc == def_loc,
					(_, None) => false,
					_ => true,
				}
			}
			Self::Desc(_) => true,
		}
	}

	pub fn generated_types(&self) -> Vec<GeneratedType<'tu, 'ge>> {
		self
			.fields(|f| f.exclude_kind().is_included())
			.into_iter()
			.flat_map(|f| f.type_ref().generated_types())
			.chain(
				self
					.methods(|m| m.exclude_kind().is_included())
					.into_iter()
					.flat_map(|m| m.generated_types()),
			)
			.collect()
	}
}

impl<'tu> ToEntity<'tu> for &Class<'tu, '_> {
	fn to_entity(self) -> Option<Entity<'tu>> {
		match self {
			Class::Clang { entity, .. } => Some(*entity),
			Class::Desc(_) => None,
		}
	}
}

impl Element for Class<'_, '_> {
	fn exclude_kind(&self) -> ExcludeKind {
		match self {
			Self::Clang { .. } => DefaultElement::exclude_kind(self)
				.with_is_ignored(|| match self.kind() {
					ClassKind::Other => true,
					ClassKind::System => {
						!settings::IMPLEMENTED_SYSTEM_CLASSES.contains(self.cpp_name(CppNameStyle::Reference).as_ref())
					}
					ClassKind::Simple | ClassKind::Boxed | ClassKind::BoxedForced => match self.template_kind() {
						TemplateKind::Template => true,
						TemplateKind::No => !self.is_definition() || self.cpp_namespace() == "",
						TemplateKind::Specialization(_) => {
							!settings::IMPLEMENTED_GENERICS.contains(self.cpp_name(CppNameStyle::Reference).as_ref())
						}
					},
				})
				.with_is_excluded(|| match self.kind() {
					ClassKind::System | ClassKind::Other => true,
					ClassKind::Simple => false,
					ClassKind::Boxed | ClassKind::BoxedForced => self.has_private_destructor(),
				}),
			Self::Desc(desc) => desc.exclude_kind,
		}
	}

	fn is_system(&self) -> bool {
		match self {
			&Self::Clang { entity, .. } => DefaultElement::is_system(entity),
			Self::Desc(desc) => matches!(desc.kind, ClassKind::System),
		}
	}

	fn is_public(&self) -> bool {
		match self {
			&Self::Clang { entity, .. } => DefaultElement::is_public(entity),
			Self::Desc(desc) => desc.is_public,
		}
	}

	fn doc_comment(&self) -> Cow<'_, str> {
		match self {
			Self::Clang { entity, .. } => entity.doc_comment(),
			Self::Desc(_) => "".into(),
		}
	}

	fn cpp_namespace(&self) -> Cow<'_, str> {
		#[inline(always)]
		fn inner(cpp_fullname: &str) -> Cow<'_, str> {
			cpp_fullname.namespace().into()
		}

		match self {
			Self::Clang {
				custom_fullname: Some(cpp_fullname),
				..
			} => inner(cpp_fullname.as_ref()),
			Self::Clang { entity, .. } => DefaultElement::cpp_namespace(*entity).into(),
			Self::Desc(desc) => inner(desc.cpp_fullname.as_ref()),
		}
	}

	fn cpp_name(&self, style: CppNameStyle) -> Cow<'_, str> {
		match self {
			Self::Clang {
				custom_fullname: Some(cpp_fullname),
				..
			} => cpp_fullname.cpp_name_from_fullname(style).into(),
			&Self::Clang { entity, .. } => DefaultElement::cpp_name(self, entity, style),
			Self::Desc(desc) => desc.cpp_fullname.cpp_name_from_fullname(style).into(),
		}
	}
}

impl Hash for Class<'_, '_> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		match self {
			Self::Clang { entity, .. } => entity.hash(state),
			Self::Desc(desc) => desc.hash(state),
		}
	}
}

impl PartialEq for Class<'_, '_> {
	fn eq(&self, other: &Self) -> bool {
		self.cpp_name(CppNameStyle::Reference) == other.cpp_name(CppNameStyle::Reference) && self.kind() == other.kind()
	}
}

impl Eq for Class<'_, '_> {}

impl<'me> NameDebug<'me> for &'me Class<'me, '_> {
	fn file_line_name(self) -> LocationName<'me> {
		match self {
			Class::Clang { entity, .. } => entity.file_line_name(),
			Class::Desc(desc) => LocationName::new(DefinitionLocation::Generated, desc.cpp_fullname.as_ref()),
		}
	}
}

impl fmt::Debug for Class<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut props = vec![];
		if self.can_be_simple() {
			props.push("can_be_simple");
		}
		if self.template_kind().is_template() {
			props.push("template");
		}
		if self.template_kind().as_template_specialization().is_some() {
			props.push("template_specialization");
		}
		if self.is_abstract() {
			props.push("abstract");
		}
		if self.is_polymorphic() {
			props.push("polymorphic");
		}
		if self.kind().is_trait() {
			props.push("trait");
		}
		if self.as_enum().is_some() {
			props.push("enum");
		}
		if self.has_explicit_clone() {
			props.push("has_explicit_clone");
		}
		if self.has_implicit_clone() {
			props.push("has_implicit_clone");
		}
		if self.has_virtual_destructor() {
			props.push("has_virtual_dtor");
		}
		if self.has_private_destructor() {
			props.push("has_private_dtor")
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
		if !self.consts().is_empty() {
			props.push("has_consts");
		}
		if self.is_definition() {
			props.push("definition");
		}
		if matches!(
			self,
			Self::Clang {
				custom_fullname: Some(_),
				..
			}
		) {
			props.push("custom_fullname");
		}
		let mut debug_struct = f.debug_struct(match self {
			Self::Clang { .. } => "Class::Clang",
			Self::Desc(_) => "Class::Desc",
		});
		self
			.update_debug_struct(&mut debug_struct)
			.field("kind", &self.kind())
			.field("props", &props.join(", "))
			.field("string_type", &self.string_type())
			.finish()
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ClassKind {
	/// Simple class, check [`Class::can_be_simple`] for more details
	Simple,
	/// Opaque class where all access to its fields is happening by C++ side using a pointer
	Boxed,
	/// Marked simple but forced to be boxed by presence of non-simple fields or descendants
	BoxedForced,
	/// System class like `std::string`
	System,
	/// Class is something else, generally ignored
	Other,
}

impl ClassKind {
	pub fn is_simple(self) -> bool {
		match self {
			Self::Simple => true,
			Self::Boxed | Self::BoxedForced | Self::System | Self::Other => false,
		}
	}

	pub fn is_boxed(self) -> bool {
		match self {
			Self::Boxed | Self::BoxedForced => true,
			Self::Simple | Self::Other | Self::System => false,
		}
	}

	pub fn is_trait(&self) -> bool {
		match self {
			Self::Boxed | Self::BoxedForced | Self::System => true,
			Self::Simple | Self::Other => false,
		}
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TemplateKind<'tu, 'ge> {
	/// Not a template or a specialization
	No,
	/// Base class template, e.g. `Point_<T>`
	Template,
	/// A specific instance (`Point_<int>`) of the class template (`Point_<T>`)
	Specialization(Class<'tu, 'ge>),
}

impl<'tu, 'ge> TemplateKind<'tu, 'ge> {
	pub fn is_template(&self) -> bool {
		match self {
			TemplateKind::Template => true,
			TemplateKind::No | TemplateKind::Specialization(_) => false,
		}
	}

	pub fn as_template_specialization(&self) -> Option<&Class<'tu, 'ge>> {
		match self {
			TemplateKind::Specialization(cls) => Some(cls),
			TemplateKind::No | TemplateKind::Template => None,
		}
	}
}

pub enum FieldMethodsIter<'tu: 'ge, 'ge, I: Iterator<Item = Func<'tu, 'ge>>> {
	Clang(I),
	Desc,
}

impl<'tu, 'ge, I: Iterator<Item = Func<'tu, 'ge>>> Iterator for FieldMethodsIter<'tu, 'ge, I> {
	type Item = Func<'tu, 'ge>;

	fn next(&mut self) -> Option<Self::Item> {
		match self {
			FieldMethodsIter::Clang(iter) => iter.next(),
			FieldMethodsIter::Desc => None,
		}
	}
}
