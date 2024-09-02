use std::borrow::Cow;
use std::fmt;
use std::sync::Arc;

use clang::Type;

use crate::type_ref::TypeRef;
use crate::writer::rust_native::type_ref::Lifetime;

#[derive(Clone, Debug, PartialEq)]
pub enum TypeRefTypeHint {
	None,
	/// This argument should be wrapped in an `Option` on Rust side
	Nullable,
	/// Treat this pointer argument as `Option`al slice
	NullableSlice,
	/// Treat this pointer argument as slice
	Slice,
	/// This argument specified the length of the slice, arguments are (rust_arg_name, divisor)
	LenForSlice(Arc<[String]>, usize),
	/// Treat C++ string as a byte buffer (`Vec<u8>`) instead of an actual string, argument is optional cpp_arg_name of the argument that specifies the buffer byte length
	StringAsBytes(Option<Arc<str>>),
	/// when C++ char needs to be represented as Rust char
	CharAsRustChar,
	/// for the cases when `char *` should not be treated as string, but as a pointer to single char
	CharPtrSingleChar,
	/// render a reference to a primitive type as a raw Rust pointer
	PrimitivePtrAsRaw,
	/// Adds a length to an unsized array
	AddArrayLength(usize),
	/// Return boxed class as a wrapped reference to maintain lifetime connection to the source boxed object
	/// (cpp_name(Declaration), lifetime)
	BoxedAsRef(Constness, &'static str, Lifetime),
	/// Make sure to pass TraitClass as a concrete type, not as a trait object, it's used for property setters as we don't want
	/// to be able to use `BoxedRef` there.
	TraitClassConcrete,
}

impl TypeRefTypeHint {
	pub fn something_or_else(self, f: impl FnOnce() -> Self) -> Self {
		match self {
			Self::None => f(),
			other => other,
		}
	}

	/// Filters current TypeRef type hint to make it suitable for inner type e.g. for the pointee
	///
	/// Useful for example to strip the nullability from the inner type of a pointer
	pub fn recurse(self) -> Self {
		match self {
			Self::Nullable => Self::None,
			Self::NullableSlice => Self::Slice,
			recursable => recursable,
		}
	}

	pub fn nullability(&self) -> Nullability {
		match self {
			TypeRefTypeHint::Nullable | TypeRefTypeHint::NullableSlice => Nullability::Nullable,
			TypeRefTypeHint::None
			| TypeRefTypeHint::Slice
			| TypeRefTypeHint::LenForSlice(_, _)
			| TypeRefTypeHint::StringAsBytes(_)
			| TypeRefTypeHint::CharAsRustChar
			| TypeRefTypeHint::CharPtrSingleChar
			| TypeRefTypeHint::PrimitivePtrAsRaw
			| TypeRefTypeHint::AddArrayLength(_)
			| TypeRefTypeHint::BoxedAsRef(_, _, _)
			| TypeRefTypeHint::TraitClassConcrete => Nullability::NotNullable,
		}
	}

	pub fn as_slice_len(&self) -> Option<(&[String], usize)> {
		if let TypeRefTypeHint::LenForSlice(ptr_arg, len_div) = self {
			Some((ptr_arg, *len_div))
		} else {
			None
		}
	}

	pub fn as_boxed_as_ref(&self) -> Option<(Constness, &'static str, Lifetime)> {
		if let TypeRefTypeHint::BoxedAsRef(constness, cpp_name, lifetime) = self {
			Some((*constness, cpp_name, *lifetime))
		} else {
			None
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub enum NameStyle {
	Declaration,
	Reference(FishStyle),
}

impl NameStyle {
	pub fn decl() -> Self {
		Self::Declaration
	}

	pub fn ref_() -> Self {
		Self::Reference(FishStyle::No)
	}

	pub fn ref_fish() -> Self {
		Self::Reference(FishStyle::Turbo)
	}

	pub fn is_declaration(&self) -> bool {
		match self {
			Self::Declaration => true,
			Self::Reference(..) => false,
		}
	}

	pub fn is_reference(&self) -> bool {
		!self.is_declaration()
	}

	pub fn turbo_fish_style(&self) -> FishStyle {
		match self {
			Self::Reference(fish_style) => *fish_style,
			Self::Declaration => FishStyle::No,
		}
	}

	pub fn rust_turbo_fish_qual(&self) -> &'static str {
		match self {
			Self::Declaration => "",
			Self::Reference(fish) => fish.rust_qual(),
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub enum FishStyle {
	No,
	Turbo,
}

impl FishStyle {
	pub fn from_is_turbo(is_turbo: bool) -> Self {
		if is_turbo {
			Self::Turbo
		} else {
			Self::No
		}
	}

	pub fn is_turbo(&self) -> bool {
		match self {
			FishStyle::No => false,
			FishStyle::Turbo => true,
		}
	}

	/// Returns "::" when turbo fish style is requested, otherwise returns empty string
	pub fn rust_qual(&self) -> &'static str {
		match self {
			FishStyle::No => "",
			FishStyle::Turbo => "::",
		}
	}

	pub fn apply<'s>(&self, rust_name: &'s str) -> Cow<'s, str> {
		match self {
			FishStyle::No => {
				if let Some(fish_idx) = rust_name.find("::<") {
					let mut rust_name = rust_name.to_string();
					rust_name.drain(fish_idx..(fish_idx + FishStyle::Turbo.rust_qual().len()));
					rust_name.into()
				} else {
					rust_name.into()
				}
			}
			FishStyle::Turbo => {
				if let Some((before_bracket, _)) = rust_name.split_once('<') {
					if before_bracket.ends_with("::") {
						rust_name.into()
					} else {
						let mut rust_name = rust_name.to_string();
						rust_name.insert_str(before_bracket.len(), "::");
						rust_name.into()
					}
				} else {
					rust_name.into()
				}
			}
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub enum CppNameStyle {
	Declaration,
	Reference,
}

impl CppNameStyle {
	pub fn is_declaration(&self) -> bool {
		match self {
			Self::Declaration => true,
			Self::Reference => false,
		}
	}

	pub fn is_reference(&self) -> bool {
		!self.is_declaration()
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StrType {
	StdString(StrEnc),
	CvString(StrEnc),
	CharPtr(StrEnc),
}

impl StrType {
	pub fn set_encoding(&mut self, enc: StrEnc) {
		match self {
			StrType::StdString(old_enc) | StrType::CvString(old_enc) | StrType::CharPtr(old_enc) => *old_enc = enc,
		}
	}

	pub fn is_binary(&self) -> bool {
		match self {
			StrType::StdString(StrEnc::Binary) | StrType::CvString(StrEnc::Binary) | StrType::CharPtr(StrEnc::Binary) => true,
			StrType::StdString(StrEnc::Text) | StrType::CvString(StrEnc::Text) | StrType::CharPtr(StrEnc::Text) => false,
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StrEnc {
	Text,
	/// string with binary data, e.g. can contain 0 byte
	Binary,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
	In,
	Out,
}

impl Dir {
	pub fn from_out_dir(is_out_dir: bool) -> Self {
		if is_out_dir {
			Self::Out
		} else {
			Self::In
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub enum ExternDir {
	/// used for inner type (e.g. for Point*) and for callbacks
	Contained,
	ToCpp,
	FromCpp,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
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

	/// Returns `""` or `"mut "`, for usage with Rust references, e.g. `&mut T`
	pub fn rust_qual(self) -> &'static str {
		if self.is_const() {
			""
		} else {
			"mut "
		}
	}

	/// Returns `""` or `"Mut"`, for usage within Rust names, e.g. `BoxRefMut`
	pub fn rust_name_qual(self) -> &'static str {
		if self.is_const() {
			""
		} else {
			"Mut"
		}
	}

	/// Returns `""` or `"_mut"`, for usage within Rust function names, e.g. `as_raw_mut`
	pub fn rust_function_name_qual(self) -> &'static str {
		if self.is_const() {
			""
		} else {
			"_mut"
		}
	}

	/// Returns `"const "` or `"mut "`, for usage with Rust pointers, e.g. `*const T`
	pub fn rust_qual_ptr(self) -> &'static str {
		if self.is_const() {
			"const "
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

	/// Returns `"const "` or `""` for usage in C++ code
	pub fn cpp_qual(self) -> &'static str {
		if self.is_const() {
			"const "
		} else {
			""
		}
	}
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Nullability {
	NotNullable,
	Nullable,
}

impl Nullability {
	pub fn is_nullable(self) -> bool {
		match self {
			Self::NotNullable => false,
			Self::Nullable => true,
		}
	}
}

#[derive(Clone, Debug)]
pub enum TemplateArg<'tu, 'ge> {
	Unknown,
	Typename(TypeRef<'tu, 'ge>),
	Constant(String),
}

impl<'tu, 'ge> TemplateArg<'tu, 'ge> {
	pub fn into_typename(self) -> Option<TypeRef<'tu, 'ge>> {
		match self {
			Self::Typename(t) => Some(t),
			Self::Unknown | Self::Constant(_) => None,
		}
	}

	pub fn as_typename(&self) -> Option<&TypeRef<'tu, 'ge>> {
		match self {
			Self::Typename(t) => Some(t),
			Self::Unknown | Self::Constant(_) => None,
		}
	}
}

#[allow(unused)]
pub fn dbg_clang_type(type_ref: Type) {
	struct TypeWrapper<'tu>(Type<'tu>);

	impl fmt::Debug for TypeWrapper<'_> {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			f.debug_struct("Type")
				.field("kind", &self.0.get_kind())
				.field("display_name", &self.0.get_display_name())
				.field("alignof", &self.0.get_alignof())
				.field("sizeof", &self.0.get_sizeof())
				.field("address_space", &self.0.get_address_space())
				.field("argument_types", &self.0.get_argument_types())
				.field("calling_convention", &self.0.get_calling_convention())
				.field("canonical_type", &self.0.get_canonical_type())
				.field("class_type", &self.0.get_class_type())
				.field("declaration", &self.0.get_declaration())
				.field("elaborated_type", &self.0.get_elaborated_type())
				.field("element_type", &self.0.get_element_type())
				.field("exception_specification", &self.0.get_exception_specification())
				.field("fields", &self.0.get_fields())
//				.field("modified_type", &self.0.get_modified_type())
//				.field("nullability", &self.0.get_nullability())
				.field("pointee_type", &self.0.get_pointee_type())
				.field("ref_qualifier", &self.0.get_ref_qualifier())
				.field("result_type", &self.0.get_result_type())
				.field("size", &self.0.get_size())
				.field("template_argument_types", &self.0.get_template_argument_types())
				.field("typedef_name", &self.0.get_typedef_name())
				.field("is_const_qualified", &self.0.is_const_qualified())
				.field("is_elaborated", &self.0.is_elaborated())
				.field("is_pod", &self.0.is_pod())
				.field("is_restrict_qualified", &self.0.is_restrict_qualified())
				.field("is_transparent_tag", &self.0.is_transparent_tag())
				.field("is_variadic", &self.0.is_variadic())
				.field("is_volatile_qualified", &self.0.is_volatile_qualified())
				.field("is_integer", &self.0.is_integer())
				.field("is_signed_integer", &self.0.is_signed_integer())
				.field("is_unsigned_integer", &self.0.is_unsigned_integer())
				.finish()
		}
	}
	eprintln!("{:#?}", TypeWrapper(type_ref));
}
