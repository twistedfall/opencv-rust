ptr_extern! { crate::videostab::IFrameSource,
	cv_PtrLcv_videostab_IFrameSourceG_new_null_const, cv_PtrLcv_videostab_IFrameSourceG_delete, cv_PtrLcv_videostab_IFrameSourceG_getInnerPtr_const, cv_PtrLcv_videostab_IFrameSourceG_getInnerPtrMut
}

impl core::Ptr<crate::videostab::IFrameSource> {
	#[inline] pub fn as_raw_PtrOfIFrameSource(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfIFrameSource(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::IFrameSourceTraitConst for core::Ptr<crate::videostab::IFrameSource> {
	#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IFrameSourceTrait for core::Ptr<crate::videostab::IFrameSource> {
	#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::videostab::IFrameSource> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfIFrameSource")
			.finish()
	}
}

