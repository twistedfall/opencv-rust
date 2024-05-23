ptr_extern! { crate::optflow::SparseRLOFOpticalFlow,
	cv_PtrLcv_optflow_SparseRLOFOpticalFlowG_new_null_const, cv_PtrLcv_optflow_SparseRLOFOpticalFlowG_delete, cv_PtrLcv_optflow_SparseRLOFOpticalFlowG_getInnerPtr_const, cv_PtrLcv_optflow_SparseRLOFOpticalFlowG_getInnerPtrMut
}

impl core::Ptr<crate::optflow::SparseRLOFOpticalFlow> {
	#[inline] pub fn as_raw_PtrOfSparseRLOFOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSparseRLOFOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::optflow::SparseRLOFOpticalFlowTraitConst for core::Ptr<crate::optflow::SparseRLOFOpticalFlow> {
	#[inline] fn as_raw_SparseRLOFOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::optflow::SparseRLOFOpticalFlowTrait for core::Ptr<crate::optflow::SparseRLOFOpticalFlow> {
	#[inline] fn as_raw_mut_SparseRLOFOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::optflow::SparseRLOFOpticalFlow> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::optflow::SparseRLOFOpticalFlow> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::optflow::SparseRLOFOpticalFlow>, core::Ptr<core::Algorithm>, cv_PtrLcv_optflow_SparseRLOFOpticalFlowG_to_PtrOfAlgorithm }

impl crate::video::SparseOpticalFlowTraitConst for core::Ptr<crate::optflow::SparseRLOFOpticalFlow> {
	#[inline] fn as_raw_SparseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::SparseOpticalFlowTrait for core::Ptr<crate::optflow::SparseRLOFOpticalFlow> {
	#[inline] fn as_raw_mut_SparseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::optflow::SparseRLOFOpticalFlow>, core::Ptr<crate::video::SparseOpticalFlow>, cv_PtrLcv_optflow_SparseRLOFOpticalFlowG_to_PtrOfSparseOpticalFlow }

impl std::fmt::Debug for core::Ptr<crate::optflow::SparseRLOFOpticalFlow> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSparseRLOFOpticalFlow")
			.finish()
	}
}

