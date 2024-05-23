ptr_extern! { crate::dnn::MVNLayer,
	cv_PtrLcv_dnn_MVNLayerG_new_null_const, cv_PtrLcv_dnn_MVNLayerG_delete, cv_PtrLcv_dnn_MVNLayerG_getInnerPtr_const, cv_PtrLcv_dnn_MVNLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::MVNLayer, cv_PtrLcv_dnn_MVNLayerG_new_const_MVNLayer }
impl core::Ptr<crate::dnn::MVNLayer> {
	#[inline] pub fn as_raw_PtrOfMVNLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMVNLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::MVNLayerTraitConst for core::Ptr<crate::dnn::MVNLayer> {
	#[inline] fn as_raw_MVNLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::MVNLayerTrait for core::Ptr<crate::dnn::MVNLayer> {
	#[inline] fn as_raw_mut_MVNLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::MVNLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::MVNLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::MVNLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_MVNLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::MVNLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::MVNLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::MVNLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_MVNLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::MVNLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMVNLayer")
			.field("eps", &crate::dnn::MVNLayerTraitConst::eps(self))
			.field("norm_variance", &crate::dnn::MVNLayerTraitConst::norm_variance(self))
			.field("across_channels", &crate::dnn::MVNLayerTraitConst::across_channels(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("inputs", &crate::dnn::LayerTraitConst::inputs(self))
			.field("outputs", &crate::dnn::LayerTraitConst::outputs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

