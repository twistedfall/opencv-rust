ptr_extern! { crate::dnn::ShrinkLayer,
	cv_PtrLcv_dnn_ShrinkLayerG_new_null_const, cv_PtrLcv_dnn_ShrinkLayerG_delete, cv_PtrLcv_dnn_ShrinkLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ShrinkLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::ShrinkLayer, cv_PtrLcv_dnn_ShrinkLayerG_new_const_ShrinkLayer }
impl core::Ptr<crate::dnn::ShrinkLayer> {
	#[inline] pub fn as_raw_PtrOfShrinkLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfShrinkLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::ShrinkLayerTraitConst for core::Ptr<crate::dnn::ShrinkLayer> {
	#[inline] fn as_raw_ShrinkLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ShrinkLayerTrait for core::Ptr<crate::dnn::ShrinkLayer> {
	#[inline] fn as_raw_mut_ShrinkLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ShrinkLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::ShrinkLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ShrinkLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ShrinkLayerG_to_PtrOfAlgorithm }

impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ShrinkLayer> {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ShrinkLayer> {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ShrinkLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_ShrinkLayerG_to_PtrOfActivationLayer }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ShrinkLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ShrinkLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ShrinkLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ShrinkLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::ShrinkLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfShrinkLayer")
			.field("bias", &crate::dnn::ShrinkLayerTraitConst::bias(self))
			.field("lambd", &crate::dnn::ShrinkLayerTraitConst::lambd(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("inputs", &crate::dnn::LayerTraitConst::inputs(self))
			.field("outputs", &crate::dnn::LayerTraitConst::outputs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

