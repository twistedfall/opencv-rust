ptr_extern! { crate::stitching::Stitcher,
	cv_PtrLcv_StitcherG_new_null_const, cv_PtrLcv_StitcherG_delete, cv_PtrLcv_StitcherG_getInnerPtr_const, cv_PtrLcv_StitcherG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Stitcher, cv_PtrLcv_StitcherG_new_const_Stitcher }
impl core::Ptr<crate::stitching::Stitcher> {
	#[inline] pub fn as_raw_PtrOfStitcher(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfStitcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::StitcherTraitConst for core::Ptr<crate::stitching::Stitcher> {
	#[inline] fn as_raw_Stitcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::StitcherTrait for core::Ptr<crate::stitching::Stitcher> {
	#[inline] fn as_raw_mut_Stitcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::stitching::Stitcher> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfStitcher")
			.finish()
	}
}

