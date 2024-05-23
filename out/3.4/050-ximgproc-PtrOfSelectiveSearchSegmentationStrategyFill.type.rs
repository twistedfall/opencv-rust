ptr_extern! { crate::ximgproc::SelectiveSearchSegmentationStrategyFill,
	cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyFillG_new_null_const, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyFillG_delete, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyFillG_getInnerPtr_const, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyFillG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyFill> {
	#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategyFill(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategyFill(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyFillTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyFill> {
	#[inline] fn as_raw_SelectiveSearchSegmentationStrategyFill(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyFillTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyFill> {
	#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategyFill(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyFill> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyFill> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyFill>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyFillG_to_PtrOfAlgorithm }

impl crate::ximgproc::SelectiveSearchSegmentationStrategyTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyFill> {
	#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyFill> {
	#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyFill>, core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategy>, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyFillG_to_PtrOfSelectiveSearchSegmentationStrategy }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyFill> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSelectiveSearchSegmentationStrategyFill")
			.finish()
	}
}

