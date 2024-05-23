ptr_extern! { crate::text::TextDetector,
	cv_PtrLcv_text_TextDetectorG_new_null_const, cv_PtrLcv_text_TextDetectorG_delete, cv_PtrLcv_text_TextDetectorG_getInnerPtr_const, cv_PtrLcv_text_TextDetectorG_getInnerPtrMut
}

impl core::Ptr<crate::text::TextDetector> {
	#[inline] pub fn as_raw_PtrOfTextDetector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTextDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::text::TextDetectorTraitConst for core::Ptr<crate::text::TextDetector> {
	#[inline] fn as_raw_TextDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::TextDetectorTrait for core::Ptr<crate::text::TextDetector> {
	#[inline] fn as_raw_mut_TextDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::text::TextDetector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTextDetector")
			.finish()
	}
}

