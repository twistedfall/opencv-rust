ptr_extern! { crate::video::SparsePyrLKOpticalFlow,
	cv_PtrLcv_SparsePyrLKOpticalFlowG_new_null_const, cv_PtrLcv_SparsePyrLKOpticalFlowG_delete, cv_PtrLcv_SparsePyrLKOpticalFlowG_getInnerPtr_const, cv_PtrLcv_SparsePyrLKOpticalFlowG_getInnerPtrMut
}

impl core::Ptr<crate::video::SparsePyrLKOpticalFlow> {
	#[inline] pub fn as_raw_PtrOfSparsePyrLKOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSparsePyrLKOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::video::SparsePyrLKOpticalFlowTraitConst for core::Ptr<crate::video::SparsePyrLKOpticalFlow> {
	#[inline] fn as_raw_SparsePyrLKOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::SparsePyrLKOpticalFlowTrait for core::Ptr<crate::video::SparsePyrLKOpticalFlow> {
	#[inline] fn as_raw_mut_SparsePyrLKOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::video::SparsePyrLKOpticalFlow> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::video::SparsePyrLKOpticalFlow> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::video::SparsePyrLKOpticalFlow>, core::Ptr<core::Algorithm>, cv_PtrLcv_SparsePyrLKOpticalFlowG_to_PtrOfAlgorithm }

impl crate::video::SparseOpticalFlowTraitConst for core::Ptr<crate::video::SparsePyrLKOpticalFlow> {
	#[inline] fn as_raw_SparseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::SparseOpticalFlowTrait for core::Ptr<crate::video::SparsePyrLKOpticalFlow> {
	#[inline] fn as_raw_mut_SparseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::video::SparsePyrLKOpticalFlow>, core::Ptr<crate::video::SparseOpticalFlow>, cv_PtrLcv_SparsePyrLKOpticalFlowG_to_PtrOfSparseOpticalFlow }

impl std::fmt::Debug for core::Ptr<crate::video::SparsePyrLKOpticalFlow> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSparsePyrLKOpticalFlow")
			.finish()
	}
}

