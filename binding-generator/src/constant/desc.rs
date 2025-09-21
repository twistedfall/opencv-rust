use std::rc::Rc;

use super::Value;
use crate::SupportedModule;

#[derive(Debug)]
pub struct ConstDesc {
	pub cpp_fullname: Rc<str>,
	pub rust_module: SupportedModule,
	pub doc_comment: Rc<str>,
	pub value: Rc<Value>,
}

impl ConstDesc {
	pub fn new(cpp_fullname: impl Into<Rc<str>>, rust_module: SupportedModule, value: Value) -> Self {
		Self {
			cpp_fullname: cpp_fullname.into(),
			rust_module,
			doc_comment: "".into(),
			value: Rc::new(value),
		}
	}

	#[inline]
	pub fn doc_comment(mut self, doc_comment: impl Into<Rc<str>>) -> Self {
		self.doc_comment = doc_comment.into();
		self
	}
}
