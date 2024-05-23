ptr_extern! { crate::sfm::BaseSFM,
	cv_PtrLcv_sfm_BaseSFMG_new_null_const, cv_PtrLcv_sfm_BaseSFMG_delete, cv_PtrLcv_sfm_BaseSFMG_getInnerPtr_const, cv_PtrLcv_sfm_BaseSFMG_getInnerPtrMut
}

impl core::Ptr<crate::sfm::BaseSFM> {
	#[inline] pub fn as_raw_PtrOfBaseSFM(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBaseSFM(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::sfm::BaseSFMTraitConst for core::Ptr<crate::sfm::BaseSFM> {
	#[inline] fn as_raw_BaseSFM(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::sfm::BaseSFMTrait for core::Ptr<crate::sfm::BaseSFM> {
	#[inline] fn as_raw_mut_BaseSFM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::sfm::BaseSFM> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfBaseSFM")
			.finish()
	}
}

