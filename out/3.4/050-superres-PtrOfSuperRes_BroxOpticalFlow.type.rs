ptr_extern! { crate::superres::SuperRes_BroxOpticalFlow,
	cv_PtrLcv_superres_BroxOpticalFlowG_new_null_const, cv_PtrLcv_superres_BroxOpticalFlowG_delete, cv_PtrLcv_superres_BroxOpticalFlowG_getInnerPtr_const, cv_PtrLcv_superres_BroxOpticalFlowG_getInnerPtrMut
}

impl core::Ptr<crate::superres::SuperRes_BroxOpticalFlow> {
	#[inline] pub fn as_raw_PtrOfSuperRes_BroxOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSuperRes_BroxOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::superres::SuperRes_BroxOpticalFlowTraitConst for core::Ptr<crate::superres::SuperRes_BroxOpticalFlow> {
	#[inline] fn as_raw_SuperRes_BroxOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::superres::SuperRes_BroxOpticalFlowTrait for core::Ptr<crate::superres::SuperRes_BroxOpticalFlow> {
	#[inline] fn as_raw_mut_SuperRes_BroxOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::superres::SuperRes_BroxOpticalFlow> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::superres::SuperRes_BroxOpticalFlow> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::superres::SuperRes_BroxOpticalFlow>, core::Ptr<core::Algorithm>, cv_PtrLcv_superres_BroxOpticalFlowG_to_PtrOfAlgorithm }

impl crate::superres::SuperRes_DenseOpticalFlowExtTraitConst for core::Ptr<crate::superres::SuperRes_BroxOpticalFlow> {
	#[inline] fn as_raw_SuperRes_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::superres::SuperRes_DenseOpticalFlowExtTrait for core::Ptr<crate::superres::SuperRes_BroxOpticalFlow> {
	#[inline] fn as_raw_mut_SuperRes_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::superres::SuperRes_BroxOpticalFlow>, core::Ptr<crate::superres::SuperRes_DenseOpticalFlowExt>, cv_PtrLcv_superres_BroxOpticalFlowG_to_PtrOfSuperRes_DenseOpticalFlowExt }

impl std::fmt::Debug for core::Ptr<crate::superres::SuperRes_BroxOpticalFlow> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSuperRes_BroxOpticalFlow")
			.finish()
	}
}

