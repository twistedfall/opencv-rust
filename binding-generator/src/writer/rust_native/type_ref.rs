use std::borrow::Cow;
use std::fmt;
use std::fmt::Write;

use crate::type_ref::{
	Constness, ConstnessOverride, Dir, ExternDir, FishStyle, Kind, NameStyle, Signedness, StrEnc, StrType, TypeRef,
};
use crate::{IteratorExt, StringExt};

use super::element::RustElement;
use super::function::FunctionExt;
use super::renderer::RustRenderer;
use super::rust_disambiguate_names;
use super::smart_ptr::SmartPtrExt;
use super::tuple::TupleExt;
use super::vector::VectorExt;

pub trait TypeRefExt {
	fn format_as_array(&self, elem_type: &str, size: Option<usize>) -> String;

	fn rust_safe_id(&self, add_const: bool) -> Cow<str>;
	fn rust_module(&self) -> Cow<str>;
	fn rust_simple_name(&self) -> String;
	fn rust_name(&self, name_style: NameStyle) -> Cow<str>;
	fn rust_name_ext(&self, name_style: NameStyle, lifetime: Lifetime) -> Cow<str>;

	fn rust_self_func_decl(&self, method_constness: Constness) -> String;
	fn rust_arg_func_decl(&self, name: &str) -> String;
	fn rust_extern_self_func_decl(&self, method_constness: Constness) -> String;
	fn rust_extern_arg_func_decl(&self, name: &str, constness: ConstnessOverride) -> String;
	fn rust_arg_pre_call(&self, name: &str, is_function_infallible: bool) -> String;
	fn rust_userdata_pre_call(&self, name: &str, callback_name: &str) -> String;
	fn rust_self_func_call(&self, method_constness: Constness) -> String;
	fn rust_arg_func_call(&self, name: &str, constness: ConstnessOverride) -> String;
	fn rust_arg_forward(&self, name: &str) -> String;
	fn rust_arg_post_call(&self, name: &str, _is_function_infallible: bool) -> String;
	fn rust_extern(&self, dir: ExternDir) -> Cow<str>;
	fn rust_return(&self, turbo_fish_style: FishStyle, is_static_func: bool) -> Cow<str>;
	fn rust_extern_return_fallible(&self) -> Cow<str>;
	fn rust_lifetime_count(&self) -> usize;

	fn cpp_arg_func_decl(&self, name: &str) -> String;
	fn cpp_arg_pre_call(&self, name: &str) -> String;
	fn cpp_arg_func_call<'a>(&self, name: impl Into<Cow<'a, str>>) -> Cow<'a, str>;
	fn cpp_arg_post_call(&self, name: &str) -> String;
	fn cpp_arg_cleanup(&self, name: &str) -> String;
}

impl<'tu, 'ge> TypeRefExt for TypeRef<'tu, 'ge> {
	fn format_as_array(&self, elem_type: &str, size: Option<usize>) -> String {
		format!(
			"&{cnst}[{typ}{size}]",
			cnst = self.constness().rust_qual(false),
			typ = elem_type,
			size = size.map_or_else(|| "".to_string(), |s| format!("; {}", s))
		)
	}

	fn rust_safe_id(&self, add_const: bool) -> Cow<str> {
		let mut out = String::with_capacity(64);
		let kind = self.kind();
		if add_const && self.clang_constness().is_const() {
			out.push_str("const_");
		}
		out.push_str(&match kind {
			Kind::Array(inner, ..) => inner.rust_safe_id(add_const).into_owned() + "_X",
			Kind::StdVector(vec) => vec.rust_localalias().into_owned(),
			Kind::StdTuple(tuple) => tuple.rust_localalias().into_owned(),
			Kind::Pointer(inner) => {
				let mut inner_safe_id: String = inner.rust_safe_id(add_const).into_owned();
				if !self.is_extern_by_ptr() {
					inner_safe_id += "_X";
				}
				inner_safe_id
			}
			Kind::Reference(inner) | Kind::RValueReference(inner) => inner.rust_safe_id(add_const).into_owned(),
			Kind::SmartPtr(ptr) => ptr.rust_localalias().into_owned(),
			Kind::Class(cls) => cls.rust_name(NameStyle::decl()).into_owned(),
			Kind::Primitive(..) | Kind::Enum(..) | Kind::Function(..) | Kind::Typedef(..) | Kind::Generic(..) | Kind::Ignored => {
				self.rust_name(NameStyle::decl()).into_owned()
			}
		});
		out.cleanup_name();
		out.into()
	}

	fn rust_module(&self) -> Cow<str> {
		match self.kind() {
			Kind::Primitive(..) => "core".into(),
			Kind::StdVector(vec) => vec.rust_element_module().into_owned().into(),
			Kind::StdTuple(tuple) => tuple.rust_element_module().into_owned().into(),
			Kind::Array(inner, ..) | Kind::Pointer(inner) | Kind::Reference(inner) | Kind::RValueReference(inner) => {
				inner.rust_module().into_owned().into()
			}
			Kind::SmartPtr(ptr) => ptr.rust_module().into_owned().into(),
			Kind::Class(cls) => cls.rust_module().into_owned().into(),
			Kind::Enum(enm) => enm.rust_module().into_owned().into(),
			Kind::Function(..) => {
				"core".into() // fixme
			}
			Kind::Typedef(tdef) => tdef.rust_module().into_owned().into(),
			Kind::Generic(..) | Kind::Ignored => "core".into(),
		}
	}

	/// For when a type needs to be part of the user-visible Rust method name
	///
	/// Return a lightweight lowercase type representation, might not be precise. For example it's used for operator bindings so
	/// that `operator &` on 2 `Mat`s translates into `and_mat_mat()`.
	fn rust_simple_name(&self) -> String {
		let maybe_ptr = self.as_pointer().or_else(|| self.as_reference());
		let type_ref = if let Some(inner) = maybe_ptr.as_ref() {
			inner
		} else {
			self
		};
		type_ref.rust_name(NameStyle::Declaration).to_lowercase()
	}

	fn rust_name(&self, name_style: NameStyle) -> Cow<str> {
		self.rust_name_ext(name_style, Lifetime::elided())
	}

	fn rust_name_ext(&self, name_style: NameStyle, lifetime: Lifetime) -> Cow<str> {
		self.render(RustRenderer::new(name_style, lifetime, self.is_rust_by_ptr()))
	}

	fn rust_self_func_decl(&self, method_constness: Constness) -> String {
		if self.is_extern_by_ptr() {
			if method_constness.is_const() {
				"&self".to_string()
			} else {
				"&mut self".to_string()
			}
		} else {
			"self".to_string()
		}
	}

	fn rust_arg_func_decl(&self, name: &str) -> String {
		#[allow(clippy::never_loop)] // fixme use named block when MSRV is 1.65
		let typ = 'decl_type: loop {
			if let Some(dir) = self.as_string() {
				break 'decl_type match dir {
					Dir::In(StrType::StdString(StrEnc::Text) | StrType::CvString(StrEnc::Text) | StrType::CharPtr) => "&str".into(),
					Dir::In(StrType::StdString(StrEnc::Binary) | StrType::CvString(StrEnc::Binary)) => "&[u8]".into(),
					Dir::Out(StrType::StdString(StrEnc::Text) | StrType::CvString(StrEnc::Text) | StrType::CharPtr) => {
						"&mut String".into()
					}
					Dir::Out(StrType::StdString(StrEnc::Binary) | StrType::CvString(StrEnc::Binary)) => "&mut Vec<u8>".into(),
				};
			} else if self.is_input_array() {
				break 'decl_type "&dyn core::ToInputArray".into();
			} else if self.is_output_array() {
				break 'decl_type "&mut dyn core::ToOutputArray".into();
			} else if self.is_input_output_array() {
				break 'decl_type "&mut dyn core::ToInputOutputArray".into();
			} else if let Some((_, size)) = self.as_string_array() {
				break 'decl_type self.format_as_array("&str", size).into();
			} else if self.as_char8().is_some() {
				break 'decl_type "char".into();
			}
			break 'decl_type self.rust_name(NameStyle::ref_());
		};
		let cnst = Constness::from_is_mut(
			self.is_by_move()
				|| self.is_extern_by_ptr()
					&& self.constness().is_mut()
					&& !self.as_pointer().is_some()
					&& !self.as_reference().is_some(),
		);
		format!("{cnst}{name}: {typ}", cnst = cnst.rust_qual(false), name = name, typ = typ)
	}

	fn rust_extern_self_func_decl(&self, method_constness: Constness) -> String {
		self.rust_extern_arg_func_decl("instance", ConstnessOverride::force(method_constness))
	}

	fn rust_extern_arg_func_decl(&self, name: &str, constness: ConstnessOverride) -> String {
		format!(
			"{name}: {typ}",
			name = name,
			typ = self.rust_extern(ExternDir::ToCpp(constness))
		)
	}

	fn rust_arg_pre_call(&self, name: &str, is_function_infallible: bool) -> String {
		if let Some(dir) = self.as_string() {
			return match dir {
				Dir::In(_) => {
					let mut flags = vec![];
					if is_function_infallible {
						flags.push("nofail")
					}
					if self.constness().is_mut() {
						flags.push("mut")
					}
					let mut flags = flags.join(" ");
					if !flags.is_empty() {
						flags.push(' ');
					}
					format!("extern_container_arg!({flags}{name})", flags = flags, name = name)
				}
				Dir::Out(_) => {
					format!("string_arg_output_send!(via {name}_via)", name = name)
				}
			};
		} else if self.is_input_array() {
			return format!("input_array_arg!({name})", name = name);
		} else if self.is_output_array() {
			return format!("output_array_arg!({name})", name = name);
		} else if self.is_input_output_array() {
			return format!("input_output_array_arg!({name})", name = name);
		} else if self.as_string_array().is_some() {
			return if self.constness().is_const() {
				format!("string_array_arg!({name})", name = name)
			} else {
				format!("string_array_arg_mut!({name})", name = name)
			};
		} else if let Some(func) = self.as_function() {
			let args = rust_disambiguate_names(func.arguments()).collect::<Vec<_>>();
			if let Some((userdata_name, _)) = args.iter().find(|(_, f)| f.is_user_data()).cloned() {
				let ret = func.return_type();
				let tramp_args = args
					.into_iter()
					.map(|(name, a)| a.type_ref().rust_extern_arg_func_decl(&name, ConstnessOverride::No))
					.join(", ");
				let fw_args = rust_disambiguate_names(func.rust_arguments())
					.map(|(name, a)| a.type_ref().rust_extern_arg_func_decl(&name, ConstnessOverride::No))
					.join(", ");
				return format!(
					"callback_arg!({name}_trampoline({tramp_args}) -> {tramp_ret} => {tramp_userdata_arg} in callbacks => {name}({fw_args}) -> {fw_ret})",
					name=name,
					tramp_args=tramp_args,
					tramp_ret=ret.rust_extern(ExternDir::FromCpp),
					tramp_userdata_arg=userdata_name,
					fw_args=fw_args,
					fw_ret=ret.rust_extern(ExternDir::FromCpp),
				);
			}
		}
		"".to_string()
	}

	fn rust_userdata_pre_call(&self, name: &str, callback_name: &str) -> String {
		format!(
			"userdata_arg!({userdata_name} in callbacks => {callback_name})",
			userdata_name = name,
			callback_name = callback_name,
		)
	}

	fn rust_self_func_call(&self, method_constness: Constness) -> String {
		self.rust_arg_func_call(
			"self",
			if method_constness.is_const() {
				ConstnessOverride::Const
			} else {
				ConstnessOverride::No
			},
		)
	}

	fn rust_arg_func_call(&self, name: &str, constness: ConstnessOverride) -> String {
		if let Some(dir) = self.as_string() {
			return match dir {
				Dir::In(_) => {
					if constness.with(self.constness()).is_const() {
						format!("{name}.opencv_as_extern()", name = name)
					} else {
						format!("{name}.opencv_as_extern_mut()", name = name)
					}
				}
				Dir::Out(_) => format!("&mut {name}_via", name = name),
			};
		}
		if self.as_reference().map_or(false, |inner| {
			(inner.as_simple_class().is_some() || inner.is_enum())
				&& (constness.with(inner.constness()).is_const() || self.is_by_move())
		}) {
			let cnst = constness.with(self.constness());
			return format!("&{cnst}{name}", cnst = cnst.rust_qual(false), name = name);
		}
		if self.as_simple_class().is_some() {
			return format!("{name}.opencv_as_extern()", name = name);
		}
		if self.is_extern_by_ptr() {
			let typ = self.source();
			let by_ptr = if constness.with(self.constness()).is_const() {
				format!(
					"{name}.as_raw_{rust_safe_id}()",
					name = name,
					rust_safe_id = typ.rust_safe_id(false)
				)
			} else {
				format!(
					"{name}.as_raw_mut_{rust_safe_id}()",
					name = name,
					rust_safe_id = typ.rust_safe_id(false)
				)
			};
			return if self.is_nullable() {
				format!(
					"{name}.map_or({null_ptr}, |{name}| {by_ptr})",
					name = name,
					null_ptr = constness.with(self.constness()).rust_null_ptr(),
					by_ptr = by_ptr
				)
			} else {
				by_ptr
			};
		}
		if self.as_variable_array().is_some() {
			let arr = if constness.with(self.constness()).is_const() {
				format!("{name}.as_ptr()", name = name)
			} else {
				format!("{name}.as_mut_ptr()", name = name)
			};
			return if self.is_nullable() {
				format!(
					"{name}.map_or({null_ptr}, |{name}| {arr})",
					name = name,
					null_ptr = constness.with(self.constness()).rust_null_ptr(),
					arr = arr
				)
			} else {
				arr
			};
		}
		if let Some(func) = self.as_function() {
			if func.arguments().into_iter().any(|a| a.is_user_data()) {
				return format!("{name}_trampoline", name = name);
			}
		}
		if let Some(inner) = self.as_pointer() {
			if inner.as_pointer().is_some() {
				// some special care for double pointers
				return format!(
					"{name} as *{cnst} _ as *{cnst} *{const_inner} _",
					name = name,
					cnst = self.clang_constness().rust_qual(true),
					const_inner = inner.clang_constness().rust_qual(true)
				);
			}
		}
		if self.is_nullable() && (self.as_reference().is_some() || self.as_pointer().is_some()) {
			let arg = if constness.with(self.constness()).is_const() {
				format!("{name} as *const _", name = name)
			} else {
				format!("{name} as *mut _", name = name)
			};
			return format!(
				"{name}.map_or({null_ptr}, |{name}| {arg})",
				name = name,
				null_ptr = constness.with(self.constness()).rust_null_ptr(),
				arg = arg,
			);
		}
		match self.as_char8() {
			Some(Signedness::Unsigned) => {
				return format!("u8::try_from({name})?", name = name);
			}
			Some(Signedness::Signed) => {
				return format!("u8::try_from({name})? as i8", name = name);
			}
			None => {}
		}
		name.to_string()
	}

	fn rust_arg_forward(&self, name: &str) -> String {
		name.to_string()
	}

	fn rust_arg_post_call(&self, name: &str, _is_function_infallible: bool) -> String {
		match self.as_string() {
			Some(Dir::Out(StrType::StdString(StrEnc::Text) | StrType::CvString(StrEnc::Text) | StrType::CharPtr)) => {
				format!("string_arg_output_receive!({name}_via => {name})", name = name)
			}
			Some(Dir::Out(StrType::StdString(StrEnc::Binary) | StrType::CvString(StrEnc::Binary))) => {
				format!("byte_string_arg_output_receive!({name}_via => {name})", name = name)
			}
			_ => "".to_string(),
		}
	}

	fn rust_extern(&self, dir: ExternDir) -> Cow<str> {
		let constness = match dir {
			ExternDir::Pure => self.constness(),
			ExternDir::ToCpp(constness) => constness.with(self.constness()),
			ExternDir::FromCpp => Constness::Mut,
		};
		#[allow(clippy::never_loop)] // fixme use named block when MSRV is 1.65
		'typ: loop {
			if let Some(arg_dir) = self.as_string() {
				break 'typ match dir {
					ExternDir::ToCpp(_) | ExternDir::Pure => match arg_dir {
						Dir::In(_) => format!("*{}c_char", constness.rust_qual(true)).into(),
						Dir::Out(_) => "*mut *mut c_void".into(),
					},
					ExternDir::FromCpp => "*mut c_void".into(),
				};
			}
			if self.is_extern_by_ptr() {
				break 'typ if constness.is_const() {
					"*const c_void"
				} else {
					"*mut c_void"
				}
				.into();
			}
			if let Some(inner) = self.as_pointer().or_else(|| self.as_reference()) {
				let mut out = String::with_capacity(64);
				write!(out, "*{}", self.constness().rust_qual(true)).expect("Impossible");
				if inner.is_void() {
					out += "c_void";
				} else if self.as_string().is_some() {
					out += "c_char";
				} else {
					out += inner.rust_extern(ExternDir::Pure).as_ref()
				}
				break 'typ out.into();
			}
			if let Some((elem, len)) = self.as_fixed_array() {
				break 'typ format!(
					"*{cnst}[{typ}; {len}]",
					cnst = self.constness().rust_qual(true),
					typ = elem.rust_extern(ExternDir::Pure),
					len = len,
				)
				.into();
			}
			if let Some(elem) = self.as_variable_array() {
				let typ = if matches!(elem.as_string(), Some(Dir::Out(StrType::CharPtr))) {
					// kind of special casing for cv_startLoop_int__X__int__charXX__int_charXX, without that
					// argv is treated as array of output arguments and it doesn't seem to be meant this way
					format!("*{cnst}c_char", cnst = elem.clang_constness().rust_qual(true)).into()
				} else {
					elem.rust_extern(ExternDir::Pure)
				};
				break 'typ format!("*{cnst}{typ}", cnst = self.constness().rust_qual(true), typ = typ).into();
			}
			if let Some(func) = self.as_function() {
				break 'typ func.rust_extern().into_owned().into();
			}
			if self.as_simple_class().is_some() && matches!(dir, ExternDir::ToCpp(_)) {
				break 'typ format!("*const {}", self.rust_name(NameStyle::ref_())).into();
			}
			break 'typ self.rust_name(NameStyle::ref_());
		}
	}

	fn rust_return(&self, turbo_fish_style: FishStyle, is_static_func: bool) -> Cow<str> {
		if self.as_abstract_class_ptr().is_some() {
			format!(
				"types::AbstractRef{mut_suf}{fish}<{lt}{typ}>",
				mut_suf = if self.constness().is_const() {
					""
				} else {
					"Mut"
				},
				fish = turbo_fish_style.rust_qual(),
				lt = if is_static_func {
					"'static, "
				} else {
					""
				},
				typ = self.source().rust_name(NameStyle::Reference(turbo_fish_style)),
			)
			.into()
		} else if self.is_extern_by_ptr() {
			self
				.source()
				.rust_name(NameStyle::Reference(turbo_fish_style))
				.into_owned()
				.into()
		} else {
			self.rust_name(NameStyle::Reference(turbo_fish_style))
		}
	}

	fn rust_extern_return_fallible(&self) -> Cow<str> {
		if self.is_void() {
			"Result_void".into()
		} else {
			format!("Result<{ext}>", ext = self.rust_extern(ExternDir::FromCpp)).into()
		}
	}

	fn rust_lifetime_count(&self) -> usize {
		if self.as_string().is_some() {
			0
		} else {
			match self.kind() {
				Kind::Pointer(inner) => {
					if inner.is_void() {
						0
					} else {
						1 + inner.rust_lifetime_count()
					}
				}
				Kind::Reference(inner) => {
					if !((inner.as_simple_class().is_some() || inner.is_enum()) && inner.clang_constness().is_const()) {
						1 + inner.rust_lifetime_count()
					} else {
						0
					}
				}
				Kind::Typedef(tdef) => tdef.underlying_type_ref().rust_lifetime_count(),
				_ => 0,
			}
		}
	}

	// fn rust_lifetimes(&self) -> impl Iterator<Item = Lifetime> {
	// 	Lifetime::explicit().into_iter().take(self.rust_lifetime_count())
	// }

	fn cpp_arg_func_decl(&self, name: &str) -> String {
		if matches!(self.as_string(), Some(Dir::Out(_))) || self.as_simple_class().is_some() {
			return format!("{typ}* {name}", typ = self.cpp_extern(), name = name);
		}
		self.cpp_extern_with_name(name).into_owned()
	}

	fn cpp_arg_pre_call(&self, name: &str) -> String {
		match self.as_string() {
			Some(Dir::Out(StrType::StdString(_))) => {
				format!("std::string {name}_out", name = name)
			}
			Some(Dir::Out(StrType::CvString(_))) => {
				format!("cv::String {name}_out", name = name)
			}
			Some(Dir::Out(StrType::CharPtr)) => {
				format!("char* {name}_out = new char[1024]()", name = name)
			}
			Some(Dir::In(_)) | None => "".to_string(),
		}
	}

	fn cpp_arg_func_call<'a>(&self, name: impl Into<Cow<'a, str>>) -> Cow<'a, str> {
		let name = name.into();

		match self.as_string() {
			Some(Dir::Out(str_type)) => {
				let ptr = if str_type != StrType::CharPtr && self.as_pointer().is_some() {
					"&"
				} else {
					""
				};
				return format!("{ptr}{name}_out", ptr = ptr, name = name).into();
			}
			Some(Dir::In(StrType::StdString(_))) => {
				return format!("std::string({name})", name = name).into();
			}
			Some(Dir::In(StrType::CvString(_))) => {
				return format!("cv::String({name})", name = name).into();
			}
			Some(Dir::In(StrType::CharPtr)) | None => {}
		}
		if self.is_by_move() {
			return format!("std::move(*{name})", name = name).into();
		}
		if self.is_extern_by_ptr() {
			return if self.as_pointer().is_some() {
				name
			} else {
				format!("*{name}", name = name).into()
			};
		}
		if self.as_reference().is_some() || self.as_fixed_array().is_some() || self.as_simple_class().is_some() {
			return format!("*{name}", name = name).into();
		}
		name
	}

	fn cpp_arg_post_call(&self, name: &str) -> String {
		match self.as_string() {
			Some(Dir::Out(StrType::StdString(StrEnc::Text) | StrType::CvString(StrEnc::Text))) => {
				format!("*{name} = ocvrs_create_string({name}_out.c_str())", name = name)
			}
			Some(Dir::Out(StrType::CharPtr)) => {
				format!("*{name} = ocvrs_create_string({name}_out)", name = name)
			}
			Some(Dir::Out(StrType::StdString(StrEnc::Binary))) => {
				format!(
					"*{name} = ocvrs_create_byte_string({name}_out.data(), {name}_out.size())",
					name = name
				)
			}
			Some(Dir::Out(StrType::CvString(StrEnc::Binary))) => {
				format!(
					"*{name} = ocvrs_create_byte_string({name}_out.begin(), {name}_out.size())",
					name = name
				)
			}
			Some(Dir::In(_)) | None => "".to_string(),
		}
	}

	// we need cleanup as a separate step from post_call because in cv_ocl_convertTypeStr_int_int_int_charX the
	// return value is actually one of the arguments and if we free it (in post_call phase) before converting
	// to string (in return statement) it will result in UB
	fn cpp_arg_cleanup(&self, name: &str) -> String {
		if let Some(Dir::Out(StrType::CharPtr)) = self.as_string() {
			return format!("delete[] {name}_out", name = name);
		}
		"".to_string()
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lifetime {
	Elided,
	Static,
	Explicit(u8),
}

impl Lifetime {
	pub fn elided() -> Self {
		Self::Elided
	}

	pub fn statik() -> Self {
		Self::Static
	}

	pub fn explicit() -> Self {
		Self::Explicit(0)
	}

	pub fn is_elided(&self) -> bool {
		matches!(self, Self::Elided)
	}

	pub fn is_explicit(&self) -> bool {
		matches!(self, Self::Explicit(_))
	}

	pub fn next(self) -> Option<Self> {
		match self {
			Self::Elided => Some(Self::Elided),
			Self::Static => Some(Self::Static),
			Self::Explicit(n) if n >= 25 => None,
			Self::Explicit(n) => Some(Self::Explicit(n + 1)),
		}
	}
}

impl IntoIterator for Lifetime {
	type Item = Lifetime;
	type IntoIter = LifetimeIterator;

	fn into_iter(self) -> LifetimeIterator {
		LifetimeIterator {
			cur_lifetime: Some(self),
		}
	}
}

impl fmt::Display for Lifetime {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		#[inline]
		fn write_align(f: &mut fmt::Formatter) -> fmt::Result {
			if f.align().is_some() {
				match f.fill() {
					',' => f.write_str(", ")?,
					' ' => f.write_char(' ')?,
					_ => {}
				}
			}
			Ok(())
		}
		match *self {
			Self::Elided => Ok(()),
			Self::Static => {
				f.write_str("'static")?;
				write_align(f)
			}
			Self::Explicit(n) if n >= 25 => {
				panic!("Too many lifetimes")
			}
			Self::Explicit(n) => {
				f.write_char('\'')?;
				f.write_char(char::from(b'a' + n))?;
				write_align(f)
			}
		}
	}
}

pub struct LifetimeIterator {
	cur_lifetime: Option<Lifetime>,
}

impl Iterator for LifetimeIterator {
	type Item = Lifetime;

	fn next(&mut self) -> Option<Self::Item> {
		let out = self.cur_lifetime;
		self.cur_lifetime = self.cur_lifetime.and_then(|l| l.next());
		out
	}
}
