ptr_extern! { crate::dnn::QuantizeLayer,
	cv_PtrLcv_dnn_QuantizeLayerG_new_null_const, cv_PtrLcv_dnn_QuantizeLayerG_delete, cv_PtrLcv_dnn_QuantizeLayerG_getInnerPtr_const, cv_PtrLcv_dnn_QuantizeLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::QuantizeLayer, cv_PtrLcv_dnn_QuantizeLayerG_new_const_QuantizeLayer }
impl core::Ptr<crate::dnn::QuantizeLayer> {
	#[inline] pub fn as_raw_PtrOfQuantizeLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfQuantizeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::QuantizeLayerTraitConst for core::Ptr<crate::dnn::QuantizeLayer> {
	#[inline] fn as_raw_QuantizeLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::QuantizeLayerTrait for core::Ptr<crate::dnn::QuantizeLayer> {
	#[inline] fn as_raw_mut_QuantizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::QuantizeLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::QuantizeLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::QuantizeLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_QuantizeLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::QuantizeLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::QuantizeLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::QuantizeLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_QuantizeLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::QuantizeLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfQuantizeLayer")
			.field("scales", &crate::dnn::QuantizeLayerTraitConst::scales(self))
			.field("zeropoints", &crate::dnn::QuantizeLayerTraitConst::zeropoints(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

