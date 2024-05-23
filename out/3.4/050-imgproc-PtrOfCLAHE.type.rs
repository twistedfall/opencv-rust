ptr_extern! { crate::imgproc::CLAHE,
	cv_PtrLcv_CLAHEG_new_null_const, cv_PtrLcv_CLAHEG_delete, cv_PtrLcv_CLAHEG_getInnerPtr_const, cv_PtrLcv_CLAHEG_getInnerPtrMut
}

impl core::Ptr<crate::imgproc::CLAHE> {
	#[inline] pub fn as_raw_PtrOfCLAHE(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCLAHE(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::imgproc::CLAHETraitConst for core::Ptr<crate::imgproc::CLAHE> {
	#[inline] fn as_raw_CLAHE(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::imgproc::CLAHETrait for core::Ptr<crate::imgproc::CLAHE> {
	#[inline] fn as_raw_mut_CLAHE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::imgproc::CLAHE> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::imgproc::CLAHE> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::imgproc::CLAHE>, core::Ptr<core::Algorithm>, cv_PtrLcv_CLAHEG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::imgproc::CLAHE> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCLAHE")
			.finish()
	}
}

