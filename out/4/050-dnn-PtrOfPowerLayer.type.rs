ptr_extern! { crate::dnn::PowerLayer,
	cv_PtrLcv_dnn_PowerLayerG_new_null_const, cv_PtrLcv_dnn_PowerLayerG_delete, cv_PtrLcv_dnn_PowerLayerG_getInnerPtr_const, cv_PtrLcv_dnn_PowerLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::PowerLayer, cv_PtrLcv_dnn_PowerLayerG_new_const_PowerLayer }
impl core::Ptr<crate::dnn::PowerLayer> {
	#[inline] pub fn as_raw_PtrOfPowerLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPowerLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::PowerLayerTraitConst for core::Ptr<crate::dnn::PowerLayer> {
	#[inline] fn as_raw_PowerLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::PowerLayerTrait for core::Ptr<crate::dnn::PowerLayer> {
	#[inline] fn as_raw_mut_PowerLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PowerLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::PowerLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::PowerLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_PowerLayerG_to_PtrOfAlgorithm }

impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::PowerLayer> {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::PowerLayer> {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::PowerLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_PowerLayerG_to_PtrOfActivationLayer }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PowerLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PowerLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::PowerLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_PowerLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::PowerLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfPowerLayer")
			.field("power", &crate::dnn::PowerLayerTraitConst::power(self))
			.field("scale", &crate::dnn::PowerLayerTraitConst::scale(self))
			.field("shift", &crate::dnn::PowerLayerTraitConst::shift(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

