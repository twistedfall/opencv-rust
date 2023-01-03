use std::borrow::Cow;
use std::fmt::Write;

use crate::type_ref::{CppNameStyle, Dir, FishStyle, Kind, NameStyle, StrEnc, StrType, TemplateArg, TypeRef, TypeRefRenderer};
use crate::{settings, Element};

use super::element::RustElement;
use super::type_ref::{Lifetime, TypeRefExt};

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

#[derive(Debug)]
pub struct RustRenderer {
	pub name_style: NameStyle,
	pub lifetime: Lifetime,
	pub rust_by_ptr: bool,
}

impl RustRenderer {
	pub fn new(name_style: NameStyle, lifetime: Lifetime, rust_by_ptr: bool) -> Self {
		Self {
			name_style,
			lifetime,
			rust_by_ptr,
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
			Kind::StdTuple(tuple) => tuple.rust_name(self.name_style).into_owned().into(),
			Kind::Reference(inner) if (inner.as_simple_class().is_some() || inner.is_enum()) && inner.constness().is_const() => {
				// const references to simple classes are passed by value for performance
				// fixme: it kind of works now, but probably it's not the best idea
				//  because some functions can potentially save the pointer to the value, but it will be destroyed after function call
				inner.render(self.recurse()).into_owned().into()
			}
			Kind::Pointer(inner) if self.rust_by_ptr => {
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
			Kind::RValueReference(inner) => inner.render(self.recurse()).into_owned().into(),
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
						write!(out, "<{}>", self.lifetime).expect("Impossible");
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
			lifetime: self.lifetime.next().expect("Too many lifetimes"),
			..*self
		}
	}
}
