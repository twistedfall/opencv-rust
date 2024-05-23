ptr_extern! { crate::rgbd::OdometryFrame,
	cv_PtrLcv_rgbd_OdometryFrameG_new_null_const, cv_PtrLcv_rgbd_OdometryFrameG_delete, cv_PtrLcv_rgbd_OdometryFrameG_getInnerPtr_const, cv_PtrLcv_rgbd_OdometryFrameG_getInnerPtrMut
}

ptr_extern_ctor! { crate::rgbd::OdometryFrame, cv_PtrLcv_rgbd_OdometryFrameG_new_const_OdometryFrame }
impl core::Ptr<crate::rgbd::OdometryFrame> {
	#[inline] pub fn as_raw_PtrOfOdometryFrame(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfOdometryFrame(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rgbd::OdometryFrameTraitConst for core::Ptr<crate::rgbd::OdometryFrame> {
	#[inline] fn as_raw_OdometryFrame(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::OdometryFrameTrait for core::Ptr<crate::rgbd::OdometryFrame> {
	#[inline] fn as_raw_mut_OdometryFrame(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::rgbd::RgbdFrameTraitConst for core::Ptr<crate::rgbd::OdometryFrame> {
	#[inline] fn as_raw_RgbdFrame(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::RgbdFrameTrait for core::Ptr<crate::rgbd::OdometryFrame> {
	#[inline] fn as_raw_mut_RgbdFrame(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::rgbd::OdometryFrame>, core::Ptr<crate::rgbd::RgbdFrame>, cv_PtrLcv_rgbd_OdometryFrameG_to_PtrOfRgbdFrame }

impl std::fmt::Debug for core::Ptr<crate::rgbd::OdometryFrame> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfOdometryFrame")
			.field("pyramid_image", &crate::rgbd::OdometryFrameTraitConst::pyramid_image(self))
			.field("pyramid_depth", &crate::rgbd::OdometryFrameTraitConst::pyramid_depth(self))
			.field("pyramid_mask", &crate::rgbd::OdometryFrameTraitConst::pyramid_mask(self))
			.field("pyramid_cloud", &crate::rgbd::OdometryFrameTraitConst::pyramid_cloud(self))
			.field("pyramid_d_i_dx", &crate::rgbd::OdometryFrameTraitConst::pyramid_d_i_dx(self))
			.field("pyramid_d_i_dy", &crate::rgbd::OdometryFrameTraitConst::pyramid_d_i_dy(self))
			.field("pyramid_textured_mask", &crate::rgbd::OdometryFrameTraitConst::pyramid_textured_mask(self))
			.field("pyramid_normals", &crate::rgbd::OdometryFrameTraitConst::pyramid_normals(self))
			.field("pyramid_normals_mask", &crate::rgbd::OdometryFrameTraitConst::pyramid_normals_mask(self))
			.field("id", &crate::rgbd::RgbdFrameTraitConst::id(self))
			.field("image", &crate::rgbd::RgbdFrameTraitConst::image(self))
			.field("depth", &crate::rgbd::RgbdFrameTraitConst::depth(self))
			.field("mask", &crate::rgbd::RgbdFrameTraitConst::mask(self))
			.field("normals", &crate::rgbd::RgbdFrameTraitConst::normals(self))
			.finish()
	}
}

