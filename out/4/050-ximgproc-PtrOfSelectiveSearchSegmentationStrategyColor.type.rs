ptr_extern! { crate::ximgproc::SelectiveSearchSegmentationStrategyColor,
	cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyColorG_new_null_const, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyColorG_delete, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyColorG_getInnerPtr_const, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyColorG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyColor> {
	#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategyColor(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategyColor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyColorTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyColor> {
	#[inline] fn as_raw_SelectiveSearchSegmentationStrategyColor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyColorTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyColor> {
	#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategyColor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyColor> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyColor> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyColor>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyColorG_to_PtrOfAlgorithm }

impl crate::ximgproc::SelectiveSearchSegmentationStrategyTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyColor> {
	#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyColor> {
	#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyColor>, core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategy>, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyColorG_to_PtrOfSelectiveSearchSegmentationStrategy }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyColor> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSelectiveSearchSegmentationStrategyColor")
			.finish()
	}
}

