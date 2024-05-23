ptr_extern! { crate::videostab::InpaintingPipeline,
	cv_PtrLcv_videostab_InpaintingPipelineG_new_null_const, cv_PtrLcv_videostab_InpaintingPipelineG_delete, cv_PtrLcv_videostab_InpaintingPipelineG_getInnerPtr_const, cv_PtrLcv_videostab_InpaintingPipelineG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::InpaintingPipeline, cv_PtrLcv_videostab_InpaintingPipelineG_new_const_InpaintingPipeline }
impl core::Ptr<crate::videostab::InpaintingPipeline> {
	#[inline] pub fn as_raw_PtrOfInpaintingPipeline(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfInpaintingPipeline(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::InpaintingPipelineTraitConst for core::Ptr<crate::videostab::InpaintingPipeline> {
	#[inline] fn as_raw_InpaintingPipeline(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::InpaintingPipelineTrait for core::Ptr<crate::videostab::InpaintingPipeline> {
	#[inline] fn as_raw_mut_InpaintingPipeline(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::InpainterBaseTraitConst for core::Ptr<crate::videostab::InpaintingPipeline> {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::InpainterBaseTrait for core::Ptr<crate::videostab::InpaintingPipeline> {
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::InpaintingPipeline>, core::Ptr<crate::videostab::InpainterBase>, cv_PtrLcv_videostab_InpaintingPipelineG_to_PtrOfInpainterBase }

impl std::fmt::Debug for core::Ptr<crate::videostab::InpaintingPipeline> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfInpaintingPipeline")
			.finish()
	}
}

