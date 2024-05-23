ptr_extern! { crate::videostab::DensePyrLkOptFlowEstimatorGpu,
	cv_PtrLcv_videostab_DensePyrLkOptFlowEstimatorGpuG_new_null_const, cv_PtrLcv_videostab_DensePyrLkOptFlowEstimatorGpuG_delete, cv_PtrLcv_videostab_DensePyrLkOptFlowEstimatorGpuG_getInnerPtr_const, cv_PtrLcv_videostab_DensePyrLkOptFlowEstimatorGpuG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::DensePyrLkOptFlowEstimatorGpu, cv_PtrLcv_videostab_DensePyrLkOptFlowEstimatorGpuG_new_const_DensePyrLkOptFlowEstimatorGpu }
impl core::Ptr<crate::videostab::DensePyrLkOptFlowEstimatorGpu> {
	#[inline] pub fn as_raw_PtrOfDensePyrLkOptFlowEstimatorGpu(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDensePyrLkOptFlowEstimatorGpu(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::DensePyrLkOptFlowEstimatorGpuTraitConst for core::Ptr<crate::videostab::DensePyrLkOptFlowEstimatorGpu> {
	#[inline] fn as_raw_DensePyrLkOptFlowEstimatorGpu(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::DensePyrLkOptFlowEstimatorGpuTrait for core::Ptr<crate::videostab::DensePyrLkOptFlowEstimatorGpu> {
	#[inline] fn as_raw_mut_DensePyrLkOptFlowEstimatorGpu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::IDenseOptFlowEstimatorTraitConst for core::Ptr<crate::videostab::DensePyrLkOptFlowEstimatorGpu> {
	#[inline] fn as_raw_IDenseOptFlowEstimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IDenseOptFlowEstimatorTrait for core::Ptr<crate::videostab::DensePyrLkOptFlowEstimatorGpu> {
	#[inline] fn as_raw_mut_IDenseOptFlowEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::DensePyrLkOptFlowEstimatorGpu>, core::Ptr<crate::videostab::IDenseOptFlowEstimator>, cv_PtrLcv_videostab_DensePyrLkOptFlowEstimatorGpuG_to_PtrOfIDenseOptFlowEstimator }

impl crate::videostab::PyrLkOptFlowEstimatorBaseTraitConst for core::Ptr<crate::videostab::DensePyrLkOptFlowEstimatorGpu> {
	#[inline] fn as_raw_PyrLkOptFlowEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::PyrLkOptFlowEstimatorBaseTrait for core::Ptr<crate::videostab::DensePyrLkOptFlowEstimatorGpu> {
	#[inline] fn as_raw_mut_PyrLkOptFlowEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::DensePyrLkOptFlowEstimatorGpu>, core::Ptr<crate::videostab::PyrLkOptFlowEstimatorBase>, cv_PtrLcv_videostab_DensePyrLkOptFlowEstimatorGpuG_to_PtrOfPyrLkOptFlowEstimatorBase }

impl std::fmt::Debug for core::Ptr<crate::videostab::DensePyrLkOptFlowEstimatorGpu> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDensePyrLkOptFlowEstimatorGpu")
			.finish()
	}
}

