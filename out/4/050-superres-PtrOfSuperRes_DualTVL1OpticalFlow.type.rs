ptr_extern! { crate::superres::SuperRes_DualTVL1OpticalFlow,
	cv_PtrLcv_superres_DualTVL1OpticalFlowG_new_null_const, cv_PtrLcv_superres_DualTVL1OpticalFlowG_delete, cv_PtrLcv_superres_DualTVL1OpticalFlowG_getInnerPtr_const, cv_PtrLcv_superres_DualTVL1OpticalFlowG_getInnerPtrMut
}

impl core::Ptr<crate::superres::SuperRes_DualTVL1OpticalFlow> {
	#[inline] pub fn as_raw_PtrOfSuperRes_DualTVL1OpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSuperRes_DualTVL1OpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::superres::SuperRes_DualTVL1OpticalFlowTraitConst for core::Ptr<crate::superres::SuperRes_DualTVL1OpticalFlow> {
	#[inline] fn as_raw_SuperRes_DualTVL1OpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::superres::SuperRes_DualTVL1OpticalFlowTrait for core::Ptr<crate::superres::SuperRes_DualTVL1OpticalFlow> {
	#[inline] fn as_raw_mut_SuperRes_DualTVL1OpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::superres::SuperRes_DualTVL1OpticalFlow> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::superres::SuperRes_DualTVL1OpticalFlow> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::superres::SuperRes_DualTVL1OpticalFlow>, core::Ptr<core::Algorithm>, cv_PtrLcv_superres_DualTVL1OpticalFlowG_to_PtrOfAlgorithm }

impl crate::superres::SuperRes_DenseOpticalFlowExtTraitConst for core::Ptr<crate::superres::SuperRes_DualTVL1OpticalFlow> {
	#[inline] fn as_raw_SuperRes_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::superres::SuperRes_DenseOpticalFlowExtTrait for core::Ptr<crate::superres::SuperRes_DualTVL1OpticalFlow> {
	#[inline] fn as_raw_mut_SuperRes_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::superres::SuperRes_DualTVL1OpticalFlow>, core::Ptr<crate::superres::SuperRes_DenseOpticalFlowExt>, cv_PtrLcv_superres_DualTVL1OpticalFlowG_to_PtrOfSuperRes_DenseOpticalFlowExt }

impl std::fmt::Debug for core::Ptr<crate::superres::SuperRes_DualTVL1OpticalFlow> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSuperRes_DualTVL1OpticalFlow")
			.finish()
	}
}

