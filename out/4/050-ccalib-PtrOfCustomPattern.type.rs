ptr_extern! { crate::ccalib::CustomPattern,
	cv_PtrLcv_ccalib_CustomPatternG_new_null_const, cv_PtrLcv_ccalib_CustomPatternG_delete, cv_PtrLcv_ccalib_CustomPatternG_getInnerPtr_const, cv_PtrLcv_ccalib_CustomPatternG_getInnerPtrMut
}

ptr_extern_ctor! { crate::ccalib::CustomPattern, cv_PtrLcv_ccalib_CustomPatternG_new_const_CustomPattern }
impl core::Ptr<crate::ccalib::CustomPattern> {
	#[inline] pub fn as_raw_PtrOfCustomPattern(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCustomPattern(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ccalib::CustomPatternTraitConst for core::Ptr<crate::ccalib::CustomPattern> {
	#[inline] fn as_raw_CustomPattern(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ccalib::CustomPatternTrait for core::Ptr<crate::ccalib::CustomPattern> {
	#[inline] fn as_raw_mut_CustomPattern(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ccalib::CustomPattern> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ccalib::CustomPattern> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ccalib::CustomPattern>, core::Ptr<core::Algorithm>, cv_PtrLcv_ccalib_CustomPatternG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::ccalib::CustomPattern> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCustomPattern")
			.finish()
	}
}

