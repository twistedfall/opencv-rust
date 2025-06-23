use std::borrow::Cow;
use std::rc::Rc;

use super::FuncKind;
use crate::debug::DefinitionLocation;
use crate::field::Field;
use crate::func::ReturnKind;
use crate::type_ref::{Constness, TypeRef, TypeRefDesc};
use crate::{Class, Func, FuncTypeHint, SupportedModule};

#[derive(Clone, Debug)]
pub struct FuncDesc<'tu, 'ge> {
	pub kind: FuncKind<'tu, 'ge>,
	pub type_hint: FuncTypeHint,
	pub constness: Constness,
	// fixme, this should be just a `is_infallible` property, but `method_get` in `Vector` forces `InfallibleViaArg`
	pub return_kind: ReturnKind,
	pub cpp_name: Rc<str>,
	pub rust_custom_leafname: Option<Rc<str>>,
	pub rust_module: SupportedModule,
	pub doc_comment: Rc<str>,
	pub def_loc: DefinitionLocation,
	pub rust_generic_decls: Rc<[(String, String)]>,
	pub arguments: Rc<[Field<'tu, 'ge>]>,
	pub return_type_ref: TypeRef<'tu, 'ge>,
	pub cpp_body: FuncCppBody,
	pub rust_body: FuncRustBody,
	pub rust_extern_definition: FuncRustExtern,
	pub cfg_attr: Rc<Option<(String, String)>>,
}

impl<'tu, 'ge> FuncDesc<'tu, 'ge> {
	pub fn new(
		kind: FuncKind<'tu, 'ge>,
		constness: Constness,
		return_kind: ReturnKind,
		cpp_name: impl Into<Rc<str>>,
		rust_module: SupportedModule,
		arguments: impl Into<Rc<[Field<'tu, 'ge>]>>,
		return_type_ref: TypeRef<'tu, 'ge>,
	) -> Self {
		Self {
			kind,
			type_hint: FuncTypeHint::None,
			constness,
			return_kind,
			cpp_name: cpp_name.into(),
			rust_custom_leafname: None,
			rust_module,
			doc_comment: "".into(),
			def_loc: DefinitionLocation::Generated,
			rust_generic_decls: Rc::new([]),
			arguments: arguments.into(),
			return_type_ref,
			cpp_body: FuncCppBody::Auto,
			rust_body: FuncRustBody::Auto,
			rust_extern_definition: FuncRustExtern::Auto,
			cfg_attr: Rc::new(None),
		}
	}

	#[inline]
	pub fn rust_custom_leafname(mut self, rust_custom_leafname: impl Into<Rc<str>>) -> Self {
		self.rust_custom_leafname = Some(rust_custom_leafname.into());
		self
	}

	#[inline]
	pub fn maybe_rust_custom_leafname(mut self, rust_custom_leafname: Option<impl Into<Rc<str>>>) -> Self {
		self.rust_custom_leafname = rust_custom_leafname.map(|name| name.into());
		self
	}

	#[inline]
	pub fn doc_comment(mut self, doc_comment: impl Into<Rc<str>>) -> Self {
		self.doc_comment = doc_comment.into();
		self
	}

	#[inline]
	pub fn def_loc(mut self, def_loc: DefinitionLocation) -> Self {
		self.def_loc = def_loc;
		self
	}

	#[inline]
	pub fn rust_generic_decls(mut self, rust_generic_decls: impl Into<Rc<[(String, String)]>>) -> Self {
		self.rust_generic_decls = rust_generic_decls.into();
		self
	}

	#[inline]
	pub fn cpp_body(mut self, cpp_body: FuncCppBody) -> Self {
		self.cpp_body = cpp_body;
		self
	}

	#[inline]
	pub fn rust_body(mut self, rust_body: FuncRustBody) -> Self {
		self.rust_body = rust_body;
		self
	}

	#[inline]
	pub fn rust_extern_definition(mut self, rust_extern_definition: FuncRustExtern) -> Self {
		self.rust_extern_definition = rust_extern_definition;
		self
	}

	#[inline]
	pub fn cfg_attr(mut self, cfg_attr: Option<(String, String)>) -> Self {
		self.cfg_attr = cfg_attr.into();
		self
	}

	pub fn method_delete(class_desc: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
		Func::new_desc(
			FuncDesc::new(
				FuncKind::InstanceMethod(class_desc),
				Constness::Mut,
				ReturnKind::InfallibleNaked,
				"delete",
				SupportedModule::Core,
				[],
				TypeRefDesc::void(),
			)
			.cpp_body(FuncCppBody::ManualCall("delete instance".into())),
		)
	}
}

#[derive(Clone, Debug)]
pub enum FuncCppBody {
	/// Handle the call automatically based on the function context, usually just forwards to the corresponding OpenCV function
	Auto,
	/// Specify manual call, use the automatic return handling (e.g. `Mat ret = <manual_call>`)
	ManualCall(Cow<'static, str>),
	/// Specify manual function call with the return
	ManualCallReturn(Cow<'static, str>),
	/// Skip generating C++ part of the function
	Absent,
}

#[derive(Clone, Copy, Debug)]
pub enum FuncRustExtern {
	/// Generate the automatic extern definition
	Auto,
	/// Don't generate any extern definition
	Absent,
}

#[derive(Clone, Debug)]
pub enum FuncRustBody {
	/// Handle the call automatically based on the function context, usually just calls the corresponding OpenCV function
	Auto,
	/// Specify manual call, use the automatic return handling
	ManualCall(Cow<'static, str>),
	/// Specify manual call, use the automatic return handling
	ManualCallReturn(Cow<'static, str>),
}
