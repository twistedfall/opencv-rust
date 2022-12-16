use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::type_ref::{Constness, NameStyle};
use crate::{AbstractRefWrapper, CompiledInterpolation, StrExt};

use super::class::ClassExt;
use super::type_ref::TypeRefExt;
use super::RustNativeGeneratedElement;

impl RustNativeGeneratedElement for AbstractRefWrapper<'_, '_> {
	fn element_order(&self) -> u8 {
		10
	}

	fn element_safe_id(&self) -> String {
		let type_ref = self.type_ref();
		format!("{}-{}", type_ref.rust_module(), type_ref.rust_safe_id(true))
	}

	fn gen_rust(&self, _opencv_version: &str) -> String {
		static RUST: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/abstract_ref_wrapper/rust.tpl.rs").compile_interpolation());

		let type_ref = self.type_ref().source();
		let cls = type_ref.as_class().expect("Can only make an abstract ref to a class");
		RUST.interpolate(&hashmap! {
			"rust_full" => cls.rust_trait_name(NameStyle::ref_(), Constness::Mut),
			"rust_const_full" => cls.rust_trait_name(NameStyle::ref_(), Constness::Const),
			"rust_local" => type_ref.rust_name(NameStyle::decl()),
		})
	}
}
