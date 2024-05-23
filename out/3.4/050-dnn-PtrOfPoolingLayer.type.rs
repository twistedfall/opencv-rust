ptr_extern! { crate::dnn::PoolingLayer,
	cv_PtrLcv_dnn_PoolingLayerG_new_null_const, cv_PtrLcv_dnn_PoolingLayerG_delete, cv_PtrLcv_dnn_PoolingLayerG_getInnerPtr_const, cv_PtrLcv_dnn_PoolingLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::PoolingLayer, cv_PtrLcv_dnn_PoolingLayerG_new_const_PoolingLayer }
impl core::Ptr<crate::dnn::PoolingLayer> {
	#[inline] pub fn as_raw_PtrOfPoolingLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPoolingLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::PoolingLayerTraitConst for core::Ptr<crate::dnn::PoolingLayer> {
	#[inline] fn as_raw_PoolingLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::PoolingLayerTrait for core::Ptr<crate::dnn::PoolingLayer> {
	#[inline] fn as_raw_mut_PoolingLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PoolingLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::PoolingLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::PoolingLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_PoolingLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PoolingLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PoolingLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::PoolingLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_PoolingLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::PoolingLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfPoolingLayer")
			.field("typ", &crate::dnn::PoolingLayerTraitConst::typ(self))
			.field("kernel_size", &crate::dnn::PoolingLayerTraitConst::kernel_size(self))
			.field("strides", &crate::dnn::PoolingLayerTraitConst::strides(self))
			.field("pads_begin", &crate::dnn::PoolingLayerTraitConst::pads_begin(self))
			.field("pads_end", &crate::dnn::PoolingLayerTraitConst::pads_end(self))
			.field("global_pooling", &crate::dnn::PoolingLayerTraitConst::global_pooling(self))
			.field("is_global_pooling", &crate::dnn::PoolingLayerTraitConst::is_global_pooling(self))
			.field("compute_max_idx", &crate::dnn::PoolingLayerTraitConst::compute_max_idx(self))
			.field("pad_mode", &crate::dnn::PoolingLayerTraitConst::pad_mode(self))
			.field("ceil_mode", &crate::dnn::PoolingLayerTraitConst::ceil_mode(self))
			.field("ave_pool_padded_area", &crate::dnn::PoolingLayerTraitConst::ave_pool_padded_area(self))
			.field("pooled_size", &crate::dnn::PoolingLayerTraitConst::pooled_size(self))
			.field("spatial_scale", &crate::dnn::PoolingLayerTraitConst::spatial_scale(self))
			.field("ps_roi_out_channels", &crate::dnn::PoolingLayerTraitConst::ps_roi_out_channels(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

