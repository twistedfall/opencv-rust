use super::TypeRefFactory;
use crate::type_ref::TypeRefDesc;
use crate::SupportedModule;

#[derive(Debug)]
pub struct ModuleTweak<'l> {
	pub generate_types: &'l [TypeRefFactory],
}

impl ModuleTweak<'_> {
	pub fn empty() -> Self {
		Self { generate_types: &[] }
	}
}

pub fn generator_module_tweaks_factory(module: SupportedModule) -> ModuleTweak<'static> {
	match module {
		SupportedModule::Core => ModuleTweak {
			generate_types: &[
				TypeRefDesc::ptr_of_float,
				// for the `field_access_on_ptr` test
				TypeRefDesc::ptr_of_cv_keypoint,
			],
		},
		SupportedModule::Aruco => ModuleTweak {
			generate_types: &[TypeRefDesc::vector_of_cv_vec3f, TypeRefDesc::vector_of_cv_vec3d],
		},
		SupportedModule::CCalib => ModuleTweak {
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
		SupportedModule::Calib3d | SupportedModule::Calib | SupportedModule::ThreeD => ModuleTweak {
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
		SupportedModule::Dnn => ModuleTweak {
			generate_types: &[TypeRefDesc::vector_of_vector_of_int], // Make sure that `Vector<MatShape>` is generated
		},
		SupportedModule::Features2d | SupportedModule::Features => ModuleTweak {
			// type used in other modules, thus needs casting (https://github.com/twistedfall/opencv-rust/issues/218)
			generate_types: &[TypeRefDesc::ptr_of_cv_feature2d],
		},
		SupportedModule::ImgProc => ModuleTweak {
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
