ptr_extern! { crate::stitching::Detail_GraphCutSeamFinderGpu,
	cv_PtrLcv_detail_GraphCutSeamFinderGpuG_new_null_const, cv_PtrLcv_detail_GraphCutSeamFinderGpuG_delete, cv_PtrLcv_detail_GraphCutSeamFinderGpuG_getInnerPtr_const, cv_PtrLcv_detail_GraphCutSeamFinderGpuG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Detail_GraphCutSeamFinderGpu, cv_PtrLcv_detail_GraphCutSeamFinderGpuG_new_const_GraphCutSeamFinderGpu }
impl core::Ptr<crate::stitching::Detail_GraphCutSeamFinderGpu> {
	#[inline] pub fn as_raw_PtrOfDetail_GraphCutSeamFinderGpu(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_GraphCutSeamFinderGpu(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_GraphCutSeamFinderGpuTraitConst for core::Ptr<crate::stitching::Detail_GraphCutSeamFinderGpu> {
	#[inline] fn as_raw_Detail_GraphCutSeamFinderGpu(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_GraphCutSeamFinderGpuTrait for core::Ptr<crate::stitching::Detail_GraphCutSeamFinderGpu> {
	#[inline] fn as_raw_mut_Detail_GraphCutSeamFinderGpu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_GraphCutSeamFinderBaseTraitConst for core::Ptr<crate::stitching::Detail_GraphCutSeamFinderGpu> {
	#[inline] fn as_raw_Detail_GraphCutSeamFinderBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_GraphCutSeamFinderBaseTrait for core::Ptr<crate::stitching::Detail_GraphCutSeamFinderGpu> {
	#[inline] fn as_raw_mut_Detail_GraphCutSeamFinderBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_GraphCutSeamFinderGpu>, core::Ptr<crate::stitching::Detail_GraphCutSeamFinderBase>, cv_PtrLcv_detail_GraphCutSeamFinderGpuG_to_PtrOfDetail_GraphCutSeamFinderBase }

impl crate::stitching::Detail_PairwiseSeamFinderTraitConst for core::Ptr<crate::stitching::Detail_GraphCutSeamFinderGpu> {
	#[inline] fn as_raw_Detail_PairwiseSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_PairwiseSeamFinderTrait for core::Ptr<crate::stitching::Detail_GraphCutSeamFinderGpu> {
	#[inline] fn as_raw_mut_Detail_PairwiseSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_GraphCutSeamFinderGpu>, core::Ptr<crate::stitching::Detail_PairwiseSeamFinder>, cv_PtrLcv_detail_GraphCutSeamFinderGpuG_to_PtrOfDetail_PairwiseSeamFinder }

impl crate::stitching::Detail_SeamFinderTraitConst for core::Ptr<crate::stitching::Detail_GraphCutSeamFinderGpu> {
	#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_SeamFinderTrait for core::Ptr<crate::stitching::Detail_GraphCutSeamFinderGpu> {
	#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_GraphCutSeamFinderGpu>, core::Ptr<crate::stitching::Detail_SeamFinder>, cv_PtrLcv_detail_GraphCutSeamFinderGpuG_to_PtrOfDetail_SeamFinder }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_GraphCutSeamFinderGpu> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_GraphCutSeamFinderGpu")
			.finish()
	}
}

