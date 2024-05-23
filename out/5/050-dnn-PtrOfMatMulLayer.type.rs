ptr_extern! { crate::dnn::MatMulLayer,
	cv_PtrLcv_dnn_MatMulLayerG_new_null_const, cv_PtrLcv_dnn_MatMulLayerG_delete, cv_PtrLcv_dnn_MatMulLayerG_getInnerPtr_const, cv_PtrLcv_dnn_MatMulLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::MatMulLayer, cv_PtrLcv_dnn_MatMulLayerG_new_const_MatMulLayer }
impl core::Ptr<crate::dnn::MatMulLayer> {
	#[inline] pub fn as_raw_PtrOfMatMulLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMatMulLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::MatMulLayerTraitConst for core::Ptr<crate::dnn::MatMulLayer> {
	#[inline] fn as_raw_MatMulLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::MatMulLayerTrait for core::Ptr<crate::dnn::MatMulLayer> {
	#[inline] fn as_raw_mut_MatMulLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::MatMulLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::MatMulLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::MatMulLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_MatMulLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::MatMulLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::MatMulLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::MatMulLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_MatMulLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::MatMulLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMatMulLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("inputs", &crate::dnn::LayerTraitConst::inputs(self))
			.field("outputs", &crate::dnn::LayerTraitConst::outputs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

