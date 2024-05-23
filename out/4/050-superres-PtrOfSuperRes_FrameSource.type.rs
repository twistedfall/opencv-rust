ptr_extern! { crate::superres::SuperRes_FrameSource,
	cv_PtrLcv_superres_FrameSourceG_new_null_const, cv_PtrLcv_superres_FrameSourceG_delete, cv_PtrLcv_superres_FrameSourceG_getInnerPtr_const, cv_PtrLcv_superres_FrameSourceG_getInnerPtrMut
}

impl core::Ptr<crate::superres::SuperRes_FrameSource> {
	#[inline] pub fn as_raw_PtrOfSuperRes_FrameSource(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSuperRes_FrameSource(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::superres::SuperRes_FrameSourceTraitConst for core::Ptr<crate::superres::SuperRes_FrameSource> {
	#[inline] fn as_raw_SuperRes_FrameSource(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::superres::SuperRes_FrameSourceTrait for core::Ptr<crate::superres::SuperRes_FrameSource> {
	#[inline] fn as_raw_mut_SuperRes_FrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::superres::SuperRes_FrameSource> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSuperRes_FrameSource")
			.finish()
	}
}

