ptr_extern! { crate::ximgproc::SparseMatchInterpolator,
	cv_PtrLcv_ximgproc_SparseMatchInterpolatorG_new_null_const, cv_PtrLcv_ximgproc_SparseMatchInterpolatorG_delete, cv_PtrLcv_ximgproc_SparseMatchInterpolatorG_getInnerPtr_const, cv_PtrLcv_ximgproc_SparseMatchInterpolatorG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::SparseMatchInterpolator> {
	#[inline] pub fn as_raw_PtrOfSparseMatchInterpolator(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSparseMatchInterpolator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::SparseMatchInterpolatorTraitConst for core::Ptr<crate::ximgproc::SparseMatchInterpolator> {
	#[inline] fn as_raw_SparseMatchInterpolator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SparseMatchInterpolatorTrait for core::Ptr<crate::ximgproc::SparseMatchInterpolator> {
	#[inline] fn as_raw_mut_SparseMatchInterpolator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::SparseMatchInterpolator> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::SparseMatchInterpolator> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::SparseMatchInterpolator>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_SparseMatchInterpolatorG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::SparseMatchInterpolator> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSparseMatchInterpolator")
			.finish()
	}
}

