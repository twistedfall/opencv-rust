ptr_extern! { crate::imgproc::GeneralizedHoughGuil,
	cv_PtrLcv_GeneralizedHoughGuilG_new_null_const, cv_PtrLcv_GeneralizedHoughGuilG_delete, cv_PtrLcv_GeneralizedHoughGuilG_getInnerPtr_const, cv_PtrLcv_GeneralizedHoughGuilG_getInnerPtrMut
}

impl core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
	#[inline] pub fn as_raw_PtrOfGeneralizedHoughGuil(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGeneralizedHoughGuil(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::imgproc::GeneralizedHoughGuilTraitConst for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
	#[inline] fn as_raw_GeneralizedHoughGuil(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::imgproc::GeneralizedHoughGuilTrait for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
	#[inline] fn as_raw_mut_GeneralizedHoughGuil(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::imgproc::GeneralizedHoughGuil>, core::Ptr<core::Algorithm>, cv_PtrLcv_GeneralizedHoughGuilG_to_PtrOfAlgorithm }

impl crate::imgproc::GeneralizedHoughTraitConst for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
	#[inline] fn as_raw_GeneralizedHough(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::imgproc::GeneralizedHoughTrait for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
	#[inline] fn as_raw_mut_GeneralizedHough(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::imgproc::GeneralizedHoughGuil>, core::Ptr<crate::imgproc::GeneralizedHough>, cv_PtrLcv_GeneralizedHoughGuilG_to_PtrOfGeneralizedHough }

impl std::fmt::Debug for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfGeneralizedHoughGuil")
			.finish()
	}
}

