use std::rc::Rc;

use clang::{Entity, EntityKind, Type, TypeKind};
use once_cell::sync::Lazy;
use once_cell::unsync::Lazy as UnsyncLazy;
use regex::{Captures, Regex};

use crate::class::ClassDesc;
use crate::function::Function;
use crate::settings::ArgOverride;
use crate::smart_ptr::{SmartPtr, SmartPtrDesc};
use crate::tuple::Tuple;
use crate::type_ref::{Constness, TemplateArg, TypeRef, TypeRefKind, TypeRefTypeHint};
use crate::typedef::NewTypedefResult;
use crate::vector::{Vector, VectorDesc};
use crate::{settings, Class, CppNameStyle, Element, Enum, GeneratorEnv, StringExt, Typedef};

#[derive(Clone, Debug)]
pub struct TypeRefDesc<'tu, 'ge> {
	pub kind: TypeRefKind<'tu, 'ge>,
	pub inherent_constness: Constness,
	pub type_hint: TypeRefTypeHint,
	pub template_specialization_args: Rc<[TemplateArg<'tu, 'ge>]>,
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

	pub fn try_primitive(cpp_name: &str) -> Option<TypeRef<'tu, 'ge>> {
		let rust_name = match cpp_name {
			"void" => Some(("()", "void")),
			"bool" => Some(("bool", "bool")),
			"char" => Some(("c_char", "char")),
			"signed char" => Some(("i8", "signed char")),
			"unsigned char" => Some(("u8", "unsigned char")),
			"wchar_t" => Some(("u16", "wchar_t")),
			"char16_t" => Some(("u16", "char16_t")),
			"char32_t" => Some(("u16", "char32_t")),
			"short" => Some(("i16", "short")),
			"unsigned short" => Some(("u16", "unsigned short")),
			"int" => Some(("i32", "int")),
			"unsigned int" => Some(("u32", "unsigned int")),
			"long" => Some(("i32", "long")),
			"unsigned long" => Some(("u32", "unsigned long")),
			"long long" => Some(("i64", "long long")),
			"unsigned long long" => Some(("u64", "unsigned long long")),
			"__int128_t" => Some(("i128", "__int128_t")),
			"__uint128_t" => Some(("u128", "__uint128_t")),
			"float" => Some(("f32", "float")),
			"double" => Some(("f64", "double")),
			_ => settings::PRIMITIVE_TYPEDEFS.get(cpp_name).copied(),
		};
		rust_name.map(|(rust, cpp)| TypeRef::new_desc(TypeRefDesc::new(TypeRefKind::Primitive(rust, cpp))))
	}

	pub fn void() -> TypeRef<'tu, 'ge> {
		Self::try_primitive("void").expect("Static primitive type")
	}

	pub fn bool() -> TypeRef<'tu, 'ge> {
		Self::try_primitive("bool").expect("Static primitive type")
	}

	pub fn schar() -> TypeRef<'tu, 'ge> {
		Self::try_primitive("signed char").expect("Static primitive type")
	}

	pub fn char() -> TypeRef<'tu, 'ge> {
		Self::try_primitive("char").expect("Static primitive type")
	}

	pub fn uchar() -> TypeRef<'tu, 'ge> {
		Self::try_primitive("unsigned char").expect("Static primitive type")
	}

	pub fn float() -> TypeRef<'tu, 'ge> {
		Self::try_primitive("float").expect("Static primitive type")
	}

	pub fn double() -> TypeRef<'tu, 'ge> {
		Self::try_primitive("double").expect("Static primitive type")
	}

	pub fn int() -> TypeRef<'tu, 'ge> {
		Self::try_primitive("int").expect("Static primitive type")
	}

	pub fn int64_t() -> TypeRef<'tu, 'ge> {
		Self::try_primitive("int64_t").expect("Static primitive type")
	}

	pub fn uint64_t() -> TypeRef<'tu, 'ge> {
		Self::try_primitive("uint64_t").expect("Static primitive type")
	}

	pub fn size_t() -> TypeRef<'tu, 'ge> {
		Self::try_primitive("size_t").expect("Static primitive type")
	}

	/// `cv::Size`
	pub fn cv_size() -> TypeRef<'tu, 'ge> {
		TypeRef::new_class(ClassDesc::cv_size())
	}

	/// `cv::Point`
	pub fn cv_point() -> TypeRef<'tu, 'ge> {
		TypeRef::new_class(ClassDesc::cv_point())
	}

	/// `cv::Point2d`
	pub fn cv_point2d() -> TypeRef<'tu, 'ge> {
		TypeRef::new_class(ClassDesc::cv_point())
	}

	/// `cv::Point3i`
	pub fn cv_point3i() -> TypeRef<'tu, 'ge> {
		TypeRef::new_class(ClassDesc::cv_point3i())
	}

	/// `cv::Point3f`
	pub fn cv_point3f() -> TypeRef<'tu, 'ge> {
		TypeRef::new_class(ClassDesc::cv_point3f())
	}

	/// `cv::Point3d`
	pub fn cv_point3d() -> TypeRef<'tu, 'ge> {
		TypeRef::new_class(ClassDesc::cv_point3d())
	}

	/// `cv::Vec2f`
	pub fn cv_vec2f() -> TypeRef<'tu, 'ge> {
		TypeRef::new_class(ClassDesc::cv_vec2f())
	}

	/// `cv::Vec2d`
	pub fn cv_vec2d() -> TypeRef<'tu, 'ge> {
		TypeRef::new_class(ClassDesc::cv_vec2d())
	}

	/// `cv::Vec3f`
	pub fn cv_vec3f() -> TypeRef<'tu, 'ge> {
		TypeRef::new_class(ClassDesc::cv_vec3f())
	}

	/// `cv::Vec3d`
	pub fn cv_vec3d() -> TypeRef<'tu, 'ge> {
		TypeRef::new_class(ClassDesc::cv_vec3d())
	}

	/// `cv::Vec4i`
	pub fn cv_vec4i() -> TypeRef<'tu, 'ge> {
		TypeRef::new_class(ClassDesc::cv_vec4i())
	}

	/// `cv::Scalar`
	pub fn cv_scalar() -> TypeRef<'tu, 'ge> {
		TypeRef::new_class(ClassDesc::cv_scalar())
	}

	/// `cv::_InputArray`
	pub fn cv_input_array() -> TypeRef<'tu, 'ge> {
		TypeRef::new_class(ClassDesc::cv_input_array())
	}

	/// `cv::_OutputArray`
	pub fn cv_output_array() -> TypeRef<'tu, 'ge> {
		TypeRef::new_class(ClassDesc::cv_output_array())
	}

	/// `cv::_InputOutputArray`
	pub fn cv_input_output_array() -> TypeRef<'tu, 'ge> {
		TypeRef::new_class(ClassDesc::cv_input_output_array())
	}

	/// `cv::String`
	pub fn cv_string() -> TypeRef<'tu, 'ge> {
		TypeRef::new_class(ClassDesc::cv_string())
	}

	/// `std::vector<std::vector<double>>`
	pub fn vector_of_vector_of_double() -> TypeRef<'tu, 'ge> {
		TypeRef::new_vector(Vector::new_desc(VectorDesc::new(TypeRef::new_vector(Vector::new_desc(
			VectorDesc::new(TypeRefDesc::double()),
		)))))
	}

	/// `std::vector<std::vector<int>>`
	pub fn vector_of_int() -> TypeRef<'tu, 'ge> {
		TypeRef::new_vector(Vector::new_desc(VectorDesc::new(TypeRefDesc::int())))
	}

	/// `std::vector<std::vector<int>>`
	pub fn vector_of_vector_of_int() -> TypeRef<'tu, 'ge> {
		TypeRef::new_vector(Vector::new_desc(VectorDesc::new(Self::vector_of_int())))
	}

	/// `std::vector<cv::String>`
	pub fn vector_of_cv_string() -> TypeRef<'tu, 'ge> {
		TypeRef::new_vector(Vector::new_desc(VectorDesc::new(TypeRefDesc::cv_string())))
	}

	/// `std::vector<cv::Vec2f>`
	pub fn vector_of_cv_vec2f() -> TypeRef<'tu, 'ge> {
		TypeRef::new_vector(Vector::new_desc(VectorDesc::new(TypeRefDesc::cv_vec2f())))
	}

	/// `std::vector<cv::Vec2d>`
	pub fn vector_of_cv_vec2d() -> TypeRef<'tu, 'ge> {
		TypeRef::new_vector(Vector::new_desc(VectorDesc::new(TypeRefDesc::cv_vec2d())))
	}

	/// `std::vector<cv::Vec3f>`
	pub fn vector_of_cv_vec3f() -> TypeRef<'tu, 'ge> {
		TypeRef::new_vector(Vector::new_desc(VectorDesc::new(TypeRefDesc::cv_vec3f())))
	}

	/// `std::vector<std::vector<cv::Vec3f>>`
	pub fn vector_of_vector_of_cv_vec3f() -> TypeRef<'tu, 'ge> {
		TypeRef::new_vector(Vector::new_desc(VectorDesc::new(TypeRefDesc::vector_of_cv_vec3f())))
	}

	/// `std::vector<cv::Vec3d>`
	pub fn vector_of_cv_vec3d() -> TypeRef<'tu, 'ge> {
		TypeRef::new_vector(Vector::new_desc(VectorDesc::new(TypeRefDesc::cv_vec3d())))
	}

	/// `std::vector<cv::Vec4i>`
	pub fn vector_of_cv_vec4i() -> TypeRef<'tu, 'ge> {
		TypeRef::new_vector(Vector::new_desc(VectorDesc::new(TypeRefDesc::cv_vec4i())))
	}

	/// `std::vector<std::vector<cv::Point>>`
	pub fn vector_of_vector_of_cv_point() -> TypeRef<'tu, 'ge> {
		TypeRef::new_vector(Vector::new_desc(VectorDesc::new(TypeRef::new_vector(Vector::new_desc(
			VectorDesc::new(TypeRefDesc::cv_point()),
		)))))
	}

	/// `std::vector<cv::Point2d>`
	pub fn vector_of_cv_point2d() -> TypeRef<'tu, 'ge> {
		TypeRef::new_vector(Vector::new_desc(VectorDesc::new(TypeRefDesc::cv_point2d())))
	}

	/// `std::vector<cv::Point3i>`
	pub fn vector_of_cv_point3i() -> TypeRef<'tu, 'ge> {
		TypeRef::new_vector(Vector::new_desc(VectorDesc::new(TypeRefDesc::cv_point3i())))
	}

	/// `std::vector<std::vector<cv::Point3i>>`
	pub fn vector_of_vector_of_cv_point3i() -> TypeRef<'tu, 'ge> {
		TypeRef::new_vector(Vector::new_desc(VectorDesc::new(Self::vector_of_cv_point3i())))
	}

	/// `std::vector<cv::Point3f>`
	pub fn vector_of_cv_point3f() -> TypeRef<'tu, 'ge> {
		TypeRef::new_vector(Vector::new_desc(VectorDesc::new(TypeRefDesc::cv_point3f())))
	}

	/// `std::vector<std::vector<cv::Point3f>>`
	pub fn vector_of_vector_of_cv_point3f() -> TypeRef<'tu, 'ge> {
		TypeRef::new_vector(Vector::new_desc(VectorDesc::new(Self::vector_of_cv_point3f())))
	}

	/// `std::vector<cv::Point3d>`
	pub fn vector_of_cv_point3d() -> TypeRef<'tu, 'ge> {
		TypeRef::new_vector(Vector::new_desc(VectorDesc::new(TypeRefDesc::cv_point3d())))
	}

	/// `std::vector<std::vector<cv::Point3d>>`
	pub fn vector_of_vector_of_cv_point3d() -> TypeRef<'tu, 'ge> {
		TypeRef::new_vector(Vector::new_desc(VectorDesc::new(Self::vector_of_cv_point3d())))
	}

	/// `cv::Ptr<float>`
	pub fn ptr_of_float() -> TypeRef<'tu, 'ge> {
		TypeRef::new_smartptr(SmartPtr::new_desc(SmartPtrDesc::new(TypeRefDesc::float())))
	}

	/// `cv::Ptr<cv::Feature2d>`
	pub fn ptr_of_cv_feature2d() -> TypeRef<'tu, 'ge> {
		TypeRef::new_smartptr(SmartPtr::new_desc(SmartPtrDesc::new(TypeRefDesc::cv_feature2d())))
	}

	/// `cv::Feature2D`
	pub fn cv_feature2d() -> TypeRef<'tu, 'ge> {
		TypeRef::new_class(ClassDesc::cv_feature2d())
	}

	/// `cv::dnn::DictValue`
	pub fn cv_dnn_dict_value() -> TypeRef<'tu, 'ge> {
		TypeRef::new_class(ClassDesc::cv_dnn_dict_value())
	}
}

pub trait ClangTypeExt<'tu> {
	fn kind<'ge>(
		self,
		type_hint: TypeRefTypeHint,
		parent_entity: Option<Entity<'tu>>,
		gen_env: &'ge GeneratorEnv<'tu>,
	) -> TypeRefKind<'tu, 'ge>;

	fn template_specialization_args<'ge>(self, gen_env: &'ge GeneratorEnv<'tu>) -> Vec<TemplateArg<'tu, 'ge>>;
}

impl<'tu> ClangTypeExt<'tu> for Type<'tu> {
	fn kind<'ge>(
		self,
		type_hint: TypeRefTypeHint,
		parent_entity: Option<Entity<'tu>>,
		gen_env: &'ge GeneratorEnv<'tu>,
	) -> TypeRefKind<'tu, 'ge> {
		match self.get_kind() {
			TypeKind::Void => TypeRefKind::Primitive("()", "void"),
			TypeKind::Bool => TypeRefKind::Primitive("bool", "bool"),
			TypeKind::CharS | TypeKind::CharU => TypeRefKind::Primitive("c_char", "char"),
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
				let pointee_typeref = TypeRef::new_ext(pointee, type_hint, parent_entity, gen_env);
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
					Regex::new(r"^.+<\s*(.+?)\s*(?:,\s*(.+?)\s*)?(?:,\s*(.+?)\s*)?(?:,\s*(.+?)\s*)?>$")
						.expect("Can't compile static regex")
				});
				// getting declaration resolves constants so `Vec<int, nFeatures>` becomes `Vec<int, 18>`
				let display_name = self
					.get_declaration()
					.and_then(|d| d.get_display_name())
					.unwrap_or_else(|| self.get_display_name());
				let generic_args: UnsyncLazy<Option<Captures>, _> = UnsyncLazy::new(|| TYPE_EXTRACT.captures(&display_name));
				args
					.into_iter()
					.enumerate()
					.map(|(i, type_ref)| {
						if let Some(type_ref) = type_ref {
							TemplateArg::Typename(TypeRef::new(type_ref, gen_env))
						} else {
							if let Some(generic_args) = &*generic_args {
								generic_args.get(i + 1).map(|m| TemplateArg::Constant(m.as_str().to_string()))
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
