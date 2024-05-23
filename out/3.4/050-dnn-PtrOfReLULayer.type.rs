ptr_extern! { crate::dnn::ReLULayer,
	cv_PtrLcv_dnn_ReLULayerG_new_null_const, cv_PtrLcv_dnn_ReLULayerG_delete, cv_PtrLcv_dnn_ReLULayerG_getInnerPtr_const, cv_PtrLcv_dnn_ReLULayerG_getInnerPtrMut
}

impl core::Ptr<crate::dnn::ReLULayer> {
	#[inline] pub fn as_raw_PtrOfReLULayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfReLULayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::ReLULayerTraitConst for core::Ptr<crate::dnn::ReLULayer> {
	#[inline] fn as_raw_ReLULayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ReLULayerTrait for core::Ptr<crate::dnn::ReLULayer> {
	#[inline] fn as_raw_mut_ReLULayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReLULayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReLULayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ReLULayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ReLULayerG_to_PtrOfAlgorithm }

impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ReLULayer> {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ReLULayer> {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ReLULayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_ReLULayerG_to_PtrOfActivationLayer }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReLULayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReLULayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ReLULayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ReLULayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::ReLULayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfReLULayer")
			.field("negative_slope", &crate::dnn::ReLULayerTraitConst::negative_slope(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

