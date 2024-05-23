ptr_extern! { crate::rgbd::ICPOdometry,
	cv_PtrLcv_rgbd_ICPOdometryG_new_null_const, cv_PtrLcv_rgbd_ICPOdometryG_delete, cv_PtrLcv_rgbd_ICPOdometryG_getInnerPtr_const, cv_PtrLcv_rgbd_ICPOdometryG_getInnerPtrMut
}

ptr_extern_ctor! { crate::rgbd::ICPOdometry, cv_PtrLcv_rgbd_ICPOdometryG_new_const_ICPOdometry }
impl core::Ptr<crate::rgbd::ICPOdometry> {
	#[inline] pub fn as_raw_PtrOfICPOdometry(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfICPOdometry(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rgbd::ICPOdometryTraitConst for core::Ptr<crate::rgbd::ICPOdometry> {
	#[inline] fn as_raw_ICPOdometry(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::ICPOdometryTrait for core::Ptr<crate::rgbd::ICPOdometry> {
	#[inline] fn as_raw_mut_ICPOdometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::rgbd::ICPOdometry> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::rgbd::ICPOdometry> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::rgbd::ICPOdometry>, core::Ptr<core::Algorithm>, cv_PtrLcv_rgbd_ICPOdometryG_to_PtrOfAlgorithm }

impl crate::rgbd::OdometryTraitConst for core::Ptr<crate::rgbd::ICPOdometry> {
	#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::OdometryTrait for core::Ptr<crate::rgbd::ICPOdometry> {
	#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::rgbd::ICPOdometry>, core::Ptr<crate::rgbd::Odometry>, cv_PtrLcv_rgbd_ICPOdometryG_to_PtrOfOdometry }

impl std::fmt::Debug for core::Ptr<crate::rgbd::ICPOdometry> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfICPOdometry")
			.finish()
	}
}

