ptr_extern! { crate::xobjdetect::BaseCascadeClassifier,
	cv_PtrLcv_BaseCascadeClassifierG_new_null_const, cv_PtrLcv_BaseCascadeClassifierG_delete, cv_PtrLcv_BaseCascadeClassifierG_getInnerPtr_const, cv_PtrLcv_BaseCascadeClassifierG_getInnerPtrMut
}

impl core::Ptr<crate::xobjdetect::BaseCascadeClassifier> {
	#[inline] pub fn as_raw_PtrOfBaseCascadeClassifier(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBaseCascadeClassifier(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::xobjdetect::BaseCascadeClassifierTraitConst for core::Ptr<crate::xobjdetect::BaseCascadeClassifier> {
	#[inline] fn as_raw_BaseCascadeClassifier(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xobjdetect::BaseCascadeClassifierTrait for core::Ptr<crate::xobjdetect::BaseCascadeClassifier> {
	#[inline] fn as_raw_mut_BaseCascadeClassifier(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::xobjdetect::BaseCascadeClassifier> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::xobjdetect::BaseCascadeClassifier> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xobjdetect::BaseCascadeClassifier>, core::Ptr<core::Algorithm>, cv_PtrLcv_BaseCascadeClassifierG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::xobjdetect::BaseCascadeClassifier> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfBaseCascadeClassifier")
			.finish()
	}
}

