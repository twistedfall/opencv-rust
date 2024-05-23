ptr_extern! { crate::face::FacemarkKazemi,
	cv_PtrLcv_face_FacemarkKazemiG_new_null_const, cv_PtrLcv_face_FacemarkKazemiG_delete, cv_PtrLcv_face_FacemarkKazemiG_getInnerPtr_const, cv_PtrLcv_face_FacemarkKazemiG_getInnerPtrMut
}

impl core::Ptr<crate::face::FacemarkKazemi> {
	#[inline] pub fn as_raw_PtrOfFacemarkKazemi(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFacemarkKazemi(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::face::FacemarkKazemiTraitConst for core::Ptr<crate::face::FacemarkKazemi> {
	#[inline] fn as_raw_FacemarkKazemi(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::FacemarkKazemiTrait for core::Ptr<crate::face::FacemarkKazemi> {
	#[inline] fn as_raw_mut_FacemarkKazemi(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::face::FacemarkKazemi> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::face::FacemarkKazemi> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::face::FacemarkKazemi>, core::Ptr<core::Algorithm>, cv_PtrLcv_face_FacemarkKazemiG_to_PtrOfAlgorithm }

impl crate::face::FacemarkTraitConst for core::Ptr<crate::face::FacemarkKazemi> {
	#[inline] fn as_raw_Facemark(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::FacemarkTrait for core::Ptr<crate::face::FacemarkKazemi> {
	#[inline] fn as_raw_mut_Facemark(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::face::FacemarkKazemi>, core::Ptr<crate::face::Facemark>, cv_PtrLcv_face_FacemarkKazemiG_to_PtrOfFacemark }

impl std::fmt::Debug for core::Ptr<crate::face::FacemarkKazemi> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfFacemarkKazemi")
			.finish()
	}
}

