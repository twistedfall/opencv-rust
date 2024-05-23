ptr_extern! { crate::imgproc::GeneralizedHoughBallard,
	cv_PtrLcv_GeneralizedHoughBallardG_new_null_const, cv_PtrLcv_GeneralizedHoughBallardG_delete, cv_PtrLcv_GeneralizedHoughBallardG_getInnerPtr_const, cv_PtrLcv_GeneralizedHoughBallardG_getInnerPtrMut
}

impl core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
	#[inline] pub fn as_raw_PtrOfGeneralizedHoughBallard(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGeneralizedHoughBallard(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::imgproc::GeneralizedHoughBallardTraitConst for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
	#[inline] fn as_raw_GeneralizedHoughBallard(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::imgproc::GeneralizedHoughBallardTrait for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
	#[inline] fn as_raw_mut_GeneralizedHoughBallard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::imgproc::GeneralizedHoughBallard>, core::Ptr<core::Algorithm>, cv_PtrLcv_GeneralizedHoughBallardG_to_PtrOfAlgorithm }

impl crate::imgproc::GeneralizedHoughTraitConst for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
	#[inline] fn as_raw_GeneralizedHough(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::imgproc::GeneralizedHoughTrait for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
	#[inline] fn as_raw_mut_GeneralizedHough(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::imgproc::GeneralizedHoughBallard>, core::Ptr<crate::imgproc::GeneralizedHough>, cv_PtrLcv_GeneralizedHoughBallardG_to_PtrOfGeneralizedHough }

impl std::fmt::Debug for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfGeneralizedHoughBallard")
			.finish()
	}
}

