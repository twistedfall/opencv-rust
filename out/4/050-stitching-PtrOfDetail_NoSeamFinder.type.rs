ptr_extern! { crate::stitching::Detail_NoSeamFinder,
	cv_PtrLcv_detail_NoSeamFinderG_new_null_const, cv_PtrLcv_detail_NoSeamFinderG_delete, cv_PtrLcv_detail_NoSeamFinderG_getInnerPtr_const, cv_PtrLcv_detail_NoSeamFinderG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Detail_NoSeamFinder, cv_PtrLcv_detail_NoSeamFinderG_new_const_NoSeamFinder }
impl core::Ptr<crate::stitching::Detail_NoSeamFinder> {
	#[inline] pub fn as_raw_PtrOfDetail_NoSeamFinder(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_NoSeamFinder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_NoSeamFinderTraitConst for core::Ptr<crate::stitching::Detail_NoSeamFinder> {
	#[inline] fn as_raw_Detail_NoSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_NoSeamFinderTrait for core::Ptr<crate::stitching::Detail_NoSeamFinder> {
	#[inline] fn as_raw_mut_Detail_NoSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_SeamFinderTraitConst for core::Ptr<crate::stitching::Detail_NoSeamFinder> {
	#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_SeamFinderTrait for core::Ptr<crate::stitching::Detail_NoSeamFinder> {
	#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_NoSeamFinder>, core::Ptr<crate::stitching::Detail_SeamFinder>, cv_PtrLcv_detail_NoSeamFinderG_to_PtrOfDetail_SeamFinder }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_NoSeamFinder> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_NoSeamFinder")
			.finish()
	}
}

