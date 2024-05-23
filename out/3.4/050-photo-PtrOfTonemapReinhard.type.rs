ptr_extern! { crate::photo::TonemapReinhard,
	cv_PtrLcv_TonemapReinhardG_new_null_const, cv_PtrLcv_TonemapReinhardG_delete, cv_PtrLcv_TonemapReinhardG_getInnerPtr_const, cv_PtrLcv_TonemapReinhardG_getInnerPtrMut
}

impl core::Ptr<crate::photo::TonemapReinhard> {
	#[inline] pub fn as_raw_PtrOfTonemapReinhard(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTonemapReinhard(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::photo::TonemapReinhardTraitConst for core::Ptr<crate::photo::TonemapReinhard> {
	#[inline] fn as_raw_TonemapReinhard(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::TonemapReinhardTrait for core::Ptr<crate::photo::TonemapReinhard> {
	#[inline] fn as_raw_mut_TonemapReinhard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::photo::TonemapReinhard> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::photo::TonemapReinhard> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::photo::TonemapReinhard>, core::Ptr<core::Algorithm>, cv_PtrLcv_TonemapReinhardG_to_PtrOfAlgorithm }

impl crate::photo::TonemapTraitConst for core::Ptr<crate::photo::TonemapReinhard> {
	#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::TonemapTrait for core::Ptr<crate::photo::TonemapReinhard> {
	#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::photo::TonemapReinhard>, core::Ptr<crate::photo::Tonemap>, cv_PtrLcv_TonemapReinhardG_to_PtrOfTonemap }

impl std::fmt::Debug for core::Ptr<crate::photo::TonemapReinhard> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTonemapReinhard")
			.finish()
	}
}

