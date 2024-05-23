ptr_extern! { crate::dnn::ProposalLayer,
	cv_PtrLcv_dnn_ProposalLayerG_new_null_const, cv_PtrLcv_dnn_ProposalLayerG_delete, cv_PtrLcv_dnn_ProposalLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ProposalLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::ProposalLayer, cv_PtrLcv_dnn_ProposalLayerG_new_const_ProposalLayer }
impl core::Ptr<crate::dnn::ProposalLayer> {
	#[inline] pub fn as_raw_PtrOfProposalLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfProposalLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::ProposalLayerTraitConst for core::Ptr<crate::dnn::ProposalLayer> {
	#[inline] fn as_raw_ProposalLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ProposalLayerTrait for core::Ptr<crate::dnn::ProposalLayer> {
	#[inline] fn as_raw_mut_ProposalLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ProposalLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::ProposalLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ProposalLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ProposalLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ProposalLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ProposalLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ProposalLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ProposalLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::ProposalLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfProposalLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

