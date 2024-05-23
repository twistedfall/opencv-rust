ptr_extern! { crate::bgsegm::SyntheticSequenceGenerator,
	cv_PtrLcv_bgsegm_SyntheticSequenceGeneratorG_new_null_const, cv_PtrLcv_bgsegm_SyntheticSequenceGeneratorG_delete, cv_PtrLcv_bgsegm_SyntheticSequenceGeneratorG_getInnerPtr_const, cv_PtrLcv_bgsegm_SyntheticSequenceGeneratorG_getInnerPtrMut
}

ptr_extern_ctor! { crate::bgsegm::SyntheticSequenceGenerator, cv_PtrLcv_bgsegm_SyntheticSequenceGeneratorG_new_const_SyntheticSequenceGenerator }
impl core::Ptr<crate::bgsegm::SyntheticSequenceGenerator> {
	#[inline] pub fn as_raw_PtrOfSyntheticSequenceGenerator(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSyntheticSequenceGenerator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::bgsegm::SyntheticSequenceGeneratorTraitConst for core::Ptr<crate::bgsegm::SyntheticSequenceGenerator> {
	#[inline] fn as_raw_SyntheticSequenceGenerator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::bgsegm::SyntheticSequenceGeneratorTrait for core::Ptr<crate::bgsegm::SyntheticSequenceGenerator> {
	#[inline] fn as_raw_mut_SyntheticSequenceGenerator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::bgsegm::SyntheticSequenceGenerator> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::bgsegm::SyntheticSequenceGenerator> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::bgsegm::SyntheticSequenceGenerator>, core::Ptr<core::Algorithm>, cv_PtrLcv_bgsegm_SyntheticSequenceGeneratorG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::bgsegm::SyntheticSequenceGenerator> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSyntheticSequenceGenerator")
			.finish()
	}
}

