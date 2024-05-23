ptr_extern! { crate::saliency::Saliency,
	cv_PtrLcv_saliency_SaliencyG_new_null_const, cv_PtrLcv_saliency_SaliencyG_delete, cv_PtrLcv_saliency_SaliencyG_getInnerPtr_const, cv_PtrLcv_saliency_SaliencyG_getInnerPtrMut
}

impl core::Ptr<crate::saliency::Saliency> {
	#[inline] pub fn as_raw_PtrOfSaliency(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSaliency(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::saliency::SaliencyTraitConst for core::Ptr<crate::saliency::Saliency> {
	#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::saliency::SaliencyTrait for core::Ptr<crate::saliency::Saliency> {
	#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::saliency::Saliency> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::saliency::Saliency> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::saliency::Saliency>, core::Ptr<core::Algorithm>, cv_PtrLcv_saliency_SaliencyG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::saliency::Saliency> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSaliency")
			.finish()
	}
}

