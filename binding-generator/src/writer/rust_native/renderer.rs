use std::borrow::Cow;
use std::fmt::Write;

use crate::renderer::TypeRefRenderer;
use crate::type_ref::{CppNameStyle, FishStyle, NameStyle, TemplateArg, TypeRef, TypeRefKind};
use crate::writer::rust_native::element::RustElement;
use crate::writer::rust_native::type_ref::TypeRefExt;
use crate::{settings, Element};

use super::type_ref::Lifetime;

fn render_rust_tpl<'a>(renderer: impl TypeRefRenderer<'a>, type_ref: &TypeRef, fish_style: FishStyle) -> String {
	let generic_types = type_ref.template_specialization_args();
	if !generic_types.is_empty() {
		let const_generics_implemented = type_ref.as_class().map_or(false, |cls| {
			settings::IMPLEMENTED_CONST_GENERICS.contains(cls.cpp_name(CppNameStyle::Reference).as_ref())
		});
		let mut constant_suffix = String::new();
		let generic_types = generic_types
			.iter()
			.filter_map(|t| match t {
				TemplateArg::Typename(type_ref) => Some(renderer.recurse().render(type_ref)),
				TemplateArg::Constant(literal) => {
					if const_generics_implemented {
						Some(literal.into())
					} else {
						constant_suffix += literal;
						None
					}
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
			format!("Option{fish}<{typ}>", fish = self.name_style.rust_turbo_fish_qual()).into()
		} else {
			typ
		}
	}
}

impl TypeRefRenderer<'_> for RustRenderer {
	type Recursed = Self;

	fn render<'t>(self, type_ref: &'t TypeRef) -> Cow<'t, str> {
		if let Some(str_type) = type_ref.as_string() {
			return if str_type.inner().is_binary() {
				if self.name_style.turbo_fish_style().is_turbo() {
					"Vec::<u8>"
				} else {
					"Vec<u8>"
				}
			} else {
				"String"
			}
			.into();
		}
		match type_ref.kind().into_owned() {
			TypeRefKind::Primitive(rust, _) => rust.into(),
			TypeRefKind::Array(elem, size) => {
				let typ = type_ref.format_as_array(&self.recurse().render(&elem), size);
				self.wrap_nullable(type_ref, typ.into())
			}
			TypeRefKind::StdVector(vec) => vec.rust_name(self.name_style).into_owned().into(),
			TypeRefKind::StdTuple(tuple) => tuple.rust_name(self.name_style).into_owned().into(),
			TypeRefKind::Reference(inner)
				if (inner.as_simple_class().is_some() || inner.is_enum()) && inner.constness().is_const() =>
			{
				// const references to simple classes are passed by value for performance
				// fixme: it kind of works now, but probably it's not the best idea
				//  because some functions can potentially save the pointer to the value, but it will be destroyed after function call
				self.recurse().render(&inner).into_owned().into()
			}
			TypeRefKind::Pointer(inner) if self.rust_by_ptr => {
				let typ = if inner.is_void() {
					"c_void".into()
				} else {
					self.recurse().render(&inner)
				};
				format!("*{cnst}{typ}", cnst = type_ref.constness().rust_qual_ptr()).into()
			}
			TypeRefKind::Pointer(inner) | TypeRefKind::Reference(inner) => {
				let typ = format!(
					"&{lt: <}{cnst}{typ}",
					cnst = type_ref.constness().rust_qual(),
					lt = self.lifetime,
					typ = self.recurse().render(&inner)
				);
				self.wrap_nullable(type_ref, typ.into())
			}
			TypeRefKind::RValueReference(inner) => self.recurse().render(&inner).into_owned().into(),
			TypeRefKind::SmartPtr(ptr) => {
				let typ = ptr.rust_name(self.name_style).into_owned();
				self.wrap_nullable(type_ref, typ.into())
			}
			TypeRefKind::Class(cls) => {
				let fish_style = self.name_style.turbo_fish_style();
				format!(
					"{name}{generic}",
					name = cls.rust_name(self.name_style),
					generic = render_rust_tpl(self, type_ref, fish_style),
				)
				.into()
			}
			TypeRefKind::Enum(enm) => enm.rust_name(self.name_style).into_owned().into(),
			TypeRefKind::Typedef(decl) => {
				let mut out: String = decl.rust_name(self.name_style).into_owned();
				let lifetime_count = decl.underlying_type_ref().rust_lifetime_count();
				if lifetime_count >= 1 && self.lifetime.is_explicit() {
					write!(out, "<{}>", self.lifetime).expect("Impossible");
				}
				out.into()
			}
			TypeRefKind::Generic(name) => name.into(),
			TypeRefKind::Function(func) => func.rust_name(self.name_style).into_owned().into(),
			TypeRefKind::Ignored => "<ignored>".into(),
		}
	}

	fn recurse(&self) -> Self {
		Self {
			lifetime: self.lifetime.next().expect("Too many lifetimes"),
			..*self
		}
	}
}
