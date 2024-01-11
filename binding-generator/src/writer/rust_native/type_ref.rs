use std::borrow::Cow;
use std::fmt;
use std::fmt::Write;

use crate::renderer::TypeRefRenderer;
use crate::type_ref::{Constness, Dir, ExternDir, FishStyle, NameStyle, StrEnc, StrType, TypeRef, TypeRefKind, TypeRefTypeHint};
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

	/// For when a type needs to be part of the user-visible Rust method name
	///
	/// Return a lightweight lowercase type representation, might not be precise. For example it's used for operator bindings so
	/// that `operator &` on 2 `Mat`s translates into `and_mat_mat()`.
	fn rust_simple_name(&self) -> String;
	fn rust_name(&self, name_style: NameStyle) -> Cow<str>;
	fn rust_name_ext(&self, name_style: NameStyle, lifetime: Lifetime) -> Cow<str>;

	fn rust_self_func_decl(&self, method_constness: Constness) -> String;
	fn rust_arg_func_decl(&self, name: &str) -> String;
	fn rust_extern_self_func_decl(&self, method_constness: Constness) -> String;
	fn rust_extern_arg_func_decl(&self, name: &str) -> String;
	fn rust_arg_pre_call(&self, name: &str, is_function_infallible: bool) -> String;
	fn rust_self_func_call(&self, method_constness: Constness) -> String;
	fn rust_arg_func_call(&self, name: &str) -> String;
	fn rust_arg_forward<'n>(&self, name: &'n str) -> Cow<'n, str>;
	fn rust_arg_post_success_call(&self, name: &str) -> String;
	fn rust_extern(&self, dir: ExternDir) -> Cow<str>;
	fn rust_return(&self, turbo_fish_style: FishStyle, is_static_func: bool) -> Cow<str>;
	fn rust_extern_return_fallible(&self) -> Cow<str>;
	fn rust_lifetime_count(&self) -> usize;

	fn cpp_self_func_decl(&self, method_constness: Constness) -> String;
	fn cpp_arg_func_decl(&self, name: &str) -> String;
	fn cpp_arg_pre_call(&self, name: &str) -> String;
	fn cpp_arg_func_call<'a>(&self, name: impl Into<Cow<'a, str>>) -> Cow<'a, str>;
	fn cpp_arg_post_call(&self, name: &str) -> String;
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
		if add_const && self.inherent_constness().is_const() {
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

	fn rust_simple_name(&self) -> String {
		let maybe_ptr = self.as_pointer_reference_move();
		let type_ref = maybe_ptr.as_ref().unwrap_or(self);
		type_ref.rust_name(NameStyle::Declaration).to_lowercase()
	}

	fn rust_name(&self, name_style: NameStyle) -> Cow<str> {
		self.rust_name_ext(name_style, Lifetime::elided())
	}

	fn rust_name_ext(&self, name_style: NameStyle, lifetime: Lifetime) -> Cow<str> {
		RustRenderer::new(name_style, lifetime, self.is_rust_by_ptr()).render(self)
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
		let typ = if let Some(dir) = self.as_string() {
			match dir {
				Dir::In(str_type) => if str_type.is_binary() {
					"&[u8]"
				} else {
					"&str"
				}
				.into(),
				Dir::Out(str_type) => if str_type.is_binary() {
					"&mut Vec<u8>"
				} else {
					"&mut String"
				}
				.into(),
			}
		} else if self.is_input_array() {
			"&impl core::ToInputArray".into()
		} else if self.is_output_array() {
			"&mut impl core::ToOutputArray".into()
		} else if self.is_input_output_array() {
			"&mut impl core::ToInputOutputArray".into()
		} else if let Some((_, size)) = self.as_string_array() {
			self.format_as_array("&str", size).into()
		} else if let Some((tref, cls)) = self.as_abstract_class_ptr() {
			let constness = tref.constness();
			format!(
				"&{cnst}impl {name}",
				cnst = constness.rust_qual(),
				name = cls.rust_trait_name(NameStyle::ref_(), constness)
			)
			.into()
		} else if self.is_rust_char() {
			"char".into()
		} else {
			self.rust_name(NameStyle::ref_())
		};
		let cnst = Constness::from_is_mut(
			self.as_by_move().is_some()
				|| self.extern_pass_kind().is_by_void_ptr()
					&& self.constness().is_mut()
					&& !self.as_pointer().is_some()
					&& !self.as_reference().is_some(),
		)
		.rust_qual();
		format!("{cnst}{name}: {typ}")
	}

	fn rust_extern_self_func_decl(&self, method_constness: Constness) -> String {
		self
			.with_inherent_constness(method_constness)
			.rust_extern_arg_func_decl("instance")
	}

	fn rust_extern_arg_func_decl(&self, name: &str) -> String {
		format!("{name}: {typ}", typ = self.rust_extern(ExternDir::ToCpp))
	}

	fn rust_arg_pre_call(&self, name: &str, is_function_infallible: bool) -> String {
		if let Some(dir) = self.as_string() {
			return match dir {
				Dir::In(_) => {
					let mut flags = vec![];
					if is_function_infallible {
						flags.push("nofail")
					}
					let mut flags = flags.join(" ");
					if !flags.is_empty() {
						flags.push(' ');
					}
					format!("extern_container_arg!({flags}{name})")
				}
				Dir::Out(_) => format!("string_arg_output_send!(via {name}_via)"),
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
			if let Some((userdata_name, _)) = args.iter().find(|(_, arg)| arg.is_user_data()) {
				let tramp_args = args
					.iter()
					.map(|(name, arg)| (name == userdata_name, arg.type_ref().rust_extern_arg_func_decl(name)))
					.collect::<Vec<_>>();
				let fw_args = tramp_args
					.iter()
					.filter(|(is_user_data, _)| !is_user_data)
					.map(|(_, decl)| decl)
					.join(", ");
				let ret = func.return_type();
				return format!(
					"callback_arg!({name}_trampoline({tramp_args}) -> {tramp_ret} => {userdata_name} in callbacks => {name}({fw_args}) -> {fw_ret})",
					tramp_args = tramp_args.iter().map(|(_, decl)| decl).join(", "),
					tramp_ret = ret.rust_extern(ExternDir::FromCpp),
					fw_ret = ret.rust_extern(ExternDir::FromCpp),
				);
			}
		}
		"".to_string()
	}

	fn rust_self_func_call(&self, method_constness: Constness) -> String {
		self.with_inherent_constness(method_constness).rust_arg_func_call("self")
	}

	fn rust_arg_func_call(&self, name: &str) -> String {
		let constness = self.constness();
		if let Some(dir) = self.as_string() {
			match dir {
				Dir::In(_) => {
					format!("{name}.opencv_as_extern()")
				}
				Dir::Out(_) => format!("&mut {name}_via"),
			}
		} else if self.as_reference().map_or(false, |inner| {
			(inner.as_simple_class().is_some() || inner.is_enum()) && (inner.constness().is_const() || self.as_by_move().is_some())
		}) {
			format!("&{cnst}{name}", cnst = constness.rust_qual())
		} else if self.as_simple_class().is_some() {
			format!("{name}.opencv_as_extern()")
		} else if self.extern_pass_kind().is_by_void_ptr() {
			let typ = self.source();
			let by_ptr = if constness.is_const() {
				format!("{name}.as_raw_{rust_safe_id}()", rust_safe_id = typ.rust_safe_id(false))
			} else {
				format!("{name}.as_raw_mut_{rust_safe_id}()", rust_safe_id = typ.rust_safe_id(false))
			};
			if self.is_nullable() {
				format!(
					"{name}.map_or({null_ptr}, |{name}| {by_ptr})",
					null_ptr = constness.rust_null_ptr()
				)
			} else {
				by_ptr
			}
		} else if self.as_variable_array().is_some() {
			let arr = if constness.is_const() {
				format!("{name}.as_ptr()")
			} else {
				format!("{name}.as_mut_ptr()")
			};
			if self.is_nullable() {
				format!(
					"{name}.map_or({null_ptr}, |{name}| {arr})",
					null_ptr = constness.rust_null_ptr()
				)
			} else {
				arr
			}
		} else if self
			.as_function()
			.map_or(false, |f| f.arguments().into_iter().any(|a| a.is_user_data()))
		{
			format!("{name}_trampoline")
		} else if let Some(inner) = self.as_pointer().filter(|inner| inner.as_pointer().is_some()) {
			// some special care for double pointers
			return format!(
				"{name} as *{cnst} _ as *{cnst} *{const_inner} _",
				cnst = self.inherent_constness().rust_qual_ptr(),
				const_inner = inner.inherent_constness().rust_qual_ptr()
			);
		} else if self.is_nullable() && (self.as_reference().is_some() || self.as_pointer().is_some()) {
			// unwrap_or doesn't work here because reference doesn't coerce to pointer in this case
			format!("{name}.map_or({null_ptr}, |x| x)", null_ptr = constness.rust_null_ptr())
		} else if self.is_rust_char() {
			format!("u8::try_from({name})? as c_char")
		} else {
			name.to_string()
		}
	}

	fn rust_arg_forward<'n>(&self, name: &'n str) -> Cow<'n, str> {
		name.into()
	}

	fn rust_arg_post_success_call(&self, name: &str) -> String {
		match self.as_string() {
			Some(Dir::Out(_)) => {
				format!("string_arg_output_receive!({name}_via => {name})")
			}
			_ => "".to_string(),
		}
	}

	fn rust_extern(&self, dir: ExternDir) -> Cow<str> {
		let constness = match dir {
			ExternDir::Contained | ExternDir::ToCpp => self.constness(),
			ExternDir::FromCpp => Constness::Mut,
		};
		if let Some(arg_dir) = self.as_string() {
			match dir {
				ExternDir::ToCpp | ExternDir::Contained => match arg_dir {
					Dir::In(_) => format!("*{cnst}c_char", cnst = Constness::Const.rust_qual_ptr()).into(),
					Dir::Out(_) => "*mut *mut c_void".into(),
				},
				ExternDir::FromCpp => "*mut c_void".into(),
			}
		} else if self.extern_pass_kind().is_by_void_ptr() {
			format!("*{cnst}c_void", cnst = constness.rust_qual_ptr()).into()
		} else if let Some(inner) = self.as_pointer().or_else(|| self.as_reference()) {
			let typ = if inner.is_void() {
				"c_void".into()
			} else if self.as_string().is_some() {
				"c_char".into()
			} else {
				inner.rust_extern(ExternDir::Contained)
			};
			format!("*{cnst}{typ}", cnst = self.constness().rust_qual_ptr()).into()
		} else if let Some((elem, len)) = self.as_array() {
			if let Some(len) = len {
				format!(
					"*{cnst}[{typ}; {len}]",
					cnst = self.constness().rust_qual_ptr(),
					typ = elem.rust_extern(ExternDir::Contained),
				)
			} else {
				let typ = if matches!(elem.as_string(), Some(Dir::Out(StrType::CharPtr(_)))) {
					// kind of special casing for cv_startLoop_int__X__int__charXX__int_charXX, without that
					// argv is treated as array of output arguments and it doesn't seem to be meant this way
					format!("*{cnst}c_char", cnst = elem.inherent_constness().rust_qual_ptr()).into()
				} else {
					elem.rust_extern(ExternDir::Contained)
				};
				format!("*{cnst}{typ}", cnst = self.constness().rust_qual_ptr())
			}
			.into()
		} else if let Some(func) = self.as_function() {
			func.rust_extern().into_owned().into()
		} else if self.as_simple_class().is_some() && matches!(dir, ExternDir::ToCpp) {
			format!("*const {}", self.rust_name(NameStyle::ref_())).into()
		} else {
			self.rust_name(NameStyle::ref_())
		}
	}

	fn rust_return(&self, turbo_fish_style: FishStyle, is_static_func: bool) -> Cow<str> {
		if self.as_abstract_class_ptr().is_some() {
			let lt = if is_static_func {
				"'static, "
			} else {
				""
			};
			format!(
				"types::AbstractRef{mut_suf}{fish}<{lt}{typ}>",
				mut_suf = self.constness().rust_name_qual(),
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
			"ResultVoid".into()
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
					if self.is_rust_by_ptr() {
						0
					} else {
						1 + inner.rust_lifetime_count()
					}
				}
				TypeRefKind::Reference(inner) => {
					if !((inner.as_simple_class().is_some() || inner.is_enum()) && inner.inherent_constness().is_const()) {
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

	fn cpp_self_func_decl(&self, method_constness: Constness) -> String {
		let mut self_with_constness = self.with_inherent_constness(method_constness);
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
			Some(Dir::Out(StrType::CharPtr(str_enc))) => {
				let len = if matches!(str_enc, StrEnc::Binary) {
					if let TypeRefTypeHint::StringAsBytes(Some(len_arg_name)) = self.type_hint() {
						len_arg_name.as_str()
					} else {
						"1024"
					}
				} else {
					"1024"
				};
				format!("std::unique_ptr<char[]> {name}_out = std::make_unique<char[]>({len})")
			}
			Some(Dir::In(_)) | None => "".to_string(),
		}
	}

	fn cpp_arg_func_call<'a>(&self, name: impl Into<Cow<'a, str>>) -> Cow<'a, str> {
		let name = name.into();

		match self.as_string() {
			Some(Dir::Out(str_type)) => {
				return match str_type {
					StrType::StdString(_) | StrType::CvString(_) => {
						let ptr = if self.as_pointer().is_some() {
							"&"
						} else {
							""
						};
						format!("{ptr}{name}_out")
					}
					StrType::CharPtr(_) => format!("{name}_out.get()"),
				}
				.into();
			}
			Some(Dir::In(StrType::StdString(_))) => {
				return format!("std::string({name})").into();
			}
			Some(Dir::In(StrType::CvString(_))) => {
				return format!("cv::String({name})").into();
			}
			Some(Dir::In(StrType::CharPtr(_))) | None => {}
		}
		if self.as_by_move().is_some() {
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
			Some(Dir::Out(StrType::StdString(StrEnc::Binary))) => {
				format!("*{name} = ocvrs_create_byte_string({name}_out.data(), {name}_out.size())")
			}
			Some(Dir::Out(StrType::CvString(StrEnc::Binary))) => {
				format!("*{name} = ocvrs_create_byte_string({name}_out.begin(), {name}_out.size())")
			}
			Some(Dir::Out(StrType::CharPtr(StrEnc::Text))) => {
				format!("*{name} = ocvrs_create_string({name}_out.get())")
			}
			Some(Dir::Out(StrType::CharPtr(StrEnc::Binary))) => {
				if let TypeRefTypeHint::StringAsBytes(Some(len_arg_name)) = self.type_hint() {
					format!("*{name} = ocvrs_create_byte_string({name}_out.get(), {len_arg_name})")
				} else {
					panic!("Output argument of type `char*` with binary encoding must have `len` argument specified")
				}
			}
			Some(Dir::In(_)) | None => "".to_string(),
		}
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
			Self::Explicit(n) if n > 25 => {
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
