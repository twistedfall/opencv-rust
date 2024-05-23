ptr_extern! { crate::videostab::InpainterBase,
	cv_PtrLcv_videostab_InpainterBaseG_new_null_const, cv_PtrLcv_videostab_InpainterBaseG_delete, cv_PtrLcv_videostab_InpainterBaseG_getInnerPtr_const, cv_PtrLcv_videostab_InpainterBaseG_getInnerPtrMut
}

impl core::Ptr<crate::videostab::InpainterBase> {
	#[inline] pub fn as_raw_PtrOfInpainterBase(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfInpainterBase(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::InpainterBaseTraitConst for core::Ptr<crate::videostab::InpainterBase> {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::InpainterBaseTrait for core::Ptr<crate::videostab::InpainterBase> {
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::videostab::InpainterBase> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfInpainterBase")
			.finish()
	}
}

