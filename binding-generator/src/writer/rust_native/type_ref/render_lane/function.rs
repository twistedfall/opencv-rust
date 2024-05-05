use std::borrow::Cow;

use crate::field::Field;
use crate::function::Function;
use crate::type_ref::{Constness, ExternDir, FishStyle, TypeRef};
use crate::writer::rust_native::rust_disambiguate_names;
use crate::writer::rust_native::type_ref::{Lifetime, TypeRefExt};
use crate::{CppNameStyle, IteratorExt, NameStyle};

use super::{rust_arg_func_decl, rust_self_func_decl, RenderLaneTrait};

pub struct FunctionRenderLane<'tu, 'ge> {
	pub non_canonical: TypeRef<'tu, 'ge>,
	pub func: Function<'tu, 'ge>,
}

impl<'tu, 'ge> FunctionRenderLane<'tu, 'ge> {
	pub fn from_non_canonical_func(non_canonical: TypeRef<'tu, 'ge>, func: Function<'tu, 'ge>) -> Self {
		Self { non_canonical, func }
	}
}

impl RenderLaneTrait for FunctionRenderLane<'_, '_> {
	fn rust_self_func_decl(&self, lifetime: Lifetime) -> Cow<'static, str> {
		rust_self_func_decl(self.non_canonical.constness(), lifetime)
	}

	fn rust_arg_func_decl(&self, name: &str, lifetime: Lifetime) -> String {
		rust_arg_func_decl(
			name,
			Constness::Const,
			&self
				.non_canonical
				.rust_name_ext(NameStyle::Reference(FishStyle::No), lifetime),
		)
	}

	fn rust_arg_pre_call(&self, name: &str, _is_function_infallible: bool) -> String {
		let args = rust_disambiguate_names(self.func.arguments()).collect::<Vec<_>>();
		if let Some((userdata_name, _)) = args.iter().find(|(_, arg)| arg.is_user_data()) {
			let tramp_args = args
				.iter()
				.map(|(name, arg)| {
					(
						name == userdata_name,
						arg.type_ref().render_lane().to_dyn().rust_extern_arg_func_decl(name),
					)
				})
				.collect::<Vec<_>>();
			let fw_args = tramp_args
				.iter()
				.filter(|(is_user_data, _)| !is_user_data)
				.map(|(_, decl)| decl)
				.join(", ");
			let ret = self.func.return_type();
			format!(
				"callback_arg!({name}_trampoline({tramp_args}) -> {tramp_ret} => {userdata_name} in callbacks => {name}({fw_args}) -> {fw_ret})",
				tramp_args = tramp_args.iter().map(|(_, decl)| decl).join(", "),
				tramp_ret = ret.rust_extern(ExternDir::FromCpp),
				fw_ret = ret.rust_extern(ExternDir::FromCpp),
			)
		} else {
			"".to_string()
		}
	}

	fn rust_arg_func_call(&self, name: &str) -> String {
		if self.func.arguments().iter().any(Field::is_user_data) {
			format!("{name}_trampoline")
		} else {
			name.to_string()
		}
	}

	fn rust_extern_arg_func_decl(&self, name: &str) -> String {
		// make sure to render canonical for externs because the representation is different from typedef
		rust_arg_func_decl(
			name,
			Constness::Const,
			&self.non_canonical.canonical().rust_extern(ExternDir::ToCpp),
		)
	}

	fn cpp_arg_func_decl(&self, name: &str) -> Cow<str> {
		self.non_canonical.cpp_name_ext(CppNameStyle::Reference, name, true)
	}

	fn cpp_arg_func_call(&self, name: &str) -> String {
		name.to_string()
	}
}
