ptr_extern! { crate::xfeatures2d::PCTSignatures,
	cv_PtrLcv_xfeatures2d_PCTSignaturesG_new_null_const, cv_PtrLcv_xfeatures2d_PCTSignaturesG_delete, cv_PtrLcv_xfeatures2d_PCTSignaturesG_getInnerPtr_const, cv_PtrLcv_xfeatures2d_PCTSignaturesG_getInnerPtrMut
}

impl core::Ptr<crate::xfeatures2d::PCTSignatures> {
	#[inline] pub fn as_raw_PtrOfPCTSignatures(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPCTSignatures(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::xfeatures2d::PCTSignaturesTraitConst for core::Ptr<crate::xfeatures2d::PCTSignatures> {
	#[inline] fn as_raw_PCTSignatures(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::PCTSignaturesTrait for core::Ptr<crate::xfeatures2d::PCTSignatures> {
	#[inline] fn as_raw_mut_PCTSignatures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::PCTSignatures> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::PCTSignatures> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::PCTSignatures>, core::Ptr<core::Algorithm>, cv_PtrLcv_xfeatures2d_PCTSignaturesG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::xfeatures2d::PCTSignatures> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfPCTSignatures")
			.finish()
	}
}

