ptr_extern! { crate::dnn::InnerProductLayerInt8,
	cv_PtrLcv_dnn_InnerProductLayerInt8G_new_null_const, cv_PtrLcv_dnn_InnerProductLayerInt8G_delete, cv_PtrLcv_dnn_InnerProductLayerInt8G_getInnerPtr_const, cv_PtrLcv_dnn_InnerProductLayerInt8G_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::InnerProductLayerInt8, cv_PtrLcv_dnn_InnerProductLayerInt8G_new_const_InnerProductLayerInt8 }
impl core::Ptr<crate::dnn::InnerProductLayerInt8> {
	#[inline] pub fn as_raw_PtrOfInnerProductLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfInnerProductLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::InnerProductLayerInt8TraitConst for core::Ptr<crate::dnn::InnerProductLayerInt8> {
	#[inline] fn as_raw_InnerProductLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::InnerProductLayerInt8Trait for core::Ptr<crate::dnn::InnerProductLayerInt8> {
	#[inline] fn as_raw_mut_InnerProductLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::InnerProductLayerInt8> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::InnerProductLayerInt8> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::InnerProductLayerInt8>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_InnerProductLayerInt8G_to_PtrOfAlgorithm }

impl crate::dnn::InnerProductLayerTraitConst for core::Ptr<crate::dnn::InnerProductLayerInt8> {
	#[inline] fn as_raw_InnerProductLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::InnerProductLayerTrait for core::Ptr<crate::dnn::InnerProductLayerInt8> {
	#[inline] fn as_raw_mut_InnerProductLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::InnerProductLayerInt8>, core::Ptr<crate::dnn::InnerProductLayer>, cv_PtrLcv_dnn_InnerProductLayerInt8G_to_PtrOfInnerProductLayer }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::InnerProductLayerInt8> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::InnerProductLayerInt8> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::InnerProductLayerInt8>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_InnerProductLayerInt8G_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::InnerProductLayerInt8> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfInnerProductLayerInt8")
			.field("input_zp", &crate::dnn::InnerProductLayerInt8TraitConst::input_zp(self))
			.field("output_zp", &crate::dnn::InnerProductLayerInt8TraitConst::output_zp(self))
			.field("input_sc", &crate::dnn::InnerProductLayerInt8TraitConst::input_sc(self))
			.field("output_sc", &crate::dnn::InnerProductLayerInt8TraitConst::output_sc(self))
			.field("per_channel", &crate::dnn::InnerProductLayerInt8TraitConst::per_channel(self))
			.field("axis", &crate::dnn::InnerProductLayerTraitConst::axis(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("inputs", &crate::dnn::LayerTraitConst::inputs(self))
			.field("outputs", &crate::dnn::LayerTraitConst::outputs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

