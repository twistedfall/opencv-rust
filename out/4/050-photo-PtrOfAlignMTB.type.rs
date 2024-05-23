ptr_extern! { crate::photo::AlignMTB,
	cv_PtrLcv_AlignMTBG_new_null_const, cv_PtrLcv_AlignMTBG_delete, cv_PtrLcv_AlignMTBG_getInnerPtr_const, cv_PtrLcv_AlignMTBG_getInnerPtrMut
}

impl core::Ptr<crate::photo::AlignMTB> {
	#[inline] pub fn as_raw_PtrOfAlignMTB(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAlignMTB(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::photo::AlignMTBTraitConst for core::Ptr<crate::photo::AlignMTB> {
	#[inline] fn as_raw_AlignMTB(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::AlignMTBTrait for core::Ptr<crate::photo::AlignMTB> {
	#[inline] fn as_raw_mut_AlignMTB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::photo::AlignMTB> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::photo::AlignMTB> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::photo::AlignMTB>, core::Ptr<core::Algorithm>, cv_PtrLcv_AlignMTBG_to_PtrOfAlgorithm }

impl crate::photo::AlignExposuresTraitConst for core::Ptr<crate::photo::AlignMTB> {
	#[inline] fn as_raw_AlignExposures(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::AlignExposuresTrait for core::Ptr<crate::photo::AlignMTB> {
	#[inline] fn as_raw_mut_AlignExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::photo::AlignMTB>, core::Ptr<crate::photo::AlignExposures>, cv_PtrLcv_AlignMTBG_to_PtrOfAlignExposures }

impl std::fmt::Debug for core::Ptr<crate::photo::AlignMTB> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfAlignMTB")
			.finish()
	}
}

