ptr_extern! { crate::ximgproc::SelectiveSearchSegmentation,
	cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationG_new_null_const, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationG_delete, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationG_getInnerPtr_const, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::SelectiveSearchSegmentation> {
	#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentation(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentation(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::SelectiveSearchSegmentationTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentation> {
	#[inline] fn as_raw_SelectiveSearchSegmentation(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SelectiveSearchSegmentationTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentation> {
	#[inline] fn as_raw_mut_SelectiveSearchSegmentation(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentation> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentation> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::SelectiveSearchSegmentation>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::SelectiveSearchSegmentation> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSelectiveSearchSegmentation")
			.finish()
	}
}

