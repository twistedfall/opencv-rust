ptr_extern! { crate::optflow::DualTVL1OpticalFlow,
	cv_PtrLcv_optflow_DualTVL1OpticalFlowG_new_null_const, cv_PtrLcv_optflow_DualTVL1OpticalFlowG_delete, cv_PtrLcv_optflow_DualTVL1OpticalFlowG_getInnerPtr_const, cv_PtrLcv_optflow_DualTVL1OpticalFlowG_getInnerPtrMut
}

impl core::Ptr<crate::optflow::DualTVL1OpticalFlow> {
	#[inline] pub fn as_raw_PtrOfDualTVL1OpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDualTVL1OpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::optflow::DualTVL1OpticalFlowTraitConst for core::Ptr<crate::optflow::DualTVL1OpticalFlow> {
	#[inline] fn as_raw_DualTVL1OpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::optflow::DualTVL1OpticalFlowTrait for core::Ptr<crate::optflow::DualTVL1OpticalFlow> {
	#[inline] fn as_raw_mut_DualTVL1OpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::optflow::DualTVL1OpticalFlow> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::optflow::DualTVL1OpticalFlow> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::optflow::DualTVL1OpticalFlow>, core::Ptr<core::Algorithm>, cv_PtrLcv_optflow_DualTVL1OpticalFlowG_to_PtrOfAlgorithm }

impl crate::video::DenseOpticalFlowTraitConst for core::Ptr<crate::optflow::DualTVL1OpticalFlow> {
	#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::DenseOpticalFlowTrait for core::Ptr<crate::optflow::DualTVL1OpticalFlow> {
	#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::optflow::DualTVL1OpticalFlow>, core::Ptr<crate::video::DenseOpticalFlow>, cv_PtrLcv_optflow_DualTVL1OpticalFlowG_to_PtrOfDenseOpticalFlow }

impl std::fmt::Debug for core::Ptr<crate::optflow::DualTVL1OpticalFlow> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDualTVL1OpticalFlow")
			.finish()
	}
}

