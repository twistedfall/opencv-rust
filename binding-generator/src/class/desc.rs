use std::rc::Rc;

use crate::element::ExcludeKind;

use super::{Class, ClassKind, TemplateKind};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClassDesc<'tu, 'ge> {
	pub kind: ClassKind,
	pub is_abstract: bool,
	pub is_system: bool,
	pub is_public: bool,
	pub exclude_kind: ExcludeKind,
	pub template_kind: TemplateKind<'tu, 'ge>,
	pub bases: Rc<[Class<'tu, 'ge>]>,
	pub cpp_fullname: Rc<str>,
	pub rust_module: Rc<str>,
}

impl<'tu, 'ge> ClassDesc<'tu, 'ge> {
	pub fn boxed(cpp_refname: impl Into<Rc<str>>, rust_module: impl Into<Rc<str>>) -> Self {
		Self {
			kind: ClassKind::Boxed,
			is_abstract: false,
			is_system: false,
			is_public: true,
			exclude_kind: ExcludeKind::Included,
			template_kind: TemplateKind::No,
			bases: Rc::new([]),
			cpp_fullname: cpp_refname.into(),
			rust_module: rust_module.into(),
		}
	}

	pub fn simple(cpp_refname: impl Into<Rc<str>>, rust_module: impl Into<Rc<str>>) -> Self {
		Self {
			kind: ClassKind::Simple,
			..Self::boxed(cpp_refname, rust_module)
		}
	}

	/// `cv::Scalar`
	pub fn cv_scalar() -> Class<'tu, 'ge> {
		Class::new_desc(Self::simple("cv::Scalar", "core"))
	}

	/// `cv::Size`
	pub fn cv_size() -> Class<'tu, 'ge> {
		Class::new_desc(Self::simple("cv::Size", "core"))
	}

	/// `cv::Point`
	pub fn cv_point() -> Class<'tu, 'ge> {
		Class::new_desc(Self::simple("cv::Point", "core"))
	}

	/// `cv::Point2d`
	pub fn cv_point2d() -> Class<'tu, 'ge> {
		Class::new_desc(Self::simple("cv::Point2d", "core"))
	}

	/// `cv::Point3i`
	pub fn cv_point3i() -> Class<'tu, 'ge> {
		Class::new_desc(Self::simple("cv::Point3i", "core"))
	}

	/// `cv::Point3f`
	pub fn cv_point3f() -> Class<'tu, 'ge> {
		Class::new_desc(Self::simple("cv::Point3f", "core"))
	}

	/// `cv::Point3d`
	pub fn cv_point3d() -> Class<'tu, 'ge> {
		Class::new_desc(Self::simple("cv::Point3d", "core"))
	}

	/// `cv::Vec2f`
	pub fn cv_vec2f() -> Class<'tu, 'ge> {
		Class::new_desc(Self::simple("cv::Vec2f", "core"))
	}

	/// `cv::Vec2d`
	pub fn cv_vec2d() -> Class<'tu, 'ge> {
		Class::new_desc(Self::simple("cv::Vec2d", "core"))
	}

	/// `cv::Vec3f`
	pub fn cv_vec3f() -> Class<'tu, 'ge> {
		Class::new_desc(Self::simple("cv::Vec3f", "core"))
	}

	/// `cv::Vec3d`
	pub fn cv_vec3d() -> Class<'tu, 'ge> {
		Class::new_desc(Self::simple("cv::Vec3d", "core"))
	}

	/// `cv::Vec4i`
	pub fn cv_vec4i() -> Class<'tu, 'ge> {
		Class::new_desc(Self::simple("cv::Vec4i", "core"))
	}

	/// `cv::String`
	pub fn cv_string() -> Class<'tu, 'ge> {
		Class::new_desc(Self::boxed("cv::String", "core"))
	}

	/// `cv::MatConstIterator`
	pub fn cv_matconstiterator() -> Class<'tu, 'ge> {
		Class::new_desc(Self::boxed("cv::MatConstIterator", "core"))
	}

	/// `cv::Mat`
	pub fn cv_mat() -> Class<'tu, 'ge> {
		Class::new_desc(Self::boxed("cv::Mat", "core"))
	}

	/// `cv::UMat`
	pub fn cv_umat() -> Class<'tu, 'ge> {
		Class::new_desc(Self::boxed("cv::UMat", "core"))
	}

	pub fn cv_input_array() -> Class<'tu, 'ge> {
		Class::new_desc(Self::boxed("cv::_InputArray", "core"))
	}

	/// `cv::_OutputArray`
	pub fn cv_output_array() -> Class<'tu, 'ge> {
		Class::new_desc(Self::boxed("cv::_OutputArray", "core"))
	}

	/// `cv::_InputOutputArray`
	pub fn cv_input_output_array() -> Class<'tu, 'ge> {
		Class::new_desc(Self::boxed("cv::_InputOutputArray", "core"))
	}

	/// `cv::Feature2D`
	pub fn cv_feature2d() -> Class<'tu, 'ge> {
		Class::new_desc(Self::boxed("cv::Feature2D", "features2d"))
	}

	/// `cv::dnn::LayerParams`
	pub fn cv_dnn_layerparams() -> Class<'tu, 'ge> {
		Class::new_desc(Self::boxed("cv::dnn::LayerParams", "dnn"))
	}

	/// `cv::dnn::DictValue`
	pub fn cv_dnn_dict_value() -> Class<'tu, 'ge> {
		Class::new_desc(Self::boxed("cv::dnn::DictValue", "dnn"))
	}
}
