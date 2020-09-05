use std::{
	borrow::Cow,
	fmt::{self, Write},
};

use crate::{Element, StringExt};

use super::{Kind, TemplateArg, TypeRef};

pub trait TypeRefRenderer<'a> {
	type Recursed: TypeRefRenderer<'a> + Sized;

	fn render<'t>(self, type_ref: &'t TypeRef) -> Cow<'t, str>;
	fn recurse(&self) -> Self::Recursed;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Lifetime {
	number: u8
}

impl Default for Lifetime {
	fn default() -> Self {
		Self { number: 0 }
	}
}

impl Lifetime {
	pub fn next(self) -> Self {
		Self { number: self.number + 1 }
	}
}

impl fmt::Display for Lifetime {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let c = char::from(b'a' + self.number);
		write!(f, "'{}", c)
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Constness {
	Const,
	Mut,
}

impl Constness {
	pub fn is_const(self) -> bool {
		match self {
			Constness::Const => true,
			Constness::Mut => false,
		}
	}

	pub fn is_mut(self) -> bool {
		match self {
			Constness::Const => false,
			Constness::Mut => true,
		}
	}

	pub fn rust_qual(self, pointer: bool) -> &'static str {
		if self.is_const() {
			if pointer {
				"const "
			} else {
				""
			}
		} else {
			"mut "
		}
	}

	pub fn cpp_qual(self) -> &'static str {
		if self.is_const() {
			"const "
		} else {
			""
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConstnessOverride {
	No,
	Yes(Constness)
}

impl ConstnessOverride {
	pub fn with(self, constness: Constness) -> Constness {
		match self {
			ConstnessOverride::No => constness,
			ConstnessOverride::Yes(cnst) => cnst,
		}
	}
}

fn render_rust<'a, 't>(renderer: impl TypeRefRenderer<'a>, type_ref: &'t TypeRef, is_full: bool, lifetimes: Option<Lifetime>, use_turbo_fish: bool) -> Cow<'t, str> {
	if type_ref.as_string().is_some() {
		#[allow(clippy::if_same_then_else)]
		return if type_ref.constness().is_const() {
			"String" // todo implement receiving const str's
		} else {
			"String"
		}.into();
	}
	match type_ref.kind() {
		Kind::Primitive(rust, _) => {
			rust.into()
		}
		Kind::Array(elem, size) => {
			type_ref.format_as_array(&elem.render(renderer.recurse()), size).into()
		}
		Kind::StdVector(vec) => {
			vec.rust_name(is_full).into_owned().into()
		}
		Kind::Reference(inner) if (inner.as_simple_class().is_some() || inner.is_enum()) && inner.constness().is_const() => {
			// const references to simple classes are passed by value for performance
			// fixme: it kind of works now, but probably it's not the best idea
			//  because some functions can potentially save the pointer to the value, but it will be destroyed after function call
			inner.render(renderer.recurse()).into_owned().into()
		}
		Kind::Pointer(inner) | Kind::Reference(inner) => {
			if inner.is_void() {
				format!(
					"*{cnst}c_void",
					cnst = type_ref.constness().rust_qual(true),
				).into()
			} else {
				let lt = match lifetimes {
					Some(lt) => format!("{} ", lt),
					_ => "".to_string()
				};
				format!(
					"&{lt}{cnst}{typ}",
					cnst = type_ref.constness().rust_qual(false),
					lt = lt,
					typ = inner.render(renderer.recurse())
				).into()
			}
		}
		Kind::SmartPtr(ptr) => {
			ptr.rust_name(is_full).into_owned().into()
		}
		Kind::Class(cls) => {
			format!(
				"{dyn}{name}{generic}",
				dyn = if is_full && cls.is_abstract() { "dyn " } else { "" },
				name = cls.rust_name(is_full),
				generic = render_rust_tpl_decl(renderer, type_ref, use_turbo_fish),
			).into()
		}
		Kind::Enum(enm) => {
			enm.rust_name(is_full).into_owned().into()
		}
		Kind::Typedef(decl) => {
			let mut out: String = decl.rust_name(is_full).into_owned();
			let lifetime_count = decl.underlying_type_ref().rust_lifetime_count();
			if lifetime_count >= 1 {
				if lifetime_count >= 2 {
					unimplemented!("Support for lifetime count >= 2 is not implemented yet");
				}
				if let Some(lt) = lifetimes {
					out.write_fmt(format_args!("<{}>", lt)).expect("Impossible");
				}
			}
			out.into()
		}
		Kind::Generic(name) => {
			name.into()
		}
		Kind::Function(func) => {
			func.rust_name(is_full).into_owned().into()
		}
		Kind::Ignored => {
			"<ignored>".into()
		}
	}
}

fn render_rust_tpl_decl<'a>(renderer: impl TypeRefRenderer<'a>, type_ref: &TypeRef, use_turbo_fish: bool) -> String {
	let generic_types = type_ref.template_specialization_args();
	if !generic_types.is_empty() {
		let mut constant_suffix = String::new();
		let generic_types = generic_types.iter()
			.filter_map(|t| {
				match t {
					TemplateArg::Typename(type_ref) => {
						Some(type_ref.render(renderer.recurse()))
					}
					TemplateArg::Constant(literal) => {
						if let Some(cnst) = type_ref.gen_env.resolve_class_constant(literal).and_then(|c| c.value()) {
							constant_suffix += &cnst.to_string();
						} else {
							constant_suffix += literal;
						}
						None
					}
					TemplateArg::Unknown => {
						None
					}
				}
			})
			.collect::<Vec<_>>();
		let fish = if use_turbo_fish { "::" } else { "" };
		format!("{cnst}{fish}<{typ}>", cnst = constant_suffix, fish = fish, typ = generic_types.join(", "))
	} else {
		"".to_string()
	}
}

fn render_cpp<'a, 't>(renderer: impl TypeRefRenderer<'a>, type_ref: &'t TypeRef, is_full: bool, name: &str, extern_types: bool, constness: ConstnessOverride) -> Cow<'t, str> {
	let cnst = constness.with(type_ref.clang_constness()).cpp_qual();
	let (space_name, space_const_name) = if name.is_empty() {
		("".to_string(), "".to_string())
	} else {
		(format!(" {}", name), format!(" {}{}", cnst, name))
	};
	match type_ref.kind() {
		Kind::Primitive(_, cpp) => {
			format!(
				"{cnst}{typ}{name}",
				cnst=cnst,
				typ=cpp.to_string(),
				name=space_name,
			).into()
		}
		Kind::Array(inner, size) => {
			if let Some(size) = size {
				format!(
					"{typ}(*{name})[{size}]",
					typ=inner.render(renderer.recurse()),
					name=name,
					size=size,
				).into()
			} else {
				format!(
					"{typ}*{name}",
					typ=inner.render(renderer.recurse()),
					name=space_name,
				).into()
			}
		}
		Kind::StdVector(vec) => {
			format!(
				"{cnst}{vec_type}<{elem_type}>{name}",
				cnst=cnst,
				vec_type=vec.cpp_name(is_full),
				elem_type=vec.element_type().render(renderer.recurse()),
				name=space_name,
			).into()
		}
		Kind::Reference(inner) if !extern_types => {
			format!(
				"{typ}&{name}",
				typ=inner.render(renderer.recurse()),
				name=space_const_name,
			).into()
		}
		Kind::Pointer(inner) | Kind::Reference(inner) => {
			format!(
				"{typ}*{name}",
				typ=inner.render(renderer.recurse()),
				name=space_const_name,
			).into()
		}
		Kind::SmartPtr(ptr) => {
			format!(
				"{cnst}{ptr_type}<{inner_type}>{name}",
				cnst=cnst,
				ptr_type=ptr.cpp_name(is_full),
				inner_type=ptr.pointee().render(renderer.recurse()),
				name=space_name,
			).into()
		}
		Kind::Class(cls) => {
			let mut out: String = cls.cpp_name(is_full).into_owned();
			if !type_ref.is_std_string() { // fixme prevents emission of std::string<char>
				out += &render_cpp_tpl_decl(renderer, type_ref);
			}
			format!(
				"{cnst}{typ}{name}",
				cnst=cnst,
				typ=out,
				name=space_name,
			).into()
		}
		Kind::Enum(enm) => {
			format!(
				"{cnst}{typ}{name}",
				cnst=cnst,
				typ=enm.cpp_name(is_full),
				name=space_name,
			).into()
		}
		Kind::Typedef(tdef) => {
			let underlying_type = tdef.underlying_type_ref();
			let typ = if underlying_type.as_reference().is_some() { // references can't be used in lvalue position
				underlying_type.render(renderer.recurse())
			} else {
				tdef.cpp_name(is_full).into_owned().into()
			};
			format!(
				"{cnst}{typ}{name}",
				cnst=cnst,
				typ=typ,
				name=space_name,
			).into()
		}
		Kind::Generic(generic_name) => {
			format!(
				"{cnst}{typ}{name}",
				cnst=cnst,
				typ=generic_name,
				name=space_name,
			).into()
		}
		Kind::Function(func) => {
			let mut typ = func.cpp_name(is_full);
			if typ.contains("(*)") {
				typ.to_mut().replacen_in_place("(*)", 1, &format!("(*{name})", name = name));
				typ.into_owned().into()
			} else {
				format!("{typ}{name}", typ=typ, name=space_name).into()
			}
		}
		Kind::Ignored => {
			format!("<ignored>{name}", name=space_name).into()
		}
	}
}

fn render_cpp_tpl_decl<'a>(renderer: impl TypeRefRenderer<'a>, type_ref: &TypeRef) -> String {
	let generic_types = type_ref.template_specialization_args();
	if !generic_types.is_empty() {
		let generic_types = generic_types.into_iter()
			.filter_map(|t| {
				match t {
					TemplateArg::Typename(type_ref) => {
						Some(type_ref.render(renderer.recurse()).into_owned())
					}
					TemplateArg::Constant(literal) => {
						if let Some(cnst) = type_ref.gen_env.resolve_class_constant(&literal).and_then(|c| c.value()) {
							Some(cnst.to_string())
						} else {
							Some(literal)
						}
					}
					TemplateArg::Unknown => {
						None
					}
				}
			})
			.collect::<Vec<_>>();
		format!("<{}>", generic_types.join(", "))
	} else {
		"".to_string()
	}
}

#[derive(Clone, Debug)]
pub struct RustDeclarationRenderer {}

impl RustDeclarationRenderer {
	pub fn new() -> Self {
		Self {}
	}
}

impl<'a> TypeRefRenderer<'a> for RustDeclarationRenderer {
	type Recursed = Self;

	fn render<'t>(self, type_ref: &'t TypeRef) -> Cow<'t, str> {
		render_rust(self, type_ref, false, None, false)
	}

	fn recurse(&self) -> Self {
		self.clone()
	}
}

#[derive(Clone, Debug)]
pub struct RustReferenceRenderer {
	pub lifetimes: Option<Lifetime>,
	pub use_turbo_fish: bool,
}

impl RustReferenceRenderer {
	pub fn new(lifetimes: Option<Lifetime>, use_turbo_fish: bool) -> Self {
		Self { lifetimes, use_turbo_fish }
	}
}

impl<'a> TypeRefRenderer<'a> for RustReferenceRenderer {
	type Recursed = Self;

	fn render<'t>(self, type_ref: &'t TypeRef) -> Cow<'t, str> {
		let lifetimes = self.lifetimes;
		let use_turbo_fish = self.use_turbo_fish;
		render_rust(self, type_ref, true, lifetimes, use_turbo_fish)
	}

	fn recurse(&self) -> Self {
		Self {
			lifetimes: self.lifetimes.map(Lifetime::next),
			use_turbo_fish: self.use_turbo_fish,
		}
	}
}

#[derive(Clone, Debug)]
pub struct CppDeclarationRenderer {
	pub extern_types: bool,
}

impl CppDeclarationRenderer {
	pub fn new(extern_types: bool) -> Self {
		Self { extern_types }
	}
}

impl<'a> TypeRefRenderer<'a> for CppDeclarationRenderer {
	type Recursed = Self;

	fn render<'t>(self, type_ref: &'t TypeRef) -> Cow<'t, str> {
		let extern_types = self.extern_types;
		render_cpp(self, type_ref, false, "", extern_types, ConstnessOverride::No)
	}

	fn recurse(&self) -> Self {
		self.clone()
	}
}

#[derive(Clone, Debug)]
pub struct CppReferenceRenderer<'s> {
	pub name: &'s str,
	pub extern_types: bool,
	pub constness_override: ConstnessOverride,
}

impl<'s> CppReferenceRenderer<'s> {
	pub fn new(name: &'s str, extern_types: bool) -> Self {
		Self { name, extern_types, constness_override: ConstnessOverride::No }
	}
}

impl<'a> TypeRefRenderer<'a> for CppReferenceRenderer<'_> {
	type Recursed = Self;

	fn render<'t>(self, type_ref: &'t TypeRef) -> Cow<'t, str> {
		let name = self.name;
		let extern_types = self.extern_types;
		let constness = self.constness_override;
		render_cpp(self, type_ref, true, name, extern_types, constness)
	}

	fn recurse(&self) -> Self {
		Self {
			name: "",
			extern_types: self.extern_types,
			constness_override: ConstnessOverride::No,
		}
	}
}

#[derive(Clone, Debug)]
pub struct CppExternReturnRenderer {
	pub constness_override: ConstnessOverride,
}

impl<'s> CppExternReturnRenderer {
	pub fn new() -> Self {
		Self { constness_override: ConstnessOverride::No }
	}
}

impl<'a> TypeRefRenderer<'a> for CppExternReturnRenderer {
	type Recursed = CppReferenceRenderer<'a>;

	fn render<'t>(self, type_ref: &'t TypeRef) -> Cow<'t, str> {
		if type_ref.as_string().is_some() {
			"void*".into()
		} else if type_ref.is_by_ptr() && !type_ref.as_abstract_class_ptr().is_some() {
			format!("{typ}*", typ=self.recurse().render(type_ref)).into()
		} else {
			self.recurse().render(type_ref)
		}
	}

	fn recurse(&self) -> Self::Recursed {
		let mut out = CppReferenceRenderer::new("", true);
		out.constness_override = self.constness_override;
		out
	}
}
