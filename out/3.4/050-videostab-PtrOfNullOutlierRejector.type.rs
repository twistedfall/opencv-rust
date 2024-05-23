ptr_extern! { crate::videostab::NullOutlierRejector,
	cv_PtrLcv_videostab_NullOutlierRejectorG_new_null_const, cv_PtrLcv_videostab_NullOutlierRejectorG_delete, cv_PtrLcv_videostab_NullOutlierRejectorG_getInnerPtr_const, cv_PtrLcv_videostab_NullOutlierRejectorG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::NullOutlierRejector, cv_PtrLcv_videostab_NullOutlierRejectorG_new_const_NullOutlierRejector }
impl core::Ptr<crate::videostab::NullOutlierRejector> {
	#[inline] pub fn as_raw_PtrOfNullOutlierRejector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfNullOutlierRejector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::NullOutlierRejectorTraitConst for core::Ptr<crate::videostab::NullOutlierRejector> {
	#[inline] fn as_raw_NullOutlierRejector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::NullOutlierRejectorTrait for core::Ptr<crate::videostab::NullOutlierRejector> {
	#[inline] fn as_raw_mut_NullOutlierRejector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::IOutlierRejectorTraitConst for core::Ptr<crate::videostab::NullOutlierRejector> {
	#[inline] fn as_raw_IOutlierRejector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IOutlierRejectorTrait for core::Ptr<crate::videostab::NullOutlierRejector> {
	#[inline] fn as_raw_mut_IOutlierRejector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::NullOutlierRejector>, core::Ptr<crate::videostab::IOutlierRejector>, cv_PtrLcv_videostab_NullOutlierRejectorG_to_PtrOfIOutlierRejector }

impl std::fmt::Debug for core::Ptr<crate::videostab::NullOutlierRejector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfNullOutlierRejector")
			.finish()
	}
}

