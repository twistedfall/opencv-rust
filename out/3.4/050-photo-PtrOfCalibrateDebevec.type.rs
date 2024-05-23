ptr_extern! { crate::photo::CalibrateDebevec,
	cv_PtrLcv_CalibrateDebevecG_new_null_const, cv_PtrLcv_CalibrateDebevecG_delete, cv_PtrLcv_CalibrateDebevecG_getInnerPtr_const, cv_PtrLcv_CalibrateDebevecG_getInnerPtrMut
}

impl core::Ptr<crate::photo::CalibrateDebevec> {
	#[inline] pub fn as_raw_PtrOfCalibrateDebevec(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCalibrateDebevec(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::photo::CalibrateDebevecTraitConst for core::Ptr<crate::photo::CalibrateDebevec> {
	#[inline] fn as_raw_CalibrateDebevec(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::CalibrateDebevecTrait for core::Ptr<crate::photo::CalibrateDebevec> {
	#[inline] fn as_raw_mut_CalibrateDebevec(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::photo::CalibrateDebevec> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::photo::CalibrateDebevec> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::photo::CalibrateDebevec>, core::Ptr<core::Algorithm>, cv_PtrLcv_CalibrateDebevecG_to_PtrOfAlgorithm }

impl crate::photo::CalibrateCRFTraitConst for core::Ptr<crate::photo::CalibrateDebevec> {
	#[inline] fn as_raw_CalibrateCRF(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::CalibrateCRFTrait for core::Ptr<crate::photo::CalibrateDebevec> {
	#[inline] fn as_raw_mut_CalibrateCRF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::photo::CalibrateDebevec>, core::Ptr<crate::photo::CalibrateCRF>, cv_PtrLcv_CalibrateDebevecG_to_PtrOfCalibrateCRF }

impl std::fmt::Debug for core::Ptr<crate::photo::CalibrateDebevec> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCalibrateDebevec")
			.finish()
	}
}

