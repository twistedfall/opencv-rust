ptr_extern! { crate::dnn::BackendNode,
	cv_PtrLcv_dnn_BackendNodeG_new_null_const, cv_PtrLcv_dnn_BackendNodeG_delete, cv_PtrLcv_dnn_BackendNodeG_getInnerPtr_const, cv_PtrLcv_dnn_BackendNodeG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::BackendNode, cv_PtrLcv_dnn_BackendNodeG_new_const_BackendNode }
impl core::Ptr<crate::dnn::BackendNode> {
	#[inline] pub fn as_raw_PtrOfBackendNode(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBackendNode(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::BackendNodeTraitConst for core::Ptr<crate::dnn::BackendNode> {
	#[inline] fn as_raw_BackendNode(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::BackendNodeTrait for core::Ptr<crate::dnn::BackendNode> {
	#[inline] fn as_raw_mut_BackendNode(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::dnn::BackendNode> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfBackendNode")
			.field("backend_id", &crate::dnn::BackendNodeTraitConst::backend_id(self))
			.finish()
	}
}

