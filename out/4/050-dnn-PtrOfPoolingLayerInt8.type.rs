ptr_extern! { crate::dnn::PoolingLayerInt8,
	cv_PtrLcv_dnn_PoolingLayerInt8G_new_null_const, cv_PtrLcv_dnn_PoolingLayerInt8G_delete, cv_PtrLcv_dnn_PoolingLayerInt8G_getInnerPtr_const, cv_PtrLcv_dnn_PoolingLayerInt8G_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::PoolingLayerInt8, cv_PtrLcv_dnn_PoolingLayerInt8G_new_const_PoolingLayerInt8 }
impl core::Ptr<crate::dnn::PoolingLayerInt8> {
	#[inline] pub fn as_raw_PtrOfPoolingLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPoolingLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::PoolingLayerInt8TraitConst for core::Ptr<crate::dnn::PoolingLayerInt8> {
	#[inline] fn as_raw_PoolingLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::PoolingLayerInt8Trait for core::Ptr<crate::dnn::PoolingLayerInt8> {
	#[inline] fn as_raw_mut_PoolingLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PoolingLayerInt8> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::PoolingLayerInt8> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::PoolingLayerInt8>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_PoolingLayerInt8G_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PoolingLayerInt8> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PoolingLayerInt8> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::PoolingLayerInt8>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_PoolingLayerInt8G_to_PtrOfLayer }

impl crate::dnn::PoolingLayerTraitConst for core::Ptr<crate::dnn::PoolingLayerInt8> {
	#[inline] fn as_raw_PoolingLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::PoolingLayerTrait for core::Ptr<crate::dnn::PoolingLayerInt8> {
	#[inline] fn as_raw_mut_PoolingLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::PoolingLayerInt8>, core::Ptr<crate::dnn::PoolingLayer>, cv_PtrLcv_dnn_PoolingLayerInt8G_to_PtrOfPoolingLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::PoolingLayerInt8> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfPoolingLayerInt8")
			.field("input_zp", &crate::dnn::PoolingLayerInt8TraitConst::input_zp(self))
			.field("output_zp", &crate::dnn::PoolingLayerInt8TraitConst::output_zp(self))
			.field("input_sc", &crate::dnn::PoolingLayerInt8TraitConst::input_sc(self))
			.field("output_sc", &crate::dnn::PoolingLayerInt8TraitConst::output_sc(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
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
			.finish()
	}
}

