ptr_extern! { crate::features2d::Feature2D,
	cv_PtrLcv_Feature2DG_new_null_const, cv_PtrLcv_Feature2DG_delete, cv_PtrLcv_Feature2DG_getInnerPtr_const, cv_PtrLcv_Feature2DG_getInnerPtrMut
}

ptr_extern_ctor! { crate::features2d::Feature2D, cv_PtrLcv_Feature2DG_new_const_Feature2D }
impl core::Ptr<crate::features2d::Feature2D> {
	#[inline] pub fn as_raw_PtrOfFeature2D(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFeature2D(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::features2d::Feature2D> {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for core::Ptr<crate::features2d::Feature2D> {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::features2d::Feature2D> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfFeature2D")
			.finish()
	}
}

