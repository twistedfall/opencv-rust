ptr_extern! { crate::dnn::GemmLayer,
	cv_PtrLcv_dnn_GemmLayerG_new_null_const, cv_PtrLcv_dnn_GemmLayerG_delete, cv_PtrLcv_dnn_GemmLayerG_getInnerPtr_const, cv_PtrLcv_dnn_GemmLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::GemmLayer, cv_PtrLcv_dnn_GemmLayerG_new_const_GemmLayer }
impl core::Ptr<crate::dnn::GemmLayer> {
	#[inline] pub fn as_raw_PtrOfGemmLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGemmLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::GemmLayerTraitConst for core::Ptr<crate::dnn::GemmLayer> {
	#[inline] fn as_raw_GemmLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::GemmLayerTrait for core::Ptr<crate::dnn::GemmLayer> {
	#[inline] fn as_raw_mut_GemmLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::GemmLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::GemmLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::GemmLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_GemmLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::GemmLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::GemmLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::GemmLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_GemmLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::GemmLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfGemmLayer")
			.field("trans_a", &crate::dnn::GemmLayerTraitConst::trans_a(self))
			.field("trans_b", &crate::dnn::GemmLayerTraitConst::trans_b(self))
			.field("alpha", &crate::dnn::GemmLayerTraitConst::alpha(self))
			.field("beta", &crate::dnn::GemmLayerTraitConst::beta(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("inputs", &crate::dnn::LayerTraitConst::inputs(self))
			.field("outputs", &crate::dnn::LayerTraitConst::outputs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

