ptr_extern! { crate::xobjdetect::BaseCascadeClassifier_MaskGenerator,
	cv_PtrLcv_BaseCascadeClassifier_MaskGeneratorG_new_null_const, cv_PtrLcv_BaseCascadeClassifier_MaskGeneratorG_delete, cv_PtrLcv_BaseCascadeClassifier_MaskGeneratorG_getInnerPtr_const, cv_PtrLcv_BaseCascadeClassifier_MaskGeneratorG_getInnerPtrMut
}

impl core::Ptr<crate::xobjdetect::BaseCascadeClassifier_MaskGenerator> {
	#[inline] pub fn as_raw_PtrOfBaseCascadeClassifier_MaskGenerator(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBaseCascadeClassifier_MaskGenerator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::xobjdetect::BaseCascadeClassifier_MaskGeneratorTraitConst for core::Ptr<crate::xobjdetect::BaseCascadeClassifier_MaskGenerator> {
	#[inline] fn as_raw_BaseCascadeClassifier_MaskGenerator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xobjdetect::BaseCascadeClassifier_MaskGeneratorTrait for core::Ptr<crate::xobjdetect::BaseCascadeClassifier_MaskGenerator> {
	#[inline] fn as_raw_mut_BaseCascadeClassifier_MaskGenerator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::xobjdetect::BaseCascadeClassifier_MaskGenerator> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfBaseCascadeClassifier_MaskGenerator")
			.finish()
	}
}

