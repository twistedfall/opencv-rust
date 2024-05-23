ptr_extern! { crate::dnn::GatherLayer,
	cv_PtrLcv_dnn_GatherLayerG_new_null_const, cv_PtrLcv_dnn_GatherLayerG_delete, cv_PtrLcv_dnn_GatherLayerG_getInnerPtr_const, cv_PtrLcv_dnn_GatherLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::GatherLayer, cv_PtrLcv_dnn_GatherLayerG_new_const_GatherLayer }
impl core::Ptr<crate::dnn::GatherLayer> {
	#[inline] pub fn as_raw_PtrOfGatherLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGatherLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::GatherLayerTraitConst for core::Ptr<crate::dnn::GatherLayer> {
	#[inline] fn as_raw_GatherLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::GatherLayerTrait for core::Ptr<crate::dnn::GatherLayer> {
	#[inline] fn as_raw_mut_GatherLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::GatherLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::GatherLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::GatherLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_GatherLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::GatherLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::GatherLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::GatherLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_GatherLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::GatherLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfGatherLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

