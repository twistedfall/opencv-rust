use std::borrow::Cow;
use std::fmt;
use std::fmt::Write;

use crate::type_ref::{Constness, Dir, ExternDir, FishStyle, NameStyle, StrEnc, StrType, TypeRef, TypeRefKind};
use crate::writer::rust_native::class::ClassExt;
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
	fn rust_extern_arg_func_decl(&self, name: &str) -> String;
	fn rust_arg_pre_call(&self, name: &str, is_function_infallible: bool) -> String;
	fn rust_userdata_pre_call(&self, name: &str, callback_name: &str) -> String;
	fn rust_self_func_call(&self, method_constness: Constness) -> String;
	fn rust_arg_func_call(&self, name: &str) -> String;
	fn rust_arg_forward(&self, name: &str) -> String;
	fn rust_arg_post_call(&self, name: &str, _is_function_infallible: bool) -> String;
	fn rust_extern(&self, dir: ExternDir) -> Cow<str>;
	fn rust_return(&self, turbo_fish_style: FishStyle, is_static_func: bool) -> Cow<str>;
	fn rust_extern_return_fallible(&self) -> Cow<str>;
	fn rust_lifetime_count(&self) -> usize;

	fn cpp_self_func_decl(&self, method_constness: Constness) -> String;
	fn cpp_arg_func_decl(&self, name: &str) -> String;
	fn cpp_arg_pre_call(&self, name: &str) -> String;
	fn cpp_arg_func_call<'a>(&self, name: impl Into<Cow<'a, str>>) -> Cow<'a, str>;
	fn cpp_arg_post_call(&self, name: &str) -> String;
	fn cpp_arg_cleanup(&self, name: &str) -> String;
}

impl TypeRefExt for TypeRef<'_, '_> {
	fn format_as_array(&self, elem_type: &str, size: Option<usize>) -> String {
		format!(
			"&{cnst}[{elem_type}{size}]",
			cnst = self.constness().rust_qual(),
			size = size.map_or_else(|| "".to_string(), |s| format!("; {s}"))
		)
	}

	fn rust_safe_id(&self, add_const: bool) -> Cow<str> {
		let mut out = String::with_capacity(64);
		if add_const && self.clang_constness().is_const() {
			out.push_str("const_");
		}
		match self.kind().as_ref() {
			TypeRefKind::Array(inner, ..) => {
				out.push_str(&inner.rust_safe_id(add_const));
				out.push_str("_X");
			}
			TypeRefKind::StdVector(vec) => out.push_str(&vec.rust_localalias()),
			TypeRefKind::StdTuple(tuple) => out.push_str(&tuple.rust_localalias()),
			TypeRefKind::Pointer(inner) => {
				out.push_str(&inner.rust_safe_id(add_const));
				if !self.extern_pass_kind().is_by_ptr() {
					out.push_str("_X");
				}
			}
			TypeRefKind::Reference(inner) | TypeRefKind::RValueReference(inner) => out.push_str(&inner.rust_safe_id(add_const)),
			TypeRefKind::SmartPtr(ptr) => out.push_str(&ptr.rust_localalias()),
			TypeRefKind::Class(cls) => out.push_str(&cls.rust_name(NameStyle::decl())),
			TypeRefKind::Primitive(..)
			| TypeRefKind::Enum(..)
			| TypeRefKind::Function(..)
			| TypeRefKind::Typedef(..)
			| TypeRefKind::Generic(..)
			| TypeRefKind::Ignored => out.push_str(&self.rust_name(NameStyle::decl())),
		}
		out.cleanup_name();
		out.into()
	}

	fn rust_module(&self) -> Cow<str> {
		match self.kind().as_ref() {
			TypeRefKind::Primitive(..) => "core".into(),
			TypeRefKind::StdVector(vec) => vec.rust_element_module().into_owned().into(),
			TypeRefKind::StdTuple(tuple) => tuple.rust_element_module().into_owned().into(),
			TypeRefKind::Array(inner, ..)
			| TypeRefKind::Pointer(inner)
			| TypeRefKind::Reference(inner)
			| TypeRefKind::RValueReference(inner) => inner.rust_module().into_owned().into(),
			TypeRefKind::SmartPtr(ptr) => ptr.rust_module().into_owned().into(),
			TypeRefKind::Class(cls) => cls.rust_module().into_owned().into(),
			TypeRefKind::Enum(enm) => enm.rust_module().into_owned().into(),
			TypeRefKind::Function(..) => {
				"core".into() // fixme
			}
			TypeRefKind::Typedef(tdef) => tdef.rust_module().into_owned().into(),
			TypeRefKind::Generic(..) | TypeRefKind::Ignored => "core".into(),
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
		if self.extern_pass_kind().is_by_void_ptr() {
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
				break 'decl_type "&impl core::ToInputArray".into();
			} else if self.is_output_array() {
				break 'decl_type "&mut impl core::ToOutputArray".into();
			} else if self.is_input_output_array() {
				break 'decl_type "&mut impl core::ToInputOutputArray".into();
			} else if let Some((_, size)) = self.as_string_array() {
				break 'decl_type self.format_as_array("&str", size).into();
			} else if self.is_rust_char() {
				break 'decl_type "char".into();
			} else if let Some((tref, cls)) = self.as_abstract_class_ptr() {
				let constness = tref.constness();
				break 'decl_type format!(
					"&{cnst}impl {name}",
					cnst = constness.rust_qual(),
					name = cls.rust_trait_name(NameStyle::ref_(), constness)
				)
				.into();
			}
			break 'decl_type self.rust_name(NameStyle::ref_());
		};
		let cnst = Constness::from_is_mut(
			self.is_by_move()
				|| self.extern_pass_kind().is_by_void_ptr()
					&& self.constness().is_mut()
					&& !self.as_pointer().is_some()
					&& !self.as_reference().is_some(),
		)
		.rust_qual();
		format!("{cnst}{name}: {typ}")
	}

	fn rust_extern_self_func_decl(&self, method_constness: Constness) -> String {
		self.with_constness(method_constness).rust_extern_arg_func_decl("instance")
	}

	fn rust_extern_arg_func_decl(&self, name: &str) -> String {
		format!("{name}: {typ}", typ = self.rust_extern(ExternDir::ToCpp))
	}

	fn rust_arg_pre_call(&self, name: &str, is_function_infallible: bool) -> String {
		let extern_container_arg = || {
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
			format!("extern_container_arg!({flags}{name})")
		};

		if let Some(dir) = self.as_string() {
			return match dir {
				Dir::In(_) => extern_container_arg(),
				Dir::Out(_) => {
					format!("string_arg_output_send!(via {name}_via)")
				}
			};
		} else if self.is_input_array() {
			return format!("input_array_arg!({name})");
		} else if self.is_output_array() {
			return format!("output_array_arg!({name})");
		} else if self.is_input_output_array() {
			return format!("input_output_array_arg!({name})");
		} else if self.as_string_array().is_some() {
			return if self.constness().is_const() {
				format!("string_array_arg!({name})")
			} else {
				format!("string_array_arg_mut!({name})")
			};
		} else if let Some(func) = self.as_function() {
			let args = rust_disambiguate_names(func.arguments()).collect::<Vec<_>>();
			if let Some((userdata_name, _)) = args.iter().find(|(_, f)| f.is_user_data()).cloned() {
				let ret = func.return_type();
				let tramp_args = args
					.into_iter()
					.map(|(name, a)| a.type_ref().rust_extern_arg_func_decl(&name))
					.join(", ");
				let fw_args = rust_disambiguate_names(func.rust_arguments())
					.map(|(name, a)| a.type_ref().rust_extern_arg_func_decl(&name))
					.join(", ");
				return format!(
					"callback_arg!({name}_trampoline({tramp_args}) -> {tramp_ret} => {userdata_name} in callbacks => {name}({fw_args}) -> {fw_ret})",
					tramp_ret=ret.rust_extern(ExternDir::FromCpp),
					fw_ret=ret.rust_extern(ExternDir::FromCpp),
				);
			}
		}
		"".to_string()
	}

	fn rust_userdata_pre_call(&self, name: &str, callback_name: &str) -> String {
		format!("userdata_arg!({name} in callbacks => {callback_name})")
	}

	fn rust_self_func_call(&self, method_constness: Constness) -> String {
		self.with_constness(method_constness).rust_arg_func_call("self")
	}

	fn rust_arg_func_call(&self, name: &str) -> String {
		let constness = self.constness();
		if let Some(dir) = self.as_string() {
			return match dir {
				Dir::In(_) => {
					if constness.is_const() {
						format!("{name}.opencv_as_extern()")
					} else {
						format!("{name}.opencv_as_extern_mut()")
					}
				}
				Dir::Out(_) => format!("&mut {name}_via"),
			};
		}
		if self.as_reference().map_or(false, |inner| {
			(inner.as_simple_class().is_some() || inner.is_enum()) && (inner.constness().is_const() || self.is_by_move())
		}) {
			return format!("&{cnst}{name}", cnst = constness.rust_qual());
		}
		if self.as_simple_class().is_some() {
			return format!("{name}.opencv_as_extern()");
		}
		if self.extern_pass_kind().is_by_void_ptr() {
			let typ = self.source();
			let by_ptr = if constness.is_const() {
				format!("{name}.as_raw_{rust_safe_id}()", rust_safe_id = typ.rust_safe_id(false))
			} else {
				format!("{name}.as_raw_mut_{rust_safe_id}()", rust_safe_id = typ.rust_safe_id(false))
			};
			return if self.is_nullable() {
				format!(
					"{name}.map_or({null_ptr}, |{name}| {by_ptr})",
					null_ptr = constness.rust_null_ptr()
				)
			} else {
				by_ptr
			};
		}
		if self.as_variable_array().is_some() {
			let arr = if constness.is_const() {
				format!("{name}.as_ptr()")
			} else {
				format!("{name}.as_mut_ptr()")
			};
			return if self.is_nullable() {
				format!(
					"{name}.map_or({null_ptr}, |{name}| {arr})",
					null_ptr = constness.rust_null_ptr()
				)
			} else {
				arr
			};
		}
		if let Some(func) = self.as_function() {
			if func.arguments().into_iter().any(|a| a.is_user_data()) {
				return format!("{name}_trampoline");
			}
		}
		if let Some(inner) = self.as_pointer() {
			if inner.as_pointer().is_some() {
				// some special care for double pointers
				return format!(
					"{name} as *{cnst} _ as *{cnst} *{const_inner} _",
					cnst = self.clang_constness().rust_qual_ptr(),
					const_inner = inner.clang_constness().rust_qual_ptr()
				);
			}
		}
		if self.is_nullable() && (self.as_reference().is_some() || self.as_pointer().is_some()) {
			let arg = if constness.is_const() {
				format!("{name} as *const _")
			} else {
				format!("{name} as *mut _")
			};
			return format!(
				"{name}.map_or({null_ptr}, |{name}| {arg})",
				null_ptr = constness.rust_null_ptr(),
			);
		}
		if self.is_rust_char() {
			return format!("u8::try_from({name})? as c_char");
		}
		name.to_string()
	}

	fn rust_arg_forward(&self, name: &str) -> String {
		name.to_string()
	}

	fn rust_arg_post_call(&self, name: &str, _is_function_infallible: bool) -> String {
		match self.as_string() {
			Some(Dir::Out(StrType::StdString(StrEnc::Text) | StrType::CvString(StrEnc::Text) | StrType::CharPtr)) => {
				format!("string_arg_output_receive!({name}_via => {name})")
			}
			Some(Dir::Out(StrType::StdString(StrEnc::Binary) | StrType::CvString(StrEnc::Binary))) => {
				format!("byte_string_arg_output_receive!({name}_via => {name})")
			}
			_ => "".to_string(),
		}
	}

	fn rust_extern(&self, dir: ExternDir) -> Cow<str> {
		let constness = match dir {
			ExternDir::Pure | ExternDir::ToCpp => self.constness(),
			ExternDir::FromCpp => Constness::Mut,
		};
		#[allow(clippy::never_loop)] // fixme use named block when MSRV is 1.65
		'typ: loop {
			if let Some(arg_dir) = self.as_string() {
				break 'typ match dir {
					ExternDir::ToCpp | ExternDir::Pure => match arg_dir {
						Dir::In(_) => format!("*{cnst}c_char", cnst = constness.rust_qual_ptr()).into(),
						Dir::Out(_) => "*mut *mut c_void".into(),
					},
					ExternDir::FromCpp => "*mut c_void".into(),
				};
			}
			if self.extern_pass_kind().is_by_void_ptr() {
				break 'typ format!("*{cnst}c_void", cnst = constness.rust_qual_ptr()).into();
			}
			if let Some(inner) = self.as_pointer().or_else(|| self.as_reference()) {
				let mut out = String::with_capacity(64);
				write!(out, "*{}", self.constness().rust_qual_ptr()).expect("Impossible");
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
					cnst = self.constness().rust_qual_ptr(),
					typ = elem.rust_extern(ExternDir::Pure),
				)
				.into();
			}
			if let Some(elem) = self.as_variable_array() {
				let typ = if matches!(elem.as_string(), Some(Dir::Out(StrType::CharPtr))) {
					// kind of special casing for cv_startLoop_int__X__int__charXX__int_charXX, without that
					// argv is treated as array of output arguments and it doesn't seem to be meant this way
					format!("*{cnst}c_char", cnst = elem.clang_constness().rust_qual_ptr()).into()
				} else {
					elem.rust_extern(ExternDir::Pure)
				};
				break 'typ format!("*{cnst}{typ}", cnst = self.constness().rust_qual_ptr()).into();
			}
			if let Some(func) = self.as_function() {
				break 'typ func.rust_extern().into_owned().into();
			}
			if self.as_simple_class().is_some() && matches!(dir, ExternDir::ToCpp) {
				break 'typ format!("*const {}", self.rust_name(NameStyle::ref_())).into();
			}
			break 'typ self.rust_name(NameStyle::ref_());
		}
	}

	fn rust_return(&self, turbo_fish_style: FishStyle, is_static_func: bool) -> Cow<str> {
		if self.as_abstract_class_ptr().is_some() {
			let mut_suf = if self.constness().is_const() {
				""
			} else {
				"Mut"
			};
			let lt = if is_static_func {
				"'static, "
			} else {
				""
			};
			format!(
				"types::AbstractRef{mut_suf}{fish}<{lt}{typ}>",
				fish = turbo_fish_style.rust_qual(),
				typ = self.source().rust_name(NameStyle::Reference(turbo_fish_style)),
			)
			.into()
		} else if self.extern_pass_kind().is_by_void_ptr() {
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
			match self.kind().as_ref() {
				TypeRefKind::Pointer(inner) => {
					if inner.is_void() {
						0
					} else {
						1 + inner.rust_lifetime_count()
					}
				}
				TypeRefKind::Reference(inner) => {
					if !((inner.as_simple_class().is_some() || inner.is_enum()) && inner.clang_constness().is_const()) {
						1 + inner.rust_lifetime_count()
					} else {
						0
					}
				}
				TypeRefKind::Typedef(tdef) => tdef.underlying_type_ref().rust_lifetime_count(),
				_ => 0,
			}
		}
	}

	// fn rust_lifetimes(&self) -> impl Iterator<Item = Lifetime> {
	// 	Lifetime::explicit().into_iter().take(self.rust_lifetime_count())
	// }

	fn cpp_self_func_decl(&self, method_constness: Constness) -> String {
		let mut self_with_constness = self.with_constness(method_constness);
		if self.extern_pass_kind().is_by_ptr() {
			self_with_constness = TypeRef::new_pointer(self_with_constness);
		}
		self_with_constness.cpp_arg_func_decl("instance")
	}

	fn cpp_arg_func_decl(&self, name: &str) -> String {
		if matches!(self.as_string(), Some(Dir::Out(_))) || self.as_simple_class().is_some() {
			return format!("{typ}* {name}", typ = self.cpp_extern());
		}
		self.cpp_extern_with_name(name).into_owned()
	}

	fn cpp_arg_pre_call(&self, name: &str) -> String {
		match self.as_string() {
			Some(Dir::Out(StrType::StdString(_))) => {
				format!("std::string {name}_out")
			}
			Some(Dir::Out(StrType::CvString(_))) => {
				format!("cv::String {name}_out")
			}
			Some(Dir::Out(StrType::CharPtr)) => {
				format!("char* {name}_out = new char[1024]()")
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
				return format!("{ptr}{name}_out").into();
			}
			Some(Dir::In(StrType::StdString(_))) => {
				return format!("std::string({name})").into();
			}
			Some(Dir::In(StrType::CvString(_))) => {
				return format!("cv::String({name})").into();
			}
			Some(Dir::In(StrType::CharPtr)) | None => {}
		}
		if self.is_by_move() {
			return format!("std::move(*{name})").into();
		}
		if self.extern_pass_kind().is_by_ptr() {
			return if self.as_pointer().is_some() {
				name
			} else {
				format!("*{name}").into()
			};
		}
		if self.as_reference().is_some() || self.as_fixed_array().is_some() || self.as_simple_class().is_some() {
			return format!("*{name}").into();
		}
		name
	}

	fn cpp_arg_post_call(&self, name: &str) -> String {
		match self.as_string() {
			Some(Dir::Out(StrType::StdString(StrEnc::Text) | StrType::CvString(StrEnc::Text))) => {
				format!("*{name} = ocvrs_create_string({name}_out.c_str())")
			}
			Some(Dir::Out(StrType::CharPtr)) => {
				format!("*{name} = ocvrs_create_string({name}_out)")
			}
			Some(Dir::Out(StrType::StdString(StrEnc::Binary))) => {
				format!("*{name} = ocvrs_create_byte_string({name}_out.data(), {name}_out.size())")
			}
			Some(Dir::Out(StrType::CvString(StrEnc::Binary))) => {
				format!("*{name} = ocvrs_create_byte_string({name}_out.begin(), {name}_out.size())")
			}
			Some(Dir::In(_)) | None => "".to_string(),
		}
	}

	// we need cleanup as a separate step from post_call because in cv_ocl_convertTypeStr_int_int_int_charX the
	// return value is actually one of the arguments and if we free it (in post_call phase) before converting
	// to string (in return statement) it will result in UB
	fn cpp_arg_cleanup(&self, name: &str) -> String {
		if let Some(Dir::Out(StrType::CharPtr)) = self.as_string() {
			return format!("delete[] {name}_out");
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
		matches!(self, Self::Static | Self::Explicit(_))
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
