ptr_extern! { crate::dnn::ScaleLayerInt8,
	cv_PtrLcv_dnn_ScaleLayerInt8G_new_null_const, cv_PtrLcv_dnn_ScaleLayerInt8G_delete, cv_PtrLcv_dnn_ScaleLayerInt8G_getInnerPtr_const, cv_PtrLcv_dnn_ScaleLayerInt8G_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::ScaleLayerInt8, cv_PtrLcv_dnn_ScaleLayerInt8G_new_const_ScaleLayerInt8 }
impl core::Ptr<crate::dnn::ScaleLayerInt8> {
	#[inline] pub fn as_raw_PtrOfScaleLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfScaleLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::ScaleLayerInt8TraitConst for core::Ptr<crate::dnn::ScaleLayerInt8> {
	#[inline] fn as_raw_ScaleLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ScaleLayerInt8Trait for core::Ptr<crate::dnn::ScaleLayerInt8> {
	#[inline] fn as_raw_mut_ScaleLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ScaleLayerInt8> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::ScaleLayerInt8> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ScaleLayerInt8>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ScaleLayerInt8G_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ScaleLayerInt8> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ScaleLayerInt8> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ScaleLayerInt8>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ScaleLayerInt8G_to_PtrOfLayer }

impl crate::dnn::ScaleLayerTraitConst for core::Ptr<crate::dnn::ScaleLayerInt8> {
	#[inline] fn as_raw_ScaleLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ScaleLayerTrait for core::Ptr<crate::dnn::ScaleLayerInt8> {
	#[inline] fn as_raw_mut_ScaleLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ScaleLayerInt8>, core::Ptr<crate::dnn::ScaleLayer>, cv_PtrLcv_dnn_ScaleLayerInt8G_to_PtrOfScaleLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::ScaleLayerInt8> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfScaleLayerInt8")
			.field("output_sc", &crate::dnn::ScaleLayerInt8TraitConst::output_sc(self))
			.field("output_zp", &crate::dnn::ScaleLayerInt8TraitConst::output_zp(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.field("has_bias", &crate::dnn::ScaleLayerTraitConst::has_bias(self))
			.field("axis", &crate::dnn::ScaleLayerTraitConst::axis(self))
			.field("mode", &crate::dnn::ScaleLayerTraitConst::mode(self))
			.finish()
	}
}

