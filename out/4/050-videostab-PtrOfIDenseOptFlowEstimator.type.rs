ptr_extern! { crate::videostab::IDenseOptFlowEstimator,
	cv_PtrLcv_videostab_IDenseOptFlowEstimatorG_new_null_const, cv_PtrLcv_videostab_IDenseOptFlowEstimatorG_delete, cv_PtrLcv_videostab_IDenseOptFlowEstimatorG_getInnerPtr_const, cv_PtrLcv_videostab_IDenseOptFlowEstimatorG_getInnerPtrMut
}

impl core::Ptr<crate::videostab::IDenseOptFlowEstimator> {
	#[inline] pub fn as_raw_PtrOfIDenseOptFlowEstimator(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfIDenseOptFlowEstimator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::IDenseOptFlowEstimatorTraitConst for core::Ptr<crate::videostab::IDenseOptFlowEstimator> {
	#[inline] fn as_raw_IDenseOptFlowEstimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IDenseOptFlowEstimatorTrait for core::Ptr<crate::videostab::IDenseOptFlowEstimator> {
	#[inline] fn as_raw_mut_IDenseOptFlowEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::videostab::IDenseOptFlowEstimator> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfIDenseOptFlowEstimator")
			.finish()
	}
}

