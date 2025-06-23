use std::rc::Rc;

use super::Typedef;
use crate::type_ref::{TypeRef, TypeRefDesc};
use crate::SupportedModule;

pub struct TypedefDesc<'tu, 'ge> {
	pub cpp_fullname: Rc<str>,
	pub rust_module: SupportedModule,
	pub underlying_type: TypeRef<'tu, 'ge>,
}

impl<'tu, 'ge> TypedefDesc<'tu, 'ge> {
	pub fn new(cpp_fullname: impl Into<Rc<str>>, rust_module: SupportedModule, underlying_type: TypeRef<'tu, 'ge>) -> Self {
		Self {
			cpp_fullname: cpp_fullname.into(),
			rust_module,
			underlying_type,
		}
	}

	/// `cv::Vec4i`
	pub fn cv_vec4i() -> Typedef<'tu, 'ge> {
		Typedef::new_desc(Self::new("cv::Vec4i", SupportedModule::Core, TypeRefDesc::cv_vec()))
	}

	/// `cv::Scalar`
	pub fn cv_scalar() -> Typedef<'tu, 'ge> {
		Typedef::new_desc(Self::new("cv::Scalar", SupportedModule::Core, TypeRefDesc::cv_scalar_()))
	}

	/// `cv::Size`
	pub fn cv_size() -> Typedef<'tu, 'ge> {
		Typedef::new_desc(Self::new("cv::Size", SupportedModule::Core, TypeRefDesc::cv_size_()))
	}

	/// `cv::Point`
	pub fn cv_point() -> Typedef<'tu, 'ge> {
		Typedef::new_desc(Self::new("cv::Point", SupportedModule::Core, TypeRefDesc::cv_point_()))
	}

	/// `cv::Point2f`
	pub fn cv_point2f() -> Typedef<'tu, 'ge> {
		Typedef::new_desc(Self::new("cv::Point2f", SupportedModule::Core, TypeRefDesc::cv_point_()))
	}

	/// `cv::Point2d`
	pub fn cv_point2d() -> Typedef<'tu, 'ge> {
		Typedef::new_desc(Self::new("cv::Point2d", SupportedModule::Core, TypeRefDesc::cv_point_()))
	}

	/// `cv::Point3i`
	pub fn cv_point3i() -> Typedef<'tu, 'ge> {
		Typedef::new_desc(Self::new("cv::Point3i", SupportedModule::Core, TypeRefDesc::cv_point_()))
	}

	/// `cv::Point3f`
	pub fn cv_point3f() -> Typedef<'tu, 'ge> {
		Typedef::new_desc(Self::new("cv::Point3f", SupportedModule::Core, TypeRefDesc::cv_point_()))
	}

	/// `cv::Point3d`
	pub fn cv_point3d() -> Typedef<'tu, 'ge> {
		Typedef::new_desc(Self::new("cv::Point3d", SupportedModule::Core, TypeRefDesc::cv_point_()))
	}

	/// `cv::Vec2f`
	pub fn cv_vec2f() -> Typedef<'tu, 'ge> {
		Typedef::new_desc(Self::new("cv::Vec2f", SupportedModule::Core, TypeRefDesc::cv_vec()))
	}

	/// `cv::Vec2d`
	pub fn cv_vec2d() -> Typedef<'tu, 'ge> {
		Typedef::new_desc(Self::new("cv::Vec2d", SupportedModule::Core, TypeRefDesc::cv_vec()))
	}

	/// `cv::Vec3f`
	pub fn cv_vec3f() -> Typedef<'tu, 'ge> {
		Typedef::new_desc(Self::new("cv::Vec3f", SupportedModule::Core, TypeRefDesc::cv_vec()))
	}

	/// `cv::Vec3d`
	pub fn cv_vec3d() -> Typedef<'tu, 'ge> {
		Typedef::new_desc(Self::new("cv::Vec3d", SupportedModule::Core, TypeRefDesc::cv_vec()))
	}
}
