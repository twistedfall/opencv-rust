ptr_extern! { crate::photo::AlignExposures,
	cv_PtrLcv_AlignExposuresG_new_null_const, cv_PtrLcv_AlignExposuresG_delete, cv_PtrLcv_AlignExposuresG_getInnerPtr_const, cv_PtrLcv_AlignExposuresG_getInnerPtrMut
}

impl core::Ptr<crate::photo::AlignExposures> {
	#[inline] pub fn as_raw_PtrOfAlignExposures(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAlignExposures(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::photo::AlignExposuresTraitConst for core::Ptr<crate::photo::AlignExposures> {
	#[inline] fn as_raw_AlignExposures(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::AlignExposuresTrait for core::Ptr<crate::photo::AlignExposures> {
	#[inline] fn as_raw_mut_AlignExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::photo::AlignExposures> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::photo::AlignExposures> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::photo::AlignExposures>, core::Ptr<core::Algorithm>, cv_PtrLcv_AlignExposuresG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::photo::AlignExposures> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfAlignExposures")
			.finish()
	}
}

