ptr_extern! { crate::dnn::GeluLayer,
	cv_PtrLcv_dnn_GeluLayerG_new_null_const, cv_PtrLcv_dnn_GeluLayerG_delete, cv_PtrLcv_dnn_GeluLayerG_getInnerPtr_const, cv_PtrLcv_dnn_GeluLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::GeluLayer, cv_PtrLcv_dnn_GeluLayerG_new_const_GeluLayer }
impl core::Ptr<crate::dnn::GeluLayer> {
	#[inline] pub fn as_raw_PtrOfGeluLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGeluLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::GeluLayerTraitConst for core::Ptr<crate::dnn::GeluLayer> {
	#[inline] fn as_raw_GeluLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::GeluLayerTrait for core::Ptr<crate::dnn::GeluLayer> {
	#[inline] fn as_raw_mut_GeluLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::GeluLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::GeluLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::GeluLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_GeluLayerG_to_PtrOfAlgorithm }

impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::GeluLayer> {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::GeluLayer> {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::GeluLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_GeluLayerG_to_PtrOfActivationLayer }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::GeluLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::GeluLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::GeluLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_GeluLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::GeluLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfGeluLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("inputs", &crate::dnn::LayerTraitConst::inputs(self))
			.field("outputs", &crate::dnn::LayerTraitConst::outputs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

