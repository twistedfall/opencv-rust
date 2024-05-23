ptr_extern! { crate::optflow::DenseRLOFOpticalFlow,
	cv_PtrLcv_optflow_DenseRLOFOpticalFlowG_new_null_const, cv_PtrLcv_optflow_DenseRLOFOpticalFlowG_delete, cv_PtrLcv_optflow_DenseRLOFOpticalFlowG_getInnerPtr_const, cv_PtrLcv_optflow_DenseRLOFOpticalFlowG_getInnerPtrMut
}

impl core::Ptr<crate::optflow::DenseRLOFOpticalFlow> {
	#[inline] pub fn as_raw_PtrOfDenseRLOFOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDenseRLOFOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::optflow::DenseRLOFOpticalFlowTraitConst for core::Ptr<crate::optflow::DenseRLOFOpticalFlow> {
	#[inline] fn as_raw_DenseRLOFOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::optflow::DenseRLOFOpticalFlowTrait for core::Ptr<crate::optflow::DenseRLOFOpticalFlow> {
	#[inline] fn as_raw_mut_DenseRLOFOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::optflow::DenseRLOFOpticalFlow> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::optflow::DenseRLOFOpticalFlow> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::optflow::DenseRLOFOpticalFlow>, core::Ptr<core::Algorithm>, cv_PtrLcv_optflow_DenseRLOFOpticalFlowG_to_PtrOfAlgorithm }

impl crate::video::DenseOpticalFlowTraitConst for core::Ptr<crate::optflow::DenseRLOFOpticalFlow> {
	#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::DenseOpticalFlowTrait for core::Ptr<crate::optflow::DenseRLOFOpticalFlow> {
	#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::optflow::DenseRLOFOpticalFlow>, core::Ptr<crate::video::DenseOpticalFlow>, cv_PtrLcv_optflow_DenseRLOFOpticalFlowG_to_PtrOfDenseOpticalFlow }

impl std::fmt::Debug for core::Ptr<crate::optflow::DenseRLOFOpticalFlow> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDenseRLOFOpticalFlow")
			.finish()
	}
}

