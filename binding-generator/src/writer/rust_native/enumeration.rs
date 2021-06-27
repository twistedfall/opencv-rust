use std::collections::HashSet;

use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::{
	CompiledInterpolation,
	Element,
	Enum,
	get_debug,
	StrExt,
	type_ref::FishStyle,
};

use super::RustNativeGeneratedElement;

impl RustNativeGeneratedElement for Enum<'_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module(), self.rust_localname(FishStyle::No))
	}

	fn gen_rust(&self, opencv_version: &str) -> String {
		static ENUM_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/enum/enum.tpl.rs").compile_interpolation()
		);

		static CONST_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/enum/const.tpl.rs").compile_interpolation()
		);

		static CONST_IGNORED_TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("tpl/enum/const_ignored.tpl.rs").compile_interpolation()
		);

		let consts = self.consts();
		let mut generated_values = HashSet::with_capacity(consts.len());
		let consts = consts.into_iter()
			.map(|c| {
				let name = c.rust_leafname(FishStyle::No);
				let value = c.value().expect("Can't get value of enum variant").to_string();
				let is_ignored = generated_values.contains(&value);
				let tpl = if is_ignored {
					&CONST_IGNORED_TPL
				} else {
					&CONST_TPL
				};
				let doc_comment = if is_ignored {
					c.rendered_doc_comment_with_prefix("//", opencv_version)
				} else {
					c.rendered_doc_comment(opencv_version)
				};
				generated_values.insert(value.clone());
				tpl.interpolate(&hashmap! {
					"doc_comment" => doc_comment,
					"name" => name.into_owned(),
					"value" => value,
				})
			}).collect::<Vec<_>>();
		ENUM_TPL.interpolate(&hashmap! {
			"doc_comment" => self.rendered_doc_comment(opencv_version).into(),
			"debug" => get_debug(self).into(),
			"rust_local" => self.rust_localname(FishStyle::No),
			"rust_full" => self.rust_fullname(FishStyle::No),
			"consts" => consts.join("").into(),
		})
	}
}
