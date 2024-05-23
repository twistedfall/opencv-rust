ptr_extern! { crate::rgbd::RgbdPlane,
	cv_PtrLcv_rgbd_RgbdPlaneG_new_null_const, cv_PtrLcv_rgbd_RgbdPlaneG_delete, cv_PtrLcv_rgbd_RgbdPlaneG_getInnerPtr_const, cv_PtrLcv_rgbd_RgbdPlaneG_getInnerPtrMut
}

ptr_extern_ctor! { crate::rgbd::RgbdPlane, cv_PtrLcv_rgbd_RgbdPlaneG_new_const_RgbdPlane }
impl core::Ptr<crate::rgbd::RgbdPlane> {
	#[inline] pub fn as_raw_PtrOfRgbdPlane(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRgbdPlane(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rgbd::RgbdPlaneTraitConst for core::Ptr<crate::rgbd::RgbdPlane> {
	#[inline] fn as_raw_RgbdPlane(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::RgbdPlaneTrait for core::Ptr<crate::rgbd::RgbdPlane> {
	#[inline] fn as_raw_mut_RgbdPlane(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::rgbd::RgbdPlane> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::rgbd::RgbdPlane> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::rgbd::RgbdPlane>, core::Ptr<core::Algorithm>, cv_PtrLcv_rgbd_RgbdPlaneG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::rgbd::RgbdPlane> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfRgbdPlane")
			.finish()
	}
}

