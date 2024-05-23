ptr_extern! { crate::ximgproc::ScanSegment,
	cv_PtrLcv_ximgproc_ScanSegmentG_new_null_const, cv_PtrLcv_ximgproc_ScanSegmentG_delete, cv_PtrLcv_ximgproc_ScanSegmentG_getInnerPtr_const, cv_PtrLcv_ximgproc_ScanSegmentG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::ScanSegment> {
	#[inline] pub fn as_raw_PtrOfScanSegment(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfScanSegment(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::ScanSegmentTraitConst for core::Ptr<crate::ximgproc::ScanSegment> {
	#[inline] fn as_raw_ScanSegment(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::ScanSegmentTrait for core::Ptr<crate::ximgproc::ScanSegment> {
	#[inline] fn as_raw_mut_ScanSegment(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::ScanSegment> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::ScanSegment> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::ScanSegment>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_ScanSegmentG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::ScanSegment> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfScanSegment")
			.finish()
	}
}

