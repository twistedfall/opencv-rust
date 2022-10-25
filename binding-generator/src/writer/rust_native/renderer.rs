use std::borrow::Cow;
use std::fmt::Write;

use crate::type_ref::{
	ConstnessOverride, CppNameStyle, Dir, FishStyle, Kind, Lifetime, NameStyle, StrEnc, StrType, TemplateArg, TypeRef,
	TypeRefRenderer,
};
use crate::{settings, Element, StringExt};

use super::type_ref::TypeRefExt;

fn render_rust_tpl_decl<'a>(renderer: impl TypeRefRenderer<'a>, type_ref: &TypeRef, fish_style: FishStyle) -> String {
	let generic_types = type_ref.template_specialization_args();
	if !generic_types.is_empty() {
		let const_generics_implemented = type_ref.as_class().map_or(false, |cls| {
			settings::IMPLEMENTED_CONST_GENERICS.contains(cls.cpp_name(CppNameStyle::Reference).as_ref())
		});
		let mut constant_suffix = String::new();
		let generic_types = generic_types
			.iter()
			.filter_map(|t| match t {
				TemplateArg::Typename(type_ref) => Some(type_ref.render(renderer.recurse())),
				TemplateArg::Constant(literal) => {
					if let Some(cnst) = type_ref.gen_env().resolve_class_constant(literal).and_then(|c| c.value()) {
						if const_generics_implemented {
							return Some(cnst.to_string().into());
						}
						constant_suffix += &cnst.to_string();
					} else {
						if const_generics_implemented {
							return Some(literal.into());
						}
						constant_suffix += literal;
					}
					None
				}
				TemplateArg::Unknown => None,
			})
			.collect::<Vec<_>>();
		format!(
			"{cnst}{fish}<{typ}>",
			cnst = constant_suffix,
			fish = fish_style.rust_qual(),
			typ = generic_types.join(", ")
		)
	} else {
		"".to_string()
	}
}

fn render_cpp_tpl_decl<'a>(renderer: impl TypeRefRenderer<'a>, type_ref: &TypeRef) -> String {
	let generic_types = type_ref.template_specialization_args();
	if !generic_types.is_empty() {
		let generic_types = generic_types
			.into_iter()
			.filter_map(|t| match t {
				TemplateArg::Typename(type_ref) => Some(type_ref.render(renderer.recurse()).into_owned()),
				TemplateArg::Constant(literal) => {
					if let Some(cnst) = type_ref.gen_env().resolve_class_constant(&literal).and_then(|c| c.value()) {
						Some(cnst.to_string())
					} else {
						Some(literal)
					}
				}
				TemplateArg::Unknown => None,
			})
			.collect::<Vec<_>>();
		format!("<{}>", generic_types.join(", "))
	} else {
		"".to_string()
	}
}

#[derive(Debug)]
pub struct RustRenderer {
	pub name_style: NameStyle,
	pub lifetime: Lifetime,
	pub primitive_ref_as_ptr: bool,
}

impl RustRenderer {
	pub fn new(name_style: NameStyle, lifetime: Lifetime, primitive_ref_as_ptr: bool) -> Self {
		Self {
			name_style,
			lifetime,
			primitive_ref_as_ptr,
		}
	}

	fn wrap_nullable<'a>(&self, type_ref: &TypeRef, typ: Cow<'a, str>) -> Cow<'a, str> {
		if type_ref.is_nullable() {
			format!(
				"Option{fish}<{typ}>",
				fish = self.name_style.rust_turbo_fish_qual(),
				typ = typ
			)
			.into()
		} else {
			typ
		}
	}
}

impl TypeRefRenderer<'_> for RustRenderer {
	type Recursed = Self;

	fn render<'t>(self, type_ref: &'t TypeRef) -> Cow<'t, str> {
		if let Some(str_type) = type_ref.as_string() {
			#[allow(clippy::if_same_then_else)]
			return if matches!(
				str_type,
				Dir::In(StrType::StdString(StrEnc::Binary) | StrType::CvString(StrEnc::Binary))
					| Dir::Out(StrType::StdString(StrEnc::Binary) | StrType::CvString(StrEnc::Binary))
			) {
				if self.name_style.turbo_fish_style().is_turbo() {
					"Vec::<u8>"
				} else {
					"Vec<u8>"
				}
			} else if type_ref.constness().is_const() {
				"String" // todo implement receiving const str's
			} else {
				"String"
			}
			.into();
		}
		match type_ref.kind() {
			Kind::Primitive(rust, _) => rust.into(),
			Kind::Array(elem, size) => {
				let typ = type_ref.format_as_array(&elem.render(self.recurse()), size);
				self.wrap_nullable(type_ref, typ.into())
			}
			Kind::StdVector(vec) => vec.rust_name(self.name_style).into_owned().into(),
			Kind::Reference(inner) if (inner.as_simple_class().is_some() || inner.is_enum()) && inner.constness().is_const() => {
				// const references to simple classes are passed by value for performance
				// fixme: it kind of works now, but probably it's not the best idea
				//  because some functions can potentially save the pointer to the value, but it will be destroyed after function call
				inner.render(self.recurse()).into_owned().into()
			}
			Kind::Pointer(inner) if self.primitive_ref_as_ptr || inner.is_void() => {
				let typ = if inner.is_void() {
					"c_void".into()
				} else {
					inner.render(self.recurse())
				};
				format!("*{cnst}{typ}", cnst = type_ref.constness().rust_qual(true), typ = typ).into()
			}
			Kind::Pointer(inner) | Kind::Reference(inner) => {
				let typ = format!(
					"&{lt: <}{cnst}{typ}",
					cnst = type_ref.constness().rust_qual(false),
					lt = self.lifetime,
					typ = inner.render(self.recurse())
				);
				self.wrap_nullable(type_ref, typ.into())
			}
			Kind::SmartPtr(ptr) => {
				let typ = ptr.rust_name(self.name_style).into_owned();
				self.wrap_nullable(type_ref, typ.into())
			}
			Kind::Class(cls) => {
				let fish_style = self.name_style.turbo_fish_style();
				format!(
					"{dyn}{name}{generic}",
					dyn=if self.name_style.is_reference() && cls.is_abstract() { "dyn " } else { "" },
					name=cls.rust_name(self.name_style),
					generic=render_rust_tpl_decl(self, type_ref, fish_style),
				)
				.into()
			}
			Kind::Enum(enm) => enm.rust_name(self.name_style).into_owned().into(),
			Kind::Typedef(decl) => {
				let mut out: String = decl.rust_name(self.name_style).into_owned();
				let lifetime_count = decl.underlying_type_ref().rust_lifetime_count();
				if lifetime_count >= 1 {
					if lifetime_count >= 2 {
						unimplemented!("Support for lifetime count >= 2 is not implemented yet");
					}
					if self.lifetime.is_explicit() {
						out.write_fmt(format_args!("<{}>", self.lifetime)).expect("Impossible");
					}
				}
				out.into()
			}
			Kind::Generic(name) => name.into(),
			Kind::Function(func) => func.rust_name(self.name_style).into_owned().into(),
			Kind::Ignored => "<ignored>".into(),
		}
	}

	fn recurse(&self) -> Self {
		Self {
			name_style: self.name_style,
			lifetime: self.lifetime.next().expect("Too many lifetimes"),
			primitive_ref_as_ptr: self.primitive_ref_as_ptr,
		}
	}
}

#[derive(Debug)]
pub struct CppRenderer<'s> {
	pub name_style: CppNameStyle,
	pub name: &'s str,
	/// true for rendering in extern contexts, references are treated as pointers
	pub extern_types: bool,
	pub constness_override: ConstnessOverride,
}

impl<'s> CppRenderer<'s> {
	pub fn new(name_style: CppNameStyle, name: &'s str, extern_types: bool) -> Self {
		Self {
			name_style,
			name,
			extern_types,
			constness_override: ConstnessOverride::No,
		}
	}
}

impl<'a> TypeRefRenderer<'a> for CppRenderer<'_> {
	type Recursed = Self;

	fn render<'t>(self, type_ref: &'t TypeRef) -> Cow<'t, str> {
		let cnst = self.constness_override.with(type_ref.clang_constness()).cpp_qual();
		let (space_name, space_const_name) = if self.name.is_empty() {
			("".to_string(), "".to_string())
		} else {
			(format!(" {}", self.name), format!(" {}{}", cnst, self.name))
		};
		match type_ref.kind() {
			Kind::Primitive(_, cpp) => {
				format!("{cnst}{typ}{name}", cnst = cnst, typ = cpp, name = space_name)
			}
			Kind::Array(inner, size) => {
				if let Some(size) = size {
					if self.name.is_empty() {
						format!("{typ}**", typ = inner.render(self.recurse()))
					} else {
						format!(
							"{typ}(*{name})[{size}]",
							typ = inner.render(self.recurse()),
							name = self.name,
							size = size,
						)
					}
				} else {
					format!("{typ}*{name}", typ = inner.render(self.recurse()), name = space_name)
				}
			}
			Kind::StdVector(vec) => {
				format!(
					"{cnst}{vec_type}<{elem_type}>{name}",
					cnst = cnst,
					vec_type = vec.cpp_name(self.name_style),
					elem_type = vec.element_type().render(self.recurse()),
					name = space_name,
				)
			}
			Kind::Reference(inner) if !self.extern_types => {
				format!("{typ}&{name}", typ = inner.render(self.recurse()), name = space_const_name)
			}
			Kind::Pointer(inner) | Kind::Reference(inner) => {
				format!("{typ}*{name}", typ = inner.render(self.recurse()), name = space_const_name)
			}
			Kind::SmartPtr(ptr) => {
				format!(
					"{cnst}{ptr_type}<{inner_type}>{name}",
					cnst = cnst,
					ptr_type = ptr.cpp_name(self.name_style),
					inner_type = ptr.pointee().render(self.recurse()),
					name = space_name,
				)
			}
			Kind::Class(cls) => {
				let mut out: String = cls.cpp_name(self.name_style).into_owned();
				if !type_ref.is_std_string() {
					// fixme prevents emission of std::string<char>
					out += &render_cpp_tpl_decl(self, type_ref);
				}
				format!("{cnst}{typ}{name}", cnst = cnst, typ = out, name = space_name)
			}
			Kind::Enum(enm) => {
				format!(
					"{cnst}{typ}{name}",
					cnst = cnst,
					typ = enm.cpp_name(self.name_style),
					name = space_name,
				)
			}
			Kind::Typedef(tdef) => {
				let underlying_type = tdef.underlying_type_ref();
				let typ = if underlying_type.as_reference().is_some() {
					// references can't be used in lvalue position
					underlying_type.render(self.recurse())
				} else {
					tdef.cpp_name(self.name_style).into_owned().into()
				};
				format!("{cnst}{typ}{name}", cnst = cnst, typ = typ, name = space_name)
			}
			Kind::Generic(generic_name) => {
				format!("{cnst}{typ}{name}", cnst = cnst, typ = generic_name, name = space_name)
			}
			Kind::Function(func) => {
				let mut typ = func.cpp_name(self.name_style);
				if typ.contains("(*)") {
					if !self.name.is_empty() {
						typ.to_mut()
							.replacen_in_place("(*)", 1, &format!("(*{name})", name = self.name));
					}
					typ.into_owned()
				} else {
					format!("{typ}{name}", typ = typ, name = space_name)
				}
			}
			Kind::Ignored => {
				format!("<ignored>{name}", name = space_name)
			}
		}
		.into()
	}

	fn recurse(&self) -> Self {
		Self {
			name_style: self.name_style,
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

impl CppExternReturnRenderer {
	pub fn new(constness_override: ConstnessOverride) -> Self {
		Self { constness_override }
	}
}

impl<'a> TypeRefRenderer<'a> for CppExternReturnRenderer {
	type Recursed = CppRenderer<'a>;

	fn render<'t>(self, type_ref: &'t TypeRef) -> Cow<'t, str> {
		if type_ref.as_string().is_some() {
			"void*".into()
		} else if type_ref.is_by_ptr() && !type_ref.as_abstract_class_ptr().is_some() {
			format!("{typ}*", typ = self.recurse().render(type_ref)).into()
		} else {
			self.recurse().render(type_ref)
		}
	}

	fn recurse(&self) -> Self::Recursed {
		let mut out = CppRenderer::new(CppNameStyle::Reference, "", true);
		out.constness_override = self.constness_override;
		out
	}
}
