ptr_extern! { crate::ximgproc::FastLineDetector,
	cv_PtrLcv_ximgproc_FastLineDetectorG_new_null_const, cv_PtrLcv_ximgproc_FastLineDetectorG_delete, cv_PtrLcv_ximgproc_FastLineDetectorG_getInnerPtr_const, cv_PtrLcv_ximgproc_FastLineDetectorG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::FastLineDetector> {
	#[inline] pub fn as_raw_PtrOfFastLineDetector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFastLineDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::FastLineDetectorTraitConst for core::Ptr<crate::ximgproc::FastLineDetector> {
	#[inline] fn as_raw_FastLineDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::FastLineDetectorTrait for core::Ptr<crate::ximgproc::FastLineDetector> {
	#[inline] fn as_raw_mut_FastLineDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::FastLineDetector> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::FastLineDetector> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::FastLineDetector>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_FastLineDetectorG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::FastLineDetector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfFastLineDetector")
			.finish()
	}
}

