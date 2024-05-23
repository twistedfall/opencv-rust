ptr_extern! { crate::imgproc::GeneralizedHough,
	cv_PtrLcv_GeneralizedHoughG_new_null_const, cv_PtrLcv_GeneralizedHoughG_delete, cv_PtrLcv_GeneralizedHoughG_getInnerPtr_const, cv_PtrLcv_GeneralizedHoughG_getInnerPtrMut
}

impl core::Ptr<crate::imgproc::GeneralizedHough> {
	#[inline] pub fn as_raw_PtrOfGeneralizedHough(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGeneralizedHough(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::imgproc::GeneralizedHoughTraitConst for core::Ptr<crate::imgproc::GeneralizedHough> {
	#[inline] fn as_raw_GeneralizedHough(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::imgproc::GeneralizedHoughTrait for core::Ptr<crate::imgproc::GeneralizedHough> {
	#[inline] fn as_raw_mut_GeneralizedHough(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::imgproc::GeneralizedHough> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::imgproc::GeneralizedHough> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::imgproc::GeneralizedHough>, core::Ptr<core::Algorithm>, cv_PtrLcv_GeneralizedHoughG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::imgproc::GeneralizedHough> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfGeneralizedHough")
			.finish()
	}
}

