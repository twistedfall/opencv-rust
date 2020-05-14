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

use crate::{
	Class,
	DefinitionLocation,
	Element,
	Enum,
	Field,
	Function,
	GeneratedElement,
	GeneratorEnv,
	IteratorExt,
	ReturnTypeWrapper,
	settings,
	SmartPtr,
	StringExt,
	Typedef,
	Vector,
};

#[derive(Clone, Copy, Debug)]
pub struct Lifetime {
	number: u8
}

impl Default for Lifetime {
	fn default() -> Self {
		Self { number: 0 }
	}
}

impl Lifetime {
	pub fn next(self) -> Self {
		Self { number: self.number + 1 }
	}
}

impl fmt::Display for Lifetime {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let c = (b'a' + self.number) as char;
		write!(f, "'{}", c)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum DependentTypeMode {
	None,
	ForReturn(DefinitionLocation),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Constness {
	Auto,
	Const,
	Mut,
}

impl Constness {
	pub fn is_const(self, type_ref: &TypeRef) -> bool {
		match self {
			Constness::Auto => type_ref.is_const(),
			Constness::Const => true,
			Constness::Mut => false,
		}
	}

	pub fn is_mut(self, type_ref: &TypeRef) -> bool {
		match self {
			Constness::Auto => !type_ref.is_const(),
			Constness::Const => false,
			Constness::Mut => true,
		}
	}
}

#[derive(Clone, Debug)]
pub enum Kind<'tu, 'g> {
	/// (rust name, cpp name)
	Primitive(&'static str, &'static str),
	/// (element type, array size)
	Array(TypeRef<'tu, 'g>, Option<usize>),
	StdVector(Vector<'tu, 'g>),
	Pointer(TypeRef<'tu, 'g>),
	Reference(TypeRef<'tu, 'g>),
	SmartPtr(SmartPtr<'tu, 'g>),
	Class(Class<'tu, 'g>),
	Enum(Enum<'tu>),
	Function(Function<'tu, 'g>),
	Typedef(Typedef<'tu, 'g>),
	Generic(String),
	Ignored,
}

impl Kind<'_, '_> {
	pub fn is_constified(&self) -> bool {
		match self {
			Kind::Pointer(inner) | Kind::Reference(inner) | Kind::Array(inner, ..) => {
				match inner.kind() {
					// avoid specifying "const" twice for "const Type**"
					Kind::Pointer(..) | Kind::Reference(..) => {
						!inner.is_const()
					}
					_ => {
						true
					}
				}
			}
			Kind::Primitive(..) | Kind::StdVector(..) | Kind::SmartPtr(..)
			| Kind::Class(..) | Kind::Enum(..) | Kind::Function(..)
			| Kind::Typedef(..) | Kind::Generic(..) | Kind::Ignored => {
				false
			}
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TypeRefTypeHint<'tu> {
	None,
	Slice,
	Specialized(Type<'tu>),
}

impl Default for TypeRefTypeHint<'_> {
	fn default() -> Self {
		TypeRefTypeHint::None
	}
}

#[derive(Clone)]
pub struct TypeRef<'tu, 'g> {
	type_ref: Type<'tu>,
	type_hint: TypeRefTypeHint<'tu>,
	parent_entity: Option<Entity<'tu>>,
	gen_env: &'g GeneratorEnv<'tu>,
}

impl<'tu, 'g> TypeRef<'tu, 'g> {
	pub fn new(type_ref: Type<'tu>, gen_env: &'g GeneratorEnv<'tu>) -> Self {
		Self::new_ext(type_ref, Default::default(), None, gen_env)
	}

	pub fn new_ext(type_ref: Type<'tu>, type_hint: TypeRefTypeHint<'tu>, parent_entity: Option<Entity<'tu>>, gen_env: &'g GeneratorEnv<'tu>) -> Self {
		Self { type_ref, type_hint, parent_entity, gen_env }
	}

	fn format_as_array(&self, elem_type: &str, size: Option<usize>) -> String {
		format!(
			"&{cnst}[{typ}{size}]",
			cnst=self.rust_const_qual(false),
			typ=elem_type,
			size=size.map_or_else(|| "".to_string(), |s| format!("; {}", s))
		)
	}

	pub fn specialize(&mut self, typ: Type<'tu>) {
		if self.is_generic() {
			self.type_hint = TypeRefTypeHint::Specialized(typ);
		}
	}

	pub fn clang_type(&self) -> Type<'tu> {
		self.type_ref
	}

	pub fn kind(&self) -> Kind<'tu, 'g> {
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
				} else if self.type_hint == TypeRefTypeHint::Slice {
					Kind::Array(TypeRef::new_ext(pointee, self.type_hint, self.parent_entity, self.gen_env), None)
				} else {
					Kind::Pointer(TypeRef::new_ext(pointee, self.type_hint, self.parent_entity, self.gen_env))
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
							return Kind::Class(Class::new_ext(decl, Some(elaborate_name), self.gen_env))
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
				} else {
					if let TypeRefTypeHint::Specialized(typ) = self.type_hint {
						TypeRef::new_ext(typ, self.type_hint, self.parent_entity, self.gen_env).kind()
					} else {
						let mut generic_type = self.type_ref.get_display_name();
						// workaround for clang6, FunctionPrototype is seen as Unexposed
						if generic_type.contains("(") && generic_type.contains(")") {
							return Kind::Function(
								Function::new(self.type_ref, self.parent_entity.expect("Can't get parent entity in function prototype"), self.gen_env)
							);
						}
						generic_type.replace_in_place("const ", "");
						Kind::Generic(generic_type)
					}
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
				Kind::Function(
					Function::new(self.type_ref, self.parent_entity.expect("Can't get parent entity in function prototype"), self.gen_env)
				)
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
			TypeKind::MemberPointer => {
				Kind::Ignored
			}

			_ => {
				unreachable!("Can't decide kind: {:#?}", self.type_ref)
			}
		}
	}

	/// TypeRef with all of the typedef's traversed
	pub fn canonical(&self) -> TypeRef<'tu, 'g> {
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
	pub fn canonical_clang(&self) -> TypeRef<'tu, 'g> {
		if let TypeRefTypeHint::Specialized(typ) = self.type_hint {
			Self::new_ext(typ.get_canonical_type(), self.type_hint, self.parent_entity, self.gen_env)
		} else {
			Self::new_ext(self.type_ref.get_canonical_type(), self.type_hint, self.parent_entity, self.gen_env)
		}
	}

	/// Like canonical(), but also removes indirection by pointer and reference
	pub fn source(&self) -> TypeRef<'tu, 'g> {
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
	pub fn base(&self) -> TypeRef<'tu, 'g> {
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
				settings::ELEMENT_IGNORE.is_match(self.cpp_full().as_ref())
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

	pub fn as_template(&self) -> Option<Class<'tu, 'g>> {
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

	pub fn is_const(&self) -> bool {
		self.type_ref.is_const_qualified() || match self.kind() {
			Kind::Primitive(..) | Kind::Generic(..) | Kind::Enum(..)
			| Kind::Class(..) | Kind::Function(..) | Kind::Ignored => {
				false
			}
			Kind::Array(elem, ..) => {
				elem.is_const()
			}
			Kind::StdVector(vec) => {
				vec.element_type().is_const()
			}
			Kind::Pointer(inner) | Kind::Reference(inner) => {
				inner.is_const()
			}
			Kind::SmartPtr(ptr) => {
				ptr.pointee().is_const()
			}
			Kind::Typedef(decl) => {
				decl.underlying_type_ref().is_const()
			}
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

	pub fn is_std_string(&self) -> bool {
		match self.canonical().kind() {
			Kind::Class(cls) => {
				let cpp_fullname = cls.cpp_fullname();
				cpp_fullname.starts_with("std::") && cpp_fullname.ends_with("::string")
			}
			Kind::Reference(inner) => {
				inner.is_std_string()
			}
			Kind::Pointer(inner) => {
				inner.is_std_string() && !inner.is_const()
			}
			_ => {
				false
			}
		}
	}

	pub fn is_cv_string(&self) -> bool {
		match self.canonical().kind() {
			Kind::Class(cls) => {
				cls.cpp_fullname() == "cv::String"
			}
			Kind::Reference(inner) => {
				inner.is_cv_string()
			}
			Kind::Pointer(inner) => {
				inner.is_cv_string() && !inner.is_const()
			}
			_ => {
				false
			}
		}
	}

	pub fn is_char_ptr_string(&self) -> bool {
		match self.canonical().kind() {
			Kind::Pointer(inner) => {
				inner.cpp_full() == "char"
			}
			Kind::Array(inner, ..) => {
				inner.cpp_full() == "char"
			}
			_ => {
				false
			}
		}
	}

	pub fn is_string(&self) -> bool {
		self.is_std_string() || self.is_cv_string() || self.is_char_ptr_string()
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

	pub fn is_void_ptr(&self) -> bool {
		if let Kind::Pointer(inner) = self.canonical().kind() {
			inner.is_void()
		} else {
			false
		}
	}

	pub fn is_bool(&self) -> bool {
		if let Kind::Primitive("bool", _) = self.canonical().kind() {
			true
		} else {
			false
		}
	}

	pub fn as_pointer(&self) -> Option<TypeRef<'tu, 'g>> {
		if let Kind::Pointer(out) = self.canonical().kind() {
			Some(out)
		} else {
			None
		}
	}

	pub fn as_reference(&self) -> Option<TypeRef<'tu, 'g>> {
		if let Kind::Reference(out) = self.canonical().kind() {
			Some(out)
		} else {
			None
		}
	}

	pub fn as_smart_ptr(&self) -> Option<SmartPtr<'tu, 'g>> {
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

	pub fn as_class(&self) -> Option<Class<'tu, 'g>> {
		if let Kind::Class(out) = self.canonical().kind() {
			Some(out)
		} else {
			None
		}
	}

	pub fn as_simple_class(&self) -> Option<Class<'tu, 'g>> {
		match self.canonical().kind() {
			Kind::Class(out) if out.is_simple() => {
				Some(out)
			}
			_ => {
				None
			}
		}
	}

	pub fn as_array(&self) -> Option<(TypeRef<'tu, 'g>, Option<usize>)> {
		if let Kind::Array(elem, size) = self.canonical().kind() {
			Some((elem, size))
		} else {
			None
		}
	}

	pub fn as_variable_array(&self) -> Option<TypeRef<'tu, 'g>> {
		if let Some((elem, None)) = self.as_array() {
			Some(elem)
		} else {
			None
		}
	}

	pub fn as_fixed_array(&self) -> Option<(TypeRef<'tu, 'g>, usize)> {
		if let Some((elem, Some(size))) = self.as_array() {
			Some((elem, size))
		} else {
			None
		}
	}

	pub fn as_string_array(&self) -> Option<(TypeRef<'tu, 'g>, Option<usize>)> {
		if let Some((elem, size)) = self.as_array() {
			if elem.is_string() {
				return Some((elem, size))
			}
		}
		None
	}

	pub fn as_vector(&self) -> Option<Vector<'tu, 'g>> {
		if let Kind::StdVector(out) = self.canonical().kind() {
			Some(out)
		} else {
			None
		}
	}

	pub fn as_function(&self) -> Option<Function<'tu, 'g>> {
		match self.canonical().kind() {
			Kind::Function(out) => {
				Some(out)
			}
			_ => {
				None
			}
		}
	}

	pub fn as_typedef(&self) -> Option<Typedef<'tu, 'g>> {
		match self.kind() {
			Kind::Typedef(out) => {
				Some(out)
			}
			_ => {
				None
			}
		}
	}

//	pub fn as_entity(&self) -> Option<Box<dyn EntityElement<'tu> + 'tu>> where 'g: 'tu {
//		match self.kind() {
//			Kind::Primitive(..) | Kind::Array(..) | Kind::Pointer(..)
//			| Kind::Reference(..) => {
//				None
//			}
//			Kind::StdVector(out) => {
//				Some(Box::new(out))
//			}
//			Kind::SmartPtr(out) => {
//				Some(Box::new(out))
//			}
//			Kind::Class(out) => {
//				Some(Box::new(out))
//			}
//			Kind::Enum(out) => {
//				Some(Box::new(out))
//			}
//			Kind::Function => {
//				None
//			}
//			Kind::Typedef(out) => {
//				Some(Box::new(out))
//			}
//			Kind::Generic(..) => {
//				None
//			}
//		}
//	}

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

	fn is_output(&self) -> bool {
		!self.is_const() && (self.as_reference().is_some() || self.as_pointer().map_or(false, |t| t.is_string()))
	}

	pub fn is_data_type(&self) -> bool {
		settings::DATA_TYPES.contains(self.cpp_full().as_ref())
	}

	pub fn rust_const_qual(&self, pointer: bool) -> &'static str {
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

	pub fn cpp_const_qual(&self) -> &'static str {
		if self.is_const() {
			"const "
		} else {
			""
		}
	}

	pub fn template_args(&self) -> Vec<TemplateArg<'tu, 'g>> {
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
		let mut out = String::with_capacity(64);
		let kind = self.kind();
		if kind.is_constified() && self.is_const() {
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
			Kind::Primitive(..) | Kind::Class(..) | Kind::Enum(..)
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

	fn rust_type_ref(&self, full: bool, lifetime: Option<Lifetime>) -> Cow<str> {
		if self.is_string() {
			return if self.is_const() {
				"String" // todo implement receiving const str's
			} else {
				"String"
			}.into();
		}
		match self.kind() {
			Kind::Primitive(rust, _) => {
				rust.into()
			}
			Kind::Array(elem, size) => {
				self.format_as_array(&elem.rust_type_ref(full, lifetime.map(Lifetime::next)), size).into()
			}
			Kind::StdVector(vec) => {
				vec.rust_name(full).into_owned().into()
			}
			Kind::Reference(inner) if (inner.as_simple_class().is_some() || inner.is_enum()) && inner.is_const() => {
				// const references to simple classes are passed by value for performance
				// fixme: it kind of works now, but probably it's not the best idea
				//  because some functions can potentially save the pointer to the value, but it will be destroyed after function call
				inner.rust_type_ref(full, lifetime.map(Lifetime::next)).into_owned().into()
			}
			Kind::Pointer(inner) | Kind::Reference(inner) => {
				if inner.is_void() {
					format!(
						"*{cnst}c_void",
						cnst=self.rust_const_qual(true),
					).into()
				} else {
					let lt = lifetime.map_or_else(|| "".to_string(), |l| format!("{} ", l));
					format!(
						"&{lt}{cnst}{typ}",
						cnst=self.rust_const_qual(false),
						lt=lt,
						typ=inner.rust_type_ref(full, lifetime.map(Lifetime::next))
					).into()
				}
			}
			Kind::SmartPtr(ptr) => {
				ptr.rust_name(full).into_owned().into()
			}
			Kind::Class(cls) => {
				format!(
					"{dyn}{name}{generic}",
					dyn=if full && cls.is_abstract() { "dyn " } else { "" },
					name=cls.rust_name(full),
					generic=self.rust_tpl_decl(full),
				).into()
			}
			Kind::Enum(enm) => {
				enm.rust_name(full).into_owned().into()
			}
			Kind::Typedef(decl) => {
				let mut out: String = decl.rust_name(full).into_owned();
				let lifetime_count = decl.underlying_type_ref().rust_lifetime_count();
				if lifetime_count >= 1 {
					if lifetime_count >= 2 {
						unimplemented!("Support for lifetime count >= 2 is not implemented yet");
					}
					if let Some(lt) = lifetime.map(|x| x.to_string()) {
						out.reserve(lt.len() + 2);
						out.write_fmt(format_args!("<{}>", lt)).expect("Impossible");
					}
				}
				out.into()
			}
			Kind::Generic(name) => {
				name.into()
			}
			Kind::Function(func) => {
				func.rust_name(full).into_owned().into()
			}
			Kind::Ignored => {
				"<ignored>".into()
			}
		}
	}

	pub fn rust_local(&self) -> Cow<str> {
		self.rust_type_ref(false, None)
	}

	pub fn rust_full(&self) -> Cow<str> {
		self.rust_type_ref(true, None)
	}

	pub fn rust_full_with_lifetimes(&self) -> Cow<str> {
		self.rust_type_ref(true, Some(Lifetime::default()))
	}

	pub fn rust_tpl_decl(&self, full: bool) -> String {
		let generic_types = self.template_args();
		if !generic_types.is_empty() {
			let mut constant_suffix = String::new();
			let generic_types = generic_types.iter()
				.filter_map(|t| {
					match t {
						TemplateArg::Typename(type_ref) => {
							Some(type_ref.rust_type_ref(full, None))
						}
						TemplateArg::Constant(literal) => {
							constant_suffix += literal;
							None
						}
						TemplateArg::Unknown => {
							None
						}
					}
				})
				.collect::<Vec<_>>();
			format!("{}<{}>", constant_suffix, generic_types.join(", "))
		} else {
			"".to_string()
		}
	}

	fn rust_lifetime_count(&self) -> usize {
		if self.is_string() {
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
					if !((inner.as_simple_class().is_some() || inner.is_enum()) && inner.is_const()) {
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

	pub fn rust_lifetimes(&self) -> Vec<Lifetime> {
		let mut cur_lifetime = Lifetime::default();
		(0..self.rust_lifetime_count())
			.map(|_| {
				let out = cur_lifetime;
				cur_lifetime = cur_lifetime.next();
				out
			})
			.collect()
	}

	pub fn rust_extern(&self) -> Cow<str> {
		self.rust_extern_with_const(Constness::Auto)
	}

	pub fn rust_extern_with_const(&self, constness: Constness) -> Cow<str> {
		'typ: loop {
			if self.is_string() {
				break 'typ if constness.is_const(self) {
					"*const c_char".into()
				} else if self.is_output() {
					"*mut *mut c_void".into()
				} else {
					"*mut c_char".into()
				};
			}
			if self.is_by_ptr() {
				break 'typ if constness.is_const(self) {
					"*const c_void"
				} else {
					"*mut c_void"
				}.into()
			}
			if let Some(inner) = self.as_pointer().or_else(|| self.as_reference()) {
				let mut out = String::with_capacity(64);
				out.write_fmt(format_args!("*{}", self.rust_const_qual(true))).expect("Impossible");
				if inner.is_void() {
					out += "c_void";
				} else if self.is_string() {
					out += "c_char";
				} else {
					out += &inner.rust_extern_with_const(constness)
				}
				break 'typ out.into();
			}
			if let Some((elem, len)) = self.as_fixed_array() {
				break 'typ format!(
					"*{cnst}[{typ}; {len}]",
					cnst = self.rust_const_qual(true),
					typ = elem.rust_extern_with_const(constness),
					len = len,
				).into();
			}
			if let Some(elem) = self.as_variable_array() {
				break 'typ format!(
					"*{cnst}{typ}",
					cnst = self.rust_const_qual(true),
					typ = elem.rust_extern_with_const(constness),
				).into()
			}
			if let Some(func) = self.as_function() {
				break 'typ func.rust_extern().into_owned().into();
			}
			break 'typ self.rust_full();
		}
	}

	pub fn rust_self_func_decl(&self, is_method_const: bool) -> String {
		if self.is_by_ptr() {
			if is_method_const {
				"&self".to_string()
			} else {
				"&mut self".to_string()
			}
		} else {
			"self".to_string()
		}
	}

	pub fn rust_arg_func_decl(&self, name: &str) -> String {
		let typ = 'decl_type: loop {
			if self.is_string() {
				if self.is_output() {
					break 'decl_type "&mut String".into();
				} else {
					break 'decl_type "&str".into();
				}
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
		let mutable = if self.is_by_ptr() && !self.is_const() && !self.as_pointer().is_some() && !self.as_reference().is_some() {
			"mut "
		} else {
			""
		};
		format!("{mutable}{name}: {typ}", mutable=mutable, name=name, typ=typ)
	}

	pub fn rust_return_func_decl(&self) -> Cow<str> {
		if self.is_by_ptr() {
			self.source().rust_full().into_owned().into()
		} else {
			self.rust_full()
		}
	}

	pub fn rust_return_map(&self, is_safe_context: bool) -> Cow<str> {
		let unsafety_call = if is_safe_context { "unsafe " } else { "" };
		if self.is_string() {
			format!(
				".map(|s| {unsafety_call}{{ crate::templ::receive_string(s as *mut String) }})",
				unsafety_call=unsafety_call,
			).into()
		} else if self.is_by_ptr() {
			format!(
				".map(|ptr| {unsafety_call}{{ {typ}::from_raw(ptr) }})",
				unsafety_call=unsafety_call,
				typ=self.rust_return_func_decl()
			).into()
		} else if !self.is_void_ptr() && (self.as_pointer().is_some() || self.as_fixed_array().is_some()) {
			let ptr_call = if self.is_const() { "ref" } else { "mut" };
			format!(
				".and_then(|x| {unsafety_call}{{ x.as_{ptr_call}() }}.ok_or_else(|| Error::new(core::StsNullPtr, \"Function returned Null pointer\".to_string())))",
				unsafety_call=unsafety_call,
				ptr_call=ptr_call,
			).into()
		} else {
			"".into()
		}
	}

	pub fn rust_arg_pre_call(&self, name: &str, is_function_infallible: bool) -> String {
		if self.is_string() {
			return if self.is_output() {
				format!("string_arg_output_send!(via {name}_via)", name=name)
			} else if is_function_infallible {
				format!("string_arg_infallible!({name})", name=name)
			} else {
				format!("string_arg!({name})", name=name)
			}
		} else if self.is_input_array() {
			return format!("input_array_arg!({name})", name=name);
		} else if self.is_output_array() {
			return format!("output_array_arg!({name})", name=name);
		} else if self.is_input_output_array() {
			return format!("input_output_array_arg!({name})", name=name);
		} else if self.as_string_array().is_some() {
			return if self.is_const() {
				format!("string_array_arg!({name})", name=name)
			} else {
				format!("string_array_arg_mut!({name})", name=name)
			}
		} else if let Some(func) = self.as_function() {
			let args = Field::rust_disambiguate_names(func.arguments()).collect::<Vec<_>>();
			if let Some((userdata_name, _)) = args.iter().find(|(_, f)| f.is_user_data()).cloned() {
				let ret = func.return_type();
				let tramp_args = args.into_iter()
					.map(|(name, a)| a.type_ref().rust_extern_arg_func_decl(&name, Constness::Auto))
					.join(", ");
				let fw_args = Field::rust_disambiguate_names(func.rust_arguments())
					.map(|(name, a)| a.type_ref().rust_extern_arg_func_decl(&name, Constness::Auto))
					.join(", ");
				return format!(
					"callback_arg!({name}_trampoline({tramp_args}) -> {tramp_ret} => {tramp_userdata_arg} in callbacks => {name}({fw_args}) -> {fw_ret})",
					name=name,
					tramp_args=tramp_args,
					tramp_ret=ret.rust_extern(),
					tramp_userdata_arg=userdata_name,
					fw_args=fw_args,
					fw_ret=ret.rust_extern(),
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

	pub fn rust_self_func_call(&self, is_method_const: bool) -> String {
		self.rust_arg_func_call("self", is_method_const)
	}

	pub fn rust_arg_func_call(&self, name: &str, is_const: bool) -> String {
		if self.is_string() {
			return if self.is_output() {
				format!("&mut {name}_via", name=name)
			} else if self.is_const() || is_const {
				format!("{name}.as_ptr()", name=name)
			} else {
				format!("{name}.as_ptr() as _", name=name) // fixme: use as_mut_ptr() when it's stabilized
			}
		}
		if self.as_reference().map_or(false, |inner| (inner.as_simple_class().is_some() || inner.is_enum()) && (inner.is_const() || is_const))
			|| self.as_simple_class().is_some() {
			return format!("&{name}", name=name);
		}
		if self.is_by_ptr() {
			let typ = self.source();
			return if self.is_const() || is_const {
				format!("{name}.as_raw_{rust_safe_id}()", name=name, rust_safe_id=typ.rust_safe_id())
			} else {
				format!("{name}.as_raw_mut_{rust_safe_id}()", name=name, rust_safe_id=typ.rust_safe_id())
			};
		}
		if self.as_variable_array().is_some() {
			return if self.is_const() || is_const {
				format!("{name}.as_ptr()", name=name)
			} else {
				format!("{name}.as_mut_ptr()", name=name)
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
					cnst=self.rust_const_qual(true),
					const_inner=inner.rust_const_qual(true)
				);
			}
		}
		name.to_string()
	}

	pub fn rust_arg_forward(&self, name: &str) -> String {
		name.to_string()
	}

	pub fn rust_extern_self_func_decl(&self, is_method_const: bool) -> String {
		let method_constness = if is_method_const {
			Constness::Const
		} else {
			Constness::Mut
		};
		self.rust_extern_arg_func_decl("instance", method_constness)
	}

	pub fn rust_extern_arg_func_decl(&self, name: &str, constness: Constness) -> String {
		let mut typ = self.rust_extern_with_const(constness);
		if self.as_simple_class().is_some() {
			*typ.to_mut() = format!("*const {}", typ)
		}
		format!("{name}: {typ}", name=name, typ=typ)
	}

	pub fn rust_extern_return(&self) -> Cow<str> {
		if self.is_string() {
			"*mut c_void".into()
		} else {
			self.rust_extern_with_const(Constness::Mut)
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
		if self.is_string() && self.is_output() {
			format!("string_arg_output_receive!(out, {name}_via => {name})", name=name)
		} else {
			"".to_string()
		}
	}

	pub fn dependent_types(&self) -> Vec<Box<dyn GeneratedElement + 'g>> {
		self.dependent_types_with_mode(DependentTypeMode::None)
	}

	pub fn dependent_types_with_mode(&self, mode: DependentTypeMode) -> Vec<Box<dyn GeneratedElement + 'g>> {
		let mut out = vec![];
		match self.source().kind() {
			Kind::StdVector(vec) => {
				out = vec.dependent_types();
				// implement workaround for race when type with std::string gets generated first
				// we only want vector<cv::String> because it's more compatible across OpenCV versions
				if vec.element_type().is_std_string() {
					let tref = TypeRef::new(
						self.gen_env.resolve_type("std::vector<cv::String>").expect("Can't resolve std::vector<cv::String>"),
						self.gen_env,
					);
					out.push(Box::new(tref.as_vector().expect("Not possible unless something is terribly broken")));
				} else {
					out.push(Box::new(vec))
				}
			},
			Kind::SmartPtr(ptr) => {
				out = ptr.dependent_types();
				out.push(Box::new(ptr))
			},
			Kind::Typedef(typedef) => {
				out = typedef.dependent_types();
			}
			_ => {
				if let DependentTypeMode::ForReturn(def_location) = mode {
					if !self.is_generic() && !self.is_void() {
						out.push(Box::new(if self.is_string() {
							let type_ref = TypeRef::new(
								self.gen_env.resolve_type(&self.cpp_extern_return()).expect("Can't resolve string cpp_extern_return()"),
								self.gen_env,
							);
							let def_location = match def_location {
								DefinitionLocation::Type => DefinitionLocation::Custom(self.rust_module().into_owned()),
								dl @ _ => dl
							};
							ReturnTypeWrapper::new(type_ref, self.gen_env, def_location)
						} else {
							ReturnTypeWrapper::new(self.canonical_clang(), self.gen_env, def_location)
						}) as _);
					}
				}
			}
		}
		out
	}

	pub fn cpp_safe_id(&self) -> Cow<str> {
		let mut out: String = self.cpp_local().into_owned();
		out.cleanup_name();
		out.into()
	}

	fn cpp_type_ref(&self, full: bool, name: &str) -> Cow<str> {
		let mut out = String::with_capacity(64);
		let kind = self.kind();
		if kind.is_constified() {
			out.push_str(self.cpp_const_qual());
		}
		let space_name = if name.is_empty() { "".to_string() } else { format!(" {}", name) };
		out.push_str(&match self.kind() {
			Kind::Primitive(_, cpp) => {
				cpp.to_string() + &space_name
			}
			Kind::Array(inner, size) => {
				if let Some(size) = size {
					format!(
						"{typ}(*{name})[{size}]",
						typ=inner.cpp_type_ref(full, ""),
						name=name,
						size=size,
					)
				} else {
					format!(
						"{typ}*{name}",
						typ=inner.cpp_type_ref(full, ""),
						name=space_name,
					)
				}
			}
			Kind::StdVector(vec) => {
				format!(
					"{vec_type}<{elem_type}>{name}",
					vec_type=vec.cpp_name(full),
					elem_type=vec.element_type().cpp_type_ref(full, ""),
					name=space_name,
				)
			}
			Kind::Pointer(inner) | Kind::Reference(inner) => {
				format!("{typ}*{name}", typ=inner.cpp_type_ref(full, ""), name=space_name)
			}
			Kind::SmartPtr(ptr) => {
				format!(
					"{ptr_type}<{inner_type}>{name}",
					ptr_type=ptr.cpp_name(full),
					inner_type=ptr.pointee().cpp_type_ref(full, ""),
					name=space_name,
				)
			}
			Kind::Class(cls) => {
				let mut out: String = cls.cpp_name(full).into_owned();
				if !self.is_std_string() { // fixme prevents emission of std::string<char>
					out += &self.cpp_tpl_decl(full);
				}
				out + &space_name

			}
			Kind::Enum(enm) => {
				enm.cpp_name(full).into_owned() + &space_name
			}
			Kind::Typedef(tdef) => {
				let underlying_type = tdef.underlying_type_ref();
				if underlying_type.as_reference().is_some() { // references can't be used in lvalue position
					underlying_type.cpp_type_ref(full, "").into_owned() + &space_name
				} else {
					tdef.cpp_name(full).into_owned() + &space_name
				}
			}
			Kind::Generic(generic_name) => {
				generic_name + &space_name
			}
			Kind::Function(func) => {
				func.cpp_name(full).into_owned() + &space_name
			}
			Kind::Ignored => {
				"<ignored>".to_string() + &space_name
			}
		});
		out.into()
	}

	pub fn cpp_local(&self) -> Cow<str> {
		self.cpp_local_with_name("")
	}

	pub fn cpp_local_with_name(&self, name: &str) -> Cow<str> {
		self.cpp_type_ref(false, name)
	}

	pub fn cpp_full(&self) -> Cow<str> {
		self.cpp_full_with_name("")
	}

	pub fn cpp_full_with_name(&self, name: &str) -> Cow<str> {
		self.cpp_type_ref(true, name)
	}

	fn cpp_tpl_decl(&self, full: bool) -> String {
		let generic_types = self.template_args();
		if !generic_types.is_empty() {
			let generic_types = generic_types.iter()
				.filter_map(|t| {
					match t {
						TemplateArg::Typename(type_ref) => {
							Some(type_ref.cpp_type_ref(full, ""))
						}
						TemplateArg::Constant(literal) => {
							Some(literal.into())
						}
						TemplateArg::Unknown => {
							None
						}
					}
				})
				.collect::<Vec<_>>();
			format!("<{}>", generic_types.join(", "))
		} else {
			"".to_string()
		}
	}

	pub fn cpp_extern(&self) -> Cow<str> {
		self.cpp_extern_with_name("")
	}

	pub fn cpp_extern_with_name(&self, name: &str) -> Cow<str> {
		let space_name = if name.is_empty() { "".to_string() } else { format!(" {}", name) };
		if self.is_by_ptr() {
			if self.as_pointer().is_some() || self.as_reference().is_some() {
				format!("{typ}{name}", typ=self.cpp_full(), name=space_name).into()
			} else {
				format!("{cnst}{typ}*{name}", cnst=self.cpp_const_qual(), typ=self.cpp_full(), name=space_name).into()
			}
		} else if self.is_string() {
			if self.is_output() {
				format!("{cnst}void*{name}", cnst=self.cpp_const_qual(), name=space_name).into()
			} else {
				format!("{cnst}char*{name}", cnst=self.cpp_const_qual(), name=space_name).into()
			}
		} else {
			self.cpp_full_with_name(name)
		}
	}

	pub fn cpp_self_func_decl(&self, method_is_const: bool) -> String {
		let cnst = if method_is_const {
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
		if self.is_output() && self.is_string() {
			return format!("{typ}* {name}", typ=self.cpp_extern(), name=name);
		}
		if self.as_function().is_some() {
			let mut typ: String = self.cpp_extern().into_owned();
			if typ.contains("(*)") {
				typ.replace_in_place("(*)", &format!("(*{name})", name=name));
				return typ;
			}
		}
		if self.as_simple_class().is_some() {
			return format!("const {typ}* {name}", typ=self.cpp_extern(), name=name)
		}
		self.cpp_extern_with_name(name).into_owned()
	}

	pub fn cpp_arg_pre_call(&self, name: &str) -> String {
		if self.is_output() {
			if self.is_std_string() {
				return format!("std::string {name}_out", name=name);
			} else if self.is_cv_string() {
				return format!("cv::String {name}_out", name=name);
			}
		}
		"".to_string()
	}

	pub fn cpp_arg_func_call<'a>(&self, name: impl Into<Cow<'a, str>>) -> Cow<'a, str> {
		let name = name.into();
		let is_cv_string = self.is_cv_string();
		if is_cv_string || self.is_std_string() {
			return if self.is_output() {
				let ptr = if self.as_pointer().is_some() { "&" } else { "" };
				format!("{ptr}{name}_out", ptr=ptr, name=name).into()
			} else if is_cv_string {
				format!("cv::String({name})", name=name).into()
			} else {
				format!("std::string({name})", name=name).into()
			};
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
	pub fn cpp_extern_return(&self) -> Cow<str> {
		if self.is_string() {
			"void*".into()
		} else if self.is_by_ptr() {
			format!("{typ}*", typ=self.cpp_full()).into()
		} else {
			self.cpp_full()
		}
	}

	pub fn cpp_extern_return_wrapper_full(&self) -> Cow<str> {
		if self.is_void() {
			"Result_void".into()
		} else {
			format!("Result<{ext}>", ext=self.cpp_extern_return()).into()
		}
	}

	pub fn cpp_arg_post_call(&self, name: &str) -> String {
		if self.is_output() && self.is_string() {
			return format!("*{name} = ocvrs_create_string({name}_out.c_str())", name=name);
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
		if self.is_const() {
			props.push("const");
		}
		if self.is_primitive() {
			props.push("primitive");
		}
		if self.is_enum() {
			props.push("enum");
		}
		if self.is_std_string() {
			props.push("std_string");
		}
		if self.is_cv_string() {
			props.push("cv_string");
		}
		if self.is_char_ptr_string() {
			props.push("char_ptr_string");
		}
		if self.is_string() {
			props.push("string");
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
		if self.is_output() {
			props.push("output_ref");
		}
		if self.is_generic() {
			props.push("generic");
		}
		let props = props.join(", ");
		let mut dbg = f.debug_struct("TypeRef");
		dbg
			.field("rust_safe_id", &self.rust_safe_id())
			.field("rust_full", &self.rust_full())
			.field("rust_extern", &self.rust_extern())
			.field("cpp_safe_id", &self.cpp_safe_id())
			.field("cpp_full", &self.cpp_full())
			.field("cpp_extern", &self.cpp_extern())
			.field("props", &props)
			.field("kind", &self.kind())
			.field("type_hint", &self.type_hint)
			.field("template_types", &self.template_args())
			.finish()
	}
}

#[derive(Debug)]
pub enum TemplateArg<'tu, 'g> {
	Unknown,
	Typename(TypeRef<'tu, 'g>),
	Constant(String),
}
