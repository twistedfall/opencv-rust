ptr_extern! { crate::dnn::BlankLayer,
	cv_PtrLcv_dnn_BlankLayerG_new_null_const, cv_PtrLcv_dnn_BlankLayerG_delete, cv_PtrLcv_dnn_BlankLayerG_getInnerPtr_const, cv_PtrLcv_dnn_BlankLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::BlankLayer, cv_PtrLcv_dnn_BlankLayerG_new_const_BlankLayer }
impl core::Ptr<crate::dnn::BlankLayer> {
	#[inline] pub fn as_raw_PtrOfBlankLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBlankLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::BlankLayerTraitConst for core::Ptr<crate::dnn::BlankLayer> {
	#[inline] fn as_raw_BlankLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::BlankLayerTrait for core::Ptr<crate::dnn::BlankLayer> {
	#[inline] fn as_raw_mut_BlankLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::BlankLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::BlankLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::BlankLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_BlankLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::BlankLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::BlankLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::BlankLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_BlankLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::BlankLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfBlankLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

