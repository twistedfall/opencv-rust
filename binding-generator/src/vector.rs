use std::borrow::Cow;
use std::fmt;
use std::rc::Rc;

use clang::Type;

pub use desc::VectorDesc;

use crate::element::ExcludeKind;
use crate::type_ref::{CppNameStyle, TemplateArg, TypeRefDesc, TypeRefKind};
use crate::{DefaultElement, Element, GeneratedType, GeneratorEnv, StrExt, TypeRef};

mod desc;

#[derive(Clone)]
pub enum Vector<'tu, 'ge> {
	Clang {
		type_ref: Type<'tu>,
		gen_env: &'ge GeneratorEnv<'tu>,
	},
	Desc(Rc<VectorDesc<'tu, 'ge>>),
}

impl<'tu, 'ge> Vector<'tu, 'ge> {
	pub fn new(type_ref: Type<'tu>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self::Clang { type_ref, gen_env }
	}

	pub fn new_desc(desc: VectorDesc<'tu, 'ge>) -> Self {
		Self::Desc(Rc::new(desc))
	}

	pub fn type_ref(&self) -> TypeRef<'tu, 'ge> {
		match self {
			&Vector::Clang { type_ref, gen_env } => TypeRef::new(type_ref, gen_env),
			Vector::Desc(desc) => TypeRef::new_desc(TypeRefDesc::new(TypeRefKind::StdVector(Vector::Desc(Rc::clone(desc))))),
		}
	}

	pub fn element_type(&self) -> TypeRef<'tu, 'ge> {
		match self {
			Vector::Clang { .. } =>
			{
				#[allow(clippy::unnecessary_to_owned)]
				self
					.type_ref()
					.template_specialization_args()
					.into_owned()
					.into_iter()
					.find_map(TemplateArg::into_typename)
					.expect("vector template argument list is empty")
			}
			Vector::Desc(desc) => desc.element_type.clone(),
		}
	}

	pub fn generated_types(&self) -> Vec<GeneratedType<'tu, 'ge>> {
		self.element_type().generated_types()
	}
}

impl Element for Vector<'_, '_> {
	fn exclude_kind(&self) -> ExcludeKind {
		DefaultElement::exclude_kind(self).with_exclude_kind(|| self.element_type().exclude_kind())
	}

	fn is_system(&self) -> bool {
		true
	}

	fn is_public(&self) -> bool {
		true
	}

	fn doc_comment(&self) -> Cow<str> {
		"".into()
	}

	fn cpp_namespace(&self) -> Cow<str> {
		// force this to be std because on some systems the actual namespace for vector is something like "std::__1"
		"std".into()
	}

	fn cpp_name(&self, style: CppNameStyle) -> Cow<str> {
		"std::vector".cpp_name_from_fullname(style).into()
	}
}

impl PartialEq for Vector<'_, '_> {
	fn eq(&self, other: &Self) -> bool {
		self.element_type() == other.element_type()
	}
}

impl fmt::Debug for Vector<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut debug_struct = f.debug_struct(match self {
			Self::Clang { .. } => "Vector::Clang",
			Self::Desc(_) => "Vector::Desc",
		});
		self
			.update_debug_struct(&mut debug_struct)
			.field("element_type", &self.element_type())
			.finish()
	}
}
