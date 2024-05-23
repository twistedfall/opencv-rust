ptr_extern! { crate::dnn::CropAndResizeLayer,
	cv_PtrLcv_dnn_CropAndResizeLayerG_new_null_const, cv_PtrLcv_dnn_CropAndResizeLayerG_delete, cv_PtrLcv_dnn_CropAndResizeLayerG_getInnerPtr_const, cv_PtrLcv_dnn_CropAndResizeLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::CropAndResizeLayer, cv_PtrLcv_dnn_CropAndResizeLayerG_new_const_CropAndResizeLayer }
impl core::Ptr<crate::dnn::CropAndResizeLayer> {
	#[inline] pub fn as_raw_PtrOfCropAndResizeLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCropAndResizeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::CropAndResizeLayerTraitConst for core::Ptr<crate::dnn::CropAndResizeLayer> {
	#[inline] fn as_raw_CropAndResizeLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::CropAndResizeLayerTrait for core::Ptr<crate::dnn::CropAndResizeLayer> {
	#[inline] fn as_raw_mut_CropAndResizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CropAndResizeLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::CropAndResizeLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::CropAndResizeLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_CropAndResizeLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CropAndResizeLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CropAndResizeLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::CropAndResizeLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_CropAndResizeLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::CropAndResizeLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCropAndResizeLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

