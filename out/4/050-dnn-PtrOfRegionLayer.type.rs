ptr_extern! { crate::dnn::RegionLayer,
	cv_PtrLcv_dnn_RegionLayerG_new_null_const, cv_PtrLcv_dnn_RegionLayerG_delete, cv_PtrLcv_dnn_RegionLayerG_getInnerPtr_const, cv_PtrLcv_dnn_RegionLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::RegionLayer, cv_PtrLcv_dnn_RegionLayerG_new_const_RegionLayer }
impl core::Ptr<crate::dnn::RegionLayer> {
	#[inline] pub fn as_raw_PtrOfRegionLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRegionLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::RegionLayerTraitConst for core::Ptr<crate::dnn::RegionLayer> {
	#[inline] fn as_raw_RegionLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::RegionLayerTrait for core::Ptr<crate::dnn::RegionLayer> {
	#[inline] fn as_raw_mut_RegionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::RegionLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::RegionLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::RegionLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_RegionLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::RegionLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::RegionLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::RegionLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_RegionLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::RegionLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfRegionLayer")
			.field("nms_threshold", &crate::dnn::RegionLayerTraitConst::nms_threshold(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

