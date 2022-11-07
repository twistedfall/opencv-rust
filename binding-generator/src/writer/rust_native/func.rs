use std::borrow::Cow;
use std::iter;

use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::func::Kind;
use crate::type_ref::{ConstnessOverride, CppNameStyle, Dir, ExternDir, FishStyle, NameStyle, StrEnc, StrType};
use crate::writer::rust_native::func_desc::{pre_post_arg_handle, CppFuncDesc, FuncDescCppCall, FuncDescKind};
use crate::{get_debug, settings, CompiledInterpolation, Element, Field, Func, StrExt, TypeRef};

use super::type_ref::TypeRefExt;
use super::RustNativeGeneratedElement;

pub fn disambiguate_single_name(name: &str) -> impl Iterator<Item = String> + '_ {
	let mut i = 0;
	iter::from_fn(move || {
		let out = format!("{}{}", name, disambiguate_num(i));
		i += 1;
		Some(out)
	})
}

pub fn disambiguate_num(counter: usize) -> String {
	match counter {
		0 => "".to_string(),
		n => format!("_{}", n),
	}
}

fn gen_rust_with_name(f: &Func, name: &str, opencv_version: &str) -> String {
	static TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/func/rust.tpl.rs").compile_interpolation());

	let args = Field::rust_disambiguate_names(f.arguments()).collect::<Vec<_>>();
	let kind = f.kind();
	let as_instance_method = kind.as_instance_method();
	let method_constness = f.constness();
	let is_infallible = f.is_infallible();
	let mut decl_args = Vec::with_capacity(args.len());
	let mut call_args = Vec::with_capacity(args.len());
	let mut forward_args = Vec::with_capacity(args.len());
	let mut pre_call_args = Vec::with_capacity(args.len());
	let mut post_call_args = Vec::with_capacity(args.len());
	if let Some(cls) = as_instance_method {
		decl_args.push(cls.type_ref().rust_self_func_decl(method_constness));
		call_args.push(cls.type_ref().rust_self_func_call(method_constness));
	}
	let mut callback_arg_name: Option<String> = None;
	for (name, arg) in args {
		let type_ref = arg.type_ref();
		if arg.is_user_data() {
			pre_post_arg_handle(
				type_ref.rust_userdata_pre_call(
					&name,
					callback_arg_name.as_deref().expect("Can't get name of the callback arg"),
				),
				&mut pre_call_args,
			);
		} else {
			if type_ref.as_function().is_some() {
				callback_arg_name = Some(name.clone());
			}
			if !arg.as_slice_len().is_some() {
				decl_args.push(type_ref.rust_arg_func_decl(&name));
			}
			pre_post_arg_handle(type_ref.rust_arg_pre_call(&name, is_infallible), &mut pre_call_args);
		}
		if let Some((slice_arg, len_div)) = arg.as_slice_len() {
			let slice_call = if len_div > 1 {
				format!(
					"({slice_arg}.len() / {len_div}) as _",
					slice_arg = slice_arg,
					len_div = len_div
				)
			} else {
				format!("{slice_arg}.len() as _", slice_arg = slice_arg)
			};
			call_args.push(slice_call);
		} else {
			call_args.push(type_ref.rust_arg_func_call(&name, ConstnessOverride::No));
		}
		forward_args.push(type_ref.rust_arg_forward(&name));
		pre_post_arg_handle(type_ref.rust_arg_post_call(&name, is_infallible), &mut post_call_args);
	}
	let naked_return = f.is_naked_return();
	if !naked_return {
		pre_call_args.push("return_send!(via ocvrs_return);".to_string());
		call_args.push("ocvrs_return.as_mut_ptr()".to_string());
	}

	let doc_comment = f.rendered_doc_comment(opencv_version);
	let debug = get_debug(f);
	let visibility = if let Some(cls) = as_instance_method {
		if cls.is_trait() {
			""
		} else {
			"pub "
		}
	} else {
		"pub "
	};
	let identifier = f.identifier();
	let is_safe = !f.is_unsafe();
	let is_static_func = matches!(f.kind(), Kind::StaticMethod(..) | Kind::Function);
	let return_type = f.return_type();
	let return_type_func_decl = if is_infallible {
		return_type.rust_return(FishStyle::No, is_static_func)
	} else {
		format!("Result<{}>", return_type.rust_return(FishStyle::No, is_static_func)).into()
	};
	let return_type_func_decl = if return_type_func_decl == "()" {
		Cow::Borrowed("")
	} else {
		format!(" -> {}", return_type_func_decl).into()
	};
	if is_infallible {
		post_call_args.push("ret".to_string());
	} else {
		post_call_args.push("Ok(ret)".to_string());
	}
	let ret_receive = if naked_return {
		"let ret = "
	} else {
		""
	};
	let mut ret_convert = Vec::with_capacity(3);
	if !naked_return {
		let spec = if is_safe {
			"return_receive!(unsafe ocvrs_return => ret);"
		} else {
			"return_receive!(ocvrs_return => ret);"
		};
		ret_convert.push(Cow::Borrowed(spec));
	}
	if !is_infallible {
		ret_convert.push("let ret = ret.into_result()?;".into())
	}
	let ret_map = rust_return_map(&return_type, "ret", is_safe, is_static_func, is_infallible);
	if !ret_map.is_empty() {
		ret_convert.push(format!("let ret = {};", ret_map).into());
	}
	let mut attributes = String::new();
	if let Some(attrs) = settings::FUNC_CFG_ATTR.get(identifier.as_ref()) {
		attributes = format!("#[cfg({})]", attrs.0);
	}
	if f.is_no_discard() {
		attributes.push_str("#[must_use]");
	}

	let decl_args = decl_args.join(", ");
	let pre_call_args = pre_call_args.join("\n");
	let call_args = call_args.join(", ");
	let forward_args = forward_args.join(", ");
	let post_call_args = post_call_args.join("\n");
	let ret_convert = ret_convert.join("\n");
	let tpl = if let Some(tpl) = settings::FUNC_MANUAL.get(identifier.as_ref()) {
		tpl
	} else {
		&TPL
	};
	tpl.interpolate(&hashmap! {
		"doc_comment" => doc_comment.as_str(),
		"debug" => &debug,
		"attributes" => &attributes,
		"visibility" => visibility,
		"unsafety_decl" => if is_safe { "" } else { "unsafe " },
		"name" => name,
		"generic_decl" => "",
		"decl_args" => &decl_args,
		"rv_rust_full" => return_type_func_decl.as_ref(),
		"pre_call_args" => &pre_call_args,
		"unsafety_call" => if is_safe { "unsafe " } else { "" },
		"identifier" => identifier.as_ref(),
		"call_args" => &call_args,
		"forward_args" => &forward_args,
		"ret_receive" => ret_receive,
		"ret_convert" => ret_convert.as_ref(),
		"post_call_args" => &post_call_args,
	})
}

fn rust_return_map(
	return_type: &TypeRef,
	ret_name: &str,
	is_safe_context: bool,
	is_static_func: bool,
	is_infallible: bool,
) -> Cow<'static, str> {
	let unsafety_call = if is_safe_context {
		"unsafe "
	} else {
		""
	};
	if return_type.as_string().is_some() || return_type.is_extern_by_ptr() {
		format!(
			"{unsafety_call}{{ {typ}::opencv_from_extern({ret_name}) }}",
			unsafety_call = unsafety_call,
			typ = return_type.rust_return(FishStyle::Turbo, is_static_func),
			ret_name = ret_name,
		)
		.into()
	} else if return_type.as_pointer().map_or(false, |i| !i.is_void()) && !return_type.is_pass_by_ptr()
		|| return_type.as_fixed_array().is_some()
	{
		let ptr_call = if return_type.constness().is_const() {
			"as_ref"
		} else {
			"as_mut"
		};
		let error_handling = if is_infallible {
			".expect(\"Function returned null pointer\")"
		} else {
			".ok_or_else(|| Error::new(core::StsNullPtr, \"Function returned null pointer\"))?"
		};
		format!(
			"{unsafety_call}{{ {ret_name}.{ptr_call}() }}{error_handling}",
			unsafety_call = unsafety_call,
			ret_name = ret_name,
			ptr_call = ptr_call,
			error_handling = error_handling,
		)
		.into()
	} else {
		"".into()
	}
}

pub fn cpp_return_map<'f>(return_type: &TypeRef, name: &'f str, is_constructor: bool) -> (Cow<'f, str>, bool) {
	if return_type.is_void() {
		("".into(), false)
	} else if return_type.is_extern_by_ptr() && !is_constructor {
		let out = return_type.source().as_class().filter(|cls| cls.is_abstract()).map_or_else(
			|| {
				format!(
					"new {typ}({name})",
					typ = return_type.cpp_name(CppNameStyle::Reference),
					name = name
				)
				.into()
			},
			|_| name.into(),
		);
		(out, false)
	} else if let Some(Dir::In(string_type)) | Some(Dir::Out(string_type)) = return_type.as_string() {
		let str_mk = match string_type {
			StrType::StdString(StrEnc::Text) | StrType::CvString(StrEnc::Text) => {
				format!("ocvrs_create_string({name}.c_str())", name = name).into()
			}
			StrType::CharPtr => format!("ocvrs_create_string({name})", name = name).into(),
			StrType::StdString(StrEnc::Binary) => {
				format!("ocvrs_create_byte_string({name}.data(), {name}.size())", name = name).into()
			}
			StrType::CvString(StrEnc::Binary) => {
				format!("ocvrs_create_byte_string({name}.begin(), {name}.size())", name = name).into()
			}
		};
		(str_mk, false)
	} else {
		(name.into(), return_type.as_fixed_array().is_some())
	}
}

pub fn cpp_return_handle(
	ret: &str,
	ret_cast: Option<Cow<str>>,
	ocv_ret_name: &str,
	is_naked_return: bool,
	is_infallible: bool,
) -> Cow<'static, str> {
	if is_naked_return {
		if ret.is_empty() {
			"".into()
		} else {
			let cast = if let Some(ret_type) = ret_cast {
				format!("({typ})", typ = ret_type.as_ref())
			} else {
				"".to_string()
			};
			format!("return {cast}{ret};", cast = cast, ret = ret).into()
		}
	} else if is_infallible {
		if ret.is_empty() {
			"".into()
		} else {
			format!("*{name} = {typ};", name = ocv_ret_name, typ = ret).into()
		}
	} else if ret.is_empty() {
		format!("Ok({name});", name = ocv_ret_name).into()
	} else {
		let cast = if let Some(ret_type) = ret_cast {
			format!("<{typ}>", typ = ret_type.as_ref())
		} else {
			"".to_string()
		};
		format!("Ok{cast}({ret}, {name});", cast = cast, ret = ret, name = ocv_ret_name).into()
	}
}

impl RustNativeGeneratedElement for Func<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module(), self.rust_name(NameStyle::decl()))
	}

	fn gen_rust(&self, opencv_version: &str) -> String {
		let name = if self.is_clone() {
			"try_clone".into()
		} else if let Some(name_hint) = self.name_hint() {
			name_hint.into()
		} else {
			self.rust_leafname(FishStyle::No)
		};
		gen_rust_with_name(self, name.as_ref(), opencv_version)
	}

	fn gen_rust_exports(&self) -> String {
		static TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/func/rust_extern.tpl.rs").compile_interpolation());

		let identifier = self.identifier();

		if settings::FUNC_MANUAL.contains_key(identifier.as_ref()) {
			return "".to_string();
		}

		let mut attributes = String::new();
		if let Some(attrs) = settings::FUNC_CFG_ATTR.get(identifier.as_ref()) {
			attributes = format!("#[cfg({})]", attrs.0);
		}
		let mut args = vec![];
		if let Some(cls) = self.kind().as_instance_method() {
			args.push(cls.type_ref().rust_extern_self_func_decl(self.constness()));
		}
		for (name, arg) in Field::rust_disambiguate_names(self.arguments()) {
			args.push(arg.type_ref().rust_extern_arg_func_decl(&name, ConstnessOverride::No))
		}

		let naked_return = self.is_naked_return();
		let is_infallible = self.is_infallible();
		let return_type = self.return_type();
		let return_wrapper_type = if is_infallible {
			return_type.rust_extern(ExternDir::FromCpp)
		} else {
			return_type.rust_extern_return_fallible()
		};
		if !naked_return {
			let ret_name = "ocvrs_return";
			args.push(format!("{name}: *mut {typ}", name = ret_name, typ = return_wrapper_type));
		}
		let return_wrapper_type = if return_type.is_void() || !naked_return {
			"".into()
		} else {
			format!(" -> {}", return_wrapper_type).into()
		};
		TPL.interpolate(&hashmap! {
			"attributes" => attributes.into(),
			"debug" => get_debug(self).into(),
			"identifier" => identifier,
			"args" => args.join(", ").into(),
			"return_type" => return_wrapper_type,
		})
	}

	fn gen_cpp(&self) -> String {
		CppFuncDesc::from(self).gen_cpp()
	}
}

impl<'tu, 'ge, 'r> From<&'r Func<'tu, 'ge>> for CppFuncDesc<'tu, 'ge, 'r> {
	fn from(f: &'r Func<'tu, 'ge>) -> Self {
		let extern_name = f.identifier();
		let constness = f.constness();
		let is_infallible = f.is_infallible();
		let is_naked_return = f.is_naked_return();
		let return_type = f.return_type();
		let kind = FuncDescKind::from(f.kind());
		let type_hint = f.type_hint();
		let name_decl = f.cpp_name(CppNameStyle::Declaration);
		let name_ref = f.cpp_name(CppNameStyle::Reference);
		let debug = get_debug(f);
		let arguments = f
			.arguments()
			.into_iter()
			.map(|arg| (arg.cpp_name(CppNameStyle::Declaration).into_owned(), arg.type_ref()))
			.collect();

		Self {
			extern_name,
			constness,
			is_infallible,
			is_naked_return,
			return_type,
			kind,
			type_hint,
			call: FuncDescCppCall::Auto { name_decl, name_ref },
			debug,
			arguments,
		}
	}
}
