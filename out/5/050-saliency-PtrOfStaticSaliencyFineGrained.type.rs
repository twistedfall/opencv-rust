ptr_extern! { crate::saliency::StaticSaliencyFineGrained,
	cv_PtrLcv_saliency_StaticSaliencyFineGrainedG_new_null_const, cv_PtrLcv_saliency_StaticSaliencyFineGrainedG_delete, cv_PtrLcv_saliency_StaticSaliencyFineGrainedG_getInnerPtr_const, cv_PtrLcv_saliency_StaticSaliencyFineGrainedG_getInnerPtrMut
}

ptr_extern_ctor! { crate::saliency::StaticSaliencyFineGrained, cv_PtrLcv_saliency_StaticSaliencyFineGrainedG_new_const_StaticSaliencyFineGrained }
impl core::Ptr<crate::saliency::StaticSaliencyFineGrained> {
	#[inline] pub fn as_raw_PtrOfStaticSaliencyFineGrained(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfStaticSaliencyFineGrained(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::saliency::StaticSaliencyFineGrainedTraitConst for core::Ptr<crate::saliency::StaticSaliencyFineGrained> {
	#[inline] fn as_raw_StaticSaliencyFineGrained(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::saliency::StaticSaliencyFineGrainedTrait for core::Ptr<crate::saliency::StaticSaliencyFineGrained> {
	#[inline] fn as_raw_mut_StaticSaliencyFineGrained(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::saliency::StaticSaliencyFineGrained> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::saliency::StaticSaliencyFineGrained> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::saliency::StaticSaliencyFineGrained>, core::Ptr<core::Algorithm>, cv_PtrLcv_saliency_StaticSaliencyFineGrainedG_to_PtrOfAlgorithm }

impl crate::saliency::SaliencyTraitConst for core::Ptr<crate::saliency::StaticSaliencyFineGrained> {
	#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::saliency::SaliencyTrait for core::Ptr<crate::saliency::StaticSaliencyFineGrained> {
	#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::saliency::StaticSaliencyFineGrained>, core::Ptr<crate::saliency::Saliency>, cv_PtrLcv_saliency_StaticSaliencyFineGrainedG_to_PtrOfSaliency }

impl crate::saliency::StaticSaliencyTraitConst for core::Ptr<crate::saliency::StaticSaliencyFineGrained> {
	#[inline] fn as_raw_StaticSaliency(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::saliency::StaticSaliencyTrait for core::Ptr<crate::saliency::StaticSaliencyFineGrained> {
	#[inline] fn as_raw_mut_StaticSaliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::saliency::StaticSaliencyFineGrained>, core::Ptr<crate::saliency::StaticSaliency>, cv_PtrLcv_saliency_StaticSaliencyFineGrainedG_to_PtrOfStaticSaliency }

impl std::fmt::Debug for core::Ptr<crate::saliency::StaticSaliencyFineGrained> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfStaticSaliencyFineGrained")
			.finish()
	}
}

