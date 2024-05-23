ptr_extern! { crate::dnn::GatherElementsLayer,
	cv_PtrLcv_dnn_GatherElementsLayerG_new_null_const, cv_PtrLcv_dnn_GatherElementsLayerG_delete, cv_PtrLcv_dnn_GatherElementsLayerG_getInnerPtr_const, cv_PtrLcv_dnn_GatherElementsLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::GatherElementsLayer, cv_PtrLcv_dnn_GatherElementsLayerG_new_const_GatherElementsLayer }
impl core::Ptr<crate::dnn::GatherElementsLayer> {
	#[inline] pub fn as_raw_PtrOfGatherElementsLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGatherElementsLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::GatherElementsLayerTraitConst for core::Ptr<crate::dnn::GatherElementsLayer> {
	#[inline] fn as_raw_GatherElementsLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::GatherElementsLayerTrait for core::Ptr<crate::dnn::GatherElementsLayer> {
	#[inline] fn as_raw_mut_GatherElementsLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::GatherElementsLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::GatherElementsLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::GatherElementsLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_GatherElementsLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::GatherElementsLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::GatherElementsLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::GatherElementsLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_GatherElementsLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::GatherElementsLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfGatherElementsLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("inputs", &crate::dnn::LayerTraitConst::inputs(self))
			.field("outputs", &crate::dnn::LayerTraitConst::outputs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

