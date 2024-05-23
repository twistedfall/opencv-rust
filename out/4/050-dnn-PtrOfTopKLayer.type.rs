ptr_extern! { crate::dnn::TopKLayer,
	cv_PtrLcv_dnn_TopKLayerG_new_null_const, cv_PtrLcv_dnn_TopKLayerG_delete, cv_PtrLcv_dnn_TopKLayerG_getInnerPtr_const, cv_PtrLcv_dnn_TopKLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::TopKLayer, cv_PtrLcv_dnn_TopKLayerG_new_const_TopKLayer }
impl core::Ptr<crate::dnn::TopKLayer> {
	#[inline] pub fn as_raw_PtrOfTopKLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTopKLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::TopKLayerTraitConst for core::Ptr<crate::dnn::TopKLayer> {
	#[inline] fn as_raw_TopKLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::TopKLayerTrait for core::Ptr<crate::dnn::TopKLayer> {
	#[inline] fn as_raw_mut_TopKLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::TopKLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::TopKLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::TopKLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_TopKLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::TopKLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::TopKLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::TopKLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_TopKLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::TopKLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTopKLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

