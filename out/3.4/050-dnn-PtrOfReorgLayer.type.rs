ptr_extern! { crate::dnn::ReorgLayer,
	cv_PtrLcv_dnn_ReorgLayerG_new_null_const, cv_PtrLcv_dnn_ReorgLayerG_delete, cv_PtrLcv_dnn_ReorgLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ReorgLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::ReorgLayer, cv_PtrLcv_dnn_ReorgLayerG_new_const_ReorgLayer }
impl core::Ptr<crate::dnn::ReorgLayer> {
	#[inline] pub fn as_raw_PtrOfReorgLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfReorgLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::ReorgLayerTraitConst for core::Ptr<crate::dnn::ReorgLayer> {
	#[inline] fn as_raw_ReorgLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ReorgLayerTrait for core::Ptr<crate::dnn::ReorgLayer> {
	#[inline] fn as_raw_mut_ReorgLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReorgLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReorgLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ReorgLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ReorgLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReorgLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReorgLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ReorgLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ReorgLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::ReorgLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfReorgLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

