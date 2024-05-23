ptr_extern! { crate::mcc::MCC_CCheckerDetector,
	cv_PtrLcv_mcc_CCheckerDetectorG_new_null_const, cv_PtrLcv_mcc_CCheckerDetectorG_delete, cv_PtrLcv_mcc_CCheckerDetectorG_getInnerPtr_const, cv_PtrLcv_mcc_CCheckerDetectorG_getInnerPtrMut
}

impl core::Ptr<crate::mcc::MCC_CCheckerDetector> {
	#[inline] pub fn as_raw_PtrOfMCC_CCheckerDetector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMCC_CCheckerDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::mcc::MCC_CCheckerDetectorTraitConst for core::Ptr<crate::mcc::MCC_CCheckerDetector> {
	#[inline] fn as_raw_MCC_CCheckerDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::mcc::MCC_CCheckerDetectorTrait for core::Ptr<crate::mcc::MCC_CCheckerDetector> {
	#[inline] fn as_raw_mut_MCC_CCheckerDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::mcc::MCC_CCheckerDetector> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::mcc::MCC_CCheckerDetector> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::mcc::MCC_CCheckerDetector>, core::Ptr<core::Algorithm>, cv_PtrLcv_mcc_CCheckerDetectorG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::mcc::MCC_CCheckerDetector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMCC_CCheckerDetector")
			.finish()
	}
}

