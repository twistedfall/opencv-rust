ptr_extern! { crate::calib3d::StereoMatcher,
	cv_PtrLcv_StereoMatcherG_new_null_const, cv_PtrLcv_StereoMatcherG_delete, cv_PtrLcv_StereoMatcherG_getInnerPtr_const, cv_PtrLcv_StereoMatcherG_getInnerPtrMut
}

impl core::Ptr<crate::calib3d::StereoMatcher> {
	#[inline] pub fn as_raw_PtrOfStereoMatcher(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfStereoMatcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::calib3d::StereoMatcherTraitConst for core::Ptr<crate::calib3d::StereoMatcher> {
	#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::calib3d::StereoMatcherTrait for core::Ptr<crate::calib3d::StereoMatcher> {
	#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::calib3d::StereoMatcher> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::calib3d::StereoMatcher> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::calib3d::StereoMatcher>, core::Ptr<core::Algorithm>, cv_PtrLcv_StereoMatcherG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::calib3d::StereoMatcher> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfStereoMatcher")
			.finish()
	}
}

