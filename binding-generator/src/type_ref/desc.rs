use std::rc::Rc;

use clang::{Entity, EntityKind, Type, TypeKind};
use once_cell::sync::Lazy;
use once_cell::unsync::Lazy as UnsyncLazy;
use regex::{Captures, Regex};

use crate::class::ClassDesc;
use crate::function::Function;
use crate::settings::ArgOverride;
use crate::smart_ptr::SmartPtr;
use crate::tuple::Tuple;
use crate::type_ref::{Constness, TemplateArg, TypeRef, TypeRefKind, TypeRefTemplateArg, TypeRefTypeHint};
use crate::typedef::NewTypedefResult;
use crate::vector::{Vector, VectorDesc};
use crate::{settings, Class, CppNameStyle, Element, Enum, GeneratorEnv, StringExt, Typedef};

#[derive(Clone, Debug)]
pub struct TypeRefDesc<'tu, 'ge> {
	pub kind: TypeRefKind<'tu, 'ge>,
	pub inherent_constness: Constness,
	pub type_hint: TypeRefTypeHint<'tu, 'ge>,
	pub template_specialization_args: Rc<[TypeRefTemplateArg<TypeRef<'tu, 'ge>>]>,
}

impl<'tu, 'ge> TypeRefDesc<'tu, 'ge> {
	pub fn new(kind: TypeRefKind<'tu, 'ge>) -> Self {
		Self {
			kind,
			inherent_constness: Constness::Mut,
			type_hint: TypeRefTypeHint::None,
			template_specialization_args: Rc::new([]),
		}
	}

	pub fn void() -> TypeRef<'tu, 'ge> {
		TypeRef::new_desc(TypeRefDesc::new(TypeRefKind::Primitive("()", "void")))
	}

	pub fn bool() -> TypeRef<'tu, 'ge> {
		TypeRef::new_desc(TypeRefDesc::new(TypeRefKind::Primitive("bool", "bool")))
	}

	pub fn double() -> TypeRef<'tu, 'ge> {
		TypeRef::new_desc(TypeRefDesc::new(TypeRefKind::Primitive("f64", "double")))
	}

	pub fn int() -> TypeRef<'tu, 'ge> {
		TypeRef::new_desc(TypeRefDesc::new(TypeRefKind::Primitive("i32", "int")))
	}

	pub fn int64_t() -> TypeRef<'tu, 'ge> {
		TypeRef::new_desc(TypeRefDesc::new(TypeRefKind::Primitive("i64", "int64_t")))
	}

	pub fn size_t() -> TypeRef<'tu, 'ge> {
		TypeRef::new_desc(TypeRefDesc::new(TypeRefKind::Primitive("size_t", "size_t")))
	}

	pub fn input_array() -> TypeRef<'tu, 'ge> {
		TypeRef::new_desc(TypeRefDesc::new(TypeRefKind::Class(Class::new_desc(ClassDesc::boxed(
			"cv::_InputArray",
			"core::_InputArray",
		)))))
	}

	pub fn output_array() -> TypeRef<'tu, 'ge> {
		TypeRef::new_desc(TypeRefDesc::new(TypeRefKind::Class(Class::new_desc(ClassDesc::boxed(
			"cv::_OutputArray",
			"core::_OutputArray",
		)))))
	}

	pub fn input_output_array() -> TypeRef<'tu, 'ge> {
		TypeRef::new_desc(TypeRefDesc::new(TypeRefKind::Class(Class::new_desc(ClassDesc::boxed(
			"cv::_InputOutputArray",
			"core::_InputOutputArray",
		)))))
	}

	pub fn string() -> TypeRef<'tu, 'ge> {
		TypeRef::new_desc(TypeRefDesc::new(TypeRefKind::Class(Class::new_desc(ClassDesc::boxed(
			"cv::String",
			"core::String",
		)))))
	}

	pub fn vector_of_string() -> TypeRef<'tu, 'ge> {
		TypeRef::new_desc(TypeRefDesc::new(TypeRefKind::StdVector(Vector::new_desc(VectorDesc::new(
			TypeRefDesc::string(),
		)))))
	}

	pub fn dict_value() -> TypeRef<'tu, 'ge> {
		TypeRef::new_desc(TypeRefDesc::new(TypeRefKind::Class(Class::new_desc(ClassDesc::boxed(
			"cv::dnn::DictValue",
			"crate::dnn::DictValue",
		)))))
	}
}

pub trait ClangTypeExt<'tu> {
	fn kind<'ge>(
		self,
		type_hint: TypeRefTypeHint<'tu, 'ge>,
		parent_entity: Option<Entity<'tu>>,
		gen_env: &'ge GeneratorEnv<'tu>,
	) -> TypeRefKind<'tu, 'ge>;

	fn template_specialization_args<'ge>(self, gen_env: &'ge GeneratorEnv<'tu>) -> Vec<TemplateArg<'tu, 'ge>>;
}

impl<'tu> ClangTypeExt<'tu> for Type<'tu> {
	fn kind<'ge>(
		self,
		type_hint: TypeRefTypeHint<'tu, 'ge>,
		parent_entity: Option<Entity<'tu>>,
		gen_env: &'ge GeneratorEnv<'tu>,
	) -> TypeRefKind<'tu, 'ge> {
		match self.get_kind() {
			TypeKind::Void => TypeRefKind::Primitive("()", "void"),
			TypeKind::Bool => TypeRefKind::Primitive("bool", "bool"),
			TypeKind::CharS => TypeRefKind::Primitive("i8", "char"),
			TypeKind::CharU => TypeRefKind::Primitive("u8", "char"),
			TypeKind::SChar => TypeRefKind::Primitive("i8", "signed char"),
			TypeKind::UChar => TypeRefKind::Primitive("u8", "unsigned char"),
			TypeKind::WChar => TypeRefKind::Primitive("u16", "wchar_t"),
			TypeKind::Char16 => TypeRefKind::Primitive("u16", "char16_t"),
			TypeKind::Char32 => TypeRefKind::Primitive("char", "char32_t"),
			TypeKind::Short => TypeRefKind::Primitive("i16", "short"),
			TypeKind::UShort => TypeRefKind::Primitive("u16", "unsigned short"),
			TypeKind::Int => TypeRefKind::Primitive("i32", "int"),
			TypeKind::UInt => TypeRefKind::Primitive("u32", "unsigned int"),
			TypeKind::Long => TypeRefKind::Primitive("i32", "long"),
			TypeKind::ULong => TypeRefKind::Primitive("u32", "unsigned long"),
			TypeKind::LongLong => TypeRefKind::Primitive("i64", "long long"),
			TypeKind::ULongLong => TypeRefKind::Primitive("u64", "unsigned long long"),
			TypeKind::Int128 => TypeRefKind::Primitive("i128", "__int128_t"),
			TypeKind::UInt128 => TypeRefKind::Primitive("u128", "__uint128_t"),
			TypeKind::Float => TypeRefKind::Primitive("f32", "float"),
			TypeKind::Double => TypeRefKind::Primitive("f64", "double"),

			TypeKind::Pointer => {
				let pointee = self.get_pointee_type().expect("No pointee type for pointer");
				let pointee_typeref = TypeRef::new_ext(pointee, type_hint.clone(), parent_entity, gen_env);
				if pointee_typeref.as_function().is_some() {
					pointee_typeref.kind().into_owned()
				} else if matches!(
					type_hint,
					TypeRefTypeHint::ArgOverride(ArgOverride::Slice | ArgOverride::NullableSlice)
				) {
					TypeRefKind::Array(pointee_typeref, None)
				} else {
					TypeRefKind::Pointer(pointee_typeref)
				}
			}

			TypeKind::LValueReference => TypeRefKind::Reference(TypeRef::new_ext(
				self.get_pointee_type().expect("No pointee type for reference"),
				type_hint,
				parent_entity,
				gen_env,
			)),

			TypeKind::RValueReference => TypeRefKind::RValueReference(TypeRef::new_ext(
				self.get_pointee_type().expect("No pointee type for reference"),
				type_hint,
				parent_entity,
				gen_env,
			)),

			TypeKind::Elaborated => {
				let out = self
					.get_elaborated_type()
					.expect("Can't get elaborated type")
					.kind(type_hint, parent_entity, gen_env);
				if matches!(out, TypeRefKind::Class(..)) {
					let mut elaborate_name = self.get_display_name();
					elaborate_name.replace_in_place("const ", "");
					if let Some(decl) = self.get_declaration() {
						if elaborate_name.starts_with("std::") {
							return TypeRefKind::Class(Class::new_ext(decl, elaborate_name, gen_env));
						}
					}
				}
				out
			}

			TypeKind::Record | TypeKind::Unexposed => {
				if let Some(decl) = self.get_declaration() {
					let cpp_refname = decl.cpp_name(CppNameStyle::Reference);
					let kind = decl.get_kind();
					let is_decl = kind == EntityKind::StructDecl || kind == EntityKind::ClassDecl;
					if cpp_refname.starts_with("std::") && cpp_refname.contains("::vector") {
						TypeRefKind::StdVector(Vector::new(self, gen_env))
					} else if cpp_refname.starts_with("std::") && cpp_refname.contains("::tuple") {
						TypeRefKind::StdTuple(Tuple::new(self, gen_env))
					} else if cpp_refname.starts_with("std::") && cpp_refname.contains("::pair") {
						TypeRefKind::StdTuple(Tuple::pair(self, gen_env))
					} else if is_decl && cpp_refname.starts_with("cv::Ptr") {
						TypeRefKind::SmartPtr(SmartPtr::new(decl, gen_env))
					} else {
						TypeRefKind::Class(Class::new(decl, gen_env))
					}
				} else if let TypeRefTypeHint::Specialized(typ) = type_hint {
					typ.kind().into_owned()
				} else {
					let mut generic_type = self.get_display_name();
					// workaround for clang6, FunctionPrototype is seen as Unexposed
					if generic_type.contains('(') && generic_type.contains(')') {
						return TypeRefKind::Function(Function::new(
							self,
							parent_entity.expect("Can't get parent entity in function prototype"),
							gen_env,
						));
					}
					// uint64_t in gapi module ends here for some reason
					if let Some(&(rust, cpp)) = settings::PRIMITIVE_TYPEDEFS.get(generic_type.as_str()) {
						TypeRefKind::Primitive(rust, cpp)
					} else {
						generic_type.replace_in_place("const ", "");
						TypeRefKind::Generic(generic_type)
					}
				}
			}

			TypeKind::Typedef => {
				let decl = self.get_declaration().expect("Can't get typedef declaration");
				let decl_name = decl.cpp_name(CppNameStyle::Reference);
				if let Some(&(rust, cpp)) = settings::PRIMITIVE_TYPEDEFS.get(decl_name.as_ref()) {
					TypeRefKind::Primitive(rust, cpp)
				} else if decl.is_system() {
					if decl_name.starts_with("std::") && decl_name.ends_with("::string") {
						TypeRefKind::Class(Class::new(decl, gen_env))
					} else {
						TypeRefKind::Ignored
					}
				} else {
					match Typedef::try_new(decl, gen_env) {
						NewTypedefResult::Typedef(tdef) => TypeRefKind::Typedef(tdef),
						NewTypedefResult::Class(cls) => TypeRefKind::Class(cls),
						NewTypedefResult::Enum(enm) => TypeRefKind::Enum(enm),
					}
				}
			}

			TypeKind::Enum => TypeRefKind::Enum(Enum::new(self.get_declaration().expect("Can't get enum declaration"))),

			TypeKind::FunctionPrototype => {
				if let Some(parent) = parent_entity {
					TypeRefKind::Function(Function::new(self, parent, gen_env))
				} else {
					TypeRefKind::Ignored
				}
			}

			TypeKind::ConstantArray | TypeKind::IncompleteArray => TypeRefKind::Array(
				TypeRef::new_ext(
					self.get_element_type().expect("Can't get array element type"),
					type_hint,
					None,
					gen_env,
				),
				self.get_size(),
			),

			TypeKind::MemberPointer | TypeKind::DependentSizedArray | TypeKind::Half => TypeRefKind::Ignored,

			_ => {
				unreachable!("Can't decide kind: {:#?}", self)
			}
		}
	}

	fn template_specialization_args<'ge>(self, gen_env: &'ge GeneratorEnv<'tu>) -> Vec<TemplateArg<'tu, 'ge>> {
		match self.get_kind() {
			TypeKind::Typedef => {
				vec![]
			}
			_ => {
				let args = self.get_template_argument_types().unwrap_or_default();
				// there is no way to extract constant generic arguments (e.g. Vec<double, 3>) via libclang
				// so we have to apply some hacks
				static TYPE_EXTRACT: Lazy<Regex> = Lazy::new(|| {
					Regex::new(r#"^.+<\s*(.+?)\s*(?:,\s*(.+?)\s*)?(?:,\s*(.+?)\s*)?(?:,\s*(.+?)\s*)?>$"#)
						.expect("Can't compile static regex")
				});
				let display_name = self.get_display_name();
				let generic_args: UnsyncLazy<Option<Captures>, _> = UnsyncLazy::new(|| TYPE_EXTRACT.captures(&display_name));
				args
					.into_iter()
					.enumerate()
					.map(|(i, type_ref)| {
						if let Some(type_ref) = type_ref {
							TemplateArg::Typename(TypeRef::new(type_ref, gen_env))
						} else {
							if let Some(generic_args) = &*generic_args {
								generic_args
									.get(i + 1)
									.map(|m| m.as_str())
									.map(|literal| {
										if let Some(cnst) = gen_env.resolve_class_constant(literal).and_then(|c| c.value()) {
											cnst.to_string()
										} else {
											literal.to_string()
										}
									})
									.map(TemplateArg::Constant)
							} else {
								None
							}
							.unwrap_or(TemplateArg::Unknown)
						}
					})
					.collect::<Vec<_>>()
			}
		}
	}
}
