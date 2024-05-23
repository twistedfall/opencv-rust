ptr_extern! { crate::rgbd::Kinfu_Detail_PoseGraph,
	cv_PtrLcv_kinfu_detail_PoseGraphG_new_null_const, cv_PtrLcv_kinfu_detail_PoseGraphG_delete, cv_PtrLcv_kinfu_detail_PoseGraphG_getInnerPtr_const, cv_PtrLcv_kinfu_detail_PoseGraphG_getInnerPtrMut
}

impl core::Ptr<crate::rgbd::Kinfu_Detail_PoseGraph> {
	#[inline] pub fn as_raw_PtrOfKinfu_Detail_PoseGraph(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfKinfu_Detail_PoseGraph(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rgbd::Kinfu_Detail_PoseGraphTraitConst for core::Ptr<crate::rgbd::Kinfu_Detail_PoseGraph> {
	#[inline] fn as_raw_Kinfu_Detail_PoseGraph(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::Kinfu_Detail_PoseGraphTrait for core::Ptr<crate::rgbd::Kinfu_Detail_PoseGraph> {
	#[inline] fn as_raw_mut_Kinfu_Detail_PoseGraph(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::rgbd::Kinfu_Detail_PoseGraph> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfKinfu_Detail_PoseGraph")
			.finish()
	}
}

