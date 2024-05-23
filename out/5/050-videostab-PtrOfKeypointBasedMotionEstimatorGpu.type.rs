ptr_extern! { crate::videostab::KeypointBasedMotionEstimatorGpu,
	cv_PtrLcv_videostab_KeypointBasedMotionEstimatorGpuG_new_null_const, cv_PtrLcv_videostab_KeypointBasedMotionEstimatorGpuG_delete, cv_PtrLcv_videostab_KeypointBasedMotionEstimatorGpuG_getInnerPtr_const, cv_PtrLcv_videostab_KeypointBasedMotionEstimatorGpuG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::KeypointBasedMotionEstimatorGpu, cv_PtrLcv_videostab_KeypointBasedMotionEstimatorGpuG_new_const_KeypointBasedMotionEstimatorGpu }
impl core::Ptr<crate::videostab::KeypointBasedMotionEstimatorGpu> {
	#[inline] pub fn as_raw_PtrOfKeypointBasedMotionEstimatorGpu(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfKeypointBasedMotionEstimatorGpu(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::KeypointBasedMotionEstimatorGpuTraitConst for core::Ptr<crate::videostab::KeypointBasedMotionEstimatorGpu> {
	#[inline] fn as_raw_KeypointBasedMotionEstimatorGpu(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::KeypointBasedMotionEstimatorGpuTrait for core::Ptr<crate::videostab::KeypointBasedMotionEstimatorGpu> {
	#[inline] fn as_raw_mut_KeypointBasedMotionEstimatorGpu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::ImageMotionEstimatorBaseTraitConst for core::Ptr<crate::videostab::KeypointBasedMotionEstimatorGpu> {
	#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ImageMotionEstimatorBaseTrait for core::Ptr<crate::videostab::KeypointBasedMotionEstimatorGpu> {
	#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::KeypointBasedMotionEstimatorGpu>, core::Ptr<crate::videostab::ImageMotionEstimatorBase>, cv_PtrLcv_videostab_KeypointBasedMotionEstimatorGpuG_to_PtrOfImageMotionEstimatorBase }

impl std::fmt::Debug for core::Ptr<crate::videostab::KeypointBasedMotionEstimatorGpu> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfKeypointBasedMotionEstimatorGpu")
			.finish()
	}
}

