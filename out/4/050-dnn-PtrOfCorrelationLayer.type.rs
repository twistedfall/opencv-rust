ptr_extern! { crate::dnn::CorrelationLayer,
	cv_PtrLcv_dnn_CorrelationLayerG_new_null_const, cv_PtrLcv_dnn_CorrelationLayerG_delete, cv_PtrLcv_dnn_CorrelationLayerG_getInnerPtr_const, cv_PtrLcv_dnn_CorrelationLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::CorrelationLayer, cv_PtrLcv_dnn_CorrelationLayerG_new_const_CorrelationLayer }
impl core::Ptr<crate::dnn::CorrelationLayer> {
	#[inline] pub fn as_raw_PtrOfCorrelationLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCorrelationLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::CorrelationLayerTraitConst for core::Ptr<crate::dnn::CorrelationLayer> {
	#[inline] fn as_raw_CorrelationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::CorrelationLayerTrait for core::Ptr<crate::dnn::CorrelationLayer> {
	#[inline] fn as_raw_mut_CorrelationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CorrelationLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::CorrelationLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::CorrelationLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_CorrelationLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CorrelationLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CorrelationLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::CorrelationLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_CorrelationLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::CorrelationLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCorrelationLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

