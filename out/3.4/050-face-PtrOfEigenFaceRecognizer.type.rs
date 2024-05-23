ptr_extern! { crate::face::EigenFaceRecognizer,
	cv_PtrLcv_face_EigenFaceRecognizerG_new_null_const, cv_PtrLcv_face_EigenFaceRecognizerG_delete, cv_PtrLcv_face_EigenFaceRecognizerG_getInnerPtr_const, cv_PtrLcv_face_EigenFaceRecognizerG_getInnerPtrMut
}

impl core::Ptr<crate::face::EigenFaceRecognizer> {
	#[inline] pub fn as_raw_PtrOfEigenFaceRecognizer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfEigenFaceRecognizer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::face::EigenFaceRecognizerTraitConst for core::Ptr<crate::face::EigenFaceRecognizer> {
	#[inline] fn as_raw_EigenFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::EigenFaceRecognizerTrait for core::Ptr<crate::face::EigenFaceRecognizer> {
	#[inline] fn as_raw_mut_EigenFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::face::EigenFaceRecognizer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::face::EigenFaceRecognizer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::face::EigenFaceRecognizer>, core::Ptr<core::Algorithm>, cv_PtrLcv_face_EigenFaceRecognizerG_to_PtrOfAlgorithm }

impl crate::face::BasicFaceRecognizerTraitConst for core::Ptr<crate::face::EigenFaceRecognizer> {
	#[inline] fn as_raw_BasicFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::BasicFaceRecognizerTrait for core::Ptr<crate::face::EigenFaceRecognizer> {
	#[inline] fn as_raw_mut_BasicFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::face::EigenFaceRecognizer>, core::Ptr<crate::face::BasicFaceRecognizer>, cv_PtrLcv_face_EigenFaceRecognizerG_to_PtrOfBasicFaceRecognizer }

impl crate::face::FaceRecognizerTraitConst for core::Ptr<crate::face::EigenFaceRecognizer> {
	#[inline] fn as_raw_FaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::FaceRecognizerTrait for core::Ptr<crate::face::EigenFaceRecognizer> {
	#[inline] fn as_raw_mut_FaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::face::EigenFaceRecognizer>, core::Ptr<crate::face::FaceRecognizer>, cv_PtrLcv_face_EigenFaceRecognizerG_to_PtrOfFaceRecognizer }

impl std::fmt::Debug for core::Ptr<crate::face::EigenFaceRecognizer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfEigenFaceRecognizer")
			.finish()
	}
}

