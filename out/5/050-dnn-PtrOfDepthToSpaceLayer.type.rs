ptr_extern! { crate::dnn::DepthToSpaceLayer,
	cv_PtrLcv_dnn_DepthToSpaceLayerG_new_null_const, cv_PtrLcv_dnn_DepthToSpaceLayerG_delete, cv_PtrLcv_dnn_DepthToSpaceLayerG_getInnerPtr_const, cv_PtrLcv_dnn_DepthToSpaceLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::DepthToSpaceLayer, cv_PtrLcv_dnn_DepthToSpaceLayerG_new_const_DepthToSpaceLayer }
impl core::Ptr<crate::dnn::DepthToSpaceLayer> {
	#[inline] pub fn as_raw_PtrOfDepthToSpaceLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDepthToSpaceLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::DepthToSpaceLayerTraitConst for core::Ptr<crate::dnn::DepthToSpaceLayer> {
	#[inline] fn as_raw_DepthToSpaceLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::DepthToSpaceLayerTrait for core::Ptr<crate::dnn::DepthToSpaceLayer> {
	#[inline] fn as_raw_mut_DepthToSpaceLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::DepthToSpaceLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::DepthToSpaceLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::DepthToSpaceLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_DepthToSpaceLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::DepthToSpaceLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::DepthToSpaceLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::DepthToSpaceLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_DepthToSpaceLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::DepthToSpaceLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDepthToSpaceLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("inputs", &crate::dnn::LayerTraitConst::inputs(self))
			.field("outputs", &crate::dnn::LayerTraitConst::outputs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

