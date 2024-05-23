ptr_extern! { crate::stitching::Detail_PairwiseSeamFinder,
	cv_PtrLcv_detail_PairwiseSeamFinderG_new_null_const, cv_PtrLcv_detail_PairwiseSeamFinderG_delete, cv_PtrLcv_detail_PairwiseSeamFinderG_getInnerPtr_const, cv_PtrLcv_detail_PairwiseSeamFinderG_getInnerPtrMut
}

impl core::Ptr<crate::stitching::Detail_PairwiseSeamFinder> {
	#[inline] pub fn as_raw_PtrOfDetail_PairwiseSeamFinder(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_PairwiseSeamFinder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_PairwiseSeamFinderTraitConst for core::Ptr<crate::stitching::Detail_PairwiseSeamFinder> {
	#[inline] fn as_raw_Detail_PairwiseSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_PairwiseSeamFinderTrait for core::Ptr<crate::stitching::Detail_PairwiseSeamFinder> {
	#[inline] fn as_raw_mut_Detail_PairwiseSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_SeamFinderTraitConst for core::Ptr<crate::stitching::Detail_PairwiseSeamFinder> {
	#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_SeamFinderTrait for core::Ptr<crate::stitching::Detail_PairwiseSeamFinder> {
	#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_PairwiseSeamFinder>, core::Ptr<crate::stitching::Detail_SeamFinder>, cv_PtrLcv_detail_PairwiseSeamFinderG_to_PtrOfDetail_SeamFinder }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_PairwiseSeamFinder> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_PairwiseSeamFinder")
			.finish()
	}
}

