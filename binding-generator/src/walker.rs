use std::path::Path;

use clang::{Entity, EntityKind, EntityVisitResult, Type};

use crate::element::main_opencv_module_from_path;

#[allow(unused)]
pub trait EntityWalkerVisitor<'tu> {
	fn wants_file(&mut self, path: &Path) -> bool { true }
	fn visit_resolve_type(&mut self, typ: Type<'tu>) -> bool { true }
	fn visit_entity(&mut self, entity: Entity<'tu>) -> bool;
}

pub struct EntityWalker<'tu> {
	root_entity: Entity<'tu>,
}

impl<'tu> EntityWalker<'tu> {
	pub fn new(root_entity: Entity<'tu>) -> Self {
		Self { root_entity }
	}

	fn visit_resolve_types_namespace(ns: Entity<'tu>, visitor: &mut impl EntityWalkerVisitor<'tu>) -> bool {
		!ns.visit_children(|decl, _| {
			let res = match decl.get_kind() {
				EntityKind::TypedefDecl | EntityKind::TypeAliasDecl => {
					if let Some(typ) = decl.get_typedef_underlying_type() {
						visitor.visit_resolve_type(typ)
					} else {
						true
					}
				},
				_ => {
					true
				}
			};
			if res {
				EntityVisitResult::Continue
			} else {
				EntityVisitResult::Break
			}
		})
	}

	fn visit_cv_namespace(ns: Entity<'tu>, visitor: &mut impl EntityWalkerVisitor<'tu>) -> bool {
		!ns.visit_children(|decl, _| {
			let res = match decl.get_kind() {
				EntityKind::Namespace => {
					Self::visit_cv_namespace(decl, visitor)
				}
				EntityKind::ClassDecl | EntityKind::ClassTemplate | EntityKind::ClassTemplatePartialSpecialization
				| EntityKind::StructDecl | EntityKind::EnumDecl | EntityKind::FunctionDecl
				| EntityKind::TypedefDecl | EntityKind::VarDecl | EntityKind::TypeAliasDecl => {
					visitor.visit_entity(decl)
				}
				EntityKind::Constructor | EntityKind::ConversionFunction | EntityKind::Destructor
				| EntityKind::Method | EntityKind::UnexposedDecl | EntityKind::FunctionTemplate
				| EntityKind::UsingDeclaration | EntityKind::UsingDirective | EntityKind::TypeAliasTemplateDecl => {
					/* ignoring */ true
				}
				_ => {
					unreachable!("Unsupported decl for OpenCV namespace: {:#?}", decl)
				}
			};
			if res {
				EntityVisitResult::Continue
			} else {
				EntityVisitResult::Break
			}
		})
	}

	pub fn walk_opencv_entities(&self, mut visitor: impl EntityWalkerVisitor<'tu>) {
		self.root_entity.visit_children(|root_decl, _| {
			let res = if let Some(loc) = root_decl.get_location() {
				if let Some(file) = loc.get_file_location().file.map(|f| f.get_path()) {
					if visitor.wants_file(&file) {
						match root_decl.get_kind() {
							EntityKind::Namespace => {
								if let Some(name) = root_decl.get_name() {
									if name == "ocvrs_resolve_types" {
										Self::visit_resolve_types_namespace(root_decl, &mut visitor)
									}
									else if name.starts_with("cv") { // + e.g. cvflann, cvv
										// fixme: it should be possible to use opencv_module_from_path here,
										// but it breaks module documentation generation
										if main_opencv_module_from_path(&file).is_some() {
											visitor.visit_entity(root_decl);
										}
										Self::visit_cv_namespace(root_decl, &mut visitor)
									} else {
										true
									}
								} else {
									true
								}
							}
							EntityKind::MacroDefinition | EntityKind::MacroExpansion | EntityKind::EnumDecl
							| EntityKind::TypedefDecl => {
								visitor.visit_entity(root_decl)
							}
							EntityKind::FunctionDecl | EntityKind::InclusionDirective
							| EntityKind::UnionDecl | EntityKind::UnexposedDecl | EntityKind::StructDecl
							| EntityKind::Constructor | EntityKind::Method | EntityKind::FunctionTemplate
							| EntityKind::ConversionFunction | EntityKind::ClassTemplate | EntityKind::ClassDecl
							| EntityKind::Destructor | EntityKind::VarDecl => {
								true
							}
							_ => {
								unreachable!("Unsupported decl for file: {:#?}", root_decl)
							}
						}
					} else {
						true
					}
				} else {
					true
				}
			} else {
				true
			};
			if res {
				EntityVisitResult::Continue
			} else {
				EntityVisitResult::Break
			}
		});
	}
}
