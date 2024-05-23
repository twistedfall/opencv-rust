ptr_extern! { crate::xfeatures2d::SURF_CUDA,
	cv_PtrLcv_cuda_SURF_CUDAG_new_null_const, cv_PtrLcv_cuda_SURF_CUDAG_delete, cv_PtrLcv_cuda_SURF_CUDAG_getInnerPtr_const, cv_PtrLcv_cuda_SURF_CUDAG_getInnerPtrMut
}

ptr_extern_ctor! { crate::xfeatures2d::SURF_CUDA, cv_PtrLcv_cuda_SURF_CUDAG_new_const_SURF_CUDA }
impl core::Ptr<crate::xfeatures2d::SURF_CUDA> {
	#[inline] pub fn as_raw_PtrOfSURF_CUDA(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSURF_CUDA(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::xfeatures2d::SURF_CUDATraitConst for core::Ptr<crate::xfeatures2d::SURF_CUDA> {
	#[inline] fn as_raw_SURF_CUDA(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::SURF_CUDATrait for core::Ptr<crate::xfeatures2d::SURF_CUDA> {
	#[inline] fn as_raw_mut_SURF_CUDA(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::xfeatures2d::SURF_CUDA> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSURF_CUDA")
			.field("hessian_threshold", &crate::xfeatures2d::SURF_CUDATraitConst::hessian_threshold(self))
			.field("n_octaves", &crate::xfeatures2d::SURF_CUDATraitConst::n_octaves(self))
			.field("n_octave_layers", &crate::xfeatures2d::SURF_CUDATraitConst::n_octave_layers(self))
			.field("extended", &crate::xfeatures2d::SURF_CUDATraitConst::extended(self))
			.field("upright", &crate::xfeatures2d::SURF_CUDATraitConst::upright(self))
			.field("keypoints_ratio", &crate::xfeatures2d::SURF_CUDATraitConst::keypoints_ratio(self))
			.field("sum", &crate::xfeatures2d::SURF_CUDATraitConst::sum(self))
			.field("mask1", &crate::xfeatures2d::SURF_CUDATraitConst::mask1(self))
			.field("mask_sum", &crate::xfeatures2d::SURF_CUDATraitConst::mask_sum(self))
			.field("det", &crate::xfeatures2d::SURF_CUDATraitConst::det(self))
			.field("trace", &crate::xfeatures2d::SURF_CUDATraitConst::trace(self))
			.field("max_pos_buffer", &crate::xfeatures2d::SURF_CUDATraitConst::max_pos_buffer(self))
			.finish()
	}
}

