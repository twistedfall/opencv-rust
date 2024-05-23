ptr_extern! { crate::dnn::BackendWrapper,
	cv_PtrLcv_dnn_BackendWrapperG_new_null_const, cv_PtrLcv_dnn_BackendWrapperG_delete, cv_PtrLcv_dnn_BackendWrapperG_getInnerPtr_const, cv_PtrLcv_dnn_BackendWrapperG_getInnerPtrMut
}

impl core::Ptr<crate::dnn::BackendWrapper> {
	#[inline] pub fn as_raw_PtrOfBackendWrapper(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBackendWrapper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::BackendWrapperTraitConst for core::Ptr<crate::dnn::BackendWrapper> {
	#[inline] fn as_raw_BackendWrapper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::BackendWrapperTrait for core::Ptr<crate::dnn::BackendWrapper> {
	#[inline] fn as_raw_mut_BackendWrapper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::dnn::BackendWrapper> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfBackendWrapper")
			.field("backend_id", &crate::dnn::BackendWrapperTraitConst::backend_id(self))
			.field("target_id", &crate::dnn::BackendWrapperTraitConst::target_id(self))
			.finish()
	}
}

