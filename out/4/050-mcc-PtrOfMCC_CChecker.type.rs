ptr_extern! { crate::mcc::MCC_CChecker,
	cv_PtrLcv_mcc_CCheckerG_new_null_const, cv_PtrLcv_mcc_CCheckerG_delete, cv_PtrLcv_mcc_CCheckerG_getInnerPtr_const, cv_PtrLcv_mcc_CCheckerG_getInnerPtrMut
}

impl core::Ptr<crate::mcc::MCC_CChecker> {
	#[inline] pub fn as_raw_PtrOfMCC_CChecker(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMCC_CChecker(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::mcc::MCC_CCheckerTraitConst for core::Ptr<crate::mcc::MCC_CChecker> {
	#[inline] fn as_raw_MCC_CChecker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::mcc::MCC_CCheckerTrait for core::Ptr<crate::mcc::MCC_CChecker> {
	#[inline] fn as_raw_mut_MCC_CChecker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::mcc::MCC_CChecker> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMCC_CChecker")
			.finish()
	}
}

