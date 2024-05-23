ptr_extern! { crate::xfeatures2d::PCTSignaturesSQFD,
	cv_PtrLcv_xfeatures2d_PCTSignaturesSQFDG_new_null_const, cv_PtrLcv_xfeatures2d_PCTSignaturesSQFDG_delete, cv_PtrLcv_xfeatures2d_PCTSignaturesSQFDG_getInnerPtr_const, cv_PtrLcv_xfeatures2d_PCTSignaturesSQFDG_getInnerPtrMut
}

impl core::Ptr<crate::xfeatures2d::PCTSignaturesSQFD> {
	#[inline] pub fn as_raw_PtrOfPCTSignaturesSQFD(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPCTSignaturesSQFD(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::xfeatures2d::PCTSignaturesSQFDTraitConst for core::Ptr<crate::xfeatures2d::PCTSignaturesSQFD> {
	#[inline] fn as_raw_PCTSignaturesSQFD(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::PCTSignaturesSQFDTrait for core::Ptr<crate::xfeatures2d::PCTSignaturesSQFD> {
	#[inline] fn as_raw_mut_PCTSignaturesSQFD(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::PCTSignaturesSQFD> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::PCTSignaturesSQFD> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::PCTSignaturesSQFD>, core::Ptr<core::Algorithm>, cv_PtrLcv_xfeatures2d_PCTSignaturesSQFDG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::xfeatures2d::PCTSignaturesSQFD> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfPCTSignaturesSQFD")
			.finish()
	}
}

