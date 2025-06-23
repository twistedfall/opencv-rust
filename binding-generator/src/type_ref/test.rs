use crate::class::ClassDesc;
use crate::type_ref::{TypeRef, TypeRefDesc};
use crate::{Class, SupportedModule};

#[test]
fn test_map_vector() {
	let vec = TypeRef::guess("std::vector<char>", SupportedModule::Core);
	let map_vec = vec.map_vector(|_| TypeRefDesc::schar());
	let expected_vec = TypeRef::guess("std::vector<schar>", SupportedModule::Core);
	assert_eq!(expected_vec, map_vec);

	let vec_vec = TypeRef::guess("std::vector<std::vector<char>>", SupportedModule::Core);
	let map_vec_vec = vec_vec.map_vector(|_| TypeRefDesc::uchar());
	let expected_vec_vec = TypeRef::guess("std::vector<std::vector<uchar>>", SupportedModule::Core);
	assert_eq!(expected_vec_vec, map_vec_vec);
}

#[test]
fn test_guess() {
	let prim = TypeRef::guess("int64_t", SupportedModule::Core);
	assert_eq!(TypeRefDesc::int64_t(), prim);

	let vec = TypeRef::guess("std::vector<int>", SupportedModule::Core);
	assert_eq!(TypeRefDesc::vector_of_int(), vec);

	let vec = TypeRef::guess("std::vector<std::vector<double>>", SupportedModule::Core);
	assert_eq!(TypeRefDesc::vector_of_vector_of_double(), vec);

	// fixme, fails after TypeRefDesc::cv_vec3d() is changed to be a typedef
	// let simple = TypeRef::guess("cv::Vec3d", SupportedModule::Core);
	// assert_eq!(TypeRefDesc::cv_vec3d(), simple);

	let unknown = TypeRef::guess("cv::UnknownClass", SupportedModule::Core);
	assert_ne!(
		TypeRef::new_class(Class::new_desc(ClassDesc::simple("cv::UnknownClass", SupportedModule::Core))),
		unknown
	);
	assert_eq!(
		TypeRef::new_class(Class::new_desc(ClassDesc::boxed("cv::UnknownClass", SupportedModule::Core))),
		unknown
	);
}
