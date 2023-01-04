use std::borrow::Cow;

use crate::field::Field;
use crate::function::Function;
use crate::type_ref::ExternDir;
use crate::{IteratorExt, NameStyle};

use super::element::RustElement;
use super::type_ref::TypeRefExt;

impl RustElement for Function<'_, '_> {
	fn rust_module(&self) -> Cow<str> {
		"<unset>".into()
	}

	fn rust_name(&self, style: NameStyle) -> Cow<str> {
		let ret = self.return_type();
		if self.has_userdata() {
			let args = self
				.rust_arguments()
				.into_iter()
				.map(|a| a.type_ref().rust_extern(ExternDir::Pure).into_owned())
				.join(", ");
			format!(
				"Option{fish}<Box{fish}<dyn FnMut({args}) -> {ret} + Send + Sync + 'static>>",
				fish = style.turbo_fish_style().rust_qual(),
				args = args,
				ret = ret.rust_extern(ExternDir::Pure),
			)
			.into()
		} else {
			self.rust_extern()
		}
	}

	fn rendered_doc_comment_with_prefix(&self, _prefix: &str, _opencv_version: &str) -> String {
		"".to_string()
	}
}

pub trait FunctionExt<'tu, 'ge> {
	fn rust_arguments(&self) -> Vec<Field<'tu, 'ge>>;
	fn rust_extern(&self) -> Cow<str>;
}

impl<'tu, 'ge> FunctionExt<'tu, 'ge> for Function<'tu, 'ge> {
	/// arguments without userdata
	fn rust_arguments(&self) -> Vec<Field<'tu, 'ge>> {
		self.arguments().into_iter().filter(|a| !a.is_user_data()).collect()
	}

	fn rust_extern(&self) -> Cow<str> {
		let args = self
			.arguments()
			.into_iter()
			.map(|a| a.type_ref().rust_extern(ExternDir::Pure).into_owned())
			.join(", ");
		let ret = self.return_type();
		format!(
			r#"Option<unsafe extern "C" fn({args}) -> {ret}>"#,
			args = args,
			ret = ret.rust_extern(ExternDir::Pure)
		)
		.into()
	}
}
