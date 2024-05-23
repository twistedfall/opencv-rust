ptr_extern! { crate::face::BIF,
	cv_PtrLcv_face_BIFG_new_null_const, cv_PtrLcv_face_BIFG_delete, cv_PtrLcv_face_BIFG_getInnerPtr_const, cv_PtrLcv_face_BIFG_getInnerPtrMut
}

impl core::Ptr<crate::face::BIF> {
	#[inline] pub fn as_raw_PtrOfBIF(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBIF(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::face::BIFTraitConst for core::Ptr<crate::face::BIF> {
	#[inline] fn as_raw_BIF(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::BIFTrait for core::Ptr<crate::face::BIF> {
	#[inline] fn as_raw_mut_BIF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::face::BIF> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::face::BIF> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::face::BIF>, core::Ptr<core::Algorithm>, cv_PtrLcv_face_BIFG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::face::BIF> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfBIF")
			.finish()
	}
}

