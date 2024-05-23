ptr_extern! { crate::saliency::Objectness,
	cv_PtrLcv_saliency_ObjectnessG_new_null_const, cv_PtrLcv_saliency_ObjectnessG_delete, cv_PtrLcv_saliency_ObjectnessG_getInnerPtr_const, cv_PtrLcv_saliency_ObjectnessG_getInnerPtrMut
}

impl core::Ptr<crate::saliency::Objectness> {
	#[inline] pub fn as_raw_PtrOfObjectness(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfObjectness(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::saliency::ObjectnessTraitConst for core::Ptr<crate::saliency::Objectness> {
	#[inline] fn as_raw_Objectness(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::saliency::ObjectnessTrait for core::Ptr<crate::saliency::Objectness> {
	#[inline] fn as_raw_mut_Objectness(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::saliency::Objectness> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::saliency::Objectness> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::saliency::Objectness>, core::Ptr<core::Algorithm>, cv_PtrLcv_saliency_ObjectnessG_to_PtrOfAlgorithm }

impl crate::saliency::SaliencyTraitConst for core::Ptr<crate::saliency::Objectness> {
	#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::saliency::SaliencyTrait for core::Ptr<crate::saliency::Objectness> {
	#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::saliency::Objectness>, core::Ptr<crate::saliency::Saliency>, cv_PtrLcv_saliency_ObjectnessG_to_PtrOfSaliency }

impl std::fmt::Debug for core::Ptr<crate::saliency::Objectness> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfObjectness")
			.finish()
	}
}

