ptr_extern! { crate::face::FacemarkAAM,
	cv_PtrLcv_face_FacemarkAAMG_new_null_const, cv_PtrLcv_face_FacemarkAAMG_delete, cv_PtrLcv_face_FacemarkAAMG_getInnerPtr_const, cv_PtrLcv_face_FacemarkAAMG_getInnerPtrMut
}

impl core::Ptr<crate::face::FacemarkAAM> {
	#[inline] pub fn as_raw_PtrOfFacemarkAAM(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFacemarkAAM(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::face::FacemarkAAMTraitConst for core::Ptr<crate::face::FacemarkAAM> {
	#[inline] fn as_raw_FacemarkAAM(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::FacemarkAAMTrait for core::Ptr<crate::face::FacemarkAAM> {
	#[inline] fn as_raw_mut_FacemarkAAM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::face::FacemarkAAM> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::face::FacemarkAAM> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::face::FacemarkAAM>, core::Ptr<core::Algorithm>, cv_PtrLcv_face_FacemarkAAMG_to_PtrOfAlgorithm }

impl crate::face::FacemarkTraitConst for core::Ptr<crate::face::FacemarkAAM> {
	#[inline] fn as_raw_Facemark(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::FacemarkTrait for core::Ptr<crate::face::FacemarkAAM> {
	#[inline] fn as_raw_mut_Facemark(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::face::FacemarkAAM>, core::Ptr<crate::face::Facemark>, cv_PtrLcv_face_FacemarkAAMG_to_PtrOfFacemark }

impl crate::face::FacemarkTrainTraitConst for core::Ptr<crate::face::FacemarkAAM> {
	#[inline] fn as_raw_FacemarkTrain(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::FacemarkTrainTrait for core::Ptr<crate::face::FacemarkAAM> {
	#[inline] fn as_raw_mut_FacemarkTrain(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::face::FacemarkAAM>, core::Ptr<crate::face::FacemarkTrain>, cv_PtrLcv_face_FacemarkAAMG_to_PtrOfFacemarkTrain }

impl std::fmt::Debug for core::Ptr<crate::face::FacemarkAAM> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfFacemarkAAM")
			.finish()
	}
}

