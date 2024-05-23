ptr_extern! { crate::videostab::PyrLkOptFlowEstimatorBase,
	cv_PtrLcv_videostab_PyrLkOptFlowEstimatorBaseG_new_null_const, cv_PtrLcv_videostab_PyrLkOptFlowEstimatorBaseG_delete, cv_PtrLcv_videostab_PyrLkOptFlowEstimatorBaseG_getInnerPtr_const, cv_PtrLcv_videostab_PyrLkOptFlowEstimatorBaseG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::PyrLkOptFlowEstimatorBase, cv_PtrLcv_videostab_PyrLkOptFlowEstimatorBaseG_new_const_PyrLkOptFlowEstimatorBase }
impl core::Ptr<crate::videostab::PyrLkOptFlowEstimatorBase> {
	#[inline] pub fn as_raw_PtrOfPyrLkOptFlowEstimatorBase(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPyrLkOptFlowEstimatorBase(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::PyrLkOptFlowEstimatorBaseTraitConst for core::Ptr<crate::videostab::PyrLkOptFlowEstimatorBase> {
	#[inline] fn as_raw_PyrLkOptFlowEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::PyrLkOptFlowEstimatorBaseTrait for core::Ptr<crate::videostab::PyrLkOptFlowEstimatorBase> {
	#[inline] fn as_raw_mut_PyrLkOptFlowEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::videostab::PyrLkOptFlowEstimatorBase> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfPyrLkOptFlowEstimatorBase")
			.finish()
	}
}

