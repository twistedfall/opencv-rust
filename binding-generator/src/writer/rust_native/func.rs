use std::borrow::Cow;

use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::{
	Class,
	CompiledInterpolation,
	Constness,
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
	let is_method_const = f.is_const();
	let is_infallible = f.is_infallible();
	let mut decl_args = Vec::with_capacity(args.len());
	let mut call_args = Vec::with_capacity(args.len());
	let mut forward_args = Vec::with_capacity(args.len());
	let mut pre_call_args = Vec::with_capacity(args.len());
	let mut post_call_args = Vec::with_capacity(args.len());
	if let Some(cls) = &as_instance_method {
		decl_args.push(cls.type_ref().rust_self_func_decl(is_method_const));
		call_args.push(cls.type_ref().rust_self_func_call(is_method_const));
	}
	let mut callback_arg_name: Option<String> = None;
	for (name, arg) in args {
		let type_ref = arg.type_ref();
		if arg.is_user_data() {
			pre_post_arg_handle(
				type_ref.rust_userdata_pre_call(&name, callback_arg_name.as_ref().map(|x| x.as_str()).expect("Can't get name of the callback arg")),
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
				format!("({slice_arg}.len() / {len_div}) as _", slice_arg = slice_arg, len_div = len_div)
			} else {
				format!("{slice_arg}.len() as _", slice_arg = slice_arg)
			};
			call_args.push(slice_call);
		} else {
			call_args.push(type_ref.rust_arg_func_call(&name, false));
		}
		forward_args.push(type_ref.rust_arg_forward(&name));
		pre_post_arg_handle(type_ref.rust_arg_post_call(&name, is_infallible), &mut post_call_args);
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
	let is_safe = !settings::FUNC_UNSAFE.contains(identifier.as_ref());
	let return_type = f.return_type();
	let return_type_func_decl = if is_infallible {
		return_type.rust_return_func_decl(false)
	} else {
		return_type.rust_return_func_decl_wrapper()
	};
	let mut prefix = String::new();
	let mut suffix = if is_infallible {
		format!(".expect(\"Infallible function failed: {name}\")", name = name)
	} else {
		String::new()
	};
	if !post_call_args.is_empty() {
		post_call_args.push("out".into());
		prefix.push_str("let out = ");
		suffix.push(';');
	}
	let decl_args = decl_args.join(", ");
	let pre_call_args = pre_call_args.join("\n");
	let call_args = call_args.join(", ");
	let forward_args = forward_args.join(", ");
	let post_call_args = post_call_args.join("\n");
	let ret_map = return_type.rust_return_map(is_safe);
	let mut attributes = String::new();
	if let Some(attrs) = settings::FUNC_CFG_ATTR.get(identifier.as_ref()) {
		attributes = format!("#[cfg({})]", attrs.0);
	}

	let tpl = if let Some(tpl) = settings::FUNC_MANUAL.get(identifier.as_ref()) {
		tpl
	} else {
		&TPL
	};
	tpl.interpolate(&hashmap! {
			"doc_comment" => doc_comment.as_str(),
			"debug" => &debug,
			"attributes" => &attributes,
			"visibility" => &visibility,
			"unsafety_decl" => if is_safe { "" } else { "unsafe " },
			"name" => name,
			"generic_decl" => "",
			"decl_args" => &decl_args,
			"rv_rust_full" => return_type_func_decl.as_ref(),
			"pre_call_args" => &pre_call_args,
			"prefix" => &prefix,
			"unsafety_call" => if is_safe { "unsafe " } else { "" },
			"identifier" => identifier.as_ref(),
			"call_args" => &call_args,
			"forward_args" => &forward_args,
			"suffix" => &suffix,
			"post_call_args" => &post_call_args,
			"ret_map" => ret_map.as_ref(),
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
		return_type.cpp_full_with_name("ret")
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

fn cpp_method_return<'f>(f: &'f Func) -> Cow<'f, str> {
	let return_type = f.return_type();
	if return_type.is_void() {
		"return Ok();".into()
	} else if return_type.is_by_ptr() && !f.as_constructor().is_some() {
		let out = return_type.source()
			.as_class()
			.and_then(|cls| if cls.is_abstract() {
				Some(Cow::Borrowed("return Ok(ret);"))
			} else {
				None
			});
		out.unwrap_or_else(|| format!("return Ok(new {typ}(ret));", typ = return_type.cpp_full()).into())
	} else if return_type.is_cv_string() || return_type.is_std_string() {
		"return Ok(ocvrs_create_string(ret.c_str()));".into()
	} else if return_type.is_char_ptr_string() {
		"return Ok(ocvrs_create_string(ret));".into()
	} else {
		"return Ok(ret);".into()
	}
}

impl RustNativeGeneratedElement for Func<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module(), self.rust_localname())
	}

	fn gen_rust(&self, opencv_version: &str) -> String {
		let name = if let Some(name_hint) = self.name_hint() {
			name_hint.into()
		} else {
			self.rust_leafname()
		};
		gen_rust_with_name(self, &name, opencv_version)
	}

	fn gen_rust_exports(&self) -> String {
		static TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/func/rust_extern.tpl.rs").compile_interpolation()
		);

		if settings::FUNC_MANUAL.contains_key(self.identifier().as_ref()) {
			return "".to_string();
		}

		let identifier = self.identifier();
		let mut attributes = String::new();
		if let Some(attrs) = settings::FUNC_CFG_ATTR.get(identifier.as_ref()) {
			attributes = format!("#[cfg({})]", attrs.0);
		}
		let mut args = vec![];
		if let Some(cls) = self.as_instance_method() {
			args.push(cls.type_ref().rust_extern_self_func_decl(self.is_const()));
		}
		for arg in self.arguments() {
			args.push(arg.type_ref().rust_extern_arg_func_decl(&arg.rust_leafname(), Constness::Auto))
		}
		let return_type = self.return_type();
		let return_wrapper_type = return_type.rust_extern_return_wrapper_full();
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

		if settings::FUNC_MANUAL.contains_key(self.identifier().as_ref()) {
			return "".to_string();
		}

		let identifier = self.identifier();
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
		if let Some(cls) = self.as_instance_method() {
			decl_args.push(cls.type_ref().cpp_self_func_decl(self.is_const()));
		}
		for (name, arg) in args {
			let type_ref = arg.type_ref();
			decl_args.push(type_ref.cpp_arg_func_decl(&name));
			let mut pre_call_arg = type_ref.cpp_arg_pre_call(&name);
			if !pre_call_arg.is_empty() {
				pre_call_arg.push(';');
				pre_call_args.push(pre_call_arg);
			}
			let mut post_call_arg = type_ref.cpp_arg_post_call(&name);
			if !post_call_arg.is_empty() {
				post_call_arg.push(';');
				post_call_args.push(post_call_arg);
			}
		}

		let return_type = self.return_type();

		TPL.interpolate(&hashmap! {
			"attributes_begin" => attributes_begin.into(),
			"debug" => get_debug(self).into(),
			"return_wrapper_type" => return_type.cpp_extern_return_wrapper_full(),
			"identifier" => identifier,
			"decl_args" => decl_args.join(", ").into(),
			"pre_call_args" => pre_call_args.join("\n").into(),
			"call" => cpp_call_invoke(self).into(),
			"post_call_args" => post_call_args.join("\n").into(),
			"return" => cpp_method_return(self),
			"attributes_end" => attributes_end.into(),
		})
	}
}

