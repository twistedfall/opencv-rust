ptr_extern! { crate::dnn::ResizeLayer,
	cv_PtrLcv_dnn_ResizeLayerG_new_null_const, cv_PtrLcv_dnn_ResizeLayerG_delete, cv_PtrLcv_dnn_ResizeLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ResizeLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::ResizeLayer, cv_PtrLcv_dnn_ResizeLayerG_new_const_ResizeLayer }
impl core::Ptr<crate::dnn::ResizeLayer> {
	#[inline] pub fn as_raw_PtrOfResizeLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfResizeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::ResizeLayerTraitConst for core::Ptr<crate::dnn::ResizeLayer> {
	#[inline] fn as_raw_ResizeLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ResizeLayerTrait for core::Ptr<crate::dnn::ResizeLayer> {
	#[inline] fn as_raw_mut_ResizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ResizeLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::ResizeLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ResizeLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ResizeLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ResizeLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ResizeLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ResizeLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ResizeLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::ResizeLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfResizeLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

