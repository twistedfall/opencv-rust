ptr_extern! { crate::ximgproc::EdgeAwareInterpolator,
	cv_PtrLcv_ximgproc_EdgeAwareInterpolatorG_new_null_const, cv_PtrLcv_ximgproc_EdgeAwareInterpolatorG_delete, cv_PtrLcv_ximgproc_EdgeAwareInterpolatorG_getInnerPtr_const, cv_PtrLcv_ximgproc_EdgeAwareInterpolatorG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::EdgeAwareInterpolator> {
	#[inline] pub fn as_raw_PtrOfEdgeAwareInterpolator(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfEdgeAwareInterpolator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::EdgeAwareInterpolatorTraitConst for core::Ptr<crate::ximgproc::EdgeAwareInterpolator> {
	#[inline] fn as_raw_EdgeAwareInterpolator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::EdgeAwareInterpolatorTrait for core::Ptr<crate::ximgproc::EdgeAwareInterpolator> {
	#[inline] fn as_raw_mut_EdgeAwareInterpolator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::EdgeAwareInterpolator> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::EdgeAwareInterpolator> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::EdgeAwareInterpolator>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_EdgeAwareInterpolatorG_to_PtrOfAlgorithm }

impl crate::ximgproc::SparseMatchInterpolatorTraitConst for core::Ptr<crate::ximgproc::EdgeAwareInterpolator> {
	#[inline] fn as_raw_SparseMatchInterpolator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SparseMatchInterpolatorTrait for core::Ptr<crate::ximgproc::EdgeAwareInterpolator> {
	#[inline] fn as_raw_mut_SparseMatchInterpolator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::EdgeAwareInterpolator>, core::Ptr<crate::ximgproc::SparseMatchInterpolator>, cv_PtrLcv_ximgproc_EdgeAwareInterpolatorG_to_PtrOfSparseMatchInterpolator }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::EdgeAwareInterpolator> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfEdgeAwareInterpolator")
			.finish()
	}
}

