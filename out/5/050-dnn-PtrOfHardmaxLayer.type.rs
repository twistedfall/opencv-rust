ptr_extern! { crate::dnn::HardmaxLayer,
	cv_PtrLcv_dnn_HardmaxLayerG_new_null_const, cv_PtrLcv_dnn_HardmaxLayerG_delete, cv_PtrLcv_dnn_HardmaxLayerG_getInnerPtr_const, cv_PtrLcv_dnn_HardmaxLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::HardmaxLayer, cv_PtrLcv_dnn_HardmaxLayerG_new_const_HardmaxLayer }
impl core::Ptr<crate::dnn::HardmaxLayer> {
	#[inline] pub fn as_raw_PtrOfHardmaxLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfHardmaxLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::HardmaxLayerTraitConst for core::Ptr<crate::dnn::HardmaxLayer> {
	#[inline] fn as_raw_HardmaxLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::HardmaxLayerTrait for core::Ptr<crate::dnn::HardmaxLayer> {
	#[inline] fn as_raw_mut_HardmaxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::HardmaxLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::HardmaxLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::HardmaxLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_HardmaxLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::HardmaxLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::HardmaxLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::HardmaxLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_HardmaxLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::HardmaxLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfHardmaxLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("inputs", &crate::dnn::LayerTraitConst::inputs(self))
			.field("outputs", &crate::dnn::LayerTraitConst::outputs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

