use std::{
	borrow::Cow,
	cmp,
	fmt::{self, Write},
};

use clang::{Entity, EntityKind, Type, TypeKind};
use once_cell::{
	sync::Lazy,
	unsync::Lazy as UnsyncLazy,
};
use regex::{Captures, Regex};

pub use renderer::{
	Constness,
	ConstnessOverride,
	CppExternReturnRenderer,
	CppRenderer,
	FishStyle,
	Lifetime,
	NameStyle,
	RustRenderer,
	TypeRefRenderer,
};

use crate::{
	AbstractRefWrapper,
	Class,
	DefinitionLocation,
	DependentType,
	Element,
	EntityExt,
	Enum,
	Field,
	Function,
	GeneratorEnv,
	IteratorExt,
	ReturnTypeWrapper,
	settings,
	SmartPtr,
	StringExt,
	Typedef,
	Vector,
};
use crate::settings::ArgOverride;

mod renderer;

#[derive(Clone, Debug, PartialEq)]
pub enum DependentTypeMode {
	None,
	ForReturn(DefinitionLocation),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StrType {
	StdString(StrEnc),
	CvString(StrEnc),
	CharPtr,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StrEnc {
	Text,
	/// string with binary data, e.g. can contain 0 byte
	Binary,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dir<T> {
	In(T),
	Out(T),
}

#[derive(Clone, Debug)]
pub enum Kind<'tu, 'ge> {
	/// (rust name, cpp name)
	Primitive(&'static str, &'static str),
	/// (element type, array size)
	Array(TypeRef<'tu, 'ge>, Option<usize>),
	StdVector(Vector<'tu, 'ge>),
	Pointer(TypeRef<'tu, 'ge>),
	Reference(TypeRef<'tu, 'ge>),
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

impl Default for TypeRefTypeHint<'_> {
	fn default() -> Self {
		TypeRefTypeHint::None
	}
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
		Self::new_ext(type_ref, Default::default(), None, gen_env)
	}

	pub fn new_ext(type_ref: Type<'tu>, type_hint: TypeRefTypeHint<'tu>, parent_entity: Option<Entity<'tu>>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self { type_ref, type_hint, parent_entity, gen_env }
	}

	fn format_as_array(&self, elem_type: &str, size: Option<usize>) -> String {
		format!(
			"&{cnst}[{typ}{size}]",
			cnst=self.constness().rust_qual(false),
			typ=elem_type,
			size=size.map_or_else(|| "".to_string(), |s| format!("; {}", s))
		)
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
				} else if matches!(self.type_hint, TypeRefTypeHint::ArgOverride(ArgOverride::Slice | ArgOverride::NullableSlice)) {
					Kind::Array(pointee_typeref, None)
				} else {
					Kind::Pointer(pointee_typeref)
				}
			}

			TypeKind::LValueReference | TypeKind::RValueReference => {
				Kind::Reference(
					TypeRef::new_ext(
						self.type_ref.get_pointee_type().expect("No pointee type for reference"),
						self.type_hint,
						self.parent_entity,
						self.gen_env,
					)
				)
			}

			TypeKind::Elaborated => {
				let out = TypeRef::new_ext(
					self.type_ref.get_elaborated_type().expect("Can't get elaborated type"),
					self.type_hint,
					self.parent_entity,
					self.gen_env,
				).kind();
				if let Kind::Class(..) = &out {
					let mut elaborate_name = self.type_ref.get_display_name();
					elaborate_name.replace_in_place("const ", "");
					if let Some(decl) = self.type_ref.get_declaration() {
						if elaborate_name.starts_with("std::") {
							return Kind::Class(Class::new_ext(decl, elaborate_name, self.gen_env))
						}
					}
				}
				out
			}

			TypeKind::Record | TypeKind::Unexposed => {
				if let Some(decl) = self.type_ref.get_declaration() {
					let cpp_fullname = decl.cpp_fullname();
					let kind = decl.get_kind();
					let is_decl = kind == EntityKind::StructDecl || kind == EntityKind::ClassDecl;
					if cpp_fullname.starts_with("std::") && cpp_fullname.contains("::vector") {
						Kind::StdVector(Vector::new(self.type_ref, self.gen_env))
					} else if is_decl && cpp_fullname.starts_with("cv::Ptr") {
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
						return Kind::Function(
							Function::new(self.type_ref, self.parent_entity.expect("Can't get parent entity in function prototype"), self.gen_env)
						);
					}
					generic_type.replace_in_place("const ", "");
					Kind::Generic(generic_type)
				}
			}

			TypeKind::Typedef => {
				let decl = self.type_ref.get_declaration().expect("Can't get typedef declaration");
				let decl_name = decl.cpp_fullname();
				if let Some(&(rust, cpp)) = settings::PRIMITIVE_TYPEDEFS.get(decl_name.as_ref()) {
					Kind::Primitive(rust, cpp)
				}
				else if decl.is_system() {
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
								non_typedef = Some(Kind::Class(Class::new_ext(child, decl.cpp_fullname().into_owned(), self.gen_env)));
							}
							EntityKind::EnumDecl if child.get_name().is_none() => {
								non_typedef = Some(Kind::Enum(Enum::new_ext(child, decl.cpp_fullname().into_owned())));
							}
							_ => {}
						}
						false
					});
					if let Some(out) = non_typedef {
						return out;
					}
					Kind::Typedef(Typedef::new(decl, self.gen_env))
				}
			}

			TypeKind::Enum => {
				Kind::Enum(
					Enum::new(
						self.type_ref.get_declaration().expect("Can't get enum declaration")
					)
				)
			}

			TypeKind::FunctionPrototype => {
				if let Some(parent) = self.parent_entity {
					Kind::Function(Function::new(self.type_ref, parent, self.gen_env))
				} else {
					Kind::Ignored
				}
			}

			TypeKind::ConstantArray | TypeKind::IncompleteArray => {
				Kind::Array(
					TypeRef::new_ext(
						self.type_ref.get_element_type().expect("Can't get array element type"),
						self.type_hint,
						self.parent_entity,
						self.gen_env,
					),
					self.type_ref.get_size(),
				)
			}

			TypeKind::MemberPointer | TypeKind::DependentSizedArray | TypeKind::Half => {
				Kind::Ignored
			}

			_ => {
				unreachable!("Can't decide kind: {:#?}", self.type_ref)
			}
		}
	}

	/// TypeRef with all of the typedef's traversed
	pub fn canonical(&self) -> TypeRef<'tu, 'ge> {
		match self.kind() {
			Kind::Typedef(tdef) => {
				tdef.underlying_type_ref().canonical()
			}
			_ => {
				self.clone()
			}
		}
	}

	/// performs canonical by calling clang function not taking application logic into account
	pub fn canonical_clang(&self) -> TypeRef<'tu, 'ge> {
		if let TypeRefTypeHint::Specialized(typ) = self.type_hint {
			Self::new_ext(typ.get_canonical_type(), self.type_hint, self.parent_entity, self.gen_env)
		} else {
			Self::new_ext(self.type_ref.get_canonical_type(), self.type_hint, self.parent_entity, self.gen_env)
		}
	}

	/// Like canonical(), but also removes indirection by pointer and reference
	pub fn source(&self) -> TypeRef<'tu, 'ge> {
		let canonical = self.canonical();
		match canonical.kind() {
			Kind::Pointer(inner) | Kind::Reference(inner) => {
				inner.source()
			}
			_ => {
				canonical
			}
		}
	}

	/// Like source(), but digs down to the elements of arrays
	pub fn base(&self) -> TypeRef<'tu, 'ge> {
		let source = self.source();
		match source.kind() {
			Kind::Array(inner, ..) => {
				inner.base()
			}
			Kind::StdVector(vec) => {
				vec.element_type().base()
			}
			Kind::SmartPtr(ptr) => {
				ptr.pointee().base()
			}
			_ => {
				source
			}
		}
	}

	pub fn is_excluded(&self) -> bool {
		self.is_ignored() || match self.source().kind() {
			Kind::Array(elem, ..) => {
				elem.is_excluded()
			}
			Kind::StdVector(vec) => {
				vec.is_excluded()
			}
			Kind::SmartPtr(ptr) => {
				ptr.is_excluded()
			}
			Kind::Class(cls) => {
				cls.is_excluded()
			}
			_ => {
				false
			}
		}
	}

	pub fn is_ignored(&self) -> bool {
		self.is_template() || self.is_generic() || match self.kind() {
			Kind::Array(inner, ..) => {
				inner.is_ignored()
			}
			Kind::StdVector(vec) => {
				vec.is_ignored()
			}
			Kind::Pointer(inner) | Kind::Reference(inner) => {
				inner.is_ignored()
			}
			Kind::SmartPtr(ptr) => {
				ptr.is_ignored()
			}
			Kind::Class(cls) => {
				cls.is_ignored()
			}
			Kind::Typedef(tdef) => {
				tdef.is_ignored()
			}
			Kind::Ignored => {
				true
			}
			_ => {
				settings::ELEMENT_IGNORE.contains(self.cpp_full().as_ref())
			}
		}
	}

	pub fn is_template(&self) -> bool {
		match self.base().kind() {
			Kind::Class(cls) => {
				cls.is_template()
			}
			_ => {
				false
			}
		}
	}

	pub fn as_template(&self) -> Option<Class<'tu, 'ge>> {
		match self.base().kind() {
			Kind::Class(cls) => {
				cls.as_template()
			}
			_ => {
				None
			}
		}
	}

	pub fn is_generic(&self) -> bool {
		match self.base().kind() {
			Kind::Generic(..) => {
				true
			}
			_ => {
				false
			}
		}
	}

	pub fn constness(&self) -> Constness {
		if self.clang_constness().is_const() {
			Constness::Const
		} else {
			match self.kind() {
				Kind::Primitive(..) | Kind::Generic(..) | Kind::Enum(..)
				| Kind::Class(..) | Kind::Function(..) | Kind::Ignored => {
					Constness::Mut
				}
				Kind::Array(elem, ..) => {
					elem.clang_constness()
				}
				Kind::StdVector(vec) => {
					vec.element_type().clang_constness()
				}
				Kind::Pointer(inner) | Kind::Reference(inner) => {
					inner.clang_constness()
				}
				Kind::SmartPtr(ptr) => {
					ptr.pointee().clang_constness()
				}
				Kind::Typedef(decl) => {
					decl.underlying_type_ref().constness()
				}
			}
		}
	}

	pub fn clang_constness(&self) -> Constness {
		if self.type_ref.is_const_qualified() {
			Constness::Const
		} else {
			Constness::Mut
		}
	}

	fn get_const_hint(&self, type_ref: &TypeRef) -> ConstnessOverride {
		let constness = self.constness();
		match constness {
			Constness::Const if type_ref.clang_constness().is_mut() => {
				ConstnessOverride::Yes(constness)
			},
			_ => ConstnessOverride::No,
		}
	}

	pub fn is_primitive(&self) -> bool {
		match self.canonical().kind() {
			Kind::Primitive(..) => {
				true
			}
			_ => {
				false
			}
		}
	}

	pub fn is_enum(&self) -> bool {
		match self.canonical().kind() {
			Kind::Enum(..) => {
				true
			}
			_ => {
				false
			}
		}
	}

	pub fn as_string(&self) -> Option<Dir<StrType>> {
		let class_string_type = |cls: Class| -> Option<StrType> {
			let cpp_fullname = cls.cpp_fullname();
			if cpp_fullname.starts_with("std::") && cpp_fullname.ends_with("::string") {
				Some(StrType::StdString(if matches!(self.type_hint, TypeRefTypeHint::ArgOverride(ArgOverride::StringAsBytes)) {
					StrEnc::Binary
				} else {
					StrEnc::Text
				}))
			} else if cpp_fullname == "cv::String" {
				Some(StrType::CvString(if matches!(self.type_hint, TypeRefTypeHint::ArgOverride(ArgOverride::StringAsBytes)) {
					StrEnc::Binary
				} else {
					StrEnc::Text
				}))
			} else {
				None
			}
		};

		match self.canonical().kind() {
			Kind::Class(cls) => {
				if let Some(typ) = class_string_type(cls) {
					return Some(Dir::In(typ))
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
					}
				} else {
					let inner_cpp_full = inner.cpp_full();
					if inner_cpp_full == "char" || inner_cpp_full == "const char" {
						return if inner.clang_constness().is_const() {
							Some(Dir::In(StrType::CharPtr))
						} else {
							Some(Dir::Out(StrType::CharPtr))
						};
					}
				}
			}
			Kind::Array(inner, ..) => {
				let inner_cpp_full = inner.cpp_full();
				if inner_cpp_full == "char" || inner_cpp_full == "const char" {
					return Some(Dir::In(StrType::CharPtr))
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
			Kind::Reference(inner) => {
				inner.is_input_array()
			}
			Kind::Class(cls) => {
				cls.cpp_fullname() == "cv::_InputArray"
			}
			Kind::Typedef(tdef) => {
				let cpp_fullname = tdef.cpp_fullname();
				cpp_fullname == "cv::InputArray" || cpp_fullname == "cv::InputArrayOfArrays"
			}
			_ => {
				false
			}
		}
	}

	pub fn is_output_array(&self) -> bool {
		match self.kind() {
			Kind::Reference(inner) => {
				inner.is_output_array()
			}
			Kind::Class(cls) => {
				cls.cpp_fullname() == "cv::_OutputArray"
			}
			Kind::Typedef(tdef) => {
				let cpp_fullname = tdef.cpp_fullname();
				cpp_fullname == "cv::OutputArray" || cpp_fullname == "cv::OutputArrayOfArrays"
			}
			_ => {
				false
			}
		}
	}

	pub fn is_input_output_array(&self) -> bool {
		match self.kind() {
			Kind::Reference(inner) => {
				inner.is_input_output_array()
			}
			Kind::Class(cls) => {
				cls.cpp_fullname() == "cv::_InputOutputArray"
			}
			Kind::Typedef(tdef) => {
				let cpp_fullname = tdef.cpp_fullname();
				cpp_fullname == "cv::InputOutputArray" || cpp_fullname == "cv::InputOutputArrayOfArrays"
			}
			_ => {
				false
			}
		}
	}

	pub fn is_void(&self) -> bool {
		if let Kind::Primitive(_, cpp) = self.canonical().kind() {
			cpp == "void"
		} else {
			false
		}
	}

	pub fn is_bool(&self) -> bool {
		matches!(self.canonical().kind(), Kind::Primitive("bool", _))
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
		if let Kind::Reference(out) = self.canonical().kind() {
			Some(out)
		} else {
			None
		}
	}

	pub fn as_smart_ptr(&self) -> Option<SmartPtr<'tu, 'ge>> {
		if let Kind::SmartPtr(out) = self.canonical().kind() {
			Some(out)
		} else {
			None
		}
	}

	pub fn is_copy(&self) -> bool {
		self.is_primitive()
			|| self.is_enum()
			|| self.is_char_ptr_string()
			|| self.canonical().as_simple_class().is_some()
	}

	pub fn is_clone(&self) -> bool {
		self.is_copy() || match self.kind() {
			Kind::StdVector(vec) => {
				vec.element_type().is_clone()
			},
			Kind::Class(cls) => {
				cls.has_clone()
			},
			_ => {
				false
			}
		}
	}

	pub fn is_nullable(&self) -> bool {
		matches!(self.type_hint, TypeRefTypeHint::ArgOverride(ArgOverride::NullableSlice) | TypeRefTypeHint::ArgOverride(ArgOverride::Nullable))
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
			Kind::Class(out) if out.is_simple() => {
				Some(out)
			}
			_ => {
				None
			}
		}
	}

	pub fn as_abstract_class_ptr(&self) -> Option<(TypeRef<'tu, 'ge>, Class<'tu, 'ge>)> {
		if let Some(pointee) = self.as_pointer() {
			if let Some(class) = pointee.as_class() {
				if class.is_abstract() {
					return Some((pointee, class))
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
				return Some((elem, size))
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

	pub fn as_function(&self) -> Option<Function<'tu, 'ge>> {
		match self.canonical().kind() {
			Kind::Function(out) => {
				Some(out)
			}
			_ => {
				None
			}
		}
	}

	pub fn as_typedef(&self) -> Option<Typedef<'tu, 'ge>> {
		match self.kind() {
			Kind::Typedef(out) => {
				Some(out)
			}
			_ => {
				None
			}
		}
	}

	pub fn is_by_ptr(&self) -> bool {
		match self.canonical().kind() {
			Kind::Class(inner) => {
				inner.is_by_ptr()
			}
			Kind::Reference(inner) | Kind::Pointer(inner) => {
				inner.is_by_ptr()
			}
			Kind::SmartPtr(..) | Kind::StdVector(..) => {
				true
			}
			_ => {
				false
			}
		}
	}

	pub fn is_data_type(&self) -> bool {
		settings::DATA_TYPES.contains(self.cpp_full().as_ref())
	}

	pub fn can_pass_by_ptr(&self) -> bool {
		if let Some(inner) = self.as_pointer() {
			if inner.is_primitive() && !self.as_string().is_some() {
				return true;
			}
		}
		false
	}

	pub fn is_pass_by_ptr(&self) -> bool {
		self.is_void_ptr() ||
			matches!(self.type_hint, TypeRefTypeHint::PrimitiveRefAsPointer) && self.can_pass_by_ptr()
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
				static TYPE_EXTRACT: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^.+<\s*(.+?)\s*(?:,\s*(.+?)\s*)?(?:,\s*(.+?)\s*)?(?:,\s*(.+?)\s*)?>$"#).expect("Can't compile static regex"));
				let display_name = self.type_ref.get_display_name();
				let generic_args: UnsyncLazy<Option<Captures>, _> = UnsyncLazy::new(|| TYPE_EXTRACT.captures(&display_name));
				args.into_iter().enumerate()
					.map(|(i, type_ref)| {
						if let Some(type_ref) = type_ref {
							TemplateArg::Typename(TypeRef::new(type_ref, self.gen_env))
						} else {
							if let Some(generic_args) = &*generic_args {
								generic_args.get(i + 1).map(|a| TemplateArg::Constant(a.as_str().to_string()))
							} else {
								None
							}.unwrap_or(TemplateArg::Unknown)
						}
					})
					.collect()
			},
		}
	}

	pub fn rust_safe_id(&self) -> Cow<str> {
		self.rust_safe_id_ext(true)
	}

	pub fn rust_safe_id_ext(&self, add_const: bool) -> Cow<str> {
		let mut out = String::with_capacity(64);
		let kind = self.kind();
		if add_const && self.clang_constness().is_const() {
			out.push_str("const_");
		}
		out.push_str(&match kind {
			Kind::Array(inner, ..) => {
				inner.rust_safe_id().into_owned() + "_X"
			}
			Kind::StdVector(vec) => {
				vec.rust_localalias().into_owned()
			}
			Kind::Pointer(inner) => {
				let mut inner_safe_id: String = inner.rust_safe_id().into_owned();
				if !self.is_by_ptr() {
					inner_safe_id += "_X";
				}
				inner_safe_id
			}
			Kind::Reference(inner) => {
				inner.rust_safe_id().into_owned()
			}
			Kind::SmartPtr(ptr) => {
				ptr.rust_localalias().into_owned()
			}
			Kind::Class(cls) => {
				cls.rust_localname(FishStyle::No).into_owned()
			}
			Kind::Primitive(..) | Kind::Enum(..)
			| Kind::Function(..) | Kind::Typedef(..) | Kind::Generic(..)
			| Kind::Ignored => {
				self.rust_local().into_owned()
			}
		});
		out.cleanup_name();
		out.into()
	}

	pub fn rust_module(&self) -> Cow<str> {
		match self.kind() {
			Kind::Primitive(..) => {
				"core".into()
			}
			Kind::StdVector(vec) => {
				vec.rust_module().into_owned().into()
			}
			Kind::Array(inner, ..) | Kind::Pointer(inner) | Kind::Reference(inner) => {
				inner.rust_module().into_owned().into()
			}
			Kind::SmartPtr(ptr) => {
				ptr.rust_module().into_owned().into()
			}
			Kind::Class(cls) => {
				cls.rust_module().into_owned().into()
			}
			Kind::Enum(enm) => {
				enm.rust_module().into_owned().into()
			}
			Kind::Function(..) => {
				"core".into() // fixme
			}
			Kind::Typedef(tdef) => {
				tdef.rust_module().into_owned().into()
			}
			Kind::Generic(..) | Kind::Ignored => {
				"core".into()
			}
		}
	}

	pub fn rust_name(&self, name_style: NameStyle, lifetime: Lifetime) -> Cow<str> {
		self.render(RustRenderer::new(name_style, lifetime, self.is_pass_by_ptr()))
	}

	pub fn rust_local(&self) -> Cow<str> {
		self.rust_name(NameStyle::Declaration, Lifetime::elided())
	}

	pub fn rust_full(&self) -> Cow<str> {
		self.rust_name(NameStyle::Reference(FishStyle::No), Lifetime::elided())
	}

	pub fn rust_lifetime_count(&self) -> usize {
		if self.as_string().is_some() {
			0
		} else {
			match self.kind() {
				Kind::Pointer(inner) => {
					if inner.is_void() {
						0
					} else {
						1 + inner.rust_lifetime_count()
					}
				}
				Kind::Reference(inner) => {
					if !((inner.as_simple_class().is_some() || inner.is_enum()) && inner.clang_constness().is_const()) {
						1 + inner.rust_lifetime_count()
					} else {
						0
					}
				}
				Kind::Typedef(tdef) => {
					tdef.underlying_type_ref().rust_lifetime_count()
				}
				_ => {
					0
				}
			}
		}
	}

	pub fn rust_lifetimes(&self) -> impl Iterator<Item=Lifetime> {
		Lifetime::explicit().into_iter().take(self.rust_lifetime_count())
	}

	pub fn rust_extern(&self, constness: ConstnessOverride) -> Cow<str> {
		let constness = constness.with(self.constness());
		#[allow(clippy::never_loop)] // fixme use named block when stable
		'typ: loop {
			if let Some(dir) = self.as_string() {
				match dir {
					Dir::In(_) => {
						break 'typ if constness.is_const() {
							"*const c_char".into()
						} else {
							"*mut c_char".into()
						};
					},
					Dir::Out(_) => {
						break 'typ "*mut *mut c_void".into();
					},
				}
			}
			if self.is_by_ptr() {
				break 'typ if constness.is_const() {
					"*const c_void"
				} else {
					"*mut c_void"
				}.into()
			}
			if let Some(inner) = self.as_pointer().or_else(|| self.as_reference()) {
				let mut out = String::with_capacity(64);
				out.write_fmt(format_args!("*{}", self.constness().rust_qual(true))).expect("Impossible");
				if inner.is_void() {
					out += "c_void";
				} else if self.as_string().is_some() {
					out += "c_char";
				} else {
					out += inner.rust_extern(ConstnessOverride::Yes(constness)).as_ref()
				}
				break 'typ out.into();
			}
			if let Some((elem, len)) = self.as_fixed_array() {
				break 'typ format!(
					"*{cnst}[{typ}; {len}]",
					cnst=self.constness().rust_qual(true),
					typ=elem.rust_extern(ConstnessOverride::Yes(constness)),
					len=len,
				).into();
			}
			if let Some(elem) = self.as_variable_array() {
				let typ = if matches!(elem.as_string(), Some(Dir::Out(StrType::CharPtr))) {
					// kind of special casing for cv_startLoop_int__X__int__charXX__int_charXX, without that
					// argv is treated as array of output arguments and it doesn't seem to be meant this way
					format!("*{cnst}c_char", cnst=elem.clang_constness().rust_qual(true)).into()
				} else {
					elem.rust_extern(ConstnessOverride::Yes(constness))
				};
				break 'typ format!(
					"*{cnst}{typ}",
					cnst=self.constness().rust_qual(true),
					typ=typ,
				).into()
			}
			if let Some(func) = self.as_function() {
				break 'typ func.rust_extern().into_owned().into();
			}
			break 'typ self.rust_full();
		}
	}

	pub fn rust_self_func_decl(&self, method_constness: Constness) -> String {
		if self.is_by_ptr() {
			if method_constness.is_const() {
				"&self".to_string()
			} else {
				"&mut self".to_string()
			}
		} else {
			"self".to_string()
		}
	}

	pub fn rust_arg_func_decl(&self, name: &str) -> String {
		#[allow(clippy::never_loop)] // fixme use named block when stable
		let typ = 'decl_type: loop {
			if let Some(dir) = self.as_string() {
				break 'decl_type match dir {
					Dir::In(StrType::StdString(StrEnc::Text) | StrType::CvString(StrEnc::Text) | StrType::CharPtr) => "&str".into(),
					Dir::In(StrType::StdString(StrEnc::Binary) | StrType::CvString(StrEnc::Binary)) => "&[u8]".into(),
					Dir::Out(StrType::StdString(StrEnc::Text) | StrType::CvString(StrEnc::Text) | StrType::CharPtr) => "&mut String".into(),
					Dir::Out(StrType::StdString(StrEnc::Binary) | StrType::CvString(StrEnc::Binary)) => "&mut Vec<u8>".into(),
				};
			} else if self.is_input_array() {
				break 'decl_type "&dyn core::ToInputArray".into();
			} else if self.is_output_array() {
				break 'decl_type "&mut dyn core::ToOutputArray".into();
			} else if self.is_input_output_array() {
				break 'decl_type "&mut dyn core::ToInputOutputArray".into();
			} else if let Some((_, size)) = self.as_string_array() {
				break 'decl_type self.format_as_array("&str", size).into();
			}
			break 'decl_type self.rust_full();
		};
		let mutable = if self.is_by_ptr() && !self.constness().is_const() && !self.as_pointer().is_some() && !self.as_reference().is_some() {
			"mut "
		} else {
			""
		};
		format!("{mutable}{name}: {typ}", mutable=mutable, name=name, typ=typ)
	}

	pub fn rust_return_func_decl(&self, turbo_fish_style: FishStyle, is_static_func: bool) -> Cow<str> {
		if self.as_abstract_class_ptr().is_some() {
			format!(
				"types::AbstractRef{mut_suf}{fish}<{lt}{typ}>",
				mut_suf=if self.constness().is_const() { "" } else { "Mut" },
				fish=turbo_fish_style.rust_qual(),
				lt=if is_static_func { "'static, " } else { "" },
				typ=self.source().rust_name(NameStyle::Reference(turbo_fish_style), Lifetime::elided()),
			).into()
		} else if self.is_by_ptr() {
			self.source().rust_name(NameStyle::Reference(turbo_fish_style), Lifetime::elided()).into_owned().into()
		} else {
			self.rust_name(NameStyle::Reference(turbo_fish_style), Lifetime::elided())
		}
	}

	pub fn rust_return_map(&self, is_safe_context: bool, is_static_func: bool, is_infallible: bool) -> Cow<str> {
		let unsafety_call = if is_safe_context { "unsafe " } else { "" };
		if self.as_string().is_some() || self.is_by_ptr() {
			format!(
				"let ret = {unsafety_call}{{ {typ}::opencv_from_extern(ret) }};",
				unsafety_call=unsafety_call,
				typ=self.rust_return_func_decl(FishStyle::Turbo, is_static_func),
			).into()
		} else if self.as_pointer().map_or(false, |i| !i.is_void()) && !self.is_pass_by_ptr() || self.as_fixed_array().is_some() {
			let ptr_call = if self.constness().is_const() { "ref" } else { "mut" };
			let error_handling = if is_infallible {
				".expect(\"Function returned null pointer\")"
			} else {
				".ok_or_else(|| Error::new(core::StsNullPtr, \"Function returned null pointer\"))?"
			};
			format!(
				"let ret = {unsafety_call}{{ ret.as_{ptr_call}() }}{error_handling};",
				unsafety_call=unsafety_call,
				ptr_call=ptr_call,
				error_handling=error_handling,
			).into()
		} else {
			"".into()
		}
	}

	pub fn rust_arg_pre_call(&self, name: &str, is_function_infallible: bool) -> String {
		if let Some(dir) = self.as_string() {
			return match dir {
				Dir::In(_) => {
					let mut flags = vec![];
					if is_function_infallible {
						flags.push("nofail")
					}
					if self.constness().is_mut() {
						flags.push("mut")
					}
					let mut flags = flags.join(" ");
					if !flags.is_empty() {
						flags.push(' ');
					}
					format!("extern_container_arg!({flags}{name})", flags=flags, name=name)
				},
				Dir::Out(_) => {
					format!("string_arg_output_send!(via {name}_via)", name=name)
				},
			};
		} else if self.is_input_array() {
			return format!("input_array_arg!({name})", name=name);
		} else if self.is_output_array() {
			return format!("output_array_arg!({name})", name=name);
		} else if self.is_input_output_array() {
			return format!("input_output_array_arg!({name})", name=name);
		} else if self.as_string_array().is_some() {
			return if self.constness().is_const() {
				format!("string_array_arg!({name})", name=name)
			} else {
				format!("string_array_arg_mut!({name})", name=name)
			}
		} else if let Some(func) = self.as_function() {
			let args = Field::rust_disambiguate_names(func.arguments()).collect::<Vec<_>>();
			if let Some((userdata_name, _)) = args.iter().find(|(_, f)| f.is_user_data()).cloned() {
				let ret = func.return_type();
				let tramp_args = args.into_iter()
					.map(|(name, a)| a.type_ref().rust_extern_arg_func_decl(&name, ConstnessOverride::No))
					.join(", ");
				let fw_args = Field::rust_disambiguate_names(func.rust_arguments())
					.map(|(name, a)| a.type_ref().rust_extern_arg_func_decl(&name, ConstnessOverride::No))
					.join(", ");
				return format!(
					"callback_arg!({name}_trampoline({tramp_args}) -> {tramp_ret} => {tramp_userdata_arg} in callbacks => {name}({fw_args}) -> {fw_ret})",
					name=name,
					tramp_args=tramp_args,
					tramp_ret=ret.rust_extern(ConstnessOverride::No),
					tramp_userdata_arg=userdata_name,
					fw_args=fw_args,
					fw_ret=ret.rust_extern(ConstnessOverride::No),
				);
			}
		}
		"".to_string()
	}

	pub fn rust_userdata_pre_call(&self, name: &str, callback_name: &str) -> String {
		format!(
			"userdata_arg!({userdata_name} in callbacks => {callback_name})",
			userdata_name=name,
			callback_name=callback_name,
		)
	}

	pub fn rust_self_func_call(&self, method_constness: Constness) -> String {
		self.rust_arg_func_call("self", if method_constness.is_const() { ConstnessOverride::Yes(Constness::Const) } else { ConstnessOverride::No })
	}

	pub fn rust_arg_func_call(&self, name: &str, constness: ConstnessOverride) -> String {
		if let Some(dir) = self.as_string() {
			return match dir {
				Dir::In(_) => if constness.with(self.constness()).is_const() {
					format!("{name}.opencv_as_extern()", name=name)
				} else {
					format!("{name}.opencv_as_extern_mut()", name=name)
				},
				Dir::Out(_) => format!("&mut {name}_via", name=name),
			}
		}
		if self.as_reference().map_or(false, |inner| (inner.as_simple_class().is_some() || inner.is_enum()) && (constness.with(inner.constness()).is_const())) {
			return format!("&{name}", name=name);
		}
		if self.as_simple_class().is_some() {
			return format!("{name}.opencv_as_extern()", name=name);
		}
		if self.is_by_ptr() {
			let typ = self.source();
			return if constness.with(self.constness()).is_const() {
				format!("{name}.as_raw_{rust_safe_id}()", name=name, rust_safe_id=typ.rust_safe_id_ext(false))
			} else {
				format!("{name}.as_raw_mut_{rust_safe_id}()", name=name, rust_safe_id=typ.rust_safe_id_ext(false))
			}
		}
		if self.as_variable_array().is_some() {
			let arr = if constness.with(self.constness()).is_const() {
				format!("{name}.as_ptr()", name=name)
			} else {
				format!("{name}.as_mut_ptr()", name=name)
			};
			return if self.is_nullable() {
				format!(
					"{name}.map_or({null_ptr}, |{name}| {arr})",
					name=name,
					null_ptr=constness.with(self.constness()).rust_null_ptr_full(),
					arr=arr
				)
			} else {
				arr
			}
		}
		if let Some(func) = self.as_function() {
			if func.arguments().into_iter().any(|a| a.is_user_data()) {
				return format!("{name}_trampoline", name=name);
			}
		}
		if let Some(inner) = self.as_pointer() {
			if inner.as_pointer().is_some() { // some special care for double pointers
				return format!(
					"{name} as *{cnst} _ as *{cnst} *{const_inner} _",
					name=name,
					cnst=self.clang_constness().rust_qual(true),
					const_inner=inner.clang_constness().rust_qual(true)
				);
			}
		}
		if self.is_nullable() && (self.as_reference().is_some() || self.as_pointer().is_some()) {
			let arg = if constness.with(self.constness()).is_const() {
				format!("{name} as *const _", name=name)
			} else {
				format!("{name} as *mut _", name=name)
			};	return format!(
				"{name}.map_or({null_ptr}, |{name}| {arg})",
				name=name,
				null_ptr=constness.with(self.constness()).rust_null_ptr_full(),
				arg=arg,
			)
		}
		name.to_string()
	}

	pub fn rust_arg_forward(&self, name: &str) -> String {
		name.to_string()
	}

	pub fn rust_extern_self_func_decl(&self, method_constness: Constness) -> String {
		self.rust_extern_arg_func_decl("instance", ConstnessOverride::Yes(method_constness))
	}

	pub fn rust_extern_arg_func_decl(&self, name: &str, constness: ConstnessOverride) -> String {
		let mut typ = self.rust_extern(constness);
		if self.as_simple_class().is_some() {
			*typ.to_mut() = format!("*const {}", typ)
		}
		format!("{name}: {typ}", name=name, typ=typ)
	}

	pub fn rust_extern_return(&self) -> Cow<str> {
		if self.as_string().is_some() {
			"*mut c_void".into()
		} else {
			self.rust_extern(ConstnessOverride::Yes(Constness::Mut))
		}
	}

	pub fn rust_extern_return_wrapper_full(&self) -> Cow<str> {
		if self.is_void() {
			"Result_void".into()
		} else {
			format!("Result<{ext}>", ext=self.rust_extern_return()).into()
		}
	}

	pub fn rust_arg_post_call(&self, name: &str, _is_function_infallible: bool) -> String {
		match self.as_string() {
			Some(Dir::Out(StrType::StdString(StrEnc::Text) | StrType::CvString(StrEnc::Text) | StrType::CharPtr)) => {
				format!("string_arg_output_receive!({name}_via => {name})", name = name)
			}
			Some(Dir::Out(StrType::StdString(StrEnc::Binary) | StrType::CvString(StrEnc::Binary))) => {
				format!("byte_string_arg_output_receive!({name}_via => {name})", name = name)
			}
			_ => {
				"".to_string()
			}
		}
	}

	pub fn dependent_types(&self, mode: DependentTypeMode) -> Vec<DependentType<'tu, 'ge>> {
		match self.source().kind() {
			Kind::StdVector(vec) => {
				let mut out = vec.dependent_types();
				out.reserve(2);
				if let Some(Dir::In(str_type)) = vec.element_type().as_string() {
					// We need to generate return wrappers for std::vector<cv::String>, but it has several issues:
					// * we can't use get_canonical_type() because it resolves into compiler dependent inner type like
					//   std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char>>
					// * we can't generate both vector<cv::String> and vector<std::string> because for OpenCV 4
					//   cv::String is an typedef to std::string and it would lead to duplicate definition error
					// That's why we try to resolve both types and check if they are the same, if they are we only generate
					// vector<std::string> if not - both.
					let vec_cv_string = self.gen_env.resolve_type("std::vector<cv::String>").expect("Can't resolve std::vector<cv::String>");
					if let DependentTypeMode::ForReturn(def_location) = mode {
						let vec_std_string = self.gen_env.resolve_type("std::vector<std::string>").expect("Can't resolve std::vector<std::string>");
						let vec_type_ref = if vec_cv_string.get_canonical_type() == vec_std_string.get_canonical_type() {
							TypeRef::new(vec_std_string, self.gen_env)
						} else {
							vec.type_ref()
						};
						let const_hint = self.get_const_hint(&vec_type_ref);
						out.push(DependentType::from_return_type_wrapper(ReturnTypeWrapper::new_ext(
							vec_type_ref,
							const_hint,
							def_location,
							self.gen_env,
						)));
					}
					// implement workaround for race when type with std::string gets generated first
					// we only want vector<cv::String> because it's more compatible across OpenCV versions
					if matches!(str_type, StrType::StdString(_)) {
						let tref = TypeRef::new(vec_cv_string, self.gen_env);
						out.push(DependentType::from_vector(tref.as_vector().expect("Not possible unless something is terribly broken")));
					} else {
						out.push(DependentType::from_vector(vec))
					}
				} else {
					if let DependentTypeMode::ForReturn(def_location) = mode {
						let vec_type_ref = vec.type_ref().canonical_clang();
						let const_hint = self.get_const_hint(&vec_type_ref);
						out.push(DependentType::from_return_type_wrapper(ReturnTypeWrapper::new_ext(
							vec_type_ref,
							const_hint,
							def_location,
							self.gen_env,
						)));
					}
					out.push(DependentType::from_vector(vec));
				}
				out
			},
			Kind::SmartPtr(ptr) => {
				let mut out = ptr.dependent_types();
				if let DependentTypeMode::ForReturn(def_location) = mode {
					let ptr_type_ref = ptr.type_ref().canonical_clang();
					let const_hint = self.get_const_hint(&ptr_type_ref);
					out.push(DependentType::from_return_type_wrapper(ReturnTypeWrapper::new_ext(
						ptr_type_ref,
						const_hint,
						def_location,
						self.gen_env,
					)));
				}
				out.push(DependentType::from_smart_ptr(ptr));
				out
			},
			Kind::Typedef(typedef) => {
				typedef.dependent_types()
			}
			_ => {
				let mut out = vec![];
				if let DependentTypeMode::ForReturn(def_location) = mode {
					if !self.is_generic() && !self.is_void() {
						if self.as_string().is_some() {
							let type_ref = TypeRef::new(
								self.gen_env.resolve_type(self.cpp_extern_return(ConstnessOverride::No).as_ref())
									.expect("Can't resolve string cpp_extern_return()"),
								self.gen_env,
							);
							let def_location = match def_location {
								DefinitionLocation::Type => DefinitionLocation::Custom(self.rust_module().into_owned()),
								dl => dl
							};
							out.push(DependentType::from_return_type_wrapper(ReturnTypeWrapper::new(type_ref, def_location, self.gen_env)));
						} else {
							let type_ref = self.canonical_clang();
							let const_hint = self.get_const_hint(&type_ref);
							out.push(DependentType::from_return_type_wrapper(ReturnTypeWrapper::new_ext(
								type_ref,
								const_hint,
								def_location,
								self.gen_env,
							)));
							if self.as_abstract_class_ptr().is_some() {
								out.push(DependentType::from_abstract_ref_wrapper(AbstractRefWrapper::new(self.clone())))
							}
						}
					}
				}
				out
			}
		}
	}

	pub fn cpp_safe_id(&self) -> Cow<str> {
		let mut out: String = self.cpp_local_ext(false).into_owned();
		out.cleanup_name();
		out.into()
	}

	#[inline]
	pub fn render<'a>(&self, renderer: impl TypeRefRenderer<'a>) -> Cow<str> {
		renderer.render(self)
	}

	pub fn cpp_local(&self) -> Cow<str> {
		self.cpp_local_ext(true)
	}

	pub fn cpp_local_ext(&self, extern_types: bool) -> Cow<str> {
		self.render(CppRenderer::new(NameStyle::Declaration, "", extern_types))
	}

	pub fn cpp_full(&self) -> Cow<str> {
		self.cpp_full_ext("", true)
	}

	pub fn cpp_full_ext(&self, name: &str, extern_types: bool) -> Cow<str> {
		self.render(CppRenderer::new(NameStyle::Reference(FishStyle::Turbo), name, extern_types))
	}

	pub fn cpp_extern(&self) -> Cow<str> {
		self.cpp_extern_with_name("")
	}

	pub fn cpp_extern_with_name(&self, name: &str) -> Cow<str> {
		let space_name = if name.is_empty() { "".to_string() } else { format!(" {}", name) };
		if let Some(dir) = self.as_string() {
			match dir {
				Dir::In(_) => format!("{cnst}char*{name}", cnst=self.constness().cpp_qual(), name=space_name).into(),
				Dir::Out(_) => format!("{cnst}void*{name}", cnst=self.constness().cpp_qual(), name=space_name).into(),
			}
		} else if self.is_by_ptr() {
			if self.as_pointer().is_some() || self.as_reference().is_some() {
				format!("{typ}{name}", typ=self.cpp_full(), name=space_name).into()
			} else {
				format!("{typ}*{name}", typ=self.cpp_full(), name=space_name).into()
			}
		} else {
			self.cpp_full_ext(name, true)
		}
	}

	pub fn cpp_self_func_decl(&self, method_constness: Constness) -> String {
		let cnst = if method_constness.is_const() {
			"const "
		} else {
			""
		};
		if self.is_by_ptr() {
			format!("{cnst}{typ}* instance", cnst=cnst, typ=self.cpp_full())
		} else {
			format!("{cnst}{typ} instance", cnst=cnst, typ=self.cpp_full())
		}
	}

	pub fn cpp_arg_func_decl(&self, name: &str) -> String {
		if matches!(self.as_string(), Some(Dir::Out(_))) || self.as_simple_class().is_some() {
			return format!("{typ}* {name}", typ=self.cpp_extern(), name=name);
		}
		self.cpp_extern_with_name(name).into_owned()
	}

	pub fn cpp_arg_pre_call(&self, name: &str) -> String {
		match self.as_string() {
			Some(Dir::Out(StrType::StdString(_))) => {
				format!("std::string {name}_out", name=name)
			}
			Some(Dir::Out(StrType::CvString(_))) => {
				format!("cv::String {name}_out", name=name)
			}
			Some(Dir::Out(StrType::CharPtr)) => {
				format!("char* {name}_out = new char[1024]()", name=name)
			}
			Some(Dir::In(_)) | None => {
				"".to_string()
			}
		}
	}

	pub fn cpp_arg_func_call<'a>(&self, name: impl Into<Cow<'a, str>>) -> Cow<'a, str> {
		let name = name.into();

		match self.as_string() {
			Some(Dir::Out(str_type)) => {
				let ptr = if str_type != StrType::CharPtr && self.as_pointer().is_some() { "&" } else { "" };
				return format!("{ptr}{name}_out", ptr=ptr, name=name).into();
			}
			Some(Dir::In(StrType::StdString(_))) => {
				return format!("std::string({name})", name=name).into();
			},
			Some(Dir::In(StrType::CvString(_))) => {
				return format!("cv::String({name})", name=name).into();
			},
			Some(Dir::In(StrType::CharPtr)) | None => {}
		}
		if self.is_by_ptr() {
			return if self.as_pointer().is_some() {
				name
			} else {
				format!("*{name}", name=name).into()
			};
		}
		if self.as_reference().is_some() || self.as_fixed_array().is_some() || self.as_simple_class().is_some() {
			return format!("*{name}", name=name).into();
		}
		name
	}

	pub fn cpp_extern_return(&self, constness: ConstnessOverride) -> Cow<str> {
		self.render(CppExternReturnRenderer::new(constness))
	}

	pub fn cpp_extern_return_wrapper_full(&self, constness: ConstnessOverride) -> Cow<str> {
		if self.is_void() {
			"Result_void".into()
		} else {
			format!("Result<{ext}>", ext=self.cpp_extern_return(constness)).into()
		}
	}

	pub fn cpp_arg_post_call(&self, name: &str) -> String {
		match self.as_string() {
			Some(Dir::Out(StrType::StdString(StrEnc::Text) | StrType::CvString(StrEnc::Text))) => {
				format!("*{name} = ocvrs_create_string({name}_out.c_str())", name=name)
			}
			Some(Dir::Out(StrType::CharPtr)) => {
				format!("*{name} = ocvrs_create_string({name}_out)", name=name)
			}
			Some(Dir::Out(StrType::StdString(StrEnc::Binary))) => {
				format!("*{name} = ocvrs_create_byte_string({name}_out.data(), {name}_out.size())", name=name)
			}
			Some(Dir::Out(StrType::CvString(StrEnc::Binary))) => {
				format!("*{name} = ocvrs_create_byte_string({name}_out.begin(), {name}_out.size())", name=name)
			}
			Some(Dir::In(_)) | None => {
				"".to_string()
			}
		}
	}

	// we need cleanup as a separate step from post_call because in cv_ocl_convertTypeStr_int_int_int_charX the
	// return value is actually one of the arguments and if we free it (in post_call phase) before converting
	// to string (in return statement) it will result in UB
	pub fn cpp_arg_cleanup(&self, name: &str) -> String {
		if let Some(Dir::Out(StrType::CharPtr)) = self.as_string() {
			return format!("delete[] {name}_out", name=name);
		}
		"".to_string()
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
		if self.is_primitive() {
			props.push("primitive");
		}
		if self.is_enum() {
			props.push("enum");
		}
		if let Some(str_type) = self.as_string() {
			props.push("string");
			let str_type = match str_type {
				Dir::In(str_type) => {
					str_type
				},
				Dir::Out(str_type) => {
					props.push("output_string");
					str_type
				},
			};
			match str_type {
				StrType::StdString(StrEnc::Text) => {
					props.push("std_string");
				},
				StrType::CvString(StrEnc::Text) => {
					props.push("cv_string");
				},
				StrType::CharPtr => {
					props.push("char_ptr_string");
				},
				StrType::StdString(StrEnc::Binary) => {
					props.push("byte_std_string");
				},
				StrType::CvString(StrEnc::Binary) => {
					props.push("byte_cv_string");
				},
			}
		}
		if self.is_void() {
			props.push("void");
		}
		if self.as_pointer().is_some() {
			props.push("pointer");
		}
		if self.as_reference().is_some() {
			props.push("reference");
		}
		if self.is_copy() {
			props.push("copy");
		}
		if self.is_by_ptr() {
			props.push("by_ptr");
		}
		if self.is_generic() {
			props.push("generic");
		}
		let props = props.join(", ");
		let mut dbg = f.debug_struct("TypeRef");
		dbg
			.field("rust_safe_id", &self.rust_safe_id())
			.field("rust_full", &self.rust_full())
			.field("rust_extern", &self.rust_extern(ConstnessOverride::No))
			.field("cpp_safe_id", &self.cpp_safe_id())
			.field("cpp_full", &self.cpp_full())
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
