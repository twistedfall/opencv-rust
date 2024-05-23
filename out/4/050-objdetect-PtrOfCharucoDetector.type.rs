ptr_extern! { crate::objdetect::CharucoDetector,
	cv_PtrLcv_aruco_CharucoDetectorG_new_null_const, cv_PtrLcv_aruco_CharucoDetectorG_delete, cv_PtrLcv_aruco_CharucoDetectorG_getInnerPtr_const, cv_PtrLcv_aruco_CharucoDetectorG_getInnerPtrMut
}

ptr_extern_ctor! { crate::objdetect::CharucoDetector, cv_PtrLcv_aruco_CharucoDetectorG_new_const_CharucoDetector }
impl core::Ptr<crate::objdetect::CharucoDetector> {
	#[inline] pub fn as_raw_PtrOfCharucoDetector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCharucoDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::objdetect::CharucoDetectorTraitConst for core::Ptr<crate::objdetect::CharucoDetector> {
	#[inline] fn as_raw_CharucoDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::objdetect::CharucoDetectorTrait for core::Ptr<crate::objdetect::CharucoDetector> {
	#[inline] fn as_raw_mut_CharucoDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::objdetect::CharucoDetector> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::objdetect::CharucoDetector> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::objdetect::CharucoDetector>, core::Ptr<core::Algorithm>, cv_PtrLcv_aruco_CharucoDetectorG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::objdetect::CharucoDetector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCharucoDetector")
			.finish()
	}
}

