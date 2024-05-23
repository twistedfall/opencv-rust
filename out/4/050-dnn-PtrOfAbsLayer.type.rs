ptr_extern! { crate::dnn::AbsLayer,
	cv_PtrLcv_dnn_AbsLayerG_new_null_const, cv_PtrLcv_dnn_AbsLayerG_delete, cv_PtrLcv_dnn_AbsLayerG_getInnerPtr_const, cv_PtrLcv_dnn_AbsLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::AbsLayer, cv_PtrLcv_dnn_AbsLayerG_new_const_AbsLayer }
impl core::Ptr<crate::dnn::AbsLayer> {
	#[inline] pub fn as_raw_PtrOfAbsLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAbsLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::AbsLayerTraitConst for core::Ptr<crate::dnn::AbsLayer> {
	#[inline] fn as_raw_AbsLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::AbsLayerTrait for core::Ptr<crate::dnn::AbsLayer> {
	#[inline] fn as_raw_mut_AbsLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AbsLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::AbsLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::AbsLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_AbsLayerG_to_PtrOfAlgorithm }

impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::AbsLayer> {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::AbsLayer> {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::AbsLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_AbsLayerG_to_PtrOfActivationLayer }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AbsLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AbsLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::AbsLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_AbsLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::AbsLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfAbsLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

