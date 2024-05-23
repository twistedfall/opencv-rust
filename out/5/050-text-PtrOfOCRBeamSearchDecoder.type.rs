ptr_extern! { crate::text::OCRBeamSearchDecoder,
	cv_PtrLcv_text_OCRBeamSearchDecoderG_new_null_const, cv_PtrLcv_text_OCRBeamSearchDecoderG_delete, cv_PtrLcv_text_OCRBeamSearchDecoderG_getInnerPtr_const, cv_PtrLcv_text_OCRBeamSearchDecoderG_getInnerPtrMut
}

ptr_extern_ctor! { crate::text::OCRBeamSearchDecoder, cv_PtrLcv_text_OCRBeamSearchDecoderG_new_const_OCRBeamSearchDecoder }
impl core::Ptr<crate::text::OCRBeamSearchDecoder> {
	#[inline] pub fn as_raw_PtrOfOCRBeamSearchDecoder(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfOCRBeamSearchDecoder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::text::OCRBeamSearchDecoderTraitConst for core::Ptr<crate::text::OCRBeamSearchDecoder> {
	#[inline] fn as_raw_OCRBeamSearchDecoder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::OCRBeamSearchDecoderTrait for core::Ptr<crate::text::OCRBeamSearchDecoder> {
	#[inline] fn as_raw_mut_OCRBeamSearchDecoder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::text::BaseOCRTraitConst for core::Ptr<crate::text::OCRBeamSearchDecoder> {
	#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::BaseOCRTrait for core::Ptr<crate::text::OCRBeamSearchDecoder> {
	#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::text::OCRBeamSearchDecoder>, core::Ptr<crate::text::BaseOCR>, cv_PtrLcv_text_OCRBeamSearchDecoderG_to_PtrOfBaseOCR }

impl std::fmt::Debug for core::Ptr<crate::text::OCRBeamSearchDecoder> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfOCRBeamSearchDecoder")
			.finish()
	}
}

