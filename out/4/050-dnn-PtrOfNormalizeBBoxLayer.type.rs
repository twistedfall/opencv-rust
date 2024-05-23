ptr_extern! { crate::dnn::NormalizeBBoxLayer,
	cv_PtrLcv_dnn_NormalizeBBoxLayerG_new_null_const, cv_PtrLcv_dnn_NormalizeBBoxLayerG_delete, cv_PtrLcv_dnn_NormalizeBBoxLayerG_getInnerPtr_const, cv_PtrLcv_dnn_NormalizeBBoxLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::NormalizeBBoxLayer, cv_PtrLcv_dnn_NormalizeBBoxLayerG_new_const_NormalizeBBoxLayer }
impl core::Ptr<crate::dnn::NormalizeBBoxLayer> {
	#[inline] pub fn as_raw_PtrOfNormalizeBBoxLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfNormalizeBBoxLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::NormalizeBBoxLayerTraitConst for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
	#[inline] fn as_raw_NormalizeBBoxLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::NormalizeBBoxLayerTrait for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
	#[inline] fn as_raw_mut_NormalizeBBoxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::NormalizeBBoxLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_NormalizeBBoxLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::NormalizeBBoxLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_NormalizeBBoxLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfNormalizeBBoxLayer")
			.field("pnorm", &crate::dnn::NormalizeBBoxLayerTraitConst::pnorm(self))
			.field("epsilon", &crate::dnn::NormalizeBBoxLayerTraitConst::epsilon(self))
			.field("across_spatial", &crate::dnn::NormalizeBBoxLayerTraitConst::across_spatial(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

