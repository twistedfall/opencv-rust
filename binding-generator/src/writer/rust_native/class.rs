use std::borrow::Cow;

use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::{
	Class,
	class::Kind,
	CompiledInterpolation,
	Constness,
	ConstnessOverride,
	Element,
	Func,
	get_debug,
	IteratorExt,
	NamePool,
	StrExt,
};

use super::RustNativeGeneratedElement;

fn gen_rust_class(c: &Class, opencv_version: &str) -> String {
	static BOXED_TPL: Lazy<CompiledInterpolation> = Lazy::new(
		|| include_str!("tpl/class/boxed.tpl.rs").compile_interpolation()
	);

	static IMPL_TPL: Lazy<CompiledInterpolation> = Lazy::new(
		|| include_str!("tpl/class/impl.tpl.rs").compile_interpolation()
	);

	static IMPL_CLONE_TPL: Lazy<CompiledInterpolation> = Lazy::new(
		|| include_str!("tpl/class/impl_clone.tpl.rs").compile_interpolation()
	);

	static SIMPLE_TPL: Lazy<CompiledInterpolation> = Lazy::new(
		|| include_str!("tpl/class/simple.tpl.rs").compile_interpolation()
	);

	static SIMPLE_FIELD_TPL: Lazy<CompiledInterpolation> = Lazy::new(
		|| include_str!("tpl/class/simple_field.tpl.rs").compile_interpolation()
	);

	static BASE_TPL: Lazy<CompiledInterpolation> = Lazy::new(
		|| include_str!("tpl/class/base.tpl.rs").compile_interpolation()
	);

	static SIMPLE_BASE_TPL: Lazy<CompiledInterpolation> = Lazy::new(
		|| include_str!("tpl/class/simple_base.tpl.rs").compile_interpolation()
	);

	static TRAIT_TPL: Lazy<CompiledInterpolation> = Lazy::new(
		|| include_str!("tpl/class/trait.tpl.rs").compile_interpolation()
	);

	static TRAIT_DYN_TPL: Lazy<CompiledInterpolation> = Lazy::new(
		|| include_str!("tpl/class/trait_dyn.tpl.rs").compile_interpolation()
	);

	let type_ref = c.type_ref();
	let is_trait = c.is_trait();
	let is_abstract = c.is_abstract();
	let is_simple = c.is_simple();

	let mut out = String::new();

	let consts = c.consts();
	let fields = c.fields();
	let mut methods = if is_simple {
		vec![]
	} else {
		c.field_methods(fields.iter())
	};
	methods.extend(c.methods());
	if is_trait {
		let mut bases = c.bases().into_iter()
			.filter(|b| !b.is_excluded() && !b.is_simple()) // todo, allow extension of simple classes for e.g. Elliptic_KeyPoint
			.map(|x| x.rust_trait_fullname().into_owned())
			.collect::<Vec<_>>();
		bases.sort_unstable();
		let mut trait_bases: String = bases.join(" + ");
		if !trait_bases.is_empty() {
			trait_bases.insert_str(0, ": ");
		};
		let trait_methods = rust_generate_funcs(
			methods.iter().filter(|m| m.as_instance_method().is_some()),
			opencv_version,
		);
		let dyn_impl = if is_abstract {
			let consts = consts.iter()
				.map(|c| c.gen_rust(opencv_version))
				.join("");

			let methods = rust_generate_funcs(
				methods.iter().filter(|m| m.as_static_method().is_some()),
				opencv_version,
			);
			if !methods.is_empty() || !consts.is_empty() {
				TRAIT_DYN_TPL.interpolate(&hashmap! {
					"rust_local" => c.rust_trait_localname(),
					"consts" => consts.into(),
					"methods" => methods.into(),
				})
			} else {
				String::new()
			}
		} else {
			String::new()
		};
		out = TRAIT_TPL.interpolate(&hashmap! {
			"doc_comment" => Cow::Owned(c.rendered_doc_comment(opencv_version)),
			"debug" => get_debug(c).into(),
			"rust_trait_local" => c.rust_trait_localname(),
			"rust_local" => type_ref.rust_local(),
			"rust_extern_const" => type_ref.rust_extern_with_const(ConstnessOverride::Yes(Constness::Const)),
			"rust_extern_mut" => type_ref.rust_extern_with_const(ConstnessOverride::Yes(Constness::Mut)),
			"bases" => trait_bases.into(),
			"trait_methods" => trait_methods.into(),
			"dyn_impl" => dyn_impl.into(),
		});
	}

	if !is_abstract {
		let mut bases = c.all_bases().into_iter()
			.filter(|b| !b.is_excluded() && !b.is_simple()) // todo, allow extension of simple classes for e.g. Elliptic_KeyPoint
			.collect::<Vec<_>>();
		if is_trait {
			bases.push(c.clone());
		}
		bases.sort_unstable_by(|a, b| a.cpp_localname().cmp(&b.cpp_localname()));
		let bases = bases.into_iter()
			.map(|base| {
				let base_type_ref = base.type_ref();
				let tpl = if is_simple {
					&SIMPLE_BASE_TPL
				} else {
					&BASE_TPL
				};
				tpl.interpolate(&hashmap! {
					"base_rust_full" => base.rust_trait_fullname(),
					"rust_local" => type_ref.rust_local(),
					"base_rust_local" => base_type_ref.rust_local(),
					"base_rust_extern_const" => base_type_ref.rust_extern_with_const(ConstnessOverride::Yes(Constness::Const)),
					"base_rust_extern_mut" => base_type_ref.rust_extern_with_const(ConstnessOverride::Yes(Constness::Mut)),
				})
			})
			.collect::<Vec<_>>();

		let fields = if is_simple {
			let mut out: Vec<_> = fields.iter()
				.map(|f| {
					let type_ref = f.type_ref();
					let mut typ = type_ref.rust_full();
					// hack for converting the references to array types in struct definitions
					if type_ref.as_fixed_array().is_some() {
						if let Some(new_typ) = typ.strip_prefix("&mut ") {
							typ = new_typ.to_string().into()
						}
					}
					SIMPLE_FIELD_TPL.interpolate(&hashmap! {
						"doc_comment" => Cow::Owned(f.rendered_doc_comment(opencv_version)),
						"visibility" => "pub ".into(),
						"name" => f.rust_leafname(),
						"type" => typ,
					})
				})
				.collect();
			if out.is_empty() {
				out.push(
					SIMPLE_FIELD_TPL.interpolate(&hashmap! {
						"doc_comment" => "",
						"visibility" => "",
						"name" => "__rust_private",
						"type" => "[u8; 0]",
					})
				)
			}
			out
		} else {
			vec![]
		};

		let mut inherent_methods = String::with_capacity(512 * methods.len());
		let is_trait = c.is_trait();
		inherent_methods.push_str(&if is_trait {
			rust_generate_funcs(
				methods.iter()
					.filter(|m| m.as_static_method().is_some() || m.as_constructor().is_some()),
				opencv_version,
			)
		} else {
			rust_generate_funcs(&methods, opencv_version)
		});

		let tpl = if is_simple {
			&SIMPLE_TPL
		} else {
			&BOXED_TPL
		};

		let consts = consts.iter()
			.map(|c| c.gen_rust(opencv_version))
			.join("");

		let rust_local = c.rust_localname();
		let impls = if methods.iter().any(|m| m.is_clone()) {
			IMPL_CLONE_TPL.interpolate(&hashmap! {
				"rust_local" => rust_local.clone(),
			})
		} else {
			"".to_string()
		};

		out += &tpl.interpolate(&hashmap! {
			"doc_comment" => Cow::Owned(c.rendered_doc_comment(opencv_version)),
			"debug" => get_debug(c).into(),
			"rust_local" => rust_local.clone(),
			"rust_full" => c.rust_fullname(),
			"rust_extern_const" => type_ref.rust_extern_with_const(ConstnessOverride::Yes(Constness::Const)),
			"rust_extern_mut" => type_ref.rust_extern_with_const(ConstnessOverride::Yes(Constness::Mut)),
			"fields" => fields.join("").into(),
			"bases" => bases.join("").into(),
			"impl" => IMPL_TPL.interpolate(&hashmap! {
				"rust_local" => rust_local,
				"consts" => consts.into(),
				"methods" => inherent_methods.into(),
			}).into(),
			"impls" => impls.into(),
		});
	}
	out
}

fn gen_rust_exports_boxed(c: &Class) -> String {
	let fields = c.fields();
	let mut out = String::with_capacity((fields.len() + 1) * 128);
	for func in c.field_methods(fields.iter().filter(|f| !f.is_excluded())) {
		if !func.is_excluded() {
			out += &func.gen_rust_exports();
		}
	}
	out
}

fn gen_cpp_boxed(c: &Class) -> String {
	static BOXED_CPP_TPL: Lazy<CompiledInterpolation> = Lazy::new(
		|| include_str!("tpl/class/boxed_dtor.tpl.cpp").compile_interpolation()
	);

	let fields = c.fields();
	let mut out = String::with_capacity(fields.len() * 512);
	for func in c.field_methods(fields.iter().filter(|f| !f.is_excluded())) {
		if !func.is_excluded() {
			out += &func.gen_cpp();
		}
	}

	if !c.is_abstract() {
		let type_ref = c.type_ref();
		out += &BOXED_CPP_TPL.interpolate(&hashmap! {
				"rust_local" => type_ref.rust_local(),
				"cpp_full" => type_ref.cpp_full(),
				"cpp_extern" => type_ref.cpp_extern(),
			})
	}
	out
}

fn rust_generate_funcs<'tu>(fns: impl IntoIterator<Item=&'tu Func<'tu>>, opencv_version: &str) -> String {
	let fns = fns.into_iter()
		.filter(|f| !f.is_excluded());
	rust_disambiguate_names(fns)
		.map(|(name, func)| {
			let mut func = func.clone();
			func.set_name_hint(Some(name.as_str()));
			func.gen_rust(opencv_version) // fixme
		})
		.join("")
}

fn rust_disambiguate_names<'tu>(fns: impl IntoIterator<Item=&'tu Func<'tu>>) -> impl Iterator<Item=(String, &'tu Func<'tu>)> {
	let args = fns.into_iter();
	NamePool::with_capacity(args.size_hint().1.unwrap_or_default())
		.into_disambiguator(args, |f| f.rust_leafname())
}

impl RustNativeGeneratedElement for Class<'_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module(), self.rust_localname())
	}

	fn gen_rust(&self, opencv_version: &str) -> String {
		match self.kind() {
			Kind::Simple => {
				gen_rust_class(self, opencv_version)
			}
			Kind::Boxed => {
				gen_rust_class(self, opencv_version)
			}
			Kind::System | Kind::Excluded => {
				"".to_string()
			}
		}
	}

	fn gen_rust_exports(&self) -> String {
		let out = match self.kind() {
			Kind::Simple => {
				"".to_string()
			}
			Kind::Boxed => {
				gen_rust_exports_boxed(self)
			}
			Kind::System | Kind::Excluded => {
				"".to_string()
			}
		};

		let mut methods = self.methods().into_iter()
			.filter(|m| !m.is_excluded())
			.map(|m| m.gen_rust_exports());

		out + &methods.join("")
	}

	fn gen_cpp(&self) -> String {
		let out = match self.kind() {
			Kind::Simple => {
				"".to_string()
			}
			Kind::Boxed => {
				gen_cpp_boxed(self)
			}
			Kind::System | Kind::Excluded => {
				"".to_string()
			}
		};

		let methods: Vec<_> = self.methods().into_iter()
			.filter(|m| !m.is_excluded())
			.map(|m| m.gen_cpp())
			.collect();

		out + &methods.join("")
	}
}
