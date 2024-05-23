ptr_extern! { crate::videostab::TranslationBasedLocalOutlierRejector,
	cv_PtrLcv_videostab_TranslationBasedLocalOutlierRejectorG_new_null_const, cv_PtrLcv_videostab_TranslationBasedLocalOutlierRejectorG_delete, cv_PtrLcv_videostab_TranslationBasedLocalOutlierRejectorG_getInnerPtr_const, cv_PtrLcv_videostab_TranslationBasedLocalOutlierRejectorG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::TranslationBasedLocalOutlierRejector, cv_PtrLcv_videostab_TranslationBasedLocalOutlierRejectorG_new_const_TranslationBasedLocalOutlierRejector }
impl core::Ptr<crate::videostab::TranslationBasedLocalOutlierRejector> {
	#[inline] pub fn as_raw_PtrOfTranslationBasedLocalOutlierRejector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTranslationBasedLocalOutlierRejector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::TranslationBasedLocalOutlierRejectorTraitConst for core::Ptr<crate::videostab::TranslationBasedLocalOutlierRejector> {
	#[inline] fn as_raw_TranslationBasedLocalOutlierRejector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::TranslationBasedLocalOutlierRejectorTrait for core::Ptr<crate::videostab::TranslationBasedLocalOutlierRejector> {
	#[inline] fn as_raw_mut_TranslationBasedLocalOutlierRejector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::IOutlierRejectorTraitConst for core::Ptr<crate::videostab::TranslationBasedLocalOutlierRejector> {
	#[inline] fn as_raw_IOutlierRejector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IOutlierRejectorTrait for core::Ptr<crate::videostab::TranslationBasedLocalOutlierRejector> {
	#[inline] fn as_raw_mut_IOutlierRejector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::TranslationBasedLocalOutlierRejector>, core::Ptr<crate::videostab::IOutlierRejector>, cv_PtrLcv_videostab_TranslationBasedLocalOutlierRejectorG_to_PtrOfIOutlierRejector }

impl std::fmt::Debug for core::Ptr<crate::videostab::TranslationBasedLocalOutlierRejector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTranslationBasedLocalOutlierRejector")
			.finish()
	}
}

