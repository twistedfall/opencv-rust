use std::borrow::Cow;
use std::fmt;
use std::rc::Rc;
use Cow::{Borrowed, Owned};

use clang::{Entity, Type};

pub use desc::{ClangTypeExt, TypeRefDesc};
pub use kind::{InputOutputArrayKind, TypeRefKind};
pub use types::{
	dbg_clang_type, Constness, CppNameStyle, Dir, ExternDir, FishStyle, NameStyle, Nullability, StrEnc, StrType, TemplateArg,
	TypeRefTypeHint,
};

use crate::class::{ClassDesc, TemplateKind};
use crate::element::ExcludeKind;
use crate::renderer::{CppExternReturnRenderer, CppRenderer, TypeRefRenderer};
use crate::vector::VectorDesc;
use crate::writer::rust_native::type_ref::TypeRefExt;
use crate::{settings, AbstractRefWrapper, ClassKindOverride, ExportConfig};
use crate::{Class, Element, GeneratedType, GeneratorEnv, SmartPtr, Vector};

mod desc;
mod kind;
#[cfg(test)]
mod test;
mod types;

#[derive(Clone)]
pub enum TypeRef<'tu, 'ge> {
	Clang {
		type_ref: Type<'tu>,
		type_hint: TypeRefTypeHint,
		parent_entity: Option<Entity<'tu>>,
		gen_env: &'ge GeneratorEnv<'tu>,
	},
	Desc(Rc<TypeRefDesc<'tu, 'ge>>),
}

impl<'tu, 'ge> TypeRef<'tu, 'ge> {
	pub fn new(type_ref: Type<'tu>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self::new_ext(type_ref, TypeRefTypeHint::None, None, gen_env)
	}

	pub fn new_ext(
		type_ref: Type<'tu>,
		type_hint: TypeRefTypeHint,
		parent_entity: Option<Entity<'tu>>,
		gen_env: &'ge GeneratorEnv<'tu>,
	) -> Self {
		Self::Clang {
			type_ref,
			type_hint,
			parent_entity,
			gen_env,
		}
	}

	pub fn new_desc(desc: TypeRefDesc<'tu, 'ge>) -> Self {
		Self::Desc(Rc::new(desc))
	}

	pub fn new_pointer(inner: TypeRef<'tu, 'ge>) -> Self {
		Self::new_desc(TypeRefDesc::new(TypeRefKind::Pointer(inner), Constness::Mut))
	}

	pub fn new_reference(inner: TypeRef<'tu, 'ge>) -> Self {
		Self::new_desc(TypeRefDesc::new(TypeRefKind::Reference(inner), Constness::Mut))
	}

	pub fn new_rvalue_reference(inner: TypeRef<'tu, 'ge>) -> Self {
		Self::new_desc(TypeRefDesc::new(TypeRefKind::RValueReference(inner), Constness::Mut))
	}

	pub fn new_class(cls: Class<'tu, 'ge>) -> Self {
		Self::new_desc(TypeRefDesc::new(TypeRefKind::Class(cls), Constness::Mut))
	}

	pub fn new_array(inner: TypeRef<'tu, 'ge>, size: Option<usize>) -> Self {
		Self::new_desc(TypeRefDesc::new(TypeRefKind::Array(inner, size), Constness::Mut))
	}

	pub fn new_vector(vector: Vector<'tu, 'ge>) -> Self {
		Self::new_desc(TypeRefDesc::new(TypeRefKind::StdVector(vector), Constness::Mut))
	}

	pub fn new_smartptr(smart_ptr: SmartPtr<'tu, 'ge>) -> Self {
		Self::new_desc(TypeRefDesc::new(TypeRefKind::SmartPtr(smart_ptr), Constness::Mut))
	}

	pub fn new_generic(name: impl Into<String>) -> Self {
		Self::new_desc(TypeRefDesc::new(TypeRefKind::Generic(name.into()), Constness::Mut))
	}

	/// Create a [TypeRef] from a textual C++ representation
	///
	/// Correctness may vary, very few [TypeRefKind]s are supported.
	pub fn guess(cpp_refname: &str, rust_module: impl Into<Rc<str>>) -> Self {
		if let Some(element_cpprefname) = cpp_refname.strip_prefix("std::vector<").and_then(|s| s.strip_suffix('>')) {
			TypeRef::new_desc(TypeRefDesc::new(
				TypeRefKind::StdVector(Vector::new_desc(VectorDesc::new(Self::guess(
					element_cpprefname,
					rust_module,
				)))),
				Constness::Mut,
			))
		} else if let Some(primitive_typeref) = TypeRefDesc::try_primitive(cpp_refname) {
			primitive_typeref
		} else {
			let simplicity = settings::DATA_TYPES
				.contains(cpp_refname)
				.then_some(ClassKindOverride::Simple)
				.unwrap_or_else(|| {
					settings::ELEMENT_EXPORT_TWEAK
						.get(cpp_refname)
						.and_then(|export_tweak| export_tweak(ExportConfig::default()))
						.map_or(ClassKindOverride::Boxed, |e| e.class_kind_override)
				});
			let cls = if simplicity.is_boxed() {
				ClassDesc::boxed(cpp_refname, rust_module)
			} else {
				ClassDesc::simple(cpp_refname, rust_module)
			};
			Self::new_class(Class::new_desc(cls))
		}
	}

	pub fn type_hint(&self) -> &TypeRefTypeHint {
		match self {
			Self::Clang { type_hint, .. } => type_hint,
			Self::Desc(desc) => &desc.type_hint,
		}
	}

	pub fn with_type_hint(self, type_hint: TypeRefTypeHint) -> Self {
		let mut out = self;
		out.set_type_hint(type_hint);
		out
	}

	pub fn set_type_hint(&mut self, type_hint: TypeRefTypeHint) {
		if *self.type_hint() != type_hint {
			match self {
				Self::Clang {
					type_hint: old_type_hint,
					..
				} => *old_type_hint = type_hint,
				Self::Desc(ref mut desc) => {
					Rc::make_mut(desc).type_hint = type_hint;
				}
			}
		}
	}

	pub fn kind(&self) -> Cow<TypeRefKind<'tu, 'ge>> {
		match self {
			Self::Clang {
				type_ref,
				type_hint,
				parent_entity,
				gen_env,
			} => Owned(type_ref.kind(type_hint.clone(), *parent_entity, gen_env)),
			Self::Desc(desc) => Borrowed(&desc.kind),
		}
	}

	/// TypeRef with all the typedef's traversed
	pub fn canonical(&self) -> Self {
		match self.kind().as_ref() {
			TypeRefKind::Typedef(tdef) => tdef.underlying_type_ref().canonical(),
			_ => self.clone(),
		}
	}

	/// Removes indirection by pointer and reference, this will also remove typedef if it references a pointer or reference
	pub fn source(&self) -> Self {
		match self.kind().as_ref() {
			TypeRefKind::Pointer(inner) | TypeRefKind::Reference(inner) | TypeRefKind::RValueReference(inner) => inner.source(),
			TypeRefKind::Typedef(tdef) => {
				let underlying_type = tdef.underlying_type_ref();
				match underlying_type.kind().as_ref() {
					TypeRefKind::Pointer(inner) | TypeRefKind::Reference(inner) | TypeRefKind::RValueReference(inner) => {
						inner.source()
					}
					_ => self.clone(),
				}
			}
			_ => self.clone(),
		}
	}

	/// Like source(), but also removes indirection by `Ptr`
	pub fn source_smart(&self) -> Self {
		match self.kind().as_ref() {
			TypeRefKind::Pointer(inner) | TypeRefKind::Reference(inner) | TypeRefKind::RValueReference(inner) => {
				inner.source_smart()
			}
			TypeRefKind::SmartPtr(ptr) => ptr.pointee().source_smart(),
			TypeRefKind::Typedef(tdef) => {
				let underlying_type = tdef.underlying_type_ref();
				match underlying_type.kind().as_ref() {
					TypeRefKind::Pointer(inner) | TypeRefKind::Reference(inner) | TypeRefKind::RValueReference(inner) => {
						inner.source_smart()
					}
					TypeRefKind::SmartPtr(ptr) => ptr.pointee().source_smart(),
					_ => self.clone(),
				}
			}
			_ => self.clone(),
		}
	}

	/// Like source(), but digs down to the elements of arrays
	pub fn base(&self) -> Self {
		match self.kind().as_ref() {
			TypeRefKind::Pointer(inner) | TypeRefKind::Reference(inner) | TypeRefKind::RValueReference(inner) => inner.base(),
			TypeRefKind::SmartPtr(ptr) => ptr.pointee().base(),
			TypeRefKind::Array(inner, ..) => inner.base(),
			TypeRefKind::StdVector(vec) => vec.element_type().base(),
			TypeRefKind::Typedef(tdef) => {
				let underlying_type = tdef.underlying_type_ref();
				match underlying_type.kind().as_ref() {
					TypeRefKind::Pointer(inner) | TypeRefKind::Reference(inner) | TypeRefKind::RValueReference(inner) => inner.base(),
					TypeRefKind::SmartPtr(ptr) => ptr.pointee().base(),
					TypeRefKind::Array(inner, ..) => inner.base(),
					TypeRefKind::StdVector(vec) => vec.element_type().base(),
					_ => self.clone(),
				}
			}
			_ => self.clone(),
		}
	}

	/// Map the contained TypeRef inside `Pointer`, `Reference`, `RValueReference` and `Array` variants,
	/// useful for specializing the templates
	pub fn map<'otu, 'oge>(&self, f: impl FnOnce(&TypeRef<'tu, 'ge>) -> TypeRef<'otu, 'oge>) -> TypeRef<'otu, 'oge> {
		match self.kind().as_ref() {
			TypeRefKind::Pointer(inner) => TypeRef::new_pointer(inner.map(f)),
			TypeRefKind::Reference(inner) => TypeRef::new_reference(inner.map(f)),
			TypeRefKind::RValueReference(inner) => TypeRef::new_rvalue_reference(inner.map(f)),
			TypeRefKind::Array(element, size) => {
				TypeRef::new_desc(TypeRefDesc::new(TypeRefKind::Array(element.map(f), *size), self.constness()))
			}
			_ => f(self),
		}
	}

	/// Map the contained TypeRef inside `Pointer` and `Reference` variants, useful for changing constness
	pub fn map_ptr_ref<'otu, 'oge>(&self, f: impl FnOnce(&TypeRef<'tu, 'ge>) -> TypeRef<'otu, 'oge>) -> TypeRef<'otu, 'oge> {
		match self.kind().as_ref() {
			TypeRefKind::Pointer(inner) => TypeRef::new_pointer(inner.map(f)),
			TypeRefKind::Reference(inner) => TypeRef::new_reference(inner.map(f)),
			_ => f(self),
		}
	}

	/// Map the contained TypeRef inside `Vector` variant
	pub fn map_vector<'otu, 'oge>(&self, f: impl FnOnce(&TypeRef<'tu, 'ge>) -> TypeRef<'otu, 'oge>) -> TypeRef<'otu, 'oge> {
		match self.kind().as_ref() {
			TypeRefKind::StdVector(vec) => TypeRef::new_vector(Vector::new_desc(VectorDesc::new(vec.element_type().map_vector(f)))),
			_ => f(self),
		}
	}

	pub fn exclude_kind(&self) -> ExcludeKind {
		match self.kind().as_ref() {
			TypeRefKind::Generic(_) | TypeRefKind::Ignored => ExcludeKind::Ignored,
			TypeRefKind::StdVector(vec) => vec.exclude_kind(),
			TypeRefKind::StdTuple(tuple) => tuple.exclude_kind(),
			TypeRefKind::Array(inner, ..) => ExcludeKind::Included.with_is_ignored(|| !inner.kind().is_copy(inner.type_hint())),
			TypeRefKind::Pointer(inner) | TypeRefKind::Reference(inner) | TypeRefKind::RValueReference(inner) => {
				inner.exclude_kind()
			}
			TypeRefKind::SmartPtr(ptr) => ptr.exclude_kind(),
			TypeRefKind::Class(cls) => cls.exclude_kind(),
			TypeRefKind::Typedef(tdef) => tdef.exclude_kind(),
			_ => settings::ELEMENT_EXCLUDE_KIND
				.get(self.cpp_name(CppNameStyle::Reference).as_ref())
				.copied()
				.unwrap_or(ExcludeKind::Included),
		}
	}

	pub fn template_kind(&self) -> TemplateKind<'tu, 'ge> {
		match self.base().kind().as_ref() {
			TypeRefKind::Class(cls) => cls.template_kind(),
			_ => TemplateKind::No,
		}
	}

	pub fn constness(&self) -> Constness {
		let inherent_constness = self.inherent_constness();
		if inherent_constness.is_const() {
			Constness::Const
		} else {
			match self.kind().as_ref() {
				TypeRefKind::Class(_) | TypeRefKind::Enum(..) | TypeRefKind::RValueReference(_) => inherent_constness,
				TypeRefKind::Primitive(..) | TypeRefKind::Generic(..) | TypeRefKind::Function(..) | TypeRefKind::Ignored => {
					Constness::Mut
				}
				TypeRefKind::Array(elem, ..) => elem.inherent_constness(),
				TypeRefKind::StdVector(vec) => vec.element_type().inherent_constness(),
				TypeRefKind::StdTuple(tuple) => tuple.constness(),
				TypeRefKind::Pointer(inner) | TypeRefKind::Reference(inner) => inner.inherent_constness(),
				TypeRefKind::SmartPtr(ptr) => ptr.pointee().inherent_constness(),
				TypeRefKind::Typedef(decl) => decl.underlying_type_ref().constness(),
			}
		}
	}

	pub fn inherent_constness(&self) -> Constness {
		match self {
			TypeRef::Clang { type_ref, .. } => Constness::from_is_const(type_ref.is_const_qualified()),
			TypeRef::Desc(desc) => desc.inherent_constness,
		}
	}

	pub fn set_inherent_constness(&mut self, constness: Constness) {
		if self.inherent_constness() != constness {
			match self {
				Self::Clang {
					type_ref,
					type_hint,
					parent_entity,
					gen_env,
				} => {
					*self = Self::new_desc(TypeRefDesc {
						kind: type_ref.kind(type_hint.clone(), *parent_entity, gen_env),
						inherent_constness: constness,
						type_hint: type_hint.clone(),
						template_specialization_args: type_ref.template_specialization_args(gen_env).into(),
					})
				}
				Self::Desc(desc) => {
					Rc::make_mut(desc).inherent_constness = constness;
				}
			}
		}
	}

	pub fn with_inherent_constness(&self, constness: Constness) -> Self {
		let mut out = self.clone();
		out.set_inherent_constness(constness);
		out
	}

	pub fn is_data_type(&self) -> bool {
		settings::DATA_TYPES.contains(self.cpp_name(CppNameStyle::Reference).as_ref())
	}

	/// Returns true if self is a data type or it's a vector with the element being a data type
	pub fn is_element_data_type(&self) -> bool {
		self
			.kind()
			.as_vector()
			.map_or_else(|| self.is_data_type(), |vec| vec.element_type().is_data_type())
	}

	pub fn template_specialization_args(&self) -> Cow<[TemplateArg<'tu, 'ge>]> {
		match self {
			&Self::Clang { type_ref, gen_env, .. } => type_ref.template_specialization_args(gen_env).into(),
			Self::Desc(desc) => desc.template_specialization_args.as_ref().into(),
		}
	}

	pub fn generated_types(&self) -> Vec<GeneratedType<'tu, 'ge>> {
		match self {
			Self::Clang { .. } => {
				let source = self.source();
				match source.kind().into_owned() {
					TypeRefKind::StdVector(vec) => {
						let mut out = vec.generated_types();
						let element_type = vec.element_type();
						if let Some((Dir::In, str_type)) = element_type.kind().as_string(element_type.type_hint()) {
							// implement workaround for race when type with std::string gets generated first
							// we only want vector<cv::String> because it's more compatible across OpenCV versions
							if matches!(str_type, StrType::StdString(_)) {
								// We need to generate return wrappers for std::vector<cv::String>, but it has several issues:
								// * we can't use get_canonical_type() because it resolves into compiler dependent inner type like
								//   std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>
								// * we can't generate both vector<cv::String> and vector<std::string> because for OpenCV 4
								//   cv::String is an typedef to std::string and it would lead to duplicate definition error
								// That's why we try to resolve both types and check if they are the same, if they are we only generate
								// vector<std::string> if not - both.
								out.push(GeneratedType::Vector(VectorDesc::vector_of_cv_string()));
							} else {
								out.push(GeneratedType::Vector(vec))
							}
						} else if element_type.base().kind().is_char() {
							out.reserve(3);
							// C++ char can be signed or unsigned based on the platform and that can lead to duplicate definitions when
							// we generate Vector<u8> together with Vector<c_char>
							out.push(source.map_vector(|_| TypeRefDesc::uchar()).try_into().expect("Known Vector"));
							out.push(source.map_vector(|_| TypeRefDesc::schar()).try_into().expect("Known Vector"));
							out.push(GeneratedType::Vector(vec));
						} else {
							out.push(GeneratedType::Vector(vec));
						}
						out
					}
					TypeRefKind::StdTuple(tuple) => vec![GeneratedType::Tuple(tuple)],
					TypeRefKind::SmartPtr(ptr) => {
						let mut out = ptr.generated_types();
						out.push(GeneratedType::SmartPtr(ptr));
						out
					}
					TypeRefKind::Typedef(typedef) => typedef.generated_types(),
					_ => {
						let mut out = vec![];
						if self.kind().as_abstract_class_ptr().is_some() {
							out.push(GeneratedType::AbstractRefWrapper(AbstractRefWrapper::new(self.clone())))
						}
						out
					}
				}
			}
			Self::Desc(_) => {
				vec![]
			}
		}
	}

	pub fn cpp_name(&self, name_style: CppNameStyle) -> Cow<str> {
		self.cpp_name_ext(name_style, "", true)
	}

	pub fn cpp_name_ext(&self, name_style: CppNameStyle, name: &str, extern_types: bool) -> Cow<str> {
		CppRenderer::new(name_style, name, extern_types).render(self)
	}

	pub fn cpp_extern_return(&self) -> Cow<str> {
		CppExternReturnRenderer.render(self)
	}

	pub fn cpp_extern_return_fallible(&self) -> Cow<str> {
		if self.kind().is_void() {
			"ResultVoid".into()
		} else {
			format!("Result<{ext}>", ext = self.cpp_extern_return()).into()
		}
	}
}

impl PartialEq for TypeRef<'_, '_> {
	fn eq(&self, other: &Self) -> bool {
		self.kind() == other.kind() && self.constness() == other.constness() && self.type_hint() == other.type_hint()
	}
}

impl fmt::Debug for TypeRef<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let kind = self.kind();
		let mut props = vec![];
		if self.template_kind().is_template() {
			props.push("template");
		}
		if kind.is_generic() {
			props.push("generic");
		}
		if kind.as_primitive().is_some() {
			props.push("primitive");
		}
		if let Some((dir, str_type)) = kind.as_string(self.type_hint()) {
			props.push("string");
			let str_type = match dir {
				Dir::In => str_type,
				Dir::Out => {
					props.push("output_string");
					str_type
				}
			};
			match str_type {
				StrType::StdString(StrEnc::Text) => {
					props.push("std_string");
				}
				StrType::CvString(StrEnc::Text) => {
					props.push("cv_string");
				}
				StrType::CharPtr(StrEnc::Text) => {
					props.push("char_ptr_string");
				}
				StrType::StdString(StrEnc::Binary) => {
					props.push("byte_std_string");
				}
				StrType::CvString(StrEnc::Binary) => {
					props.push("byte_cv_string");
				}
				StrType::CharPtr(StrEnc::Binary) => {
					props.push("byte_ptr_string");
				}
			}
		}
		if kind.as_by_move().is_some() {
			props.push("by_move");
		}
		if kind.is_copy(self.type_hint()) {
			props.push("copy");
		}
		if kind.is_clone(self.type_hint()) {
			props.push("clone");
		}
		if kind.is_debug() {
			props.push("debug");
		}
		if self.is_data_type() {
			props.push("data_type");
		}
		if kind.return_as_naked(self.type_hint()) {
			props.push("return_naked");
		}
		if kind.is_rust_by_ptr(self.type_hint()) {
			props.push("rust_by_ptr");
		}
		let props = props.join(", ");
		let mut dbg = f.debug_struct(match self {
			Self::Clang { .. } => "TypeRef::Clang",
			Self::Desc(_) => "TypeRef::Desc",
		});
		dbg.field("cpp_full", &self.cpp_name(CppNameStyle::Reference))
			.field("props", &props)
			.field("render_lane", &self.render_lane())
			.field("kind", &self.kind())
			.field("extern_pass_kind", &self.kind().extern_pass_kind())
			.field("type_hint", &self.type_hint())
			.field("exclude_kind", &self.exclude_kind())
			.field("constness", &self.constness())
			.field("inherent_constness", &self.inherent_constness())
			.field("template_types", &self.template_specialization_args())
			.finish()
	}
}
