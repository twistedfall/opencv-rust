ptr_extern! { crate::text::OCRHMMDecoder_ClassifierCallback,
	cv_PtrLcv_text_OCRHMMDecoder_ClassifierCallbackG_new_null_const, cv_PtrLcv_text_OCRHMMDecoder_ClassifierCallbackG_delete, cv_PtrLcv_text_OCRHMMDecoder_ClassifierCallbackG_getInnerPtr_const, cv_PtrLcv_text_OCRHMMDecoder_ClassifierCallbackG_getInnerPtrMut
}

ptr_extern_ctor! { crate::text::OCRHMMDecoder_ClassifierCallback, cv_PtrLcv_text_OCRHMMDecoder_ClassifierCallbackG_new_const_ClassifierCallback }
impl core::Ptr<crate::text::OCRHMMDecoder_ClassifierCallback> {
	#[inline] pub fn as_raw_PtrOfOCRHMMDecoder_ClassifierCallback(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfOCRHMMDecoder_ClassifierCallback(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::text::OCRHMMDecoder_ClassifierCallbackTraitConst for core::Ptr<crate::text::OCRHMMDecoder_ClassifierCallback> {
	#[inline] fn as_raw_OCRHMMDecoder_ClassifierCallback(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::OCRHMMDecoder_ClassifierCallbackTrait for core::Ptr<crate::text::OCRHMMDecoder_ClassifierCallback> {
	#[inline] fn as_raw_mut_OCRHMMDecoder_ClassifierCallback(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::text::OCRHMMDecoder_ClassifierCallback> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfOCRHMMDecoder_ClassifierCallback")
			.finish()
	}
}

