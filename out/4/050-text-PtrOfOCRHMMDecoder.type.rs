ptr_extern! { crate::text::OCRHMMDecoder,
	cv_PtrLcv_text_OCRHMMDecoderG_new_null_const, cv_PtrLcv_text_OCRHMMDecoderG_delete, cv_PtrLcv_text_OCRHMMDecoderG_getInnerPtr_const, cv_PtrLcv_text_OCRHMMDecoderG_getInnerPtrMut
}

ptr_extern_ctor! { crate::text::OCRHMMDecoder, cv_PtrLcv_text_OCRHMMDecoderG_new_const_OCRHMMDecoder }
impl core::Ptr<crate::text::OCRHMMDecoder> {
	#[inline] pub fn as_raw_PtrOfOCRHMMDecoder(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfOCRHMMDecoder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::text::OCRHMMDecoderTraitConst for core::Ptr<crate::text::OCRHMMDecoder> {
	#[inline] fn as_raw_OCRHMMDecoder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::OCRHMMDecoderTrait for core::Ptr<crate::text::OCRHMMDecoder> {
	#[inline] fn as_raw_mut_OCRHMMDecoder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::text::BaseOCRTraitConst for core::Ptr<crate::text::OCRHMMDecoder> {
	#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::BaseOCRTrait for core::Ptr<crate::text::OCRHMMDecoder> {
	#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::text::OCRHMMDecoder>, core::Ptr<crate::text::BaseOCR>, cv_PtrLcv_text_OCRHMMDecoderG_to_PtrOfBaseOCR }

impl std::fmt::Debug for core::Ptr<crate::text::OCRHMMDecoder> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfOCRHMMDecoder")
			.finish()
	}
}

