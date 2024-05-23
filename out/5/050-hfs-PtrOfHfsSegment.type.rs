ptr_extern! { crate::hfs::HfsSegment,
	cv_PtrLcv_hfs_HfsSegmentG_new_null_const, cv_PtrLcv_hfs_HfsSegmentG_delete, cv_PtrLcv_hfs_HfsSegmentG_getInnerPtr_const, cv_PtrLcv_hfs_HfsSegmentG_getInnerPtrMut
}

impl core::Ptr<crate::hfs::HfsSegment> {
	#[inline] pub fn as_raw_PtrOfHfsSegment(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfHfsSegment(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::hfs::HfsSegmentTraitConst for core::Ptr<crate::hfs::HfsSegment> {
	#[inline] fn as_raw_HfsSegment(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::hfs::HfsSegmentTrait for core::Ptr<crate::hfs::HfsSegment> {
	#[inline] fn as_raw_mut_HfsSegment(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::hfs::HfsSegment> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::hfs::HfsSegment> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::hfs::HfsSegment>, core::Ptr<core::Algorithm>, cv_PtrLcv_hfs_HfsSegmentG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::hfs::HfsSegment> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfHfsSegment")
			.finish()
	}
}

