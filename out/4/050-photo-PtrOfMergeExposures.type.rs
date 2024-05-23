ptr_extern! { crate::photo::MergeExposures,
	cv_PtrLcv_MergeExposuresG_new_null_const, cv_PtrLcv_MergeExposuresG_delete, cv_PtrLcv_MergeExposuresG_getInnerPtr_const, cv_PtrLcv_MergeExposuresG_getInnerPtrMut
}

impl core::Ptr<crate::photo::MergeExposures> {
	#[inline] pub fn as_raw_PtrOfMergeExposures(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMergeExposures(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::photo::MergeExposuresTraitConst for core::Ptr<crate::photo::MergeExposures> {
	#[inline] fn as_raw_MergeExposures(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::MergeExposuresTrait for core::Ptr<crate::photo::MergeExposures> {
	#[inline] fn as_raw_mut_MergeExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::photo::MergeExposures> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::photo::MergeExposures> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::photo::MergeExposures>, core::Ptr<core::Algorithm>, cv_PtrLcv_MergeExposuresG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::photo::MergeExposures> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMergeExposures")
			.finish()
	}
}

