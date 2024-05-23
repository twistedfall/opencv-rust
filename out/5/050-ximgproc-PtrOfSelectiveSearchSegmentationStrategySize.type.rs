ptr_extern! { crate::ximgproc::SelectiveSearchSegmentationStrategySize,
	cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategySizeG_new_null_const, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategySizeG_delete, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategySizeG_getInnerPtr_const, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategySizeG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategySize> {
	#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategySize(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategySize(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategySizeTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategySize> {
	#[inline] fn as_raw_SelectiveSearchSegmentationStrategySize(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategySizeTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategySize> {
	#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategySize(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategySize> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategySize> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategySize>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategySizeG_to_PtrOfAlgorithm }

impl crate::ximgproc::SelectiveSearchSegmentationStrategyTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategySize> {
	#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategySize> {
	#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategySize>, core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategy>, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategySizeG_to_PtrOfSelectiveSearchSegmentationStrategy }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategySize> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSelectiveSearchSegmentationStrategySize")
			.finish()
	}
}

