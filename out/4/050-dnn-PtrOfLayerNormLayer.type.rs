ptr_extern! { crate::dnn::LayerNormLayer,
	cv_PtrLcv_dnn_LayerNormLayerG_new_null_const, cv_PtrLcv_dnn_LayerNormLayerG_delete, cv_PtrLcv_dnn_LayerNormLayerG_getInnerPtr_const, cv_PtrLcv_dnn_LayerNormLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::LayerNormLayer, cv_PtrLcv_dnn_LayerNormLayerG_new_const_LayerNormLayer }
impl core::Ptr<crate::dnn::LayerNormLayer> {
	#[inline] pub fn as_raw_PtrOfLayerNormLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLayerNormLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::LayerNormLayerTraitConst for core::Ptr<crate::dnn::LayerNormLayer> {
	#[inline] fn as_raw_LayerNormLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerNormLayerTrait for core::Ptr<crate::dnn::LayerNormLayer> {
	#[inline] fn as_raw_mut_LayerNormLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::LayerNormLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::LayerNormLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::LayerNormLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_LayerNormLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::LayerNormLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::LayerNormLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::LayerNormLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_LayerNormLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::LayerNormLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfLayerNormLayer")
			.field("has_bias", &crate::dnn::LayerNormLayerTraitConst::has_bias(self))
			.field("axis", &crate::dnn::LayerNormLayerTraitConst::axis(self))
			.field("epsilon", &crate::dnn::LayerNormLayerTraitConst::epsilon(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

