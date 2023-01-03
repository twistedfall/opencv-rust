use std::borrow::Cow;
use std::fmt::Debug;

use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::func::{Kind, OperatorKind};
use crate::type_ref::{Constness, ConstnessOverride};
use crate::{
	settings, Class, CompiledInterpolation, CppNameStyle, Element, Field, FunctionTypeHint, IteratorExt, NamePool, StrExt,
	StringExt, TypeRef,
};

use super::func::{cpp_return_handle, cpp_return_map, disambiguate_single_name};
use super::type_ref::TypeRefExt;

/// Allows generation of functions without tying them to the real C++ items
pub struct CppFuncDesc<'tu, 'ge, 'r> {
	pub extern_name: Cow<'r, str>,
	pub constness: Constness,
	pub is_infallible: bool,
	pub is_naked_return: bool,
	pub return_type: TypeRef<'tu, 'ge>,
	pub kind: FuncDescKind<'tu, 'ge>,
	pub type_hint: FunctionTypeHint,
	pub call: FuncDescCppCall<'r>,
	pub debug: String,
	pub arguments: Vec<(String, TypeRef<'tu, 'ge>)>,
}

impl<'tu, 'ge> CppFuncDesc<'tu, 'ge, '_> {
	fn cpp_call_invoke(&self) -> String {
		static CALL_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| "{{name}}{{generic}}({{args}})".compile_interpolation());

		static VOID_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| "{{call}};".compile_interpolation());

		static RETURN_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| "{{ret_with_type}} = {{doref}}{{call}};".compile_interpolation());

		static FIELD_READ_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| "{{ret_with_type}} = {{doref}}{{name}};".compile_interpolation());

		static FIELD_WRITE_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| "{{name}} = {{args}};".compile_interpolation());

		static CONSTRUCTOR_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| "{{ret_with_type}}({{args}});".compile_interpolation());

		static CONSTRUCTOR_NO_ARGS_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| "{{ret_with_type}};".compile_interpolation());

		static BOXED_CONSTRUCTOR_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| "{{ret_type}}* ret = new {{ret_type}}({{args}});".compile_interpolation());

		let mut generic = String::new();
		if let Some(spec) = self.type_hint.as_specialized() {
			generic.reserve(64);
			generic.push('<');
			generic.extend_join(spec.values(), ", ");
			generic.push('>');
		}

		let args = cpp_disambiguate_names(self.arguments.clone())
			.map(|(name, arg)| arg.cpp_arg_func_call(name).into_owned())
			.join(", ");

		let ret_type = self.return_type.cpp_name(CppNameStyle::Reference);
		let ret_with_type = self.return_type.cpp_name_ext(CppNameStyle::Reference, "ret", true);
		let doref = if self.return_type.as_fixed_array().is_some() {
			"&"
		} else {
			""
		};

		let mut inter_vars = hashmap! {
			"ret_type" => ret_type,
			"ret_with_type" => ret_with_type,
			"doref" => doref.into(),
			"generic" => generic.into(),
			"args" => args.as_str().into(),
		};

		let call_name = match &self.call {
			FuncDescCppCall::Auto { name_decl, name_ref } => match &self.kind {
				FuncDescKind::Constructor(cls) => cls.cpp_name_ref.clone().into(),
				FuncDescKind::Function
				| FuncDescKind::GenericFunction
				| FuncDescKind::StaticMethod(..)
				| FuncDescKind::FunctionOperator(..) => name_ref.as_ref().into(),
				FuncDescKind::FieldAccessor(cls, fld) if self.type_hint == FunctionTypeHint::FieldSetter => {
					cpp_method_call_name(cls.is_boxed, &fld.cpp_name(CppNameStyle::Declaration)).into()
				}
				FuncDescKind::InstanceMethod(cls)
				| FuncDescKind::FieldAccessor(cls, _)
				| FuncDescKind::GenericInstanceMethod(cls)
				| FuncDescKind::ConversionMethod(cls)
				| FuncDescKind::InstanceOperator(cls, ..) => cpp_method_call_name(cls.is_boxed, name_decl).into(),
			},
			FuncDescCppCall::Manual(_) => "".into(),
		};

		inter_vars.insert("name", call_name);

		let tpl = if let Some(cls) = self.kind.as_constructor() {
			if cls.is_boxed {
				&BOXED_CONSTRUCTOR_TPL
			} else if args.is_empty() {
				&CONSTRUCTOR_NO_ARGS_TPL
			} else {
				&CONSTRUCTOR_TPL
			}
		} else if let FuncDescKind::FieldAccessor(..) = &self.kind {
			if self.type_hint == FunctionTypeHint::FieldSetter {
				&FIELD_WRITE_TPL
			} else {
				&FIELD_READ_TPL
			}
		} else {
			let call = match &self.call {
				FuncDescCppCall::Auto { .. } => &*CALL_TPL,
				FuncDescCppCall::Manual(call) => call,
			}
			.interpolate(&inter_vars);
			inter_vars.insert("call", call.into());
			if self.return_type.is_void() {
				&VOID_TPL
			} else {
				&RETURN_TPL
			}
		};

		tpl.interpolate(&inter_vars)
	}

	pub fn gen_cpp(&self) -> String {
		static TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/func/cpp.tpl.cpp").compile_interpolation());

		if settings::FUNC_MANUAL.contains_key(self.extern_name.as_ref()) {
			return "".to_string();
		}

		// attributes
		let mut attributes_begin = String::new();
		let mut attributes_end = String::new();
		if let Some((_, cpp_attr)) = settings::FUNC_CFG_ATTR.get(self.extern_name.as_ref()) {
			attributes_begin = format!("#if {}", cpp_attr);
			attributes_end = "#endif".to_string();
		}

		// arguments
		let args = cpp_disambiguate_names(self.arguments.clone()).collect::<Vec<_>>();
		let mut decl_args = Vec::with_capacity(args.len());
		let mut pre_call_args = Vec::with_capacity(args.len());
		let mut post_call_args = Vec::with_capacity(args.len());
		let mut cleanup_args = Vec::with_capacity(args.len());
		if let Some(cls) = self.kind.as_instance_method() {
			decl_args.push(cls.cpp_self_func_decl(self.constness));
		}
		for (name, type_ref) in args {
			decl_args.push(type_ref.cpp_arg_func_decl(&name));
			pre_post_arg_handle(type_ref.cpp_arg_pre_call(&name), &mut pre_call_args);
			pre_post_arg_handle(type_ref.cpp_arg_post_call(&name), &mut post_call_args);
			pre_post_arg_handle(type_ref.cpp_arg_cleanup(&name), &mut cleanup_args);
		}

		// return
		let ocv_ret_name = "ocvrs_return";
		let cpp_extern_return = self.return_type.cpp_extern_return(ConstnessOverride::No);
		let ret_full = if self.is_infallible {
			cpp_extern_return.clone()
		} else {
			self.return_type.cpp_extern_return_fallible(ConstnessOverride::No)
		};
		let mut_ret_wrapper_full = if self.is_infallible {
			self.return_type.cpp_extern_return(ConstnessOverride::Mut)
		} else {
			self.return_type.cpp_extern_return_fallible(ConstnessOverride::Mut)
		};
		if !self.is_naked_return {
			decl_args.push(format!("{typ}* {name}", name = ocv_ret_name, typ = mut_ret_wrapper_full));
		}
		let return_spec = if self.is_naked_return {
			Cow::Borrowed(ret_full.as_ref())
		} else {
			"void".into()
		};
		let mut rets = disambiguate_single_name("ret");
		let ret_name = rets.next().expect("Endless iterator returned nothing");
		let (ret, ret_cast) = cpp_return_map(&self.return_type, &ret_name, self.kind.as_constructor().is_some());
		let ret = if cleanup_args.is_empty() {
			ret
		} else {
			let ret_name = rets.next().expect("Endless iterator returned nothing");
			pre_post_arg_handle(
				format!("{typ} {ret} = {expr}", typ = cpp_extern_return, ret = ret_name, expr = ret),
				&mut post_call_args,
			);
			ret_name.into()
		};
		let ret = cpp_return_handle(
			&ret,
			ret_cast.then(|| ret_full.as_ref().into()),
			ocv_ret_name,
			self.is_naked_return,
			self.is_infallible,
		);

		// exception handling
		let func_try = if self.is_infallible {
			""
		} else {
			"try {"
		};
		let catch = if self.is_infallible {
			"".into()
		} else {
			let typ = if mut_ret_wrapper_full.contains(',') {
				format!("OCVRS_TYPE({})", mut_ret_wrapper_full).into()
			} else {
				mut_ret_wrapper_full
			};
			format!("}} OCVRS_CATCH({typ}, {name});", typ = typ, name = ocv_ret_name).into()
		};

		TPL.interpolate(&hashmap! {
			"attributes_begin" => attributes_begin.into(),
			"debug" => self.debug.as_str().into(),
			"return_spec" => return_spec,
			"identifier" => self.extern_name.as_ref().into(),
			"decl_args" => decl_args.join(", ").into(),
			"try" => func_try.into(),
			"pre_call_args" => pre_call_args.join("\n").into(),
			"call" => self.cpp_call_invoke().into(),
			"post_call_args" => post_call_args.join("\n").into(),
			"cleanup_args" => cleanup_args.join("\n").into(),
			"return" => ret,
			"catch" => catch,
			"attributes_end" => attributes_end.into(),
		})
	}
}

#[derive(Debug)]
pub enum FuncDescKind<'tu, 'ge> {
	Function,
	FunctionOperator(OperatorKind),
	Constructor(ClassDesc),
	InstanceMethod(ClassDesc),
	StaticMethod(ClassDesc),
	FieldAccessor(ClassDesc, Field<'tu, 'ge>),
	ConversionMethod(ClassDesc),
	InstanceOperator(ClassDesc, OperatorKind),
	GenericFunction,
	GenericInstanceMethod(ClassDesc),
}

impl<'tu, 'ge> From<Kind<'tu, 'ge>> for FuncDescKind<'tu, 'ge> {
	fn from(kind: Kind<'tu, 'ge>) -> Self {
		match kind {
			Kind::Function => Self::Function,
			Kind::FunctionOperator(op) => Self::FunctionOperator(op),
			Kind::Constructor(cls) => Self::Constructor(ClassDesc::from(&cls)),
			Kind::InstanceMethod(cls) => Self::InstanceMethod(ClassDesc::from(&cls)),
			Kind::StaticMethod(cls) => Self::StaticMethod(ClassDesc::from(&cls)),
			Kind::FieldAccessor(cls, fld) => Self::FieldAccessor(ClassDesc::from(&cls), fld),
			Kind::ConversionMethod(cls) => Self::ConversionMethod(ClassDesc::from(&cls)),
			Kind::InstanceOperator(cls, op) => Self::InstanceOperator(ClassDesc::from(&cls), op),
			Kind::GenericFunction => Self::GenericFunction,
			Kind::GenericInstanceMethod(cls) => Self::GenericInstanceMethod(ClassDesc::from(&cls)),
		}
	}
}

impl<'tu, 'ge> FuncDescKind<'tu, 'ge> {
	pub fn as_instance_method(&self) -> Option<&ClassDesc> {
		match self {
			Self::InstanceMethod(out)
			| Self::FieldAccessor(out, _)
			| Self::GenericInstanceMethod(out)
			| Self::ConversionMethod(out)
			| Self::InstanceOperator(out, ..) => Some(out),
			_ => None,
		}
	}

	pub fn as_constructor(&self) -> Option<&ClassDesc> {
		if let Self::Constructor(out) = self {
			Some(out)
		} else {
			None
		}
	}
}

pub enum FuncDescCppCall<'r> {
	Auto {
		name_decl: Cow<'r, str>,
		name_ref: Cow<'r, str>,
	},
	Manual(CompiledInterpolation<'r>),
}

#[derive(Debug, Clone)]
pub struct ClassDesc {
	pub is_boxed: bool,
	pub cpp_name_ref: String,
}

impl From<&Class<'_, '_>> for ClassDesc {
	fn from(cls: &Class<'_, '_>) -> Self {
		let is_boxed = cls.is_boxed();
		let cpp_name_ref = cls.cpp_name(CppNameStyle::Reference).into_owned();
		Self { is_boxed, cpp_name_ref }
	}
}

impl ClassDesc {
	fn cpp_self_func_decl(&self, method_constness: Constness) -> String {
		let cnst = if method_constness.is_const() {
			"const "
		} else {
			""
		};
		if self.is_boxed {
			format!("{cnst}{typ}* instance", cnst = cnst, typ = self.cpp_name_ref)
		} else {
			format!("{cnst}{typ} instance", cnst = cnst, typ = self.cpp_name_ref)
		}
	}
}

fn cpp_method_call_name(extern_by_ptr: bool, method_name: &str) -> String {
	if extern_by_ptr {
		format!("instance->{name}", name = method_name)
	} else {
		format!("instance.{name}", name = method_name)
	}
}

pub fn pre_post_arg_handle(mut arg: String, args: &mut Vec<String>) {
	if !arg.is_empty() {
		arg.push(';');
		args.push(arg);
	}
}

fn cpp_disambiguate_names<'tu, 'ge>(
	args: impl IntoIterator<Item = (String, TypeRef<'tu, 'ge>)>,
) -> impl Iterator<Item = (String, TypeRef<'tu, 'ge>)>
where
	'tu: 'ge,
{
	let args = args.into_iter();
	let size_hint = args.size_hint();
	NamePool::with_capacity(size_hint.1.unwrap_or(size_hint.0))
		.into_disambiguator(args, |(name, _)| name.into())
		.map(|(new_name, (_, typ))| (new_name, typ))
}
