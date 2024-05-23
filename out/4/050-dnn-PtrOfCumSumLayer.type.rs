ptr_extern! { crate::dnn::CumSumLayer,
	cv_PtrLcv_dnn_CumSumLayerG_new_null_const, cv_PtrLcv_dnn_CumSumLayerG_delete, cv_PtrLcv_dnn_CumSumLayerG_getInnerPtr_const, cv_PtrLcv_dnn_CumSumLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::CumSumLayer, cv_PtrLcv_dnn_CumSumLayerG_new_const_CumSumLayer }
impl core::Ptr<crate::dnn::CumSumLayer> {
	#[inline] pub fn as_raw_PtrOfCumSumLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCumSumLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::CumSumLayerTraitConst for core::Ptr<crate::dnn::CumSumLayer> {
	#[inline] fn as_raw_CumSumLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::CumSumLayerTrait for core::Ptr<crate::dnn::CumSumLayer> {
	#[inline] fn as_raw_mut_CumSumLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CumSumLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::CumSumLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::CumSumLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_CumSumLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CumSumLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CumSumLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::CumSumLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_CumSumLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::CumSumLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCumSumLayer")
			.field("exclusive", &crate::dnn::CumSumLayerTraitConst::exclusive(self))
			.field("reverse", &crate::dnn::CumSumLayerTraitConst::reverse(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

