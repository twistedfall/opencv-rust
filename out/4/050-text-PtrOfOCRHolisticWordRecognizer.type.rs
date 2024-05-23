ptr_extern! { crate::text::OCRHolisticWordRecognizer,
	cv_PtrLcv_text_OCRHolisticWordRecognizerG_new_null_const, cv_PtrLcv_text_OCRHolisticWordRecognizerG_delete, cv_PtrLcv_text_OCRHolisticWordRecognizerG_getInnerPtr_const, cv_PtrLcv_text_OCRHolisticWordRecognizerG_getInnerPtrMut
}

impl core::Ptr<crate::text::OCRHolisticWordRecognizer> {
	#[inline] pub fn as_raw_PtrOfOCRHolisticWordRecognizer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfOCRHolisticWordRecognizer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::text::OCRHolisticWordRecognizerTraitConst for core::Ptr<crate::text::OCRHolisticWordRecognizer> {
	#[inline] fn as_raw_OCRHolisticWordRecognizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::OCRHolisticWordRecognizerTrait for core::Ptr<crate::text::OCRHolisticWordRecognizer> {
	#[inline] fn as_raw_mut_OCRHolisticWordRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::text::BaseOCRTraitConst for core::Ptr<crate::text::OCRHolisticWordRecognizer> {
	#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::BaseOCRTrait for core::Ptr<crate::text::OCRHolisticWordRecognizer> {
	#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::text::OCRHolisticWordRecognizer>, core::Ptr<crate::text::BaseOCR>, cv_PtrLcv_text_OCRHolisticWordRecognizerG_to_PtrOfBaseOCR }

impl std::fmt::Debug for core::Ptr<crate::text::OCRHolisticWordRecognizer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfOCRHolisticWordRecognizer")
			.finish()
	}
}

