ptr_extern! { crate::dnn::SoftmaxLayerInt8,
	cv_PtrLcv_dnn_SoftmaxLayerInt8G_new_null_const, cv_PtrLcv_dnn_SoftmaxLayerInt8G_delete, cv_PtrLcv_dnn_SoftmaxLayerInt8G_getInnerPtr_const, cv_PtrLcv_dnn_SoftmaxLayerInt8G_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::SoftmaxLayerInt8, cv_PtrLcv_dnn_SoftmaxLayerInt8G_new_const_SoftmaxLayerInt8 }
impl core::Ptr<crate::dnn::SoftmaxLayerInt8> {
	#[inline] pub fn as_raw_PtrOfSoftmaxLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSoftmaxLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::SoftmaxLayerInt8TraitConst for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
	#[inline] fn as_raw_SoftmaxLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::SoftmaxLayerInt8Trait for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
	#[inline] fn as_raw_mut_SoftmaxLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::SoftmaxLayerInt8>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_SoftmaxLayerInt8G_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::SoftmaxLayerInt8>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_SoftmaxLayerInt8G_to_PtrOfLayer }

impl crate::dnn::SoftmaxLayerTraitConst for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
	#[inline] fn as_raw_SoftmaxLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::SoftmaxLayerTrait for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
	#[inline] fn as_raw_mut_SoftmaxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::SoftmaxLayerInt8>, core::Ptr<crate::dnn::SoftmaxLayer>, cv_PtrLcv_dnn_SoftmaxLayerInt8G_to_PtrOfSoftmaxLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSoftmaxLayerInt8")
			.field("output_sc", &crate::dnn::SoftmaxLayerInt8TraitConst::output_sc(self))
			.field("output_zp", &crate::dnn::SoftmaxLayerInt8TraitConst::output_zp(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.field("log_soft_max", &crate::dnn::SoftmaxLayerTraitConst::log_soft_max(self))
			.finish()
	}
}

