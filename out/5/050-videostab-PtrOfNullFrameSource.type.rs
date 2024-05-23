ptr_extern! { crate::videostab::NullFrameSource,
	cv_PtrLcv_videostab_NullFrameSourceG_new_null_const, cv_PtrLcv_videostab_NullFrameSourceG_delete, cv_PtrLcv_videostab_NullFrameSourceG_getInnerPtr_const, cv_PtrLcv_videostab_NullFrameSourceG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::NullFrameSource, cv_PtrLcv_videostab_NullFrameSourceG_new_const_NullFrameSource }
impl core::Ptr<crate::videostab::NullFrameSource> {
	#[inline] pub fn as_raw_PtrOfNullFrameSource(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfNullFrameSource(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::NullFrameSourceTraitConst for core::Ptr<crate::videostab::NullFrameSource> {
	#[inline] fn as_raw_NullFrameSource(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::NullFrameSourceTrait for core::Ptr<crate::videostab::NullFrameSource> {
	#[inline] fn as_raw_mut_NullFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::IFrameSourceTraitConst for core::Ptr<crate::videostab::NullFrameSource> {
	#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IFrameSourceTrait for core::Ptr<crate::videostab::NullFrameSource> {
	#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::NullFrameSource>, core::Ptr<crate::videostab::IFrameSource>, cv_PtrLcv_videostab_NullFrameSourceG_to_PtrOfIFrameSource }

impl std::fmt::Debug for core::Ptr<crate::videostab::NullFrameSource> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfNullFrameSource")
			.finish()
	}
}

