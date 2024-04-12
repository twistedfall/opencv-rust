use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Write;
use std::rc::Rc;
use Cow::{Borrowed, Owned};

use once_cell::sync::Lazy;

use crate::field::Field;
use crate::func::{FuncCppBody, FuncKind, FuncRustBody, FuncRustExtern, InheritConfig, OperatorKind, ReturnKind, Safety};
use crate::name_pool::NamePool;
use crate::settings::ARG_OVERRIDE_SELF;
use crate::type_ref::{Constness, CppNameStyle, ExternDir, FishStyle, NameStyle, StrEnc, StrType, TypeRef, TypeRefTypeHint};
use crate::{reserved_rename, settings, CompiledInterpolation, Element, Func, IteratorExt, NameDebug, StrExt, StringExt};

use super::comment::{render_ref, RenderComment};
use super::element::{DefaultRustNativeElement, RustElement};
use super::type_ref::{Lifetime, TypeRefExt};
use super::{comment, rust_disambiguate_names, RustNativeGeneratedElement};

pub trait FuncExt<'tu, 'ge> {
	fn companion_functions(&self) -> Vec<Func<'tu, 'ge>>;
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
					name.as_ref().into()
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
				'ctor_name: {
					if args.is_empty() {
						break 'ctor_name "default";
					} else if args.len() == 1 {
						let arg_typeref = args[0].type_ref();
						let source = arg_typeref.source_smart();
						let source_kind = source.kind();
						let class_arg = source_kind.as_class();
						if let Some(class_arg) = class_arg {
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
								.map_or(false, |single_class| single_class.as_ref() == cls)
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

	fn rendered_doc_comment(&self, comment_marker: &str, opencv_version: &str) -> String {
		let mut comment = RenderComment::new(&self.doc_comment_overloaded(), opencv_version);
		let default_args_comment = comment::render_cpp_default_args(self.arguments().as_ref());
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

	fn gen_rust(&self, _opencv_version: &str) -> String {
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
		let boxed_ref_arg = if let Some((name, lt)) = return_type_ref.type_hint().as_boxed_as_ref() {
			Some((name, lt))
		} else {
			None
		};
		if let Some(cls) = as_instance_method {
			let constness = self.constness();
			let cls_type_ref = cls.type_ref().with_inherent_constness(constness);
			let render_lane = cls_type_ref.render_lane();
			let render_lane = render_lane.to_dyn();
			let lt = boxed_ref_arg
				.filter(|(boxed_arg_name, _)| *boxed_arg_name == ARG_OVERRIDE_SELF)
				.map_or(Lifetime::Elided, |(_, lt)| lt);
			decl_args.push(render_lane.rust_self_func_decl(lt));
			call_args.push(render_lane.rust_arg_func_call("self"));
		}
		let mut callback_arg_name: Option<&str> = None;
		for (name, arg) in &args {
			let arg_type_ref = arg.type_ref();
			let arg_kind = arg_type_ref.kind();
			let render_lane = arg_type_ref.render_lane();
			let render_lane = render_lane.to_dyn();
			if arg.is_user_data() {
				pre_post_arg_handle(
					format!(
						"userdata_arg!({name} in callbacks => {callback_name})",
						callback_name = callback_arg_name.expect("Can't get name of the callback arg")
					),
					&mut pre_call_args,
				);
			} else {
				if arg_kind.is_function() {
					callback_arg_name = Some(name);
				}
				if !arg_type_ref.type_hint().as_slice_len().is_some() {
					let lt = boxed_ref_arg
						.filter(|(boxed_arg_name, _)| *boxed_arg_name == name)
						.map_or(Lifetime::Elided, |(_, lt)| lt);
					decl_args.push(render_lane.rust_arg_func_decl(name, lt).into());
				}
				pre_post_arg_handle(
					render_lane.rust_arg_pre_call(name, return_kind.is_infallible()),
					&mut pre_call_args,
				);
			}
			if let Some((slice_args, len_div)) = arg_type_ref.type_hint().as_slice_len() {
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

		let doc_comment = self.rendered_doc_comment("///", _opencv_version);
		let visibility = if let Some(cls) = as_instance_method {
			if cls.is_trait() {
				""
			} else {
				"pub "
			}
		} else {
			"pub "
		};
		let return_lifetime = match boxed_ref_arg {
			Some((_, lifetime)) => lifetime,
			None if matches!(kind.as_ref(), FuncKind::Function | FuncKind::StaticMethod(_)) => Lifetime::statik(),
			None => Lifetime::Elided,
		};
		let mut return_type_func_decl = return_type_ref.rust_return(FishStyle::No, return_lifetime);
		if !return_kind.is_infallible() {
			return_type_func_decl = format!("Result<{return_type_func_decl}>").into()
		};
		let return_type_func_decl = if return_type_func_decl == "()" {
			String::new()
		} else {
			format!(" -> {return_type_func_decl}")
		};
		let (ret_pre_call, ret_handle, ret_stmt) = rust_return(self, &return_type_ref, return_kind, safety, return_lifetime);
		let mut attributes = Vec::with_capacity(2);
		if self.is_no_discard() {
			attributes.push(Borrowed("#[must_use]"));
		}
		if let Some((rust_attr, _)) = settings::FUNC_CFG_ATTR.get(identifier.as_str()) {
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
				&rust_call(self, safety, &identifier, &name, &call_args, &forward_args, return_kind),
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
		if let Some((rust_attr, _)) = settings::FUNC_CFG_ATTR.get(identifier.as_str()) {
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
		if let Some((_, cpp_attr)) = settings::FUNC_CFG_ATTR.get(identifier.as_str()) {
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
					.cpp_arg_func_decl("instance"),
			);
		}
		for (name, arg) in &args {
			let arg_type_ref = arg.type_ref();
			let render_lane = arg_type_ref.render_lane();
			let render_lane = render_lane.to_dyn();
			decl_args.push(render_lane.cpp_arg_func_decl(name));
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
		let return_type_ref_mut = return_type_ref.with_inherent_constness(Constness::Mut);
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
			"".into()
		} else {
			format!("}} OCVRS_CATCH({ocv_ret_name});").into()
		};

		TPL.interpolate(&HashMap::from([
			("attributes_begin", attributes_begin.into()),
			("debug", self.get_debug().into()),
			("return_spec", return_spec),
			("identifier", identifier.into()),
			("decl_args", decl_args.join(", ").into()),
			("try", func_try.into()),
			("pre_call_args", pre_call_args.join("\n").into()),
			("call", cpp_call(self, &kind, &call_args, &return_type_ref).into()),
			("post_call_args", post_call_args.join("\n").into()),
			(
				"return",
				cpp_return(
					self,
					return_kind,
					&ret,
					ret_cast.then(|| ret_full.as_ref().into()),
					ocv_ret_name,
				),
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

fn rust_call(
	f: &Func,
	func_safety: Safety,
	identifier: &str,
	func_name: &str,
	call_args: &[String],
	forward_args: &[&str],
	return_kind: ReturnKind,
) -> String {
	#![allow(clippy::too_many_arguments)]
	static CALL_TPL: Lazy<CompiledInterpolation> =
		Lazy::new(|| "{{ret_receive}}{{unsafety_call}}{ sys::{{identifier}}({{call_args}}) };".compile_interpolation());

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
		("unsafety_call", func_safety.rust_block_safety_qual()),
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
	safety: Safety,
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
				let spec = if safety.is_safe() {
					"return_receive!(unsafe ocvrs_return => ret);"
				} else {
					"return_receive!(ocvrs_return => ret);"
				};
				ret_convert.push(Borrowed(spec));
			}
			if !return_kind.is_infallible() {
				ret_convert.push("let ret = ret.into_result()?;".into())
			}
			let ret_map = rust_return_map(return_type_ref, "ret", safety, return_kind, lifetime);
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

fn rust_return_map(
	return_type: &TypeRef,
	ret_name: &str,
	context_safety: Safety,
	return_kind: ReturnKind,
	lifetime: Lifetime,
) -> Cow<'static, str> {
	let unsafety_call = context_safety.rust_block_safety_qual();
	let return_type_kind = return_type.kind();
	if return_type_kind.as_string(return_type.type_hint()).is_some() || return_type_kind.extern_pass_kind().is_by_void_ptr() {
		format!(
			"{unsafety_call}{{ {typ}::opencv_from_extern({ret_name}) }}",
			typ = return_type.rust_return(FishStyle::Turbo, lifetime),
		)
		.into()
	} else if return_type_kind.as_pointer().map_or(false, |i| !i.kind().is_void())
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
		format!("{unsafety_call}{{ {ret_name}.{ptr_call}() }}{error_handling}").into()
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

fn cpp_return(f: &Func, return_kind: ReturnKind, ret: &str, ret_cast: Option<Cow<str>>, ocv_ret_name: &str) -> Cow<'static, str> {
	match &f.cpp_body() {
		FuncCppBody::Auto | FuncCppBody::ManualCall(_) => match return_kind {
			ReturnKind::InfallibleNaked => {
				if ret.is_empty() {
					"".into()
				} else {
					let cast = if let Some(ret_type) = ret_cast {
						format!("({typ})", typ = ret_type.as_ref())
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
						format!("<{typ}>", typ = ret_type.as_ref())
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
		let out = return_type
			.source()
			.kind()
			.as_class()
			.filter(|cls| cls.is_abstract())
			.map_or_else(
				|| format!("new {typ}({name})", typ = return_type.cpp_name(CppNameStyle::Reference)).into(),
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
	if let Some((_, lt)) = return_type_ref.type_hint().as_boxed_as_ref() {
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

/// Companion function with all optional arguments as defaults
fn companion_func_default_args<'tu, 'ge>(f: &Func<'tu, 'ge>) -> Option<Func<'tu, 'ge>> {
	fn viable_default_arg(arg: &Field) -> bool {
		arg.default_value().is_some() && !arg.is_user_data() && {
			let type_ref = arg.type_ref();
			!type_ref.kind().is_function()
				// don't remove the arguments that are used to pass the slice or its length
				&& !matches!(type_ref.type_hint(), TypeRefTypeHint::Slice | TypeRefTypeHint::LenForSlice(..))
		}
	}

	if f.kind().as_field_accessor().is_some() {
		return None;
	}

	let args = f.arguments();
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
		let out = match f.clone() {
			Func::Clang { .. } => {
				let mut desc = f.to_desc(InheritConfig::empty().doc_comment().arguments());
				let desc_mut = Rc::make_mut(&mut desc);
				desc_mut.rust_custom_leafname = Some(rust_leafname.into());
				desc_mut.arguments = args_without_def.into();
				desc_mut.doc_comment = doc_comment.into();
				Func::Desc(desc)
			}
			Func::Desc(mut desc) => {
				let desc_ref = Rc::make_mut(&mut desc);
				desc_ref.arguments = args_without_def.into();
				desc_ref.rust_custom_leafname = Some(rust_leafname.into());
				Func::Desc(desc)
			}
		};
		if out.exclude_kind().is_included() {
			Some(out)
		} else {
			None
		}
	} else {
		None
	}
}

/// Companion function returning `BoxRefMut` for a corresponding function returning `BoxRef`
fn companion_func_boxref_mut<'tu, 'ge>(f: &Func<'tu, 'ge>) -> Option<Func<'tu, 'ge>> {
	let ret_type_ref = f.return_type_ref();
	if let Some((borrow_arg_name, _)) = ret_type_ref.type_hint().as_boxed_as_ref() {
		let mut desc = f.to_desc(InheritConfig::empty());
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
		if borrow_arg_name == ARG_OVERRIDE_SELF {
			if desc_mut.kind.as_instance_method().is_some() && desc_mut.constness.is_const() {
				borrow_arg_is_const = true;
				desc_mut.constness = Constness::Mut;
			}
		} else {
			let borrow_arg = args
				.iter_mut()
				.find(|arg| arg.cpp_name(CppNameStyle::Declaration) == *borrow_arg_name);
			if let Some(borrow_arg) = borrow_arg {
				let type_ref = borrow_arg.type_ref();
				let kind = type_ref.kind();
				borrow_arg_is_const = type_ref.constness().is_const()
					&& kind
						.as_pointer_reference_move()
						.map_or(false, |ptr_or_ref| ptr_or_ref.kind().as_class().is_some());
				if borrow_arg_is_const {
					*borrow_arg = borrow_arg.with_type_ref(
						borrow_arg
							.type_ref()
							.map_ptr_ref(|inner| inner.with_inherent_constness(Constness::Mut)),
					);
				}
			}
		}
		if borrow_arg_is_const {
			if let Some(args) = cloned_args {
				desc_mut.arguments = args.into();
			}
			desc_mut.rust_custom_leafname = Some(format!("{}_mut", f.rust_leafname(FishStyle::No)).into());
			desc_mut.return_type_ref = desc_mut.return_type_ref.with_inherent_constness(Constness::Mut);
			Some(Func::Desc(desc))
		} else {
			None
		}
	} else {
		None
	}
}
