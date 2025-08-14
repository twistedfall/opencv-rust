use std::borrow::Cow;
use std::borrow::Cow::{Borrowed, Owned};

pub use kind::{BorrowKind, TypeRefKindExt};
pub use lifetime::{Lifetime, LifetimeIterator};
pub use nullability::NullabilityExt;

use super::element::RustElement;
use super::renderer::{RustExternRenderer, RustRenderer, RustReturnRenderer};
use super::smart_ptr::SmartPtrExt;
use super::tuple::TupleExt;
use super::vector::VectorExt;
use crate::renderer::TypeRefRenderer;
use crate::type_ref::{
	Constness, Dir, ExternDir, FishStyle, InputOutputArrayKind, NameStyle, TypeRef, TypeRefKind, TypeRefTypeHint,
};
use crate::writer::rust_native::class::ClassExt;
use crate::writer::rust_native::type_ref::render_lane::{
	ByMoveRenderLane, CppPassByVoidPtrRenderLane, EnumRenderLane, FixedArrayRenderLane, FunctionRenderLane, InStringRenderLane,
	IndirectRenderLane, Indirection, InputArrayRenderLane, InputOutputArrayRenderLane, OutStringRenderLane, OutputArrayRenderLane,
	PrimitiveRenderLane, RenderLane, SimpleClassRenderLane, TraitClassRenderLane, VariableArrayRenderLane, VoidSliceRenderLane,
};
use crate::{StringExt, SupportedModule};

mod kind;
mod lifetime;
mod nullability;
pub mod render_lane;

pub trait TypeRefExt {
	/// A high level category of the type that affects how it's passed from Rust to C++
	fn render_lane(&self) -> RenderLane<'_, '_>;

	fn rust_as_raw_name(&self, constness: Constness) -> String;
	fn rust_safe_id(&self, add_const: bool) -> Cow<'_, str>;
	fn rust_module(&self) -> SupportedModule;

	/// For when a type needs to be part of the user-visible Rust method name
	///
	/// Return a lightweight lowercase type representation, might not be precise. For example, it's used for operator bindings so
	/// that `operator &` on 2 `Mat`s translates into `and_mat_mat()`.
	fn rust_simple_name(&self) -> String;
	fn rust_name(&self, name_style: NameStyle) -> Cow<'_, str>;
	fn rust_name_ext(&self, name_style: NameStyle, lifetime: Lifetime) -> Cow<'_, str>;

	fn rust_extern(&self, dir: ExternDir) -> Cow<'_, str>;
	fn rust_return(&self, turbo_fish_style: FishStyle, lifetime: Lifetime) -> Cow<'_, str>;
	fn rust_extern_return_fallible(&self) -> Cow<'_, str>;
	fn rust_lifetime_count(&self) -> usize;
}

impl TypeRefExt for TypeRef<'_, '_> {
	fn render_lane(&self) -> RenderLane<'_, '_> {
		// todo, some of the `self.clone()` can be replaced with `canonical`, e.g. `FunctionRenderLane` and `TraitClassRenderLane`, the output will change, but will remain valid
		let canonical = self.canonical();
		let kind = canonical.kind();
		if let Some((dir, str_type)) = kind.as_string(self.type_hint()) {
			match dir {
				Dir::In => RenderLane::InString(InStringRenderLane::from_str_type_non_canonical(str_type, self.clone())),
				Dir::Out => RenderLane::OutString(OutStringRenderLane::from_str_type_canonical(str_type, canonical.into_owned())),
			}
		} else if let Some(input_output_array_kind) = kind.input_output_array_kind() {
			match input_output_array_kind {
				InputOutputArrayKind::Input => RenderLane::InputArray(InputArrayRenderLane::from_canonical(canonical.into_owned())),
				InputOutputArrayKind::Output => {
					RenderLane::OutputArray(OutputArrayRenderLane::from_canonical(canonical.into_owned()))
				}
				InputOutputArrayKind::InputOutput => {
					RenderLane::InputOutputArray(InputOutputArrayRenderLane::from_canonical(canonical.into_owned()))
				}
			}
		} else {
			match kind.into_owned() {
				TypeRefKind::Primitive(_, cpp) => {
					RenderLane::Primitive(PrimitiveRenderLane::from_cpp_non_canonical(cpp, self.clone()))
				}
				TypeRefKind::Function(f) => RenderLane::Function(FunctionRenderLane::from_non_canonical_func(self.clone(), f)),
				TypeRefKind::StdVector(_) | TypeRefKind::SmartPtr(_) | TypeRefKind::StdTuple(_) => RenderLane::CppPassByVoidPtr(
					CppPassByVoidPtrRenderLane::from_non_canonical_indirection(self.clone(), Indirection::None),
				),
				TypeRefKind::Array(elem, None) => {
					if matches!(self.type_hint(), TypeRefTypeHint::Slice) && elem.kind().is_void() {
						RenderLane::VoidSlice(VoidSliceRenderLane::from_canonical(canonical.into_owned()))
					} else {
						RenderLane::VariableArray(VariableArrayRenderLane::from_canonical_element(canonical.into_owned(), elem))
					}
				}
				TypeRefKind::Array(elem, Some(len)) => RenderLane::FixedArray(FixedArrayRenderLane::from_canonical_element_len(
					canonical.into_owned(),
					elem,
					len,
				)),
				TypeRefKind::RValueReference(inner) => {
					if inner.kind().extern_pass_kind().is_by_void_ptr() {
						RenderLane::CppPassByVoidPtr(CppPassByVoidPtrRenderLane::from_non_canonical_indirection(
							self.clone(),
							Indirection::None,
						))
					} else {
						RenderLane::ByMove(ByMoveRenderLane::from_non_canonical(self.clone()))
					}
				}
				kind => {
					let (indirection, tref_kind, tref) = match kind {
						TypeRefKind::Pointer(pointee) => {
							let pointee = pointee.with_type_hint(self.type_hint().clone());
							(Indirection::Pointer, pointee.kind().into_owned(), Owned(pointee))
						}
						TypeRefKind::Reference(pointee) => {
							let pointee = pointee.with_type_hint(self.type_hint().clone());
							(Indirection::Reference, pointee.kind().into_owned(), Owned(pointee))
						}
						kind => (Indirection::None, kind, Borrowed(self)),
					};
					match tref_kind.canonical().into_owned() {
						TypeRefKind::Class(cls) => {
							let cls_kind = cls.kind();
							if cls_kind.is_simple() {
								RenderLane::SimpleClass(SimpleClassRenderLane::from_non_canonical_indirection(
									tref.into_owned(),
									indirection,
								))
							} else if cls_kind.is_trait() {
								RenderLane::TraitClass(TraitClassRenderLane::from_non_canonical_class_indirection(
									self.clone(),
									cls,
									indirection,
								))
							} else {
								unreachable!("Any other kind of class shouldn't be generated")
							}
						}
						TypeRefKind::Enum(enm) => RenderLane::Enum(EnumRenderLane::from_non_canonical_enum_indirection(
							tref.into_owned(),
							enm,
							indirection,
						)),
						kind if kind.extern_pass_kind().is_by_void_ptr() => RenderLane::CppPassByVoidPtr(
							CppPassByVoidPtrRenderLane::from_non_canonical_indirection(self.clone(), indirection),
						),
						_ => RenderLane::Indirect(IndirectRenderLane::from_non_canonical_indirection(self.clone(), indirection)),
					}
				}
			}
		}
	}

	fn rust_as_raw_name(&self, constness: Constness) -> String {
		match self.kind().as_ref() {
			TypeRefKind::Class(cls) => cls.rust_as_raw_name(constness),
			TypeRefKind::Typedef(inner) => inner.underlying_type_ref().rust_as_raw_name(constness),
			_ => format!(
				"as_raw{const_qual}_{rust_safe_id}",
				const_qual = constness.rust_function_name_qual(),
				rust_safe_id = self.rust_safe_id(false)
			),
		}
	}

	fn rust_safe_id(&self, add_const: bool) -> Cow<'_, str> {
		let mut out = String::with_capacity(64);
		if add_const && self.inherent_constness().is_const() {
			out.push_str("const_");
		}
		let kind = self.kind();
		match kind.as_ref() {
			TypeRefKind::Array(inner, ..) => {
				out.push_str(&inner.rust_safe_id(add_const));
				out.push_str("_X");
			}
			TypeRefKind::StdVector(vec) => out.push_str(&vec.rust_localalias()),
			TypeRefKind::StdTuple(tuple) => out.push_str(&tuple.rust_localalias()),
			TypeRefKind::Pointer(inner) => {
				out.push_str(&inner.rust_safe_id(add_const));
				if !kind.extern_pass_kind().is_by_ptr() {
					out.push_str("_X");
				}
			}
			TypeRefKind::Reference(inner) | TypeRefKind::RValueReference(inner) => out.push_str(&inner.rust_safe_id(add_const)),
			TypeRefKind::SmartPtr(ptr) => out.push_str(&ptr.rust_localalias()),
			TypeRefKind::Class(cls) => out.push_str(&cls.rust_name(NameStyle::decl())),
			TypeRefKind::Primitive(..)
			| TypeRefKind::Enum(..)
			| TypeRefKind::Function(..)
			| TypeRefKind::Typedef(..)
			| TypeRefKind::Generic(..)
			| TypeRefKind::Ignored => out.push_str(&self.rust_name(NameStyle::decl())),
		}
		out.cleanup_name();
		out.into()
	}

	fn rust_module(&self) -> SupportedModule {
		match self.kind().as_ref() {
			TypeRefKind::Primitive(..) => SupportedModule::Core,
			TypeRefKind::StdVector(vec) => vec.rust_element_module(),
			TypeRefKind::StdTuple(tuple) => tuple.rust_element_module(),
			TypeRefKind::Array(inner, ..)
			| TypeRefKind::Pointer(inner)
			| TypeRefKind::Reference(inner)
			| TypeRefKind::RValueReference(inner) => inner.rust_module(),
			TypeRefKind::SmartPtr(ptr) => ptr.rust_module(),
			TypeRefKind::Class(cls) => cls.rust_module(),
			TypeRefKind::Enum(enm) => enm.rust_module(),
			TypeRefKind::Function(..) => {
				SupportedModule::Core // fixme
			}
			TypeRefKind::Typedef(tdef) => tdef.rust_module(),
			TypeRefKind::Generic(..) | TypeRefKind::Ignored => SupportedModule::Core,
		}
	}

	fn rust_simple_name(&self) -> String {
		let kind = self.kind();
		let maybe_ptr = kind.as_pointer_reference_move();
		let type_ref = maybe_ptr.as_ref().map_or(self, |cow| cow.as_ref());
		type_ref.rust_name(NameStyle::Declaration).to_lowercase()
	}

	fn rust_name(&self, name_style: NameStyle) -> Cow<'_, str> {
		let lt = self.type_hint().as_explicit_lifetime().unwrap_or(Lifetime::Elided);
		self.rust_name_ext(name_style, lt)
	}

	fn rust_name_ext(&self, name_style: NameStyle, lifetime: Lifetime) -> Cow<'_, str> {
		RustRenderer::new(name_style, lifetime).render(self)
	}

	fn rust_extern(&self, dir: ExternDir) -> Cow<'_, str> {
		RustExternRenderer::new(dir).render(self)
	}

	fn rust_return(&self, turbo_fish_style: FishStyle, lifetime: Lifetime) -> Cow<'_, str> {
		RustReturnRenderer::new(turbo_fish_style, lifetime).render(self)
	}

	fn rust_extern_return_fallible(&self) -> Cow<'_, str> {
		format!("Result<{ext}>", ext = self.rust_extern(ExternDir::FromCpp)).into()
	}

	fn rust_lifetime_count(&self) -> usize {
		let kind = self.kind();
		if kind.as_string(self.type_hint()).is_some() {
			0
		} else {
			match kind.as_ref() {
				kind @ TypeRefKind::Pointer(inner) if !kind.is_rust_by_ptr(self.type_hint()) => 1 + inner.rust_lifetime_count(),
				TypeRefKind::Reference(inner) => 1 + inner.rust_lifetime_count(),
				TypeRefKind::Typedef(tdef) => tdef.underlying_type_ref().rust_lifetime_count(),
				TypeRefKind::Class(cls) => cls.rust_lifetime().map_or(0, |_| 1),
				_ => 0,
			}
		}
	}
}
