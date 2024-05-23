ptr_extern! { crate::text::OCRBeamSearchDecoder_ClassifierCallback,
	cv_PtrLcv_text_OCRBeamSearchDecoder_ClassifierCallbackG_new_null_const, cv_PtrLcv_text_OCRBeamSearchDecoder_ClassifierCallbackG_delete, cv_PtrLcv_text_OCRBeamSearchDecoder_ClassifierCallbackG_getInnerPtr_const, cv_PtrLcv_text_OCRBeamSearchDecoder_ClassifierCallbackG_getInnerPtrMut
}

ptr_extern_ctor! { crate::text::OCRBeamSearchDecoder_ClassifierCallback, cv_PtrLcv_text_OCRBeamSearchDecoder_ClassifierCallbackG_new_const_ClassifierCallback }
impl core::Ptr<crate::text::OCRBeamSearchDecoder_ClassifierCallback> {
	#[inline] pub fn as_raw_PtrOfOCRBeamSearchDecoder_ClassifierCallback(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfOCRBeamSearchDecoder_ClassifierCallback(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::text::OCRBeamSearchDecoder_ClassifierCallbackTraitConst for core::Ptr<crate::text::OCRBeamSearchDecoder_ClassifierCallback> {
	#[inline] fn as_raw_OCRBeamSearchDecoder_ClassifierCallback(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::OCRBeamSearchDecoder_ClassifierCallbackTrait for core::Ptr<crate::text::OCRBeamSearchDecoder_ClassifierCallback> {
	#[inline] fn as_raw_mut_OCRBeamSearchDecoder_ClassifierCallback(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::text::OCRBeamSearchDecoder_ClassifierCallback> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfOCRBeamSearchDecoder_ClassifierCallback")
			.finish()
	}
}

