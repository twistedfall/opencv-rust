use std::borrow::Cow;

use crate::type_ref::{ConstnessOverride, Kind, TemplateArg, TypeRef, TypeRefRenderer};
use crate::{CppNameStyle, Element, StringExt};

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
			Kind::StdTuple(tuple) => format!(
				"{cnst}{typ}{name}",
				cnst = tuple.constness().cpp_qual(),
				typ = tuple.cpp_name(self.name_style),
				name = space_name,
			),
			Kind::Reference(inner) if !self.extern_types => {
				format!("{typ}&{name}", typ = inner.render(self.recurse()), name = space_const_name)
			}
			Kind::RValueReference(inner) if !self.extern_types => {
				format!("{typ}&&{name}", typ = inner.render(self.recurse()), name = space_const_name)
			}
			Kind::Pointer(inner) | Kind::Reference(inner) | Kind::RValueReference(inner) => {
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
				let mut out = cls.cpp_name(self.name_style).into_owned();
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
		} else if type_ref.is_extern_by_ptr() && !type_ref.as_abstract_class_ptr().is_some() {
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
