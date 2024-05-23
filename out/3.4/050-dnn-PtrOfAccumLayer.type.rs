ptr_extern! { crate::dnn::AccumLayer,
	cv_PtrLcv_dnn_AccumLayerG_new_null_const, cv_PtrLcv_dnn_AccumLayerG_delete, cv_PtrLcv_dnn_AccumLayerG_getInnerPtr_const, cv_PtrLcv_dnn_AccumLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::AccumLayer, cv_PtrLcv_dnn_AccumLayerG_new_const_AccumLayer }
impl core::Ptr<crate::dnn::AccumLayer> {
	#[inline] pub fn as_raw_PtrOfAccumLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAccumLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::AccumLayerTraitConst for core::Ptr<crate::dnn::AccumLayer> {
	#[inline] fn as_raw_AccumLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::AccumLayerTrait for core::Ptr<crate::dnn::AccumLayer> {
	#[inline] fn as_raw_mut_AccumLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AccumLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::AccumLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::AccumLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_AccumLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AccumLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AccumLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::AccumLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_AccumLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::AccumLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfAccumLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

