ptr_extern! { crate::face::Facemark,
	cv_PtrLcv_face_FacemarkG_new_null_const, cv_PtrLcv_face_FacemarkG_delete, cv_PtrLcv_face_FacemarkG_getInnerPtr_const, cv_PtrLcv_face_FacemarkG_getInnerPtrMut
}

impl core::Ptr<crate::face::Facemark> {
	#[inline] pub fn as_raw_PtrOfFacemark(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFacemark(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::face::FacemarkTraitConst for core::Ptr<crate::face::Facemark> {
	#[inline] fn as_raw_Facemark(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::FacemarkTrait for core::Ptr<crate::face::Facemark> {
	#[inline] fn as_raw_mut_Facemark(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::face::Facemark> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::face::Facemark> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::face::Facemark>, core::Ptr<core::Algorithm>, cv_PtrLcv_face_FacemarkG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::face::Facemark> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfFacemark")
			.finish()
	}
}

