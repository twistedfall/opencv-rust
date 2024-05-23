ptr_extern! { crate::dnn::Expand2Layer,
	cv_PtrLcv_dnn_Expand2LayerG_new_null_const, cv_PtrLcv_dnn_Expand2LayerG_delete, cv_PtrLcv_dnn_Expand2LayerG_getInnerPtr_const, cv_PtrLcv_dnn_Expand2LayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::Expand2Layer, cv_PtrLcv_dnn_Expand2LayerG_new_const_Expand2Layer }
impl core::Ptr<crate::dnn::Expand2Layer> {
	#[inline] pub fn as_raw_PtrOfExpand2Layer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfExpand2Layer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::Expand2LayerTraitConst for core::Ptr<crate::dnn::Expand2Layer> {
	#[inline] fn as_raw_Expand2Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::Expand2LayerTrait for core::Ptr<crate::dnn::Expand2Layer> {
	#[inline] fn as_raw_mut_Expand2Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::Expand2Layer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::Expand2Layer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::Expand2Layer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_Expand2LayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::Expand2Layer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::Expand2Layer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::Expand2Layer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_Expand2LayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::Expand2Layer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfExpand2Layer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("inputs", &crate::dnn::LayerTraitConst::inputs(self))
			.field("outputs", &crate::dnn::LayerTraitConst::outputs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

