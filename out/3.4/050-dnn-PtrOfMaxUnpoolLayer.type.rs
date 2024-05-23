ptr_extern! { crate::dnn::MaxUnpoolLayer,
	cv_PtrLcv_dnn_MaxUnpoolLayerG_new_null_const, cv_PtrLcv_dnn_MaxUnpoolLayerG_delete, cv_PtrLcv_dnn_MaxUnpoolLayerG_getInnerPtr_const, cv_PtrLcv_dnn_MaxUnpoolLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::MaxUnpoolLayer, cv_PtrLcv_dnn_MaxUnpoolLayerG_new_const_MaxUnpoolLayer }
impl core::Ptr<crate::dnn::MaxUnpoolLayer> {
	#[inline] pub fn as_raw_PtrOfMaxUnpoolLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMaxUnpoolLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::MaxUnpoolLayerTraitConst for core::Ptr<crate::dnn::MaxUnpoolLayer> {
	#[inline] fn as_raw_MaxUnpoolLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::MaxUnpoolLayerTrait for core::Ptr<crate::dnn::MaxUnpoolLayer> {
	#[inline] fn as_raw_mut_MaxUnpoolLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::MaxUnpoolLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::MaxUnpoolLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::MaxUnpoolLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_MaxUnpoolLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::MaxUnpoolLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::MaxUnpoolLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::MaxUnpoolLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_MaxUnpoolLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::MaxUnpoolLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMaxUnpoolLayer")
			.field("pool_kernel", &crate::dnn::MaxUnpoolLayerTraitConst::pool_kernel(self))
			.field("pool_pad", &crate::dnn::MaxUnpoolLayerTraitConst::pool_pad(self))
			.field("pool_stride", &crate::dnn::MaxUnpoolLayerTraitConst::pool_stride(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

