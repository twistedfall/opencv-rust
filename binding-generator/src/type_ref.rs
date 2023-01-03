use std::borrow::Cow;
use std::{cmp, fmt};

use clang::{Entity, EntityKind, Type, TypeKind};
use once_cell::sync::Lazy;
use once_cell::unsync::Lazy as UnsyncLazy;
use regex::{Captures, Regex};

use crate::entity::WalkAction;
use crate::renderer::{CppExternReturnRenderer, CppRenderer};
use crate::{
	settings::{self, ArgOverride},
	AbstractRefWrapper, Class, Element, EntityExt, Enum, Function, GeneratedType, GeneratorEnv, SmartPtr, StringExt, Tuple,
	Typedef, Vector,
};

pub trait TypeRefRenderer<'a> {
	type Recursed: TypeRefRenderer<'a> + Sized;

	fn render<'t>(self, type_ref: &'t TypeRef) -> Cow<'t, str>;
	fn recurse(&self) -> Self::Recursed;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StrType {
	StdString(StrEnc),
	CvString(StrEnc),
	CharPtr,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StrEnc {
	Text,
	/// string with binary data, e.g. can contain 0 byte
	Binary,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Signedness {
	Unsigned,
	Signed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir<T> {
	In(T),
	Out(T),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExternDir {
	Pure, // used for inner type (e.g. for Point*) and for callbacks
	ToCpp(ConstnessOverride),
	FromCpp,
}

#[derive(Clone, Debug)]
pub enum Kind<'tu, 'ge> {
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

#[derive(Clone, Copy, Debug)]
pub enum TypeRefTypeHint<'tu> {
	None,
	ArgOverride(ArgOverride),
	PrimitiveRefAsPointer,
	Specialized(Type<'tu>),
}

#[derive(Clone)]
pub struct TypeRef<'tu, 'ge> {
	type_ref: Type<'tu>,
	type_hint: TypeRefTypeHint<'tu>,
	parent_entity: Option<Entity<'tu>>,
	gen_env: &'ge GeneratorEnv<'tu>,
}

impl<'tu, 'ge> TypeRef<'tu, 'ge> {
	pub fn new(type_ref: Type<'tu>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self::new_ext(type_ref, TypeRefTypeHint::None, None, gen_env)
	}

	pub fn new_ext(
		type_ref: Type<'tu>,
		type_hint: TypeRefTypeHint<'tu>,
		parent_entity: Option<Entity<'tu>>,
		gen_env: &'ge GeneratorEnv<'tu>,
	) -> Self {
		Self {
			type_ref,
			type_hint,
			parent_entity,
			gen_env,
		}
	}

	pub fn gen_env(&self) -> &'ge GeneratorEnv<'tu> {
		self.gen_env
	}

	pub fn set_type_hint(&mut self, typ: TypeRefTypeHint<'tu>) {
		self.type_hint = typ;
	}

	pub fn clang_type(&self) -> Type<'tu> {
		self.type_ref
	}

	pub fn kind(&self) -> Kind<'tu, 'ge> {
		match self.type_ref.get_kind() {
			TypeKind::Void => Kind::Primitive("()", "void"),
			TypeKind::Bool => Kind::Primitive("bool", "bool"),
			TypeKind::CharS => Kind::Primitive("i8", "char"),
			TypeKind::CharU => Kind::Primitive("u8", "char"),
			TypeKind::SChar => Kind::Primitive("i8", "signed char"),
			TypeKind::UChar => Kind::Primitive("u8", "unsigned char"),
			TypeKind::WChar => Kind::Primitive("u16", "wchar_t"),
			TypeKind::Char16 => Kind::Primitive("u16", "char16_t"),
			TypeKind::Char32 => Kind::Primitive("char", "char32_t"),
			TypeKind::Short => Kind::Primitive("i16", "short"),
			TypeKind::UShort => Kind::Primitive("u16", "unsigned short"),
			TypeKind::Int => Kind::Primitive("i32", "int"),
			TypeKind::UInt => Kind::Primitive("u32", "unsigned int"),
			TypeKind::Long => Kind::Primitive("i32", "long"),
			TypeKind::ULong => Kind::Primitive("u32", "unsigned long"),
			TypeKind::LongLong => Kind::Primitive("i64", "long long"),
			TypeKind::ULongLong => Kind::Primitive("u64", "unsigned long long"),
			TypeKind::Int128 => Kind::Primitive("i128", "__int128_t"),
			TypeKind::UInt128 => Kind::Primitive("u128", "__uint128_t"),
			TypeKind::Float => Kind::Primitive("f32", "float"),
			TypeKind::Double => Kind::Primitive("f64", "double"),

			TypeKind::Pointer => {
				let pointee = self.type_ref.get_pointee_type().expect("No pointee type for pointer");
				let pointee_typeref = TypeRef::new_ext(pointee, self.type_hint, self.parent_entity, self.gen_env);
				if pointee_typeref.as_function().is_some() {
					pointee_typeref.kind()
				} else if matches!(
					self.type_hint,
					TypeRefTypeHint::ArgOverride(ArgOverride::Slice | ArgOverride::NullableSlice)
				) {
					Kind::Array(pointee_typeref, None)
				} else {
					Kind::Pointer(pointee_typeref)
				}
			}

			TypeKind::LValueReference => Kind::Reference(TypeRef::new_ext(
				self.type_ref.get_pointee_type().expect("No pointee type for reference"),
				self.type_hint,
				self.parent_entity,
				self.gen_env,
			)),

			TypeKind::RValueReference => Kind::RValueReference(TypeRef::new_ext(
				self.type_ref.get_pointee_type().expect("No pointee type for reference"),
				self.type_hint,
				self.parent_entity,
				self.gen_env,
			)),

			TypeKind::Elaborated => {
				let out = TypeRef::new_ext(
					self.type_ref.get_elaborated_type().expect("Can't get elaborated type"),
					self.type_hint,
					self.parent_entity,
					self.gen_env,
				)
				.kind();
				if let Kind::Class(..) = &out {
					let mut elaborate_name = self.type_ref.get_display_name();
					elaborate_name.replace_in_place("const ", "");
					if let Some(decl) = self.type_ref.get_declaration() {
						if elaborate_name.starts_with("std::") {
							return Kind::Class(Class::new_ext(decl, elaborate_name, self.gen_env));
						}
					}
				}
				out
			}

			TypeKind::Record | TypeKind::Unexposed => {
				if let Some(decl) = self.type_ref.get_declaration() {
					let cpp_refname = decl.cpp_name(CppNameStyle::Reference);
					let kind = decl.get_kind();
					let is_decl = kind == EntityKind::StructDecl || kind == EntityKind::ClassDecl;
					if cpp_refname.starts_with("std::") && cpp_refname.contains("::vector") {
						Kind::StdVector(Vector::new(self.type_ref, self.gen_env))
					} else if cpp_refname.starts_with("std::") && cpp_refname.contains("::tuple") {
						Kind::StdTuple(Tuple::new(self.type_ref, self.gen_env))
					} else if cpp_refname.starts_with("std::") && cpp_refname.contains("::pair") {
						Kind::StdTuple(Tuple::pair(self.type_ref, self.gen_env))
					} else if is_decl && cpp_refname.starts_with("cv::Ptr") {
						Kind::SmartPtr(SmartPtr::new(decl, self.gen_env))
					} else {
						Kind::Class(Class::new(decl, self.gen_env))
					}
				} else if let TypeRefTypeHint::Specialized(typ) = self.type_hint {
					TypeRef::new_ext(typ, self.type_hint, self.parent_entity, self.gen_env).kind()
				} else {
					let mut generic_type = self.type_ref.get_display_name();
					// workaround for clang6, FunctionPrototype is seen as Unexposed
					if generic_type.contains('(') && generic_type.contains(')') {
						return Kind::Function(Function::new(
							self.type_ref,
							self.parent_entity.expect("Can't get parent entity in function prototype"),
							self.gen_env,
						));
					}
					// uint64_t in gapi module ends here for some reason
					if let Some(&(rust, cpp)) = settings::PRIMITIVE_TYPEDEFS.get(generic_type.as_str()) {
						Kind::Primitive(rust, cpp)
					} else {
						generic_type.replace_in_place("const ", "");
						Kind::Generic(generic_type)
					}
				}
			}

			TypeKind::Typedef => {
				let decl = self.type_ref.get_declaration().expect("Can't get typedef declaration");
				let decl_name = decl.cpp_name(CppNameStyle::Reference);
				if let Some(&(rust, cpp)) = settings::PRIMITIVE_TYPEDEFS.get(decl_name.as_ref()) {
					Kind::Primitive(rust, cpp)
				} else if decl.is_system() {
					if decl_name.starts_with("std::") && decl_name.ends_with("::string") {
						Kind::Class(Class::new(decl, self.gen_env))
					} else {
						Kind::Ignored
					}
				} else {
					let mut non_typedef = None;
					decl.walk_children_while(|child| {
						match child.get_kind() {
							EntityKind::StructDecl if child.get_name().is_none() => {
								non_typedef = Some(Kind::Class(Class::new_ext(
									child,
									decl.cpp_name(CppNameStyle::Reference).into_owned(),
									self.gen_env,
								)));
							}
							EntityKind::EnumDecl if child.get_name().is_none() => {
								non_typedef = Some(Kind::Enum(Enum::new_ext(
									child,
									decl.cpp_name(CppNameStyle::Reference).into_owned(),
								)));
							}
							_ => {}
						}
						WalkAction::Interrupt
					});
					if let Some(out) = non_typedef {
						return out;
					}
					Kind::Typedef(Typedef::new(decl, self.gen_env))
				}
			}

			TypeKind::Enum => Kind::Enum(Enum::new(
				self.type_ref.get_declaration().expect("Can't get enum declaration"),
			)),

			TypeKind::FunctionPrototype => {
				if let Some(parent) = self.parent_entity {
					Kind::Function(Function::new(self.type_ref, parent, self.gen_env))
				} else {
					Kind::Ignored
				}
			}

			TypeKind::ConstantArray | TypeKind::IncompleteArray => Kind::Array(
				TypeRef::new_ext(
					self.type_ref.get_element_type().expect("Can't get array element type"),
					self.type_hint,
					self.parent_entity,
					self.gen_env,
				),
				self.type_ref.get_size(),
			),

			TypeKind::MemberPointer | TypeKind::DependentSizedArray | TypeKind::Half => Kind::Ignored,

			_ => {
				unreachable!("Can't decide kind: {:#?}", self.type_ref)
			}
		}
	}

	/// TypeRef with all of the typedef's traversed
	pub fn canonical(&self) -> TypeRef<'tu, 'ge> {
		match self.kind() {
			Kind::Typedef(tdef) => tdef.underlying_type_ref().canonical(),
			_ => self.clone(),
		}
	}

	/// performs canonical by calling clang function not taking application logic into account
	pub fn canonical_clang(&self) -> TypeRef<'tu, 'ge> {
		if let TypeRefTypeHint::Specialized(typ) = self.type_hint {
			Self::new_ext(typ.get_canonical_type(), self.type_hint, self.parent_entity, self.gen_env)
		} else {
			Self::new_ext(
				self.type_ref.get_canonical_type(),
				self.type_hint,
				self.parent_entity,
				self.gen_env,
			)
		}
	}

	/// Like canonical(), but also removes indirection by pointer and reference
	pub fn source(&self) -> TypeRef<'tu, 'ge> {
		let canonical = self.canonical();
		match canonical.kind() {
			Kind::Pointer(inner) | Kind::Reference(inner) | Kind::RValueReference(inner) => inner.source(),
			_ => canonical,
		}
	}

	/// Like source(), but digs down to the elements of arrays
	pub fn base(&self) -> TypeRef<'tu, 'ge> {
		let source = self.source();
		match source.kind() {
			Kind::Array(inner, ..) => inner.base(),
			Kind::StdVector(vec) => vec.element_type().base(),
			Kind::SmartPtr(ptr) => ptr.pointee().base(),
			_ => source,
		}
	}

	pub fn is_excluded(&self) -> bool {
		self.is_ignored()
			|| match self.source().kind() {
				Kind::Array(elem, ..) => elem.is_excluded(),
				Kind::StdVector(vec) => vec.is_excluded(),
				Kind::SmartPtr(ptr) => ptr.is_excluded(),
				Kind::Class(cls) => cls.is_excluded(),
				_ => false,
			}
	}

	pub fn is_ignored(&self) -> bool {
		self.is_template()
			|| self.is_generic()
			|| match self.kind() {
				Kind::Array(inner, ..) => inner.is_ignored(),
				Kind::StdVector(vec) => vec.is_ignored(),
				Kind::StdTuple(tuple) => tuple.is_ignored(),
				Kind::Pointer(inner) | Kind::Reference(inner) | Kind::RValueReference(inner) => inner.is_ignored(),
				Kind::SmartPtr(ptr) => ptr.is_ignored(),
				Kind::Class(cls) => cls.is_ignored(),
				Kind::Typedef(tdef) => tdef.is_ignored(),
				Kind::Ignored => true,
				_ => settings::ELEMENT_IGNORE.contains(self.cpp_name(CppNameStyle::Reference).as_ref()),
			}
	}

	pub fn is_template(&self) -> bool {
		match self.base().kind() {
			Kind::Class(cls) => cls.is_template(),
			_ => false,
		}
	}

	pub fn as_template_specialization(&self) -> Option<Class<'tu, 'ge>> {
		match self.base().kind() {
			Kind::Class(cls) => cls.as_template_specialization(),
			_ => None,
		}
	}

	pub fn is_generic(&self) -> bool {
		matches!(self.base().kind(), Kind::Generic(..))
	}

	pub fn constness(&self) -> Constness {
		if self.clang_constness().is_const() {
			Constness::Const
		} else {
			match self.kind() {
				Kind::Primitive(..) | Kind::Generic(..) | Kind::Enum(..) | Kind::Class(..) | Kind::Function(..) | Kind::Ignored => {
					Constness::Mut
				}
				Kind::Array(elem, ..) => elem.clang_constness(),
				Kind::StdVector(vec) => vec.element_type().clang_constness(),
				Kind::StdTuple(tuple) => tuple.constness(),
				Kind::Pointer(inner) | Kind::Reference(inner) | Kind::RValueReference(inner) => inner.clang_constness(),
				Kind::SmartPtr(ptr) => ptr.pointee().clang_constness(),
				Kind::Typedef(decl) => decl.underlying_type_ref().constness(),
			}
		}
	}

	pub fn clang_constness(&self) -> Constness {
		Constness::from_is_const(self.type_ref.is_const_qualified())
	}

	pub fn is_primitive(&self) -> bool {
		matches!(self.canonical().kind(), Kind::Primitive(..))
	}

	pub fn is_enum(&self) -> bool {
		matches!(self.canonical().kind(), Kind::Enum(..))
	}

	pub fn as_string(&self) -> Option<Dir<StrType>> {
		let class_string_type = |cls: Class| -> Option<StrType> {
			let cpp_refname = cls.cpp_name(CppNameStyle::Reference);
			if cpp_refname.starts_with("std::") && cpp_refname.ends_with("::string") {
				Some(StrType::StdString(
					if matches!(self.type_hint, TypeRefTypeHint::ArgOverride(ArgOverride::StringAsBytes)) {
						StrEnc::Binary
					} else {
						StrEnc::Text
					},
				))
			} else if cpp_refname == "cv::String" {
				Some(StrType::CvString(
					if matches!(self.type_hint, TypeRefTypeHint::ArgOverride(ArgOverride::StringAsBytes)) {
						StrEnc::Binary
					} else {
						StrEnc::Text
					},
				))
			} else {
				None
			}
		};

		match self.canonical().kind() {
			Kind::Class(cls) => {
				if let Some(typ) = class_string_type(cls) {
					return Some(Dir::In(typ));
				}
			}
			Kind::Reference(inner) => {
				if let Some(typ) = inner.as_class().and_then(class_string_type) {
					return if inner.clang_constness().is_const() {
						Some(Dir::In(typ))
					} else {
						Some(Dir::Out(typ))
					};
				}
			}
			Kind::Pointer(inner) => {
				if let Some(typ) = inner.as_class().and_then(class_string_type) {
					return if inner.clang_constness().is_const() {
						Some(Dir::In(typ))
					} else {
						Some(Dir::Out(typ))
					};
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
			Kind::Array(inner, ..) => {
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
		match self.kind() {
			Kind::Reference(inner) => inner.is_input_array(),
			Kind::Class(cls) => cls.cpp_name(CppNameStyle::Reference) == "cv::_InputArray",
			Kind::Typedef(tdef) => {
				let cpp_refname = tdef.cpp_name(CppNameStyle::Reference);
				cpp_refname == "cv::InputArray" || cpp_refname == "cv::InputArrayOfArrays"
			}
			_ => false,
		}
	}

	pub fn is_output_array(&self) -> bool {
		match self.kind() {
			Kind::Reference(inner) => inner.is_output_array(),
			Kind::Class(cls) => cls.cpp_name(CppNameStyle::Reference) == "cv::_OutputArray",
			Kind::Typedef(tdef) => {
				let cpp_refname = tdef.cpp_name(CppNameStyle::Reference);
				cpp_refname == "cv::OutputArray" || cpp_refname == "cv::OutputArrayOfArrays"
			}
			_ => false,
		}
	}

	pub fn is_input_output_array(&self) -> bool {
		match self.kind() {
			Kind::Reference(inner) => inner.is_input_output_array(),
			Kind::Class(cls) => cls.cpp_name(CppNameStyle::Reference) == "cv::_InputOutputArray",
			Kind::Typedef(tdef) => {
				let cpp_refname = tdef.cpp_name(CppNameStyle::Reference);
				cpp_refname == "cv::InputOutputArray" || cpp_refname == "cv::InputOutputArrayOfArrays"
			}
			_ => false,
		}
	}

	pub fn is_void(&self) -> bool {
		matches!(self.canonical().kind(), Kind::Primitive(_, "void"))
	}

	pub fn is_bool(&self) -> bool {
		matches!(self.canonical().kind(), Kind::Primitive(_, "bool"))
	}

	pub fn is_void_ptr(&self) -> bool {
		self.as_pointer().map_or(false, |inner| inner.is_void())
	}

	pub fn as_pointer(&self) -> Option<TypeRef<'tu, 'ge>> {
		if let Kind::Pointer(out) = self.canonical().kind() {
			Some(out)
		} else {
			None
		}
	}

	pub fn as_reference(&self) -> Option<TypeRef<'tu, 'ge>> {
		if let Kind::Reference(out) | Kind::RValueReference(out) = self.canonical().kind() {
			Some(out)
		} else {
			None
		}
	}

	/// True for types whose values are moved as per C++ function specification
	pub fn is_by_move(&self) -> bool {
		matches!(self.canonical().kind(), Kind::RValueReference(_))
	}

	pub fn is_copy(&self) -> bool {
		self.is_primitive() || self.is_enum() || self.is_char_ptr_string() || self.canonical().as_simple_class().is_some()
	}

	pub fn is_clone(&self) -> bool {
		self.is_copy()
			|| match self.kind() {
				Kind::StdVector(vec) => vec.element_type().is_clone(),
				Kind::Class(cls) => cls.has_clone(),
				_ => false,
			}
	}

	pub fn as_char8(&self) -> Option<Signedness> {
		if matches!(self.type_hint, TypeRefTypeHint::ArgOverride(ArgOverride::Char8AsChar)) {
			match self.type_ref.get_kind() {
				TypeKind::CharS => Some(Signedness::Signed),
				TypeKind::CharU => Some(Signedness::Unsigned),
				_ => None,
			}
		} else {
			None
		}
	}

	pub fn is_nullable(&self) -> bool {
		matches!(
			self.type_hint,
			TypeRefTypeHint::ArgOverride(ArgOverride::NullableSlice) | TypeRefTypeHint::ArgOverride(ArgOverride::Nullable)
		)
	}

	pub fn as_smart_ptr(&self) -> Option<SmartPtr<'tu, 'ge>> {
		if let Kind::SmartPtr(out) = self.canonical().kind() {
			Some(out)
		} else {
			None
		}
	}

	pub fn as_class(&self) -> Option<Class<'tu, 'ge>> {
		if let Kind::Class(out) = self.canonical().kind() {
			Some(out)
		} else {
			None
		}
	}

	pub fn as_simple_class(&self) -> Option<Class<'tu, 'ge>> {
		match self.canonical().kind() {
			Kind::Class(out) if out.is_simple() => Some(out),
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
		if let Kind::Array(elem, size) = self.canonical().kind() {
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
		if let Kind::StdVector(out) = self.canonical().kind() {
			Some(out)
		} else {
			None
		}
	}

	pub fn as_tuple(&self) -> Option<Tuple<'tu, 'ge>> {
		if let Kind::StdTuple(out) = self.canonical().kind() {
			Some(out)
		} else {
			None
		}
	}

	pub fn as_function(&self) -> Option<Function<'tu, 'ge>> {
		match self.canonical().kind() {
			Kind::Function(out) => Some(out),
			_ => None,
		}
	}

	pub fn as_typedef(&self) -> Option<Typedef<'tu, 'ge>> {
		match self.kind() {
			Kind::Typedef(out) => Some(out),
			_ => None,
		}
	}

	/// True if value of the type that need to be passed by pointer to a heap-allocated object to and from the C++ side
	pub fn is_extern_by_ptr(&self) -> bool {
		match self.canonical().kind() {
			Kind::Class(inner) => inner.is_boxed(),
			Kind::Pointer(inner) | Kind::Reference(inner) | Kind::RValueReference(inner) => inner.is_extern_by_ptr(),
			Kind::SmartPtr(_) | Kind::StdVector(_) | Kind::StdTuple(_) => true,
			_ => false,
		}
	}

	pub fn is_data_type(&self) -> bool {
		settings::DATA_TYPES.contains(self.cpp_name(CppNameStyle::Reference).as_ref())
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
		self.is_void_ptr() || matches!(self.type_hint, TypeRefTypeHint::PrimitiveRefAsPointer) && self.can_rust_by_ptr()
	}

	pub fn template_specialization_args(&self) -> Vec<TemplateArg<'tu, 'ge>> {
		match self.type_ref.get_kind() {
			TypeKind::Typedef => {
				vec![]
			}
			_ => {
				let args = self.type_ref.get_template_argument_types().unwrap_or_default();
				// there is no way to extract constant generic arguments (e.g. Vec<double, 3>) via libclang
				// so we have to apply some hacks
				static TYPE_EXTRACT: Lazy<Regex> = Lazy::new(|| {
					Regex::new(r#"^.+<\s*(.+?)\s*(?:,\s*(.+?)\s*)?(?:,\s*(.+?)\s*)?(?:,\s*(.+?)\s*)?>$"#)
						.expect("Can't compile static regex")
				});
				let display_name = self.type_ref.get_display_name();
				let generic_args: UnsyncLazy<Option<Captures>, _> = UnsyncLazy::new(|| TYPE_EXTRACT.captures(&display_name));
				args
					.into_iter()
					.enumerate()
					.map(|(i, type_ref)| {
						if let Some(type_ref) = type_ref {
							TemplateArg::Typename(TypeRef::new(type_ref, self.gen_env))
						} else {
							if let Some(generic_args) = &*generic_args {
								generic_args.get(i + 1).map(|a| TemplateArg::Constant(a.as_str().to_string()))
							} else {
								None
							}
							.unwrap_or(TemplateArg::Unknown)
						}
					})
					.collect()
			}
		}
	}

	pub fn generated_types(&self) -> Vec<GeneratedType<'tu, 'ge>> {
		match self.source().kind() {
			Kind::StdVector(vec) => {
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
						let vec_cv_string = self.gen_env.resolve_typeref("std::vector<cv::String>");
						out.push(GeneratedType::Vector(
							vec_cv_string
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
			Kind::StdTuple(tuple) => vec![GeneratedType::Tuple(tuple)],
			Kind::SmartPtr(ptr) => {
				let mut out = ptr.generated_types();
				out.push(GeneratedType::SmartPtr(ptr));
				out
			}
			Kind::Typedef(typedef) => typedef.generated_types(),
			_ => {
				let mut out = vec![];
				if self.as_abstract_class_ptr().is_some() {
					out.push(GeneratedType::AbstractRefWrapper(AbstractRefWrapper::new(self.clone())))
				}
				out
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
			format!(" {}", name)
		};
		if let Some(dir) = self.as_string() {
			match dir {
				Dir::In(_) => format!("{cnst}char*{name}", cnst = self.constness().cpp_qual(), name = space_name).into(),
				Dir::Out(_) => format!("{cnst}void*{name}", cnst = self.constness().cpp_qual(), name = space_name).into(),
			}
		} else if self.is_extern_by_ptr() {
			if self.as_pointer().is_some() || self.as_reference().is_some() {
				format!("{typ}{name}", typ = self.cpp_name(CppNameStyle::Reference), name = space_name).into()
			} else {
				format!(
					"{typ}*{name}",
					typ = self.cpp_name(CppNameStyle::Reference),
					name = space_name
				)
				.into()
			}
		} else {
			self.cpp_name_ext(CppNameStyle::Reference, name, true)
		}
	}

	pub fn cpp_extern_return(&self, constness: ConstnessOverride) -> Cow<str> {
		self.render(CppExternReturnRenderer::new(constness))
	}

	pub fn cpp_extern_return_fallible(&self, constness: ConstnessOverride) -> Cow<str> {
		if self.is_void() {
			"Result_void".into()
		} else {
			format!("Result<{ext}>", ext = self.cpp_extern_return(constness)).into()
		}
	}
}

impl cmp::PartialEq for TypeRef<'_, '_> {
	fn eq(&self, other: &Self) -> bool {
		self.type_ref == other.type_ref
	}
}

impl fmt::Debug for TypeRef<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut props = vec![];
		if self.is_excluded() {
			props.push("excluded");
		}
		if self.is_ignored() {
			props.push("ignored");
		}
		if self.is_template() {
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
		if self.is_void() {
			props.push("void");
		}
		if self.is_bool() {
			props.push("bool");
		}
		if self.is_void_ptr() {
			props.push("void_ptr");
		}
		if self.as_pointer().is_some() {
			props.push("pointer");
		}
		if self.is_by_move() {
			props.push("by_move");
		}
		if self.as_reference().is_some() {
			props.push("reference");
		}
		if self.is_copy() {
			props.push("copy");
		}
		if self.is_clone() {
			props.push("clone");
		}
		if self.is_extern_by_ptr() {
			props.push("extern_by_ptr");
		}
		let props = props.join(", ");
		let mut dbg = f.debug_struct("TypeRef");
		dbg.field("cpp_safe_id", &self.cpp_safe_id())
			.field("cpp_full", &self.cpp_name(CppNameStyle::Reference))
			.field("cpp_extern", &self.cpp_extern())
			.field("props", &props)
			.field("constness", &self.constness())
			.field("clang_constness", &self.clang_constness())
			.field("kind", &self.kind())
			.field("type_hint", &self.type_hint)
			.field("template_types", &self.template_specialization_args())
			.finish()
	}
}

#[derive(Debug)]
pub enum TemplateArg<'tu, 'ge> {
	Unknown,
	Typename(TypeRef<'tu, 'ge>),
	Constant(String),
}

impl<'tu, 'ge> TemplateArg<'tu, 'ge> {
	pub fn into_typename(self) -> Option<TypeRef<'tu, 'ge>> {
		match self {
			TemplateArg::Typename(t) => Some(t),
			TemplateArg::Unknown | TemplateArg::Constant(_) => None,
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

	pub fn rust_qual(&self) -> &'static str {
		match self {
			FishStyle::No => "",
			FishStyle::Turbo => "::",
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
			Constness::Const => true,
			Constness::Mut => false,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConstnessOverride {
	No,
	Const,
	Mut,
}

impl ConstnessOverride {
	pub fn force(constness: Constness) -> Self {
		match constness {
			Constness::Const => Self::Const,
			Constness::Mut => Self::Mut,
		}
	}

	pub fn with(self, constness: Constness) -> Constness {
		match self {
			ConstnessOverride::No => constness,
			ConstnessOverride::Const => Constness::Const,
			ConstnessOverride::Mut => Constness::Mut,
		}
	}
}
