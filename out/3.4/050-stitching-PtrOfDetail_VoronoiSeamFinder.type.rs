ptr_extern! { crate::stitching::Detail_VoronoiSeamFinder,
	cv_PtrLcv_detail_VoronoiSeamFinderG_new_null_const, cv_PtrLcv_detail_VoronoiSeamFinderG_delete, cv_PtrLcv_detail_VoronoiSeamFinderG_getInnerPtr_const, cv_PtrLcv_detail_VoronoiSeamFinderG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Detail_VoronoiSeamFinder, cv_PtrLcv_detail_VoronoiSeamFinderG_new_const_VoronoiSeamFinder }
impl core::Ptr<crate::stitching::Detail_VoronoiSeamFinder> {
	#[inline] pub fn as_raw_PtrOfDetail_VoronoiSeamFinder(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_VoronoiSeamFinder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_VoronoiSeamFinderTraitConst for core::Ptr<crate::stitching::Detail_VoronoiSeamFinder> {
	#[inline] fn as_raw_Detail_VoronoiSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_VoronoiSeamFinderTrait for core::Ptr<crate::stitching::Detail_VoronoiSeamFinder> {
	#[inline] fn as_raw_mut_Detail_VoronoiSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_PairwiseSeamFinderTraitConst for core::Ptr<crate::stitching::Detail_VoronoiSeamFinder> {
	#[inline] fn as_raw_Detail_PairwiseSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_PairwiseSeamFinderTrait for core::Ptr<crate::stitching::Detail_VoronoiSeamFinder> {
	#[inline] fn as_raw_mut_Detail_PairwiseSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_VoronoiSeamFinder>, core::Ptr<crate::stitching::Detail_PairwiseSeamFinder>, cv_PtrLcv_detail_VoronoiSeamFinderG_to_PtrOfDetail_PairwiseSeamFinder }

impl crate::stitching::Detail_SeamFinderTraitConst for core::Ptr<crate::stitching::Detail_VoronoiSeamFinder> {
	#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_SeamFinderTrait for core::Ptr<crate::stitching::Detail_VoronoiSeamFinder> {
	#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_VoronoiSeamFinder>, core::Ptr<crate::stitching::Detail_SeamFinder>, cv_PtrLcv_detail_VoronoiSeamFinderG_to_PtrOfDetail_SeamFinder }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_VoronoiSeamFinder> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_VoronoiSeamFinder")
			.finish()
	}
}

