ptr_extern! { crate::aruco::EstimateParameters,
	cv_PtrLcv_aruco_EstimateParametersG_new_null_const, cv_PtrLcv_aruco_EstimateParametersG_delete, cv_PtrLcv_aruco_EstimateParametersG_getInnerPtr_const, cv_PtrLcv_aruco_EstimateParametersG_getInnerPtrMut
}

ptr_extern_ctor! { crate::aruco::EstimateParameters, cv_PtrLcv_aruco_EstimateParametersG_new_const_EstimateParameters }
impl core::Ptr<crate::aruco::EstimateParameters> {
	#[inline] pub fn as_raw_PtrOfEstimateParameters(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfEstimateParameters(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::aruco::EstimateParametersTraitConst for core::Ptr<crate::aruco::EstimateParameters> {
	#[inline] fn as_raw_EstimateParameters(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::aruco::EstimateParametersTrait for core::Ptr<crate::aruco::EstimateParameters> {
	#[inline] fn as_raw_mut_EstimateParameters(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::aruco::EstimateParameters> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfEstimateParameters")
			.field("pattern", &crate::aruco::EstimateParametersTraitConst::pattern(self))
			.field("use_extrinsic_guess", &crate::aruco::EstimateParametersTraitConst::use_extrinsic_guess(self))
			.field("solve_pnp_method", &crate::aruco::EstimateParametersTraitConst::solve_pnp_method(self))
			.finish()
	}
}

