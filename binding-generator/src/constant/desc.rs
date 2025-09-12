use std::rc::Rc;

use super::Value;
use crate::SupportedModule;

#[derive(Debug)]
pub struct ConstDesc {
	pub cpp_fullname: Rc<str>,
	pub rust_module: SupportedModule,
	pub value: Rc<Value>,
}
