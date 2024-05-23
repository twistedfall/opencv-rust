ptr_extern! { crate::objdetect::ArucoDetector,
	cv_PtrLcv_aruco_ArucoDetectorG_new_null_const, cv_PtrLcv_aruco_ArucoDetectorG_delete, cv_PtrLcv_aruco_ArucoDetectorG_getInnerPtr_const, cv_PtrLcv_aruco_ArucoDetectorG_getInnerPtrMut
}

ptr_extern_ctor! { crate::objdetect::ArucoDetector, cv_PtrLcv_aruco_ArucoDetectorG_new_const_ArucoDetector }
impl core::Ptr<crate::objdetect::ArucoDetector> {
	#[inline] pub fn as_raw_PtrOfArucoDetector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfArucoDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::objdetect::ArucoDetectorTraitConst for core::Ptr<crate::objdetect::ArucoDetector> {
	#[inline] fn as_raw_ArucoDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::objdetect::ArucoDetectorTrait for core::Ptr<crate::objdetect::ArucoDetector> {
	#[inline] fn as_raw_mut_ArucoDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::objdetect::ArucoDetector> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::objdetect::ArucoDetector> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::objdetect::ArucoDetector>, core::Ptr<core::Algorithm>, cv_PtrLcv_aruco_ArucoDetectorG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::objdetect::ArucoDetector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfArucoDetector")
			.finish()
	}
}

