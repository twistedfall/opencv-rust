ptr_extern! { crate::face::LBPHFaceRecognizer,
	cv_PtrLcv_face_LBPHFaceRecognizerG_new_null_const, cv_PtrLcv_face_LBPHFaceRecognizerG_delete, cv_PtrLcv_face_LBPHFaceRecognizerG_getInnerPtr_const, cv_PtrLcv_face_LBPHFaceRecognizerG_getInnerPtrMut
}

impl core::Ptr<crate::face::LBPHFaceRecognizer> {
	#[inline] pub fn as_raw_PtrOfLBPHFaceRecognizer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLBPHFaceRecognizer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::face::LBPHFaceRecognizerTraitConst for core::Ptr<crate::face::LBPHFaceRecognizer> {
	#[inline] fn as_raw_LBPHFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::LBPHFaceRecognizerTrait for core::Ptr<crate::face::LBPHFaceRecognizer> {
	#[inline] fn as_raw_mut_LBPHFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::face::LBPHFaceRecognizer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::face::LBPHFaceRecognizer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::face::LBPHFaceRecognizer>, core::Ptr<core::Algorithm>, cv_PtrLcv_face_LBPHFaceRecognizerG_to_PtrOfAlgorithm }

impl crate::face::FaceRecognizerTraitConst for core::Ptr<crate::face::LBPHFaceRecognizer> {
	#[inline] fn as_raw_FaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::FaceRecognizerTrait for core::Ptr<crate::face::LBPHFaceRecognizer> {
	#[inline] fn as_raw_mut_FaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::face::LBPHFaceRecognizer>, core::Ptr<crate::face::FaceRecognizer>, cv_PtrLcv_face_LBPHFaceRecognizerG_to_PtrOfFaceRecognizer }

impl std::fmt::Debug for core::Ptr<crate::face::LBPHFaceRecognizer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfLBPHFaceRecognizer")
			.finish()
	}
}

