ptr_extern! { crate::ximgproc::SelectiveSearchSegmentationStrategyTexture,
	cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyTextureG_new_null_const, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyTextureG_delete, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyTextureG_getInnerPtr_const, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyTextureG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyTexture> {
	#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategyTexture(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategyTexture(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyTextureTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyTexture> {
	#[inline] fn as_raw_SelectiveSearchSegmentationStrategyTexture(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyTextureTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyTexture> {
	#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategyTexture(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyTexture> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyTexture> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyTexture>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyTextureG_to_PtrOfAlgorithm }

impl crate::ximgproc::SelectiveSearchSegmentationStrategyTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyTexture> {
	#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyTexture> {
	#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyTexture>, core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategy>, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyTextureG_to_PtrOfSelectiveSearchSegmentationStrategy }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyTexture> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSelectiveSearchSegmentationStrategyTexture")
			.finish()
	}
}

