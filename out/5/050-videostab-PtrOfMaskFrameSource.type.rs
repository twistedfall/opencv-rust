ptr_extern! { crate::videostab::MaskFrameSource,
	cv_PtrLcv_videostab_MaskFrameSourceG_new_null_const, cv_PtrLcv_videostab_MaskFrameSourceG_delete, cv_PtrLcv_videostab_MaskFrameSourceG_getInnerPtr_const, cv_PtrLcv_videostab_MaskFrameSourceG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::MaskFrameSource, cv_PtrLcv_videostab_MaskFrameSourceG_new_const_MaskFrameSource }
impl core::Ptr<crate::videostab::MaskFrameSource> {
	#[inline] pub fn as_raw_PtrOfMaskFrameSource(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMaskFrameSource(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::MaskFrameSourceTraitConst for core::Ptr<crate::videostab::MaskFrameSource> {
	#[inline] fn as_raw_MaskFrameSource(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::MaskFrameSourceTrait for core::Ptr<crate::videostab::MaskFrameSource> {
	#[inline] fn as_raw_mut_MaskFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::IFrameSourceTraitConst for core::Ptr<crate::videostab::MaskFrameSource> {
	#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IFrameSourceTrait for core::Ptr<crate::videostab::MaskFrameSource> {
	#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::MaskFrameSource>, core::Ptr<crate::videostab::IFrameSource>, cv_PtrLcv_videostab_MaskFrameSourceG_to_PtrOfIFrameSource }

impl std::fmt::Debug for core::Ptr<crate::videostab::MaskFrameSource> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMaskFrameSource")
			.finish()
	}
}

