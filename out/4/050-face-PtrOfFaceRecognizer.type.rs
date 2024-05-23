ptr_extern! { crate::face::FaceRecognizer,
	cv_PtrLcv_face_FaceRecognizerG_new_null_const, cv_PtrLcv_face_FaceRecognizerG_delete, cv_PtrLcv_face_FaceRecognizerG_getInnerPtr_const, cv_PtrLcv_face_FaceRecognizerG_getInnerPtrMut
}

impl core::Ptr<crate::face::FaceRecognizer> {
	#[inline] pub fn as_raw_PtrOfFaceRecognizer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFaceRecognizer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::face::FaceRecognizerTraitConst for core::Ptr<crate::face::FaceRecognizer> {
	#[inline] fn as_raw_FaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::FaceRecognizerTrait for core::Ptr<crate::face::FaceRecognizer> {
	#[inline] fn as_raw_mut_FaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::face::FaceRecognizer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::face::FaceRecognizer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::face::FaceRecognizer>, core::Ptr<core::Algorithm>, cv_PtrLcv_face_FaceRecognizerG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::face::FaceRecognizer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfFaceRecognizer")
			.finish()
	}
}

