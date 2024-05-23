ptr_extern! { crate::dnn::ShiftLayerInt8,
	cv_PtrLcv_dnn_ShiftLayerInt8G_new_null_const, cv_PtrLcv_dnn_ShiftLayerInt8G_delete, cv_PtrLcv_dnn_ShiftLayerInt8G_getInnerPtr_const, cv_PtrLcv_dnn_ShiftLayerInt8G_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::ShiftLayerInt8, cv_PtrLcv_dnn_ShiftLayerInt8G_new_const_ShiftLayerInt8 }
impl core::Ptr<crate::dnn::ShiftLayerInt8> {
	#[inline] pub fn as_raw_PtrOfShiftLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfShiftLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::ShiftLayerInt8TraitConst for core::Ptr<crate::dnn::ShiftLayerInt8> {
	#[inline] fn as_raw_ShiftLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ShiftLayerInt8Trait for core::Ptr<crate::dnn::ShiftLayerInt8> {
	#[inline] fn as_raw_mut_ShiftLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ShiftLayerInt8> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::ShiftLayerInt8> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ShiftLayerInt8>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ShiftLayerInt8G_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ShiftLayerInt8> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ShiftLayerInt8> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ShiftLayerInt8>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ShiftLayerInt8G_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::ShiftLayerInt8> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfShiftLayerInt8")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

