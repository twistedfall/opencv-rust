ptr_extern! { crate::videostab::ISparseOptFlowEstimator,
	cv_PtrLcv_videostab_ISparseOptFlowEstimatorG_new_null_const, cv_PtrLcv_videostab_ISparseOptFlowEstimatorG_delete, cv_PtrLcv_videostab_ISparseOptFlowEstimatorG_getInnerPtr_const, cv_PtrLcv_videostab_ISparseOptFlowEstimatorG_getInnerPtrMut
}

impl core::Ptr<crate::videostab::ISparseOptFlowEstimator> {
	#[inline] pub fn as_raw_PtrOfISparseOptFlowEstimator(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfISparseOptFlowEstimator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::ISparseOptFlowEstimatorTraitConst for core::Ptr<crate::videostab::ISparseOptFlowEstimator> {
	#[inline] fn as_raw_ISparseOptFlowEstimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ISparseOptFlowEstimatorTrait for core::Ptr<crate::videostab::ISparseOptFlowEstimator> {
	#[inline] fn as_raw_mut_ISparseOptFlowEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::videostab::ISparseOptFlowEstimator> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfISparseOptFlowEstimator")
			.finish()
	}
}

