ptr_extern! { crate::stitching::Detail_SeamFinder,
	cv_PtrLcv_detail_SeamFinderG_new_null_const, cv_PtrLcv_detail_SeamFinderG_delete, cv_PtrLcv_detail_SeamFinderG_getInnerPtr_const, cv_PtrLcv_detail_SeamFinderG_getInnerPtrMut
}

impl core::Ptr<crate::stitching::Detail_SeamFinder> {
	#[inline] pub fn as_raw_PtrOfDetail_SeamFinder(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_SeamFinder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_SeamFinderTraitConst for core::Ptr<crate::stitching::Detail_SeamFinder> {
	#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_SeamFinderTrait for core::Ptr<crate::stitching::Detail_SeamFinder> {
	#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_SeamFinder> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_SeamFinder")
			.finish()
	}
}

