ptr_extern! { crate::face::FisherFaceRecognizer,
	cv_PtrLcv_face_FisherFaceRecognizerG_new_null_const, cv_PtrLcv_face_FisherFaceRecognizerG_delete, cv_PtrLcv_face_FisherFaceRecognizerG_getInnerPtr_const, cv_PtrLcv_face_FisherFaceRecognizerG_getInnerPtrMut
}

impl core::Ptr<crate::face::FisherFaceRecognizer> {
	#[inline] pub fn as_raw_PtrOfFisherFaceRecognizer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFisherFaceRecognizer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::face::FisherFaceRecognizerTraitConst for core::Ptr<crate::face::FisherFaceRecognizer> {
	#[inline] fn as_raw_FisherFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::FisherFaceRecognizerTrait for core::Ptr<crate::face::FisherFaceRecognizer> {
	#[inline] fn as_raw_mut_FisherFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::face::FisherFaceRecognizer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::face::FisherFaceRecognizer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::face::FisherFaceRecognizer>, core::Ptr<core::Algorithm>, cv_PtrLcv_face_FisherFaceRecognizerG_to_PtrOfAlgorithm }

impl crate::face::BasicFaceRecognizerTraitConst for core::Ptr<crate::face::FisherFaceRecognizer> {
	#[inline] fn as_raw_BasicFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::BasicFaceRecognizerTrait for core::Ptr<crate::face::FisherFaceRecognizer> {
	#[inline] fn as_raw_mut_BasicFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::face::FisherFaceRecognizer>, core::Ptr<crate::face::BasicFaceRecognizer>, cv_PtrLcv_face_FisherFaceRecognizerG_to_PtrOfBasicFaceRecognizer }

impl crate::face::FaceRecognizerTraitConst for core::Ptr<crate::face::FisherFaceRecognizer> {
	#[inline] fn as_raw_FaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::FaceRecognizerTrait for core::Ptr<crate::face::FisherFaceRecognizer> {
	#[inline] fn as_raw_mut_FaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::face::FisherFaceRecognizer>, core::Ptr<crate::face::FaceRecognizer>, cv_PtrLcv_face_FisherFaceRecognizerG_to_PtrOfFaceRecognizer }

impl std::fmt::Debug for core::Ptr<crate::face::FisherFaceRecognizer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfFisherFaceRecognizer")
			.finish()
	}
}

