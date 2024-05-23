ptr_extern! { crate::calib3d::StereoBM,
	cv_PtrLcv_StereoBMG_new_null_const, cv_PtrLcv_StereoBMG_delete, cv_PtrLcv_StereoBMG_getInnerPtr_const, cv_PtrLcv_StereoBMG_getInnerPtrMut
}

impl core::Ptr<crate::calib3d::StereoBM> {
	#[inline] pub fn as_raw_PtrOfStereoBM(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfStereoBM(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::calib3d::StereoBMTraitConst for core::Ptr<crate::calib3d::StereoBM> {
	#[inline] fn as_raw_StereoBM(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::calib3d::StereoBMTrait for core::Ptr<crate::calib3d::StereoBM> {
	#[inline] fn as_raw_mut_StereoBM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::calib3d::StereoBM> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::calib3d::StereoBM> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::calib3d::StereoBM>, core::Ptr<core::Algorithm>, cv_PtrLcv_StereoBMG_to_PtrOfAlgorithm }

impl crate::calib3d::StereoMatcherTraitConst for core::Ptr<crate::calib3d::StereoBM> {
	#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::calib3d::StereoMatcherTrait for core::Ptr<crate::calib3d::StereoBM> {
	#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::calib3d::StereoBM>, core::Ptr<crate::calib3d::StereoMatcher>, cv_PtrLcv_StereoBMG_to_PtrOfStereoMatcher }

impl std::fmt::Debug for core::Ptr<crate::calib3d::StereoBM> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfStereoBM")
			.finish()
	}
}

