ptr_extern! { crate::mcc::MCC_CCheckerDraw,
	cv_PtrLcv_mcc_CCheckerDrawG_new_null_const, cv_PtrLcv_mcc_CCheckerDrawG_delete, cv_PtrLcv_mcc_CCheckerDrawG_getInnerPtr_const, cv_PtrLcv_mcc_CCheckerDrawG_getInnerPtrMut
}

impl core::Ptr<crate::mcc::MCC_CCheckerDraw> {
	#[inline] pub fn as_raw_PtrOfMCC_CCheckerDraw(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMCC_CCheckerDraw(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::mcc::MCC_CCheckerDrawTraitConst for core::Ptr<crate::mcc::MCC_CCheckerDraw> {
	#[inline] fn as_raw_MCC_CCheckerDraw(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::mcc::MCC_CCheckerDrawTrait for core::Ptr<crate::mcc::MCC_CCheckerDraw> {
	#[inline] fn as_raw_mut_MCC_CCheckerDraw(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::mcc::MCC_CCheckerDraw> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMCC_CCheckerDraw")
			.finish()
	}
}

