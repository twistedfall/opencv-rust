ptr_extern! { crate::superres::SuperRes_SuperResolution,
	cv_PtrLcv_superres_SuperResolutionG_new_null_const, cv_PtrLcv_superres_SuperResolutionG_delete, cv_PtrLcv_superres_SuperResolutionG_getInnerPtr_const, cv_PtrLcv_superres_SuperResolutionG_getInnerPtrMut
}

impl core::Ptr<crate::superres::SuperRes_SuperResolution> {
	#[inline] pub fn as_raw_PtrOfSuperRes_SuperResolution(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSuperRes_SuperResolution(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::superres::SuperRes_SuperResolutionTraitConst for core::Ptr<crate::superres::SuperRes_SuperResolution> {
	#[inline] fn as_raw_SuperRes_SuperResolution(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::superres::SuperRes_SuperResolutionTrait for core::Ptr<crate::superres::SuperRes_SuperResolution> {
	#[inline] fn as_raw_mut_SuperRes_SuperResolution(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::superres::SuperRes_SuperResolution> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::superres::SuperRes_SuperResolution> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::superres::SuperRes_SuperResolution>, core::Ptr<core::Algorithm>, cv_PtrLcv_superres_SuperResolutionG_to_PtrOfAlgorithm }

impl crate::superres::SuperRes_FrameSourceTraitConst for core::Ptr<crate::superres::SuperRes_SuperResolution> {
	#[inline] fn as_raw_SuperRes_FrameSource(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::superres::SuperRes_FrameSourceTrait for core::Ptr<crate::superres::SuperRes_SuperResolution> {
	#[inline] fn as_raw_mut_SuperRes_FrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::superres::SuperRes_SuperResolution>, core::Ptr<crate::superres::SuperRes_FrameSource>, cv_PtrLcv_superres_SuperResolutionG_to_PtrOfSuperRes_FrameSource }

impl std::fmt::Debug for core::Ptr<crate::superres::SuperRes_SuperResolution> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSuperRes_SuperResolution")
			.finish()
	}
}

