ptr_extern! { crate::rgbd::RgbdICPOdometry,
	cv_PtrLcv_rgbd_RgbdICPOdometryG_new_null_const, cv_PtrLcv_rgbd_RgbdICPOdometryG_delete, cv_PtrLcv_rgbd_RgbdICPOdometryG_getInnerPtr_const, cv_PtrLcv_rgbd_RgbdICPOdometryG_getInnerPtrMut
}

ptr_extern_ctor! { crate::rgbd::RgbdICPOdometry, cv_PtrLcv_rgbd_RgbdICPOdometryG_new_const_RgbdICPOdometry }
impl core::Ptr<crate::rgbd::RgbdICPOdometry> {
	#[inline] pub fn as_raw_PtrOfRgbdICPOdometry(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRgbdICPOdometry(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rgbd::RgbdICPOdometryTraitConst for core::Ptr<crate::rgbd::RgbdICPOdometry> {
	#[inline] fn as_raw_RgbdICPOdometry(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::RgbdICPOdometryTrait for core::Ptr<crate::rgbd::RgbdICPOdometry> {
	#[inline] fn as_raw_mut_RgbdICPOdometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::rgbd::RgbdICPOdometry> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::rgbd::RgbdICPOdometry> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::rgbd::RgbdICPOdometry>, core::Ptr<core::Algorithm>, cv_PtrLcv_rgbd_RgbdICPOdometryG_to_PtrOfAlgorithm }

impl crate::rgbd::OdometryTraitConst for core::Ptr<crate::rgbd::RgbdICPOdometry> {
	#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::OdometryTrait for core::Ptr<crate::rgbd::RgbdICPOdometry> {
	#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::rgbd::RgbdICPOdometry>, core::Ptr<crate::rgbd::Odometry>, cv_PtrLcv_rgbd_RgbdICPOdometryG_to_PtrOfOdometry }

impl std::fmt::Debug for core::Ptr<crate::rgbd::RgbdICPOdometry> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfRgbdICPOdometry")
			.finish()
	}
}

