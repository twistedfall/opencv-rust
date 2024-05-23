ptr_extern! { crate::rgbd::LineMod_Modality,
	cv_PtrLcv_linemod_ModalityG_new_null_const, cv_PtrLcv_linemod_ModalityG_delete, cv_PtrLcv_linemod_ModalityG_getInnerPtr_const, cv_PtrLcv_linemod_ModalityG_getInnerPtrMut
}

impl core::Ptr<crate::rgbd::LineMod_Modality> {
	#[inline] pub fn as_raw_PtrOfLineMod_Modality(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLineMod_Modality(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rgbd::LineMod_ModalityTraitConst for core::Ptr<crate::rgbd::LineMod_Modality> {
	#[inline] fn as_raw_LineMod_Modality(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::LineMod_ModalityTrait for core::Ptr<crate::rgbd::LineMod_Modality> {
	#[inline] fn as_raw_mut_LineMod_Modality(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::rgbd::LineMod_Modality> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfLineMod_Modality")
			.finish()
	}
}

