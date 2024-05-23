ptr_extern! { crate::text::OCRTesseract,
	cv_PtrLcv_text_OCRTesseractG_new_null_const, cv_PtrLcv_text_OCRTesseractG_delete, cv_PtrLcv_text_OCRTesseractG_getInnerPtr_const, cv_PtrLcv_text_OCRTesseractG_getInnerPtrMut
}

impl core::Ptr<crate::text::OCRTesseract> {
	#[inline] pub fn as_raw_PtrOfOCRTesseract(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfOCRTesseract(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::text::OCRTesseractTraitConst for core::Ptr<crate::text::OCRTesseract> {
	#[inline] fn as_raw_OCRTesseract(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::OCRTesseractTrait for core::Ptr<crate::text::OCRTesseract> {
	#[inline] fn as_raw_mut_OCRTesseract(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::text::BaseOCRTraitConst for core::Ptr<crate::text::OCRTesseract> {
	#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::BaseOCRTrait for core::Ptr<crate::text::OCRTesseract> {
	#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::text::OCRTesseract>, core::Ptr<crate::text::BaseOCR>, cv_PtrLcv_text_OCRTesseractG_to_PtrOfBaseOCR }

impl std::fmt::Debug for core::Ptr<crate::text::OCRTesseract> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfOCRTesseract")
			.finish()
	}
}

