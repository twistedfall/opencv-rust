ptr_extern! { crate::rgbd::ColoredKinfu_ColoredKinFu,
	cv_PtrLcv_colored_kinfu_ColoredKinFuG_new_null_const, cv_PtrLcv_colored_kinfu_ColoredKinFuG_delete, cv_PtrLcv_colored_kinfu_ColoredKinFuG_getInnerPtr_const, cv_PtrLcv_colored_kinfu_ColoredKinFuG_getInnerPtrMut
}

impl core::Ptr<crate::rgbd::ColoredKinfu_ColoredKinFu> {
	#[inline] pub fn as_raw_PtrOfColoredKinfu_ColoredKinFu(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfColoredKinfu_ColoredKinFu(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rgbd::ColoredKinfu_ColoredKinFuTraitConst for core::Ptr<crate::rgbd::ColoredKinfu_ColoredKinFu> {
	#[inline] fn as_raw_ColoredKinfu_ColoredKinFu(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::ColoredKinfu_ColoredKinFuTrait for core::Ptr<crate::rgbd::ColoredKinfu_ColoredKinFu> {
	#[inline] fn as_raw_mut_ColoredKinfu_ColoredKinFu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::rgbd::ColoredKinfu_ColoredKinFu> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfColoredKinfu_ColoredKinFu")
			.finish()
	}
}

