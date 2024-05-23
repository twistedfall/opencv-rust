ptr_extern! { crate::dnn::GatherNDLayer,
	cv_PtrLcv_dnn_GatherNDLayerG_new_null_const, cv_PtrLcv_dnn_GatherNDLayerG_delete, cv_PtrLcv_dnn_GatherNDLayerG_getInnerPtr_const, cv_PtrLcv_dnn_GatherNDLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::GatherNDLayer, cv_PtrLcv_dnn_GatherNDLayerG_new_const_GatherNDLayer }
impl core::Ptr<crate::dnn::GatherNDLayer> {
	#[inline] pub fn as_raw_PtrOfGatherNDLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGatherNDLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::GatherNDLayerTraitConst for core::Ptr<crate::dnn::GatherNDLayer> {
	#[inline] fn as_raw_GatherNDLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::GatherNDLayerTrait for core::Ptr<crate::dnn::GatherNDLayer> {
	#[inline] fn as_raw_mut_GatherNDLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::GatherNDLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::GatherNDLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::GatherNDLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_GatherNDLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::GatherNDLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::GatherNDLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::GatherNDLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_GatherNDLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::GatherNDLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfGatherNDLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("inputs", &crate::dnn::LayerTraitConst::inputs(self))
			.field("outputs", &crate::dnn::LayerTraitConst::outputs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

