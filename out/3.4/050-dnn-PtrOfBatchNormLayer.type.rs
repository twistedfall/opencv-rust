ptr_extern! { crate::dnn::BatchNormLayer,
	cv_PtrLcv_dnn_BatchNormLayerG_new_null_const, cv_PtrLcv_dnn_BatchNormLayerG_delete, cv_PtrLcv_dnn_BatchNormLayerG_getInnerPtr_const, cv_PtrLcv_dnn_BatchNormLayerG_getInnerPtrMut
}

impl core::Ptr<crate::dnn::BatchNormLayer> {
	#[inline] pub fn as_raw_PtrOfBatchNormLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBatchNormLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::BatchNormLayerTraitConst for core::Ptr<crate::dnn::BatchNormLayer> {
	#[inline] fn as_raw_BatchNormLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::BatchNormLayerTrait for core::Ptr<crate::dnn::BatchNormLayer> {
	#[inline] fn as_raw_mut_BatchNormLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::BatchNormLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::BatchNormLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::BatchNormLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_BatchNormLayerG_to_PtrOfAlgorithm }

impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::BatchNormLayer> {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::BatchNormLayer> {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::BatchNormLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_BatchNormLayerG_to_PtrOfActivationLayer }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::BatchNormLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::BatchNormLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::BatchNormLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_BatchNormLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::BatchNormLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfBatchNormLayer")
			.field("has_weights", &crate::dnn::BatchNormLayerTraitConst::has_weights(self))
			.field("has_bias", &crate::dnn::BatchNormLayerTraitConst::has_bias(self))
			.field("epsilon", &crate::dnn::BatchNormLayerTraitConst::epsilon(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

