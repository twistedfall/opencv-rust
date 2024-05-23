ptr_extern! { crate::videostab::IOutlierRejector,
	cv_PtrLcv_videostab_IOutlierRejectorG_new_null_const, cv_PtrLcv_videostab_IOutlierRejectorG_delete, cv_PtrLcv_videostab_IOutlierRejectorG_getInnerPtr_const, cv_PtrLcv_videostab_IOutlierRejectorG_getInnerPtrMut
}

impl core::Ptr<crate::videostab::IOutlierRejector> {
	#[inline] pub fn as_raw_PtrOfIOutlierRejector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfIOutlierRejector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::IOutlierRejectorTraitConst for core::Ptr<crate::videostab::IOutlierRejector> {
	#[inline] fn as_raw_IOutlierRejector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IOutlierRejectorTrait for core::Ptr<crate::videostab::IOutlierRejector> {
	#[inline] fn as_raw_mut_IOutlierRejector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::videostab::IOutlierRejector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfIOutlierRejector")
			.finish()
	}
}

