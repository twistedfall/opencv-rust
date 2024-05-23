ptr_extern! { crate::dnn::AsinhLayer,
	cv_PtrLcv_dnn_AsinhLayerG_new_null_const, cv_PtrLcv_dnn_AsinhLayerG_delete, cv_PtrLcv_dnn_AsinhLayerG_getInnerPtr_const, cv_PtrLcv_dnn_AsinhLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::AsinhLayer, cv_PtrLcv_dnn_AsinhLayerG_new_const_AsinhLayer }
impl core::Ptr<crate::dnn::AsinhLayer> {
	#[inline] pub fn as_raw_PtrOfAsinhLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAsinhLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::AsinhLayerTraitConst for core::Ptr<crate::dnn::AsinhLayer> {
	#[inline] fn as_raw_AsinhLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::AsinhLayerTrait for core::Ptr<crate::dnn::AsinhLayer> {
	#[inline] fn as_raw_mut_AsinhLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AsinhLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::AsinhLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::AsinhLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_AsinhLayerG_to_PtrOfAlgorithm }

impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::AsinhLayer> {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::AsinhLayer> {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::AsinhLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_AsinhLayerG_to_PtrOfActivationLayer }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AsinhLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AsinhLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::AsinhLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_AsinhLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::AsinhLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfAsinhLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

