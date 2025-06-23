use std::rc::Rc;

use super::{Class, ClassKind, TemplateKind};
use crate::element::ExcludeKind;
use crate::SupportedModule;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClassDesc<'tu, 'ge> {
	pub kind: ClassKind,
	pub is_abstract: bool,
	pub is_public: bool,
	pub exclude_kind: ExcludeKind,
	pub template_kind: TemplateKind<'tu, 'ge>,
	pub bases: Rc<[Class<'tu, 'ge>]>,
	pub cpp_fullname: Rc<str>,
	pub rust_module: SupportedModule,
}

impl<'tu, 'ge> ClassDesc<'tu, 'ge> {
	pub fn boxed(cpp_refname: impl Into<Rc<str>>, rust_module: SupportedModule) -> Self {
		Self {
			kind: ClassKind::Boxed,
			is_abstract: false,
			is_public: true,
			exclude_kind: ExcludeKind::Included,
			template_kind: TemplateKind::No,
			bases: Rc::new([]),
			cpp_fullname: cpp_refname.into(),
			rust_module,
		}
	}

	pub fn simple(cpp_refname: impl Into<Rc<str>>, rust_module: SupportedModule) -> Self {
		Self {
			kind: ClassKind::Simple,
			..Self::boxed(cpp_refname, rust_module)
		}
	}

	pub fn system(cpp_refname: impl Into<Rc<str>>, rust_module: SupportedModule) -> Self {
		Self {
			kind: ClassKind::System,
			..Self::boxed(cpp_refname, rust_module)
		}
	}

	/// `cv::Scalar_`
	pub fn cv_scalar_() -> Class<'tu, 'ge> {
		Class::new_desc(Self::simple("cv::Scalar_", SupportedModule::Core))
	}

	/// `cv::Size_`
	pub fn cv_size_() -> Class<'tu, 'ge> {
		Class::new_desc(Self::simple("cv::Size_", SupportedModule::Core))
	}

	/// `cv::Point_`
	pub fn cv_point_() -> Class<'tu, 'ge> {
		Class::new_desc(Self::simple("cv::Point_", SupportedModule::Core))
	}

	/// `cv::Vec`
	pub fn cv_vec() -> Class<'tu, 'ge> {
		Class::new_desc(Self::simple("cv::Vec", SupportedModule::Core))
	}

	/// `cv::String`
	pub fn cv_string() -> Class<'tu, 'ge> {
		Class::new_desc(Self::system("cv::String", SupportedModule::Core))
	}

	/// `std::String`
	pub fn std_string() -> Class<'tu, 'ge> {
		Class::new_desc(Self::system("std::string", SupportedModule::Core))
	}

	/// `cv::MatConstIterator`
	pub fn cv_matconstiterator() -> Class<'tu, 'ge> {
		Class::new_desc(Self::boxed("cv::MatConstIterator", SupportedModule::Core))
	}

	/// `cv::Mat`
	pub fn cv_mat() -> Class<'tu, 'ge> {
		Class::new_desc(Self::boxed("cv::Mat", SupportedModule::Core))
	}

	/// `cv::UMat`
	pub fn cv_umat() -> Class<'tu, 'ge> {
		Class::new_desc(Self::boxed("cv::UMat", SupportedModule::Core))
	}

	pub fn cv_input_array() -> Class<'tu, 'ge> {
		Class::new_desc(Self::boxed("cv::_InputArray", SupportedModule::Core))
	}

	/// `cv::_OutputArray`
	pub fn cv_output_array() -> Class<'tu, 'ge> {
		Class::new_desc(Self::boxed("cv::_OutputArray", SupportedModule::Core))
	}

	/// `cv::_InputOutputArray`
	pub fn cv_input_output_array() -> Class<'tu, 'ge> {
		Class::new_desc(Self::boxed("cv::_InputOutputArray", SupportedModule::Core))
	}

	/// `cv::Feature2D`
	pub fn cv_feature2d() -> Class<'tu, 'ge> {
		Class::new_desc(Self::boxed("cv::Feature2D", SupportedModule::Features2d))
	}

	/// `cv::dnn::DictValue`
	pub fn cv_dnn_dict_value() -> Class<'tu, 'ge> {
		Class::new_desc(Self::boxed("cv::dnn::DictValue", SupportedModule::Dnn))
	}

	/// `cv::Feature2D`
	pub fn cv_keypoint() -> Class<'tu, 'ge> {
		Class::new_desc(Self::boxed("cv::KeyPoint", SupportedModule::Core))
	}
}
