use std::rc::Rc;

use clang::{Entity, EntityKind, Type, TypeKind};
use once_cell::sync::Lazy;
use once_cell::unsync::Lazy as UnsyncLazy;
use regex::bytes::{Captures, Regex};

use crate::class::ClassDesc;
use crate::function::Function;
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
		let names = match cpp_name {
			"void" => Some(TypeKind::Void),
			"bool" => Some(TypeKind::Bool),
			"char" => Some(TypeKind::CharS),
			"signed char" => Some(TypeKind::SChar),
			"unsigned char" => Some(TypeKind::UChar),
			"wchar_t" => Some(TypeKind::WChar),
			"char16_t" => Some(TypeKind::Char16),
			"char32_t" => Some(TypeKind::Char32),
			"short" => Some(TypeKind::Short),
			"unsigned short" => Some(TypeKind::UShort),
			"int" => Some(TypeKind::Int),
			"unsigned int" => Some(TypeKind::UInt),
			"long" => Some(TypeKind::Long),
			"unsigned long" => Some(TypeKind::ULong),
			"long long" => Some(TypeKind::LongLong),
			"unsigned long long" => Some(TypeKind::ULongLong),
			"__int128_t" => Some(TypeKind::Int128),
			"__uint128_t" => Some(TypeKind::UInt128),
			"float" => Some(TypeKind::Float),
			"double" => Some(TypeKind::Double),
			_ => None,
		};
		names
			.and_then(TypeRefKind::try_from_clang_primitive)
			.or_else(|| {
				settings::PRIMITIVE_TYPEDEFS
					.get(cpp_name)
					.map(|(rust, cpp)| TypeRefKind::Primitive(rust, cpp))
			})
			.map(|type_ref_kind| TypeRef::new_desc(TypeRefDesc::new(type_ref_kind)))
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

	pub fn array_int(size: Option<usize>) -> TypeRef<'tu, 'ge> {
		TypeRef::new_array(Self::int(), size)
	}

	pub fn array_uchar(size: Option<usize>) -> TypeRef<'tu, 'ge> {
		TypeRef::new_array(Self::uchar(), size)
	}

	/// `cv::Size`
	pub fn cv_size() -> TypeRef<'tu, 'ge> {
		TypeRef::new_class(ClassDesc::cv_size())
	}

	/// `cv::Point`
	pub fn cv_point() -> TypeRef<'tu, 'ge> {
		TypeRef::new_class(ClassDesc::cv_point())
	}

	/// `cv::Point2f`
	pub fn cv_point2f() -> TypeRef<'tu, 'ge> {
		TypeRef::new_class(ClassDesc::cv_point2f())
	}

	/// `cv::Point2d`
	pub fn cv_point2d() -> TypeRef<'tu, 'ge> {
		TypeRef::new_class(ClassDesc::cv_point2d())
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

	/// `std::vector<std::vector<cv::Vec2f>>`
	pub fn vector_of_vector_of_cv_vec2f() -> TypeRef<'tu, 'ge> {
		TypeRef::new_vector(Vector::new_desc(VectorDesc::new(TypeRefDesc::vector_of_cv_vec2f())))
	}

	/// `std::vector<std::vector<cv::Vec2d>>`
	pub fn vector_of_vector_of_cv_vec2d() -> TypeRef<'tu, 'ge> {
		TypeRef::new_vector(Vector::new_desc(VectorDesc::new(TypeRefDesc::vector_of_cv_vec2d())))
	}

	/// `std::vector<std::vector<cv::Vec3f>>`
	pub fn vector_of_vector_of_cv_vec3f() -> TypeRef<'tu, 'ge> {
		TypeRef::new_vector(Vector::new_desc(VectorDesc::new(TypeRefDesc::vector_of_cv_vec3f())))
	}

	/// `std::vector<std::vector<cv::Vec3d>>`
	pub fn vector_of_vector_of_cv_vec3d() -> TypeRef<'tu, 'ge> {
		TypeRef::new_vector(Vector::new_desc(VectorDesc::new(TypeRefDesc::vector_of_cv_vec3d())))
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

	/// `std::vector<cv::Point2f>`
	pub fn vector_of_cv_point2f() -> TypeRef<'tu, 'ge> {
		TypeRef::new_vector(Vector::new_desc(VectorDesc::new(TypeRefDesc::cv_point2f())))
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

	/// `std::vector<std::vector<cv::Point2f>>`
	pub fn vector_of_vector_of_cv_point2f() -> TypeRef<'tu, 'ge> {
		TypeRef::new_vector(Vector::new_desc(VectorDesc::new(Self::vector_of_cv_point2f())))
	}

	/// `std::vector<std::vector<cv::Point2d>>`
	pub fn vector_of_vector_of_cv_point2d() -> TypeRef<'tu, 'ge> {
		TypeRef::new_vector(Vector::new_desc(VectorDesc::new(Self::vector_of_cv_point2d())))
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
		let kind = self.get_kind();
		TypeRefKind::try_from_clang_primitive(kind).unwrap_or_else(|| {
			match kind {
				TypeKind::Pointer => {
					let pointee = self.get_pointee_type().expect("No pointee type for pointer");
					let pointee_typeref = TypeRef::new_ext(pointee, type_hint.clone(), parent_entity, gen_env);
					if pointee_typeref.as_function().is_some() {
						pointee_typeref.kind().into_owned()
					} else if matches!(type_hint, TypeRefTypeHint::Slice | TypeRefTypeHint::NullableSlice) {
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
							if let Some(parent_entity) = parent_entity {
								TypeRefKind::Function(Function::new(self, parent_entity, gen_env))
							} else {
								TypeRefKind::Ignored
							}
						} else if let Some(&(rust, cpp)) = settings::PRIMITIVE_TYPEDEFS.get(generic_type.as_str()) {
							// uint64_t in gapi module ends here for some reason
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

				TypeKind::ConstantArray | TypeKind::IncompleteArray => {
					let mut size = self.get_size();
					if size.is_none() {
						if let TypeRefTypeHint::AddArrayLength(force_size) = type_hint {
							size = Some(force_size);
						}
					}
					TypeRefKind::Array(
						TypeRef::new_ext(
							self.get_element_type().expect("Can't get array element type"),
							type_hint,
							None,
							gen_env,
						),
						size,
					)
				}

				TypeKind::MemberPointer | TypeKind::DependentSizedArray | TypeKind::Half => TypeRefKind::Ignored,

				_ => {
					unreachable!("Can't decide kind: {:#?}", self)
				}
			}
		})
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
				let generic_args: UnsyncLazy<Option<Captures>, _> =
					UnsyncLazy::new(|| TYPE_EXTRACT.captures(display_name.as_bytes()));
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
									.map(|m| TemplateArg::Constant(String::from_utf8_lossy(m.as_bytes()).into_owned()))
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
