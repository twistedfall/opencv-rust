ptr_extern! { crate::mod_3d::SACSegmentation,
	cv_PtrLcv_SACSegmentationG_new_null_const, cv_PtrLcv_SACSegmentationG_delete, cv_PtrLcv_SACSegmentationG_getInnerPtr_const, cv_PtrLcv_SACSegmentationG_getInnerPtrMut
}

impl core::Ptr<crate::mod_3d::SACSegmentation> {
	#[inline] pub fn as_raw_PtrOfSACSegmentation(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSACSegmentation(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::mod_3d::SACSegmentationTraitConst for core::Ptr<crate::mod_3d::SACSegmentation> {
	#[inline] fn as_raw_SACSegmentation(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::mod_3d::SACSegmentationTrait for core::Ptr<crate::mod_3d::SACSegmentation> {
	#[inline] fn as_raw_mut_SACSegmentation(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::mod_3d::SACSegmentation> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSACSegmentation")
			.finish()
	}
}

