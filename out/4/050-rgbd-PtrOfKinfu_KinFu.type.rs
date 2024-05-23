ptr_extern! { crate::rgbd::Kinfu_KinFu,
	cv_PtrLcv_kinfu_KinFuG_new_null_const, cv_PtrLcv_kinfu_KinFuG_delete, cv_PtrLcv_kinfu_KinFuG_getInnerPtr_const, cv_PtrLcv_kinfu_KinFuG_getInnerPtrMut
}

impl core::Ptr<crate::rgbd::Kinfu_KinFu> {
	#[inline] pub fn as_raw_PtrOfKinfu_KinFu(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfKinfu_KinFu(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rgbd::Kinfu_KinFuTraitConst for core::Ptr<crate::rgbd::Kinfu_KinFu> {
	#[inline] fn as_raw_Kinfu_KinFu(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::Kinfu_KinFuTrait for core::Ptr<crate::rgbd::Kinfu_KinFu> {
	#[inline] fn as_raw_mut_Kinfu_KinFu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::rgbd::Kinfu_KinFu> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfKinfu_KinFu")
			.finish()
	}
}

