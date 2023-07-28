use crate::class::ClassDesc;
use crate::type_ref::{TypeRef, TypeRefDesc};
use crate::Class;

#[test]
fn test_map_vector() {
	let vec = TypeRef::guess("std::vector<char>", "core");
	let map_vec = vec.map_vector(|_| TypeRefDesc::schar());
	let expected_vec = TypeRef::guess("std::vector<schar>", "core");
	assert_eq!(expected_vec, map_vec);

	let vec_vec = TypeRef::guess("std::vector<std::vector<char>>", "core");
	let map_vec_vec = vec_vec.map_vector(|_| TypeRefDesc::uchar());
	let expected_vec_vec = TypeRef::guess("std::vector<std::vector<uchar>>", "core");
	assert_eq!(expected_vec_vec, map_vec_vec);
}

#[test]
fn test_guess() {
	let prim = TypeRef::guess("int64_t", "core");
	assert_eq!(TypeRefDesc::int64_t(), prim);

	let vec = TypeRef::guess("std::vector<int>", "core");
	assert_eq!(TypeRefDesc::vector_of_int(), vec);

	let vec = TypeRef::guess("std::vector<std::vector<double>>", "core");
	assert_eq!(TypeRefDesc::vector_of_vector_of_double(), vec);

	let simple = TypeRef::guess("cv::Vec3d", "core");
	assert_eq!(TypeRefDesc::cv_vec3d(), simple);

	let unknown = TypeRef::guess("cv::UnknownClass", "core");
	assert_ne!(
		TypeRef::new_class(Class::new_desc(ClassDesc::simple("cv::UnknownClass", "core"))),
		unknown
	);
	assert_eq!(
		TypeRef::new_class(Class::new_desc(ClassDesc::boxed("cv::UnknownClass", "core"))),
		unknown
	);
}
