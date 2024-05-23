ptr_extern! { crate::photo::CalibrateCRF,
	cv_PtrLcv_CalibrateCRFG_new_null_const, cv_PtrLcv_CalibrateCRFG_delete, cv_PtrLcv_CalibrateCRFG_getInnerPtr_const, cv_PtrLcv_CalibrateCRFG_getInnerPtrMut
}

impl core::Ptr<crate::photo::CalibrateCRF> {
	#[inline] pub fn as_raw_PtrOfCalibrateCRF(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCalibrateCRF(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::photo::CalibrateCRFTraitConst for core::Ptr<crate::photo::CalibrateCRF> {
	#[inline] fn as_raw_CalibrateCRF(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::CalibrateCRFTrait for core::Ptr<crate::photo::CalibrateCRF> {
	#[inline] fn as_raw_mut_CalibrateCRF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::photo::CalibrateCRF> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::photo::CalibrateCRF> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::photo::CalibrateCRF>, core::Ptr<core::Algorithm>, cv_PtrLcv_CalibrateCRFG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::photo::CalibrateCRF> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCalibrateCRF")
			.finish()
	}
}

