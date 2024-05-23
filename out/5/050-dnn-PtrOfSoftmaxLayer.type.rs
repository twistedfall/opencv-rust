ptr_extern! { crate::dnn::SoftmaxLayer,
	cv_PtrLcv_dnn_SoftmaxLayerG_new_null_const, cv_PtrLcv_dnn_SoftmaxLayerG_delete, cv_PtrLcv_dnn_SoftmaxLayerG_getInnerPtr_const, cv_PtrLcv_dnn_SoftmaxLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::SoftmaxLayer, cv_PtrLcv_dnn_SoftmaxLayerG_new_const_SoftmaxLayer }
impl core::Ptr<crate::dnn::SoftmaxLayer> {
	#[inline] pub fn as_raw_PtrOfSoftmaxLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSoftmaxLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::SoftmaxLayerTraitConst for core::Ptr<crate::dnn::SoftmaxLayer> {
	#[inline] fn as_raw_SoftmaxLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::SoftmaxLayerTrait for core::Ptr<crate::dnn::SoftmaxLayer> {
	#[inline] fn as_raw_mut_SoftmaxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SoftmaxLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::SoftmaxLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::SoftmaxLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_SoftmaxLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SoftmaxLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SoftmaxLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::SoftmaxLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_SoftmaxLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::SoftmaxLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSoftmaxLayer")
			.field("log_soft_max", &crate::dnn::SoftmaxLayerTraitConst::log_soft_max(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("inputs", &crate::dnn::LayerTraitConst::inputs(self))
			.field("outputs", &crate::dnn::LayerTraitConst::outputs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

