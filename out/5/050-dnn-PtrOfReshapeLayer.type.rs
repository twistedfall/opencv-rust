ptr_extern! { crate::dnn::ReshapeLayer,
	cv_PtrLcv_dnn_ReshapeLayerG_new_null_const, cv_PtrLcv_dnn_ReshapeLayerG_delete, cv_PtrLcv_dnn_ReshapeLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ReshapeLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::ReshapeLayer, cv_PtrLcv_dnn_ReshapeLayerG_new_const_ReshapeLayer }
impl core::Ptr<crate::dnn::ReshapeLayer> {
	#[inline] pub fn as_raw_PtrOfReshapeLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfReshapeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::ReshapeLayerTraitConst for core::Ptr<crate::dnn::ReshapeLayer> {
	#[inline] fn as_raw_ReshapeLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ReshapeLayerTrait for core::Ptr<crate::dnn::ReshapeLayer> {
	#[inline] fn as_raw_mut_ReshapeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReshapeLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReshapeLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ReshapeLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ReshapeLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReshapeLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReshapeLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ReshapeLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ReshapeLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::ReshapeLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfReshapeLayer")
			.field("new_shape_desc", &crate::dnn::ReshapeLayerTraitConst::new_shape_desc(self))
			.field("new_shape_range", &crate::dnn::ReshapeLayerTraitConst::new_shape_range(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("inputs", &crate::dnn::LayerTraitConst::inputs(self))
			.field("outputs", &crate::dnn::LayerTraitConst::outputs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

