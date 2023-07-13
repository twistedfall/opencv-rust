use std::borrow::Cow;
use std::fmt;

use clang::{Entity, EntityKind, EntityVisitResult, StorageClass};

use crate::type_ref::CppNameStyle;
use crate::{DefaultElement, Element, EntityElement};

impl<'tu> EntityElement<'tu> for Entity<'tu> {
	fn entity(&self) -> Entity<'tu> {
		*self
	}
}

impl Element for Entity<'_> {
	fn is_system(&self) -> bool {
		DefaultElement::is_system(self.entity())
	}

	fn is_public(&self) -> bool {
		DefaultElement::is_public(self.entity())
	}

	fn doc_comment(&self) -> Cow<str> {
		self.get_comment().unwrap_or_default().into()
	}

	fn cpp_namespace(&self) -> Cow<str> {
		DefaultElement::cpp_namespace(self.entity()).into()
	}

	fn cpp_name(&self, style: CppNameStyle) -> Cow<str> {
		DefaultElement::cpp_name(self, self.entity(), style)
	}
}

#[derive(Copy, Clone)]
pub enum WalkAction {
	Continue,
	Interrupt,
}

impl WalkAction {
	pub fn continue_until(condition: bool) -> Self {
		if condition {
			WalkAction::Interrupt
		} else {
			WalkAction::Continue
		}
	}
}

impl From<WalkAction> for EntityVisitResult {
	fn from(action: WalkAction) -> Self {
		match action {
			WalkAction::Continue => Self::Continue,
			WalkAction::Interrupt => Self::Break,
		}
	}
}

#[derive(Copy, Clone)]
pub enum WalkResult {
	Completed,
	Interrupted,
}

impl WalkResult {
	pub fn is_interrupted(self) -> bool {
		matches!(self, WalkResult::Interrupted)
	}
}

pub trait EntityExt<'tu> {
	fn walk_children_while(&self, predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult;
	fn walk_bases_while(&self, predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult;
	fn walk_enums_while(&self, predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult;
	fn walk_classes_while(&self, predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult;
	fn walk_typedefs_while(&self, predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult;
	fn walk_fields_while(&self, predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult;
	fn walk_consts_while(&self, predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult;
	fn walk_methods_while(&self, predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult;
}

impl<'tu> EntityExt<'tu> for Entity<'tu> {
	fn walk_children_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult {
		let res = self.visit_children(|child, _| predicate(child).into());
		if res {
			WalkResult::Interrupted
		} else {
			WalkResult::Completed
		}
	}

	fn walk_bases_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult {
		self.walk_children_while(|child| match child.get_kind() {
			EntityKind::BaseSpecifier => predicate(child),
			_ => WalkAction::Continue,
		})
	}

	fn walk_enums_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult {
		self.walk_children_while(|child| match child.get_kind() {
			EntityKind::EnumDecl => predicate(child),
			_ => WalkAction::Continue,
		})
	}

	fn walk_classes_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult {
		self.walk_children_while(|child| match child.get_kind() {
			EntityKind::ClassDecl | EntityKind::StructDecl => predicate(child),
			_ => WalkAction::Continue,
		})
	}

	fn walk_typedefs_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult {
		self.walk_children_while(|child| match child.get_kind() {
			EntityKind::TypedefDecl | EntityKind::TypeAliasDecl => predicate(child),
			_ => WalkAction::Continue,
		})
	}

	fn walk_fields_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult {
		self.walk_children_while(|child| match child.get_kind() {
			EntityKind::FieldDecl => predicate(child),
			_ => WalkAction::Continue,
		})
	}

	fn walk_consts_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult {
		self.walk_children_while(|child| match child.get_kind() {
			EntityKind::VarDecl => {
				if let Some(StorageClass::Static) = child.get_storage_class() {
					if child.evaluate().is_some() {
						predicate(child)
					} else {
						WalkAction::Continue
					}
				} else {
					panic!("Non-static constant: {child:#?}")
				}
			}
			_ => WalkAction::Continue,
		})
	}

	fn walk_methods_while(&self, mut predicate: impl FnMut(Entity<'tu>) -> WalkAction) -> WalkResult {
		self.walk_children_while(|child| match child.get_kind() {
			EntityKind::Constructor | EntityKind::Method | EntityKind::FunctionTemplate | EntityKind::ConversionFunction => {
				predicate(child)
			}
			_ => WalkAction::Continue,
		})
	}
}

#[allow(unused)]
pub fn dbg_clang_entity(entity: Entity) {
	struct EntityWrapper<'tu>(Entity<'tu>);

	impl fmt::Debug for EntityWrapper<'_> {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			f.debug_struct("Entity")
				.field("evaluate", &self.0.evaluate())
				.field("kind", &self.0.get_kind())
				.field("display_name", &self.0.get_display_name())
				.field("location", &self.0.get_location())
				.field("range", &self.0.get_range())
				.field("accessibility", &self.0.get_accessibility())
				.field("arguments", &self.0.get_arguments())
				.field("availability", &self.0.get_availability())
				.field("bit_field_width", &self.0.get_bit_field_width())
				.field("canonical_entity", &self.0.get_canonical_entity())
				.field("comment", &self.0.get_comment())
				.field("parsed_comment", &self.0.get_parsed_comment())
				.field("comment_brief", &self.0.get_comment_brief())
				.field("comment_range", &self.0.get_comment_range())
				.field("completion_string", &self.0.get_completion_string())
				.field("children", &self.0.get_children())
				.field("definition", &self.0.get_definition())
				.field("enum_constant_value", &self.0.get_enum_constant_value())
				.field("enum_underlying_type", &self.0.get_enum_underlying_type())
				.field("exception_specification", &self.0.get_exception_specification())
				.field("external_symbol", &self.0.get_external_symbol())
				.field("file", &self.0.get_file())
				.field("language", &self.0.get_language())
				.field("lexical_parent", &self.0.get_lexical_parent())
				.field("linkage", &self.0.get_linkage())
				.field("mangled_name", &self.0.get_mangled_name())
				.field("mangled_names", &self.0.get_mangled_names())
				.field("module", &self.0.get_module())
				.field("name", &self.0.get_name())
				.field("name_ranges", &self.0.get_name_ranges())
				.field("offset_of_field", &self.0.get_offset_of_field())
				.field("overloaded_declarations", &self.0.get_overloaded_declarations())
				.field("overridden_methods", &self.0.get_overridden_methods())
				.field("platform_availability", &self.0.get_platform_availability())
				.field("reference", &self.0.get_reference())
				.field("semantic_parent", &self.0.get_semantic_parent())
				.field("storage_class", &self.0.get_storage_class())
				.field("template", &self.0.get_template())
				.field("template_arguments", &self.0.get_template_arguments())
				.field("template_kind", &self.0.get_template_kind())
				.field("tls_kind", &self.0.get_tls_kind())
				.field("translation_unit", &self.0.get_translation_unit())
				.field("type", &self.0.get_type())
				.field("typedef_underlying_type", &self.0.get_typedef_underlying_type())
				.field("usr", &self.0.get_usr())
				.field("visibility", &self.0.get_visibility())
				.field("result_type", &self.0.get_result_type())
				.field("has_attributes", &self.0.has_attributes())
				.field("is_abstract_record", &self.0.is_abstract_record())
				.field("is_anonymous", &self.0.is_anonymous())
				.field("is_bit_field", &self.0.is_bit_field())
				.field("is_builtin_macro", &self.0.is_builtin_macro())
				.field("is_const_method", &self.0.is_const_method())
				.field("is_converting_constructor", &self.0.is_converting_constructor())
				.field("is_copy_constructor", &self.0.is_copy_constructor())
				.field("is_default_constructor", &self.0.is_default_constructor())
				.field("is_defaulted", &self.0.is_defaulted())
				.field("is_definition", &self.0.is_definition())
				.field("is_dynamic_call", &self.0.is_dynamic_call())
				.field("is_function_like_macro", &self.0.is_function_like_macro())
				.field("is_inline_function", &self.0.is_inline_function())
//				.field("is_invalid_declaration", &self.0.is_invalid_declaration())
				.field("is_move_constructor", &self.0.is_move_constructor())
				.field("is_mutable", &self.0.is_mutable())
				.field("is_pure_virtual_method", &self.0.is_pure_virtual_method())
				.field("is_scoped", &self.0.is_scoped())
				.field("is_static_method", &self.0.is_static_method())
				.field("is_variadic", &self.0.is_variadic())
				.field("is_virtual_base", &self.0.is_virtual_base())
				.field("is_virtual_method", &self.0.is_virtual_method())
				.field("is_attribute", &self.0.is_attribute())
				.field("is_declaration", &self.0.is_declaration())
				.field("is_expression", &self.0.is_expression())
				.field("is_preprocessing", &self.0.is_preprocessing())
				.field("is_reference", &self.0.is_reference())
				.field("is_statement", &self.0.is_statement())
				.field("is_unexposed", &self.0.is_unexposed())
				.field("is_in_main_file", &self.0.is_in_main_file())
				.field("is_in_system_header", &self.0.is_in_system_header())
				.finish()
		}
	}
	eprintln!("{:#?}", EntityWrapper(entity));
}
