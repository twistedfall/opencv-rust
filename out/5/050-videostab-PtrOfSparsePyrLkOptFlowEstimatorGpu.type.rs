ptr_extern! { crate::videostab::SparsePyrLkOptFlowEstimatorGpu,
	cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorGpuG_new_null_const, cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorGpuG_delete, cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorGpuG_getInnerPtr_const, cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorGpuG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::SparsePyrLkOptFlowEstimatorGpu, cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorGpuG_new_const_SparsePyrLkOptFlowEstimatorGpu }
impl core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimatorGpu> {
	#[inline] pub fn as_raw_PtrOfSparsePyrLkOptFlowEstimatorGpu(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSparsePyrLkOptFlowEstimatorGpu(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::SparsePyrLkOptFlowEstimatorGpuTraitConst for core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimatorGpu> {
	#[inline] fn as_raw_SparsePyrLkOptFlowEstimatorGpu(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::SparsePyrLkOptFlowEstimatorGpuTrait for core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimatorGpu> {
	#[inline] fn as_raw_mut_SparsePyrLkOptFlowEstimatorGpu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::ISparseOptFlowEstimatorTraitConst for core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimatorGpu> {
	#[inline] fn as_raw_ISparseOptFlowEstimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ISparseOptFlowEstimatorTrait for core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimatorGpu> {
	#[inline] fn as_raw_mut_ISparseOptFlowEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimatorGpu>, core::Ptr<crate::videostab::ISparseOptFlowEstimator>, cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorGpuG_to_PtrOfISparseOptFlowEstimator }

impl crate::videostab::PyrLkOptFlowEstimatorBaseTraitConst for core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimatorGpu> {
	#[inline] fn as_raw_PyrLkOptFlowEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::PyrLkOptFlowEstimatorBaseTrait for core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimatorGpu> {
	#[inline] fn as_raw_mut_PyrLkOptFlowEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimatorGpu>, core::Ptr<crate::videostab::PyrLkOptFlowEstimatorBase>, cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorGpuG_to_PtrOfPyrLkOptFlowEstimatorBase }

impl std::fmt::Debug for core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimatorGpu> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSparsePyrLkOptFlowEstimatorGpu")
			.finish()
	}
}

