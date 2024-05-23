ptr_extern! { crate::dnn::CropLayer,
	cv_PtrLcv_dnn_CropLayerG_new_null_const, cv_PtrLcv_dnn_CropLayerG_delete, cv_PtrLcv_dnn_CropLayerG_getInnerPtr_const, cv_PtrLcv_dnn_CropLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::CropLayer, cv_PtrLcv_dnn_CropLayerG_new_const_CropLayer }
impl core::Ptr<crate::dnn::CropLayer> {
	#[inline] pub fn as_raw_PtrOfCropLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCropLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::CropLayerTraitConst for core::Ptr<crate::dnn::CropLayer> {
	#[inline] fn as_raw_CropLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::CropLayerTrait for core::Ptr<crate::dnn::CropLayer> {
	#[inline] fn as_raw_mut_CropLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CropLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::CropLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::CropLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_CropLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CropLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CropLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::CropLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_CropLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::CropLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCropLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

