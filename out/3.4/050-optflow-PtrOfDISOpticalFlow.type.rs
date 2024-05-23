ptr_extern! { crate::optflow::DISOpticalFlow,
	cv_PtrLcv_optflow_DISOpticalFlowG_new_null_const, cv_PtrLcv_optflow_DISOpticalFlowG_delete, cv_PtrLcv_optflow_DISOpticalFlowG_getInnerPtr_const, cv_PtrLcv_optflow_DISOpticalFlowG_getInnerPtrMut
}

impl core::Ptr<crate::optflow::DISOpticalFlow> {
	#[inline] pub fn as_raw_PtrOfDISOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDISOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::optflow::DISOpticalFlowTraitConst for core::Ptr<crate::optflow::DISOpticalFlow> {
	#[inline] fn as_raw_DISOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::optflow::DISOpticalFlowTrait for core::Ptr<crate::optflow::DISOpticalFlow> {
	#[inline] fn as_raw_mut_DISOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::optflow::DISOpticalFlow> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::optflow::DISOpticalFlow> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::optflow::DISOpticalFlow>, core::Ptr<core::Algorithm>, cv_PtrLcv_optflow_DISOpticalFlowG_to_PtrOfAlgorithm }

impl crate::video::DenseOpticalFlowTraitConst for core::Ptr<crate::optflow::DISOpticalFlow> {
	#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::DenseOpticalFlowTrait for core::Ptr<crate::optflow::DISOpticalFlow> {
	#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::optflow::DISOpticalFlow>, core::Ptr<crate::video::DenseOpticalFlow>, cv_PtrLcv_optflow_DISOpticalFlowG_to_PtrOfDenseOpticalFlow }

impl std::fmt::Debug for core::Ptr<crate::optflow::DISOpticalFlow> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDISOpticalFlow")
			.finish()
	}
}

