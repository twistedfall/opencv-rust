ptr_extern! { crate::dnn::LRNLayer,
	cv_PtrLcv_dnn_LRNLayerG_new_null_const, cv_PtrLcv_dnn_LRNLayerG_delete, cv_PtrLcv_dnn_LRNLayerG_getInnerPtr_const, cv_PtrLcv_dnn_LRNLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::LRNLayer, cv_PtrLcv_dnn_LRNLayerG_new_const_LRNLayer }
impl core::Ptr<crate::dnn::LRNLayer> {
	#[inline] pub fn as_raw_PtrOfLRNLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLRNLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::LRNLayerTraitConst for core::Ptr<crate::dnn::LRNLayer> {
	#[inline] fn as_raw_LRNLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LRNLayerTrait for core::Ptr<crate::dnn::LRNLayer> {
	#[inline] fn as_raw_mut_LRNLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::LRNLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::LRNLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::LRNLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_LRNLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::LRNLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::LRNLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::LRNLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_LRNLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::LRNLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfLRNLayer")
			.field("typ", &crate::dnn::LRNLayerTraitConst::typ(self))
			.field("size", &crate::dnn::LRNLayerTraitConst::size(self))
			.field("alpha", &crate::dnn::LRNLayerTraitConst::alpha(self))
			.field("beta", &crate::dnn::LRNLayerTraitConst::beta(self))
			.field("bias", &crate::dnn::LRNLayerTraitConst::bias(self))
			.field("norm_by_size", &crate::dnn::LRNLayerTraitConst::norm_by_size(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

