use std::rc::Rc;

use super::Typedef;
use crate::type_ref::{TypeRef, TypeRefDesc};

pub struct TypedefDesc<'tu, 'ge> {
	pub cpp_fullname: Rc<str>,
	pub rust_module: Rc<str>,
	pub underlying_type: TypeRef<'tu, 'ge>,
}

impl<'tu, 'ge> TypedefDesc<'tu, 'ge> {
	pub fn new(cpp_fullname: impl Into<Rc<str>>, rust_module: impl Into<Rc<str>>, underlying_type: TypeRef<'tu, 'ge>) -> Self {
		Self {
			cpp_fullname: cpp_fullname.into(),
			rust_module: rust_module.into(),
			underlying_type,
		}
	}

	/// `cv::Vec4i`
	pub fn cv_vec4i() -> Typedef<'tu, 'ge> {
		Typedef::new_desc(Self::new("cv::Vec4i", "core", TypeRefDesc::cv_vec()))
	}

	/// `cv::Scalar`
	pub fn cv_scalar() -> Typedef<'tu, 'ge> {
		Typedef::new_desc(Self::new("cv::Scalar", "core", TypeRefDesc::cv_scalar_()))
	}

	/// `cv::Size`
	pub fn cv_size() -> Typedef<'tu, 'ge> {
		Typedef::new_desc(Self::new("cv::Size", "core", TypeRefDesc::cv_size_()))
	}

	/// `cv::Point`
	pub fn cv_point() -> Typedef<'tu, 'ge> {
		Typedef::new_desc(Self::new("cv::Point", "core", TypeRefDesc::cv_point_()))
	}

	/// `cv::Point2f`
	pub fn cv_point2f() -> Typedef<'tu, 'ge> {
		Typedef::new_desc(Self::new("cv::Point2f", "core", TypeRefDesc::cv_point_()))
	}

	/// `cv::Point2d`
	pub fn cv_point2d() -> Typedef<'tu, 'ge> {
		Typedef::new_desc(Self::new("cv::Point2d", "core", TypeRefDesc::cv_point_()))
	}

	/// `cv::Point3i`
	pub fn cv_point3i() -> Typedef<'tu, 'ge> {
		Typedef::new_desc(Self::new("cv::Point3i", "core", TypeRefDesc::cv_point_()))
	}

	/// `cv::Point3f`
	pub fn cv_point3f() -> Typedef<'tu, 'ge> {
		Typedef::new_desc(Self::new("cv::Point3f", "core", TypeRefDesc::cv_point_()))
	}

	/// `cv::Point3d`
	pub fn cv_point3d() -> Typedef<'tu, 'ge> {
		Typedef::new_desc(Self::new("cv::Point3d", "core", TypeRefDesc::cv_point_()))
	}

	/// `cv::Vec2f`
	pub fn cv_vec2f() -> Typedef<'tu, 'ge> {
		Typedef::new_desc(Self::new("cv::Vec2f", "core", TypeRefDesc::cv_vec()))
	}

	/// `cv::Vec2d`
	pub fn cv_vec2d() -> Typedef<'tu, 'ge> {
		Typedef::new_desc(Self::new("cv::Vec2d", "core", TypeRefDesc::cv_vec()))
	}

	/// `cv::Vec3f`
	pub fn cv_vec3f() -> Typedef<'tu, 'ge> {
		Typedef::new_desc(Self::new("cv::Vec3f", "core", TypeRefDesc::cv_vec()))
	}

	/// `cv::Vec3d`
	pub fn cv_vec3d() -> Typedef<'tu, 'ge> {
		Typedef::new_desc(Self::new("cv::Vec3d", "core", TypeRefDesc::cv_vec()))
	}
}
