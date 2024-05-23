ptr_extern! { crate::photo::CalibrateRobertson,
	cv_PtrLcv_CalibrateRobertsonG_new_null_const, cv_PtrLcv_CalibrateRobertsonG_delete, cv_PtrLcv_CalibrateRobertsonG_getInnerPtr_const, cv_PtrLcv_CalibrateRobertsonG_getInnerPtrMut
}

impl core::Ptr<crate::photo::CalibrateRobertson> {
	#[inline] pub fn as_raw_PtrOfCalibrateRobertson(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCalibrateRobertson(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::photo::CalibrateRobertsonTraitConst for core::Ptr<crate::photo::CalibrateRobertson> {
	#[inline] fn as_raw_CalibrateRobertson(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::CalibrateRobertsonTrait for core::Ptr<crate::photo::CalibrateRobertson> {
	#[inline] fn as_raw_mut_CalibrateRobertson(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::photo::CalibrateRobertson> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::photo::CalibrateRobertson> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::photo::CalibrateRobertson>, core::Ptr<core::Algorithm>, cv_PtrLcv_CalibrateRobertsonG_to_PtrOfAlgorithm }

impl crate::photo::CalibrateCRFTraitConst for core::Ptr<crate::photo::CalibrateRobertson> {
	#[inline] fn as_raw_CalibrateCRF(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::CalibrateCRFTrait for core::Ptr<crate::photo::CalibrateRobertson> {
	#[inline] fn as_raw_mut_CalibrateCRF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::photo::CalibrateRobertson>, core::Ptr<crate::photo::CalibrateCRF>, cv_PtrLcv_CalibrateRobertsonG_to_PtrOfCalibrateCRF }

impl std::fmt::Debug for core::Ptr<crate::photo::CalibrateRobertson> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCalibrateRobertson")
			.finish()
	}
}

