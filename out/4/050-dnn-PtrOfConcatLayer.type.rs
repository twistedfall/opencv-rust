ptr_extern! { crate::dnn::ConcatLayer,
	cv_PtrLcv_dnn_ConcatLayerG_new_null_const, cv_PtrLcv_dnn_ConcatLayerG_delete, cv_PtrLcv_dnn_ConcatLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ConcatLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::ConcatLayer, cv_PtrLcv_dnn_ConcatLayerG_new_const_ConcatLayer }
impl core::Ptr<crate::dnn::ConcatLayer> {
	#[inline] pub fn as_raw_PtrOfConcatLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfConcatLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::ConcatLayerTraitConst for core::Ptr<crate::dnn::ConcatLayer> {
	#[inline] fn as_raw_ConcatLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ConcatLayerTrait for core::Ptr<crate::dnn::ConcatLayer> {
	#[inline] fn as_raw_mut_ConcatLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ConcatLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::ConcatLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ConcatLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ConcatLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ConcatLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ConcatLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ConcatLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ConcatLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::ConcatLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfConcatLayer")
			.field("axis", &crate::dnn::ConcatLayerTraitConst::axis(self))
			.field("padding", &crate::dnn::ConcatLayerTraitConst::padding(self))
			.field("padding_value", &crate::dnn::ConcatLayerTraitConst::padding_value(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

