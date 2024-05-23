ptr_extern! { crate::dnn::ShiftLayer,
	cv_PtrLcv_dnn_ShiftLayerG_new_null_const, cv_PtrLcv_dnn_ShiftLayerG_delete, cv_PtrLcv_dnn_ShiftLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ShiftLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::ShiftLayer, cv_PtrLcv_dnn_ShiftLayerG_new_const_ShiftLayer }
impl core::Ptr<crate::dnn::ShiftLayer> {
	#[inline] pub fn as_raw_PtrOfShiftLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfShiftLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::ShiftLayerTraitConst for core::Ptr<crate::dnn::ShiftLayer> {
	#[inline] fn as_raw_ShiftLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ShiftLayerTrait for core::Ptr<crate::dnn::ShiftLayer> {
	#[inline] fn as_raw_mut_ShiftLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ShiftLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::ShiftLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ShiftLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ShiftLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ShiftLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ShiftLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ShiftLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ShiftLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::ShiftLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfShiftLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

