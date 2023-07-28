use std::borrow::Cow;
use std::fmt;

use clang::{Entity, EntityKind};

use crate::debug::LocationName;
use crate::element::ExcludeKind;
use crate::type_ref::{CppNameStyle, NameStyle, TypeRefTypeHint};
use crate::writer::rust_native::type_ref::TypeRefExt;
use crate::{
	settings, Class, DefaultElement, Element, EntityElement, EntityExt, Enum, GeneratedType, GeneratorEnv, NameDebug, TypeRef,
	WalkAction,
};

#[derive(Clone)]
pub struct Typedef<'tu, 'ge> {
	entity: Entity<'tu>,
	gen_env: &'ge GeneratorEnv<'tu>,
}

impl<'tu, 'ge> Typedef<'tu, 'ge> {
	/// Create a corresponding type from EntityKind::TypedefDecl or EntityKind::TypeAliasDecl.
	///
	/// Sometimes the actual type is not a `TypeDef`, but a `Class` for example in case of:
	/// ```c
	/// typedef struct
	/// {
	///   int x, y, w, h;
	///   float score;
	/// } Box;
	/// ```
	pub fn try_new(entity: Entity<'tu>, gen_env: &'ge GeneratorEnv<'tu>) -> NewTypedefResult<'tu, 'ge> {
		let mut out = NewTypedefResult::Typedef(Self { entity, gen_env });
		entity.walk_children_while(|child| {
			let child_unnamed_or_same_name = child
				.get_name()
				.map_or(true, |child_name| Some(child_name) == entity.get_name());
			if child_unnamed_or_same_name {
				match child.get_kind() {
					EntityKind::StructDecl => {
						out = NewTypedefResult::Class(Class::new_ext(
							child,
							entity.cpp_name(CppNameStyle::Reference).into_owned(),
							gen_env,
						));
					}
					EntityKind::EnumDecl => {
						out = NewTypedefResult::Enum(Enum::new_ext(child, entity.cpp_name(CppNameStyle::Reference).into_owned()));
					}
					_ => {}
				}
			}
			WalkAction::Interrupt
		});
		out
	}

	pub fn type_ref(&self) -> TypeRef<'tu, 'ge> {
		TypeRef::new(self.entity.get_type().expect("Can't get typedef type"), self.gen_env)
	}

	pub fn underlying_type_ref(&self) -> TypeRef<'tu, 'ge> {
		TypeRef::new_ext(
			self
				.entity
				.get_typedef_underlying_type()
				.expect("Can't get typedef underlying type"),
			TypeRefTypeHint::None,
			Some(self.entity),
			self.gen_env,
		)
	}

	pub fn generated_types(&self) -> Vec<GeneratedType<'tu, 'ge>> {
		self.underlying_type_ref().generated_types()
	}
}

impl<'tu> EntityElement<'tu> for Typedef<'tu, '_> {
	fn entity(&self) -> Entity<'tu> {
		self.entity
	}
}

impl Element for Typedef<'_, '_> {
	fn exclude_kind(&self) -> ExcludeKind {
		DefaultElement::exclude_kind(self)
			.with_exclude_kind(|| self.underlying_type_ref().exclude_kind())
			.with_is_excluded(|| {
				settings::PRIMITIVE_TYPEDEFS.contains_key(self.cpp_name(CppNameStyle::Reference).as_ref()) || {
					let underlying_type = self.underlying_type_ref();
					// fixes recursive typedefs like Cv16suf or GKernelPackage
					// fixme: don't rely on rust name to disconnect this generic module from rust_native
					self.type_ref().rust_name(NameStyle::ref_()) == underlying_type.rust_name(NameStyle::ref_())
				}
			})
	}

	fn is_system(&self) -> bool {
		DefaultElement::is_system(self.entity)
	}

	fn is_public(&self) -> bool {
		DefaultElement::is_public(self.entity)
	}

	fn doc_comment(&self) -> Cow<str> {
		self.entity.get_comment().unwrap_or_default().into()
	}

	fn cpp_namespace(&self) -> Cow<str> {
		DefaultElement::cpp_namespace(self.entity).into()
	}

	fn cpp_name(&self, style: CppNameStyle) -> Cow<str> {
		DefaultElement::cpp_name(self, self.entity, style)
	}
}

pub enum NewTypedefResult<'tu, 'ge> {
	Typedef(Typedef<'tu, 'ge>),
	Class(Class<'tu, 'ge>),
	Enum(Enum<'tu>),
}

impl NewTypedefResult<'_, '_> {
	pub fn exclude_kind(&self) -> ExcludeKind {
		match self {
			NewTypedefResult::Typedef(tdef) => tdef.exclude_kind(),
			NewTypedefResult::Class(cls) => cls.exclude_kind(),
			NewTypedefResult::Enum(enm) => enm.exclude_kind(),
		}
	}
}

impl<'me> NameDebug<'me> for &'me Typedef<'_, '_> {
	fn file_line_name(self) -> LocationName<'me> {
		self.entity.file_line_name()
	}
}

impl PartialEq for Typedef<'_, '_> {
	fn eq(&self, other: &Self) -> bool {
		self.entity == other.entity
	}
}

impl fmt::Debug for Typedef<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut debug_struct = f.debug_struct("Typedef");
		self
			.update_debug_struct(&mut debug_struct)
			.field("export_config", &self.gen_env.get_export_config(self.entity))
			.field("underlying_type_ref", &self.underlying_type_ref())
			.finish()
	}
}
