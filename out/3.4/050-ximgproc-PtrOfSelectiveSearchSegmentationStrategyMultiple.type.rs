ptr_extern! { crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple,
	cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultipleG_new_null_const, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultipleG_delete, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultipleG_getInnerPtr_const, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultipleG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple> {
	#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategyMultiple(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategyMultiple(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyMultipleTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple> {
	#[inline] fn as_raw_SelectiveSearchSegmentationStrategyMultiple(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyMultipleTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple> {
	#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategyMultiple(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultipleG_to_PtrOfAlgorithm }

impl crate::ximgproc::SelectiveSearchSegmentationStrategyTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple> {
	#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple> {
	#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple>, core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategy>, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultipleG_to_PtrOfSelectiveSearchSegmentationStrategy }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSelectiveSearchSegmentationStrategyMultiple")
			.finish()
	}
}

