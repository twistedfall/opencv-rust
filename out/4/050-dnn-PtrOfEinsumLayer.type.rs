ptr_extern! { crate::dnn::EinsumLayer,
	cv_PtrLcv_dnn_EinsumLayerG_new_null_const, cv_PtrLcv_dnn_EinsumLayerG_delete, cv_PtrLcv_dnn_EinsumLayerG_getInnerPtr_const, cv_PtrLcv_dnn_EinsumLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::EinsumLayer, cv_PtrLcv_dnn_EinsumLayerG_new_const_EinsumLayer }
impl core::Ptr<crate::dnn::EinsumLayer> {
	#[inline] pub fn as_raw_PtrOfEinsumLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfEinsumLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::EinsumLayerTraitConst for core::Ptr<crate::dnn::EinsumLayer> {
	#[inline] fn as_raw_EinsumLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::EinsumLayerTrait for core::Ptr<crate::dnn::EinsumLayer> {
	#[inline] fn as_raw_mut_EinsumLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::EinsumLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::EinsumLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::EinsumLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_EinsumLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::EinsumLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::EinsumLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::EinsumLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_EinsumLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::EinsumLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfEinsumLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

