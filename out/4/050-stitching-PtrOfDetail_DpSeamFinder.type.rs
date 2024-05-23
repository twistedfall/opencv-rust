ptr_extern! { crate::stitching::Detail_DpSeamFinder,
	cv_PtrLcv_detail_DpSeamFinderG_new_null_const, cv_PtrLcv_detail_DpSeamFinderG_delete, cv_PtrLcv_detail_DpSeamFinderG_getInnerPtr_const, cv_PtrLcv_detail_DpSeamFinderG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Detail_DpSeamFinder, cv_PtrLcv_detail_DpSeamFinderG_new_const_DpSeamFinder }
impl core::Ptr<crate::stitching::Detail_DpSeamFinder> {
	#[inline] pub fn as_raw_PtrOfDetail_DpSeamFinder(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_DpSeamFinder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_DpSeamFinderTraitConst for core::Ptr<crate::stitching::Detail_DpSeamFinder> {
	#[inline] fn as_raw_Detail_DpSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_DpSeamFinderTrait for core::Ptr<crate::stitching::Detail_DpSeamFinder> {
	#[inline] fn as_raw_mut_Detail_DpSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_SeamFinderTraitConst for core::Ptr<crate::stitching::Detail_DpSeamFinder> {
	#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_SeamFinderTrait for core::Ptr<crate::stitching::Detail_DpSeamFinder> {
	#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_DpSeamFinder>, core::Ptr<crate::stitching::Detail_SeamFinder>, cv_PtrLcv_detail_DpSeamFinderG_to_PtrOfDetail_SeamFinder }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_DpSeamFinder> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_DpSeamFinder")
			.finish()
	}
}

