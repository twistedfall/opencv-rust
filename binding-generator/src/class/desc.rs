use std::rc::Rc;

use crate::element::ExcludeKind;

use super::{Class, ClassKind, TemplateKind};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClassDesc<'tu, 'ge> {
	pub kind: ClassKind,
	pub is_abstract: bool,
	pub is_system: bool,
	pub is_public: bool,
	pub exclude_kind: ExcludeKind,
	pub template_kind: TemplateKind<'tu, 'ge>,
	pub bases: Rc<[Class<'tu, 'ge>]>,
	pub cpp_fullname: Rc<str>,
	pub rust_fullname: Rc<str>,
}

impl<'tu, 'ge> ClassDesc<'tu, 'ge> {
	pub fn boxed(cpp_fullname: impl Into<Rc<str>>, rust_fullname: impl Into<Rc<str>>) -> Self {
		Self {
			kind: ClassKind::Boxed,
			is_abstract: false,
			is_system: false,
			is_public: true,
			exclude_kind: ExcludeKind::Included,
			template_kind: TemplateKind::No,
			bases: Rc::new([]),
			cpp_fullname: cpp_fullname.into(),
			rust_fullname: rust_fullname.into(),
		}
	}
}
