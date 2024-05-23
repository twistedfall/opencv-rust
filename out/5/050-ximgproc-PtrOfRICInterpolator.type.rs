ptr_extern! { crate::ximgproc::RICInterpolator,
	cv_PtrLcv_ximgproc_RICInterpolatorG_new_null_const, cv_PtrLcv_ximgproc_RICInterpolatorG_delete, cv_PtrLcv_ximgproc_RICInterpolatorG_getInnerPtr_const, cv_PtrLcv_ximgproc_RICInterpolatorG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::RICInterpolator> {
	#[inline] pub fn as_raw_PtrOfRICInterpolator(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRICInterpolator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::RICInterpolatorTraitConst for core::Ptr<crate::ximgproc::RICInterpolator> {
	#[inline] fn as_raw_RICInterpolator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::RICInterpolatorTrait for core::Ptr<crate::ximgproc::RICInterpolator> {
	#[inline] fn as_raw_mut_RICInterpolator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::RICInterpolator> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::RICInterpolator> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::RICInterpolator>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_RICInterpolatorG_to_PtrOfAlgorithm }

impl crate::ximgproc::SparseMatchInterpolatorTraitConst for core::Ptr<crate::ximgproc::RICInterpolator> {
	#[inline] fn as_raw_SparseMatchInterpolator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SparseMatchInterpolatorTrait for core::Ptr<crate::ximgproc::RICInterpolator> {
	#[inline] fn as_raw_mut_SparseMatchInterpolator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::RICInterpolator>, core::Ptr<crate::ximgproc::SparseMatchInterpolator>, cv_PtrLcv_ximgproc_RICInterpolatorG_to_PtrOfSparseMatchInterpolator }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::RICInterpolator> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfRICInterpolator")
			.finish()
	}
}

