ptr_extern! { crate::text::BaseOCR,
	cv_PtrLcv_text_BaseOCRG_new_null_const, cv_PtrLcv_text_BaseOCRG_delete, cv_PtrLcv_text_BaseOCRG_getInnerPtr_const, cv_PtrLcv_text_BaseOCRG_getInnerPtrMut
}

impl core::Ptr<crate::text::BaseOCR> {
	#[inline] pub fn as_raw_PtrOfBaseOCR(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBaseOCR(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::text::BaseOCRTraitConst for core::Ptr<crate::text::BaseOCR> {
	#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::BaseOCRTrait for core::Ptr<crate::text::BaseOCR> {
	#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::text::BaseOCR> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfBaseOCR")
			.finish()
	}
}

