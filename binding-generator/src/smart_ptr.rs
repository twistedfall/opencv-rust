use std::borrow::Cow;
use std::fmt;
use std::rc::Rc;

use clang::Entity;

pub use desc::SmartPtrDesc;

use crate::element::ExcludeKind;
use crate::type_ref::{CppNameStyle, TypeRef, TypeRefDesc, TypeRefKind};
use crate::{DefaultElement, Element, GeneratedType, GeneratorEnv, StrExt};

mod desc;

#[derive(Clone)]
pub enum SmartPtr<'tu, 'ge> {
	Clang {
		entity: Entity<'tu>,
		gen_env: &'ge GeneratorEnv<'tu>,
	},
	Desc(Rc<SmartPtrDesc<'tu, 'ge>>),
}

impl<'tu, 'ge> SmartPtr<'tu, 'ge> {
	pub fn new(entity: Entity<'tu>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self::Clang { entity, gen_env }
	}

	pub fn new_desc(desc: SmartPtrDesc<'tu, 'ge>) -> Self {
		Self::Desc(Rc::new(desc))
	}

	pub fn gen_env(&self) -> &'ge GeneratorEnv<'tu> {
		match self {
			&Self::Clang { gen_env, .. } => gen_env,
			Self::Desc(desc) => desc.gen_env,
		}
	}

	pub fn type_ref(&self) -> TypeRef<'tu, 'ge> {
		match self {
			&Self::Clang { entity, gen_env, .. } => TypeRef::new(entity.get_type().expect("Can't get smart pointer type"), gen_env),
			Self::Desc(_) => TypeRef::new_desc(TypeRefDesc::new(TypeRefKind::SmartPtr(self.clone()))),
		}
	}

	pub fn pointee(&self) -> TypeRef<'tu, 'ge> {
		match self {
			&Self::Clang { .. } => self
				.type_ref()
				.template_specialization_args()
				.iter()
				.find_map(|arg| arg.as_typename())
				.cloned()
				.expect("Smart pointer template argument list is empty"),
			Self::Desc(desc) => desc.pointee_type_ref.clone(),
		}
	}

	pub fn generated_types(&self) -> Vec<GeneratedType<'tu, 'ge>> {
		if let Some(typedef) = self.pointee().as_typedef() {
			typedef.generated_types()
		} else {
			vec![]
		}
	}
}

impl Element for SmartPtr<'_, '_> {
	fn exclude_kind(&self) -> ExcludeKind {
		DefaultElement::exclude_kind(self).with_exclude_kind(|| self.pointee().exclude_kind())
	}

	fn is_system(&self) -> bool {
		match self {
			&Self::Clang { entity, .. } => DefaultElement::is_system(entity),
			Self::Desc(_) => false,
		}
	}

	fn is_public(&self) -> bool {
		match self {
			&Self::Clang { entity, .. } => DefaultElement::is_public(entity),
			Self::Desc(_) => false,
		}
	}

	fn doc_comment(&self) -> Cow<str> {
		match self {
			&Self::Clang { entity, .. } => entity.get_comment().unwrap_or_default().into(),
			Self::Desc(_) => "".into(),
		}
	}

	fn cpp_namespace(&self) -> Cow<str> {
		"cv".into()
	}

	fn cpp_name(&self, style: CppNameStyle) -> Cow<str> {
		"cv::Ptr".cpp_name_from_fullname(style).into()
	}
}

impl fmt::Debug for SmartPtr<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut debug_struct = f.debug_struct(match self {
			Self::Clang { .. } => "SmartPtr::Clang",
			Self::Desc(_) => "SmartPtr::Desc",
		});
		self
			.update_debug_struct(&mut debug_struct)
			.field("pointee", &self.pointee())
			.finish()
	}
}
