ptr_extern! { crate::dnn::SpaceToDepthLayer,
	cv_PtrLcv_dnn_SpaceToDepthLayerG_new_null_const, cv_PtrLcv_dnn_SpaceToDepthLayerG_delete, cv_PtrLcv_dnn_SpaceToDepthLayerG_getInnerPtr_const, cv_PtrLcv_dnn_SpaceToDepthLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::SpaceToDepthLayer, cv_PtrLcv_dnn_SpaceToDepthLayerG_new_const_SpaceToDepthLayer }
impl core::Ptr<crate::dnn::SpaceToDepthLayer> {
	#[inline] pub fn as_raw_PtrOfSpaceToDepthLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSpaceToDepthLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::SpaceToDepthLayerTraitConst for core::Ptr<crate::dnn::SpaceToDepthLayer> {
	#[inline] fn as_raw_SpaceToDepthLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::SpaceToDepthLayerTrait for core::Ptr<crate::dnn::SpaceToDepthLayer> {
	#[inline] fn as_raw_mut_SpaceToDepthLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SpaceToDepthLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::SpaceToDepthLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::SpaceToDepthLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_SpaceToDepthLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SpaceToDepthLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SpaceToDepthLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::SpaceToDepthLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_SpaceToDepthLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::SpaceToDepthLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSpaceToDepthLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

