use std::borrow::Cow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::{fmt, iter};

use clang::{Accessibility, Entity, EntityKind};

pub use desc::ClassDesc;

use crate::debug::{DefinitionLocation, LocationName};
use crate::element::ExcludeKind;
use crate::entity::{WalkAction, WalkResult};
use crate::field::{FieldDesc, FieldTypeHint};
use crate::func::{FuncCppBody, FuncDesc, FuncKind, FuncRustBody, ReturnKind};
use crate::type_ref::{Constness, CppNameStyle, StrEnc, StrType, TypeRef, TypeRefDesc, TypeRefTypeHint};
use crate::writer::rust_native::element::RustElement;
use crate::{
	settings, ClassSimplicity, Const, DefaultElement, Element, EntityExt, Enum, Field, Func, FuncTypeHint, GeneratedType,
	GeneratorEnv, NameDebug, StrExt,
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

	/// Checks whether a class can be simple on Rust side, i.e. represented by plain struct with public fields
	pub fn can_be_simple(&self) -> bool {
		let cpp_refname = self.cpp_name(CppNameStyle::Reference);
		if settings::IMPLEMENTED_GENERICS.contains(cpp_refname.as_ref()) {
			return true;
		}
		self.has_fields()
			&& !self.has_descendants()
			&& !self.has_bases()
			&& !self
				.for_each_field(|field| WalkAction::continue_until(!field.type_ref().is_copy()))
				.is_interrupted()
	}

	pub fn kind(&self) -> ClassKind {
		match self {
			&Self::Clang { entity, gen_env, .. } => {
				if settings::ELEMENT_EXCLUDE_KIND
					.get(self.cpp_name(CppNameStyle::Reference).as_ref())
					.map_or(false, |ek| ek.is_excluded())
				{
					return ClassKind::Other;
				}
				match gen_env.get_export_config(entity).map(|c| c.simplicity) {
					Some(ClassSimplicity::Simple) => {
						if self.can_be_simple() {
							ClassKind::Simple
						} else {
							ClassKind::BoxedForced
						}
					}
					Some(ClassSimplicity::Boxed) => ClassKind::Boxed,
					Some(ClassSimplicity::BoxedForced) => ClassKind::BoxedForced,
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
			Self::Desc(desc) => TypeRef::guess(desc.cpp_fullname.as_ref(), Rc::clone(&desc.rust_module)),
		}
	}

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
					TemplateKind::Specialization(Box::new(Self::new(template_entity, gen_env)))
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

	/// True if class has virtual methods
	pub fn is_polymorphic(&self) -> bool {
		match self {
			Self::Clang { entity, .. } => entity
				.walk_methods_while(|f| WalkAction::continue_until(f.is_virtual_method() || f.is_pure_virtual_method()))
				.is_interrupted(),
			Self::Desc(_) => false,
		}
	}

	pub fn is_trait(&self) -> bool {
		self.kind().is_boxed()
		//		self.is_abstract() || self.has_descendants() || settings::FORCE_CLASS_TRAIT.contains(self.cpp_name(CppNameStyle::Reference).as_ref())
	}

	/// Special case of an empty class with only an anonymous enum inside (e.g. DrawLinesMatchesFlags)
	pub fn as_enum(&self) -> Option<Enum<'tu>> {
		match self {
			&Self::Clang { entity, .. } => {
				if !self.has_methods() && !self.has_fields() && !self.has_descendants() && !self.has_bases() {
					let children = entity.get_children();
					if let [single] = children.as_slice() {
						if matches!(single.get_kind(), EntityKind::EnumDecl) {
							Some(Enum::new_ext(*single, self.cpp_name(CppNameStyle::Declaration).into_owned()))
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
		self
			.for_each_method(|m| WalkAction::continue_until(m.is_clone()))
			.is_interrupted()
	}

	/// Class is simple (i.e. constructor-copiable in C++), but can't be simple in Rust
	pub fn has_implicit_clone(&self) -> bool {
		!self.is_abstract() && matches!(self.kind(), ClassKind::BoxedForced) && !self.has_virtual_destructor()
	}

	pub fn has_virtual_destructor(&self) -> bool {
		match self {
			Class::Clang { entity, .. } => entity
				.walk_children_while(|f| WalkAction::continue_until(f.get_kind() == EntityKind::Destructor && f.is_virtual_method()))
				.is_interrupted(),
			Class::Desc(_) => false,
		}
	}

	pub fn has_private_destructor(&self) -> bool {
		match self {
			Class::Clang { entity, .. } => entity
				.walk_children_while(|f| {
					WalkAction::continue_until(
						f.get_kind() == EntityKind::Destructor
							&& f.get_accessibility().map_or(true, |acc| acc != Accessibility::Public),
					)
				})
				.is_interrupted(),
			Class::Desc(_) => false,
		}
	}

	pub fn has_bases(&self) -> bool {
		match self {
			&Self::Clang { entity, .. } => entity.walk_bases_while(|_| WalkAction::Interrupt).is_interrupted(),
			Self::Desc(desc) => !desc.bases.is_empty(),
		}
	}

	pub fn bases(&self) -> Cow<[Class<'tu, 'ge>]> {
		match self {
			&Self::Clang { entity, gen_env, .. } => {
				let mut out = vec![];
				let entity = entity.get_template().unwrap_or(entity);
				entity.walk_bases_while(|child| {
					out.push(Self::new(Self::definition_entity(child), gen_env));
					WalkAction::Continue
				});
				out.into()
			}
			Self::Desc(desc) => desc.bases.as_ref().into(),
		}
	}

	pub fn all_bases(&self) -> HashSet<Class<'tu, 'ge>> {
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
		self.for_each_method(|_| WalkAction::Interrupt).is_interrupted()
	}

	#[inline]
	pub fn for_each_method(&self, mut predicate: impl FnMut(Func<'tu, 'ge>) -> WalkAction) -> WalkResult {
		match self {
			&Self::Clang { entity, gen_env, .. } => entity.walk_methods_while(|f| predicate(Func::new(f, gen_env))),
			Self::Desc(_) => WalkResult::Completed,
		}
	}

	pub fn methods(&self, constness_filter: Option<Constness>) -> Vec<Func<'tu, 'ge>> {
		let mut out = Vec::with_capacity(32);
		self.for_each_method(|func| {
			if constness_filter.map_or(true, |c| c == func.constness()) {
				if func.is_generic() {
					if let Some(specs) = settings::FUNC_SPECIALIZE.get(&func.func_id()) {
						for spec in specs {
							out.push(func.clone().specialize(spec));
						}
						return WalkAction::Continue;
					}
				}
				out.push(func);
			}
			WalkAction::Continue
		});
		let rust_module = match self {
			Class::Clang { gen_env, .. } => gen_env.module(),
			Class::Desc(desc) => desc.rust_module.as_ref(),
		};
		for inject_func_fact in settings::FUNC_INJECT_MANUAL.get(rust_module).into_iter().flatten() {
			let inject_func: Func = inject_func_fact();
			if constness_filter.map_or(true, |c| c == inject_func.constness()) {
				if let Some(cls) = inject_func.kind().as_class_method() {
					if cls == self {
						out.push(inject_func);
					}
				}
			}
		}
		out
	}

	pub fn has_fields(&self) -> bool {
		self.for_each_field(|_| WalkAction::Interrupt).is_interrupted()
	}

	#[inline]
	pub fn for_each_field(&self, mut predicate: impl FnMut(Field<'tu, 'ge>) -> WalkAction) -> WalkResult {
		match self {
			&Self::Clang { entity, gen_env, .. } => entity.walk_fields_while(|f| predicate(Field::new(f, gen_env))),
			Self::Desc(_) => WalkResult::Completed,
		}
	}

	pub fn fields(&self) -> Vec<Field<'tu, 'ge>> {
		let mut out = Vec::with_capacity(32);
		self.for_each_field(|f| {
			out.push(f);
			WalkAction::Continue
		});
		out
	}

	#[inline]
	pub fn for_each_const(&self, mut predicate: impl FnMut(Const<'tu>) -> WalkAction) -> WalkResult {
		match self {
			&Self::Clang { entity, .. } => entity.walk_consts_while(|f| predicate(Const::new(f))),
			Self::Desc(_) => WalkResult::Completed,
		}
	}

	pub fn consts(&self) -> Vec<Const<'tu>> {
		let mut out = Vec::with_capacity(8);
		self.for_each_const(|c| {
			out.push(c);
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
		match self {
			&Self::Clang { .. } => {
				let mut out = Vec::with_capacity(fields.size_hint().1.map_or(8, |x| x * 2));
				out.extend(fields.flat_map(|fld| {
					iter::from_fn({
						let doc_comment = Rc::from(fld.doc_comment());
						let fld_type_ref = fld.type_ref().with_type_hint(TypeRefTypeHint::PrimitiveRefAsPointer);
						let mut read_yield = if constness_filter.map_or(true, |c| c == fld.constness()) {
							let read_func = Func::new_desc(FuncDesc {
								type_hint: FuncTypeHint::None,
								kind: FuncKind::FieldAccessor(self.clone(), fld.clone()),
								cpp_name: fld.cpp_name(CppNameStyle::Reference).into(),
								rust_custom_leafname: None,
								rust_module: fld.rust_module().into(),
								constness: fld.constness(),
								return_kind: ReturnKind::infallible(fld_type_ref.return_as_naked()),
								doc_comment: Rc::clone(&doc_comment),
								def_loc: fld.file_line_name().location,
								arguments: Rc::new([]),
								return_type_ref: fld_type_ref.clone(),
								cpp_body: FuncCppBody::ManualCall("{{name}}".into()),
								rust_body: FuncRustBody::Auto,
							});
							Some(read_func)
						} else {
							None
						};
						let mut write_yield = if constness_filter.map_or(true, |c| c.is_mut())
							&& !fld_type_ref.constness().is_const()
							&& !fld_type_ref.as_fixed_array().is_some()
						{
							let cpp_name = fld.cpp_name(CppNameStyle::Declaration);
							let (first_letter, rest) = cpp_name.split_at(1);
							let write_func = Func::new_desc(FuncDesc {
								type_hint: FuncTypeHint::None,
								kind: FuncKind::FieldAccessor(self.clone(), fld.clone()),
								cpp_name: format!("{}::set{}{rest}", fld.cpp_namespace(), first_letter.to_uppercase()).into(),
								rust_custom_leafname: None,
								rust_module: fld.rust_module().into(),
								constness: Constness::Mut,
								doc_comment,
								def_loc: fld.file_line_name().location,
								arguments: Rc::new([Field::new_desc(FieldDesc {
									cpp_fullname: "val".into(),
									type_ref: fld_type_ref,
									type_hint: FieldTypeHint::None,
									default_value: fld.default_value().map(|v| v.into()),
								})]),
								return_kind: ReturnKind::InfallibleNaked,
								return_type_ref: TypeRefDesc::void(),
								cpp_body: FuncCppBody::ManualCall("{{name}} = {{args}}".into()),
								rust_body: FuncRustBody::Auto,
							});
							Some(write_func)
						} else {
							None
						};
						move || read_yield.take().or_else(|| write_yield.take())
					})
				}));
				out
			}
			Self::Desc(_) => {
				vec![]
			}
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
			.fields()
			.into_iter()
			.filter(|f| f.exclude_kind().is_included())
			.flat_map(|f| f.type_ref().generated_types())
			.chain(
				self
					.methods(None)
					.into_iter()
					.filter(|m| m.exclude_kind().is_included())
					.flat_map(|m| m.generated_types()),
			)
			.collect()
	}
}

impl Element for Class<'_, '_> {
	fn exclude_kind(&self) -> ExcludeKind {
		match self {
			Self::Clang { .. } => {
				DefaultElement::exclude_kind(self)
					.with_is_excluded(|| self.has_private_destructor())
					.with_is_ignored(|| match self.template_kind() {
						TemplateKind::No => !self.is_definition(),
						TemplateKind::Template => true,
						TemplateKind::Specialization(_) => !settings::IMPLEMENTED_GENERICS.contains(self.cpp_name(CppNameStyle::Reference).as_ref()),
					} || self.is_system() && !settings::IMPLEMENTED_SYSTEM_CLASSES.contains(self.cpp_name(CppNameStyle::Reference).as_ref())
						|| matches!(self.kind(), ClassKind::Other)
						|| self.cpp_namespace() == "")
			}
			Self::Desc(desc) => desc.exclude_kind,
		}
	}

	fn is_system(&self) -> bool {
		match self {
			&Self::Clang { entity, .. } => DefaultElement::is_system(entity),
			Self::Desc(desc) => desc.is_system,
		}
	}

	fn is_public(&self) -> bool {
		match self {
			&Self::Clang { entity, .. } => DefaultElement::is_public(entity),
			Self::Desc(desc) => desc.is_public,
		}
	}

	fn doc_comment(&self) -> Cow<str> {
		match self {
			&Self::Clang { entity, .. } => entity.get_comment().unwrap_or_default().into(),
			Self::Desc(_) => "".into(),
		}
	}

	fn cpp_namespace(&self) -> Cow<str> {
		#[inline(always)]
		fn inner(cpp_fullname: &str) -> Cow<str> {
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

	fn cpp_name(&self, style: CppNameStyle) -> Cow<str> {
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
		if self.is_trait() {
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
			Self::Simple | Self::System | Self::Other => false,
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
	Specialization(Box<Class<'tu, 'ge>>),
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
			TemplateKind::Specialization(cls) => Some(cls.as_ref()),
			TemplateKind::No | TemplateKind::Template => None,
		}
	}
}
