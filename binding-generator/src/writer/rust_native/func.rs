use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Write;
use std::rc::Rc;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::comment as crate_comment;
use crate::field::Field;
use crate::func::{cpp_disambiguate_names, FuncCppBody, FuncDesc, FuncKind, FuncRustBody, OperatorKind, ReturnKind, Safety};
use crate::type_ref::{Constness, CppNameStyle, Dir, ExternDir, FishStyle, NameStyle, StrEnc, StrType, TypeRef};
use crate::writer::rust_native::disambiguate_single_name;
use crate::{
	reserved_rename, settings, CompiledInterpolation, Element, Func, FuncTypeHint, IteratorExt, NameDebug, StrExt, StringExt,
};

use super::comment;
use super::element::{DefaultRustNativeElement, RustElement};
use super::type_ref::TypeRefExt;
use super::{rust_disambiguate_names, RustNativeGeneratedElement};

pub trait FuncExt<'tu, 'ge> {
	fn companion_func_default_args(&self) -> Option<Func<'tu, 'ge>>;
	fn companion_functions(&self) -> Vec<Func<'tu, 'ge>>;
}

impl<'tu, 'ge> FuncExt<'tu, 'ge> for Func<'tu, 'ge> {
	/// Companion function with all optional arguments as defaults
	fn companion_func_default_args(&self) -> Option<Func<'tu, 'ge>> {
		fn viable_default_arg(arg: &Field) -> bool {
			arg.default_value().is_some() && !arg.is_user_data() && !arg.type_ref().as_function().is_some()
		}

		let identifier = self.identifier();
		if settings::FUNC_MANUAL.contains_key(identifier.as_str()) || self.kind().as_field_accessor().is_some() {
			return None;
		}

		let args = self.arguments();
		// default args are usually in the end
		if args.iter().rev().any(viable_default_arg) {
			let first_non_default_arg_idx = args.iter().rposition(|arg| !viable_default_arg(arg));
			let (args_without_def, args_with_def) = if let Some(first_non_default_arg_idx) = first_non_default_arg_idx {
				if first_non_default_arg_idx + 1 == args.len() {
					return None;
				}
				(&args[..=first_non_default_arg_idx], &args[first_non_default_arg_idx..])
			} else {
				([].as_slice(), args.as_ref())
			};
			let original_rust_leafname = self.rust_leafname(FishStyle::No);
			let mut doc_comment = crate_comment::strip_comment_markers(&self.doc_comment());
			let rust_leafname = format!("{}_def", original_rust_leafname);
			let default_args = comment::render_cpp_default_args(args_with_def);
			if !doc_comment.is_empty() {
				doc_comment.push_str("\n\n");
			}
			write!(
				&mut doc_comment,
				"## Note\nThis alternative version of #{original_rust_leafname} function uses the following default values for its arguments:\n{default_args}"
			)
				.expect("Write to String doesn't fail");
			let desc = match self.clone() {
				Func::Clang { .. } => FuncDesc {
					kind: self.kind().into_owned(),
					type_hint: FuncTypeHint::None,
					constness: self.constness(),
					return_kind: self.return_kind(),
					cpp_name: self.cpp_name(CppNameStyle::Reference).into(),
					rust_custom_leafname: Some(rust_leafname.into()),
					rust_module: self.rust_module().into(),
					doc_comment: doc_comment.into(),
					def_loc: self.file_line_name().location,
					arguments: args_without_def.into(),
					return_type_ref: self.return_type_ref(),
					cpp_body: FuncCppBody::Auto,
					rust_body: FuncRustBody::Auto,
				},
				Func::Desc(desc) => {
					let mut desc = Rc::try_unwrap(desc).unwrap_or_else(|desc| desc.as_ref().clone());
					desc.arguments = args_without_def.into();
					desc.rust_custom_leafname = Some(rust_leafname.into());
					desc
				}
			};
			let out = Self::new_desc(desc);
			if out.exclude_kind().is_included() {
				Some(out)
			} else {
				None
			}
		} else {
			None
		}
	}

	fn companion_functions(&self) -> Vec<Func<'tu, 'ge>> {
		let mut out = vec![];
		out.extend(self.companion_func_default_args());
		out
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
					name.into()
				} else {
					self.cpp_name(CppNameStyle::Declaration)
				}
			}
			Self::Desc(_) => self.cpp_name(CppNameStyle::Declaration),
		};
		let rust_name = if self.is_clone() {
			"try_clone".into()
		} else {
			let kind = self.kind();
			if let Some(cls) = kind.as_constructor() {
				let args = self.arguments();
				#[allow(clippy::never_loop)] // fixme use named block when MSRV is 1.65
				'ctor_name: loop {
					if args.is_empty() {
						break 'ctor_name "default";
					} else if args.len() == 1 {
						let arg_typeref = args[0].type_ref();
						let class_arg = arg_typeref.source_smart().as_class();
						if let Some(class_arg) = class_arg {
							if *cls == class_arg {
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
					break 'ctor_name "new";
				}
				.into()
			} else if kind.as_conversion_method().is_some() {
				let mut name = self.return_type_ref().rust_name(NameStyle::decl()).into_owned();
				name.cleanup_name();
				format!("to_{name}").into()
			} else if let Some((cls, kind)) = kind.as_operator() {
				if cpp_name.starts_with("operator") {
					let name = match kind {
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
								.as_class()
								.map_or(false, |single_class| &single_class == cls)
						} else {
							false
						};
						if args.is_empty() || is_single_arg_same_as_class {
							name.into()
						} else {
							let args = args.iter().map(|arg| arg.type_ref().rust_simple_name()).join("_");
							format!("{name}_{args}").into()
						}
					} else {
						name.into()
					}
				} else {
					cpp_name
				}
			} else {
				cpp_name
			}
		};
		if let Some(&name) = settings::FUNC_RENAME.get(self.identifier().as_str()) {
			if name.contains('+') {
				reserved_rename(name.replace('+', rust_name.as_ref()).cpp_name_to_rust_case().into())
			} else {
				name.into()
			}
		} else {
			reserved_rename(rust_name.cpp_name_to_rust_case().into())
		}
	}

	fn rendered_doc_comment_with_prefix(&self, prefix: &str, opencv_version: &str) -> String {
		let comment = match self {
			&Self::Clang { entity, gen_env, .. } => {
				let mut comment = entity.doc_comment().into_owned();
				let line = self.file_line_name().location.as_file().map_or(0, |(_, line)| line);
				const OVERLOAD: &str = "@overload";
				if let Some(idx) = comment.find(OVERLOAD) {
					let rep = if let Some(copy) = gen_env.get_func_comment(line, entity.cpp_name(CppNameStyle::Reference).as_ref()) {
						Cow::Owned(format!("{copy}\n\n## Overloaded parameters\n"))
					} else {
						Cow::Borrowed("This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.")
					};
					comment.replace_range(idx..idx + OVERLOAD.len(), &rep);
				}
				static COPY_BRIEF: Lazy<Regex> = Lazy::new(|| Regex::new(r"@copybrief\s+(\w+)").unwrap());
				comment.replace_in_place_regex_cb(&COPY_BRIEF, |comment, caps| {
					let copy_name = caps.get(1).map(|(s, e)| &comment[s..e]).expect("Impossible");
					let mut copy_full_name = self.cpp_namespace().into_owned();
					copy_full_name.extend_sep("::", copy_name);
					if let Some(copy) = gen_env.get_func_comment(line, &copy_full_name) {
						Some(copy.into())
					} else {
						Some("".into())
					}
				});
				Cow::Owned(comment)
			}
			Self::Desc(desc) => desc.doc_comment.as_ref().into(),
		};
		comment::render_doc_comment_with_processor(&comment, prefix, opencv_version, |out| {
			let default_args_comment = comment::render_cpp_default_args(self.arguments().as_ref());
			if !default_args_comment.is_empty() {
				if !out.is_empty() {
					out.push_str("\n\n");
				}
				out.push_str("## C++ default parameters\n");
				out.push_str(default_args_comment.trim_end());
			}
		})
	}
}

impl RustNativeGeneratedElement for Func<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module(), self.rust_name(NameStyle::decl()))
	}

	fn gen_rust(&self, _opencv_version: &str) -> String {
		static TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/func/rust.tpl.rs").compile_interpolation());

		let name = self.rust_leafname(FishStyle::No);
		let kind = self.kind();
		let constness = self.constness();
		let return_kind = self.return_kind();
		let return_type_ref = self.return_type_ref();
		let safety = self.safety();
		let identifier = self.identifier();

		let args: Vec<(String, Field)> = rust_disambiguate_names(self.arguments().into_owned()).collect::<Vec<_>>();
		let as_instance_method = kind.as_instance_method();
		let mut decl_args = Vec::with_capacity(args.len());
		let mut call_args = Vec::with_capacity(args.len());
		let mut forward_args = Vec::with_capacity(args.len());
		let mut pre_call_args = Vec::with_capacity(args.len());
		let mut post_call_args = Vec::with_capacity(args.len());
		if let Some(cls) = as_instance_method {
			let cls_type_ref = cls.type_ref();
			decl_args.push(cls_type_ref.rust_self_func_decl(constness));
			call_args.push(cls_type_ref.rust_self_func_call(constness));
		}
		let mut callback_arg_name: Option<String> = None;
		for (name, arg) in args {
			let arg_type_ref = arg.type_ref();
			let arg_as_slice_len = arg.as_slice_len();
			if arg.is_user_data() {
				pre_post_arg_handle(
					arg.type_ref().rust_userdata_pre_call(
						&name,
						callback_arg_name.as_deref().expect("Can't get name of the callback arg"),
					),
					&mut pre_call_args,
				);
			} else {
				if arg_type_ref.as_function().is_some() {
					callback_arg_name = Some(name.clone());
				}
				if !arg_as_slice_len.is_some() {
					decl_args.push(arg_type_ref.rust_arg_func_decl(&name));
				}
				pre_post_arg_handle(
					arg_type_ref.rust_arg_pre_call(&name, return_kind.is_infallible()),
					&mut pre_call_args,
				);
			}
			if let Some((slice_arg, len_div)) = arg_as_slice_len {
				let slice_call = if len_div > 1 {
					format!("({slice_arg}.len() / {len_div}) as _")
				} else {
					format!("{slice_arg}.len() as _")
				};
				call_args.push(slice_call);
			} else {
				call_args.push(arg_type_ref.rust_arg_func_call(&name));
			}
			forward_args.push(arg_type_ref.rust_arg_forward(&name));
			pre_post_arg_handle(
				arg_type_ref.rust_arg_post_call(&name, return_kind.is_infallible()),
				&mut post_call_args,
			);
		}
		if !return_kind.is_naked() {
			pre_call_args.push("return_send!(via ocvrs_return);".to_string());
			call_args.push("ocvrs_return.as_mut_ptr()".to_string());
		}

		let doc_comment = self.rendered_doc_comment_with_prefix("///", _opencv_version);
		let visibility = if let Some(cls) = as_instance_method {
			if cls.is_trait() {
				""
			} else {
				"pub "
			}
		} else {
			"pub "
		};
		let is_static_func = matches!(kind.as_ref(), FuncKind::Function | FuncKind::StaticMethod(_));
		let return_type_func_decl = if return_kind.is_infallible() {
			return_type_ref.rust_return(FishStyle::No, is_static_func)
		} else {
			format!(
				"Result<{}>",
				self.return_type_ref().rust_return(FishStyle::No, is_static_func)
			)
			.into()
		};
		let return_type_func_decl = if return_type_func_decl == "()" {
			Cow::Borrowed("")
		} else {
			format!(" -> {return_type_func_decl}").into()
		};
		if return_kind.is_infallible() {
			post_call_args.push("ret".to_string());
		} else {
			post_call_args.push("Ok(ret)".to_string());
		}
		let ret_receive = if return_kind.is_naked() {
			"let ret = "
		} else {
			""
		};
		let mut ret_convert = Vec::with_capacity(3);
		if !return_kind.is_naked() {
			let spec = if safety.is_safe() {
				"return_receive!(unsafe ocvrs_return => ret);"
			} else {
				"return_receive!(ocvrs_return => ret);"
			};
			ret_convert.push(Cow::Borrowed(spec));
		}
		if !return_kind.is_infallible() {
			ret_convert.push("let ret = ret.into_result()?;".into())
		}
		let ret_map = rust_return_map(&return_type_ref, "ret", safety, return_kind, is_static_func);
		if !ret_map.is_empty() {
			ret_convert.push(format!("let ret = {ret_map};").into());
		}
		let mut attributes = String::new();
		if let Some((rust_attr, _)) = settings::FUNC_CFG_ATTR.get(identifier.as_str()) {
			attributes = format!("#[cfg({rust_attr})]");
		}
		if self.is_no_discard() {
			attributes.push_str("#[must_use]");
		}

		let decl_args = decl_args.join(", ");
		let pre_call_args = pre_call_args.join("\n");
		let call_args = call_args.join(", ");
		let forward_args = forward_args.join(", ");
		let post_call_args = post_call_args.join("\n");
		let ret_convert = ret_convert.join("\n");
		let tpl = if let Some(tpl) = settings::FUNC_MANUAL.get(identifier.as_str()) {
			tpl
		} else {
			&TPL
		};
		tpl.interpolate(&HashMap::from([
			("doc_comment", doc_comment.as_str()),
			("debug", &self.get_debug()),
			("attributes", &attributes),
			("visibility", visibility),
			(
				"unsafety_decl",
				if safety.is_safe() {
					""
				} else {
					"unsafe "
				},
			),
			("name", name.as_ref()),
			("generic_decl", ""),
			("decl_args", &decl_args),
			("rv_rust_full", return_type_func_decl.as_ref()),
			("pre_call_args", &pre_call_args),
			(
				"unsafety_call",
				if safety.is_safe() {
					"unsafe "
				} else {
					""
				},
			),
			("identifier", identifier.as_ref()),
			("call_args", &call_args),
			("forward_args", &forward_args),
			("ret_receive", ret_receive),
			("ret_convert", ret_convert.as_ref()),
			("post_call_args", &post_call_args),
		]))
	}

	fn gen_rust_exports(&self) -> String {
		static TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/func/rust_extern.tpl.rs").compile_interpolation());

		let identifier = self.identifier();

		if settings::FUNC_MANUAL.contains_key(identifier.as_str()) {
			return "".to_string();
		}

		let mut attributes = String::new();
		if let Some((rust_attr, _)) = settings::FUNC_CFG_ATTR.get(identifier.as_str()) {
			attributes = format!("#[cfg({rust_attr})]");
		}
		let mut args = vec![];
		if let Some(cls) = self.kind().as_instance_method() {
			args.push(cls.type_ref().rust_extern_self_func_decl(self.constness()));
		}
		for (name, arg) in rust_disambiguate_names(self.arguments().into_owned()) {
			args.push(arg.type_ref().rust_extern_arg_func_decl(&name))
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
		let return_wrapper_type = if return_type.is_void() || !naked_return {
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

		let identifier = self.identifier();

		if settings::FUNC_MANUAL.contains_key(identifier.as_str()) {
			return "".to_string();
		}

		let kind = self.kind();
		let constness = self.constness();
		let return_kind = self.return_kind();
		let return_type_ref = self.return_type_ref();

		// attributes
		let mut attributes_begin = String::new();
		let mut attributes_end = String::new();
		if let Some((_, cpp_attr)) = settings::FUNC_CFG_ATTR.get(identifier.as_str()) {
			attributes_begin = format!("#if {cpp_attr}");
			attributes_end = "#endif".to_string();
		}

		// arguments
		let args = cpp_disambiguate_names(self.arguments().into_owned()).collect::<Vec<_>>();
		let mut decl_args = Vec::with_capacity(args.len());
		let mut pre_call_args = Vec::with_capacity(args.len());
		let mut post_call_args = Vec::with_capacity(args.len());
		let mut cleanup_args = Vec::with_capacity(args.len());
		if let Some(cls) = kind.as_instance_method() {
			decl_args.push(cls.type_ref().cpp_self_func_decl(constness));
		}
		for (name, arg) in args {
			let arg_type_ref = arg.type_ref();
			decl_args.push(arg_type_ref.cpp_arg_func_decl(&name));
			pre_post_arg_handle(arg_type_ref.cpp_arg_pre_call(&name), &mut pre_call_args);
			pre_post_arg_handle(arg_type_ref.cpp_arg_post_call(&name), &mut post_call_args);
			pre_post_arg_handle(arg_type_ref.cpp_arg_cleanup(&name), &mut cleanup_args);
		}

		// return
		let ocv_ret_name = "ocvrs_return";
		let cpp_extern_return = return_type_ref.cpp_extern_return();
		let ret_full = if return_kind.is_infallible() {
			cpp_extern_return.clone()
		} else {
			return_type_ref.cpp_extern_return_fallible()
		};
		let mut_ret_wrapper_full = if return_kind.is_infallible() {
			return_type_ref
				.with_constness(Constness::Mut)
				.cpp_extern_return()
				.into_owned()
		} else {
			return_type_ref
				.with_constness(Constness::Mut)
				.cpp_extern_return_fallible()
				.into_owned()
		};
		if !return_kind.is_naked() {
			decl_args.push(format!("{mut_ret_wrapper_full}* {ocv_ret_name}"));
		}
		let return_spec = if return_kind.is_naked() {
			Cow::Borrowed(ret_full.as_ref())
		} else {
			"void".into()
		};
		let mut rets = disambiguate_single_name("ret");
		let ret_name = rets.next().expect("Endless iterator returned nothing");
		let (ret, ret_cast) = cpp_return_map(&return_type_ref, &ret_name, kind.as_constructor().is_some());
		let ret = if cleanup_args.is_empty() {
			ret
		} else {
			let ret_name = rets.next().expect("Endless iterator returned nothing");
			pre_post_arg_handle(format!("{cpp_extern_return} {ret_name} = {ret}"), &mut post_call_args);
			ret_name.into()
		};

		// exception handling
		let func_try = if return_kind.is_infallible() {
			""
		} else {
			"try {"
		};
		let catch = if return_kind.is_infallible() {
			"".into()
		} else {
			let typ = if mut_ret_wrapper_full.contains(',') {
				format!("OCVRS_TYPE({mut_ret_wrapper_full})")
			} else {
				mut_ret_wrapper_full
			};
			format!("}} OCVRS_CATCH({typ}, {ocv_ret_name});").into()
		};

		TPL.interpolate(&HashMap::from([
			("attributes_begin", attributes_begin.into()),
			("debug", self.get_debug().into()),
			("return_spec", return_spec),
			("identifier", identifier.into()),
			("decl_args", decl_args.join(", ").into()),
			("try", func_try.into()),
			("pre_call_args", pre_call_args.join("\n").into()),
			("call", self.cpp_call_invoke().into()),
			("post_call_args", post_call_args.join("\n").into()),
			("cleanup_args", cleanup_args.join("\n").into()),
			(
				"return",
				self.cpp_return(&ret, ret_cast.then(|| ret_full.as_ref().into()), ocv_ret_name),
			),
			("catch", catch),
			("attributes_end", attributes_end.into()),
		]))
	}
}

fn pre_post_arg_handle(mut arg: String, args: &mut Vec<String>) {
	if !arg.is_empty() {
		arg.push(';');
		args.push(arg);
	}
}

fn rust_return_map(
	return_type: &TypeRef,
	ret_name: &str,
	context_safety: Safety,
	return_kind: ReturnKind,
	is_static_func: bool,
) -> Cow<'static, str> {
	let unsafety_call = if context_safety.is_safe() {
		"unsafe "
	} else {
		""
	};
	if return_type.as_string().is_some() || return_type.extern_pass_kind().is_by_void_ptr() {
		format!(
			"{unsafety_call}{{ {typ}::opencv_from_extern({ret_name}) }}",
			unsafety_call = unsafety_call,
			typ = return_type.rust_return(FishStyle::Turbo, is_static_func),
			ret_name = ret_name,
		)
		.into()
	} else if return_type.as_pointer().map_or(false, |i| !i.is_void()) && !return_type.is_rust_by_ptr()
		|| return_type.as_fixed_array().is_some()
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
		format!("{unsafety_call}{{ {ret_name}.{ptr_call}() }}{error_handling}").into()
	} else {
		"".into()
	}
}

pub fn cpp_return_map<'f>(return_type: &TypeRef, name: &'f str, is_constructor: bool) -> (Cow<'f, str>, bool) {
	if return_type.is_void() {
		("".into(), false)
	} else if let Some(Dir::In(string_type)) | Some(Dir::Out(string_type)) = return_type.as_string() {
		let str_mk = match string_type {
			StrType::StdString(StrEnc::Text) | StrType::CvString(StrEnc::Text) => {
				format!("ocvrs_create_string({name}.c_str())").into()
			}
			StrType::CharPtr => format!("ocvrs_create_string({name})").into(),
			StrType::StdString(StrEnc::Binary) => format!("ocvrs_create_byte_string({name}.data(), {name}.size())").into(),
			StrType::CvString(StrEnc::Binary) => format!("ocvrs_create_byte_string({name}.begin(), {name}.size())").into(),
		};
		(str_mk, false)
	} else if return_type.extern_pass_kind().is_by_void_ptr() && !is_constructor {
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
	} else {
		(name.into(), return_type.as_fixed_array().is_some())
	}
}
