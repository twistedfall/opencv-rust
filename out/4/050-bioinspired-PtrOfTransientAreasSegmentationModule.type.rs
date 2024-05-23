ptr_extern! { crate::bioinspired::TransientAreasSegmentationModule,
	cv_PtrLcv_bioinspired_TransientAreasSegmentationModuleG_new_null_const, cv_PtrLcv_bioinspired_TransientAreasSegmentationModuleG_delete, cv_PtrLcv_bioinspired_TransientAreasSegmentationModuleG_getInnerPtr_const, cv_PtrLcv_bioinspired_TransientAreasSegmentationModuleG_getInnerPtrMut
}

impl core::Ptr<crate::bioinspired::TransientAreasSegmentationModule> {
	#[inline] pub fn as_raw_PtrOfTransientAreasSegmentationModule(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTransientAreasSegmentationModule(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::bioinspired::TransientAreasSegmentationModuleTraitConst for core::Ptr<crate::bioinspired::TransientAreasSegmentationModule> {
	#[inline] fn as_raw_TransientAreasSegmentationModule(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::bioinspired::TransientAreasSegmentationModuleTrait for core::Ptr<crate::bioinspired::TransientAreasSegmentationModule> {
	#[inline] fn as_raw_mut_TransientAreasSegmentationModule(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::bioinspired::TransientAreasSegmentationModule> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::bioinspired::TransientAreasSegmentationModule> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::bioinspired::TransientAreasSegmentationModule>, core::Ptr<core::Algorithm>, cv_PtrLcv_bioinspired_TransientAreasSegmentationModuleG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::bioinspired::TransientAreasSegmentationModule> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTransientAreasSegmentationModule")
			.finish()
	}
}

