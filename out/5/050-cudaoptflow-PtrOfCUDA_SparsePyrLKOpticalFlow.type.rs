ptr_extern! { crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow,
	cv_PtrLcv_cuda_SparsePyrLKOpticalFlowG_new_null_const, cv_PtrLcv_cuda_SparsePyrLKOpticalFlowG_delete, cv_PtrLcv_cuda_SparsePyrLKOpticalFlowG_getInnerPtr_const, cv_PtrLcv_cuda_SparsePyrLKOpticalFlowG_getInnerPtrMut
}

impl core::Ptr<crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow> {
	#[inline] pub fn as_raw_PtrOfCUDA_SparsePyrLKOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCUDA_SparsePyrLKOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlowTraitConst for core::Ptr<crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow> {
	#[inline] fn as_raw_CUDA_SparsePyrLKOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlowTrait for core::Ptr<crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow> {
	#[inline] fn as_raw_mut_CUDA_SparsePyrLKOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow>, core::Ptr<core::Algorithm>, cv_PtrLcv_cuda_SparsePyrLKOpticalFlowG_to_PtrOfAlgorithm }

impl crate::cudaoptflow::CUDA_SparseOpticalFlowTraitConst for core::Ptr<crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow> {
	#[inline] fn as_raw_CUDA_SparseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::cudaoptflow::CUDA_SparseOpticalFlowTrait for core::Ptr<crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow> {
	#[inline] fn as_raw_mut_CUDA_SparseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow>, core::Ptr<crate::cudaoptflow::CUDA_SparseOpticalFlow>, cv_PtrLcv_cuda_SparsePyrLKOpticalFlowG_to_PtrOfCUDA_SparseOpticalFlow }

impl std::fmt::Debug for core::Ptr<crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCUDA_SparsePyrLKOpticalFlow")
			.finish()
	}
}

