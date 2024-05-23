ptr_extern! { crate::dnn::ReLU6Layer,
	cv_PtrLcv_dnn_ReLU6LayerG_new_null_const, cv_PtrLcv_dnn_ReLU6LayerG_delete, cv_PtrLcv_dnn_ReLU6LayerG_getInnerPtr_const, cv_PtrLcv_dnn_ReLU6LayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::ReLU6Layer, cv_PtrLcv_dnn_ReLU6LayerG_new_const_ReLU6Layer }
impl core::Ptr<crate::dnn::ReLU6Layer> {
	#[inline] pub fn as_raw_PtrOfReLU6Layer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfReLU6Layer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::ReLU6LayerTraitConst for core::Ptr<crate::dnn::ReLU6Layer> {
	#[inline] fn as_raw_ReLU6Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ReLU6LayerTrait for core::Ptr<crate::dnn::ReLU6Layer> {
	#[inline] fn as_raw_mut_ReLU6Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReLU6Layer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReLU6Layer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ReLU6Layer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ReLU6LayerG_to_PtrOfAlgorithm }

impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ReLU6Layer> {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ReLU6Layer> {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ReLU6Layer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_ReLU6LayerG_to_PtrOfActivationLayer }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReLU6Layer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReLU6Layer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ReLU6Layer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ReLU6LayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::ReLU6Layer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfReLU6Layer")
			.field("min_value", &crate::dnn::ReLU6LayerTraitConst::min_value(self))
			.field("max_value", &crate::dnn::ReLU6LayerTraitConst::max_value(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

