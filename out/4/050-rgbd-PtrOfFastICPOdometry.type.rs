ptr_extern! { crate::rgbd::FastICPOdometry,
	cv_PtrLcv_rgbd_FastICPOdometryG_new_null_const, cv_PtrLcv_rgbd_FastICPOdometryG_delete, cv_PtrLcv_rgbd_FastICPOdometryG_getInnerPtr_const, cv_PtrLcv_rgbd_FastICPOdometryG_getInnerPtrMut
}

ptr_extern_ctor! { crate::rgbd::FastICPOdometry, cv_PtrLcv_rgbd_FastICPOdometryG_new_const_FastICPOdometry }
impl core::Ptr<crate::rgbd::FastICPOdometry> {
	#[inline] pub fn as_raw_PtrOfFastICPOdometry(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFastICPOdometry(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rgbd::FastICPOdometryTraitConst for core::Ptr<crate::rgbd::FastICPOdometry> {
	#[inline] fn as_raw_FastICPOdometry(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::FastICPOdometryTrait for core::Ptr<crate::rgbd::FastICPOdometry> {
	#[inline] fn as_raw_mut_FastICPOdometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::rgbd::FastICPOdometry> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::rgbd::FastICPOdometry> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::rgbd::FastICPOdometry>, core::Ptr<core::Algorithm>, cv_PtrLcv_rgbd_FastICPOdometryG_to_PtrOfAlgorithm }

impl crate::rgbd::OdometryTraitConst for core::Ptr<crate::rgbd::FastICPOdometry> {
	#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::OdometryTrait for core::Ptr<crate::rgbd::FastICPOdometry> {
	#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::rgbd::FastICPOdometry>, core::Ptr<crate::rgbd::Odometry>, cv_PtrLcv_rgbd_FastICPOdometryG_to_PtrOfOdometry }

impl std::fmt::Debug for core::Ptr<crate::rgbd::FastICPOdometry> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfFastICPOdometry")
			.finish()
	}
}

