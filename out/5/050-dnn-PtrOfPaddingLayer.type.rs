ptr_extern! { crate::dnn::PaddingLayer,
	cv_PtrLcv_dnn_PaddingLayerG_new_null_const, cv_PtrLcv_dnn_PaddingLayerG_delete, cv_PtrLcv_dnn_PaddingLayerG_getInnerPtr_const, cv_PtrLcv_dnn_PaddingLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::PaddingLayer, cv_PtrLcv_dnn_PaddingLayerG_new_const_PaddingLayer }
impl core::Ptr<crate::dnn::PaddingLayer> {
	#[inline] pub fn as_raw_PtrOfPaddingLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPaddingLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::PaddingLayerTraitConst for core::Ptr<crate::dnn::PaddingLayer> {
	#[inline] fn as_raw_PaddingLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::PaddingLayerTrait for core::Ptr<crate::dnn::PaddingLayer> {
	#[inline] fn as_raw_mut_PaddingLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PaddingLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::PaddingLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::PaddingLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_PaddingLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PaddingLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PaddingLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::PaddingLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_PaddingLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::PaddingLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfPaddingLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("inputs", &crate::dnn::LayerTraitConst::inputs(self))
			.field("outputs", &crate::dnn::LayerTraitConst::outputs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

