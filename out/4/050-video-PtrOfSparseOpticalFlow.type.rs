ptr_extern! { crate::video::SparseOpticalFlow,
	cv_PtrLcv_SparseOpticalFlowG_new_null_const, cv_PtrLcv_SparseOpticalFlowG_delete, cv_PtrLcv_SparseOpticalFlowG_getInnerPtr_const, cv_PtrLcv_SparseOpticalFlowG_getInnerPtrMut
}

impl core::Ptr<crate::video::SparseOpticalFlow> {
	#[inline] pub fn as_raw_PtrOfSparseOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSparseOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::video::SparseOpticalFlowTraitConst for core::Ptr<crate::video::SparseOpticalFlow> {
	#[inline] fn as_raw_SparseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::SparseOpticalFlowTrait for core::Ptr<crate::video::SparseOpticalFlow> {
	#[inline] fn as_raw_mut_SparseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::video::SparseOpticalFlow> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::video::SparseOpticalFlow> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::video::SparseOpticalFlow>, core::Ptr<core::Algorithm>, cv_PtrLcv_SparseOpticalFlowG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::video::SparseOpticalFlow> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSparseOpticalFlow")
			.finish()
	}
}

