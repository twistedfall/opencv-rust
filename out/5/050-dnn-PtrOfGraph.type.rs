ptr_extern! { crate::dnn::Graph,
	cv_PtrLcv_dnn_GraphG_new_null_const, cv_PtrLcv_dnn_GraphG_delete, cv_PtrLcv_dnn_GraphG_getInnerPtr_const, cv_PtrLcv_dnn_GraphG_getInnerPtrMut
}

impl core::Ptr<crate::dnn::Graph> {
	#[inline] pub fn as_raw_PtrOfGraph(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGraph(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::GraphTraitConst for core::Ptr<crate::dnn::Graph> {
	#[inline] fn as_raw_Graph(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::GraphTrait for core::Ptr<crate::dnn::Graph> {
	#[inline] fn as_raw_mut_Graph(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::dnn::Graph> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfGraph")
			.finish()
	}
}

