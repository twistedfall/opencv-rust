use std::borrow::Cow;
use std::fmt;
use std::ops::ControlFlow;
use std::rc::Rc;

use clang::{Entity, EntityKind, EntityVisitResult};

use crate::debug::LocationName;
use crate::element::ExcludeKind;
use crate::type_ref::CppNameStyle;
use crate::{Const, DefaultElement, Element, EntityElement, EntityExt, GeneratorEnv, NameDebug, StrExt};

#[derive(Debug, Clone, Copy)]
pub enum EnumBitfield {
	NotBitfield,
	BitfieldWithoutZero,
	BitfieldWithZero,
}

impl EnumBitfield {
	pub fn is_bitfield(self) -> bool {
		match self {
			Self::NotBitfield => false,
			Self::BitfieldWithoutZero | Self::BitfieldWithZero => true,
		}
	}
}

#[derive(Clone)]
pub struct Enum<'tu, 'ge> {
	entity: Entity<'tu>,
	gen_env: &'ge GeneratorEnv<'tu>,
	custom_fullname: Option<Rc<str>>,
}

impl<'tu, 'ge> Enum<'tu, 'ge> {
	pub fn new(entity: Entity<'tu>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self {
			entity,
			gen_env,
			custom_fullname: None,
		}
	}

	pub fn new_ext(entity: Entity<'tu>, custom_fullname: impl Into<Rc<str>>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self {
			entity,
			gen_env,
			custom_fullname: Some(custom_fullname.into()),
		}
	}

	pub fn is_anonymous(&self) -> bool {
		self.entity.is_anonymous() || /* clang-6 quirk */ self.cpp_name(CppNameStyle::Declaration).starts_with("(anonymous enum")
	}

	pub fn as_typedefed(&self) -> Option<Entity<'tu>> {
		if matches!(self.entity.get_kind(), EntityKind::TypedefDecl | EntityKind::TypeAliasDecl) {
			let mut child = None;
			let _ = self.entity.walk_children_while(|c| {
				child = Some(c);
				ControlFlow::Break(())
			});
			Some(child.expect("Invalid anonymous typedefed enum"))
		} else {
			None
		}
	}

	pub fn consts(&self) -> Vec<Const<'tu>> {
		let mut out = vec![];
		self.as_typedefed().unwrap_or(self.entity).visit_children(|const_decl, _| {
			if const_decl.get_kind() == EntityKind::EnumConstantDecl {
				out.push(Const::new(const_decl));
			}
			EntityVisitResult::Continue
		});
		out
	}

	/// True if this enum is bitfield, e.g. intended to be used as a combination of flags.
	pub fn bitfield(&self) -> EnumBitfield {
		let name = self.cpp_name(CppNameStyle::Reference);
		if name.ends_with("Flags") || name.ends_with("FLAGS") || name.ends_with("Settings") {
			self
				.gen_env
				.settings
				.enum_bitfield_override
				.get(name.as_ref())
				.copied()
				.unwrap_or_else(|| {
					let mut has_zero = false;
					let mut var_count = 0;
					let not_bitfield = self.as_typedefed().unwrap_or(self.entity).visit_children(|const_decl, _| {
						if const_decl.get_kind() != EntityKind::EnumConstantDecl {
							return EntityVisitResult::Continue;
						}
						if let Some(val) = const_decl
							.get_enum_constant_value()
							.and_then(|(val, _)| u64::try_from(val).ok())
						{
							var_count += 1;
							if val == 0 {
								has_zero = true;
								EntityVisitResult::Continue
							} else if val.is_power_of_two() {
								EntityVisitResult::Continue
							} else {
								EntityVisitResult::Break
							}
						} else {
							EntityVisitResult::Break
						}
					});
					if not_bitfield {
						EnumBitfield::NotBitfield
					} else if has_zero {
						if var_count <= 2 {
							// enum with 0 and one/no other option is self-exclusive, no point in bitfield machinery
							EnumBitfield::NotBitfield
						} else {
							EnumBitfield::BitfieldWithZero
						}
					} else if var_count <= 1 {
						EnumBitfield::NotBitfield
					} else {
						EnumBitfield::BitfieldWithoutZero
					}
				})
		} else {
			EnumBitfield::NotBitfield
		}
	}
}

impl<'tu> EntityElement<'tu> for Enum<'tu, '_> {
	fn entity(&self) -> Entity<'tu> {
		self.entity
	}
}

impl Element for Enum<'_, '_> {
	fn exclude_kind(&self) -> ExcludeKind {
		DefaultElement::exclude_kind(self).with_is_excluded(|| self.as_typedefed().is_some())
	}

	fn is_system(&self) -> bool {
		DefaultElement::is_system(self.entity)
	}

	fn is_public(&self) -> bool {
		DefaultElement::is_public(self.entity)
	}

	fn doc_comment(&self) -> Cow<'_, str> {
		self.entity.doc_comment()
	}

	fn cpp_namespace(&self) -> Cow<'_, str> {
		if let Some(custom_fullname) = &self.custom_fullname {
			custom_fullname.namespace().into()
		} else {
			DefaultElement::cpp_namespace(self.entity).into()
		}
	}

	fn cpp_name(&self, style: CppNameStyle) -> Cow<'_, str> {
		if let Some(custom_fullname) = self.custom_fullname.as_deref() {
			custom_fullname.cpp_name_from_fullname(style).into()
		} else {
			DefaultElement::cpp_name(self, self.entity, style)
		}
	}
}

impl PartialEq for Enum<'_, '_> {
	fn eq(&self, other: &Self) -> bool {
		self.entity == other.entity && self.custom_fullname == other.custom_fullname
	}
}

impl<'me> NameDebug<'me> for &'me Enum<'_, '_> {
	fn file_line_name(self) -> LocationName<'me> {
		self.entity.file_line_name()
	}
}

impl fmt::Debug for Enum<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut debug_struct = f.debug_struct("Enum");
		self
			.update_debug_struct(&mut debug_struct)
			.field("consts", &self.consts())
			.finish()
	}
}
