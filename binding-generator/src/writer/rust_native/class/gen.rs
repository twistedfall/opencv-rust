use std::borrow::Cow;
use std::collections::HashMap;
use std::iter;

use once_cell::sync::Lazy;

use super::rust_generate_debug_fields;
use crate::debug::NameDebug;
use crate::func::FuncDesc;
use crate::name_pool::NamePool;
use crate::type_ref::{ExternDir, FishStyle, TypeRef};
use crate::writer::rust_native::class::ClassExt;
use crate::writer::rust_native::element::{RustElement, RustNativeGeneratedElement};
use crate::writer::rust_native::func::FuncExt;
use crate::writer::rust_native::type_ref::TypeRefExt;
use crate::{settings, Class, CompiledInterpolation, Constness, CppNameStyle, Element, Func, IteratorExt, NameStyle, StrExt};

// todo, allow extension of simple classes for e.g. Elliptic_KeyPoint
pub fn gen_simple_class(c: &Class, opencv_version: &str) -> String {
	static SIMPLE_TPL: Lazy<CompiledInterpolation> =
		Lazy::new(|| include_str!("../tpl/class/simple.tpl.rs").compile_interpolation());

	static SIMPLE_FIELD_TPL_SRC: &str = include_str!("../tpl/class/simple_field.tpl.rs");

	static SIMPLE_FIELD_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| SIMPLE_FIELD_TPL_SRC.compile_interpolation());

	let rust_local = c.rust_name(NameStyle::decl());

	let class_fields = c.fields(|_| true);
	let mut fields = String::with_capacity(class_fields.len() * (SIMPLE_FIELD_TPL_SRC.len() + 64));
	class_fields.into_iter().for_each(|f| {
		let type_ref = f.type_ref();
		let typ = type_ref.rust_name(NameStyle::ref_());
		let visibility = if f.is_public() {
			"pub "
		} else {
			""
		};
		SIMPLE_FIELD_TPL.interpolate_into(
			&mut fields,
			&HashMap::from([
				("doc_comment", Cow::Owned(f.rendered_doc_comment("///", opencv_version))),
				("visibility", visibility.into()),
				("name", f.rust_leafname(FishStyle::No)),
				("type", typ),
			]),
		)
	});

	let (const_methods, mut_methods) = all_methods_const_mut(c);

	let mut impls = String::with_capacity(512);
	impls.add_default_impl(c, &mut_methods, &rust_local);
	impls.add_explicit_clone(c, &rust_local);

	SIMPLE_TPL.interpolate(&HashMap::from([
		("doc_comment", c.rendered_doc_comment("///", opencv_version).as_str()),
		("debug", &c.get_debug()),
		("rust_local", &rust_local),
		("rust_full", &c.rust_name(NameStyle::ref_())),
		("fields", &fields),
		(
			"impl",
			&gen_impl(c, const_methods.iter().chain(mut_methods.iter()), &rust_local, opencv_version),
		),
		("impls", &impls),
	]))
}

pub fn gen_boxed_class(c: &Class, opencv_version: &str) -> String {
	static BOXED_TPL: Lazy<CompiledInterpolation> =
		Lazy::new(|| include_str!("../tpl/class/boxed.tpl.rs").compile_interpolation());

	static BASE_TPL_SRC: &str = include_str!("../tpl/class/base.tpl.rs");

	static BASE_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| BASE_TPL_SRC.compile_interpolation());

	let fields = c.fields(|f| f.exclude_kind().is_included());
	let (mut const_methods, mut mut_methods) = (
		c.field_methods(&fields, Some(Constness::Const))
			.filter(|f| f.exclude_kind().is_included())
			.collect::<Vec<_>>(),
		c.field_methods(&fields, Some(Constness::Mut))
			.filter(|f| f.exclude_kind().is_included())
			.collect::<Vec<_>>(),
	);
	let field_const_methods = const_methods.clone();
	let (more_const_methods, more_mut_methods) = all_methods_const_mut(c);
	const_methods.extend(more_const_methods);
	mut_methods.extend(more_mut_methods);

	let type_ref = c.type_ref();
	let bases = c.bases();

	let traits = gen_traits(c, &type_ref, &bases, &const_methods, &mut_methods, opencv_version);

	let rust_local = c.rust_name(NameStyle::decl());

	let all_bases = all_bases(bases.into_owned());

	let mut impls = String::with_capacity(1024);
	impls.add_default_impl(c, &mut_methods, &rust_local);
	impls.add_explicit_clone(c, &rust_local);
	impls.add_implicit_clone(c, &type_ref, &rust_local);
	if !settings::IMPLEMENTED_MANUAL_DEBUG.contains(c.cpp_name(CppNameStyle::Reference).as_ref()) {
		impls.add_debug(&all_bases, field_const_methods, &rust_local);
	}
	for b in &all_bases {
		impls.add_base_cast(c, b, &rust_local);
	}
	if c.is_polymorphic() {
		for d in descendants(c) {
			impls.add_descendant_cast(c, &d, &rust_local);
		}
	}

	let mut bases = String::with_capacity(all_bases.len() * (BASE_TPL_SRC.len() + 64));
	all_bases.iter().chain(iter::once(c)).for_each(|base| {
		let base_type_ref = base.type_ref();
		BASE_TPL.interpolate_into(
			&mut bases,
			&HashMap::from([
				(
					"base_rust_full_mut",
					base.rust_trait_name(NameStyle::ref_(), Constness::Mut).as_ref(),
				),
				(
					"base_rust_full_const",
					&base.rust_trait_name(NameStyle::ref_(), Constness::Const),
				),
				("rust_local", &type_ref.rust_name(NameStyle::decl())),
				("rust_as_raw_const", &base.rust_as_raw_name(Constness::Const)),
				("rust_as_raw_mut", &base.rust_as_raw_name(Constness::Mut)),
				(
					"base_rust_extern_const",
					&base_type_ref
						.clone()
						.with_inherent_constness(Constness::Const)
						.rust_extern(ExternDir::ToCpp),
				),
				(
					"base_rust_extern_mut",
					&base_type_ref
						.with_inherent_constness(Constness::Mut)
						.rust_extern(ExternDir::ToCpp),
				),
			]),
		)
	});

	let extern_delete = FuncDesc::method_delete(c.clone()).identifier();

	let extern_default_new = method::default_new(c.clone(), type_ref.clone());
	let methods = iter::once(&extern_default_new)
		.filter(|_| needs_default_ctor(c, true))
		.chain(&const_methods)
		.chain(&mut_methods)
		.filter(|m| {
			let kind = m.kind();
			kind.as_static_method().is_some() || kind.as_constructor().is_some()
		});

	BOXED_TPL.interpolate(&HashMap::from([
		("doc_comment", c.rendered_doc_comment("///", opencv_version).as_str()),
		("debug", &c.get_debug()),
		("rust_local", &rust_local),
		("rust_full", &c.rust_name(NameStyle::ref_())),
		(
			"rust_extern_const",
			&type_ref
				.clone()
				.with_inherent_constness(Constness::Const)
				.rust_extern(ExternDir::ToCpp),
		),
		(
			"rust_extern_mut",
			&type_ref.with_inherent_constness(Constness::Mut).rust_extern(ExternDir::ToCpp),
		),
		("traits", &traits),
		("extern_delete", &extern_delete),
		("bases", &bases),
		("impl", &gen_impl(c, methods, &rust_local, opencv_version)),
		("impls", &impls),
	]))
}

pub fn extern_functions<'tu, 'ge>(c: &Class<'tu, 'ge>) -> Vec<Func<'tu, 'ge>> {
	let mut out = c
		.methods(|m| m.exclude_kind().is_included())
		.into_iter()
		.flat_map(|m| m.with_companion_functions())
		.collect::<Vec<_>>();

	if c.has_implicit_clone() {
		out.push(method::implicit_clone(c.clone(), c.type_ref()));
	}
	if needs_default_ctor(c, c.kind().is_boxed()) {
		out.push(method::default_new(c.clone(), c.type_ref()));
	}
	if c.kind().is_boxed() {
		out.extend(
			c.field_methods(&c.fields(|f| f.exclude_kind().is_included()), None)
				.filter(|f| f.exclude_kind().is_included()),
		);

		if c.is_polymorphic() {
			for d in descendants(c) {
				out.push(method::cast_to_descendant(c.clone(), d));
			}
		}
		for b in all_bases(c.bases().into_owned()) {
			out.push(method::cast_to_base(c.clone(), b));
		}

		out.push(FuncDesc::method_delete(c.clone()));
	}
	out
}

fn needs_default_ctor(c: &Class, is_boxed: bool) -> bool {
	is_boxed && !c.is_abstract() && c.has_implicit_default_constructor()
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

fn all_bases<'tu, 'ge>(bases: Vec<Class<'tu, 'ge>>) -> Vec<Class<'tu, 'ge>> {
	#![allow(clippy::mutable_key_type)]
	let mut out = bases
		.into_iter()
		.flat_map(|b| {
			let mut out = b.all_bases();
			out.insert(b);
			out
		})
		.filter(|b| b.exclude_kind().is_included() && !b.kind().is_simple())
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

fn all_methods_const_mut<'tu, 'ge>(c: &Class<'tu, 'ge>) -> (Vec<Func<'tu, 'ge>>, Vec<Func<'tu, 'ge>>) {
	let methods = c.methods(|m| m.exclude_kind().is_included());
	let mut const_methods = Vec::with_capacity(methods.len());
	let mut mut_methods = Vec::with_capacity(methods.len());
	methods.into_iter().for_each(|m| {
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
	(const_methods, mut_methods)
}

fn gen_impl<'f>(c: &Class, methods: impl Iterator<Item = &'f Func<'f, 'f>>, rust_local: &str, opencv_version: &str) -> String {
	static IMPL_TPL: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("../tpl/class/impl.tpl.rs").compile_interpolation());

	let consts = c.consts().iter().map(|c| c.gen_rust(opencv_version)).join("");

	let size_hint = methods.size_hint();
	let mut inherent_methods_pool = NamePool::with_capacity(size_hint.1.unwrap_or(size_hint.0));
	let inherent_methods = rust_generate_funcs(methods, &mut inherent_methods_pool, opencv_version);

	if consts.is_empty() && inherent_methods.is_empty() {
		"".to_string()
	} else {
		IMPL_TPL.interpolate(&HashMap::from([
			("rust_local", rust_local),
			("consts", &consts),
			("inherent_methods", &inherent_methods),
		]))
	}
}

fn gen_traits(
	c: &Class,
	type_ref: &TypeRef,
	bases: &[Class],
	const_methods: &[Func],
	mut_methods: &[Func],
	opencv_version: &str,
) -> String {
	static TRAIT_TPL: Lazy<CompiledInterpolation> =
		Lazy::new(|| include_str!("../tpl/class/trait.tpl.rs").compile_interpolation());

	let mut bases_const = Vec::with_capacity(bases.len());
	let mut bases_mut = Vec::with_capacity(bases.len() + 1);
	bases_mut.push(c.rust_trait_name(NameStyle::ref_(), Constness::Const));
	for b in bases
		.iter()
		.filter(|b| b.exclude_kind().is_included() && !b.kind().is_simple())
	{
		bases_const.push(b.rust_trait_name(NameStyle::ref_(), Constness::Const));
		bases_mut.push(b.rust_trait_name(NameStyle::ref_(), Constness::Mut));
	}
	bases_const.sort_unstable();
	bases_mut.sort_unstable();

	let mut trait_bases_const = bases_const.join(" + ");
	if !trait_bases_const.is_empty() {
		trait_bases_const.insert_str(0, ": ");
	};

	// bases_mut always contains at least its const variant
	let trait_bases_mut = ": ".to_string() + &bases_mut.join(" + ");

	let mut trait_methods_pool = NamePool::with_capacity(const_methods.len() + mut_methods.len());
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

	TRAIT_TPL.interpolate(&HashMap::from([
		("debug", c.get_debug().as_str()),
		("rust_trait_local_mut", &c.rust_trait_name(NameStyle::decl(), Constness::Mut)),
		(
			"rust_trait_local_const",
			&c.rust_trait_name(NameStyle::decl(), Constness::Const),
		),
		("rust_as_raw_const", &c.rust_as_raw_name(Constness::Const)),
		("rust_as_raw_mut", &c.rust_as_raw_name(Constness::Mut)),
		("rust_name_ref", &c.rust_name(NameStyle::ref_())),
		(
			"rust_extern_const",
			&type_ref
				.clone()
				.with_inherent_constness(Constness::Const)
				.rust_extern(ExternDir::ToCpp),
		),
		(
			"rust_extern_mut",
			&type_ref
				.clone()
				.with_inherent_constness(Constness::Mut)
				.rust_extern(ExternDir::ToCpp),
		),
		("trait_bases_const", &trait_bases_const),
		("trait_bases_mut", &trait_bases_mut),
		("trait_const_methods", &trait_const_methods),
		("trait_mut_methods", &trait_mut_methods),
	]))
}

trait Impls {
	fn add_default_impl(&mut self, c: &Class, mut_methods: &[Func], rust_local: &str);
	fn add_explicit_clone(&mut self, c: &Class, rust_local: &str);
	fn add_implicit_clone(&mut self, c: &Class, type_ref: &TypeRef, rust_local: &str);
	fn add_descendant_cast(&mut self, c: &Class, descendant: &Class, rust_local: &str);
	fn add_base_cast(&mut self, c: &Class, base: &Class, rust_local: &str);
	fn add_debug<'tu, 'ge>(&mut self, bases: &[Class<'tu, 'ge>], field_const_methods: Vec<Func<'tu, 'ge>>, rust_local: &str);
}

impl Impls for String {
	fn add_default_impl(&mut self, c: &Class, mut_methods: &[Func], rust_local: &str) {
		static IMPL_DEFAULT_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("../tpl/class/impl_default.tpl.rs").compile_interpolation());

		let needs_default_impl = needs_default_ctor(c, c.kind().is_boxed())
			|| mut_methods
				.iter()
				.find(|m| m.is_default_constructor())
				.is_some_and(|def_cons| def_cons.return_kind().is_infallible());
		if needs_default_impl {
			IMPL_DEFAULT_TPL.interpolate_into(self, &HashMap::from([("rust_local", rust_local)]));
		}
	}

	fn add_explicit_clone(&mut self, c: &Class, rust_local: &str) {
		static IMPL_EXPLICIT_CLONE_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("../tpl/class/impl_explicit_clone.tpl.rs").compile_interpolation());

		if c.has_explicit_clone() {
			IMPL_EXPLICIT_CLONE_TPL.interpolate_into(self, &HashMap::from([("rust_local", rust_local)]));
		}
	}

	fn add_implicit_clone(&mut self, c: &Class, type_ref: &TypeRef, rust_local: &str) {
		static IMPL_IMPLICIT_CLONE_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("../tpl/class/impl_implicit_clone.tpl.rs").compile_interpolation());

		if c.has_implicit_clone() {
			let extern_implicit_clone = method::implicit_clone(c.clone(), type_ref.clone()).identifier();
			IMPL_IMPLICIT_CLONE_TPL.interpolate_into(
				self,
				&HashMap::from([
					("rust_local", rust_local),
					("rust_as_raw_const", &c.rust_as_raw_name(Constness::Const)),
					("extern_implicit_clone", &extern_implicit_clone),
				]),
			);
		}
	}

	fn add_descendant_cast(&mut self, c: &Class, descendant: &Class, rust_local: &str) {
		static DESCENDANT_CAST_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("../tpl/class/descendant_cast.tpl.rs").compile_interpolation());

		let desc_full = descendant.rust_name(NameStyle::ref_());
		let extern_cast_to_descendant = method::cast_to_descendant(c.clone(), descendant.clone()).identifier();
		DESCENDANT_CAST_TPL.interpolate_into(
			self,
			&HashMap::from([
				("rust_local", rust_local),
				("descendant_rust_full", &desc_full),
				("extern_cast_to_descendant", &extern_cast_to_descendant),
			]),
		);
	}

	fn add_base_cast(&mut self, c: &Class, base: &Class, rust_local: &str) {
		static BASE_CAST_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("../tpl/class/base_cast.tpl.rs").compile_interpolation());

		let base_full = base.rust_name(NameStyle::ref_());
		let extern_cast_to_base = method::cast_to_base(c.clone(), base.clone()).identifier();
		BASE_CAST_TPL.interpolate_into(
			self,
			&HashMap::from([
				("rust_local", rust_local),
				("base_rust_full_mut", &base_full),
				("extern_cast_to_base", &extern_cast_to_base),
			]),
		);
	}

	fn add_debug<'tu, 'ge>(&mut self, bases: &[Class<'tu, 'ge>], mut field_const_methods: Vec<Func<'tu, 'ge>>, rust_local: &str) {
		static IMPL_DEBUG_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("../tpl/class/impl_debug.rs").compile_interpolation());

		for b in bases {
			field_const_methods.extend(b.field_methods(&b.fields(|f| f.exclude_kind().is_included()), Some(Constness::Const)));
		}
		let debug_fields = rust_generate_debug_fields(field_const_methods);
		IMPL_DEBUG_TPL.interpolate_into(
			self,
			&HashMap::from([("rust_local", rust_local), ("debug_fields", &debug_fields)]),
		);
	}
}

mod method {
	use crate::func::{FuncCppBody, FuncDesc, FuncKind, ReturnKind};
	use crate::type_ref::TypeRef;
	use crate::writer::rust_native::element::RustElement;
	use crate::writer::rust_native::func::cpp_return_map;
	use crate::Constness::{Const, Mut};
	use crate::{Class, Func, NameStyle};

	pub fn default_new<'tu, 'ge>(class: Class<'tu, 'ge>, type_ref: TypeRef<'tu, 'ge>) -> Func<'tu, 'ge> {
		Func::new_desc(
			FuncDesc::new(
				FuncKind::Constructor(class),
				Const,
				ReturnKind::InfallibleNaked,
				"defaultNew",
				"<unused>",
				[],
				type_ref,
			)
			.doc_comment("Creates a default instance of the class by calling the default constructor"),
		)
	}

	pub fn implicit_clone<'tu, 'ge>(class: Class<'tu, 'ge>, type_ref: TypeRef<'tu, 'ge>) -> Func<'tu, 'ge> {
		let cpp_body = FuncCppBody::ManualCallReturn(format!("return {};", cpp_return_map(&type_ref, "*instance", false).0).into());
		Func::new_desc(
			FuncDesc::new(
				FuncKind::InstanceMethod(class),
				Const,
				ReturnKind::InfallibleNaked,
				"implicitClone",
				"<unused>",
				[],
				type_ref,
			)
			.cpp_body(cpp_body),
		)
	}

	pub fn cast_to_base<'tu, 'ge>(class: Class<'tu, 'ge>, base_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
		let base_rust_local = base_class.rust_name(NameStyle::decl());
		Func::new_desc(
			FuncDesc::new(
				FuncKind::InstanceMethod(class),
				Mut,
				ReturnKind::InfallibleNaked,
				format!("to_{base_rust_local}"),
				"<unused>",
				[],
				TypeRef::new_class(base_class),
			)
			.cpp_body(FuncCppBody::ManualCallReturn(
				"return dynamic_cast<{{ret_type}}*>(instance);".into(),
			)),
		)
	}

	pub fn cast_to_descendant<'tu, 'ge>(class: Class<'tu, 'ge>, descendant_class: Class<'tu, 'ge>) -> Func<'tu, 'ge> {
		let descendant_rust_local = descendant_class.rust_name(NameStyle::decl());
		Func::new_desc(
			FuncDesc::new(
				FuncKind::InstanceMethod(class),
				Mut,
				ReturnKind::InfallibleNaked,
				format!("to_{descendant_rust_local}"),
				"<unused>",
				[],
				TypeRef::new_class(descendant_class),
			)
			.cpp_body(FuncCppBody::ManualCallReturn(
				"return dynamic_cast<{{ret_type}}*>(instance);".into(),
			)),
		)
	}
}
