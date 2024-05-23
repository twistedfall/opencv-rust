ptr_extern! { crate::dnn::HardSigmoidLayer,
	cv_PtrLcv_dnn_HardSigmoidLayerG_new_null_const, cv_PtrLcv_dnn_HardSigmoidLayerG_delete, cv_PtrLcv_dnn_HardSigmoidLayerG_getInnerPtr_const, cv_PtrLcv_dnn_HardSigmoidLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::HardSigmoidLayer, cv_PtrLcv_dnn_HardSigmoidLayerG_new_const_HardSigmoidLayer }
impl core::Ptr<crate::dnn::HardSigmoidLayer> {
	#[inline] pub fn as_raw_PtrOfHardSigmoidLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfHardSigmoidLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::HardSigmoidLayerTraitConst for core::Ptr<crate::dnn::HardSigmoidLayer> {
	#[inline] fn as_raw_HardSigmoidLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::HardSigmoidLayerTrait for core::Ptr<crate::dnn::HardSigmoidLayer> {
	#[inline] fn as_raw_mut_HardSigmoidLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::HardSigmoidLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::HardSigmoidLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::HardSigmoidLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_HardSigmoidLayerG_to_PtrOfAlgorithm }

impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::HardSigmoidLayer> {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::HardSigmoidLayer> {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::HardSigmoidLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_HardSigmoidLayerG_to_PtrOfActivationLayer }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::HardSigmoidLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::HardSigmoidLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::HardSigmoidLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_HardSigmoidLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::HardSigmoidLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfHardSigmoidLayer")
			.field("alpha", &crate::dnn::HardSigmoidLayerTraitConst::alpha(self))
			.field("beta", &crate::dnn::HardSigmoidLayerTraitConst::beta(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

