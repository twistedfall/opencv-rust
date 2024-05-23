ptr_extern! { crate::face::FacemarkTrain,
	cv_PtrLcv_face_FacemarkTrainG_new_null_const, cv_PtrLcv_face_FacemarkTrainG_delete, cv_PtrLcv_face_FacemarkTrainG_getInnerPtr_const, cv_PtrLcv_face_FacemarkTrainG_getInnerPtrMut
}

impl core::Ptr<crate::face::FacemarkTrain> {
	#[inline] pub fn as_raw_PtrOfFacemarkTrain(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFacemarkTrain(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::face::FacemarkTrainTraitConst for core::Ptr<crate::face::FacemarkTrain> {
	#[inline] fn as_raw_FacemarkTrain(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::FacemarkTrainTrait for core::Ptr<crate::face::FacemarkTrain> {
	#[inline] fn as_raw_mut_FacemarkTrain(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::face::FacemarkTrain> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::face::FacemarkTrain> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::face::FacemarkTrain>, core::Ptr<core::Algorithm>, cv_PtrLcv_face_FacemarkTrainG_to_PtrOfAlgorithm }

impl crate::face::FacemarkTraitConst for core::Ptr<crate::face::FacemarkTrain> {
	#[inline] fn as_raw_Facemark(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::FacemarkTrait for core::Ptr<crate::face::FacemarkTrain> {
	#[inline] fn as_raw_mut_Facemark(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::face::FacemarkTrain>, core::Ptr<crate::face::Facemark>, cv_PtrLcv_face_FacemarkTrainG_to_PtrOfFacemark }

impl std::fmt::Debug for core::Ptr<crate::face::FacemarkTrain> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfFacemarkTrain")
			.finish()
	}
}

