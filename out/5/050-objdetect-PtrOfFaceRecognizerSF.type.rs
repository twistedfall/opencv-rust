ptr_extern! { crate::objdetect::FaceRecognizerSF,
	cv_PtrLcv_FaceRecognizerSFG_new_null_const, cv_PtrLcv_FaceRecognizerSFG_delete, cv_PtrLcv_FaceRecognizerSFG_getInnerPtr_const, cv_PtrLcv_FaceRecognizerSFG_getInnerPtrMut
}

impl core::Ptr<crate::objdetect::FaceRecognizerSF> {
	#[inline] pub fn as_raw_PtrOfFaceRecognizerSF(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFaceRecognizerSF(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::objdetect::FaceRecognizerSFTraitConst for core::Ptr<crate::objdetect::FaceRecognizerSF> {
	#[inline] fn as_raw_FaceRecognizerSF(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::objdetect::FaceRecognizerSFTrait for core::Ptr<crate::objdetect::FaceRecognizerSF> {
	#[inline] fn as_raw_mut_FaceRecognizerSF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::objdetect::FaceRecognizerSF> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfFaceRecognizerSF")
			.finish()
	}
}

