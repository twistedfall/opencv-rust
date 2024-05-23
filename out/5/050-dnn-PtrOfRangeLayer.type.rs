ptr_extern! { crate::dnn::RangeLayer,
	cv_PtrLcv_dnn_RangeLayerG_new_null_const, cv_PtrLcv_dnn_RangeLayerG_delete, cv_PtrLcv_dnn_RangeLayerG_getInnerPtr_const, cv_PtrLcv_dnn_RangeLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::RangeLayer, cv_PtrLcv_dnn_RangeLayerG_new_const_RangeLayer }
impl core::Ptr<crate::dnn::RangeLayer> {
	#[inline] pub fn as_raw_PtrOfRangeLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRangeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::RangeLayerTraitConst for core::Ptr<crate::dnn::RangeLayer> {
	#[inline] fn as_raw_RangeLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::RangeLayerTrait for core::Ptr<crate::dnn::RangeLayer> {
	#[inline] fn as_raw_mut_RangeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::RangeLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::RangeLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::RangeLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_RangeLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::RangeLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::RangeLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::RangeLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_RangeLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::RangeLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfRangeLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("inputs", &crate::dnn::LayerTraitConst::inputs(self))
			.field("outputs", &crate::dnn::LayerTraitConst::outputs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

