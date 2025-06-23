use std::borrow::Cow;

use super::element::RustElement;
use super::type_ref::{NullabilityExt, TypeRefExt};
use crate::function::Function;
use crate::type_ref::{ExternDir, Nullability};
use crate::{NameStyle, StringExt, SupportedModule};

impl RustElement for Function<'_, '_> {
	fn rust_module(&self) -> SupportedModule {
		SupportedModule::Core
	}

	fn rust_name(&self, style: NameStyle) -> Cow<str> {
		let ret = self.return_type();
		let args = self.arguments();
		if let Some(userdata_idx) = args.iter().position(|a| a.is_user_data()) {
			let mut args_str = String::with_capacity(args.len() * 16);
			for (idx, arg) in args.iter().enumerate() {
				if idx != userdata_idx {
					args_str.extend_sep(", ", arg.type_ref().rust_extern(ExternDir::Contained).as_ref());
				}
			}
			Nullability::Nullable.rust_wrap_nullable_decl(
				format!(
					"Box{fish}<dyn FnMut({args_str}) -> {ret} + Send + Sync + 'static>",
					fish = style.rust_turbo_fish_qual(),
					ret = ret.rust_extern(ExternDir::Contained),
				)
				.into(),
				style,
			)
		} else {
			self.rust_extern()
		}
	}

	fn rendered_doc_comment(&self, _comment_marker: &str, _opencv_version: &str) -> String {
		"".to_string()
	}
}

pub trait FunctionExt<'tu, 'ge> {
	fn rust_extern(&self) -> Cow<str>;
}

impl<'tu, 'ge> FunctionExt<'tu, 'ge> for Function<'tu, 'ge> {
	fn rust_extern(&self) -> Cow<str> {
		let args = self.arguments();
		let mut args_str = String::with_capacity(args.len() * 16);
		for arg in &args {
			args_str.extend_sep(", ", arg.type_ref().rust_extern(ExternDir::Contained).as_ref());
		}
		let ret = self.return_type();
		Nullability::Nullable.rust_wrap_nullable_decl(
			format!(
				r#"unsafe extern "C" fn({args_str}) -> {ret}"#,
				ret = ret.rust_extern(ExternDir::Contained)
			)
			.into(),
			NameStyle::decl(),
		)
	}
}
