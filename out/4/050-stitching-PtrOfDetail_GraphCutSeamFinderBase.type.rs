ptr_extern! { crate::stitching::Detail_GraphCutSeamFinderBase,
	cv_PtrLcv_detail_GraphCutSeamFinderBaseG_new_null_const, cv_PtrLcv_detail_GraphCutSeamFinderBaseG_delete, cv_PtrLcv_detail_GraphCutSeamFinderBaseG_getInnerPtr_const, cv_PtrLcv_detail_GraphCutSeamFinderBaseG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Detail_GraphCutSeamFinderBase, cv_PtrLcv_detail_GraphCutSeamFinderBaseG_new_const_GraphCutSeamFinderBase }
impl core::Ptr<crate::stitching::Detail_GraphCutSeamFinderBase> {
	#[inline] pub fn as_raw_PtrOfDetail_GraphCutSeamFinderBase(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_GraphCutSeamFinderBase(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_GraphCutSeamFinderBaseTraitConst for core::Ptr<crate::stitching::Detail_GraphCutSeamFinderBase> {
	#[inline] fn as_raw_Detail_GraphCutSeamFinderBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_GraphCutSeamFinderBaseTrait for core::Ptr<crate::stitching::Detail_GraphCutSeamFinderBase> {
	#[inline] fn as_raw_mut_Detail_GraphCutSeamFinderBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_GraphCutSeamFinderBase> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_GraphCutSeamFinderBase")
			.finish()
	}
}

