ptr_extern! { crate::dnn::BatchNormLayerInt8,
	cv_PtrLcv_dnn_BatchNormLayerInt8G_new_null_const, cv_PtrLcv_dnn_BatchNormLayerInt8G_delete, cv_PtrLcv_dnn_BatchNormLayerInt8G_getInnerPtr_const, cv_PtrLcv_dnn_BatchNormLayerInt8G_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::BatchNormLayerInt8, cv_PtrLcv_dnn_BatchNormLayerInt8G_new_const_BatchNormLayerInt8 }
impl core::Ptr<crate::dnn::BatchNormLayerInt8> {
	#[inline] pub fn as_raw_PtrOfBatchNormLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBatchNormLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::BatchNormLayerInt8TraitConst for core::Ptr<crate::dnn::BatchNormLayerInt8> {
	#[inline] fn as_raw_BatchNormLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::BatchNormLayerInt8Trait for core::Ptr<crate::dnn::BatchNormLayerInt8> {
	#[inline] fn as_raw_mut_BatchNormLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::BatchNormLayerInt8> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::BatchNormLayerInt8> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::BatchNormLayerInt8>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_BatchNormLayerInt8G_to_PtrOfAlgorithm }

impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::BatchNormLayerInt8> {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::BatchNormLayerInt8> {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::BatchNormLayerInt8>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_BatchNormLayerInt8G_to_PtrOfActivationLayer }

impl crate::dnn::BatchNormLayerTraitConst for core::Ptr<crate::dnn::BatchNormLayerInt8> {
	#[inline] fn as_raw_BatchNormLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::BatchNormLayerTrait for core::Ptr<crate::dnn::BatchNormLayerInt8> {
	#[inline] fn as_raw_mut_BatchNormLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::BatchNormLayerInt8>, core::Ptr<crate::dnn::BatchNormLayer>, cv_PtrLcv_dnn_BatchNormLayerInt8G_to_PtrOfBatchNormLayer }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::BatchNormLayerInt8> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::BatchNormLayerInt8> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::BatchNormLayerInt8>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_BatchNormLayerInt8G_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::BatchNormLayerInt8> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfBatchNormLayerInt8")
			.field("input_sc", &crate::dnn::BatchNormLayerInt8TraitConst::input_sc(self))
			.field("output_sc", &crate::dnn::BatchNormLayerInt8TraitConst::output_sc(self))
			.field("input_zp", &crate::dnn::BatchNormLayerInt8TraitConst::input_zp(self))
			.field("output_zp", &crate::dnn::BatchNormLayerInt8TraitConst::output_zp(self))
			.field("has_weights", &crate::dnn::BatchNormLayerTraitConst::has_weights(self))
			.field("has_bias", &crate::dnn::BatchNormLayerTraitConst::has_bias(self))
			.field("epsilon", &crate::dnn::BatchNormLayerTraitConst::epsilon(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("inputs", &crate::dnn::LayerTraitConst::inputs(self))
			.field("outputs", &crate::dnn::LayerTraitConst::outputs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

