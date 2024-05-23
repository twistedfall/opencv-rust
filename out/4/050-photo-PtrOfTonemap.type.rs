ptr_extern! { crate::photo::Tonemap,
	cv_PtrLcv_TonemapG_new_null_const, cv_PtrLcv_TonemapG_delete, cv_PtrLcv_TonemapG_getInnerPtr_const, cv_PtrLcv_TonemapG_getInnerPtrMut
}

impl core::Ptr<crate::photo::Tonemap> {
	#[inline] pub fn as_raw_PtrOfTonemap(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTonemap(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::photo::TonemapTraitConst for core::Ptr<crate::photo::Tonemap> {
	#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::TonemapTrait for core::Ptr<crate::photo::Tonemap> {
	#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::photo::Tonemap> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::photo::Tonemap> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::photo::Tonemap>, core::Ptr<core::Algorithm>, cv_PtrLcv_TonemapG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::photo::Tonemap> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTonemap")
			.finish()
	}
}

