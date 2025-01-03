use std::borrow::Cow;

use clang::TypeKind;
use Cow::{Borrowed, Owned};

use crate::function::Function;
use crate::smart_ptr::SmartPtr;
use crate::tuple::Tuple;
use crate::type_ref::{Dir, StrEnc, StrType, TypeRef, TypeRefTypeHint};
use crate::vector::Vector;
use crate::{Class, CppNameStyle, Element, Enum, Typedef};

#[derive(Clone, Debug, PartialEq)]
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

impl<'tu, 'ge> TypeRefKind<'tu, 'ge> {
	pub fn try_from_clang_primitive(type_kind: TypeKind) -> Option<Self> {
		match type_kind {
			TypeKind::Void => Some(TypeRefKind::Primitive("()", "void")),
			TypeKind::Bool => Some(TypeRefKind::Primitive("bool", "bool")),
			TypeKind::CharS | TypeKind::CharU => Some(TypeRefKind::Primitive("c_char", "char")),
			TypeKind::SChar => Some(TypeRefKind::Primitive("i8", "signed char")),
			TypeKind::UChar => Some(TypeRefKind::Primitive("u8", "unsigned char")),
			TypeKind::WChar => Some(TypeRefKind::Primitive("u16", "wchar_t")),
			TypeKind::Char16 => Some(TypeRefKind::Primitive("u16", "char16_t")),
			TypeKind::Char32 => Some(TypeRefKind::Primitive("char", "char32_t")),
			TypeKind::Short => Some(TypeRefKind::Primitive("i16", "short")),
			TypeKind::UShort => Some(TypeRefKind::Primitive("u16", "unsigned short")),
			TypeKind::Int => Some(TypeRefKind::Primitive("i32", "int")),
			TypeKind::UInt => Some(TypeRefKind::Primitive("u32", "unsigned int")),
			TypeKind::Long => Some(TypeRefKind::Primitive("i32", "long")),
			TypeKind::ULong => Some(TypeRefKind::Primitive("u32", "unsigned long")),
			TypeKind::LongLong => Some(TypeRefKind::Primitive("i64", "long long")),
			TypeKind::ULongLong => Some(TypeRefKind::Primitive("u64", "unsigned long long")),
			TypeKind::Int128 => Some(TypeRefKind::Primitive("i128", "__int128_t")),
			TypeKind::UInt128 => Some(TypeRefKind::Primitive("u128", "__uint128_t")),
			TypeKind::Float => Some(TypeRefKind::Primitive("f32", "float")),
			TypeKind::Double => Some(TypeRefKind::Primitive("f64", "double")),
			TypeKind::LongDouble => Some(TypeRefKind::Primitive("u128", "long double")),
			TypeKind::Float128 => Some(TypeRefKind::Primitive("u128", "__float128")),
			TypeKind::Half => Some(TypeRefKind::Primitive("u16", "__fp16")),
			_ => None,
		}
	}

	pub fn extern_pass_kind(&self) -> ExternPassKind {
		match self {
			TypeRefKind::Typedef(tdef) => tdef.underlying_type_ref().kind().extern_pass_kind(),
			TypeRefKind::Class(inner) if !inner.string_type().is_some() => {
				if inner.kind().is_trait() {
					ExternPassKind::ByVoidPtr
				} else {
					ExternPassKind::ByPtr
				}
			}
			TypeRefKind::Pointer(inner) | TypeRefKind::Reference(inner) | TypeRefKind::RValueReference(inner) => {
				inner.kind().extern_pass_kind()
			}
			TypeRefKind::SmartPtr(_) | TypeRefKind::StdVector(_) | TypeRefKind::StdTuple(_) => ExternPassKind::ByVoidPtr,
			TypeRefKind::Primitive(_, _)
			| TypeRefKind::Array(_, _)
			| TypeRefKind::Class(_)
			| TypeRefKind::Enum(_)
			| TypeRefKind::Function(_)
			| TypeRefKind::Generic(_)
			| TypeRefKind::Ignored => ExternPassKind::AsIs,
		}
	}

	/// TypeRefKind with all of the typedef's traversed
	#[inline]
	pub fn canonical(&self) -> Cow<Self> {
		match self {
			TypeRefKind::Typedef(tdef) => Owned(tdef.underlying_type_ref().kind().canonical().into_owned()),
			_ => Borrowed(self),
		}
	}

	/// Returns Some((rust_name, cpp_name)) if canonical kind is primitive, None otherwise
	pub fn as_primitive(&self) -> Option<(&'static str, &'static str)> {
		match self {
			TypeRefKind::Primitive(rust, cpp) => Some((rust, cpp)),
			TypeRefKind::Typedef(tdef) => tdef.underlying_type_ref().kind().as_primitive(),
			_ => None,
		}
	}

	pub fn as_pointer(&self) -> Option<Cow<TypeRef<'tu, 'ge>>> {
		match self {
			TypeRefKind::Pointer(out) => Some(Borrowed(out)),
			TypeRefKind::Typedef(tdef) => tdef
				.underlying_type_ref()
				.kind()
				.as_pointer()
				.map(|inner| Owned(inner.into_owned())),
			_ => None,
		}
	}

	pub fn as_reference(&self) -> Option<Cow<TypeRef<'tu, 'ge>>> {
		match self {
			TypeRefKind::Reference(inner) => Some(Borrowed(inner)),
			TypeRefKind::Typedef(tdef) => tdef
				.underlying_type_ref()
				.kind()
				.as_reference()
				.map(|inner| Owned(inner.into_owned())),
			_ => None,
		}
	}

	pub fn is_pointer_reference(&self) -> bool {
		match self {
			TypeRefKind::Pointer(_) | TypeRefKind::Reference(_) => true,
			TypeRefKind::Typedef(tdef) => tdef.underlying_type_ref().kind().is_pointer_reference(),
			_ => false,
		}
	}

	pub fn as_pointer_reference_move(&self) -> Option<Cow<TypeRef<'tu, 'ge>>> {
		match self {
			TypeRefKind::Pointer(inner) | TypeRefKind::Reference(inner) | TypeRefKind::RValueReference(inner) => {
				Some(Borrowed(inner))
			}
			TypeRefKind::Typedef(tdef) => tdef
				.underlying_type_ref()
				.kind()
				.as_pointer_reference_move()
				.map(|inner| Owned(inner.into_owned())),
			_ => None,
		}
	}

	/// Some with inner type for types whose values are moved as per C++ function specification (denoted with &&)
	pub fn as_by_move(&self) -> Option<Cow<TypeRef<'tu, 'ge>>> {
		match self {
			TypeRefKind::RValueReference(inner) => Some(Borrowed(inner)),
			TypeRefKind::Typedef(tdef) => tdef
				.underlying_type_ref()
				.kind()
				.as_by_move()
				.map(|inner| Owned(inner.into_owned())),
			_ => None,
		}
	}

	pub fn as_enum(&self) -> Option<&Enum<'tu>> {
		if let TypeRefKind::Enum(out) = self {
			Some(out)
		} else {
			None
		}
	}

	pub fn as_fixed_array(&self) -> Option<(Cow<TypeRef<'tu, 'ge>>, usize)> {
		match self {
			TypeRefKind::Array(elem, Some(size)) => Some((Borrowed(elem), *size)),
			TypeRefKind::Typedef(tdef) => tdef
				.underlying_type_ref()
				.kind()
				.as_fixed_array()
				.map(|(elem, size)| (Owned(elem.into_owned()), size)),
			_ => None,
		}
	}

	pub fn as_variable_array(&self) -> Option<Cow<TypeRef<'tu, 'ge>>> {
		match self {
			TypeRefKind::Array(elem, None) => Some(Borrowed(elem)),
			TypeRefKind::Typedef(tdef) => tdef
				.underlying_type_ref()
				.kind()
				.as_variable_array()
				.map(|elem| Owned(elem.into_owned())),
			_ => None,
		}
	}

	pub fn as_smart_ptr(&self) -> Option<&SmartPtr<'tu, 'ge>> {
		if let TypeRefKind::SmartPtr(out) = self {
			Some(out)
		} else {
			None
		}
	}

	pub fn as_class(&self) -> Option<Cow<Class<'tu, 'ge>>> {
		match self {
			TypeRefKind::Class(out) => Some(Borrowed(out)),
			TypeRefKind::Typedef(tdef) => tdef
				.underlying_type_ref()
				.kind()
				.as_class()
				.map(|inner| Owned(inner.into_owned())),
			_ => None,
		}
	}

	pub fn as_vector(&self) -> Option<Cow<Vector<'tu, 'ge>>> {
		match self {
			TypeRefKind::StdVector(out) => Some(Borrowed(out)),
			TypeRefKind::Typedef(tdef) => tdef
				.underlying_type_ref()
				.kind()
				.as_vector()
				.map(|inner| Owned(inner.into_owned())),
			_ => None,
		}
	}

	pub fn as_typedef(&self) -> Option<&Typedef<'tu, 'ge>> {
		match self {
			TypeRefKind::Typedef(out) => Some(out),
			_ => None,
		}
	}

	pub fn as_string(&self, type_hint: &TypeRefTypeHint) -> Option<(Dir, StrType)> {
		let mut out = match self {
			TypeRefKind::Class(cls) => cls.string_type().map(|str_type| (Dir::In, str_type)),
			TypeRefKind::Reference(inner) => inner
				.kind()
				.as_string(type_hint)
				.map(|(_, str_type)| (Dir::from_out_dir(inner.inherent_constness().is_mut()), str_type)),
			TypeRefKind::Pointer(inner) => inner
				.kind()
				.as_string(type_hint)
				.or_else(|| {
					(!matches!(
						type_hint,
						TypeRefTypeHint::CharPtrSingleChar | TypeRefTypeHint::PrimitivePtrAsRaw
					) && inner.kind().is_char())
					.then_some((Dir::In, StrType::CharPtr(StrEnc::Text)))
				})
				.map(|(_, str_type)| (Dir::from_out_dir(inner.inherent_constness().is_mut()), str_type)),
			TypeRefKind::Array(inner, ..) => {
				if inner.kind().is_char() {
					Some((Dir::from_out_dir(inner.constness().is_mut()), StrType::CharPtr(StrEnc::Text)))
				} else {
					None
				}
			}
			TypeRefKind::Typedef(tdef) => tdef.underlying_type_ref().kind().as_string(type_hint),
			_ => None,
		};
		if let Some((_, str_type)) = out.as_mut() {
			if matches!(type_hint, TypeRefTypeHint::StringAsBytes(_)) {
				str_type.set_encoding(StrEnc::Binary)
			}
		}
		out
	}

	pub fn is_std_string(&self, type_hint: &TypeRefTypeHint) -> bool {
		matches!(self.as_string(type_hint), Some((_, StrType::StdString(_))))
	}

	pub fn is_cv_string(&self, type_hint: &TypeRefTypeHint) -> bool {
		matches!(self.as_string(type_hint), Some((_, StrType::CvString(_))))
	}

	pub fn is_char_ptr_string(&self, type_hint: &TypeRefTypeHint) -> bool {
		matches!(self.as_string(type_hint), Some((_, StrType::CharPtr(_))))
	}

	pub fn is_bool(&self) -> bool {
		matches!(self.as_primitive(), Some((_, "bool")))
	}

	pub fn is_char(&self) -> bool {
		matches!(self.as_primitive(), Some((_, "char")))
	}

	pub fn is_size_t(&self) -> bool {
		matches!(self.as_primitive(), Some((_, "size_t")))
	}

	pub fn is_unsigned_char(&self) -> bool {
		matches!(self.as_primitive(), Some((_, "unsigned char")))
	}

	pub fn is_void(&self) -> bool {
		matches!(self.as_primitive(), Some((_, "void")))
	}

	pub fn is_void_ptr(&self) -> bool {
		self.as_pointer().is_some_and(|inner| inner.kind().is_void())
	}

	/// Returns true if the type is a slice (judging by the type_hint) and its element is C++ `void`
	///
	/// We want to present such cases as `&[u8]` on the Rust side.
	pub fn is_void_slice(&self, type_hint: &TypeRefTypeHint) -> bool {
		matches!(type_hint, TypeRefTypeHint::Slice) && self.as_variable_array().is_some_and(|inner| inner.kind().is_void())
	}

	pub fn is_generic(&self) -> bool {
		match self {
			TypeRefKind::Generic(_) => true,
			TypeRefKind::Pointer(inner)
			| TypeRefKind::Reference(inner)
			| TypeRefKind::RValueReference(inner)
			| TypeRefKind::Array(inner, ..) => inner.kind().is_generic(),
			TypeRefKind::Typedef(tdef) => tdef.underlying_type_ref().kind().is_generic(),
			TypeRefKind::StdVector(vec) => vec.element_type().kind().is_generic(),
			TypeRefKind::SmartPtr(ptr) => ptr.pointee().kind().is_generic(),
			_ => false,
		}
	}

	pub fn is_function(&self) -> bool {
		match self {
			TypeRefKind::Function(_) => true,
			TypeRefKind::Typedef(tdef) => tdef.underlying_type_ref().kind().is_function(),
			_ => false,
		}
	}

	pub fn input_output_array_kind(&self) -> Option<InputOutputArrayKind> {
		match self {
			TypeRefKind::Reference(inner) => inner.kind().input_output_array_kind(),
			TypeRefKind::Class(cls) => match cls.cpp_name(CppNameStyle::Reference).as_ref() {
				"cv::_InputArray" => Some(InputOutputArrayKind::Input),
				"cv::_OutputArray" => Some(InputOutputArrayKind::Output),
				"cv::_InputOutputArray" => Some(InputOutputArrayKind::InputOutput),
				_ => None,
			},
			TypeRefKind::Typedef(tdef) => match tdef.cpp_name(CppNameStyle::Reference).as_ref() {
				"cv::InputArray" | "cv::InputArrayOfArrays" => Some(InputOutputArrayKind::Input),
				"cv::OutputArray" | "cv::OutputArrayOfArrays" => Some(InputOutputArrayKind::Output),
				"cv::InputOutputArray" | "cv::InputOutputArrayOfArrays" => Some(InputOutputArrayKind::InputOutput),
				_ => None,
			},
			_ => None,
		}
	}

	pub fn as_abstract_class_ptr(&self) -> Option<(Cow<TypeRef<'tu, 'ge>>, Class<'tu, 'ge>)> {
		if let Some(pointee) = self.as_pointer() {
			if let Some(class) = pointee.kind().as_class().filter(|cls| cls.is_abstract()) {
				let class = class.into_owned();
				return Some((pointee, class));
			}
		}
		None
	}

	pub fn is_copy(&self, type_hint: &TypeRefTypeHint) -> bool {
		match self {
			TypeRefKind::Primitive(_, _) | TypeRefKind::Enum(_) => true,
			TypeRefKind::Class(cls) if cls.kind().is_simple() => true,
			TypeRefKind::Typedef(tdef) => tdef.underlying_type_ref().kind().is_copy(type_hint),
			kind => kind.is_char_ptr_string(type_hint),
		}
	}

	pub fn is_clone(&self, type_hint: &TypeRefTypeHint) -> bool {
		self.is_copy(type_hint)
			|| match self {
				TypeRefKind::StdVector(vec) => vec.element_type().kind().is_clone(type_hint),
				TypeRefKind::Class(cls) => cls.has_explicit_clone() || cls.has_implicit_clone(),
				_ => false,
			}
	}

	/// True if a `TypeRef` has `std::fmt::Debug` implementation
	pub fn is_debug(&self) -> bool {
		match self {
			TypeRefKind::Primitive(..) | TypeRefKind::Class(_) | TypeRefKind::Enum(_) | TypeRefKind::SmartPtr(_) => true,
			TypeRefKind::Array(elem, _) => elem.kind().is_debug(),
			TypeRefKind::StdVector(vec) => vec.element_type().kind().is_debug(),
			TypeRefKind::StdTuple(tuple) => tuple.elements().into_iter().all(|e| e.kind().is_debug()),
			TypeRefKind::Pointer(inner) | TypeRefKind::Reference(inner) | TypeRefKind::RValueReference(inner) => {
				inner.kind().is_debug()
			}
			TypeRefKind::Typedef(tdef) => tdef.underlying_type_ref().kind().is_debug(),
			TypeRefKind::Function(_) | TypeRefKind::Generic(_) | TypeRefKind::Ignored => false,
		}
	}

	/// True for types that get passed in Rust by pointer as opposed to a reference or an owned value
	pub fn is_rust_by_ptr(&self, type_hint: &TypeRefTypeHint) -> bool {
		self.as_pointer().is_some_and(|inner| {
			let inner_kind = inner.kind();
			// todo: support receiving slices for CUDA_RawVideoSourceTrait::get_next_packet
			inner_kind.is_void()
				|| inner_kind.is_unsigned_char()
				|| inner_kind.as_pointer().is_some_and(|inner| inner.kind().is_copy(type_hint))
				|| matches!(type_hint, TypeRefTypeHint::PrimitivePtrAsRaw)
					&& inner_kind.as_primitive().is_some()
					&& !self.as_string(type_hint).is_some()
		})
	}

	/// True for types that can be returned as direct reference to the underlying data that's allocated on C++ side
	pub fn can_return_as_direct_reference(&self) -> bool {
		match self.canonical().as_ref() {
			TypeRefKind::Array(elem, _) => elem.kind().is_copy(elem.type_hint()),
			TypeRefKind::Pointer(inner) => match inner.kind().canonical().as_ref() {
				TypeRefKind::Primitive(_, cpp) => *cpp != "void",
				TypeRefKind::Enum(_) => true,
				TypeRefKind::Class(cls) => cls.kind().is_simple(),
				TypeRefKind::Array(_, _)
				| TypeRefKind::StdVector(_)
				| TypeRefKind::StdTuple(_)
				| TypeRefKind::Pointer(_)
				| TypeRefKind::Reference(_)
				| TypeRefKind::RValueReference(_)
				| TypeRefKind::SmartPtr(_)
				| TypeRefKind::Function(_)
				| TypeRefKind::Typedef(_)
				| TypeRefKind::Generic(_)
				| TypeRefKind::Ignored => false,
			},
			TypeRefKind::Primitive(_, _)
			| TypeRefKind::StdVector(_)
			| TypeRefKind::StdTuple(_)
			| TypeRefKind::Reference(_)
			| TypeRefKind::RValueReference(_)
			| TypeRefKind::SmartPtr(_)
			| TypeRefKind::Class(_)
			| TypeRefKind::Enum(_)
			| TypeRefKind::Function(_)
			| TypeRefKind::Typedef(_)
			| TypeRefKind::Generic(_)
			| TypeRefKind::Ignored => false,
		}
	}

	/// True for types that can be returned via C++ `return`, otherwise they are returned through an argument
	///
	/// Some types especially larger `struct`s are not safe to just `return` over the FFI boundary because compilers don't always
	/// agree on how exactly those should be returned. So only designated simpler and shorter types are returned this way. For
	/// all other cases the return is passed through an additional argument.
	pub fn return_as_naked(&self, type_hint: &TypeRefTypeHint) -> bool {
		match self {
			TypeRefKind::Primitive(..) | TypeRefKind::Pointer(_) => true,
			TypeRefKind::Array(elem, _) => elem.kind().is_copy(elem.type_hint()),
			kind => kind.extern_pass_kind().is_by_void_ptr() || kind.as_string(type_hint).is_some(),
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub enum ExternPassKind {
	AsIs,
	ByPtr,
	/// Value of the type needs to be passed by pointer to a heap-allocated object to and from the C++ side
	ByVoidPtr,
}

impl ExternPassKind {
	pub fn is_by_ptr(&self) -> bool {
		match self {
			ExternPassKind::AsIs => false,
			ExternPassKind::ByPtr | ExternPassKind::ByVoidPtr => true,
		}
	}

	pub fn is_by_void_ptr(&self) -> bool {
		match self {
			ExternPassKind::AsIs | ExternPassKind::ByPtr => false,
			ExternPassKind::ByVoidPtr => true,
		}
	}
}

pub enum InputOutputArrayKind {
	Input,
	Output,
	InputOutput,
}

#[cfg(test)]
mod test {
	use crate::type_ref::{Dir, StrEnc, StrType, TypeRef, TypeRefDesc, TypeRefTypeHint};

	fn as_string(type_ref: TypeRef) -> Option<(Dir, StrType)> {
		type_ref.kind().as_string(type_ref.type_hint())
	}

	#[test]
	fn test_as_string_char_ptr() {
		{
			let char = TypeRefDesc::char();
			assert_eq!(None, as_string(char));
		}

		{
			let char_ptr = TypeRefDesc::char_ptr();
			assert_eq!(Some((Dir::Out, StrType::CharPtr(StrEnc::Text))), as_string(char_ptr));
		}

		{
			let char_ptr_const = TypeRefDesc::char_const_ptr();
			assert_eq!(Some((Dir::In, StrType::CharPtr(StrEnc::Text))), as_string(char_ptr_const));
		}

		{
			let char_ptr_const_slice = TypeRefDesc::char_const_ptr().with_type_hint(TypeRefTypeHint::StringAsBytes(None));
			assert_eq!(
				Some((Dir::In, StrType::CharPtr(StrEnc::Binary))),
				as_string(char_ptr_const_slice)
			);
		}

		{
			let single_char_ptr = TypeRefDesc::char_ptr().with_type_hint(TypeRefTypeHint::CharPtrSingleChar);
			assert_eq!(None, as_string(single_char_ptr));
		}

		{
			let char_array = TypeRef::new_array(TypeRefDesc::char(), None);
			assert_eq!(Some((Dir::Out, StrType::CharPtr(StrEnc::Text))), as_string(char_array));
		}
	}
}
