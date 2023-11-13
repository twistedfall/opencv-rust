use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::settings::TypeRefFactory;
use crate::type_ref::TypeRefDesc;

pub struct ModuleTweak {
	pub generate_types: Vec<TypeRefFactory>,
}

pub static GENERATOR_MODULE_TWEAKS: Lazy<HashMap<&str, ModuleTweak>> = Lazy::new(|| {
	HashMap::from([
		(
			"core",
			ModuleTweak {
				generate_types: vec![TypeRefDesc::ptr_of_float],
			},
		),
		(
			"aruco",
			ModuleTweak {
				generate_types: vec![TypeRefDesc::vector_of_cv_vec3f, TypeRefDesc::vector_of_cv_vec3d],
			},
		),
		(
			"calib3d",
			ModuleTweak {
				generate_types: vec![
					// for calibrate_camera
					TypeRefDesc::vector_of_cv_point3i,
					TypeRefDesc::vector_of_vector_of_cv_point3i,
					TypeRefDesc::vector_of_cv_point3f,
					TypeRefDesc::vector_of_vector_of_cv_point3f,
					TypeRefDesc::vector_of_cv_point3d,
					TypeRefDesc::vector_of_vector_of_cv_point3d,
					TypeRefDesc::vector_of_cv_vec3f,
					TypeRefDesc::vector_of_vector_of_cv_vec3f,
					// for solve_pnp tvec and rvec parameters
					TypeRefDesc::vector_of_vector_of_double,
					// for solve_pnp_ransac imagePoints parameter
					TypeRefDesc::vector_of_cv_point2d,
				],
			},
		),
		(
			"dnn",
			ModuleTweak {
				generate_types: vec![TypeRefDesc::vector_of_vector_of_int], // Make sure that `Vector<MatShape>` is generated
			},
		),
		(
			"features2d",
			ModuleTweak {
				// type used in other modules, thus needs casting (https://github.com/twistedfall/opencv-rust/issues/218)
				generate_types: vec![TypeRefDesc::ptr_of_cv_feature2d],
			},
		),
		(
			"imgproc",
			ModuleTweak {
				generate_types: vec![
					// for findContours()
					TypeRefDesc::vector_of_cv_vec4i,
					TypeRefDesc::vector_of_vector_of_cv_point,
					// for HoughLines()
					TypeRefDesc::vector_of_cv_vec2f,
					TypeRefDesc::vector_of_cv_vec2d,
					TypeRefDesc::vector_of_cv_vec3f,
					TypeRefDesc::vector_of_cv_vec3d,
				],
			},
		),
	])
});
