ptr_extern! { crate::dnn::Concat2Layer,
	cv_PtrLcv_dnn_Concat2LayerG_new_null_const, cv_PtrLcv_dnn_Concat2LayerG_delete, cv_PtrLcv_dnn_Concat2LayerG_getInnerPtr_const, cv_PtrLcv_dnn_Concat2LayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::Concat2Layer, cv_PtrLcv_dnn_Concat2LayerG_new_const_Concat2Layer }
impl core::Ptr<crate::dnn::Concat2Layer> {
	#[inline] pub fn as_raw_PtrOfConcat2Layer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfConcat2Layer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::Concat2LayerTraitConst for core::Ptr<crate::dnn::Concat2Layer> {
	#[inline] fn as_raw_Concat2Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::Concat2LayerTrait for core::Ptr<crate::dnn::Concat2Layer> {
	#[inline] fn as_raw_mut_Concat2Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::Concat2Layer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::Concat2Layer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::Concat2Layer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_Concat2LayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::Concat2Layer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::Concat2Layer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::Concat2Layer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_Concat2LayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::Concat2Layer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfConcat2Layer")
			.field("axis", &crate::dnn::Concat2LayerTraitConst::axis(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("inputs", &crate::dnn::LayerTraitConst::inputs(self))
			.field("outputs", &crate::dnn::LayerTraitConst::outputs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

