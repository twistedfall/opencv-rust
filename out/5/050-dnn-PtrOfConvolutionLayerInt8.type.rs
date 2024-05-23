ptr_extern! { crate::dnn::ConvolutionLayerInt8,
	cv_PtrLcv_dnn_ConvolutionLayerInt8G_new_null_const, cv_PtrLcv_dnn_ConvolutionLayerInt8G_delete, cv_PtrLcv_dnn_ConvolutionLayerInt8G_getInnerPtr_const, cv_PtrLcv_dnn_ConvolutionLayerInt8G_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::ConvolutionLayerInt8, cv_PtrLcv_dnn_ConvolutionLayerInt8G_new_const_ConvolutionLayerInt8 }
impl core::Ptr<crate::dnn::ConvolutionLayerInt8> {
	#[inline] pub fn as_raw_PtrOfConvolutionLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfConvolutionLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::ConvolutionLayerInt8TraitConst for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
	#[inline] fn as_raw_ConvolutionLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ConvolutionLayerInt8Trait for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
	#[inline] fn as_raw_mut_ConvolutionLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ConvolutionLayerInt8>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ConvolutionLayerInt8G_to_PtrOfAlgorithm }

impl crate::dnn::BaseConvolutionLayerTraitConst for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
	#[inline] fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::BaseConvolutionLayerTrait for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
	#[inline] fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ConvolutionLayerInt8>, core::Ptr<crate::dnn::BaseConvolutionLayer>, cv_PtrLcv_dnn_ConvolutionLayerInt8G_to_PtrOfBaseConvolutionLayer }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ConvolutionLayerInt8>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ConvolutionLayerInt8G_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfConvolutionLayerInt8")
			.field("input_zp", &crate::dnn::ConvolutionLayerInt8TraitConst::input_zp(self))
			.field("output_zp", &crate::dnn::ConvolutionLayerInt8TraitConst::output_zp(self))
			.field("input_sc", &crate::dnn::ConvolutionLayerInt8TraitConst::input_sc(self))
			.field("output_sc", &crate::dnn::ConvolutionLayerInt8TraitConst::output_sc(self))
			.field("per_channel", &crate::dnn::ConvolutionLayerInt8TraitConst::per_channel(self))
			.field("use_winograd", &crate::dnn::ConvolutionLayerInt8TraitConst::use_winograd(self))
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

