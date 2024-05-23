ptr_extern! { crate::dnn::ArgLayer,
	cv_PtrLcv_dnn_ArgLayerG_new_null_const, cv_PtrLcv_dnn_ArgLayerG_delete, cv_PtrLcv_dnn_ArgLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ArgLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::ArgLayer, cv_PtrLcv_dnn_ArgLayerG_new_const_ArgLayer }
impl core::Ptr<crate::dnn::ArgLayer> {
	#[inline] pub fn as_raw_PtrOfArgLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfArgLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::ArgLayerTraitConst for core::Ptr<crate::dnn::ArgLayer> {
	#[inline] fn as_raw_ArgLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ArgLayerTrait for core::Ptr<crate::dnn::ArgLayer> {
	#[inline] fn as_raw_mut_ArgLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ArgLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::ArgLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ArgLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ArgLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ArgLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ArgLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ArgLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ArgLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::ArgLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfArgLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

