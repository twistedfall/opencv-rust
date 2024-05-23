ptr_extern! { crate::dpm::DPMDetector,
	cv_PtrLcv_dpm_DPMDetectorG_new_null_const, cv_PtrLcv_dpm_DPMDetectorG_delete, cv_PtrLcv_dpm_DPMDetectorG_getInnerPtr_const, cv_PtrLcv_dpm_DPMDetectorG_getInnerPtrMut
}

impl core::Ptr<crate::dpm::DPMDetector> {
	#[inline] pub fn as_raw_PtrOfDPMDetector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDPMDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dpm::DPMDetectorTraitConst for core::Ptr<crate::dpm::DPMDetector> {
	#[inline] fn as_raw_DPMDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dpm::DPMDetectorTrait for core::Ptr<crate::dpm::DPMDetector> {
	#[inline] fn as_raw_mut_DPMDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::dpm::DPMDetector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDPMDetector")
			.finish()
	}
}

