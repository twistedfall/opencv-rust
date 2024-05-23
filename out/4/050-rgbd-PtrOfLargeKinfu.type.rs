ptr_extern! { crate::rgbd::LargeKinfu,
	cv_PtrLcv_large_kinfu_LargeKinfuG_new_null_const, cv_PtrLcv_large_kinfu_LargeKinfuG_delete, cv_PtrLcv_large_kinfu_LargeKinfuG_getInnerPtr_const, cv_PtrLcv_large_kinfu_LargeKinfuG_getInnerPtrMut
}

impl core::Ptr<crate::rgbd::LargeKinfu> {
	#[inline] pub fn as_raw_PtrOfLargeKinfu(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLargeKinfu(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rgbd::LargeKinfuTraitConst for core::Ptr<crate::rgbd::LargeKinfu> {
	#[inline] fn as_raw_LargeKinfu(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::LargeKinfuTrait for core::Ptr<crate::rgbd::LargeKinfu> {
	#[inline] fn as_raw_mut_LargeKinfu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::rgbd::LargeKinfu> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfLargeKinfu")
			.finish()
	}
}

