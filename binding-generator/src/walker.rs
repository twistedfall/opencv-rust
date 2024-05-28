use std::ops::ControlFlow;
use std::path::Path;

use clang::{Entity, EntityKind, EntityVisitResult};

use crate::entity::ControlFlowExt;

/// Visitor to be used in conjunction with [EntityWalker]
pub trait EntityWalkerVisitor<'tu> {
	/// Check whether the visitor is interested in entities from the specified file
	#[allow(unused)]
	fn wants_file(&mut self, path: &Path) -> bool {
		true
	}

	/// Pass a supported [Entity] to the visitor
	fn visit_entity(&mut self, entity: Entity<'tu>) -> ControlFlow<()>;
}

/// Trait to recursively visit every clang [Entity] supported by the OpenCV binding generator
pub trait EntityWalkerExt<'tu> {
	/// Recursively visits every clang [Entity] supported by the OpenCV binding generator starting from the `self`
	fn walk_opencv_entities(self, visitor: impl EntityWalkerVisitor<'tu>);
}

impl<'tu> EntityWalkerExt<'tu> for Entity<'tu> {
	fn walk_opencv_entities(self, mut visitor: impl EntityWalkerVisitor<'tu>) {
		self.visit_children(|root_decl, _| {
			let visitor_wants = root_decl
				.get_location()
				.and_then(|loc| loc.get_file_location().file.map(|f| f.get_path()))
				.filter(|file| visitor.wants_file(file))
				.is_some();
			if visitor_wants {
				match root_decl.get_kind() {
					EntityKind::Namespace if root_decl.get_name().map_or(false, |name| name.starts_with("cv")) => {
						visit_cv_namespace(root_decl, &mut visitor)
					}
					EntityKind::MacroDefinition | EntityKind::MacroExpansion | EntityKind::EnumDecl | EntityKind::TypedefDecl => {
						visitor.visit_entity(root_decl)
					}
					EntityKind::Namespace
					| EntityKind::FunctionDecl
					| EntityKind::InclusionDirective
					| EntityKind::UnionDecl
					| EntityKind::UnexposedDecl
					| EntityKind::StructDecl
					| EntityKind::Constructor
					| EntityKind::Method
					| EntityKind::FunctionTemplate
					| EntityKind::ConversionFunction
					| EntityKind::ClassTemplate
					| EntityKind::ClassDecl
					| EntityKind::Destructor
					| EntityKind::LinkageSpec
					| EntityKind::VarDecl => ControlFlow::Continue(()),
					_ => {
						unreachable!("Unsupported decl for file: {:#?}", root_decl)
					}
				}
				.into_entity_visit_result()
			} else {
				EntityVisitResult::Continue
			}
		});
	}
}

fn visit_cv_namespace<'tu>(ns: Entity<'tu>, visitor: &mut impl EntityWalkerVisitor<'tu>) -> ControlFlow<()> {
	let is_interrupted = ns.visit_children(|decl, _| {
		match decl.get_kind() {
			EntityKind::Namespace => visit_cv_namespace(decl, visitor),
			EntityKind::ClassDecl
			| EntityKind::ClassTemplate
			| EntityKind::ClassTemplatePartialSpecialization
			| EntityKind::StructDecl
			| EntityKind::EnumDecl
			| EntityKind::FunctionDecl
			| EntityKind::TypedefDecl
			| EntityKind::VarDecl
			| EntityKind::TypeAliasDecl => visitor.visit_entity(decl),
			EntityKind::Constructor
			| EntityKind::ConversionFunction
			| EntityKind::Destructor
			| EntityKind::Method
			| EntityKind::UnexposedDecl
			| EntityKind::FunctionTemplate
			| EntityKind::UsingDeclaration
			| EntityKind::UsingDirective
			| EntityKind::TypeAliasTemplateDecl
			| EntityKind::LinkageSpec => {
				/* ignoring */
				ControlFlow::Continue(())
			}
			_ => {
				unreachable!("Unsupported decl for OpenCV namespace: {:#?}", decl)
			}
		}
		.into_entity_visit_result()
	});
	ControlFlow::continue_until(is_interrupted)
}
