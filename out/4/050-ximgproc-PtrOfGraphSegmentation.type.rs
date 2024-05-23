ptr_extern! { crate::ximgproc::GraphSegmentation,
	cv_PtrLcv_ximgproc_segmentation_GraphSegmentationG_new_null_const, cv_PtrLcv_ximgproc_segmentation_GraphSegmentationG_delete, cv_PtrLcv_ximgproc_segmentation_GraphSegmentationG_getInnerPtr_const, cv_PtrLcv_ximgproc_segmentation_GraphSegmentationG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::GraphSegmentation> {
	#[inline] pub fn as_raw_PtrOfGraphSegmentation(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGraphSegmentation(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::GraphSegmentationTraitConst for core::Ptr<crate::ximgproc::GraphSegmentation> {
	#[inline] fn as_raw_GraphSegmentation(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::GraphSegmentationTrait for core::Ptr<crate::ximgproc::GraphSegmentation> {
	#[inline] fn as_raw_mut_GraphSegmentation(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::GraphSegmentation> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::GraphSegmentation> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::GraphSegmentation>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_segmentation_GraphSegmentationG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::GraphSegmentation> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfGraphSegmentation")
			.finish()
	}
}

