ptr_extern! { crate::dnn::AttentionLayer,
	cv_PtrLcv_dnn_AttentionLayerG_new_null_const, cv_PtrLcv_dnn_AttentionLayerG_delete, cv_PtrLcv_dnn_AttentionLayerG_getInnerPtr_const, cv_PtrLcv_dnn_AttentionLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::AttentionLayer, cv_PtrLcv_dnn_AttentionLayerG_new_const_AttentionLayer }
impl core::Ptr<crate::dnn::AttentionLayer> {
	#[inline] pub fn as_raw_PtrOfAttentionLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAttentionLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::AttentionLayerTraitConst for core::Ptr<crate::dnn::AttentionLayer> {
	#[inline] fn as_raw_AttentionLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::AttentionLayerTrait for core::Ptr<crate::dnn::AttentionLayer> {
	#[inline] fn as_raw_mut_AttentionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AttentionLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::AttentionLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::AttentionLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_AttentionLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AttentionLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AttentionLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::AttentionLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_AttentionLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::AttentionLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfAttentionLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

