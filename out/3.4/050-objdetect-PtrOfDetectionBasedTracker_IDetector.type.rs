ptr_extern! { crate::objdetect::DetectionBasedTracker_IDetector,
	cv_PtrLcv_DetectionBasedTracker_IDetectorG_new_null_const, cv_PtrLcv_DetectionBasedTracker_IDetectorG_delete, cv_PtrLcv_DetectionBasedTracker_IDetectorG_getInnerPtr_const, cv_PtrLcv_DetectionBasedTracker_IDetectorG_getInnerPtrMut
}

impl core::Ptr<crate::objdetect::DetectionBasedTracker_IDetector> {
	#[inline] pub fn as_raw_PtrOfDetectionBasedTracker_IDetector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetectionBasedTracker_IDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::objdetect::DetectionBasedTracker_IDetectorTraitConst for core::Ptr<crate::objdetect::DetectionBasedTracker_IDetector> {
	#[inline] fn as_raw_DetectionBasedTracker_IDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::objdetect::DetectionBasedTracker_IDetectorTrait for core::Ptr<crate::objdetect::DetectionBasedTracker_IDetector> {
	#[inline] fn as_raw_mut_DetectionBasedTracker_IDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::objdetect::DetectionBasedTracker_IDetector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetectionBasedTracker_IDetector")
			.finish()
	}
}

