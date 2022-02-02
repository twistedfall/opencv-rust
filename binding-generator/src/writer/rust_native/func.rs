use std::borrow::Cow;

use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::{
	Class,
	CompiledInterpolation,
	Constness,
	ConstnessOverride,
	DefaultElement,
	Element,
	Field,
	Func,
	func::Kind,
	FunctionTypeHint,
	get_debug,
	IteratorExt,
	settings,
	StrExt,
	StringExt,
	type_ref::{Dir, FishStyle, StrEnc, StrType},
	TypeRef,
};

use super::RustNativeGeneratedElement;

fn pre_post_arg_handle(mut arg: String, args: &mut Vec<String>) {
	if !arg.is_empty() {
		arg.push(';');
		args.push(arg);
	}
}

fn gen_rust_with_name(f: &Func, name: &str, opencv_version: &str) -> String {
	static TPL: Lazy<CompiledInterpolation> = Lazy::new(
		|| include_str!("tpl/func/rust.tpl.rs").compile_interpolation()
	);

	let args = Field::rust_disambiguate_names(f.arguments()).collect::<Vec<_>>();
	let as_instance_method = f.as_instance_method();
	let method_constness = f.constness();
	let is_infallible = f.is_infallible();
	let mut decl_args = Vec::with_capacity(args.len());
	let mut call_args = Vec::with_capacity(args.len());
	let mut forward_args = Vec::with_capacity(args.len());
	let mut pre_call_args = Vec::with_capacity(args.len());
	let mut post_call_args = Vec::with_capacity(args.len());
	if let Some(cls) = &as_instance_method {
		decl_args.push(cls.type_ref().rust_self_func_decl(method_constness));
		call_args.push(cls.type_ref().rust_self_func_call(method_constness));
	}
	let mut callback_arg_name: Option<String> = None;
	for (name, arg) in args {
		let type_ref = arg.type_ref();
		if arg.is_user_data() {
			pre_post_arg_handle(
				type_ref.rust_userdata_pre_call(&name, callback_arg_name.as_deref().expect("Can't get name of the callback arg")),
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
				format!("({slice_arg}.len() / {len_div}) as _", slice_arg=slice_arg, len_div=len_div)
			} else {
				format!("{slice_arg}.len() as _", slice_arg=slice_arg)
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
		return_type.rust_return_func_decl(FishStyle::No, is_static_func)
	} else {
		format!("Result<{}>", return_type.rust_return_func_decl(FishStyle::No, is_static_func)).into()
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
	let ret_map = return_type.rust_return_map(is_safe, is_static_func, is_infallible);
	if !ret_map.is_empty() {
		ret_convert.push(ret_map);
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

fn cpp_method_call_name(c: &Class, method_name: &str) -> String {
	if c.is_by_ptr() {
		format!("instance->{name}", name = method_name)
	} else {
		format!("instance.{name}", name = method_name)
	}
}

fn cpp_call_invoke(f: &Func) -> String {
	static VOID_TPL: Lazy<CompiledInterpolation> = Lazy::new(||
		"{{name}}({{args}});".compile_interpolation()
	);

	static NORMAL_TPL: Lazy<CompiledInterpolation> = Lazy::new(||
		"{{ret_type}} = {{doref}}{{name}}{{generic}}({{args}});".compile_interpolation()
	);

	static FIELD_READ_TPL: Lazy<CompiledInterpolation> = Lazy::new(||
		"{{ret_type}} = {{doref}}{{name}};".compile_interpolation()
	);

	static FIELD_WRITE_TPL: Lazy<CompiledInterpolation> = Lazy::new(||
		"{{name}} = {{args}};".compile_interpolation()
	);

	static CONSTRUCTOR_TPL: Lazy<CompiledInterpolation> = Lazy::new(||
		"{{ret_type}} ret({{args}});".compile_interpolation()
	);

	static CONSTRUCTOR_NO_ARGS_TPL: Lazy<CompiledInterpolation> = Lazy::new(||
		"{{ret_type}} ret;".compile_interpolation()
	);

	static BOXED_CONSTRUCTOR_TPL: Lazy<CompiledInterpolation> = Lazy::new(||
		"{{ret_type}}* ret = new {{ret_type}}({{args}});".compile_interpolation()
	);

	let call_name = match f.kind() {
		Kind::Function | Kind::GenericFunction | Kind::StaticMethod(..)
		| Kind::FunctionOperator(..) => {
			f.cpp_fullname()
		}
		Kind::Constructor(class) => {
			class.cpp_fullname().into_owned().into()
		}
		Kind::FieldAccessor(class) if f.type_hint() == FunctionTypeHint::FieldSetter => {
			cpp_method_call_name(&class, DefaultElement::cpp_localname(f).as_ref()).into()
		}
		Kind::InstanceMethod(class) | Kind::FieldAccessor(class) | Kind::GenericInstanceMethod(class)
		| Kind::ConversionMethod(class) | Kind::InstanceOperator(class, ..) => {
			cpp_method_call_name(&class, f.cpp_localname().as_ref()).into()
		}
	};

	let mut generic = String::new();
	if let Some(spec) = f.as_specialized() {
		generic.reserve(64);
		generic.push('<');
		generic.extend_join(spec.values(), ", ");
		generic.push('>');
	}

	let args = Field::cpp_disambiguate_names(f.arguments())
		.map(|(name, arg)| arg.type_ref().cpp_arg_func_call(name).into_owned())
		.join(", ");

	let return_type = f.return_type();
	let tpl = if let Some(cls) = f.as_constructor() {
		if cls.is_by_ptr() {
			&BOXED_CONSTRUCTOR_TPL
		} else if args.is_empty() {
			&CONSTRUCTOR_NO_ARGS_TPL
		} else {
			&CONSTRUCTOR_TPL
		}
	} else if let Kind::FieldAccessor(..) = f.kind() {
		if f.type_hint() == FunctionTypeHint::FieldSetter {
			&FIELD_WRITE_TPL
		} else {
			&FIELD_READ_TPL
		}
	} else if return_type.is_void() {
		&VOID_TPL
	} else {
		&NORMAL_TPL
	};
	let ret_type = if f.as_constructor().is_some() {
		return_type.cpp_full()
	} else {
		return_type.cpp_full_ext("ret", true)
	};
	let doref = if return_type.as_fixed_array().is_some() {
		"&"
	} else {
		""
	};
	tpl.interpolate(&hashmap! {
			"ret_type" => ret_type,
			"doref" => doref.into(),
			"name" => call_name,
			"generic" => generic.into(),
			"args" => args.into(),
		})
}

fn cpp_method_return<'f>(f: &'f Func, return_type: &TypeRef) -> (Cow<'f, str>, bool) {
	if return_type.is_void() {
		("".into(), false)
	} else if return_type.is_by_ptr() && !f.as_constructor().is_some() {
		let out = return_type.source()
			.as_class()
			.and_then(|cls| if cls.is_abstract() {
				Some(Cow::Borrowed("ret"))
			} else {
				None
			});
		let out = out.unwrap_or_else(|| format!("new {typ}(ret)", typ = return_type.cpp_full()).into());
		(out, false)
	} else if let Some(Dir::In(string_type)) | Some(Dir::Out(string_type)) = return_type.as_string() {
		let str_mk = match string_type {
			StrType::StdString(StrEnc::Text) | StrType::CvString(StrEnc::Text) => {
				"ocvrs_create_string(ret.c_str())".into()
			},
			StrType::CharPtr => {
				"ocvrs_create_string(ret)".into()
			},
			StrType::StdString(StrEnc::Binary) => {
				"ocvrs_create_byte_string(ret.data(), ret.size())".into()
			}
			StrType::CvString(StrEnc::Binary) => {
				"ocvrs_create_byte_string(ret.begin(), ret.size())".into()
			}
		};
		(str_mk, false)
	} else if return_type.as_fixed_array().is_some() {
		("ret".into(), true)
	} else {
		("ret".into(), false)
	}
}

impl RustNativeGeneratedElement for Func<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module(), self.rust_localname(FishStyle::No))
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
		static TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/func/rust_extern.tpl.rs").compile_interpolation()
		);

		let identifier = self.identifier();

		if settings::FUNC_MANUAL.contains_key(identifier.as_ref()) {
			return "".to_string();
		}

		let mut attributes = String::new();
		if let Some(attrs) = settings::FUNC_CFG_ATTR.get(identifier.as_ref()) {
			attributes = format!("#[cfg({})]", attrs.0);
		}
		let mut args = vec![];
		if let Some(cls) = self.as_instance_method() {
			args.push(cls.type_ref().rust_extern_self_func_decl(self.constness()));
		}
		for (name, arg) in Field::rust_disambiguate_names(self.arguments()) {
			args.push(arg.type_ref().rust_extern_arg_func_decl(&name, ConstnessOverride::No))
		}
		let naked_return = self.is_naked_return();
		let is_infallible = self.is_infallible();
		let return_type = self.return_type();
		let return_wrapper_type = if is_infallible {
			return_type.rust_extern_return()
		} else {
			return_type.rust_extern_return_wrapper_full()
		};
		if !naked_return {
			args.push(format!("ocvrs_return: *mut {}", return_wrapper_type));
		}

		let return_wrapper_type = if return_wrapper_type == "()" || !naked_return {
			Cow::Borrowed("")
		} else {
			format!(" -> {}", return_wrapper_type).into()
		};
		TPL.interpolate(&hashmap! {
			"attributes" => attributes.into(),
			"debug" => get_debug(self).into(),
			"identifier" => identifier,
			"args" => args.join(", ").into(),
			"return_wrapper_type" => return_wrapper_type,
		})
	}

	fn gen_cpp(&self) -> String {
		static TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/func/cpp.tpl.cpp").compile_interpolation()
		);

		let identifier = self.identifier();
		if settings::FUNC_MANUAL.contains_key(identifier.as_ref()) {
			return "".to_string();
		}

		let mut attributes_begin = String::new();
		let mut attributes_end = String::new();
		if let Some(attrs) = settings::FUNC_CFG_ATTR.get(identifier.as_ref()) {
			attributes_begin = format!("#if {}", attrs.1);
			attributes_end = "#endif".to_string();
		}
		let args = Field::cpp_disambiguate_names(self.arguments()).collect::<Vec<_>>();
		let mut decl_args = Vec::with_capacity(args.len());
		let mut pre_call_args = Vec::with_capacity(args.len());
		let mut post_call_args = Vec::with_capacity(args.len());
		let mut cleanup_args = Vec::with_capacity(args.len());
		if let Some(cls) = self.as_instance_method() {
			decl_args.push(cls.type_ref().cpp_self_func_decl(self.constness()));
		}
		for (name, arg) in args {
			let type_ref = arg.type_ref();
			decl_args.push(type_ref.cpp_arg_func_decl(&name));
			pre_post_arg_handle(type_ref.cpp_arg_pre_call(&name), &mut pre_call_args);
			pre_post_arg_handle(type_ref.cpp_arg_post_call(&name), &mut post_call_args);
			pre_post_arg_handle(type_ref.cpp_arg_cleanup(&name), &mut cleanup_args);
		}

		let is_infallible = self.is_infallible();
		let naked_return = self.is_naked_return();
		let return_type = self.return_type();
		let cpp_extern_return = return_type.cpp_extern_return(ConstnessOverride::No);
		let ret_wrapper_full = if is_infallible {
			cpp_extern_return.clone()
		} else {
			return_type.cpp_extern_return_wrapper_full(ConstnessOverride::No)
		};
		let mut_ret_wrapper_full = if is_infallible {
			return_type.cpp_extern_return(ConstnessOverride::Yes(Constness::Mut))
		} else {
			return_type.cpp_extern_return_wrapper_full(ConstnessOverride::Yes(Constness::Mut))
		};
		if !naked_return {
			decl_args.push(format!("{}* ocvrs_return", mut_ret_wrapper_full));
		}
		let return_spec = if !naked_return {
			"void".into()
		} else {
			ret_wrapper_full.clone()
		};
		let (ret, ret_cast) = cpp_method_return(self, &return_type);
		let ret = if cleanup_args.is_empty() {
			if ret.is_empty() {
				"".into()
			} else {
				ret
			}
		} else {
			pre_post_arg_handle(format!("{typ} f_ret = {expr}", typ=cpp_extern_return, expr=ret), &mut post_call_args);
			"f_ret".into()
		};
		let ret = if naked_return {
			if ret.is_empty() {
				"".into()
			} else if ret_cast {
				format!("return ({typ}){ret};", typ=ret_wrapper_full, ret=ret).into()
			} else {
				format!("return {};", ret).into()
			}
		} else if is_infallible {
			if ret.is_empty() {
				"".into()
			} else {
				format!("*ocvrs_return = {};", ret).into()
			}
		} else if ret.is_empty() {
			"Ok(ocvrs_return);".into()
		} else if ret_cast {
			format!("Ok<{typ}>({ret}, ocvrs_return);", typ=cpp_extern_return, ret=ret).into()
		} else {
			format!("Ok({}, ocvrs_return);", ret).into()
		};

		let func_try = if is_infallible {
			""
		} else {
			"try {"
		};

		let func_catch = if is_infallible {
			Cow::Borrowed("")
		} else {
			format!("}} OCVRS_CATCH(OCVRS_TYPE({return_wrapper_full}))", return_wrapper_full= mut_ret_wrapper_full).into()
		};

		TPL.interpolate(&hashmap! {
			"attributes_begin" => attributes_begin.into(),
			"debug" => get_debug(self).into(),
			"return_spec" => return_spec,
			"identifier" => identifier,
			"decl_args" => decl_args.join(", ").into(),
			"try" => func_try.into(),
			"pre_call_args" => pre_call_args.join("\n").into(),
			"call" => cpp_call_invoke(self).into(),
			"post_call_args" => post_call_args.join("\n").into(),
			"cleanup_args" => cleanup_args.join("\n").into(),
			"return" => ret,
			"catch" => func_catch,
			"attributes_end" => attributes_end.into(),
		})
	}
}

