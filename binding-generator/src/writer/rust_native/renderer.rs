use std::borrow::Cow;
use std::fmt::Write;

use crate::renderer::TypeRefRenderer;
use crate::type_ref::{
	Constness, CppNameStyle, Dir, ExternDir, FishStyle, NameStyle, StrType, TemplateArg, TypeRef, TypeRefDesc, TypeRefKind,
};
use crate::writer::rust_native::element::RustElement;
use crate::writer::rust_native::function::FunctionExt;
use crate::writer::rust_native::type_ref::{Lifetime, NullabilityExt, TypeRefExt};
use crate::{settings, CowMapBorrowedExt, Element};

fn render_rust_tpl<'a>(renderer: impl TypeRefRenderer<'a>, type_ref: &TypeRef, fish_style: FishStyle) -> String {
	let generic_types = type_ref.template_specialization_args();
	if !generic_types.is_empty() {
		let const_generics_implemented = type_ref.kind().as_class().map_or(false, |cls| {
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

pub struct RustRenderer {
	pub name_style: NameStyle,
	pub lifetime: Lifetime,
}

impl RustRenderer {
	pub fn new(name_style: NameStyle, lifetime: Lifetime) -> Self {
		Self { name_style, lifetime }
	}

	pub fn format_as_array(constness: Constness, elem_type: &str, size: Option<usize>) -> String {
		format!(
			"&{cnst}[{elem_type}{size}]",
			cnst = constness.rust_qual(),
			size = size.map_or_else(|| "".to_string(), |s| format!("; {s}"))
		)
	}
}

impl TypeRefRenderer<'_> for RustRenderer {
	type Recursed = Self;

	fn render<'t>(self, type_ref: &'t TypeRef) -> Cow<'t, str> {
		let kind = type_ref.kind();
		if let Some((_, str_type)) = kind.as_string(type_ref.type_hint()) {
			if str_type.is_binary() {
				if self.name_style.turbo_fish_style().is_turbo() {
					"Vec::<u8>"
				} else {
					"Vec<u8>"
				}
			} else {
				"String"
			}
			.into()
		} else {
			kind.map_borrowed(|kind| match kind {
				TypeRefKind::Primitive(rust, _) => (*rust).into(),
				TypeRefKind::Array(elem, size) => {
					let typ = RustRenderer::format_as_array(type_ref.constness(), &self.recurse().render(elem), *size);
					type_ref
						.type_hint()
						.nullability()
						.rust_wrap_nullable_decl(typ.into(), self.name_style)
				}
				TypeRefKind::StdVector(vec) => vec.rust_name(self.name_style),
				TypeRefKind::StdTuple(tuple) => tuple.rust_name(self.name_style),
				TypeRefKind::RValueReference(inner) => self.recurse().render(inner),
				kind @ TypeRefKind::Pointer(inner) if kind.is_rust_by_ptr(type_ref.type_hint()) => {
					let typ = if inner.kind().is_void() {
						"c_void".into()
					} else {
						self.recurse().render(inner)
					};
					format!("*{cnst}{typ}", cnst = type_ref.constness().rust_qual_ptr()).into()
				}
				TypeRefKind::Pointer(inner) | TypeRefKind::Reference(inner) => {
					let typ = format!(
						"&{lt: <}{cnst}{typ}",
						cnst = type_ref.constness().rust_qual(),
						lt = self.lifetime,
						typ = self.recurse().render(inner)
					);
					type_ref
						.type_hint()
						.nullability()
						.rust_wrap_nullable_decl(typ.into(), self.name_style)
				}
				TypeRefKind::SmartPtr(ptr) => {
					let typ = ptr.rust_name(self.name_style);
					type_ref
						.type_hint()
						.nullability()
						.rust_wrap_nullable_decl(typ, self.name_style)
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
				TypeRefKind::Enum(enm) => enm.rust_name(self.name_style),
				TypeRefKind::Typedef(decl) => {
					let mut out = decl.rust_name(self.name_style);
					let lifetime_count = decl.underlying_type_ref().rust_lifetime_count();
					if lifetime_count >= 1 && self.lifetime.is_explicit() {
						write!(out.to_mut(), "<{}>", self.lifetime).expect("Impossible");
					}
					out
				}
				TypeRefKind::Generic(name) => name.into(),
				TypeRefKind::Function(func) => func.rust_name(self.name_style),
				TypeRefKind::Ignored => "<ignored>".into(),
			})
		}
	}

	fn recurse(&self) -> Self::Recursed {
		Self {
			lifetime: Lifetime::Elided,
			..*self
		}
	}
}

pub struct RustExternRenderer {
	direction: ExternDir,
}

impl RustExternRenderer {
	pub fn new(direction: ExternDir) -> Self {
		Self { direction }
	}
}

impl TypeRefRenderer<'_> for RustExternRenderer {
	type Recursed = RustRenderer;

	fn render<'t>(self, type_ref: &'t TypeRef) -> Cow<'t, str> {
		let kind = type_ref.kind();
		if let Some((arg_dir, _)) = kind.as_string(type_ref.type_hint()) {
			match self.direction {
				ExternDir::ToCpp | ExternDir::Contained => match arg_dir {
					Dir::In => format!("*{cnst}c_char", cnst = Constness::Const.rust_qual_ptr()).into(),
					Dir::Out => "*mut *mut c_void".into(),
				},
				ExternDir::FromCpp => "*mut c_void".into(),
			}
		} else if kind.extern_pass_kind().is_by_void_ptr() || kind.is_void_slice(type_ref.type_hint()) {
			let void_ptr_constness = match self.direction {
				ExternDir::Contained | ExternDir::ToCpp => type_ref.constness(),
				ExternDir::FromCpp => Constness::Mut,
			};
			TypeRef::new_pointer(TypeRefDesc::void().with_inherent_constness(void_ptr_constness))
				.rust_extern(self.direction)
				.into_owned()
				.into()
		} else {
			kind.map_borrowed(|kind| {
				match kind {
					TypeRefKind::Pointer(inner) | TypeRefKind::Reference(inner) | TypeRefKind::RValueReference(inner) => {
						let typ = if inner.kind().is_void() {
							"c_void".into()
						} else {
							inner.rust_extern(ExternDir::Contained)
						};
						format!("*{cnst}{typ}", cnst = type_ref.constness().rust_qual_ptr()).into()
					}
					TypeRefKind::Array(elem, None) => {
						let typ = if matches!(elem.kind().as_string(elem.type_hint()), Some((Dir::Out, StrType::CharPtr(_)))) {
							// kind of special casing for cv_startLoop_int__X__int__charXX__int_charXX, without that
							// argv is treated as array of output arguments, and it doesn't seem to be meant this way
							format!("*{cnst}c_char", cnst = elem.inherent_constness().rust_qual_ptr()).into()
						} else {
							elem.rust_extern(ExternDir::Contained)
						};
						format!("*{cnst}{typ}", cnst = type_ref.constness().rust_qual_ptr()).into()
					}
					TypeRefKind::Array(elem, Some(len)) => format!(
						"*{cnst}[{typ}; {len}]",
						cnst = type_ref.constness().rust_qual_ptr(),
						typ = elem.rust_extern(ExternDir::Contained),
					)
					.into(),
					TypeRefKind::Function(func) => func.rust_extern(),
					TypeRefKind::Primitive(_, _)
					| TypeRefKind::StdVector(_)
					| TypeRefKind::StdTuple(_)
					| TypeRefKind::SmartPtr(_)
					| TypeRefKind::Enum(_)
					| TypeRefKind::Typedef(_)
					| TypeRefKind::Generic(_)
					| TypeRefKind::Class(_)
					| TypeRefKind::Ignored => type_ref.rust_name(NameStyle::ref_()),
				}
			})
		}
	}

	fn recurse(&self) -> Self::Recursed {
		RustRenderer::new(NameStyle::Reference(FishStyle::No), Lifetime::Elided)
	}
}

pub struct RustReturnRenderer {
	turbo_fish_style: FishStyle,
	lifetime: Lifetime,
}

impl RustReturnRenderer {
	pub fn new(turbo_fish_style: FishStyle, lifetime: Lifetime) -> Self {
		Self {
			turbo_fish_style,
			lifetime,
		}
	}
}

impl TypeRefRenderer<'_> for RustReturnRenderer {
	type Recursed = RustRenderer;

	fn render<'t>(self, type_ref: &'t TypeRef) -> Cow<'t, str> {
		let kind = type_ref.kind();
		if kind.as_abstract_class_ptr().is_some() {
			format!(
				"types::AbstractRef{mut_suf}{fish}<{lt:,<}{typ}>",
				mut_suf = type_ref.constness().rust_name_qual(),
				fish = self.turbo_fish_style.rust_qual(),
				lt = self.lifetime,
				typ = type_ref
					.source()
					.rust_name_ext(NameStyle::Reference(self.turbo_fish_style), self.lifetime),
			)
			.into()
		} else if type_ref.type_hint().as_boxed_as_ref().is_some() {
			format!(
				"BoxedRef{mut_suf}{fish}<{lt:,<}{name}>",
				mut_suf = type_ref.constness().rust_name_qual(),
				fish = self.turbo_fish_style.rust_qual(),
				lt = self.lifetime,
				name = type_ref.rust_name_ext(NameStyle::Reference(self.turbo_fish_style), self.lifetime)
			)
			.into()
		} else if kind.extern_pass_kind().is_by_void_ptr() {
			type_ref
				.source()
				.rust_name(NameStyle::Reference(self.turbo_fish_style))
				.into_owned()
				.into()
		} else {
			type_ref.rust_name(NameStyle::Reference(self.turbo_fish_style))
		}
	}

	fn recurse(&self) -> Self::Recursed {
		RustRenderer::new(NameStyle::Reference(self.turbo_fish_style), Lifetime::Elided)
	}
}
