ptr_extern! { crate::videostab::SparsePyrLkOptFlowEstimator,
	cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorG_new_null_const, cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorG_delete, cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorG_getInnerPtr_const, cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::SparsePyrLkOptFlowEstimator, cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorG_new_const_SparsePyrLkOptFlowEstimator }
impl core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimator> {
	#[inline] pub fn as_raw_PtrOfSparsePyrLkOptFlowEstimator(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSparsePyrLkOptFlowEstimator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::SparsePyrLkOptFlowEstimatorTraitConst for core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimator> {
	#[inline] fn as_raw_SparsePyrLkOptFlowEstimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::SparsePyrLkOptFlowEstimatorTrait for core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimator> {
	#[inline] fn as_raw_mut_SparsePyrLkOptFlowEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::ISparseOptFlowEstimatorTraitConst for core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimator> {
	#[inline] fn as_raw_ISparseOptFlowEstimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ISparseOptFlowEstimatorTrait for core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimator> {
	#[inline] fn as_raw_mut_ISparseOptFlowEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimator>, core::Ptr<crate::videostab::ISparseOptFlowEstimator>, cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorG_to_PtrOfISparseOptFlowEstimator }

impl crate::videostab::PyrLkOptFlowEstimatorBaseTraitConst for core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimator> {
	#[inline] fn as_raw_PyrLkOptFlowEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::PyrLkOptFlowEstimatorBaseTrait for core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimator> {
	#[inline] fn as_raw_mut_PyrLkOptFlowEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimator>, core::Ptr<crate::videostab::PyrLkOptFlowEstimatorBase>, cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorG_to_PtrOfPyrLkOptFlowEstimatorBase }

impl std::fmt::Debug for core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimator> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSparsePyrLkOptFlowEstimator")
			.finish()
	}
}

