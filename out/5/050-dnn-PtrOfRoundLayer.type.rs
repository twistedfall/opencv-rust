ptr_extern! { crate::dnn::RoundLayer,
	cv_PtrLcv_dnn_RoundLayerG_new_null_const, cv_PtrLcv_dnn_RoundLayerG_delete, cv_PtrLcv_dnn_RoundLayerG_getInnerPtr_const, cv_PtrLcv_dnn_RoundLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::RoundLayer, cv_PtrLcv_dnn_RoundLayerG_new_const_RoundLayer }
impl core::Ptr<crate::dnn::RoundLayer> {
	#[inline] pub fn as_raw_PtrOfRoundLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRoundLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::RoundLayerTraitConst for core::Ptr<crate::dnn::RoundLayer> {
	#[inline] fn as_raw_RoundLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::RoundLayerTrait for core::Ptr<crate::dnn::RoundLayer> {
	#[inline] fn as_raw_mut_RoundLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::RoundLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::RoundLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::RoundLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_RoundLayerG_to_PtrOfAlgorithm }

impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::RoundLayer> {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::RoundLayer> {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::RoundLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_RoundLayerG_to_PtrOfActivationLayer }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::RoundLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::RoundLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::RoundLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_RoundLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::RoundLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfRoundLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("inputs", &crate::dnn::LayerTraitConst::inputs(self))
			.field("outputs", &crate::dnn::LayerTraitConst::outputs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

