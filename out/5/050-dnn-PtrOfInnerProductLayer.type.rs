ptr_extern! { crate::dnn::InnerProductLayer,
	cv_PtrLcv_dnn_InnerProductLayerG_new_null_const, cv_PtrLcv_dnn_InnerProductLayerG_delete, cv_PtrLcv_dnn_InnerProductLayerG_getInnerPtr_const, cv_PtrLcv_dnn_InnerProductLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::InnerProductLayer, cv_PtrLcv_dnn_InnerProductLayerG_new_const_InnerProductLayer }
impl core::Ptr<crate::dnn::InnerProductLayer> {
	#[inline] pub fn as_raw_PtrOfInnerProductLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfInnerProductLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::InnerProductLayerTraitConst for core::Ptr<crate::dnn::InnerProductLayer> {
	#[inline] fn as_raw_InnerProductLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::InnerProductLayerTrait for core::Ptr<crate::dnn::InnerProductLayer> {
	#[inline] fn as_raw_mut_InnerProductLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::InnerProductLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::InnerProductLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::InnerProductLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_InnerProductLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::InnerProductLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::InnerProductLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::InnerProductLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_InnerProductLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::InnerProductLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfInnerProductLayer")
			.field("axis", &crate::dnn::InnerProductLayerTraitConst::axis(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("inputs", &crate::dnn::LayerTraitConst::inputs(self))
			.field("outputs", &crate::dnn::LayerTraitConst::outputs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

