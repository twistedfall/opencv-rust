ptr_extern! { crate::rgbd::RgbdFrame,
	cv_PtrLcv_rgbd_RgbdFrameG_new_null_const, cv_PtrLcv_rgbd_RgbdFrameG_delete, cv_PtrLcv_rgbd_RgbdFrameG_getInnerPtr_const, cv_PtrLcv_rgbd_RgbdFrameG_getInnerPtrMut
}

ptr_extern_ctor! { crate::rgbd::RgbdFrame, cv_PtrLcv_rgbd_RgbdFrameG_new_const_RgbdFrame }
impl core::Ptr<crate::rgbd::RgbdFrame> {
	#[inline] pub fn as_raw_PtrOfRgbdFrame(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRgbdFrame(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rgbd::RgbdFrameTraitConst for core::Ptr<crate::rgbd::RgbdFrame> {
	#[inline] fn as_raw_RgbdFrame(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::RgbdFrameTrait for core::Ptr<crate::rgbd::RgbdFrame> {
	#[inline] fn as_raw_mut_RgbdFrame(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::rgbd::RgbdFrame> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfRgbdFrame")
			.field("id", &crate::rgbd::RgbdFrameTraitConst::id(self))
			.field("image", &crate::rgbd::RgbdFrameTraitConst::image(self))
			.field("depth", &crate::rgbd::RgbdFrameTraitConst::depth(self))
			.field("mask", &crate::rgbd::RgbdFrameTraitConst::mask(self))
			.field("normals", &crate::rgbd::RgbdFrameTraitConst::normals(self))
			.finish()
	}
}

