use crate::function::Function;
use crate::smart_ptr::SmartPtr;
use crate::tuple::Tuple;
use crate::type_ref::{Dir, StrEnc, StrType, TypeRef, TypeRefTypeHint};
use crate::vector::Vector;
use crate::{Class, Enum, Typedef};
use clang::TypeKind;

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
			_ => None,
		}
	}

	pub fn as_string(&self, type_hint: &TypeRefTypeHint) -> Option<Dir<StrType>> {
		let mut out = match self {
			TypeRefKind::Class(cls) => cls.string_type().map(Dir::In),
			TypeRefKind::Reference(inner) => inner.as_string().map(|d| d.with_out_dir(inner.inherent_constness().is_mut())),
			TypeRefKind::Pointer(inner) => inner
				.as_string()
				.or_else(|| {
					(!matches!(
						type_hint,
						TypeRefTypeHint::CharPtrSingleChar | TypeRefTypeHint::PrimitivePtrAsRaw
					) && inner.is_char())
					.then_some(Dir::In(StrType::CharPtr(StrEnc::Text)))
				})
				.map(|d| d.with_out_dir(inner.inherent_constness().is_mut())),
			TypeRefKind::Array(inner, ..) => {
				if inner.is_char() {
					Some(Dir::In(StrType::CharPtr(StrEnc::Text)))
				} else {
					None
				}
			}
			_ => None,
		};
		if let Some(dir) = out.as_mut() {
			if matches!(type_hint, TypeRefTypeHint::StringAsBytes(_)) {
				dir.inner_mut().set_encoding(StrEnc::Binary)
			}
		}
		out
	}

	pub fn is_std_string(&self, type_hint: &TypeRefTypeHint) -> bool {
		self
			.as_string(type_hint)
			.map_or(false, |dir| matches!(dir.inner(), StrType::StdString(_)))
	}

	pub fn is_cv_string(&self, type_hint: &TypeRefTypeHint) -> bool {
		self
			.as_string(type_hint)
			.map_or(false, |dir| matches!(dir.inner(), StrType::CvString(_)))
	}

	pub fn is_char_ptr_string(&self, type_hint: &TypeRefTypeHint) -> bool {
		self
			.as_string(type_hint)
			.map_or(false, |dir| matches!(dir.inner(), StrType::CharPtr(_)))
	}
}
