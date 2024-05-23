ptr_extern! { crate::dnn::AcoshLayer,
	cv_PtrLcv_dnn_AcoshLayerG_new_null_const, cv_PtrLcv_dnn_AcoshLayerG_delete, cv_PtrLcv_dnn_AcoshLayerG_getInnerPtr_const, cv_PtrLcv_dnn_AcoshLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::AcoshLayer, cv_PtrLcv_dnn_AcoshLayerG_new_const_AcoshLayer }
impl core::Ptr<crate::dnn::AcoshLayer> {
	#[inline] pub fn as_raw_PtrOfAcoshLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAcoshLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::AcoshLayerTraitConst for core::Ptr<crate::dnn::AcoshLayer> {
	#[inline] fn as_raw_AcoshLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::AcoshLayerTrait for core::Ptr<crate::dnn::AcoshLayer> {
	#[inline] fn as_raw_mut_AcoshLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AcoshLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::AcoshLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::AcoshLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_AcoshLayerG_to_PtrOfAlgorithm }

impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::AcoshLayer> {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::AcoshLayer> {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::AcoshLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_AcoshLayerG_to_PtrOfActivationLayer }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AcoshLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AcoshLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::AcoshLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_AcoshLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::AcoshLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfAcoshLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

