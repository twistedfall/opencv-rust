ptr_extern! { crate::dnn::Reshape2Layer,
	cv_PtrLcv_dnn_Reshape2LayerG_new_null_const, cv_PtrLcv_dnn_Reshape2LayerG_delete, cv_PtrLcv_dnn_Reshape2LayerG_getInnerPtr_const, cv_PtrLcv_dnn_Reshape2LayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::Reshape2Layer, cv_PtrLcv_dnn_Reshape2LayerG_new_const_Reshape2Layer }
impl core::Ptr<crate::dnn::Reshape2Layer> {
	#[inline] pub fn as_raw_PtrOfReshape2Layer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfReshape2Layer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::Reshape2LayerTraitConst for core::Ptr<crate::dnn::Reshape2Layer> {
	#[inline] fn as_raw_Reshape2Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::Reshape2LayerTrait for core::Ptr<crate::dnn::Reshape2Layer> {
	#[inline] fn as_raw_mut_Reshape2Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::Reshape2Layer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::Reshape2Layer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::Reshape2Layer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_Reshape2LayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::Reshape2Layer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::Reshape2Layer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::Reshape2Layer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_Reshape2LayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::Reshape2Layer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfReshape2Layer")
			.field("new_shape_desc", &crate::dnn::Reshape2LayerTraitConst::new_shape_desc(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("inputs", &crate::dnn::LayerTraitConst::inputs(self))
			.field("outputs", &crate::dnn::LayerTraitConst::outputs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

