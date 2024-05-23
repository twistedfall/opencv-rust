ptr_extern! { crate::ximgproc::ContourFitting,
	cv_PtrLcv_ximgproc_ContourFittingG_new_null_const, cv_PtrLcv_ximgproc_ContourFittingG_delete, cv_PtrLcv_ximgproc_ContourFittingG_getInnerPtr_const, cv_PtrLcv_ximgproc_ContourFittingG_getInnerPtrMut
}

ptr_extern_ctor! { crate::ximgproc::ContourFitting, cv_PtrLcv_ximgproc_ContourFittingG_new_const_ContourFitting }
impl core::Ptr<crate::ximgproc::ContourFitting> {
	#[inline] pub fn as_raw_PtrOfContourFitting(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfContourFitting(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::ContourFittingTraitConst for core::Ptr<crate::ximgproc::ContourFitting> {
	#[inline] fn as_raw_ContourFitting(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::ContourFittingTrait for core::Ptr<crate::ximgproc::ContourFitting> {
	#[inline] fn as_raw_mut_ContourFitting(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::ContourFitting> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::ContourFitting> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::ContourFitting>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_ContourFittingG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::ContourFitting> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfContourFitting")
			.finish()
	}
}

