ptr_extern! { crate::dnn::ELULayer,
	cv_PtrLcv_dnn_ELULayerG_new_null_const, cv_PtrLcv_dnn_ELULayerG_delete, cv_PtrLcv_dnn_ELULayerG_getInnerPtr_const, cv_PtrLcv_dnn_ELULayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::ELULayer, cv_PtrLcv_dnn_ELULayerG_new_const_ELULayer }
impl core::Ptr<crate::dnn::ELULayer> {
	#[inline] pub fn as_raw_PtrOfELULayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfELULayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::ELULayerTraitConst for core::Ptr<crate::dnn::ELULayer> {
	#[inline] fn as_raw_ELULayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ELULayerTrait for core::Ptr<crate::dnn::ELULayer> {
	#[inline] fn as_raw_mut_ELULayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ELULayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::ELULayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ELULayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ELULayerG_to_PtrOfAlgorithm }

impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ELULayer> {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ELULayer> {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ELULayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_ELULayerG_to_PtrOfActivationLayer }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ELULayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ELULayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ELULayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ELULayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::ELULayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfELULayer")
			.field("alpha", &crate::dnn::ELULayerTraitConst::alpha(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

