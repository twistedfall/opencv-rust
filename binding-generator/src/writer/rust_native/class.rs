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
	type_ref::{FishStyle, NameStyle},
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

	static IMPL_DEFAULT_TPL: Lazy<CompiledInterpolation> = Lazy::new(
		|| include_str!("tpl/class/impl_default.tpl.rs").compile_interpolation()
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

	static BASE_CAST_TPL: Lazy<CompiledInterpolation> = Lazy::new(
		|| include_str!("tpl/class/base_cast.tpl.rs").compile_interpolation()
	);

	static DESCENDANT_CAST_TPL: Lazy<CompiledInterpolation> = Lazy::new(
		|| include_str!("tpl/class/descendant_cast.tpl.rs").compile_interpolation()
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
	let (mut const_methods, mut mut_methods) = if is_simple {
		(vec![], vec![])
	} else {
		(
			c.field_methods(fields.iter(), Some(Constness::Const)),
			c.field_methods(fields.iter(), Some(Constness::Mut)),
		)
	};
	const_methods.extend(c.methods(Some(Constness::Const)));
	mut_methods.extend(c.methods(Some(Constness::Mut)));
	let method_count = const_methods.len() + mut_methods.len();
	if is_trait {
		let bases = c.bases();
		let mut bases_const = Vec::with_capacity(bases.len());
		let mut bases_mut = Vec::with_capacity(bases.len() + 1);
		bases_mut.push(c.rust_trait_name(NameStyle::Reference(FishStyle::Turbo), Constness::Const).into_owned());
		// todo, allow extension of simple classes for e.g. Elliptic_KeyPoint
		for b in bases.into_iter().filter(|b| !b.is_excluded() && !b.is_simple()) {
			bases_const.push(b.rust_trait_name(NameStyle::Reference(FishStyle::Turbo), Constness::Const).into_owned());
			bases_mut.push(b.rust_trait_name(NameStyle::Reference(FishStyle::Turbo), Constness::Mut).into_owned());
		}
		bases_const.sort_unstable();
		bases_mut.sort_unstable();
		let mut trait_bases_const: String = bases_const.join(" + ");
		if !trait_bases_const.is_empty() {
			trait_bases_const.insert_str(0, ": ");
		};
		let mut trait_bases_mut: String = bases_mut.join(" + ");
		if !trait_bases_mut.is_empty() {
			trait_bases_mut.insert_str(0, ": ");
		};
		let mut trait_methods_pool = NamePool::with_capacity(method_count);
		let trait_const_methods = rust_generate_funcs(
			const_methods.iter().filter(|m| m.as_instance_method().is_some()),
			&mut trait_methods_pool,
			opencv_version,
		);
		let trait_mut_methods = rust_generate_funcs(
			mut_methods.iter().filter(|m| m.as_instance_method().is_some()),
			&mut trait_methods_pool,
			opencv_version,
		);
		let dyn_impl = if is_abstract {
			let consts = consts.iter()
				.map(|c| c.gen_rust(opencv_version))
				.join("");

			let mut methods_pool = NamePool::with_capacity(method_count);
			let const_methods = rust_generate_funcs(
				const_methods.iter().filter(|m| m.as_static_method().is_some()),
				&mut methods_pool,
				opencv_version,
			);
			let mut_methods = rust_generate_funcs(
				mut_methods.iter().filter(|m| m.as_static_method().is_some()),
				&mut methods_pool,
				opencv_version,
			);
			if !const_methods.is_empty() || !mut_methods.is_empty() || !consts.is_empty() {
				TRAIT_DYN_TPL.interpolate(&hashmap! {
					"rust_local" => c.rust_trait_name(NameStyle::Declaration, Constness::Mut),
					"consts" => consts.into(),
					"const_methods" => const_methods.into(),
					"mut_methods" => mut_methods.into(),
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
			"rust_trait_local" => c.rust_trait_name(NameStyle::Declaration, Constness::Mut),
			"rust_trait_local_const" => c.rust_trait_name(NameStyle::Declaration, Constness::Const),
			"rust_local" => type_ref.rust_local(),
			"rust_extern_const" => type_ref.rust_extern(ConstnessOverride::Yes(Constness::Const)),
			"rust_extern_mut" => type_ref.rust_extern(ConstnessOverride::Yes(Constness::Mut)),
			"trait_bases_const" => trait_bases_const.into(),
			"trait_bases_mut" => trait_bases_mut.into(),
			"trait_const_methods" => trait_const_methods.into(),
			"trait_mut_methods" => trait_mut_methods.into(),
			"dyn_impl" => dyn_impl.into(),
		});
	}

	if !is_abstract {
		let rust_local = c.rust_localname(FishStyle::No);
		let mut impls = if const_methods.iter().any(|m| m.is_clone()) {
			IMPL_CLONE_TPL.interpolate(&hashmap! {
				"rust_local" => rust_local.as_ref(),
			})
		} else {
			"".to_string()
		};

		let mut bases = c.all_bases().into_iter()
			.filter(|b| !b.is_excluded() && !b.is_simple()) // todo, allow extension of simple classes for e.g. Elliptic_KeyPoint
			.collect::<Vec<_>>();
		bases.sort_unstable_by(|a, b| a.cpp_localname().cmp(&b.cpp_localname()));
		if !is_simple {
			if c.is_polymorphic() {
				let mut descendants = c.descendants()
					.filter(|d| !d.is_excluded() && !d.is_simple() && !d.is_abstract())
					.collect::<Vec<_>>();
				descendants.sort_unstable_by(|a, b| a.cpp_localname().cmp(&b.cpp_localname()));
				for d in descendants {
					let desc_local = d.rust_localname(FishStyle::No);
					let desc_full = d.rust_fullname(FishStyle::No);
					impls += &DESCENDANT_CAST_TPL.interpolate(&hashmap! {
						"rust_local" => rust_local.as_ref(),
						"descendant_rust_local" => desc_local.as_ref(),
						"descendant_rust_full" => desc_full.as_ref(),
					});
				}
			}
			for b in &bases {
				if !b.is_abstract() {
					let base_local = b.rust_localname(FishStyle::No);
					let base_full = b.rust_fullname(FishStyle::No);
					impls += &BASE_CAST_TPL.interpolate(&hashmap! {
						"rust_local" => rust_local.as_ref(),
						"base_rust_local" => base_local.as_ref(),
						"base_rust_full" => base_full.as_ref(),
					});
				}
			}
		}

		if is_trait {
			bases.push(c.clone());
		}
		let bases = bases.into_iter()
			.map(|base| {
				let base_type_ref = base.type_ref();
				let tpl = if is_simple {
					&SIMPLE_BASE_TPL
				} else {
					&BASE_TPL
				};
				tpl.interpolate(&hashmap! {
					"base_rust_full" => base.rust_trait_name(NameStyle::Reference(FishStyle::Turbo), Constness::Mut),
					"base_const_rust_full" => base.rust_trait_name(NameStyle::Reference(FishStyle::Turbo), Constness::Const),
					"rust_local" => type_ref.rust_local(),
					"base_rust_local" => base_type_ref.rust_local(),
					"base_rust_extern_const" => base_type_ref.rust_extern(ConstnessOverride::Yes(Constness::Const)),
					"base_rust_extern_mut" => base_type_ref.rust_extern(ConstnessOverride::Yes(Constness::Mut)),
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
						"name" => f.rust_leafname(FishStyle::No),
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

		let mut inherent_const_methods = String::with_capacity(512 * const_methods.len());
		let mut inherent_mut_methods = String::with_capacity(512 * mut_methods.len());
		let mut inherent_methods_pool = NamePool::with_capacity(method_count);
		let is_trait = c.is_trait();

		if let Some(def_cons) = mut_methods.iter().find(|m| m.is_default_constructor() && !m.is_excluded()) {
			if def_cons.is_infallible() {
				impls += &IMPL_DEFAULT_TPL.interpolate(&hashmap! {
					"rust_local" => rust_local.as_ref(),
				});
			}
		}

		inherent_const_methods.push_str(&if is_trait {
			rust_generate_funcs(
				const_methods.iter()
					.filter(|m| m.as_static_method().is_some() || m.as_constructor().is_some()),
				&mut inherent_methods_pool,
				opencv_version,
			)
		} else {
			rust_generate_funcs(const_methods.iter(), &mut inherent_methods_pool, opencv_version)
		});
		inherent_mut_methods.push_str(&if is_trait {
			rust_generate_funcs(
				mut_methods.iter()
					.filter(|m| m.as_static_method().is_some() || m.as_constructor().is_some()),
				&mut inherent_methods_pool,
				opencv_version,
			)
		} else {
			rust_generate_funcs(mut_methods.iter(), &mut inherent_methods_pool, opencv_version)
		});

		let tpl = if is_simple {
			&SIMPLE_TPL
		} else {
			&BOXED_TPL
		};

		let consts = consts.iter()
			.map(|c| c.gen_rust(opencv_version))
			.join("");

		out += &tpl.interpolate(&hashmap! {
			"doc_comment" => Cow::Owned(c.rendered_doc_comment(opencv_version)),
			"debug" => get_debug(c).into(),
			"rust_local" => rust_local.clone(),
			"rust_full" => c.rust_fullname(FishStyle::No),
			"rust_extern_const" => type_ref.rust_extern(ConstnessOverride::Yes(Constness::Const)),
			"rust_extern_mut" => type_ref.rust_extern(ConstnessOverride::Yes(Constness::Mut)),
			"fields" => fields.join("").into(),
			"bases" => bases.join("").into(),
			"impl" => IMPL_TPL.interpolate(&hashmap! {
				"rust_local" => rust_local,
				"consts" => consts.into(),
				"const_methods" => inherent_const_methods.into(),
				"mut_methods" => inherent_mut_methods.into(),
			}).into(),
			"impls" => impls.into(),
		});
	}
	out
}

fn gen_rust_exports_boxed(c: &Class) -> String {
	let fields = c.fields();
	let mut out = String::with_capacity((fields.len() + 1) * 128);
	for func in c.field_methods(fields.iter().filter(|f| !f.is_excluded()), None) {
		if !func.is_excluded() {
			out += &func.gen_rust_exports();
		}
	}
	out
}

fn gen_cpp_boxed(c: &Class) -> String {
	static BOXED_CPP_TPL: Lazy<CompiledInterpolation> = Lazy::new(
		|| include_str!("tpl/class/boxed.tpl.cpp").compile_interpolation()
	);

	static DESCENDANT_CAST_TPL: Lazy<CompiledInterpolation> = Lazy::new(
		|| include_str!("tpl/class/descendant_cast.tpl.cpp").compile_interpolation()
	);

	static BASE_CAST_TPL: Lazy<CompiledInterpolation> = Lazy::new(
		|| include_str!("tpl/class/base_cast.tpl.cpp").compile_interpolation()
	);

	let fields = c.fields();
	let mut out = String::with_capacity(fields.len() * 512);
	for func in c.field_methods(fields.iter().filter(|f| !f.is_excluded()), None) {
		if !func.is_excluded() {
			out += &func.gen_cpp();
		}
	}

	let mut casts = String::new();
	if !c.is_abstract() {
		let rust_local = c.rust_localname(FishStyle::No);
		let mut bases = c.all_bases().into_iter()
			.filter(|b| !b.is_excluded() && !b.is_simple() && !b.is_abstract())
			.collect::<Vec<_>>();
		bases.sort_unstable_by(|a, b| a.cpp_localname().cmp(&b.cpp_localname()));
		if !c.is_simple() {
			let cpp_decl = c.type_ref().cpp_arg_func_decl("instance");
			if c.is_polymorphic() {
				let mut descendants = c.descendants()
					.filter(|d| !d.is_excluded() && !d.is_simple() && !d.is_abstract())
					.collect::<Vec<_>>();
				descendants.sort_unstable_by(|a, b| a.cpp_localname().cmp(&b.cpp_localname()));
				for d in descendants {
					let desc_rust_local = d.rust_localname(FishStyle::No);
					let desc_cpp_full = d.cpp_fullname();
					casts += &DESCENDANT_CAST_TPL.interpolate(&hashmap! {
						"rust_local" => rust_local.as_ref(),
						"descendant_rust_local" => desc_rust_local.as_ref(),
						"descendant_cpp_full" => desc_cpp_full.as_ref(),
						"cpp_decl" => &cpp_decl,
					});
				}
			}
			for b in bases {
				let base_rust_local = b.rust_localname(FishStyle::No);
				let base_cpp_full = b.cpp_fullname();
				casts += &BASE_CAST_TPL.interpolate(&hashmap! {
					"rust_local" => rust_local.as_ref(),
					"base_rust_local" => base_rust_local.as_ref(),
					"base_cpp_full" => base_cpp_full.as_ref(),
					"cpp_decl" => &cpp_decl,
				});
			}
		}

		let type_ref = c.type_ref();
		out += &BOXED_CPP_TPL.interpolate(&hashmap! {
				"rust_local" => type_ref.rust_local(),
				"cpp_full" => type_ref.cpp_full(),
				"cpp_extern" => type_ref.cpp_extern(),
				"casts" => casts.into(),
			})
	}
	out
}

fn rust_generate_funcs<'f, 'tu, 'ge>(fns: impl Iterator<Item=&'f Func<'tu, 'ge>>, name_pool: &mut NamePool, opencv_version: &str) -> String where 'tu: 'ge, 'ge: 'f {
	let fns = fns.filter(|f| !f.is_excluded());
	fns.map(move |func| {
		let mut func = Cow::Borrowed(func);
		let mut name = func.rust_leafname(FishStyle::No);
		name_pool.make_unique_name(&mut name);
		if let Cow::Owned(name) = name {
			func.to_mut().set_name_hint(Some(name));
		}
		func.gen_rust(opencv_version) // fixme
	})
		.join("")
}

impl RustNativeGeneratedElement for Class<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module(), self.rust_localname(FishStyle::No))
	}

	fn gen_rust(&self, opencv_version: &str) -> String {
		match self.kind() {
			Kind::Simple | Kind::Boxed => {
				gen_rust_class(self, opencv_version)
			}
			Kind::System | Kind::Excluded => {
				"".to_string()
			}
		}
	}

	fn gen_rust_exports(&self) -> String {
		let out = match self.kind() {
			Kind::Boxed => {
				gen_rust_exports_boxed(self)
			}
			Kind::Simple | Kind::System | Kind::Excluded => {
				"".to_string()
			}
		};

		let mut methods = self.methods(None).into_iter()
			.filter(|m| !m.is_excluded())
			.map(|m| m.gen_rust_exports());

		out + &methods.join("")
	}

	fn gen_cpp(&self) -> String {
		let out = match self.kind() {
			Kind::Boxed => {
				gen_cpp_boxed(self)
			}
			Kind::Simple | Kind::System | Kind::Excluded => {
				"".to_string()
			}
		};

		let methods: Vec<_> = self.methods(None).into_iter()
			.filter(|m| !m.is_excluded())
			.map(|m| m.gen_cpp())
			.collect();

		out + &methods.join("")
	}
}
