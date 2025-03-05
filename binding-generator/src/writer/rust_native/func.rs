use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Write;
use std::iter;
use std::iter::{Chain, Once};
use std::rc::Rc;
use std::vec::IntoIter;

use once_cell::sync::Lazy;
use Cow::{Borrowed, Owned};

use super::comment::{render_ref, RenderComment};
use super::element::{DefaultRustNativeElement, RustElement};
use super::type_ref::{Lifetime, TypeRefExt};
use super::{comment, rust_disambiguate_names, RustNativeGeneratedElement};
use crate::field::Field;
use crate::func::{FuncCppBody, FuncKind, FuncRustBody, FuncRustExtern, InheritConfig, OperatorKind, ReturnKind};
use crate::name_pool::NamePool;
use crate::settings::ARG_OVERRIDE_SELF;
use crate::type_ref::{Constness, CppNameStyle, ExternDir, FishStyle, NameStyle, StrEnc, StrType, TypeRef, TypeRefTypeHint};
use crate::writer::rust_native::type_ref::render_lane::FunctionProps;
use crate::{reserved_rename, CompiledInterpolation, Element, Func, IteratorExt, NameDebug, StrExt, StringExt};

pub trait FuncExt<'tu, 'ge> {
	fn companion_functions(&self) -> Vec<Func<'tu, 'ge>>;
	fn with_companion_functions(self) -> Chain<Once<Self>, IntoIter<Self>>
	where
		Self: Sized;
}

impl<'tu, 'ge> FuncExt<'tu, 'ge> for Func<'tu, 'ge> {
	fn companion_functions(&self) -> Vec<Func<'tu, 'ge>> {
		let mut out = vec![];
		if let Some(default_func) = companion_func_default_args(self) {
			out.extend(default_func.companion_functions());
			out.push(default_func);
		}
		out.extend(companion_func_boxref_mut(self));
		out
	}

	fn with_companion_functions(self) -> Chain<Once<Self>, IntoIter<Self>> {
		let companions = self.companion_functions();
		iter::once(self).chain(companions)
	}
}

impl RustElement for Func<'_, '_> {
	fn rust_module(&self) -> Cow<str> {
		match self {
			&Self::Clang { entity, .. } => DefaultRustNativeElement::rust_module(entity),
			Self::Desc(desc) => desc.rust_module.as_ref().into(),
		}
	}

	fn rust_name(&self, style: NameStyle) -> Cow<str> {
		match self {
			&Self::Clang { entity, .. } => DefaultRustNativeElement::rust_name(self, entity, style).into(),
			Self::Desc(_) => match style {
				NameStyle::Declaration => self.rust_leafname(FishStyle::No),
				NameStyle::Reference(fish_style) => format!(
					"{}::{}",
					DefaultRustNativeElement::rust_module_reference(self),
					self.rust_leafname(fish_style)
				)
				.into(),
			},
		}
	}

	fn rust_leafname(&self, _fish_style: FishStyle) -> Cow<str> {
		if let Some(rust_custom_leafname) = self.rust_custom_leafname() {
			return rust_custom_leafname.into();
		}
		let cpp_name = match self {
			&Self::Clang { entity, gen_env, .. } => {
				if let Some(name) = gen_env.get_rename_config(entity).map(|c| &c.rename) {
					Borrowed(name.as_ref())
				} else {
					self.cpp_name(CppNameStyle::Declaration)
				}
			}
			Self::Desc(_) => self.cpp_name(CppNameStyle::Declaration),
		};
		let rust_name = if self.is_clone() {
			Borrowed("try_clone")
		} else {
			let kind = self.kind();
			if let Some(cls) = kind.as_constructor() {
				let args = self.arguments();
				let ctor_name = 'ctor_name: {
					if args.is_empty() {
						break 'ctor_name "default";
					} else if args.len() == 1 {
						let arg_typeref = args[0].type_ref();
						let source = arg_typeref.source_smart();
						if let Some(class_arg) = source.kind().as_class() {
							if cls == class_arg.as_ref() {
								break 'ctor_name if arg_typeref.constness().is_const() {
									"copy"
								} else {
									"copy_mut"
								};
							} else if class_arg.descendants().contains(cls) {
								break 'ctor_name "from_base";
							}
						}
					}
					"new"
				};
				Borrowed(ctor_name)
			} else if kind.as_conversion_method().is_some() {
				let mut conv_name = self.return_type_ref().rust_name(NameStyle::decl()).into_owned();
				conv_name.cleanup_name();
				conv_name.insert_str(0, "to_");
				Owned(conv_name)
			} else if let Some((cls, kind)) = kind.as_operator() {
				if cpp_name.starts_with("operator") {
					let op_name = match kind {
						OperatorKind::Unsupported => cpp_name.as_ref(),
						OperatorKind::Index => {
							if self.constness().is_const() {
								"get"
							} else {
								"get_mut"
							}
						}
						OperatorKind::Add => "add",
						OperatorKind::Sub => "sub",
						OperatorKind::Mul => "mul",
						OperatorKind::Div => "div",
						OperatorKind::Apply => "apply",
						OperatorKind::Set => "set",
						OperatorKind::Deref => {
							if self.constness().is_const() {
								"try_deref"
							} else {
								"try_deref_mut"
							}
						}
						OperatorKind::Equals => "equals",
						OperatorKind::NotEquals => "not_equals",
						OperatorKind::GreaterThan => "greater_than",
						OperatorKind::GreaterThanOrEqual => "greater_than_or_equal",
						OperatorKind::LessThan => "less_than",
						OperatorKind::LessThanOrEqual => "less_than_or_equal",
						OperatorKind::Incr => "incr",
						OperatorKind::Decr => "decr",
						OperatorKind::And => "and",
						OperatorKind::Or => "or",
						OperatorKind::Xor => "xor",
						OperatorKind::BitwiseNot => "negate",
					};
					if kind.add_args_to_name() {
						let args = self.arguments();
						let args = args.as_ref();
						let is_single_arg_same_as_class = if let (Some(cls), [single_arg]) = (cls, args) {
							single_arg
								.type_ref()
								.source()
								.kind()
								.as_class()
								.is_some_and(|single_class| single_class.as_ref() == cls)
						} else {
							false
						};
						if args.is_empty() || is_single_arg_same_as_class {
							Borrowed(op_name)
						} else {
							let args = args.iter().map(|arg| arg.type_ref().rust_simple_name()).join("_");
							Owned(format!("{op_name}_{args}"))
						}
					} else {
						Borrowed(op_name)
					}
				} else {
					cpp_name
				}
			} else {
				cpp_name
			}
		};
		let rust_name = reserved_rename(rust_name.cpp_name_to_rust_fn_case());
		if let Self::Clang { gen_env, .. } = self {
			if let Some(&name) = gen_env.settings.func_rename.get(self.identifier().as_str()) {
				return if name.contains('+') {
					Owned(name.replacen('+', rust_name.as_ref(), 1))
				} else {
					name.into()
				};
			}
		}
		Owned(rust_name.into_owned())
	}

	fn rendered_doc_comment(&self, comment_marker: &str, opencv_version: &str) -> String {
		let mut comment = RenderComment::new(&self.doc_comment_overloaded(), opencv_version);
		let args = self.arguments();
		let (_, default_args) = split_default_args(&args);
		let default_args_comment = comment::render_cpp_default_args(default_args);
		if !default_args_comment.is_empty() {
			if !comment.comment.is_empty() {
				comment.comment.push_str("\n\n");
			}
			comment.comment.push_str("## C++ default parameters\n");
			comment.comment.push_str(default_args_comment.trim_end());
		}
		comment.render_with_comment_marker(comment_marker).into_owned()
	}
}

impl RustNativeGeneratedElement for Func<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module(), self.rust_name(NameStyle::decl()))
	}

	fn gen_rust(&self, opencv_version: &str) -> String {
		static TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/func/rust.tpl.rs").compile_interpolation());

		let name = self.rust_leafname(FishStyle::No);
		let kind = self.kind();
		let return_kind = self.return_kind();
		let return_type_ref = self.return_type_ref();
		let safety = self.safety();
		let identifier = self.identifier();

		let args: Vec<(String, Field)> = rust_disambiguate_names(self.arguments().into_owned()).collect::<Vec<_>>();
		let as_instance_method = kind.as_instance_method();
		let mut decl_args = Vec::with_capacity(args.len());
		let mut pre_call_args = Vec::with_capacity(args.len());
		let mut call_args = Vec::with_capacity(args.len() + 1);
		let mut forward_args = Vec::with_capacity(args.len());
		let mut post_success_call_args = Vec::with_capacity(args.len());
		let boxed_ref_arg = return_type_ref.type_hint().as_boxed_as_ref();
		if let Some(cls) = as_instance_method {
			let constness = self.constness();
			let cls_type_ref = cls.type_ref().with_inherent_constness(constness);
			let render_lane = cls_type_ref.render_lane();
			let render_lane = render_lane.to_dyn();
			let lt = boxed_ref_arg
				.filter(|(_, boxed_arg_names, _)| boxed_arg_names.contains(&ARG_OVERRIDE_SELF))
				.map_or(Lifetime::Elided, |(_, _, lt)| lt);
			decl_args.push(render_lane.rust_self_func_decl(lt));
			call_args.push(render_lane.rust_arg_func_call("self"));
		}
		let mut callback_arg_name: Option<&str> = None;
		let function_props = FunctionProps {
			is_infallible: return_kind.is_infallible(),
		};
		for (name, arg) in &args {
			let arg_type_ref = arg.type_ref();
			let arg_type_hint = arg_type_ref.type_hint();
			let arg_as_slice_len = arg_type_hint.as_slice_len();
			let arg_kind = arg_type_ref.kind();
			let render_lane = arg_type_ref.render_lane();
			let render_lane = render_lane.to_dyn();
			if arg.is_user_data() {
				pre_post_arg_handle(
					format!(
						"userdata_arg!({decl} => {callback_name})",
						decl = arg_type_ref.render_lane().to_dyn().rust_extern_arg_func_decl(name),
						callback_name = callback_arg_name.expect("Can't get name of the callback arg")
					),
					&mut pre_call_args,
				);
			} else {
				if arg_kind.is_function() {
					callback_arg_name = Some(name);
				}
				if !arg_as_slice_len.is_some() {
					let lt = boxed_ref_arg
						.filter(|(_, boxed_arg_names, _)| boxed_arg_names.contains(&name.as_str()))
						.map(|(_, _, lt)| lt)
						.or_else(|| arg_type_hint.as_explicit_lifetime())
						.unwrap_or(Lifetime::Elided);
					decl_args.push(render_lane.rust_arg_func_decl(name, lt).into());
				}
				pre_post_arg_handle(render_lane.rust_arg_pre_call(name, &function_props), &mut pre_call_args);
			}
			if let Some((slice_args, len_div)) = arg_as_slice_len {
				let arg_is_size_t = arg_kind.is_size_t();
				let mut slice_len_call = String::new();
				for slice_arg in slice_args {
					let len_divided = if len_div > 1 {
						format!("{slice_arg}.len() / {len_div}")
					} else {
						format!("{slice_arg}.len()")
					};
					if slice_len_call.is_empty() {
						if len_div > 1 && (!arg_is_size_t || slice_args.len() > 1) {
							write!(&mut slice_len_call, "({len_divided})").expect("Impossible");
						} else {
							slice_len_call.push_str(&len_divided);
						}
					} else {
						write!(&mut slice_len_call, ".min({len_divided})").expect("Impossible");
					}
				}
				if !arg_is_size_t {
					slice_len_call.push_str(".try_into()?");
				}
				call_args.push(slice_len_call);
			} else {
				call_args.push(render_lane.rust_arg_func_call(name));
			}
			forward_args.push(name.as_str());
			pre_post_arg_handle(render_lane.rust_arg_post_success_call(name), &mut post_success_call_args);
		}
		if !return_kind.is_naked() {
			call_args.push("ocvrs_return.as_mut_ptr()".to_string());
		}

		let doc_comment = self.rendered_doc_comment("///", opencv_version);
		let visibility = if let Some(cls) = as_instance_method {
			if cls.kind().is_trait() {
				""
			} else {
				"pub "
			}
		} else {
			"pub "
		};
		let return_lifetime = match boxed_ref_arg {
			Some((_, _, lifetime)) => lifetime,
			None if matches!(kind.as_ref(), FuncKind::Function | FuncKind::StaticMethod(_)) => Lifetime::statik(),
			None => Lifetime::Elided,
		};
		let mut return_type_func_decl = return_type_ref.rust_return(FishStyle::No, return_lifetime);
		if !return_kind.is_infallible() {
			return_type_func_decl = format!("Result<{return_type_func_decl}>").into()
		};
		let return_type_func_decl = if return_type_func_decl == "()" {
			"".to_string()
		} else {
			format!(" -> {return_type_func_decl}")
		};
		let (ret_pre_call, ret_handle, ret_stmt) = rust_return(self, &return_type_ref, return_kind, return_lifetime);
		let mut attributes = Vec::with_capacity(2);
		if self.is_no_discard() {
			attributes.push(Borrowed("#[must_use]"));
		}
		if let Some((rust_attr, _)) = self.cfg_attrs() {
			if !rust_attr.is_empty() {
				attributes.push(format!("#[cfg({rust_attr})]").into());
			}
		}

		TPL.interpolate(&HashMap::from([
			("doc_comment", doc_comment.as_str()),
			("debug", &self.get_debug()),
			("attributes", &attributes.join("\n")),
			("visibility", visibility),
			("unsafety_decl", safety.rust_func_safety_qual()),
			("name", name.as_ref()),
			("generic_decl", &rust_generic_decl(self, &return_type_ref)),
			("decl_args", &decl_args.join(", ")),
			("rv_rust_full", &return_type_func_decl),
			("pre_call_args", &pre_call_args.join("\n")),
			("return_pre_call", ret_pre_call),
			(
				"call",
				&rust_call(self, &identifier, &name, &call_args, &forward_args, return_kind),
			),
			("return_handle", &ret_handle),
			("post_success_call_args", &post_success_call_args.join("\n")),
			("return", ret_stmt),
		]))
	}

	fn gen_rust_externs(&self) -> String {
		static TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/func/rust_extern.tpl.rs").compile_interpolation());

		if matches!(self.rust_extern_definition(), FuncRustExtern::Absent) {
			return "".to_string();
		}

		let identifier = self.identifier();
		let mut attributes = String::new();
		if let Some((rust_attr, _)) = self.cfg_attrs() {
			attributes = format!("#[cfg({rust_attr})]");
		}
		let mut args = vec![];
		if let Some(cls) = self.kind().as_instance_method() {
			args.push(
				cls.type_ref()
					.with_inherent_constness(self.constness())
					.render_lane()
					.to_dyn()
					.rust_extern_arg_func_decl("instance"),
			);
		}
		for (name, arg) in rust_disambiguate_names(self.arguments().into_owned()) {
			args.push(arg.type_ref().render_lane().to_dyn().rust_extern_arg_func_decl(&name))
		}

		let return_kind = self.return_kind();
		let naked_return = return_kind.is_naked();
		let is_infallible = return_kind.is_infallible();
		let return_type = self.return_type_ref();
		let return_wrapper_type = if is_infallible {
			return_type.rust_extern(ExternDir::FromCpp)
		} else {
			return_type.rust_extern_return_fallible()
		};
		if !naked_return {
			let ret_name = "ocvrs_return";
			args.push(format!("{ret_name}: *mut {return_wrapper_type}"));
		}
		let return_type_kind = return_type.kind();
		let return_wrapper_type = if return_type_kind.is_void() || !naked_return {
			"".to_string()
		} else {
			format!(" -> {return_wrapper_type}")
		};
		TPL.interpolate(&HashMap::from([
			("attributes", attributes),
			("debug", self.get_debug()),
			("identifier", identifier),
			("args", args.join(", ")),
			("return_type", return_wrapper_type),
		]))
	}

	fn gen_cpp(&self) -> String {
		static TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/func/cpp.tpl.cpp").compile_interpolation());

		if matches!(self.cpp_body(), FuncCppBody::Absent) {
			return "".to_string();
		}

		let identifier = self.identifier();

		let kind = self.kind();
		let return_kind = self.return_kind();
		let return_type_ref = self.return_type_ref();

		// attributes
		let mut attributes_begin = String::new();
		let mut attributes_end = String::new();
		if let Some((_, cpp_attr)) = self.cfg_attrs() {
			attributes_begin = format!("#if {cpp_attr}");
			attributes_end = "#endif".to_string();
		}

		// arguments
		let args = cpp_disambiguate_names(self.arguments().into_owned()).collect::<Vec<_>>();
		let mut decl_args = Vec::with_capacity(args.len());
		let mut pre_call_args = Vec::with_capacity(args.len());
		let mut call_args = Vec::with_capacity(args.len());
		let mut post_call_args = Vec::with_capacity(args.len());
		if let Some(cls) = kind.as_instance_method() {
			decl_args.push(
				cls.type_ref()
					.with_inherent_constness(self.constness())
					.render_lane()
					.to_dyn()
					.cpp_arg_func_decl("instance")
					.into_owned(),
			);
		}
		for (name, arg) in &args {
			let arg_type_ref = arg.type_ref();
			let render_lane = arg_type_ref.render_lane();
			let render_lane = render_lane.to_dyn();
			decl_args.push(render_lane.cpp_arg_func_decl(name).into_owned());
			pre_post_arg_handle(render_lane.cpp_arg_pre_call(name), &mut pre_call_args);
			call_args.push(render_lane.cpp_arg_func_call(name));
			pre_post_arg_handle(render_lane.cpp_arg_post_call(name), &mut post_call_args);
		}

		// return
		let ocv_ret_name = "ocvrs_return";
		let cpp_extern_return = return_type_ref.cpp_extern_return();
		let ret_full = if return_kind.is_infallible() {
			cpp_extern_return
		} else {
			return_type_ref.cpp_extern_return_fallible()
		};
		let return_type_ref_mut = return_type_ref.as_ref().clone().with_inherent_constness(Constness::Mut);
		let ret_wrapper_full_mut = if return_kind.is_infallible() {
			return_type_ref_mut.cpp_extern_return()
		} else {
			return_type_ref_mut.cpp_extern_return_fallible()
		};
		if !return_kind.is_naked() {
			decl_args.push(format!("{ret_wrapper_full_mut}* {ocv_ret_name}"));
		}
		let return_spec = if return_kind.is_naked() {
			Borrowed(ret_full.as_ref())
		} else {
			"void".into()
		};
		let (ret, ret_cast) = cpp_return_map(&return_type_ref, "ret", kind.as_constructor().is_some());

		// exception handling
		let func_try = if return_kind.is_infallible() {
			""
		} else {
			"try {"
		};
		let catch = if return_kind.is_infallible() {
			"".to_string()
		} else {
			format!("}} OCVRS_CATCH({ocv_ret_name});")
		};

		TPL.interpolate(&HashMap::from([
			("attributes_begin", attributes_begin.as_str()),
			("debug", &self.get_debug()),
			("return_spec", &return_spec),
			("identifier", &identifier),
			("decl_args", &decl_args.join(", ")),
			("try", func_try),
			("pre_call_args", &pre_call_args.join("\n")),
			("call", &cpp_call(self, &kind, &call_args, &return_type_ref)),
			("post_call_args", &post_call_args.join("\n")),
			(
				"return",
				&cpp_return(self, return_kind, &ret, ret_cast.then(|| ret_full.as_ref()), ocv_ret_name),
			),
			("catch", &catch),
			("attributes_end", &attributes_end),
		]))
	}
}

fn pre_post_arg_handle(mut arg: String, args: &mut Vec<String>) {
	if !arg.is_empty() {
		arg.push(';');
		args.push(arg);
	}
}

fn rust_call(
	f: &Func,
	identifier: &str,
	func_name: &str,
	call_args: &[String],
	forward_args: &[&str],
	return_kind: ReturnKind,
) -> String {
	#![allow(clippy::too_many_arguments)]
	static CALL_TPL: Lazy<CompiledInterpolation> =
		Lazy::new(|| "{{ret_receive}}unsafe { sys::{{identifier}}({{call_args}}) };".compile_interpolation());

	let ret_receive = if return_kind.is_naked() {
		"let ret = "
	} else {
		""
	};
	let tpl = match f.rust_body() {
		FuncRustBody::Auto => Borrowed(&*CALL_TPL),
		FuncRustBody::ManualCall(body) | FuncRustBody::ManualCallReturn(body) => Owned(body.compile_interpolation()),
	};
	tpl.interpolate(&HashMap::from([
		("ret_receive", ret_receive),
		("identifier", identifier),
		("name", func_name),
		("call_args", &call_args.join(", ")),
		("forward_args", &forward_args.join(", ")),
	]))
}

fn rust_return(
	f: &Func,
	return_type_ref: &TypeRef,
	return_kind: ReturnKind,
	lifetime: Lifetime,
) -> (&'static str, String, &'static str) {
	match f.rust_body() {
		FuncRustBody::Auto | FuncRustBody::ManualCall(_) => {
			let ret_pre = if !return_kind.is_naked() {
				"return_send!(via ocvrs_return);"
			} else {
				""
			};

			let mut ret_convert = Vec::with_capacity(3);
			if !return_kind.is_naked() {
				ret_convert.push(Borrowed("return_receive!(ocvrs_return => ret);"));
			}
			if !return_kind.is_infallible() {
				ret_convert.push("let ret = ret.into_result()?;".into())
			}
			let ret_map = rust_return_map(return_type_ref, "ret", return_kind, lifetime);
			if !ret_map.is_empty() {
				ret_convert.push(format!("let ret = {ret_map};").into());
			}

			let ret_stmt = if return_kind.is_infallible() {
				"ret"
			} else {
				"Ok(ret)"
			};
			(ret_pre, ret_convert.join("\n"), ret_stmt)
		}
		FuncRustBody::ManualCallReturn(_) => ("", "".to_string(), ""),
	}
}

fn rust_return_map(return_type: &TypeRef, ret_name: &str, return_kind: ReturnKind, lifetime: Lifetime) -> Cow<'static, str> {
	let return_type_kind = return_type.kind();
	if return_type_kind.as_string(return_type.type_hint()).is_some() || return_type_kind.extern_pass_kind().is_by_void_ptr() {
		format!(
			"unsafe {{ {typ}::opencv_from_extern({ret_name}) }}",
			typ = return_type.rust_return(FishStyle::Turbo, lifetime),
		)
		.into()
	} else if return_type_kind.as_pointer().is_some_and(|i| !i.kind().is_void())
		&& !return_type_kind.is_rust_by_ptr(return_type.type_hint())
		|| return_type_kind.as_fixed_array().is_some()
	{
		let ptr_call = if return_type.constness().is_const() {
			"as_ref"
		} else {
			"as_mut"
		};
		let error_handling = if return_kind.is_infallible() {
			".expect(\"Function returned null pointer\")"
		} else {
			".ok_or_else(|| Error::new(core::StsNullPtr, \"Function returned null pointer\"))?"
		};
		format!("unsafe {{ {ret_name}.{ptr_call}() }}{error_handling}").into()
	} else {
		"".into()
	}
}

fn cpp_call(f: &Func, kind: &FuncKind, call_args: &[String], return_type_ref: &TypeRef) -> String {
	static CALL_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| "{{name}}({{args}})".compile_interpolation());

	static VOID_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| "{{call}};".compile_interpolation());

	static RETURN_TPL: Lazy<CompiledInterpolation> =
		Lazy::new(|| "{{ret_with_type}} = {{doref}}{{call}};".compile_interpolation());

	static CONSTRUCTOR_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| "{{ret_with_type}}({{args}});".compile_interpolation());

	static CONSTRUCTOR_NO_ARGS_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| "{{ret_with_type}};".compile_interpolation());

	static BOXED_CONSTRUCTOR_TPL: Lazy<CompiledInterpolation> =
		Lazy::new(|| "{{ret_type}}* ret = new {{ret_type}}({{args}});".compile_interpolation());

	let call_args = call_args.join(", ");

	let return_type_kind = return_type_ref.kind();
	let ret_type = return_type_ref.cpp_name(CppNameStyle::Reference);
	let ret_with_type = return_type_ref.cpp_name_ext(CppNameStyle::Reference, "ret", true);
	let doref = if return_type_kind.as_fixed_array().is_some() {
		"&"
	} else {
		""
	};

	let call_name = match kind {
		FuncKind::Constructor(cls) => cls.cpp_name(CppNameStyle::Reference),
		FuncKind::Function | FuncKind::GenericFunction | FuncKind::StaticMethod(..) | FuncKind::FunctionOperator(..) => {
			f.cpp_name(CppNameStyle::Reference)
		}
		FuncKind::FieldAccessor(cls, fld) => cpp_method_call_name(
			cls.type_ref().kind().extern_pass_kind().is_by_ptr(),
			&fld.cpp_name(CppNameStyle::Declaration),
		)
		.into(),
		FuncKind::InstanceMethod(cls)
		| FuncKind::GenericInstanceMethod(cls)
		| FuncKind::ConversionMethod(cls)
		| FuncKind::InstanceOperator(cls, ..) => cpp_method_call_name(
			cls.type_ref().kind().extern_pass_kind().is_by_ptr(),
			&f.cpp_name(CppNameStyle::Declaration),
		)
		.into(),
	};

	let mut inter_vars = HashMap::from([
		("ret_type", ret_type),
		("ret_with_type", ret_with_type),
		("doref", doref.into()),
		("args", call_args.as_str().into()),
		("name", call_name),
	]);

	let (call_tpl, full_tpl) = match f.cpp_body() {
		FuncCppBody::Auto { .. } => {
			if let Some(cls) = kind.as_constructor() {
				if cls.kind().is_boxed() {
					(None, Some(Borrowed(&*BOXED_CONSTRUCTOR_TPL)))
				} else if call_args.is_empty() {
					(None, Some(Borrowed(&*CONSTRUCTOR_NO_ARGS_TPL)))
				} else {
					(None, Some(Borrowed(&*CONSTRUCTOR_TPL)))
				}
			} else {
				(Some(Borrowed(&*CALL_TPL)), None)
			}
		}
		FuncCppBody::ManualCall(call) => (Some(Owned(call.compile_interpolation())), None),
		FuncCppBody::ManualCallReturn(full_tpl) => (None, Some(Owned(full_tpl.compile_interpolation()))),
		FuncCppBody::Absent => (None, None),
	};
	let tpl = full_tpl
		.or_else(|| {
			call_tpl.map(|call_tpl| {
				let call = call_tpl.interpolate(&inter_vars);
				inter_vars.insert("call", call.into());
				if return_type_ref.kind().is_void() {
					Borrowed(&*VOID_TPL)
				} else {
					Borrowed(&*RETURN_TPL)
				}
			})
		})
		.expect("Impossible");

	tpl.interpolate(&inter_vars)
}

fn cpp_return(f: &Func, return_kind: ReturnKind, ret: &str, ret_cast: Option<&str>, ocv_ret_name: &str) -> Cow<'static, str> {
	match &f.cpp_body() {
		FuncCppBody::Auto | FuncCppBody::ManualCall(_) => match return_kind {
			ReturnKind::InfallibleNaked => {
				if ret.is_empty() {
					"".into()
				} else {
					let cast = if let Some(ret_type) = ret_cast {
						format!("({ret_type})")
					} else {
						"".to_string()
					};
					format!("return {cast}{ret};").into()
				}
			}
			ReturnKind::InfallibleViaArg => {
				if ret.is_empty() {
					"".into()
				} else {
					format!("*{ocv_ret_name} = {ret};").into()
				}
			}
			ReturnKind::Fallible => {
				if ret.is_empty() {
					format!("Ok({ocv_ret_name});").into()
				} else {
					let cast = if let Some(ret_type) = ret_cast {
						format!("<{ret_type}>")
					} else {
						"".to_string()
					};
					format!("Ok{cast}({ret}, {ocv_ret_name});").into()
				}
			}
		},
		FuncCppBody::ManualCallReturn(_) | FuncCppBody::Absent => "".into(),
	}
}

pub fn cpp_return_map<'f>(return_type: &TypeRef, name: &'f str, is_constructor: bool) -> (Cow<'f, str>, bool) {
	let return_kind = return_type.kind();
	if return_kind.is_void() {
		("".into(), false)
	} else if let Some((_, string_type)) = return_kind.as_string(return_type.type_hint()) {
		let str_mk = match string_type {
			StrType::StdString(StrEnc::Text) | StrType::CvString(StrEnc::Text) => {
				format!("ocvrs_create_string({name}.c_str())").into()
			}
			StrType::StdString(StrEnc::Binary) => format!("ocvrs_create_byte_string({name}.data(), {name}.size())").into(),
			StrType::CvString(StrEnc::Binary) => format!("ocvrs_create_byte_string({name}.begin(), {name}.size())").into(),
			StrType::CharPtr(StrEnc::Text) => format!("ocvrs_create_string({name})").into(),
			StrType::CharPtr(StrEnc::Binary) => panic!("Returning a byte string via char* is not supported yet"),
		};
		(str_mk, false)
	} else if return_kind.extern_pass_kind().is_by_void_ptr() && !is_constructor {
		let ret_source = return_type.source();
		let out = ret_source.kind().as_class().filter(|cls| cls.is_abstract()).map_or_else(
			|| {
				// todo implement higher count if it's needed
				let deref_count = return_type.kind().as_pointer().map_or(0, |_| 1);
				format!(
					"new {typ}({:*<deref_count$}{name})",
					"",
					typ = ret_source.cpp_name(CppNameStyle::Reference)
				)
				.into()
			},
			|_| name.into(),
		);
		(out, false)
	} else {
		(name.into(), return_kind.as_fixed_array().is_some())
	}
}

fn cpp_method_call_name(extern_by_ptr: bool, method_name: &str) -> String {
	if extern_by_ptr {
		format!("instance->{method_name}")
	} else {
		format!("instance.{method_name}")
	}
}

pub fn cpp_disambiguate_names<'tu, 'ge>(
	args: impl IntoIterator<Item = Field<'tu, 'ge>>,
) -> impl Iterator<Item = (String, Field<'tu, 'ge>)>
where
	'tu: 'ge,
{
	let args = args.into_iter();
	let size_hint = args.size_hint();
	NamePool::with_capacity(size_hint.1.unwrap_or(size_hint.0)).into_disambiguator(args, |f| f.cpp_name(CppNameStyle::Declaration))
}

fn rust_generic_decl<'f>(f: &'f Func, return_type_ref: &TypeRef) -> Cow<'f, str> {
	let mut decls = vec![];
	if let Some((_, _, lt)) = return_type_ref.type_hint().as_boxed_as_ref() {
		decls.push(lt.to_string());
	}
	match f {
		Func::Clang { .. } => {}
		Func::Desc(desc) => {
			decls.reserve(desc.rust_generic_decls.len());
			for (typ, constraint) in desc.rust_generic_decls.as_ref() {
				decls.push(format!("{typ}: {constraint}"));
			}
		}
	}
	let decls = decls.join(", ");
	if decls.is_empty() {
		"".into()
	} else {
		format!("<{decls}>").into()
	}
}

fn viable_default_arg(arg: &Field) -> bool {
	arg.default_value().is_some() && !arg.is_user_data() && {
		let type_ref = arg.type_ref();
		// don't remove the arguments that are used to pass the slice or its length
		!matches!(
			type_ref.type_hint(),
			TypeRefTypeHint::Slice | TypeRefTypeHint::LenForSlice(..)
		)
	}
}

/// Split `arg` into 2 sub-slices, (args_without_default_value, args_with_default_value)
///
/// Arguments with special meanings like userdata and slice length are not included in the args_with_default_value slice.
fn split_default_args<'a, 'tu, 'ge>(args: &'a [Field<'tu, 'ge>]) -> (&'a [Field<'tu, 'ge>], &'a [Field<'tu, 'ge>]) {
	// default args are in the end
	let last_non_default_arg_idx = args.iter().rposition(|arg| !viable_default_arg(arg));
	if let Some(last_non_default_arg_idx) = last_non_default_arg_idx {
		args.split_at(last_non_default_arg_idx + 1)
	} else {
		(&[], args)
	}
}

/// Companion function with all optional arguments as defaults
fn companion_func_default_args<'tu, 'ge>(f: &Func<'tu, 'ge>) -> Option<Func<'tu, 'ge>> {
	if f.kind().as_field_accessor().is_some() {
		return None;
	}

	match f {
		Func::Clang { gen_env, .. } => {
			if gen_env
				.settings
				.func_companion_tweak
				.get(&mut f.matcher())
				.is_some_and(|t| t.skip_default())
			{
				return None;
			}
		}
		Func::Desc(_) => {}
	}

	let args = f.arguments();
	let (args_without_def, args_with_def) = split_default_args(&args);
	if args_with_def.is_empty() {
		return None;
	}
	let original_rust_leafname = f.rust_leafname(FishStyle::No);
	let mut doc_comment = f.doc_comment().into_owned();
	let rust_leafname = format!("{}_def", original_rust_leafname);
	let default_args = comment::render_cpp_default_args(args_with_def);
	if !doc_comment.is_empty() {
		doc_comment.push_str("\n\n");
	}
	write!(
			&mut doc_comment,
			"## Note\nThis alternative version of [{refr}] function uses the following default values for its arguments:\n{default_args}",
			refr = render_ref(f, Some(&original_rust_leafname))
		)
	.expect("Impossible");
	let mut desc = f.to_desc_with_skip_config(InheritConfig::empty().doc_comment().arguments());
	let desc_mut = Rc::make_mut(&mut desc);
	desc_mut.rust_custom_leafname = Some(rust_leafname.into());
	desc_mut.arguments = args_without_def.into();
	desc_mut.doc_comment = doc_comment.into();
	let out = Func::Desc(desc);
	if out.exclude_kind().is_included() {
		Some(out)
	} else {
		None
	}
}

/// Companion function returning `BoxRefMut` for a corresponding function returning `BoxRef`
fn companion_func_boxref_mut<'tu, 'ge>(f: &Func<'tu, 'ge>) -> Option<Func<'tu, 'ge>> {
	let ret_type_ref = f.return_type_ref();
	if let Some((Constness::Mut, borrow_arg_names, _)) = ret_type_ref.type_hint().as_boxed_as_ref() {
		let mut desc = f.to_desc_with_skip_config(InheritConfig::empty());
		let desc_mut = Rc::make_mut(&mut desc);
		let mut cloned_args = None;
		// Rc::make_mut doesn't work on slices
		let args = if let Some(args) = Rc::get_mut(&mut desc_mut.arguments) {
			args
		} else {
			cloned_args = Some(desc_mut.arguments.to_vec());
			cloned_args.as_mut().unwrap()
		};
		let mut borrow_arg_is_const = false;
		if borrow_arg_names.contains(&ARG_OVERRIDE_SELF) {
			if desc_mut.kind.as_instance_method().is_some() && desc_mut.constness.is_const() {
				borrow_arg_is_const = true;
				desc_mut.constness = Constness::Mut;
			}
		} else {
			let borrow_arg = args
				.iter_mut()
				.find(|arg| borrow_arg_names.contains(&arg.cpp_name(CppNameStyle::Declaration).as_ref()));
			if let Some(borrow_arg) = borrow_arg {
				let type_ref = borrow_arg.type_ref();
				let kind = type_ref.kind();
				borrow_arg_is_const = type_ref.constness().is_const()
					&& kind
						.as_pointer_reference_move()
						.is_some_and(|ptr_or_ref| ptr_or_ref.kind().as_class().is_some());
				if borrow_arg_is_const {
					*borrow_arg = borrow_arg.clone().with_type_ref(
						borrow_arg
							.type_ref()
							.map_ptr_ref(|inner| inner.clone().with_inherent_constness(Constness::Mut)),
					);
				}
			}
		}
		if borrow_arg_is_const {
			if let Some(args) = cloned_args {
				desc_mut.arguments = args.into();
			}
			desc_mut.rust_custom_leafname = Some(format!("{}_mut", f.rust_leafname(FishStyle::No)).into());
			desc_mut.return_type_ref.set_inherent_constness(Constness::Mut);
			Some(Func::Desc(desc))
		} else {
			None
		}
	} else {
		None
	}
}
