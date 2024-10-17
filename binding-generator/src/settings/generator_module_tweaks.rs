use crate::settings::TypeRefFactory;
use crate::type_ref::TypeRefDesc;

#[derive(Debug)]
pub struct ModuleTweak<'l> {
	pub generate_types: &'l [TypeRefFactory],
}

impl ModuleTweak<'_> {
	pub fn empty() -> Self {
		Self { generate_types: &[] }
	}
}

pub fn generator_module_tweaks_factory(module: &str) -> ModuleTweak<'static> {
	match module {
		"core" => ModuleTweak {
			generate_types: &[TypeRefDesc::ptr_of_float],
		},
		"aruco" => ModuleTweak {
			generate_types: &[TypeRefDesc::vector_of_cv_vec3f, TypeRefDesc::vector_of_cv_vec3d],
		},
		"ccalib" => ModuleTweak {
			generate_types: &[
				// for cv::omnidir::calibrate objectPoints
				TypeRefDesc::vector_of_vector_of_cv_vec3f,
				TypeRefDesc::vector_of_vector_of_cv_vec3d,
				TypeRefDesc::vector_of_vector_of_cv_point3f,
				TypeRefDesc::vector_of_vector_of_cv_point3d,
				// for cv::omnidir::calibrate imagePoints
				TypeRefDesc::vector_of_vector_of_cv_vec2f,
				TypeRefDesc::vector_of_vector_of_cv_point2f,
				TypeRefDesc::vector_of_vector_of_cv_vec2d,
				TypeRefDesc::vector_of_vector_of_cv_point2d,
			],
		},
		"calib3d" => ModuleTweak {
			generate_types: &[
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
		"dnn" => ModuleTweak {
			generate_types: &[TypeRefDesc::vector_of_vector_of_int], // Make sure that `Vector<MatShape>` is generated
		},
		"features2d" => ModuleTweak {
			// type used in other modules, thus needs casting (https://github.com/twistedfall/opencv-rust/issues/218)
			generate_types: &[TypeRefDesc::ptr_of_cv_feature2d],
		},
		"imgproc" => ModuleTweak {
			generate_types: &[
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
		_ => ModuleTweak::empty(),
	}
}
