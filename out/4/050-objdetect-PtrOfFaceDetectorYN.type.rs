ptr_extern! { crate::objdetect::FaceDetectorYN,
	cv_PtrLcv_FaceDetectorYNG_new_null_const, cv_PtrLcv_FaceDetectorYNG_delete, cv_PtrLcv_FaceDetectorYNG_getInnerPtr_const, cv_PtrLcv_FaceDetectorYNG_getInnerPtrMut
}

impl core::Ptr<crate::objdetect::FaceDetectorYN> {
	#[inline] pub fn as_raw_PtrOfFaceDetectorYN(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFaceDetectorYN(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::objdetect::FaceDetectorYNTraitConst for core::Ptr<crate::objdetect::FaceDetectorYN> {
	#[inline] fn as_raw_FaceDetectorYN(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::objdetect::FaceDetectorYNTrait for core::Ptr<crate::objdetect::FaceDetectorYN> {
	#[inline] fn as_raw_mut_FaceDetectorYN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::objdetect::FaceDetectorYN> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfFaceDetectorYN")
			.finish()
	}
}

