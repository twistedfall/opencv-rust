use std::rc::Rc;

use super::{Class, ClassKind, TemplateKind};
use crate::element::ExcludeKind;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClassDesc<'tu, 'ge> {
	pub kind: ClassKind,
	pub is_abstract: bool,
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

	pub fn system(cpp_refname: impl Into<Rc<str>>, rust_module: impl Into<Rc<str>>) -> Self {
		Self {
			kind: ClassKind::System,
			..Self::boxed(cpp_refname, rust_module)
		}
	}

	/// `cv::Scalar_`
	pub fn cv_scalar_() -> Class<'tu, 'ge> {
		Class::new_desc(Self::simple("cv::Scalar_", "core"))
	}

	/// `cv::Size_`
	pub fn cv_size_() -> Class<'tu, 'ge> {
		Class::new_desc(Self::simple("cv::Size_", "core"))
	}

	/// `cv::Point_`
	pub fn cv_point_() -> Class<'tu, 'ge> {
		Class::new_desc(Self::simple("cv::Point_", "core"))
	}

	/// `cv::Vec`
	pub fn cv_vec() -> Class<'tu, 'ge> {
		Class::new_desc(Self::simple("cv::Vec", "core"))
	}

	/// `cv::String`
	pub fn cv_string() -> Class<'tu, 'ge> {
		Class::new_desc(Self::system("cv::String", "core"))
	}

	/// `std::String`
	pub fn std_string() -> Class<'tu, 'ge> {
		Class::new_desc(Self::system("std::string", "core"))
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

	/// `cv::dnn::DictValue`
	pub fn cv_dnn_dict_value() -> Class<'tu, 'ge> {
		Class::new_desc(Self::boxed("cv::dnn::DictValue", "dnn"))
	}

	/// `cv::Feature2D`
	pub fn cv_keypoint() -> Class<'tu, 'ge> {
		Class::new_desc(Self::boxed("cv::KeyPoint", "core"))
	}
}
