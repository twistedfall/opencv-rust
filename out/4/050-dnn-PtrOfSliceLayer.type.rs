ptr_extern! { crate::dnn::SliceLayer,
	cv_PtrLcv_dnn_SliceLayerG_new_null_const, cv_PtrLcv_dnn_SliceLayerG_delete, cv_PtrLcv_dnn_SliceLayerG_getInnerPtr_const, cv_PtrLcv_dnn_SliceLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::SliceLayer, cv_PtrLcv_dnn_SliceLayerG_new_const_SliceLayer }
impl core::Ptr<crate::dnn::SliceLayer> {
	#[inline] pub fn as_raw_PtrOfSliceLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSliceLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::SliceLayerTraitConst for core::Ptr<crate::dnn::SliceLayer> {
	#[inline] fn as_raw_SliceLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::SliceLayerTrait for core::Ptr<crate::dnn::SliceLayer> {
	#[inline] fn as_raw_mut_SliceLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SliceLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::SliceLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::SliceLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_SliceLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SliceLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SliceLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::SliceLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_SliceLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::SliceLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSliceLayer")
			.field("slice_ranges", &crate::dnn::SliceLayerTraitConst::slice_ranges(self))
			.field("slice_steps", &crate::dnn::SliceLayerTraitConst::slice_steps(self))
			.field("axis", &crate::dnn::SliceLayerTraitConst::axis(self))
			.field("num_split", &crate::dnn::SliceLayerTraitConst::num_split(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

