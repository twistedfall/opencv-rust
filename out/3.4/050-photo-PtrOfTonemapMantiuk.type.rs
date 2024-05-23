ptr_extern! { crate::photo::TonemapMantiuk,
	cv_PtrLcv_TonemapMantiukG_new_null_const, cv_PtrLcv_TonemapMantiukG_delete, cv_PtrLcv_TonemapMantiukG_getInnerPtr_const, cv_PtrLcv_TonemapMantiukG_getInnerPtrMut
}

impl core::Ptr<crate::photo::TonemapMantiuk> {
	#[inline] pub fn as_raw_PtrOfTonemapMantiuk(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTonemapMantiuk(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::photo::TonemapMantiukTraitConst for core::Ptr<crate::photo::TonemapMantiuk> {
	#[inline] fn as_raw_TonemapMantiuk(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::TonemapMantiukTrait for core::Ptr<crate::photo::TonemapMantiuk> {
	#[inline] fn as_raw_mut_TonemapMantiuk(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::photo::TonemapMantiuk> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::photo::TonemapMantiuk> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::photo::TonemapMantiuk>, core::Ptr<core::Algorithm>, cv_PtrLcv_TonemapMantiukG_to_PtrOfAlgorithm }

impl crate::photo::TonemapTraitConst for core::Ptr<crate::photo::TonemapMantiuk> {
	#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::TonemapTrait for core::Ptr<crate::photo::TonemapMantiuk> {
	#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::photo::TonemapMantiuk>, core::Ptr<crate::photo::Tonemap>, cv_PtrLcv_TonemapMantiukG_to_PtrOfTonemap }

impl std::fmt::Debug for core::Ptr<crate::photo::TonemapMantiuk> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTonemapMantiuk")
			.finish()
	}
}

