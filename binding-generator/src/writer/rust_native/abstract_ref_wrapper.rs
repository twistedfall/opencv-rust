use std::collections::HashMap;

use once_cell::sync::Lazy;

use super::type_ref::TypeRefExt;
use super::RustNativeGeneratedElement;
use crate::type_ref::Constness;
use crate::writer::rust_native::class::ClassExt;
use crate::writer::rust_native::element::RustElement;
use crate::{AbstractRefWrapper, CompiledInterpolation, NameStyle, StrExt};

impl RustNativeGeneratedElement for AbstractRefWrapper<'_, '_> {
	fn element_order(&self) -> u8 {
		10
	}

	fn element_safe_id(&self) -> String {
		let type_ref = self.type_ref();
		format!("{}-{}", type_ref.rust_module().opencv_name(), type_ref.rust_safe_id(true))
	}

	fn gen_rust(&self, _opencv_version: &str) -> String {
		static RUST: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/abstract_ref_wrapper/rust.tpl.rs").compile_interpolation());

		let type_ref = self.type_ref().source();
		let type_ref_kind = type_ref.kind();
		let cls = type_ref_kind.as_class().expect("Can only make an abstract ref to a class");
		RUST.interpolate(&HashMap::from([
			("rust_full", cls.rust_name(NameStyle::ref_())),
			("rust_trait_const", cls.rust_trait_name(NameStyle::ref_(), Constness::Const)),
			("rust_trait_mut", cls.rust_trait_name(NameStyle::ref_(), Constness::Mut)),
			("rust_as_raw_const", cls.rust_as_raw_name(Constness::Const).into()),
			("rust_as_raw_mut", cls.rust_as_raw_name(Constness::Mut).into()),
		]))
	}
}
