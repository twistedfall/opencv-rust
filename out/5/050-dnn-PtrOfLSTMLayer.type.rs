ptr_extern! { crate::dnn::LSTMLayer,
	cv_PtrLcv_dnn_LSTMLayerG_new_null_const, cv_PtrLcv_dnn_LSTMLayerG_delete, cv_PtrLcv_dnn_LSTMLayerG_getInnerPtr_const, cv_PtrLcv_dnn_LSTMLayerG_getInnerPtrMut
}

impl core::Ptr<crate::dnn::LSTMLayer> {
	#[inline] pub fn as_raw_PtrOfLSTMLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLSTMLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::LSTMLayerTraitConst for core::Ptr<crate::dnn::LSTMLayer> {
	#[inline] fn as_raw_LSTMLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LSTMLayerTrait for core::Ptr<crate::dnn::LSTMLayer> {
	#[inline] fn as_raw_mut_LSTMLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::LSTMLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::LSTMLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::LSTMLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_LSTMLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::LSTMLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::LSTMLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::LSTMLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_LSTMLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::LSTMLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfLSTMLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("inputs", &crate::dnn::LayerTraitConst::inputs(self))
			.field("outputs", &crate::dnn::LayerTraitConst::outputs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

