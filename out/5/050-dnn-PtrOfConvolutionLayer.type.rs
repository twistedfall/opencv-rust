ptr_extern! { crate::dnn::ConvolutionLayer,
	cv_PtrLcv_dnn_ConvolutionLayerG_new_null_const, cv_PtrLcv_dnn_ConvolutionLayerG_delete, cv_PtrLcv_dnn_ConvolutionLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ConvolutionLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::ConvolutionLayer, cv_PtrLcv_dnn_ConvolutionLayerG_new_const_ConvolutionLayer }
impl core::Ptr<crate::dnn::ConvolutionLayer> {
	#[inline] pub fn as_raw_PtrOfConvolutionLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfConvolutionLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::ConvolutionLayerTraitConst for core::Ptr<crate::dnn::ConvolutionLayer> {
	#[inline] fn as_raw_ConvolutionLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ConvolutionLayerTrait for core::Ptr<crate::dnn::ConvolutionLayer> {
	#[inline] fn as_raw_mut_ConvolutionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ConvolutionLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::ConvolutionLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ConvolutionLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ConvolutionLayerG_to_PtrOfAlgorithm }

impl crate::dnn::BaseConvolutionLayerTraitConst for core::Ptr<crate::dnn::ConvolutionLayer> {
	#[inline] fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::BaseConvolutionLayerTrait for core::Ptr<crate::dnn::ConvolutionLayer> {
	#[inline] fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ConvolutionLayer>, core::Ptr<crate::dnn::BaseConvolutionLayer>, cv_PtrLcv_dnn_ConvolutionLayerG_to_PtrOfBaseConvolutionLayer }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ConvolutionLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ConvolutionLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ConvolutionLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ConvolutionLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::ConvolutionLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfConvolutionLayer")
			.field("fused_activation", &crate::dnn::ConvolutionLayerTraitConst::fused_activation(self))
			.field("fused_add", &crate::dnn::ConvolutionLayerTraitConst::fused_add(self))
			.field("use_winograd", &crate::dnn::ConvolutionLayerTraitConst::use_winograd(self))
			.field("kernel", &crate::dnn::BaseConvolutionLayerTraitConst::kernel(self))
			.field("stride", &crate::dnn::BaseConvolutionLayerTraitConst::stride(self))
			.field("pad", &crate::dnn::BaseConvolutionLayerTraitConst::pad(self))
			.field("dilation", &crate::dnn::BaseConvolutionLayerTraitConst::dilation(self))
			.field("adjust_pad", &crate::dnn::BaseConvolutionLayerTraitConst::adjust_pad(self))
			.field("adjust_pads", &crate::dnn::BaseConvolutionLayerTraitConst::adjust_pads(self))
			.field("kernel_size", &crate::dnn::BaseConvolutionLayerTraitConst::kernel_size(self))
			.field("strides", &crate::dnn::BaseConvolutionLayerTraitConst::strides(self))
			.field("dilations", &crate::dnn::BaseConvolutionLayerTraitConst::dilations(self))
			.field("pads_begin", &crate::dnn::BaseConvolutionLayerTraitConst::pads_begin(self))
			.field("pads_end", &crate::dnn::BaseConvolutionLayerTraitConst::pads_end(self))
			.field("pad_mode", &crate::dnn::BaseConvolutionLayerTraitConst::pad_mode(self))
			.field("num_output", &crate::dnn::BaseConvolutionLayerTraitConst::num_output(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("inputs", &crate::dnn::LayerTraitConst::inputs(self))
			.field("outputs", &crate::dnn::LayerTraitConst::outputs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

