ptr_extern! { crate::dnn::QuantizeLinearLayer,
	cv_PtrLcv_dnn_QuantizeLinearLayerG_new_null_const, cv_PtrLcv_dnn_QuantizeLinearLayerG_delete, cv_PtrLcv_dnn_QuantizeLinearLayerG_getInnerPtr_const, cv_PtrLcv_dnn_QuantizeLinearLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::QuantizeLinearLayer, cv_PtrLcv_dnn_QuantizeLinearLayerG_new_const_QuantizeLinearLayer }
impl core::Ptr<crate::dnn::QuantizeLinearLayer> {
	#[inline] pub fn as_raw_PtrOfQuantizeLinearLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfQuantizeLinearLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::QuantizeLinearLayerTraitConst for core::Ptr<crate::dnn::QuantizeLinearLayer> {
	#[inline] fn as_raw_QuantizeLinearLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::QuantizeLinearLayerTrait for core::Ptr<crate::dnn::QuantizeLinearLayer> {
	#[inline] fn as_raw_mut_QuantizeLinearLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::QuantizeLinearLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::QuantizeLinearLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::QuantizeLinearLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_QuantizeLinearLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::QuantizeLinearLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::QuantizeLinearLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::QuantizeLinearLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_QuantizeLinearLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::QuantizeLinearLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfQuantizeLinearLayer")
			.field("axis", &crate::dnn::QuantizeLinearLayerTraitConst::axis(self))
			.field("block_size", &crate::dnn::QuantizeLinearLayerTraitConst::block_size(self))
			.field("output_dtype", &crate::dnn::QuantizeLinearLayerTraitConst::output_dtype(self))
			.field("saturate", &crate::dnn::QuantizeLinearLayerTraitConst::saturate(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("inputs", &crate::dnn::LayerTraitConst::inputs(self))
			.field("outputs", &crate::dnn::LayerTraitConst::outputs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

