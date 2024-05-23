ptr_extern! { crate::imgproc::LineSegmentDetector,
	cv_PtrLcv_LineSegmentDetectorG_new_null_const, cv_PtrLcv_LineSegmentDetectorG_delete, cv_PtrLcv_LineSegmentDetectorG_getInnerPtr_const, cv_PtrLcv_LineSegmentDetectorG_getInnerPtrMut
}

impl core::Ptr<crate::imgproc::LineSegmentDetector> {
	#[inline] pub fn as_raw_PtrOfLineSegmentDetector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLineSegmentDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::imgproc::LineSegmentDetectorTraitConst for core::Ptr<crate::imgproc::LineSegmentDetector> {
	#[inline] fn as_raw_LineSegmentDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::imgproc::LineSegmentDetectorTrait for core::Ptr<crate::imgproc::LineSegmentDetector> {
	#[inline] fn as_raw_mut_LineSegmentDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::imgproc::LineSegmentDetector> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::imgproc::LineSegmentDetector> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::imgproc::LineSegmentDetector>, core::Ptr<core::Algorithm>, cv_PtrLcv_LineSegmentDetectorG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::imgproc::LineSegmentDetector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfLineSegmentDetector")
			.finish()
	}
}

