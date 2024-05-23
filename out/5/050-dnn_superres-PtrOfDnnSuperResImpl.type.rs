ptr_extern! { crate::dnn_superres::DnnSuperResImpl,
	cv_PtrLcv_dnn_superres_DnnSuperResImplG_new_null_const, cv_PtrLcv_dnn_superres_DnnSuperResImplG_delete, cv_PtrLcv_dnn_superres_DnnSuperResImplG_getInnerPtr_const, cv_PtrLcv_dnn_superres_DnnSuperResImplG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn_superres::DnnSuperResImpl, cv_PtrLcv_dnn_superres_DnnSuperResImplG_new_const_DnnSuperResImpl }
impl core::Ptr<crate::dnn_superres::DnnSuperResImpl> {
	#[inline] pub fn as_raw_PtrOfDnnSuperResImpl(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDnnSuperResImpl(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn_superres::DnnSuperResImplTraitConst for core::Ptr<crate::dnn_superres::DnnSuperResImpl> {
	#[inline] fn as_raw_DnnSuperResImpl(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn_superres::DnnSuperResImplTrait for core::Ptr<crate::dnn_superres::DnnSuperResImpl> {
	#[inline] fn as_raw_mut_DnnSuperResImpl(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::dnn_superres::DnnSuperResImpl> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDnnSuperResImpl")
			.finish()
	}
}

