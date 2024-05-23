ptr_extern! { crate::rgbd::RgbdOdometry,
	cv_PtrLcv_rgbd_RgbdOdometryG_new_null_const, cv_PtrLcv_rgbd_RgbdOdometryG_delete, cv_PtrLcv_rgbd_RgbdOdometryG_getInnerPtr_const, cv_PtrLcv_rgbd_RgbdOdometryG_getInnerPtrMut
}

ptr_extern_ctor! { crate::rgbd::RgbdOdometry, cv_PtrLcv_rgbd_RgbdOdometryG_new_const_RgbdOdometry }
impl core::Ptr<crate::rgbd::RgbdOdometry> {
	#[inline] pub fn as_raw_PtrOfRgbdOdometry(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRgbdOdometry(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rgbd::RgbdOdometryTraitConst for core::Ptr<crate::rgbd::RgbdOdometry> {
	#[inline] fn as_raw_RgbdOdometry(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::RgbdOdometryTrait for core::Ptr<crate::rgbd::RgbdOdometry> {
	#[inline] fn as_raw_mut_RgbdOdometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::rgbd::RgbdOdometry> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::rgbd::RgbdOdometry> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::rgbd::RgbdOdometry>, core::Ptr<core::Algorithm>, cv_PtrLcv_rgbd_RgbdOdometryG_to_PtrOfAlgorithm }

impl crate::rgbd::OdometryTraitConst for core::Ptr<crate::rgbd::RgbdOdometry> {
	#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::OdometryTrait for core::Ptr<crate::rgbd::RgbdOdometry> {
	#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::rgbd::RgbdOdometry>, core::Ptr<crate::rgbd::Odometry>, cv_PtrLcv_rgbd_RgbdOdometryG_to_PtrOfOdometry }

impl std::fmt::Debug for core::Ptr<crate::rgbd::RgbdOdometry> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfRgbdOdometry")
			.finish()
	}
}

