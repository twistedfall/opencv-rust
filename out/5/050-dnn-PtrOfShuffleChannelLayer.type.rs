ptr_extern! { crate::dnn::ShuffleChannelLayer,
	cv_PtrLcv_dnn_ShuffleChannelLayerG_new_null_const, cv_PtrLcv_dnn_ShuffleChannelLayerG_delete, cv_PtrLcv_dnn_ShuffleChannelLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ShuffleChannelLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::ShuffleChannelLayer, cv_PtrLcv_dnn_ShuffleChannelLayerG_new_const_ShuffleChannelLayer }
impl core::Ptr<crate::dnn::ShuffleChannelLayer> {
	#[inline] pub fn as_raw_PtrOfShuffleChannelLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfShuffleChannelLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::ShuffleChannelLayerTraitConst for core::Ptr<crate::dnn::ShuffleChannelLayer> {
	#[inline] fn as_raw_ShuffleChannelLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ShuffleChannelLayerTrait for core::Ptr<crate::dnn::ShuffleChannelLayer> {
	#[inline] fn as_raw_mut_ShuffleChannelLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ShuffleChannelLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::ShuffleChannelLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ShuffleChannelLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ShuffleChannelLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ShuffleChannelLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ShuffleChannelLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ShuffleChannelLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ShuffleChannelLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::ShuffleChannelLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfShuffleChannelLayer")
			.field("group", &crate::dnn::ShuffleChannelLayerTraitConst::group(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("inputs", &crate::dnn::LayerTraitConst::inputs(self))
			.field("outputs", &crate::dnn::LayerTraitConst::outputs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

