ptr_extern! { crate::dnn::MishLayer,
	cv_PtrLcv_dnn_MishLayerG_new_null_const, cv_PtrLcv_dnn_MishLayerG_delete, cv_PtrLcv_dnn_MishLayerG_getInnerPtr_const, cv_PtrLcv_dnn_MishLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::MishLayer, cv_PtrLcv_dnn_MishLayerG_new_const_MishLayer }
impl core::Ptr<crate::dnn::MishLayer> {
	#[inline] pub fn as_raw_PtrOfMishLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMishLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::MishLayerTraitConst for core::Ptr<crate::dnn::MishLayer> {
	#[inline] fn as_raw_MishLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::MishLayerTrait for core::Ptr<crate::dnn::MishLayer> {
	#[inline] fn as_raw_mut_MishLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::MishLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::MishLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::MishLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_MishLayerG_to_PtrOfAlgorithm }

impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::MishLayer> {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::MishLayer> {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::MishLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_MishLayerG_to_PtrOfActivationLayer }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::MishLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::MishLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::MishLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_MishLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::MishLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMishLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("inputs", &crate::dnn::LayerTraitConst::inputs(self))
			.field("outputs", &crate::dnn::LayerTraitConst::outputs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

