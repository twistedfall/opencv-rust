ptr_extern! { crate::tracking::TrackerSamplerAlgorithm,
	cv_PtrLcv_TrackerSamplerAlgorithmG_new_null_const, cv_PtrLcv_TrackerSamplerAlgorithmG_delete, cv_PtrLcv_TrackerSamplerAlgorithmG_getInnerPtr_const, cv_PtrLcv_TrackerSamplerAlgorithmG_getInnerPtrMut
}

impl core::Ptr<crate::tracking::TrackerSamplerAlgorithm> {
	#[inline] pub fn as_raw_PtrOfTrackerSamplerAlgorithm(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerSamplerAlgorithm(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::tracking::TrackerSamplerAlgorithmTraitConst for core::Ptr<crate::tracking::TrackerSamplerAlgorithm> {
	#[inline] fn as_raw_TrackerSamplerAlgorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerSamplerAlgorithmTrait for core::Ptr<crate::tracking::TrackerSamplerAlgorithm> {
	#[inline] fn as_raw_mut_TrackerSamplerAlgorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::tracking::TrackerSamplerAlgorithm> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrackerSamplerAlgorithm")
			.finish()
	}
}

