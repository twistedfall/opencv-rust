ptr_extern! { crate::text::TextDetectorCNN,
	cv_PtrLcv_text_TextDetectorCNNG_new_null_const, cv_PtrLcv_text_TextDetectorCNNG_delete, cv_PtrLcv_text_TextDetectorCNNG_getInnerPtr_const, cv_PtrLcv_text_TextDetectorCNNG_getInnerPtrMut
}

impl core::Ptr<crate::text::TextDetectorCNN> {
	#[inline] pub fn as_raw_PtrOfTextDetectorCNN(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTextDetectorCNN(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::text::TextDetectorCNNTraitConst for core::Ptr<crate::text::TextDetectorCNN> {
	#[inline] fn as_raw_TextDetectorCNN(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::TextDetectorCNNTrait for core::Ptr<crate::text::TextDetectorCNN> {
	#[inline] fn as_raw_mut_TextDetectorCNN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::text::TextDetectorTraitConst for core::Ptr<crate::text::TextDetectorCNN> {
	#[inline] fn as_raw_TextDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::TextDetectorTrait for core::Ptr<crate::text::TextDetectorCNN> {
	#[inline] fn as_raw_mut_TextDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::text::TextDetectorCNN>, core::Ptr<crate::text::TextDetector>, cv_PtrLcv_text_TextDetectorCNNG_to_PtrOfTextDetector }

impl std::fmt::Debug for core::Ptr<crate::text::TextDetectorCNN> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTextDetectorCNN")
			.finish()
	}
}

