use std::borrow::Cow;
use std::fmt;
use std::ops::ControlFlow;
use std::rc::Rc;

use clang::{Entity, EntityKind};
pub use desc::TypedefDesc;

use crate::debug::{DefinitionLocation, LocationName};
use crate::element::ExcludeKind;
use crate::type_ref::{CppNameStyle, NameStyle, TypeRefDesc, TypeRefKind, TypeRefTypeHint};
use crate::writer::rust_native::type_ref::TypeRefExt;
use crate::{
	settings, Class, Constness, DefaultElement, Element, EntityExt, Enum, GeneratedType, GeneratorEnv, NameDebug, StrExt, TypeRef,
};

mod desc;

#[derive(Clone)]
pub enum Typedef<'tu, 'ge> {
	Clang {
		entity: Entity<'tu>,
		gen_env: &'ge GeneratorEnv<'tu>,
	},
	Desc(Rc<TypedefDesc<'tu, 'ge>>),
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
		let mut out = NewTypedefResult::Typedef(Self::Clang { entity, gen_env });
		let _ = entity.walk_children_while(|child| {
			let child_unnamed_or_same_name = child
				.get_name()
				.map_or(true, |child_name| Some(child_name) == entity.get_name());
			if child_unnamed_or_same_name {
				match child.get_kind() {
					EntityKind::StructDecl => {
						out = NewTypedefResult::Class(Class::new_ext(child, entity.cpp_name(CppNameStyle::Reference), gen_env));
					}
					EntityKind::EnumDecl => {
						out = NewTypedefResult::Enum(Enum::new_ext(child, entity.cpp_name(CppNameStyle::Reference)));
					}
					_ => {}
				}
			}
			ControlFlow::Break(())
		});
		out
	}

	pub fn new_desc(desc: TypedefDesc<'tu, 'ge>) -> Self {
		Self::Desc(Rc::new(desc))
	}

	pub fn type_ref(&self) -> TypeRef<'tu, 'ge> {
		match self {
			Self::Clang { entity, gen_env } => TypeRef::new(entity.get_type().expect("Can't get typedef type"), gen_env),
			Self::Desc(desc) => TypeRef::new_desc(TypeRefDesc::new(
				TypeRefKind::Typedef(Self::Desc(Rc::clone(desc))),
				Constness::Mut,
			)),
		}
	}

	pub fn underlying_type_ref(&self) -> TypeRef<'tu, 'ge> {
		match self {
			Self::Clang { entity, gen_env } => TypeRef::new_ext(
				entity
					.get_typedef_underlying_type()
					.expect("Can't get typedef underlying type"),
				TypeRefTypeHint::None,
				Some(*entity),
				gen_env,
			),
			Self::Desc(desc) => desc.underlying_type.clone(),
		}
	}

	pub fn generated_types(&self) -> Vec<GeneratedType<'tu, 'ge>> {
		self.underlying_type_ref().generated_types()
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
		match self {
			Self::Clang { entity, .. } => DefaultElement::is_system(*entity),
			Self::Desc(_) => false,
		}
	}

	fn is_public(&self) -> bool {
		match self {
			Self::Clang { entity, .. } => DefaultElement::is_public(*entity),
			Self::Desc(_) => true,
		}
	}

	fn doc_comment(&self) -> Cow<'_, str> {
		match self {
			Self::Clang { entity, .. } => entity.doc_comment(),
			Self::Desc(_) => "".into(),
		}
	}

	fn cpp_namespace(&self) -> Cow<'_, str> {
		match self {
			Self::Clang { entity, .. } => DefaultElement::cpp_namespace(*entity).into(),
			Self::Desc(desc) => desc.cpp_fullname.namespace().into(),
		}
	}

	fn cpp_name(&self, style: CppNameStyle) -> Cow<'_, str> {
		match self {
			Self::Clang { entity, .. } => DefaultElement::cpp_name(self, *entity, style),
			Self::Desc(desc) => desc.cpp_fullname.cpp_name_from_fullname(style).into(),
		}
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
		match self {
			Typedef::Clang { entity, .. } => entity.file_line_name(),
			Typedef::Desc(desc) => LocationName::new(DefinitionLocation::Generated, desc.cpp_fullname.as_ref()),
		}
	}
}

impl PartialEq for Typedef<'_, '_> {
	fn eq(&self, other: &Self) -> bool {
		self.cpp_name(CppNameStyle::Reference) == other.cpp_name(CppNameStyle::Reference)
	}
}

impl fmt::Debug for Typedef<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut debug_struct = f.debug_struct(match self {
			Self::Clang { .. } => "Typedef::Clang",
			Self::Desc(_) => "Typedef::Desc",
		});
		self
			.update_debug_struct(&mut debug_struct)
			.field("underlying_type_ref", &self.underlying_type_ref())
			.finish()
	}
}
