use std::borrow::Cow;

use super::{FunctionProps, RenderLaneTrait, rust_arg_func_decl, rust_self_func_decl};
use crate::NameStyle;
use crate::type_ref::{Constness, ExternDir, FishStyle, StrEnc, StrType, TypeRef, TypeRefTypeHint};
use crate::writer::rust_native::type_ref::{Lifetime, NullabilityExt, TypeRefExt};

pub struct InStringRenderLane<'tu, 'ge> {
	str_type: StrType,
	non_canonical: TypeRef<'tu, 'ge>,
}

impl<'tu, 'ge> InStringRenderLane<'tu, 'ge> {
	pub fn from_str_type_non_canonical(str_type: StrType, non_canonical: TypeRef<'tu, 'ge>) -> Self {
		Self { str_type, non_canonical }
	}
}

impl RenderLaneTrait for InStringRenderLane<'_, '_> {
	fn rust_self_func_decl(&self, lifetime: Lifetime) -> Cow<'static, str> {
		rust_self_func_decl(Constness::Const, lifetime)
	}

	fn rust_arg_func_decl(&self, name: &str, _lifetime: Lifetime) -> String {
		let typ = match self.str_type.encoding() {
			StrEnc::Text => "&str",
			StrEnc::Binary => "&[u8]",
			StrEnc::OsStr => "impl AsRef<OsStr>",
		};
		let typ = self
			.non_canonical
			.type_hint()
			.nullability()
			.rust_wrap_nullable_func_decl(typ.into(), NameStyle::Reference(FishStyle::No));
		rust_arg_func_decl(name, Constness::Const, &typ)
	}

	fn rust_arg_pre_call(&self, name: &str, function_props: &FunctionProps) -> String {
		let fail_spec = if function_props.is_infallible {
			"nofail "
		} else {
			""
		};
		let option_spec = if self.non_canonical.type_hint().nullability().is_nullable() {
			"option "
		} else {
			""
		};
		match self.str_type.encoding() {
			StrEnc::Text | StrEnc::Binary => format!("extern_container_arg!({fail_spec}{name})"),
			StrEnc::OsStr => format!("path_arg!({fail_spec}{option_spec}{name})"),
		}
	}

	fn rust_arg_func_call(&self, name: &str) -> String {
		format!("{name}.opencv_as_extern()")
	}

	fn rust_extern_arg_func_decl(&self, name: &str) -> String {
		rust_arg_func_decl(name, Constness::Const, &self.non_canonical.rust_extern(ExternDir::ToCpp))
	}

	fn cpp_arg_func_decl(&self, name: &str) -> Cow<'_, str> {
		format!("const char* {name}").into()
	}

	fn cpp_arg_func_call(&self, name: &str) -> String {
		match self.str_type {
			StrType::StdString(_) => format!("std::string({name})"),
			StrType::CvString(_) => format!("cv::String({name})"),
			StrType::CharPtr(_) => name.to_string(),
		}
	}
}

pub struct OutStringRenderLane<'tu, 'ge> {
	str_type: StrType,
	canonical: TypeRef<'tu, 'ge>,
}

impl<'tu, 'ge> OutStringRenderLane<'tu, 'ge> {
	pub fn from_str_type_canonical(str_type: StrType, canonical: TypeRef<'tu, 'ge>) -> Self {
		Self { str_type, canonical }
	}
}

impl RenderLaneTrait for OutStringRenderLane<'_, '_> {
	fn rust_self_func_decl(&self, lifetime: Lifetime) -> Cow<'static, str> {
		rust_self_func_decl(Constness::Mut, lifetime)
	}

	fn rust_arg_func_decl(&self, name: &str, _lifetime: Lifetime) -> String {
		let typ = match self.str_type.encoding() {
			StrEnc::Text => "&mut String",
			StrEnc::Binary => "&mut Vec<u8>",
			StrEnc::OsStr => "&mut OsString",
		};
		rust_arg_func_decl(name, Constness::Const, typ)
	}

	fn rust_arg_pre_call(&self, name: &str, _function_props: &FunctionProps) -> String {
		format!("string_arg_output_send!(via {name}_via)")
	}

	fn rust_arg_func_call(&self, name: &str) -> String {
		format!("&mut {name}_via")
	}

	fn rust_arg_post_call(&self, name: &str) -> String {
		format!("string_arg_output_receive!({name}_via => {name})")
	}

	fn rust_extern_arg_func_decl(&self, name: &str) -> String {
		rust_arg_func_decl(name, Constness::Const, &self.canonical.rust_extern(ExternDir::ToCpp))
	}

	fn cpp_arg_func_decl(&self, name: &str) -> Cow<'_, str> {
		format!("void** {name}").into()
	}

	fn cpp_arg_pre_call(&self, name: &str) -> String {
		match self.str_type {
			StrType::StdString(_) => format!("std::string {name}_out"),
			StrType::CvString(_) => format!("cv::String {name}_out"),
			StrType::CharPtr(str_enc) => {
				let len = match self.canonical.type_hint() {
					TypeRefTypeHint::StringAsBytes(Some(len_arg_name)) if matches!(str_enc, StrEnc::Binary) => len_arg_name.as_ref(),
					TypeRefTypeHint::StringWithLen(len_arg_name) => len_arg_name.as_ref(),
					TypeRefTypeHint::None
					| TypeRefTypeHint::Nullable
					| TypeRefTypeHint::Slice(_)
					| TypeRefTypeHint::LenForSlice(_, _)
					| TypeRefTypeHint::StringAsBytes(_)
					| TypeRefTypeHint::StringAsPath(_)
					| TypeRefTypeHint::CharAsRustChar
					| TypeRefTypeHint::CharPtrSingleChar
					| TypeRefTypeHint::PrimitivePtrAsRaw
					| TypeRefTypeHint::AddArrayLength(_)
					| TypeRefTypeHint::BoxedAsRef(_, _, _)
					| TypeRefTypeHint::TraitClassConcrete
					| TypeRefTypeHint::ExplicitLifetime(_) => "1024",
				};
				format!("std::unique_ptr<char[]> {name}_out = std::make_unique<char[]>({len})")
			}
		}
	}

	fn cpp_arg_func_call(&self, name: &str) -> String {
		match self.str_type {
			StrType::StdString(_) | StrType::CvString(_) => {
				let ptr = if self.canonical.kind().as_pointer().is_some() {
					"&"
				} else {
					""
				};
				format!("{ptr}{name}_out")
			}
			StrType::CharPtr(_) => format!("{name}_out.get()"),
		}
	}

	fn cpp_arg_post_call(&self, name: &str) -> String {
		match self.str_type {
			StrType::StdString(StrEnc::Text) | StrType::CvString(StrEnc::Text) => {
				format!("*{name} = ocvrs_create_string({name}_out.c_str())")
			}
			StrType::CharPtr(StrEnc::Text) => {
				format!("*{name} = ocvrs_create_string({name}_out.get())")
			}
			StrType::StdString(StrEnc::Binary) => {
				format!("*{name} = ocvrs_create_byte_string({name}_out.data(), {name}_out.size())")
			}
			StrType::CvString(StrEnc::Binary) => {
				format!("*{name} = ocvrs_create_byte_string({name}_out.begin(), {name}_out.size())")
			}
			StrType::CharPtr(StrEnc::Binary) => {
				let TypeRefTypeHint::StringAsBytes(Some(len_arg_name)) = self.canonical.type_hint() else {
					panic!("Output argument of type `char*` with binary encoding must have `len` argument specified")
				};
				format!("*{name} = ocvrs_create_byte_string({name}_out.get(), {len_arg_name})")
			}
			StrType::StdString(StrEnc::OsStr) | StrType::CvString(StrEnc::OsStr) | StrType::CharPtr(_) => {
				panic!("Output string argument with OsStr encoding is not supported")
			}
		}
	}
}
