ptr_extern! { crate::dnn::ScaleLayer,
	cv_PtrLcv_dnn_ScaleLayerG_new_null_const, cv_PtrLcv_dnn_ScaleLayerG_delete, cv_PtrLcv_dnn_ScaleLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ScaleLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::ScaleLayer, cv_PtrLcv_dnn_ScaleLayerG_new_const_ScaleLayer }
impl core::Ptr<crate::dnn::ScaleLayer> {
	#[inline] pub fn as_raw_PtrOfScaleLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfScaleLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::ScaleLayerTraitConst for core::Ptr<crate::dnn::ScaleLayer> {
	#[inline] fn as_raw_ScaleLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ScaleLayerTrait for core::Ptr<crate::dnn::ScaleLayer> {
	#[inline] fn as_raw_mut_ScaleLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ScaleLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::ScaleLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ScaleLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ScaleLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ScaleLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ScaleLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ScaleLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ScaleLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::ScaleLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfScaleLayer")
			.field("has_bias", &crate::dnn::ScaleLayerTraitConst::has_bias(self))
			.field("axis", &crate::dnn::ScaleLayerTraitConst::axis(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

