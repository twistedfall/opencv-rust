ptr_extern! { crate::rgbd::DepthCleaner,
	cv_PtrLcv_rgbd_DepthCleanerG_new_null_const, cv_PtrLcv_rgbd_DepthCleanerG_delete, cv_PtrLcv_rgbd_DepthCleanerG_getInnerPtr_const, cv_PtrLcv_rgbd_DepthCleanerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::rgbd::DepthCleaner, cv_PtrLcv_rgbd_DepthCleanerG_new_const_DepthCleaner }
impl core::Ptr<crate::rgbd::DepthCleaner> {
	#[inline] pub fn as_raw_PtrOfDepthCleaner(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDepthCleaner(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rgbd::DepthCleanerTraitConst for core::Ptr<crate::rgbd::DepthCleaner> {
	#[inline] fn as_raw_DepthCleaner(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::DepthCleanerTrait for core::Ptr<crate::rgbd::DepthCleaner> {
	#[inline] fn as_raw_mut_DepthCleaner(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::rgbd::DepthCleaner> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::rgbd::DepthCleaner> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::rgbd::DepthCleaner>, core::Ptr<core::Algorithm>, cv_PtrLcv_rgbd_DepthCleanerG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::rgbd::DepthCleaner> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDepthCleaner")
			.finish()
	}
}

