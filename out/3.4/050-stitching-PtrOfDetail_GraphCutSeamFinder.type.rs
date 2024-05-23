ptr_extern! { crate::stitching::Detail_GraphCutSeamFinder,
	cv_PtrLcv_detail_GraphCutSeamFinderG_new_null_const, cv_PtrLcv_detail_GraphCutSeamFinderG_delete, cv_PtrLcv_detail_GraphCutSeamFinderG_getInnerPtr_const, cv_PtrLcv_detail_GraphCutSeamFinderG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Detail_GraphCutSeamFinder, cv_PtrLcv_detail_GraphCutSeamFinderG_new_const_GraphCutSeamFinder }
impl core::Ptr<crate::stitching::Detail_GraphCutSeamFinder> {
	#[inline] pub fn as_raw_PtrOfDetail_GraphCutSeamFinder(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_GraphCutSeamFinder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_GraphCutSeamFinderTraitConst for core::Ptr<crate::stitching::Detail_GraphCutSeamFinder> {
	#[inline] fn as_raw_Detail_GraphCutSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_GraphCutSeamFinderTrait for core::Ptr<crate::stitching::Detail_GraphCutSeamFinder> {
	#[inline] fn as_raw_mut_Detail_GraphCutSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_GraphCutSeamFinderBaseTraitConst for core::Ptr<crate::stitching::Detail_GraphCutSeamFinder> {
	#[inline] fn as_raw_Detail_GraphCutSeamFinderBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_GraphCutSeamFinderBaseTrait for core::Ptr<crate::stitching::Detail_GraphCutSeamFinder> {
	#[inline] fn as_raw_mut_Detail_GraphCutSeamFinderBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_GraphCutSeamFinder>, core::Ptr<crate::stitching::Detail_GraphCutSeamFinderBase>, cv_PtrLcv_detail_GraphCutSeamFinderG_to_PtrOfDetail_GraphCutSeamFinderBase }

impl crate::stitching::Detail_SeamFinderTraitConst for core::Ptr<crate::stitching::Detail_GraphCutSeamFinder> {
	#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_SeamFinderTrait for core::Ptr<crate::stitching::Detail_GraphCutSeamFinder> {
	#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_GraphCutSeamFinder>, core::Ptr<crate::stitching::Detail_SeamFinder>, cv_PtrLcv_detail_GraphCutSeamFinderG_to_PtrOfDetail_SeamFinder }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_GraphCutSeamFinder> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_GraphCutSeamFinder")
			.finish()
	}
}

