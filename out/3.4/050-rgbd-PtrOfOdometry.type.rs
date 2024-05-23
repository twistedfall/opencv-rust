ptr_extern! { crate::rgbd::Odometry,
	cv_PtrLcv_rgbd_OdometryG_new_null_const, cv_PtrLcv_rgbd_OdometryG_delete, cv_PtrLcv_rgbd_OdometryG_getInnerPtr_const, cv_PtrLcv_rgbd_OdometryG_getInnerPtrMut
}

impl core::Ptr<crate::rgbd::Odometry> {
	#[inline] pub fn as_raw_PtrOfOdometry(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfOdometry(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rgbd::OdometryTraitConst for core::Ptr<crate::rgbd::Odometry> {
	#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::OdometryTrait for core::Ptr<crate::rgbd::Odometry> {
	#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::rgbd::Odometry> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::rgbd::Odometry> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::rgbd::Odometry>, core::Ptr<core::Algorithm>, cv_PtrLcv_rgbd_OdometryG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::rgbd::Odometry> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfOdometry")
			.finish()
	}
}

