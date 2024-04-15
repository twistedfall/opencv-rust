use std::borrow::Cow;
use std::collections::HashMap;
use std::iter;

use once_cell::sync::Lazy;

use crate::class::ClassKind;
use crate::debug::NameDebug;
use crate::func::{FuncCppBody, FuncDesc, FuncKind, FuncRustBody, ReturnKind};
use crate::type_ref::{Constness, CppNameStyle, ExternDir, FishStyle, NameStyle, TypeRef};
use crate::writer::rust_native::func::{cpp_return_map, FuncExt};
use crate::{settings, Class, CompiledInterpolation, Element, Func, IteratorExt, NamePool, StrExt};

use super::element::{DefaultRustNativeElement, RustElement};
use super::type_ref::TypeRefExt;
use super::RustNativeGeneratedElement;

fn gen_rust_class(c: &Class, opencv_version: &str) -> String {
	static BOXED_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/class/boxed.tpl.rs").compile_interpolation());

	static IMPL_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/class/impl.tpl.rs").compile_interpolation());

	static IMPL_EXPLICIT_CLONE_TPL: Lazy<CompiledInterpolation> =
		Lazy::new(|| include_str!("tpl/class/impl_explicit_clone.tpl.rs").compile_interpolation());

	static IMPL_IMPLICIT_CLONE_TPL: Lazy<CompiledInterpolation> =
		Lazy::new(|| include_str!("tpl/class/impl_implicit_clone.tpl.rs").compile_interpolation());

	static IMPL_DEFAULT_TPL: Lazy<CompiledInterpolation> =
		Lazy::new(|| include_str!("tpl/class/impl_default.tpl.rs").compile_interpolation());

	static IMPL_DEBUG_TPL: Lazy<CompiledInterpolation> =
		Lazy::new(|| include_str!("tpl/class/impl_debug.rs").compile_interpolation());

	static DEFAULT_CTOR: Lazy<CompiledInterpolation> =
		Lazy::new(|| include_str!("tpl/class/default_ctor.tpl.rs").compile_interpolation());

	static SIMPLE_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/class/simple.tpl.rs").compile_interpolation());

	static SIMPLE_FIELD_TPL: Lazy<CompiledInterpolation> =
		Lazy::new(|| include_str!("tpl/class/simple_field.tpl.rs").compile_interpolation());

	static BASE_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/class/base.tpl.rs").compile_interpolation());

	static BASE_CAST_TPL: Lazy<CompiledInterpolation> =
		Lazy::new(|| include_str!("tpl/class/base_cast.tpl.rs").compile_interpolation());

	static DESCENDANT_CAST_TPL: Lazy<CompiledInterpolation> =
		Lazy::new(|| include_str!("tpl/class/descendant_cast.tpl.rs").compile_interpolation());

	static TRAIT_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/class/trait.tpl.rs").compile_interpolation());

	let type_ref = c.type_ref();
	let class_kind = c.kind();
	let is_trait = class_kind.is_trait();
	let doc_comment = c.rendered_doc_comment("///", opencv_version);

	let mut out = String::new();

	let consts = c.consts();
	let fields = c.fields();
	let (mut const_methods, mut mut_methods) = if class_kind.is_simple() {
		(vec![], vec![])
	} else {
		(
			c.field_methods(
				fields.iter().filter(|f| f.exclude_kind().is_included()),
				Some(Constness::Const),
			)
			.into_iter()
			.filter(|f| f.exclude_kind().is_included())
			.collect(),
			c.field_methods(fields.iter().filter(|f| f.exclude_kind().is_included()), Some(Constness::Mut))
				.into_iter()
				.filter(|f| f.exclude_kind().is_included())
				.collect(),
		)
	};
	let mut field_const_methods = const_methods.clone();
	let methods = c.methods();

	// make some more room for companion funcs
	const_methods.reserve(methods.len());
	mut_methods.reserve(methods.len());
	methods.into_iter().filter(|m| m.exclude_kind().is_included()).for_each(|m| {
		let companion_funcs = m.companion_functions().into_iter().filter(|f| f.exclude_kind().is_included());
		match m.constness() {
			Constness::Const => const_methods.push(m),
			Constness::Mut => mut_methods.push(m),
		};
		for f in companion_funcs {
			match f.constness() {
				Constness::Const => const_methods.push(f),
				Constness::Mut => mut_methods.push(f),
			};
		}
	});
	let method_count = const_methods.len() + mut_methods.len();
	if is_trait {
		let bases = c.bases();
		let mut bases_const = Vec::with_capacity(bases.len());
		let mut bases_mut = Vec::with_capacity(bases.len() + 1);
		bases_mut.push(c.rust_trait_name(NameStyle::ref_(), Constness::Const).into_owned());
		// todo, allow extension of simple classes for e.g. Elliptic_KeyPoint
		for b in bases
			.iter()
			.filter(|b| b.exclude_kind().is_included() && !b.kind().is_simple())
		{
			bases_const.push(b.rust_trait_name(NameStyle::ref_(), Constness::Const).into_owned());
			bases_mut.push(b.rust_trait_name(NameStyle::ref_(), Constness::Mut).into_owned());
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
			const_methods.iter().filter(|m| m.kind().as_instance_method().is_some()),
			&mut trait_methods_pool,
			opencv_version,
		);
		let trait_mut_methods = rust_generate_funcs(
			mut_methods.iter().filter(|m| m.kind().as_instance_method().is_some()),
			&mut trait_methods_pool,
			opencv_version,
		);

		out = TRAIT_TPL.interpolate(&HashMap::from([
			("debug", c.get_debug().into()),
			("rust_trait_local_mut", c.rust_trait_name(NameStyle::decl(), Constness::Mut)),
			(
				"rust_trait_local_const",
				c.rust_trait_name(NameStyle::decl(), Constness::Const),
			),
			("rust_as_raw_const", c.rust_as_raw_name(Constness::Const).into()),
			("rust_as_raw_mut", c.rust_as_raw_name(Constness::Mut).into()),
			("rust_name_ref", c.rust_name(NameStyle::ref_())),
			(
				"rust_extern_const",
				type_ref
					.with_inherent_constness(Constness::Const)
					.rust_extern(ExternDir::ToCpp),
			),
			(
				"rust_extern_mut",
				type_ref.with_inherent_constness(Constness::Mut).rust_extern(ExternDir::ToCpp),
			),
			("trait_bases_const", trait_bases_const.into()),
			("trait_bases_mut", trait_bases_mut.into()),
			("trait_const_methods", trait_const_methods.into()),
			("trait_mut_methods", trait_mut_methods.into()),
		]));
	}

	let rust_local = c.rust_name(NameStyle::decl());
	let mut impls = if c.has_explicit_clone() {
		IMPL_EXPLICIT_CLONE_TPL.interpolate(&HashMap::from([("rust_local", rust_local.as_ref())]))
	} else if c.has_implicit_clone() {
		let extern_implicit_clone = method_implicit_clone(c.clone(), type_ref.clone()).identifier();
		IMPL_IMPLICIT_CLONE_TPL.interpolate(&HashMap::from([
			("rust_local", rust_local.as_ref()),
			("rust_as_raw_const", &c.rust_as_raw_name(Constness::Const)),
			("extern_implicit_clone", &extern_implicit_clone),
		]))
	} else {
		"".to_string()
	};

	let mut bases = all_bases(c);
	if class_kind.is_boxed() {
		if c.is_polymorphic() {
			let descendants = descendants(c);
			for d in descendants {
				let desc_full = d.rust_name(NameStyle::ref_());
				let extern_cast_to_descendant = method_cast_to_descendant(c.clone(), d.clone()).identifier();
				impls += &DESCENDANT_CAST_TPL.interpolate(&HashMap::from([
					("rust_local", rust_local.as_ref()),
					("descendant_rust_full", desc_full.as_ref()),
					("extern_cast_to_descendant", &extern_cast_to_descendant),
				]));
			}
		}
		for b in &bases {
			let base_full = b.rust_name(NameStyle::ref_());
			let extern_cast_to_base = method_cast_to_base(c.clone(), b.clone()).identifier();
			impls += &BASE_CAST_TPL.interpolate(&HashMap::from([
				("rust_local", rust_local.as_ref()),
				("base_rust_full_mut", base_full.as_ref()),
				("extern_cast_to_base", &extern_cast_to_base),
			]));
		}
		if !settings::IMPLEMENTED_MANUAL_DEBUG.contains(c.cpp_name(CppNameStyle::Reference).as_ref()) {
			for b in &bases {
				let base_fields = b.fields();
				let base_field_const_methods = b.field_methods(
					base_fields.iter().filter(|f| f.exclude_kind().is_included()),
					Some(Constness::Const),
				);
				field_const_methods.extend(base_field_const_methods);
			}
			let debug_fields = rust_generate_debug_fields(field_const_methods);
			impls += &IMPL_DEBUG_TPL.interpolate(&HashMap::from([
				("rust_local", rust_local.as_ref()),
				("debug_fields", &debug_fields),
			]));
		}
	}

	if is_trait {
		bases.push(c.clone());
	}
	let bases = bases
		.into_iter()
		.map(|base| {
			let base_type_ref = base.type_ref();
			BASE_TPL.interpolate(&HashMap::from([
				("base_rust_full_mut", base.rust_trait_name(NameStyle::ref_(), Constness::Mut)),
				(
					"base_rust_full_const",
					base.rust_trait_name(NameStyle::ref_(), Constness::Const),
				),
				("rust_local", type_ref.rust_name(NameStyle::decl())),
				("rust_as_raw_const", base.rust_as_raw_name(Constness::Const).into()),
				("rust_as_raw_mut", base.rust_as_raw_name(Constness::Mut).into()),
				(
					"base_rust_extern_const",
					base_type_ref
						.with_inherent_constness(Constness::Const)
						.rust_extern(ExternDir::ToCpp),
				),
				(
					"base_rust_extern_mut",
					base_type_ref
						.with_inherent_constness(Constness::Mut)
						.rust_extern(ExternDir::ToCpp),
				),
			]))
		})
		.collect::<Vec<_>>();

	let fields = if class_kind.is_simple() {
		fields
			.into_iter()
			.map(|f| {
				let type_ref = f.type_ref();
				let typ = type_ref.rust_name(NameStyle::ref_());
				SIMPLE_FIELD_TPL.interpolate(&HashMap::from([
					("doc_comment", Cow::Owned(f.rendered_doc_comment("///", opencv_version))),
					("visibility", "pub ".into()),
					("name", f.rust_leafname(FishStyle::No)),
					("type", typ),
				]))
			})
			.collect()
	} else {
		vec![]
	};

	let mut inherent_methods = String::with_capacity(512 * (method_count));
	let mut inherent_methods_pool = NamePool::with_capacity(method_count);

	let mut needs_default_impl = false;
	if let Some(def_cons) = mut_methods.iter().find(|m| m.is_default_constructor()) {
		if def_cons.return_kind().is_infallible() {
			needs_default_impl = true;
		}
	}
	if needs_default_ctor(c, class_kind) {
		let extern_default_new = method_default_new(c.clone(), type_ref.clone()).identifier();
		inherent_methods.push_str(&DEFAULT_CTOR.interpolate(&HashMap::from([("extern_default_new", extern_default_new)])));
		inherent_methods_pool.add_name("default");
		needs_default_impl = true;
	}

	if needs_default_impl {
		impls += &IMPL_DEFAULT_TPL.interpolate(&HashMap::from([("rust_local", rust_local.as_ref())]));
	}

	inherent_methods.push_str(&if is_trait {
		rust_generate_funcs(
			const_methods.iter().chain(mut_methods.iter()).filter(|m| {
				let kind = m.kind();
				kind.as_static_method().is_some() || kind.as_constructor().is_some()
			}),
			&mut inherent_methods_pool,
			opencv_version,
		)
	} else {
		rust_generate_funcs(
			const_methods.iter().chain(mut_methods.iter()),
			&mut inherent_methods_pool,
			opencv_version,
		)
	});

	let tpl = if class_kind.is_simple() {
		&SIMPLE_TPL
	} else {
		&BOXED_TPL
	};

	let consts = consts.iter().map(|c| c.gen_rust(opencv_version)).join("");

	let extern_delete = FuncDesc::method_delete(c.clone()).identifier();

	out += &tpl.interpolate(&HashMap::from([
		("doc_comment", Cow::Owned(doc_comment)),
		("debug", c.get_debug().into()),
		("rust_local", rust_local.clone()),
		("rust_full", c.rust_name(NameStyle::ref_())),
		(
			"rust_extern_const",
			type_ref
				.with_inherent_constness(Constness::Const)
				.rust_extern(ExternDir::ToCpp),
		),
		(
			"rust_extern_mut",
			type_ref.with_inherent_constness(Constness::Mut).rust_extern(ExternDir::ToCpp),
		),
		("extern_delete", extern_delete.into()),
		("fields", fields.join("").into()),
		("bases", bases.join("").into()),
		(
			"impl",
			IMPL_TPL
				.interpolate(&HashMap::from([
					("rust_local", rust_local),
					("consts", consts.into()),
					("inherent_methods", inherent_methods.into()),
				]))
				.into(),
		),
		("impls", impls.into()),
	]));
	out
}

fn rust_generate_funcs<'f, 'tu, 'ge>(
	fns: impl Iterator<Item = &'f Func<'tu, 'ge>>,
	name_pool: &mut NamePool,
	opencv_version: &str,
) -> String
where
	'tu: 'ge,
	'ge: 'f,
{
	fns.map(move |func| {
		let mut func = Cow::Borrowed(func);
		let mut name = func.rust_leafname(FishStyle::No);
		if name_pool.make_unique_name(&mut name).is_changed() {
			let name = name.into();
			func.to_mut().set_rust_custom_leafname(Some(name));
		}
		func.gen_rust(opencv_version)
	})
	.join("")
}

pub fn rust_generate_debug_fields(field_const_methods: Vec<Func>) -> String {
	field_const_methods
		.into_iter()
		.filter(|f| f.exclude_kind().is_included() && f.return_type_ref().kind().is_debug())
		.filter_map(|f| {
			f.kind().as_field_accessor().map(|(cls, _)| {
				format!(
					"\n\t.field(\"{name}\", &{trait_name}::{name}(self))",
					trait_name = cls.rust_trait_name(NameStyle::ref_fish(), Constness::Const),
					name = f.rust_leafname(FishStyle::No)
				)
			})
		})
		.join("")
}

impl RustElement for Class<'_, '_> {
	fn rust_module(&self) -> Cow<str> {
		match self {
			&Self::Clang { entity, .. } => DefaultRustNativeElement::rust_module(entity),
			Self::Desc(desc) => desc.rust_module.as_ref().into(),
		}
	}

	fn rust_name(&self, style: NameStyle) -> Cow<str> {
		match self {
			&Self::Clang { entity, .. } => DefaultRustNativeElement::rust_name(self, entity, style).into(),
			Self::Desc(_) => match style {
				NameStyle::Declaration => self.rust_leafname(FishStyle::No),
				NameStyle::Reference(fish_style) => format!(
					"{}::{}",
					DefaultRustNativeElement::rust_module_reference(self),
					self.rust_leafname(fish_style)
				)
				.into(),
			},
		}
	}

	fn rust_leafname(&self, _fish_style: FishStyle) -> Cow<str> {
		if self.string_type().is_some() {
			"String".into()
		} else {
			let cpp_declname = self.cpp_name(CppNameStyle::Declaration);
			if cpp_declname == "Vec" {
				"VecN".into()
			} else {
				cpp_declname
			}
		}
	}

	fn rendered_doc_comment(&self, comment_marker: &str, opencv_version: &str) -> String {
		match self {
			&Self::Clang { entity, .. } => DefaultRustNativeElement::rendered_doc_comment(entity, comment_marker, opencv_version),
			Self::Desc(_) => "".to_string(),
		}
	}
}

impl RustNativeGeneratedElement for Class<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module(), self.rust_name(NameStyle::decl()))
	}

	fn gen_rust(&self, _opencv_version: &str) -> String {
		match self.kind() {
			ClassKind::Simple | ClassKind::Boxed | ClassKind::BoxedForced => gen_rust_class(self, _opencv_version),
			ClassKind::System | ClassKind::Other => "".to_string(),
		}
	}

	fn gen_rust_externs(&self) -> String {
		extern_functions(self).iter().map(Func::gen_rust_externs).join("")
	}

	fn gen_cpp(&self) -> String {
		extern_functions(self).iter().map(Func::gen_cpp).join("")
	}
}

fn extern_functions<'tu, 'ge>(c: &Class<'tu, 'ge>) -> Vec<Func<'tu, 'ge>> {
	let mut out = c
		.methods()
		.into_iter()
		.filter(|m| m.exclude_kind().is_included())
		.flat_map(|m| {
			let companion_func = m.companion_functions();
			iter::once(m).chain(companion_func)
		})
		.collect::<Vec<_>>();

	if c.has_implicit_clone() {
		out.push(method_implicit_clone(c.clone(), c.type_ref()));
	}
	if needs_default_ctor(c, c.kind()) {
		out.push(method_default_new(c.clone(), c.type_ref()));
	}
	if c.kind().is_boxed() {
		let fields = c.fields();
		out.extend(
			c.field_methods(fields.iter().filter(|f| f.exclude_kind().is_included()), None)
				.into_iter()
				.filter(|f| f.exclude_kind().is_included()),
		);

		if c.is_polymorphic() {
			for d in descendants(c) {
				out.push(method_cast_to_descendant(c.clone(), d));
			}
		}
		for b in all_bases(c) {
			out.push(method_cast_to_base(c.clone(), b));
		}

		out.push(FuncDesc::method_delete(c.clone()));
	}
	out
}

fn needs_default_ctor(c: &Class, kind: ClassKind) -> bool {
	kind.is_boxed() && !c.is_abstract() && c.has_implicit_default_constructor()
}

fn all_bases<'tu, 'ge>(cls: &Class<'tu, 'ge>) -> Vec<Class<'tu, 'ge>> {
	let mut out = cls.all_bases().into_iter()
		.filter(|b| b.exclude_kind().is_included() && !b.kind().is_simple()) // todo, allow extension of simple classes for e.g. Elliptic_KeyPoint
		.collect::<Vec<_>>();
	out.sort_unstable_by(|a, b| {
		a.cpp_name(CppNameStyle::Declaration)
			.cmp(&b.cpp_name(CppNameStyle::Declaration))
	});
	out
}

fn descendants<'tu, 'ge>(cls: &Class<'tu, 'ge>) -> Vec<Class<'tu, 'ge>> {
	let mut out = cls
		.all_descendants()
		.into_iter()
		.filter(|d| d.exclude_kind().is_included() && !d.kind().is_simple())
		.collect::<Vec<_>>();
	out.sort_unstable_by(|a, b| {
		a.cpp_name(CppNameStyle::Declaration)
			.cmp(&b.cpp_name(CppNameStyle::Declaration))
	});
	out
}

fn method_default_new<'tu, 'ge>(class: Class<'tu, 'ge>, type_ref: TypeRef<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::Constructor(class),
		Constness::Const,
		ReturnKind::InfallibleNaked,
		"defaultNew",
		"<unused>",
		vec![],
		FuncCppBody::Auto,
		FuncRustBody::Auto,
		type_ref,
	))
}

fn method_implicit_clone<'tu, 'ge>(class: Class<'tu, 'ge>, type_ref: TypeRef<'tu, 'ge>) -> Func<'tu, 'ge> {
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(class),
		Constness::Const,
		ReturnKind::InfallibleNaked,
		"implicitClone",
		"<unused>",
		vec![],
		FuncCppBody::ManualCallReturn(format!("return {};", cpp_return_map(&type_ref, "*instance", false).0).into()),
		FuncRustBody::Auto,
		type_ref,
	))
}

fn method_cast_to_base<'tu, 'ge>(class: Class<'tu, 'ge>, base_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	let base_rust_local = base_class.rust_name(NameStyle::decl());
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(class),
		Constness::Mut,
		ReturnKind::InfallibleNaked,
		format!("to_{base_rust_local}"),
		"<unused>",
		vec![],
		FuncCppBody::ManualCallReturn("return dynamic_cast<{{ret_type}}*>(instance);".into()),
		FuncRustBody::Auto,
		TypeRef::new_class(base_class),
	))
}

fn method_cast_to_descendant<'tu, 'ge>(class: Class<'tu, 'ge>, descendant_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
	let descendant_rust_local = descendant_class.rust_name(NameStyle::decl());
	Func::new_desc(FuncDesc::new(
		FuncKind::InstanceMethod(class),
		Constness::Mut,
		ReturnKind::InfallibleNaked,
		format!("to_{descendant_rust_local}"),
		"<unused>",
		vec![],
		FuncCppBody::ManualCallReturn("return dynamic_cast<{{ret_type}}*>(instance);".into()),
		FuncRustBody::Auto,
		TypeRef::new_class(descendant_class),
	))
}

pub trait ClassExt {
	fn rust_trait_name(&self, style: NameStyle, constness: Constness) -> Cow<str>;
	fn rust_as_raw_name(&self, constness: Constness) -> String;
}

impl ClassExt for Class<'_, '_> {
	fn rust_trait_name(&self, style: NameStyle, constness: Constness) -> Cow<str> {
		let mut out = self.rust_name(style);
		if self.kind().is_trait() {
			if constness.is_const() {
				out.to_mut().push_str("TraitConst");
			} else {
				out.to_mut().push_str("Trait");
			}
		}
		out
	}

	fn rust_as_raw_name(&self, constness: Constness) -> String {
		format!(
			"as_raw{const_qual}_{name}",
			const_qual = constness.rust_function_name_qual(),
			name = self.rust_name(NameStyle::Declaration)
		)
	}
}
