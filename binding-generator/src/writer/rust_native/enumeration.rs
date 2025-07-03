use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::LazyLock;

use super::element::{DefaultRustNativeElement, RustElement};
use super::RustNativeGeneratedElement;
use crate::constant::{ConstDesc, Value};
use crate::debug::NameDebug;
use crate::enumeration::EnumBitfield;
use crate::type_ref::{FishStyle, NameStyle};
use crate::writer::rust_native::constant::ValueExt;
use crate::{CompiledInterpolation, Const, EntityElement, Enum, StrExt, StringExt, SupportedModule};

impl RustElement for Enum<'_, '_> {
	fn rust_module(&self) -> SupportedModule {
		DefaultRustNativeElement::rust_module(self.entity())
	}

	fn rust_name(&self, style: NameStyle) -> Cow<'_, str> {
		DefaultRustNativeElement::rust_name(self, self.entity(), style).into()
	}
}

impl RustNativeGeneratedElement for Enum<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module().opencv_name(), self.rust_name(NameStyle::decl()))
	}

	fn gen_rust(&self, opencv_version: &str) -> String {
		static ENUM_TPL: LazyLock<CompiledInterpolation> =
			LazyLock::new(|| include_str!("tpl/enum/enum.tpl.rs").compile_interpolation());
		static ENUM_BITFIELD_TPL: LazyLock<CompiledInterpolation> =
			LazyLock::new(|| include_str!("tpl/enum/enum_bitfield.tpl.rs").compile_interpolation());

		static CONST_TPL: LazyLock<CompiledInterpolation> =
			LazyLock::new(|| include_str!("tpl/enum/const.tpl.rs").compile_interpolation());
		static CONST_BITFIELD_TPL: LazyLock<CompiledInterpolation> =
			LazyLock::new(|| include_str!("tpl/enum/const_bitfield.tpl.rs").compile_interpolation());

		static CONST_IGNORED_TPL_SRC: &str = include_str!("tpl/enum/const_ignored.tpl.rs");
		static CONST_IGNORED_TPL: LazyLock<CompiledInterpolation> = LazyLock::new(|| CONST_IGNORED_TPL_SRC.compile_interpolation());

		let mut consts = self.consts();

		let bitfield = self.bitfield();

		let (const_tpl, const_ignored_tpl) = if bitfield.is_bitfield() {
			// it's ok to have duplicates in bitfield
			(&CONST_BITFIELD_TPL, &CONST_BITFIELD_TPL)
		} else {
			(&CONST_TPL, &CONST_IGNORED_TPL)
		};

		let mut enum_consts = String::with_capacity(consts.len() * CONST_IGNORED_TPL_SRC.len());
		let mut consts_list = String::with_capacity(consts.len() * 20);

		let mut generated_values = HashMap::<String, Cow<str>>::with_capacity(consts.len());
		match bitfield {
			EnumBitfield::BitfieldWithoutZero => consts.insert(
				0,
				Const::new_desc(
					ConstDesc::new("NONE", self.rust_module(), Value::integer(0))
						.doc_comment("No flags are set, might not make sense for all enums"),
				),
			),
			EnumBitfield::NotBitfield | EnumBitfield::BitfieldWithZero => {}
		}
		for c in &consts {
			let name = c.rust_leafname(FishStyle::No);
			let value = c.value().expect("Can't get value of enum variant");
			let value = value.rust_render();
			let duplicate_name = generated_values.get(value.as_ref()).map(|s| s.as_ref());
			let enum_const_tpl = if duplicate_name.is_some() {
				const_ignored_tpl
			} else {
				consts_list.extend_sep(", ", name.as_ref());
				const_tpl
			};
			let comment_marker = if duplicate_name.is_some() && !bitfield.is_bitfield() {
				"//"
			} else {
				"///"
			};
			let doc_comment = c.rust_doc_comment(comment_marker, opencv_version);
			enum_const_tpl.interpolate_into(
				&mut enum_consts,
				&HashMap::from([
					("name", name.as_ref()),
					("value", value.as_ref()),
					("doc_comment", &doc_comment),
					("duplicate_name", duplicate_name.unwrap_or("")),
				]),
			);

			generated_values.insert(value.into_owned(), name);
		}

		let tpl = if bitfield.is_bitfield() {
			&ENUM_BITFIELD_TPL
		} else {
			&ENUM_TPL
		};
		let rust_decl = self.rust_name(NameStyle::decl());
		let rust_ref = &self.rust_name(NameStyle::ref_());
		tpl.interpolate(&HashMap::from([
			("rust_decl", rust_decl.as_ref()),
			("rust_ref", rust_ref.as_ref()),
			("doc_comment", &self.rust_doc_comment("///", opencv_version)),
			("debug", &self.get_debug()),
			("enum_consts", &enum_consts),
			("consts_list", &consts_list),
		]))
	}
}
