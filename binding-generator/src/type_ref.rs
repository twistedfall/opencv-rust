use std::borrow::Cow;
use std::fmt;
use std::rc::Rc;

use clang::{Entity, Type};

pub use desc::TypeRefDesc;
pub use style::{CppNameStyle, Dir, ExternDir, FishStyle, NameStyle, Signedness, StrEnc, StrType};

use crate::class::TemplateKind;
use crate::element::ExcludeKind;
use crate::renderer::{CppExternReturnRenderer, CppRenderer, TypeRefRenderer};
use crate::settings;
use crate::settings::ArgOverride;
use crate::{Class, Element, Enum, Function, GeneratedType, GeneratorEnv, SmartPtr, StringExt, Tuple, Typedef, Vector};
use desc::ClangTypeExt;
use style::ExternPassKind;

mod desc;
mod style;

#[derive(Clone, Debug)]
pub enum TypeRefKind<'tu, 'ge> {
	/// (rust name, cpp name)
	Primitive(&'static str, &'static str),
	/// (element type, array size)
	Array(TypeRef<'tu, 'ge>, Option<usize>),
	StdVector(Vector<'tu, 'ge>),
	StdTuple(Tuple<'tu, 'ge>),
	Pointer(TypeRef<'tu, 'ge>),
	Reference(TypeRef<'tu, 'ge>),
	RValueReference(TypeRef<'tu, 'ge>),
	SmartPtr(SmartPtr<'tu, 'ge>),
	Class(Class<'tu, 'ge>),
	Enum(Enum<'tu>),
	Function(Function<'tu, 'ge>),
	Typedef(Typedef<'tu, 'ge>),
	Generic(String),
	Ignored,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeRefTypeHint<'tu, 'ge> {
	None,
	ArgOverride(ArgOverride),
	PrimitiveRefAsPointer,
	Specialized(Rc<TypeRef<'tu, 'ge>>),
}

#[derive(Clone)]
pub enum TypeRef<'tu, 'ge> {
	Clang {
		type_ref: Type<'tu>,
		type_hint: TypeRefTypeHint<'tu, 'ge>,
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
		type_hint: TypeRefTypeHint<'tu, 'ge>,
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
		Self::new_desc(TypeRefDesc::new(TypeRefKind::Pointer(inner)))
	}

	pub fn new_reference(inner: TypeRef<'tu, 'ge>) -> Self {
		Self::new_desc(TypeRefDesc::new(TypeRefKind::Reference(inner)))
	}

	pub fn new_rvalue_reference(inner: TypeRef<'tu, 'ge>) -> Self {
		Self::new_desc(TypeRefDesc::new(TypeRefKind::RValueReference(inner)))
	}

	pub fn type_hint(&self) -> &TypeRefTypeHint<'tu, 'ge> {
		match self {
			Self::Clang { type_hint, .. } => type_hint,
			Self::Desc(desc) => &desc.type_hint,
		}
	}

	pub fn with_type_hint(self, type_hint: TypeRefTypeHint<'tu, 'ge>) -> Self {
		match self {
			Self::Clang {
				type_ref,
				parent_entity,
				gen_env,
				..
			} => Self::Clang {
				type_ref,
				type_hint,
				parent_entity,
				gen_env,
			},
			Self::Desc(desc) => {
				if desc.type_hint != type_hint {
					let mut desc = Rc::try_unwrap(desc).unwrap_or_else(|desc| (*desc).clone());
					desc.type_hint = type_hint;
					Self::Desc(Rc::new(desc))
				} else {
					Self::Desc(desc)
				}
			}
		}
	}

	pub fn with_constness(&self, constness: Constness) -> Self {
		match self {
			Self::Clang {
				type_ref,
				type_hint,
				parent_entity,
				gen_env,
			} => Self::new_desc(TypeRefDesc {
				kind: type_ref.kind(type_hint.clone(), *parent_entity, gen_env),
				inherent_constness: constness,
				type_hint: type_hint.clone(),
				template_specialization_args: type_ref.template_specialization_args(gen_env).into(),
			}),
			Self::Desc(desc) => {
				if desc.inherent_constness != constness {
					let mut desc = (**desc).clone();
					desc.inherent_constness = constness;
					Self::Desc(Rc::new(desc))
				} else {
					Self::Desc(Rc::clone(desc))
				}
			}
		}
	}

	pub fn clang_type(&self) -> Option<Type<'tu>> {
		match self {
			&TypeRef::Clang { type_ref, .. } => Some(type_ref),
			TypeRef::Desc(_) => None,
		}
	}

	pub fn kind(&self) -> Cow<TypeRefKind<'tu, 'ge>> {
		match self {
			Self::Clang {
				type_ref,
				type_hint,
				parent_entity,
				gen_env,
			} => Cow::Owned(type_ref.kind(type_hint.clone(), *parent_entity, gen_env)),
			Self::Desc(desc) => Cow::Borrowed(&desc.kind),
		}
	}

	/// TypeRef with all of the typedef's traversed
	pub fn canonical(&self) -> TypeRef<'tu, 'ge> {
		match self.kind().as_ref() {
			TypeRefKind::Typedef(tdef) => tdef.underlying_type_ref().canonical(),
			_ => self.clone(),
		}
	}

	/// Like canonical(), but also removes indirection by pointer and reference
	pub fn source(&self) -> TypeRef<'tu, 'ge> {
		let canonical = self.canonical();
		match canonical.kind().as_ref() {
			TypeRefKind::Pointer(inner) | TypeRefKind::Reference(inner) | TypeRefKind::RValueReference(inner) => inner.source(),
			_ => canonical,
		}
	}

	/// Like source(), but digs down to the elements of arrays
	pub fn base(&self) -> TypeRef<'tu, 'ge> {
		let source = self.source();
		match source.kind().as_ref() {
			TypeRefKind::Array(inner, ..) => inner.base(),
			TypeRefKind::StdVector(vec) => vec.element_type().base(),
			TypeRefKind::SmartPtr(ptr) => ptr.pointee().base(),
			_ => source,
		}
	}

	/// Map the contained TypeRef inside `Pointer`, `Reference`, `RValueReference` and `Array` variants,
	/// useful for specializing the templates
	pub fn map<'otu, 'oge>(&self, f: impl FnOnce(&TypeRef<'tu, 'ge>) -> TypeRef<'otu, 'oge>) -> TypeRef<'otu, 'oge> {
		match self.kind().as_ref() {
			TypeRefKind::Pointer(inner) => TypeRef::new_pointer(inner.map(f)),
			TypeRefKind::Reference(inner) => TypeRef::new_reference(inner.map(f)),
			TypeRefKind::RValueReference(inner) => TypeRef::new_rvalue_reference(inner.map(f)),
			TypeRefKind::Array(element, size) => TypeRef::new_desc(TypeRefDesc::new(TypeRefKind::Array(element.map(f), *size))),
			_ => f(self),
		}
	}

	pub fn exclude_kind(&self) -> ExcludeKind {
		match self.kind().as_ref() {
			TypeRefKind::Generic(_) | TypeRefKind::Ignored => ExcludeKind::Ignored,
			TypeRefKind::StdVector(vec) => vec.exclude_kind(),
			TypeRefKind::StdTuple(tuple) => tuple.exclude_kind(),
			TypeRefKind::Array(inner, ..)
			| TypeRefKind::Pointer(inner)
			| TypeRefKind::Reference(inner)
			| TypeRefKind::RValueReference(inner) => inner.exclude_kind(),
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

	pub fn is_generic(&self) -> bool {
		matches!(self.base().kind().as_ref(), TypeRefKind::Generic(..))
	}

	pub fn constness(&self) -> Constness {
		if self.clang_constness().is_const() {
			Constness::Const
		} else {
			match self.kind().as_ref() {
				TypeRefKind::Primitive(..)
				| TypeRefKind::Generic(..)
				| TypeRefKind::Enum(..)
				| TypeRefKind::Class(..)
				| TypeRefKind::Function(..)
				| TypeRefKind::Ignored => Constness::Mut,
				TypeRefKind::Array(elem, ..) => elem.clang_constness(),
				TypeRefKind::StdVector(vec) => vec.element_type().clang_constness(),
				TypeRefKind::StdTuple(tuple) => tuple.constness(),
				TypeRefKind::Pointer(inner) | TypeRefKind::Reference(inner) | TypeRefKind::RValueReference(inner) => {
					inner.clang_constness()
				}
				TypeRefKind::SmartPtr(ptr) => ptr.pointee().clang_constness(),
				TypeRefKind::Typedef(decl) => decl.underlying_type_ref().constness(),
			}
		}
	}

	pub fn clang_constness(&self) -> Constness {
		match self {
			TypeRef::Clang { type_ref, .. } => Constness::from_is_const(type_ref.is_const_qualified()),
			TypeRef::Desc(desc) => desc.inherent_constness,
		}
	}

	pub fn is_primitive(&self) -> bool {
		matches!(self.canonical().kind().as_ref(), TypeRefKind::Primitive(..))
	}

	pub fn is_enum(&self) -> bool {
		matches!(self.canonical().kind().as_ref(), TypeRefKind::Enum(..))
	}

	pub fn as_string(&self) -> Option<Dir<StrType>> {
		match self.canonical().kind().as_ref() {
			TypeRefKind::Class(cls) => {
				if let Some(mut typ) = cls.string_type() {
					if matches!(self.type_hint(), TypeRefTypeHint::ArgOverride(ArgOverride::StringAsBytes)) {
						typ.set_encoding(StrEnc::Binary)
					}
					return Some(Dir::In(typ));
				}
			}
			TypeRefKind::Reference(inner) => {
				if let Some(dir) = inner.as_string().map(|d| d.with_out_dir(inner.clang_constness().is_mut())) {
					return Some(dir);
				}
			}
			TypeRefKind::Pointer(inner) => {
				if let Some(dir) = inner.as_string().map(|d| d.with_out_dir(inner.clang_constness().is_mut())) {
					return Some(dir);
				} else {
					let inner_cpp_ref = inner.cpp_name(CppNameStyle::Reference);
					if inner_cpp_ref == "char" || inner_cpp_ref == "const char" {
						return if inner.clang_constness().is_const() {
							Some(Dir::In(StrType::CharPtr))
						} else {
							Some(Dir::Out(StrType::CharPtr))
						};
					}
				}
			}
			TypeRefKind::Array(inner, ..) => {
				let inner_cpp_ref = inner.cpp_name(CppNameStyle::Reference);
				if inner_cpp_ref == "char" || inner_cpp_ref == "const char" {
					return Some(Dir::In(StrType::CharPtr));
				}
			}
			_ => {}
		}
		None
	}

	pub fn is_std_string(&self) -> bool {
		matches!(self.as_string(), Some(Dir::In(StrType::StdString(_))))
	}

	pub fn is_cv_string(&self) -> bool {
		matches!(self.as_string(), Some(Dir::In(StrType::CvString(_))))
	}

	pub fn is_char_ptr_string(&self) -> bool {
		matches!(self.as_string(), Some(Dir::In(StrType::CharPtr)))
	}

	pub fn is_input_array(&self) -> bool {
		match self.kind().as_ref() {
			TypeRefKind::Reference(inner) => inner.is_input_array(),
			TypeRefKind::Class(cls) => cls.cpp_name(CppNameStyle::Reference) == "cv::_InputArray",
			TypeRefKind::Typedef(tdef) => {
				let cpp_refname = tdef.cpp_name(CppNameStyle::Reference);
				cpp_refname == "cv::InputArray" || cpp_refname == "cv::InputArrayOfArrays"
			}
			_ => false,
		}
	}

	pub fn is_output_array(&self) -> bool {
		match self.kind().as_ref() {
			TypeRefKind::Reference(inner) => inner.is_output_array(),
			TypeRefKind::Class(cls) => cls.cpp_name(CppNameStyle::Reference) == "cv::_OutputArray",
			TypeRefKind::Typedef(tdef) => {
				let cpp_refname = tdef.cpp_name(CppNameStyle::Reference);
				cpp_refname == "cv::OutputArray" || cpp_refname == "cv::OutputArrayOfArrays"
			}
			_ => false,
		}
	}

	pub fn is_input_output_array(&self) -> bool {
		match self.kind().as_ref() {
			TypeRefKind::Reference(inner) => inner.is_input_output_array(),
			TypeRefKind::Class(cls) => cls.cpp_name(CppNameStyle::Reference) == "cv::_InputOutputArray",
			TypeRefKind::Typedef(tdef) => {
				let cpp_refname = tdef.cpp_name(CppNameStyle::Reference);
				cpp_refname == "cv::InputOutputArray" || cpp_refname == "cv::InputOutputArrayOfArrays"
			}
			_ => false,
		}
	}

	pub fn is_void(&self) -> bool {
		matches!(self.canonical().kind().as_ref(), TypeRefKind::Primitive(_, "void"))
	}

	pub fn is_bool(&self) -> bool {
		matches!(self.canonical().kind().as_ref(), TypeRefKind::Primitive(_, "bool"))
	}

	pub fn is_void_ptr(&self) -> bool {
		self.as_pointer().map_or(false, |inner| inner.is_void())
	}

	pub fn as_pointer(&self) -> Option<TypeRef<'tu, 'ge>> {
		if let TypeRefKind::Pointer(out) = self.canonical().kind().into_owned() {
			Some(out)
		} else {
			None
		}
	}

	pub fn as_reference(&self) -> Option<TypeRef<'tu, 'ge>> {
		if let TypeRefKind::Reference(out) | TypeRefKind::RValueReference(out) = self.canonical().kind().into_owned() {
			Some(out)
		} else {
			None
		}
	}

	/// True for types whose values are moved as per C++ function specification
	pub fn is_by_move(&self) -> bool {
		matches!(self.canonical().kind().as_ref(), TypeRefKind::RValueReference(_))
	}

	pub fn is_copy(&self) -> bool {
		self.is_primitive() || self.is_enum() || self.is_char_ptr_string() || self.canonical().as_simple_class().is_some()
	}

	pub fn is_clone(&self) -> bool {
		self.is_copy()
			|| match self.kind().as_ref() {
				TypeRefKind::StdVector(vec) => vec.element_type().is_clone(),
				TypeRefKind::Class(cls) => cls.has_explicit_clone() || cls.has_implicit_clone(),
				_ => false,
			}
	}

	/// True if a `TypeRef` has `std::fmt::Debug` implementation
	pub fn is_debug(&self) -> bool {
		match self.kind().as_ref() {
			TypeRefKind::Primitive(_, _) | TypeRefKind::Class(_) | TypeRefKind::Enum(_) | TypeRefKind::SmartPtr(_) => true,
			TypeRefKind::Array(elem, _) => elem.is_debug(),
			TypeRefKind::StdVector(vec) => vec.element_type().is_debug(),
			TypeRefKind::StdTuple(tuple) => tuple.elements().into_iter().all(|e| e.is_debug()),
			TypeRefKind::Pointer(inner) | TypeRefKind::Reference(inner) | TypeRefKind::RValueReference(inner) => inner.is_debug(),
			TypeRefKind::Function(_) | TypeRefKind::Generic(_) | TypeRefKind::Ignored => false,
			TypeRefKind::Typedef(tdef) => tdef.underlying_type_ref().is_debug(),
		}
	}

	pub fn as_char8(&self) -> Option<Signedness> {
		if matches!(self.type_hint(), TypeRefTypeHint::ArgOverride(ArgOverride::Char8AsChar)) {
			match self.kind().as_ref() {
				TypeRefKind::Primitive("i8", "char") => Some(Signedness::Signed),
				TypeRefKind::Primitive("u8", "char") => Some(Signedness::Unsigned),
				_ => None,
			}
		} else {
			None
		}
	}

	pub fn is_nullable(&self) -> bool {
		matches!(
			self.type_hint(),
			TypeRefTypeHint::ArgOverride(ArgOverride::NullableSlice) | TypeRefTypeHint::ArgOverride(ArgOverride::Nullable)
		)
	}

	pub fn as_smart_ptr(&self) -> Option<SmartPtr<'tu, 'ge>> {
		if let TypeRefKind::SmartPtr(out) = self.canonical().kind().into_owned() {
			Some(out)
		} else {
			None
		}
	}

	pub fn as_class(&self) -> Option<Class<'tu, 'ge>> {
		if let TypeRefKind::Class(out) = self.canonical().kind().into_owned() {
			Some(out)
		} else {
			None
		}
	}

	pub fn as_simple_class(&self) -> Option<Class<'tu, 'ge>> {
		match self.canonical().kind().into_owned() {
			TypeRefKind::Class(out) if out.kind().is_simple() => Some(out),
			_ => None,
		}
	}

	pub fn as_abstract_class_ptr(&self) -> Option<(TypeRef<'tu, 'ge>, Class<'tu, 'ge>)> {
		if let Some(pointee) = self.as_pointer() {
			if let Some(class) = pointee.as_class() {
				if class.is_abstract() {
					return Some((pointee, class));
				}
			}
		}
		None
	}

	pub fn as_array(&self) -> Option<(TypeRef<'tu, 'ge>, Option<usize>)> {
		if let TypeRefKind::Array(elem, size) = self.canonical().kind().into_owned() {
			Some((elem, size))
		} else {
			None
		}
	}

	pub fn as_variable_array(&self) -> Option<TypeRef<'tu, 'ge>> {
		if let Some((elem, None)) = self.as_array() {
			Some(elem)
		} else {
			None
		}
	}

	pub fn as_fixed_array(&self) -> Option<(TypeRef<'tu, 'ge>, usize)> {
		if let Some((elem, Some(size))) = self.as_array() {
			Some((elem, size))
		} else {
			None
		}
	}

	pub fn as_string_array(&self) -> Option<(TypeRef<'tu, 'ge>, Option<usize>)> {
		if let Some((elem, size)) = self.as_array() {
			if elem.as_string().is_some() {
				return Some((elem, size));
			}
		}
		None
	}

	pub fn as_vector(&self) -> Option<Vector<'tu, 'ge>> {
		if let TypeRefKind::StdVector(out) = self.canonical().kind().into_owned() {
			Some(out)
		} else {
			None
		}
	}

	pub fn as_tuple(&self) -> Option<Tuple<'tu, 'ge>> {
		if let TypeRefKind::StdTuple(out) = self.canonical().kind().into_owned() {
			Some(out)
		} else {
			None
		}
	}

	pub fn as_function(&self) -> Option<Function<'tu, 'ge>> {
		match self.canonical().kind().into_owned() {
			TypeRefKind::Function(out) => Some(out),
			_ => None,
		}
	}

	pub fn as_typedef(&self) -> Option<Typedef<'tu, 'ge>> {
		match self.kind().into_owned() {
			TypeRefKind::Typedef(out) => Some(out),
			_ => None,
		}
	}

	pub fn extern_pass_kind(&self) -> ExternPassKind {
		match self.canonical().kind().as_ref() {
			TypeRefKind::Class(inner) if !inner.string_type().is_some() => {
				if inner.kind().is_boxed() {
					ExternPassKind::ByVoidPtr
				} else {
					ExternPassKind::ByPtr
				}
			}
			TypeRefKind::Pointer(inner) | TypeRefKind::Reference(inner) | TypeRefKind::RValueReference(inner) => {
				inner.extern_pass_kind()
			}
			TypeRefKind::SmartPtr(_) | TypeRefKind::StdVector(_) | TypeRefKind::StdTuple(_) => ExternPassKind::ByVoidPtr,
			_ => ExternPassKind::AsIs,
		}
	}

	pub fn is_data_type(&self) -> bool {
		settings::DATA_TYPES.contains(self.cpp_name(CppNameStyle::Reference).as_ref())
	}

	pub fn is_element_data_type(&self) -> bool {
		self
			.as_vector()
			.map_or_else(|| self.is_data_type(), |vec| vec.element_type().is_data_type())
	}

	pub fn return_as_naked(&self) -> bool {
		match self.kind().as_ref() {
			TypeRefKind::Primitive(_, _) | TypeRefKind::Pointer(_) | TypeRefKind::Array(_, _) => true,
			_ => self.extern_pass_kind().is_by_void_ptr() || self.as_string().is_some(),
		}
	}

	fn can_rust_by_ptr(&self) -> bool {
		if let Some(inner) = self.as_pointer() {
			if inner.is_primitive() && !self.as_string().is_some() {
				return true;
			}
		}
		false
	}

	/// True for types that get passed by Rust pointer as opposed to a reference or an owned value
	pub fn is_rust_by_ptr(&self) -> bool {
		self.is_void_ptr() || matches!(self.type_hint(), TypeRefTypeHint::PrimitiveRefAsPointer) && self.can_rust_by_ptr()
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
				match self.source().kind().into_owned() {
					TypeRefKind::StdVector(vec) => {
						let mut out = vec.generated_types();
						out.reserve(2);
						if let Some(Dir::In(str_type)) = vec.element_type().as_string() {
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
								out.push(GeneratedType::Vector(
									TypeRefDesc::vector_of_string()
										.as_vector()
										.expect("Not possible unless something is terribly broken"),
								));
							} else {
								out.push(GeneratedType::Vector(vec))
							}
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
					_ => vec![],
				}
			}
			Self::Desc(_) => {
				vec![]
			}
		}
	}

	pub fn cpp_safe_id(&self) -> Cow<str> {
		let mut out: String = self.cpp_name_ext(CppNameStyle::Declaration, "", false).into_owned();
		out.cleanup_name();
		out.into()
	}

	#[inline]
	pub fn render<'a>(&self, renderer: impl TypeRefRenderer<'a>) -> Cow<str> {
		renderer.render(self)
	}

	pub fn cpp_name(&self, name_style: CppNameStyle) -> Cow<str> {
		self.cpp_name_ext(name_style, "", true)
	}

	pub fn cpp_name_ext(&self, name_style: CppNameStyle, name: &str, extern_types: bool) -> Cow<str> {
		self.render(CppRenderer::new(name_style, name, extern_types))
	}

	pub fn cpp_extern(&self) -> Cow<str> {
		self.cpp_extern_with_name("")
	}

	pub fn cpp_extern_with_name(&self, name: &str) -> Cow<str> {
		let space_name = if name.is_empty() {
			"".to_string()
		} else {
			format!(" {name}")
		};
		if let Some(dir) = self.as_string() {
			match dir {
				Dir::In(_) => format!("{cnst}char*{name}", cnst = self.constness().cpp_qual(), name = space_name).into(),
				Dir::Out(_) => format!("{cnst}void*{name}", cnst = self.constness().cpp_qual(), name = space_name).into(),
			}
		} else if self.extern_pass_kind().is_by_void_ptr() {
			if self.as_pointer().is_some() || self.as_reference().is_some() {
				self.cpp_name_ext(CppNameStyle::Reference, name, true)
			} else {
				TypeRef::new_pointer(self.with_constness(self.constness()))
					.cpp_name_ext(CppNameStyle::Reference, name, true)
					.into_owned()
					.into()
			}
		} else {
			self.cpp_name_ext(CppNameStyle::Reference, name, true)
		}
	}

	pub fn cpp_extern_return(&self) -> Cow<str> {
		self.render(CppExternReturnRenderer)
	}

	pub fn cpp_extern_return_fallible(&self) -> Cow<str> {
		if self.is_void() {
			"Result_void".into()
		} else {
			format!("Result<{ext}>", ext = self.cpp_extern_return()).into()
		}
	}
}

impl PartialEq for TypeRef<'_, '_> {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(
				Self::Clang {
					type_ref: left_type_ref, ..
				},
				Self::Clang {
					type_ref: right_type_ref,
					..
				},
			) => left_type_ref.eq(right_type_ref),
			(left, right) => left
				.cpp_name(CppNameStyle::Reference)
				.eq(right.cpp_name(CppNameStyle::Reference).as_ref()),
		}
	}
}

impl fmt::Debug for TypeRef<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut props = vec![];
		if self.template_kind().is_template() {
			props.push("template");
		}
		if self.is_generic() {
			props.push("generic");
		}
		if self.is_primitive() {
			props.push("primitive");
		}
		if self.is_enum() {
			props.push("enum");
		}
		if let Some(str_type) = self.as_string() {
			props.push("string");
			let str_type = match str_type {
				Dir::In(str_type) => str_type,
				Dir::Out(str_type) => {
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
				StrType::CharPtr => {
					props.push("char_ptr_string");
				}
				StrType::StdString(StrEnc::Binary) => {
					props.push("byte_std_string");
				}
				StrType::CvString(StrEnc::Binary) => {
					props.push("byte_cv_string");
				}
			}
		}
		if self.is_input_array() {
			props.push("input_array");
		}
		if self.is_output_array() {
			props.push("output_array");
		}
		if self.is_input_output_array() {
			props.push("input_output_array");
		}
		if self.is_by_move() {
			props.push("by_move");
		}
		if self.is_copy() {
			props.push("copy");
		}
		if self.is_clone() {
			props.push("clone");
		}
		if self.is_debug() {
			props.push("debug");
		}
		if self.is_nullable() {
			props.push("nullable");
		}
		if self.is_data_type() {
			props.push("data_type");
		}
		if self.return_as_naked() {
			props.push("return_naked");
		}
		if self.is_rust_by_ptr() {
			props.push("rust_by_ptr");
		}
		let props = props.join(", ");
		let mut dbg = f.debug_struct(match self {
			Self::Clang { .. } => "TypeRef::Clang",
			Self::Desc(_) => "TypeRef::Desc",
		});
		dbg.field("cpp_full", &self.cpp_name(CppNameStyle::Reference))
			.field("props", &props)
			.field("kind", &self.kind())
			.field("type_hint", self.type_hint())
			.field("exclude_kind", &self.exclude_kind())
			.field("constness", &self.constness())
			.field("clang_constness", &self.clang_constness())
			.field("extern_pass_kind", &self.extern_pass_kind())
			.field("template_types", &self.template_specialization_args())
			.finish()
	}
}

#[derive(Clone, Debug)]
pub enum TypeRefTemplateArg<TRef> {
	Unknown,
	Typename(TRef),
	Constant(String),
}

pub type TemplateArg<'tu, 'ge> = TypeRefTemplateArg<TypeRef<'tu, 'ge>>;

impl<TRef> TypeRefTemplateArg<TRef> {
	pub fn into_typename(self) -> Option<TRef> {
		match self {
			TypeRefTemplateArg::Typename(t) => Some(t),
			TypeRefTemplateArg::Unknown | TypeRefTemplateArg::Constant(_) => None,
		}
	}

	pub fn as_typename(&self) -> Option<&TRef> {
		match self {
			TypeRefTemplateArg::Typename(t) => Some(t),
			TypeRefTemplateArg::Unknown | TypeRefTemplateArg::Constant(_) => None,
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Constness {
	Const,
	Mut,
}

impl Constness {
	pub fn from_is_const(is_const: bool) -> Self {
		if is_const {
			Self::Const
		} else {
			Self::Mut
		}
	}

	pub fn from_is_mut(is_mut: bool) -> Self {
		Self::from_is_const(!is_mut)
	}

	pub fn is_const(self) -> bool {
		match self {
			Self::Const => true,
			Self::Mut => false,
		}
	}

	pub fn is_mut(self) -> bool {
		!self.is_const()
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

	pub fn rust_null_ptr(self) -> &'static str {
		if self.is_const() {
			"::core::ptr::null()"
		} else {
			"::core::ptr::null_mut()"
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
